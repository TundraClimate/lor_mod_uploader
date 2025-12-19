use tokio::process::Command;

pub async fn login(uid: &str) {
    Command::new("steamcmd")
        .args(["+login", uid])
        .arg("+quit")
        .status()
        .await
        .ok();
}
