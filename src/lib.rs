use thiserror::Error;
#[derive(Error,Debug)]
enum Error {
    #[error("Rustup failed")]
    RustUp(#[from] std::io::Error)
}
mod install_rustup;
