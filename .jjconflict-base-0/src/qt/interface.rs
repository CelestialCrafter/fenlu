#[cxx_qt::bridge(cxx_file_stem = "interface")]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(u16, thumbnail_size)]
        type FenluInterface = super::Interface;
    }
}

use crate::config::{self, CONFIG};

type Interface = config::Interface;

impl Default for Interface {
    fn default() -> Self {
        Self { thumbnail_size: CONFIG.interface.thumbnail_size }
    }
}

