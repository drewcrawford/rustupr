use command_rs::Command;
use crate::Error;

async fn update_rustup() -> Result<(),Error> {
    Command::new("rustup")
        .arg("update")
        .status().await?;
    Ok(())
}

async fn is_rustup_installed() -> bool {
    Command::new("rustup").status().await.map_or_else(|_| false,|_| true)
}

pub async fn ensure_rustup() -> Result<(),Error> {
    if !is_rustup_installed().await {
        let r = Command::new("/bin/sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
            .status().await;
        r?;
    }
    else {
        update_rustup().await?;
    }
    Ok(())
}
#[test] fn test_rustup() {
    let f = ensure_rustup();
    let a = kiruna::test::test_await(f, std::time::Duration::from_secs(20));
    a.unwrap()
}