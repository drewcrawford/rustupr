/*! Rust library that installs Rust.

The phone call is coming from inside the building!

# Status

Installs rust on macOS 12+.  Windows support is planned.

# Examples
```
async fn example() -> Result<(), rustupr::Error>{
    rustupr::ensure_rustup(kiruna::Priority::Testing).await?;
    Ok(())
}
```

# Similar libraries

Since Rust is a compiled language, its binaries are self-contained.  Therefore you can write tools to bring up a production or
development environment in Rust itself, compile them, and shoot them over to new servers via SSH.

You might be interested my expanded universe of sysadmin libraries:

* [github-actions-runner](https://github.com/drewcrawford/github-actions-runner), which installs GitHub's action runner
* [mac-install](https://github.com/drewcrawford/mac-install) which installs mac packages
* [dmg](https://github.com/drewcrawford/dmg) to mount DMG images

*/
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