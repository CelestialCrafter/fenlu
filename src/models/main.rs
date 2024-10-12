use std::{str::FromStr, sync::Arc};

use fluent_uri::UriRef;
use iced::{
    widget::{column, image, Column},
    Task,
};
use tokio::{sync::mpsc::channel, task};

use crate::{config::CONFIG, pipeline::Pipeline, protocol::media::Media};

use super::media_list::{self, image_handle_from_uri};

#[derive(Debug, Default)]
pub struct Main {
    pipeline: Option<Arc<Pipeline>>,
    media_list: media_list::MediaList,
}

#[derive(Debug)]
pub enum Message {
    NewMedia((Media, image::Handle)),
    PipelineReady(Pipeline),
    PipelineFinished,
}

impl Main {
    pub fn new() -> (Self, Task<Message>) {
        let model = Self::default();
        let task = Task::perform(Pipeline::new(), move |r| match r {
            Ok(pipeline) => Message::PipelineReady(pipeline),
            // @TODO handle this
            Err(err) => panic!("{}", err),
        });

        (model, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NewMedia((media, handle)) => {
                self.media_list.add_image(media, handle);

                Task::none()
            }
            Message::PipelineReady(pipeline) => {
                println!("pipeline ready");
                self.pipeline = Some(Arc::new(pipeline));

                let (tx, rx) = channel(CONFIG.batch_size);
                let future = self.pipeline.unwrap().run(CONFIG.batch_size, tx);

                Task::perform(
                    async {
                        // @TODO use rx and send new media messages
                        let handle = task::spawn(async {});

                        future.await;
                        handle.await;
                    },
                    |_| Message::PipelineFinished,
                )
            }
            Message::PipelineFinished => {
                println!("pipeline finished");

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        if let None = self.pipeline {
            return column!["loading pipeline..."];
        }

        column![self.media_list.view()]
    }
}
