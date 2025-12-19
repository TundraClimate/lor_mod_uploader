use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

mod args;

use args::{Args, SubCommand};

pub const LOR_ID: usize = 1256670;

fn main() {
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
        SubCommand::Login { user_id } => {}
        SubCommand::New {
            content,
            thumbnail,
            title,
            description,
        } => {}
        SubCommand::Update {
            mod_id,
            content,
            thumbnail,
            title,
            description,
        } => {}
        SubCommand::FromInfo {
            mod_info,
            mod_id,
            content,
            thumbnail,
            title,
            description,
        } => {}
    }
}

fn tmp_dir() -> PathBuf {
    PathBuf::from("/tmp/lor_uploader")
}

fn vdf_path() -> PathBuf {
    tmp_dir().join(".vdf")
}
