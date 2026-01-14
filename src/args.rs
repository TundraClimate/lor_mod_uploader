use clap::{Parser, Subcommand, value_parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about = "Library of Ruina workshop publisher and updator")]
pub struct Args {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    New {
        #[arg(long = "content", value_parser = value_parser!(PathBuf))]
        content: PathBuf,

        #[arg(long = "thumbnail", value_parser = value_parser!(PathBuf))]
        thumbnail: PathBuf,

        #[arg(long = "title")]
        title: String,

        #[arg(long = "desc")]
        description: String,

        #[arg(long = "vis")]
        visibility: u8,
    },

    Update {
        mod_id: u32,

        #[arg(long = "content", value_parser = value_parser!(PathBuf))]
        content: PathBuf,

        #[arg(long = "thumbnail", value_parser = value_parser!(PathBuf))]
        thumbnail: PathBuf,

        #[arg(long = "title")]
        title: String,

        #[arg(long = "desc")]
        description: String,

        #[arg(long = "vis")]
        visibility: u8,
    },

    FromInfo {
        #[arg(value_parser = value_parser!(PathBuf))]
        mod_info: PathBuf,

        mod_id: Option<u32>,

        #[arg(long = "content", value_parser = value_parser!(PathBuf))]
        content: Option<PathBuf>,

        #[arg(long = "thumbnail", value_parser = value_parser!(PathBuf))]
        thumbnail: Option<PathBuf>,

        #[arg(long = "title")]
        title: Option<String>,

        #[arg(long = "desc")]
        description: Option<String>,

        #[arg(long = "vis")]
        visibility: Option<u8>,
    },

    Login {
        user_id: String,
    },
}
