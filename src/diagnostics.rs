use std::path::Path;

use crate::{
    clippy::ClippyCommand,
    context::Context,
    error::{Error, Result},
    matcher::FileMatcher,
};

use cargo_metadata::{Message, diagnostic::DiagnosticLevel};
use clap::ValueEnum;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Note,
    Help,
    #[default]
    Warning,
    Error,
}

impl From<DiagnosticLevel> for Level {
    fn from(level: DiagnosticLevel) -> Self {
        match level {
            DiagnosticLevel::Note => Self::Note,
            DiagnosticLevel::Help => Self::Help,
            DiagnosticLevel::Error | DiagnosticLevel::Ice => Self::Error,
            _ => Self::Warning,
        }
    }
}

pub fn run_diagnostics(ctx: &Context) -> Result<bool> {
    let mut clippy_command = ctx
        .workspace_packages
        .iter()
        .fold(ClippyCommand::new().args(&ctx.clippy_args), |cmd, pkg| cmd.arg("-p").arg(pkg));
    debug!("running clippy with: `{clippy_command}`");

    let mut clippy_runner = clippy_command.spawn()?;

    let mut file_matcher =
        (!ctx.files.is_empty()).then(|| FileMatcher::new(&ctx.files, &ctx.workspace_root));

    let mut success = true;
    for message_result in &mut clippy_runner {
        let message = message_result.map_err(Error::ClippyMessageParse)?;

        let Message::CompilerMessage(compiler_message) = message else {
            continue;
        };

        let diagnostic = &compiler_message.message;
        let Some(rendered) = &diagnostic.rendered else {
            continue;
        };

        let message_level: Level = diagnostic.level.into();
        if message_level < ctx.level {
            debug!(
                "skipping diagnostic below threshold {:?} (found {:?}) for: {}",
                message_level, ctx.level, diagnostic.message
            );
            continue;
        }

        let matched = if let Some(matcher) = &mut file_matcher {
            let mut matched = false;
            for span in &diagnostic.spans {
                debug!("checking span file: `{}`", span.file_name);
                matched = matcher.matches(Path::new(&span.file_name))?;
                if matched {
                    debug!(
                        "file `{}` matched for diagnostic: {}",
                        span.file_name, diagnostic.message
                    );
                    break;
                }
            }
            if !matched {
                debug!("no matching file found for diagnostic: {}", diagnostic.message);
            }
            matched
        } else {
            debug!("no file matcher, all diagnostics considered matched");
            true
        };

        if matched {
            success = false;
            println!("{rendered}");
        }
    }

    let (status, stderr) = clippy_runner.wait()?;

    if !status.success() {
        if !stderr.is_empty() {
            eprintln!("{stderr}");
        }

        return Err(Error::ClippyFailed { exit_code: status.code().unwrap_or(-1) });
    }

    Ok(success)
}
