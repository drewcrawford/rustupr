use command_rs::{Command, ExitStatus};
use crate::Error;
use pcore::release_pool::ReleasePool;

async fn update_rustup(priority: kiruna::Priority) -> Result<(),Error> {
    Command::new("rustup")
        .arg("update")
        .status(priority).await?;
    Ok(())
}

async fn is_rustup_installed(priority: kiruna::Priority) -> bool {
    Command::new("rustup").status(priority).await.map_or_else(|_| false, |_| true)
}
async fn install_rustup(priority: kiruna::Priority) -> Result<(),Error> {
    #[cfg(target_os = "macos")] {
        let status = Command::new("/bin/sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
            .status(priority).await?;
        status.check_err().map_err(|e| e.into())
    }
   #[cfg(target_os="windows")] {
       //we're on windows, so eh
       let pool = unsafe{ReleasePool::assuming_pool()};
       let download_request = requestr::Request::new(pcore::pstr!("https://win.rustup.rs/x86_64"), pool)?;
       let download = download_request.download(pool).await?;
       let path = download.copy_path();
       /*see https://github.com/rust-lang/rustup/blob/44be718122ecff073bcb2dfd44c6b50ed84c7696/src/bin/rustup-init.rs#L100-L109, but the file
       actually needs to be named 'rustup.exe' for some reason
        */
       let mut new_path = path.clone();
       new_path.pop();
       new_path.push("rustup-init.exe");
       std::fs::rename(path,new_path.clone()).unwrap();
       std::mem::forget(download); //don't bother cleaning up tempfile; it was moved
       let status = Command::new(new_path)
           .arg("-y")
           .status(priority).await?;
       status.check_err().map_err(|e| e.into())
   }
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