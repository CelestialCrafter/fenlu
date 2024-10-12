use std::str::FromStr;

use fluent_uri::UriRef;
use iced::{
    widget::{column, image, Column},
    Task,
};

use crate::protocol::media::Media;

use super::media_list::{self, image_handle_from_uri};

#[derive(Debug, Default)]
pub struct Main {
    media_list: media_list::MediaList,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewMedia((Media, image::Handle)),
}

impl Main {
    pub fn new() -> (Self, Task<Message>) {
        let model = Self::default();
        let media = Media {
            title: "my media".to_string(),
            uri: UriRef::from_str("https://avatars.githubusercontent.com/u/44733683").unwrap(),
            extra_source: "".to_string(),
            history: vec![],
            tags: vec![],
            extra: Option::None,
        };

        let mut task = Task::none();
        for _ in 0..10 {
            let media = media.clone();
            task = task.chain(Task::perform(
                image_handle_from_uri(media.uri.to_string()),
                move |r| match r {
                    Ok(handle) => Message::NewMedia((media.clone(), handle)),
                    // @TODO handle this
                    Err(err) => panic!("{}", err),
                },
            ));
        }

        (model, task)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NewMedia((media, handle)) => {
                self.media_list.add_image(media, handle);
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        column![self.media_list.view()]
    }
}
