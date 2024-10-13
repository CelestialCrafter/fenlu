pub mod config;
pub mod models;
pub mod pipeline;
pub mod protocol;
pub mod script;
pub mod utils;

use models::main::Main;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    iced::application("Fenlu", Main::update, Main::view)
        .subscription(Main::subscribe)
        .run()
}
