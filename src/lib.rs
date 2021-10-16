use thiserror::Error;
#[derive(Error,Debug)]
pub enum Error {
    #[error("Rustup failed")]
    RustUp(#[from] command_rs::Error),
    #[cfg(target_os = "windows")]
    #[error("Download failed")]
    Requestr(#[from] requestr::Error)
}
mod install_rustup;

pub use install_rustup::ensure_rustup;