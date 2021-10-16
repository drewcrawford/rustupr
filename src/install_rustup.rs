use command_rs::{Command, ExitStatus};
use crate::Error;

async fn update_rustup(priority: kiruna::Priority) -> Result<(),Error> {
    Command::new("rustup")
        .arg("update")
        .status(priority).await?;
    Ok(())
}

async fn is_rustup_installed(priority: kiruna::Priority) -> bool {
    Command::new("rustup").status(priority).await.map_or_else(|_| false,|_| true)
}
async fn install_rustup(priority: kiruna::Priority) -> Result<(),Error> {
    let status = Command::new("/bin/sh")
        .arg("-c")
        .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
        .status(priority).await?;
    status.check_err().map_err(|e| e.into())
}
pub async fn ensure_rustup(priority: kiruna::Priority) -> Result<(),Error> {
    if !is_rustup_installed(priority).await {
        install_rustup(priority).await?;
    }
    else {
        update_rustup(priority).await?;
    }
    Ok(())
}
#[test] fn test_rustup() {
    let f = ensure_rustup(kiruna::Priority::Testing);
    let a = kiruna::test::test_await(f, std::time::Duration::from_secs(180));
    a.unwrap()
}