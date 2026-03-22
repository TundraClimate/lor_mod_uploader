use clap::Parser;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use steamworks::{AppId, Client, FileType, PublishedFileId, UGC};

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

    let Ok(client) = Client::init_app(LOR_ID) else {
        eprintln!("Steam client not launched");

        return;
    };

    let ugc = client.ugc();

    let item_id = Arc::new(AtomicU64::new(args.id));

    println!("Using WORKSHOP_ID is {}", item_id.load(Ordering::SeqCst));

    if item_id.load(Ordering::SeqCst) == 0 {
        println!("ID is zero: create item");

        create_item(&client, &ugc, item_id.clone());
    }

    println!("Start Upload");

    upload_item(
        &client,
        &ugc,
        entry,
        PublishedFileId(item_id.load(Ordering::SeqCst)),
    );
}

fn create_item(client: &Client, ugc: &UGC, item_id: Arc<AtomicU64>) {
    let wait = Arc::new(AtomicBool::new(false));
    let wait_cb = wait.clone();
    ugc.create_item(AppId(LOR_ID), FileType::Community, move |create_result| {
        match create_result {
            Ok((_, needs_agree)) if needs_agree => eprintln!("Needs agree to terms"),
            Ok((published_id, _)) => {
                item_id.store(published_id.0, Ordering::SeqCst);

                println!("Create item with id {:?}", published_id);
            }
            Err(e) => eprintln!("Error create item: {:?}", e),
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

fn upload_item(client: &Client, ugc: &UGC, entry: UpdateEntry, id: PublishedFileId) {
    let wait = Arc::new(AtomicBool::new(false));
    let wait_cb = wait.clone();
    let _handle = ugc
        .start_item_update(AppId(LOR_ID), id)
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
