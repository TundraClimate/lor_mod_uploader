use serde::Serialize;
use std::path::PathBuf;

#[allow(non_camel_case_types)]
#[derive(Serialize)]
pub struct workshopitem {
    appid: u32,
    publishedfileid: u32,
    contentfolder: PathBuf,
    previewfile: PathBuf,
    title: String,
    description: String,
    visibility: u8,
}

impl workshopitem {
    pub fn new(
        publishedfileid: u32,
        contentfolder: PathBuf,
        previewfile: PathBuf,
        title: String,
        description: String,
        visibility: u8,
    ) -> Self {
        Self {
            appid: crate::LOR_ID,
            publishedfileid,
            contentfolder: contentfolder.canonicalize().unwrap(),
            previewfile: previewfile.canonicalize().unwrap(),
            title,
            description,
            visibility,
        }
    }
}
