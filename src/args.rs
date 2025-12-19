use clap::{Parser, Subcommand, value_parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about = "Library of Ruina workshop publisher and updator")]
pub struct Args {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    New {
        #[arg(short = 'C', long = "content", value_parser = value_parser!(PathBuf))]
        content: PathBuf,

        #[arg(short = 'p', long = "thumbnail", value_parser = value_parser!(PathBuf))]
        thumbnail: PathBuf,

        #[arg(short = 't', long = "title")]
        title: String,

        #[arg(short = 'd', long = "desc")]
        description: String,
    },

    Update {
        mod_id: u32,

        #[arg(short = 'C', long = "content", value_parser = value_parser!(PathBuf))]
        content: PathBuf,

        #[arg(short = 'p', long = "thumbnail", value_parser = value_parser!(PathBuf))]
        thumbnail: PathBuf,

        #[arg(short = 't', long = "title")]
        title: String,

        #[arg(short = 'd', long = "desc")]
        description: String,
    },

    FromInfo {
        #[arg(value_parser = value_parser!(PathBuf))]
        mod_info: PathBuf,

        mod_id: Option<u32>,

        #[arg(short = 'C', long = "content", value_parser = value_parser!(PathBuf))]
        content: Option<PathBuf>,

        #[arg(short = 'p', long = "thumbnail", value_parser = value_parser!(PathBuf))]
        thumbnail: Option<PathBuf>,

        #[arg(short = 't', long = "title")]
        title: Option<String>,

        #[arg(short = 'd', long = "desc")]
        description: Option<String>,
    },

    Login {
        user_id: String,
    },
}
