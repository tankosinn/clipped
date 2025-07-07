pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to get cargo metadata: `{0}`")]
    CargoMetadata(#[from] Box<cargo_metadata::Error>),

    #[error("failed to build config: `{0}`")]
    Config(#[from] Box<figment::Error>),

    #[error("failed to resolve `{path}` file: `{source}`")]
    PathResolution {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to spawn `cargo clippy`: `{0}`")]
    ClippySpawn(#[source] std::io::Error),

    #[error("failed to capture {stream} from `cargo clippy`")]
    ClippyCapture { stream: &'static str },

    #[error("failed to parse `cargo clippy` output: `{0}`")]
    ClippyMessageParse(#[source] std::io::Error),

    #[error("failed to read from clippy's stderr: {0}")]
    ClippyStderrRead(#[source] std::io::Error),

    #[error("failed to wait for `cargo clippy` process: `{0}`")]
    ClippyWait(#[source] std::io::Error),

    #[error("`cargo clippy` process exited with non-zero status: {exit_code}")]
    ClippyFailed { exit_code: i32 },
}
