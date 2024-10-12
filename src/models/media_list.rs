use std::{collections::HashMap, sync::LazyLock};

use eyre::Result;
use iced::{
    widget::{column, image, text, Image, Row},
    Element,
};
use tokio::sync::RwLock;

use crate::protocol::media::Media;

use super::main;

static IMAGE_CACHE: LazyLock<RwLock<HashMap<String, image::Handle>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Default)]
pub struct MediaList {
    media: Vec<(Media, image::Handle)>,
}

pub async fn image_handle_from_uri(uri: String) -> Result<image::Handle> {
    let cache_read = IMAGE_CACHE.read().await;
    println!("{:?}", cache_read);

    let handle = match cache_read.get(&uri) {
        Some(h) => h.clone(),
        None => {
            drop(cache_read);
            let bytes = reqwest::get(uri.clone()).await?.bytes().await?;
            let handle = image::Handle::from_bytes(bytes);

            let mut cache = IMAGE_CACHE.write().await;
            cache.insert(uri, handle.clone());

            handle
        }
    };

    Ok(handle)
}

impl MediaList {
    pub fn add_image(&mut self, media: Media, handle: image::Handle) {
        self.media.push((media, handle));
    }
    pub fn view(&self) -> Element<main::Message> {
        Row::with_children(self.media.clone().into_iter().map(|(media, handle)| {
            column![text(media.title), Image::new(handle).width(128).height(128)].into()
        }))
        .wrap()
        .into()
    }
}
