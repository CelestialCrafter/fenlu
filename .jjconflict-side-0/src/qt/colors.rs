#[cxx_qt::bridge(cxx_file_stem = "colors")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(QColor, base)]
        #[qproperty(QColor, surface)]
        #[qproperty(QColor, highlight_medium)]
        #[qproperty(QColor, highlight_high)]
        #[qproperty(QColor, text)]
        #[qproperty(QColor, accent)]
        type FenluColors = super::Colors;
    }
}

use cxx_qt_lib::QColor;

use crate::config::CONFIG;

pub struct Colors {
        base: QColor,
        surface: QColor,   
        highlight_medium: QColor,   
        highlight_high: QColor,   
        text: QColor,   
        accent: QColor,   
}

impl Default for Colors {
    fn default() -> Self {
        let convert = |rgb: [u8; 3]| QColor::from_rgb(rgb[0].into(), rgb[1].into(), rgb[2].into());

        Self {
            base: convert(CONFIG.colors.base),
            surface: convert(CONFIG.colors.surface),
            highlight_medium: convert(CONFIG.colors.highlight_medium),
            highlight_high: convert(CONFIG.colors.highlight_high),
            text: convert(CONFIG.colors.text),
            accent: convert(CONFIG.colors.accent)
        }
    }
}

