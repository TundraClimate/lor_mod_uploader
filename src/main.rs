use clap::Parser;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use steamworks::{AppId, Client, PublishedFileId};

use lor_mod_uploader::UpdateEntry;
use lor_mod_uploader::args::Args;

pub const LOR_ID: u32 = 1256670;

fn main() {
    let args = Args::parse();

    let Some(entry) = UpdateEntry::new(
        args.title,
        args.desc,
        args.thumbnail_path,
        args.content_path,
        args.vis.into(),
        args.tags,
    ) else {
        eprintln!("Upload failed.");
        eprintln!("The paths is not absolute");

        return;
    };

    let Ok(client) = Client::init() else {
        eprintln!("Steam client not launched");

        return;
    };

    let ugc = client.ugc();
    let wait = Arc::new(AtomicBool::new(false));

    let item_id = args.id;

    println!("Using WORKSHOP_ID is {}", item_id);

    let wait_cb = wait.clone();

    println!("Start Upload");

    let _handle = ugc
        .start_item_update(AppId(LOR_ID), PublishedFileId(item_id))
        .title(&entry.title)
        .description(&entry.desc)
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
