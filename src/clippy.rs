use std::{
    env, ffi,
    fmt::{self, Display, Formatter},
    io::{BufReader, Read},
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use crate::error::{Error, Result};

use cargo_metadata::Message;

#[derive(Debug)]
pub struct ClippyRunner {
    child: Child,
    stderr_handle: JoinHandle<Result<String>>,
    receiver: Receiver<std::result::Result<Message, std::io::Error>>,
}

impl ClippyRunner {
    pub fn wait(mut self) -> Result<(ExitStatus, String)> {
        let status = self.child.wait().map_err(Error::ClippyWait)?;
        let stderr = self.stderr_handle.join().expect("failed to join stderr thread")?;
        Ok((status, stderr))
    }
}

impl Iterator for ClippyRunner {
    type Item = std::result::Result<Message, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}

#[derive(Debug)]
pub struct ClippyCommand {
    cmd: Command,
}

impl ClippyCommand {
    pub fn new() -> Self {
        let mut command = Command::new(env::var("CARGO").unwrap_or("cargo".into()));
        command.arg("clippy").arg("--message-format=json-diagnostic-rendered-ansi");
        ClippyCommand { cmd: command }
    }

    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<ffi::OsStr>,
    {
        self.cmd.args(args);
        self
    }

    pub fn spawn(&mut self) -> Result<ClippyRunner> {
        let mut child = self
            .cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(Error::ClippySpawn)?;

        let stdout = child.stdout.take().ok_or(Error::ClippyCapture { stream: "stdout" })?;
        let mut stderr = child.stderr.take().ok_or(Error::ClippyCapture { stream: "stderr" })?;

        let (tx, rx) = mpsc::sync_channel::<std::result::Result<Message, std::io::Error>>(100);

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for message in Message::parse_stream(reader) {
                if tx.send(message).is_err() {
                    break;
                }
            }
        });

        let stderr_handle = thread::spawn(move || {
            let mut output = String::new();
            stderr.read_to_string(&mut output).map(|_| output).map_err(Error::ClippyStderrRead)
        });

        Ok(ClippyRunner { child, stderr_handle, receiver: rx })
    }
}

impl Default for ClippyCommand {
    fn default() -> Self {
        ClippyCommand::new()
    }
}

impl Display for ClippyCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cmd.get_program().to_string_lossy())?;
        for arg in self.cmd.get_args() {
            write!(f, " {}", arg.to_string_lossy())?;
        }
        Ok(())
    }
}
