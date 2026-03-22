use std::path::PathBuf;

use crate::Visibility;

use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about = "Library of Ruina workshop publisher and updator")]
pub struct Args {
    /// A title of workshop item
    pub title: String,

    /// A description of workshop item
    pub desc: String,

    /// A path of the thumbnail
    pub thumbnail_path: PathBuf,

    /// A path of the publish target
    pub content_path: PathBuf,

    #[arg(long = "id", default_value_t = 0)]
    pub id: u64,

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
