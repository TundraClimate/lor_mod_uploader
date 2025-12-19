use std::process;
use tokio::process::Command;

pub async fn login(uid: &str) {
    Command::new("steamcmd")
        .args(["+login", uid])
        .arg("+quit")
        .status()
        .await
        .ok();
}

pub async fn exec_item() {
    let Some(uid) = crate::read_uid() else {
        eprintln!("Uploader failed");
        eprintln!("Unable to read a UID: needs login");
        eprintln!();
        eprintln!("Exec here: lorupdator login");

        process::exit(1);
    };

    Command::new("steamcmd")
        .args(["+login", &uid])
        .arg("+workshop_build_item")
        .arg(crate::vdf_path())
        .arg("+quit")
        .status()
        .await
        .ok();
}
