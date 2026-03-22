use clap::Parser;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use steamworks::{AppId, Client, PublishedFileId};

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

    println!("$WORKSHOP_TITLE suggested");
    println!("$WORKSHOP_DESC suggested");
    println!("$WORKSHOP_THUMBNAIL suggested");

    let Ok(client) = Client::init() else {
        eprintln!("Steam client not launched");

        return;
    };

    let ugc = client.ugc();
    let wait = Arc::new(AtomicBool::new(false));

    let item_id = option_env!("WORKSHOP_ID");

    if item_id.is_none() {
        eprintln!("$WORKSHOP_ID is not suggested");
    }

    let item_id = item_id.and_then(|tid| tid.parse::<u64>().ok()).unwrap_or(0);

    println!("Using WORKSHOP_ID is {}", item_id);

    let wait_cb = wait.clone();

    let _handle = ugc
        .start_item_update(AppId(LOR_ID), PublishedFileId(item_id))
        .title(entry.title)
        .description(entry.desc)
        .preview_path(&entry.thumbnail_path)
        .content_path(&entry.content_path)
        .visibility(entry.visibility)
        .tags(entry.tags, false)
        .submit(Some(""), move |upload_result| {
            match upload_result {
                Ok((_, needs_agree)) if needs_agree => eprintln!("Needs agree to terms"),
                Ok((published_id, _)) => println!("Uploaded item with id {:?}", published_id),
                Err(e) => eprintln!("Error uploading item: {:?}", e),
            }

            wait_cb.store(true, Ordering::SeqCst);
        });

    loop {
        if wait.load(Ordering::SeqCst) {
            break;
        }

        client.run_callbacks();

        thread::sleep(Duration::from_millis(100));
    }
}
