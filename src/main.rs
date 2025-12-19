use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

mod args;
mod steam;
mod vdf;

use args::{Args, SubCommand};
use vdf::workshopitem;

pub const LOR_ID: u32 = 1256670;

#[tokio::main]
async fn main() {
    let tmp = tmp_dir();

    if !tmp.exists()
        && let Err(e) = fs::create_dir_all(tmp)
    {
        eprintln!("Uploader failed");
        eprintln!("Unable to create a TEMP directory: {}", e);

        process::exit(1);
    }

    let args = Args::parse();

    match args.command {
        SubCommand::Login { user_id } => {
            if let Err(e) = fs::write(uid_path(), user_id.as_bytes()) {
                eprintln!("Uploader failed");
                eprintln!("Unable to write a UID for file: {}", e);

                process::exit(1);
            }

            steam::login(&user_id).await;
        }
        SubCommand::New {
            content,
            thumbnail,
            title,
            description,
            visibility,
            ..
        } => {
            let workshopitem =
                workshopitem::new(0, content, thumbnail, title, description, visibility);

            let content = vdf_serde::to_string(&workshopitem).unwrap();

            if let Err(e) = fs::write(vdf_path(), &content) {
                eprintln!("Uploader failed");
                eprintln!("Unable to write a VDF: {}", e);

                process::exit(1);
            }

            steam::exec_item().await;
        }
        _ => {}
    }
}

fn tmp_dir() -> PathBuf {
    PathBuf::from("/tmp/lor_uploader")
}

fn vdf_path() -> PathBuf {
    tmp_dir().join(".vdf")
}

fn uid_path() -> PathBuf {
    tmp_dir().join(".uid")
}

fn read_uid() -> Option<String> {
    fs::read_to_string(uid_path()).ok()
}
