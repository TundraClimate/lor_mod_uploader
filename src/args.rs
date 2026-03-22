use std::path::PathBuf;

use crate::Visibility;

use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about = "Library of Ruina workshop publisher and updator")]
pub struct Args {
    /// A path of the publish target
    pub content_path: PathBuf,

    #[arg(short = 'v', default_value = "public")]
    /// Specify workshop visibility
    ///
    /// - public
    ///
    /// - friendsonly
    ///
    /// - private
    ///
    /// - unlisted
    pub vis: Visibility,

    #[arg(long = "tag")]
    /// Specify workshop tag
    pub tags: Vec<String>,
}
