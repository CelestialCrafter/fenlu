use futures::{executor::block_on, SinkExt, Stream};
use iced::{
    stream,
    widget::{column, image, Column},
    Subscription, Task,
};
use tokio::{sync::mpsc, task};
use tracing::{info, instrument};

use crate::{config::CONFIG, pipeline::Pipeline, protocol::media::Media};

use super::media_list::{self, image_handle_from_uri};

#[derive(Debug, Default)]
pub struct Main {
    media_list: media_list::MediaList,
    pipeline_control: Option<mpsc::Sender<()>>,
}

#[derive(Debug)]
pub enum Message {
    NewMedia((Media, image::Handle)),
    PipelineReady(mpsc::Sender<()>),
    PipelineFinished,
}

fn start_pipeline() -> impl Stream<Item = Message> {
    stream::channel(CONFIG.buffer_size, |mut message_tx| async move {
        // @TODO handle unwraps
        let mut pipeline = Pipeline::default();
        pipeline.populate().await.unwrap();

        let (work_tx, mut work_rx) = mpsc::channel(1);
        message_tx
            .send(Message::PipelineReady(work_tx))
            .await
            .unwrap();

        while let Some(_) = work_rx.recv().await {
            info!("running pipeline");

            let (media_tx, mut media_rx) = mpsc::channel::<Media>(CONFIG.buffer_size);
            pipeline.run(CONFIG.buffer_size, media_tx).await.unwrap();

            let mut message_tx = message_tx.clone();
            task::spawn(async move {
                while let Some(media) = media_rx.recv().await {
                    let handle = image_handle_from_uri(media.uri.clone()).await?;
                    message_tx.send(Message::NewMedia((media, handle))).await?;
                }

                Ok::<_, eyre::Report>(())
            })
            .await
            .unwrap()
            .unwrap();
        }
    })
}

impl Main {
    pub fn subscribe(&self) -> Subscription<Message> {
        Subscription::run(start_pipeline)
    }

    #[instrument(name = "main update", level = "debug", skip(self))]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NewMedia((media, handle)) => {
                info!(uri = media.uri.to_string(), "recieved media");
                self.media_list.add_image(media, handle);

                Task::none()
            }
            Message::PipelineReady(tx) => {
                info!("pipeline ready");

                block_on(tx.send(())).unwrap();
                self.pipeline_control = Some(tx);

                Task::none()
            }
            Message::PipelineFinished => {
                info!("pipeline finished");

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        if self.pipeline_control.is_none() {
            return column!["loading pipeline..."];
        }

        column![self.media_list.view()]
    }
}
