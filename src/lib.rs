use thiserror::Error;
#[derive(Error,Debug)]
pub enum Error {
    #[error("Rustup failed")]
    RustUp(#[from] std::io::Error)
}
mod install_rustup;

pub use install_rustup::ensure_rustup;