use clap::Parser;

use lor_mod_uploader::UpdateEntry;
use lor_mod_uploader::args::Args;

pub const LOR_ID: u32 = 1256670;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let title = option_env!("WORKSHOP_TITLE");
    let desc = option_env!("WORKSHOP_DESC");
    let thumbnail_path = option_env!("WORKSHOP_THUMBNAIL");

    let Some(entry) = UpdateEntry::new(
        title,
        desc,
        thumbnail_path,
        args.content_path,
        args.vis.into(),
        args.tags,
    ) else {
        eprintln!("Upload failed.");

        if title.is_none() {
            eprintln!("$WORKSHOP_TITLE is not suggested");
        }

        if desc.is_none() {
            eprintln!("$WORKSHOP_DESC is not suggested");
        }

        if thumbnail_path.is_none() {
            eprintln!("$WORKSHOP_THUMBNAIL is not suggested");
        }

        return;
    };
}
