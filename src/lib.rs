pub mod args;

use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use steamworks::PublishedFileVisibility;

pub struct UpdateEntry {
    pub title: &'static str,
    pub desc: &'static str,
    pub thumbnail_path: PathBuf,
    pub content_path: PathBuf,
    pub visibility: PublishedFileVisibility,
    pub tags: Vec<String>,
}

impl UpdateEntry {
    pub fn new(
        title: Option<&'static str>,
        desc: Option<&'static str>,
        thumbnail: Option<&'static str>,
        content: PathBuf,
        visibility: PublishedFileVisibility,
        tags: Vec<String>,
    ) -> Option<Self> {
        Some(Self {
            title: title?,
            desc: desc?,
            thumbnail_path: PathBuf::from(thumbnail?).canonicalize().ok()?,
            content_path: content.canonicalize().ok()?,
            visibility,
            tags,
        })
    }
}

#[derive(Default, Clone, Copy)]
pub enum Visibility {
    #[default]
    Public,
    FriendsOnly,
    Private,
    Unlisted,
}

impl From<Visibility> for PublishedFileVisibility {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Public => PublishedFileVisibility::Public,
            Visibility::FriendsOnly => PublishedFileVisibility::FriendsOnly,
            Visibility::Private => PublishedFileVisibility::Private,
            Visibility::Unlisted => PublishedFileVisibility::Unlisted,
        }
    }
}

#[derive(Debug)]
pub struct VisParseErr(&'static str);

impl std::error::Error for VisParseErr {}

impl Display for VisParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Visibility {
    type Err = VisParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_lowercase();

        match input.as_str() {
            "public" => Ok(Self::Public),
            "friendsonly" => Ok(Self::FriendsOnly),
            "private" => Ok(Self::Private),
            "unlisted" => Ok(Self::Unlisted),
            _ => Err(VisParseErr("Not valid input")),
        }
    }
}
