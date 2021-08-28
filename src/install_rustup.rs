use command_rs::Command;
use crate::Error;

async fn is_rustup_installed() -> bool {
    Command::new("rustup").status().await.map_or_else(|_| false,|_| true)
}

async fn install_rustup_if_needed() -> Result<(),Error> {
    if !is_rustup_installed().await {
        let r = Command::new("/bin/sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
            .status().await;
        r?;
    }
    Ok(())
}

#[test] fn test_rustup() {
    let f = install_rustup_if_needed();
    let a = kiruna::test::test_await(f, std::time::Duration::from_secs(20));
    a.unwrap()
}