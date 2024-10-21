use serde::Deserialize;

use crate::qt::pipeline::Media;

pub const ACTION_BASE_METHOD: &str = "action/";

pub type ActionRequest = Media;
#[derive(Debug, Deserialize, Clone)]
pub struct ActionResponse {}
