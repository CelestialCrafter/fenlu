#[cxx_qt::bridge(cxx_file_stem = "queries")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(usize, total)]
        type FenluScripts = super::Scripts;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn item(self: &FenluScripts, index: usize) -> QString;
    }

    impl cxx_qt::Constructor<()> for FenluScripts {}
}

use std::pin::Pin;
use cxx_qt_lib::QString;
use qobject::FenluScripts;

use crate::{pipeline::{sources, transforms}, utils};

impl qobject::FenluScripts {
    pub fn item(self: &FenluScripts, index: usize) -> QString {
        match self.scripts.get(index as usize) {
            Some(name) => name.clone(),
            None => QString::default()
        }
    }
}

#[derive(Default)]
pub struct Scripts {
    scripts: Vec<QString>,
    total: usize
}

impl cxx_qt::Initialize for FenluScripts {
    fn initialize(mut self: Pin<&mut Self>) {
        let scripts: Vec<QString> = sources::scripts()
                .chain(transforms::scripts())
                .map(|path| path.expect("could not read path"))
                .filter(|path| utils::is_script_whitelisted(path))
                .map(|path| utils::path_to_name(&path))
                .map(|name| QString::from(&name))
                .collect();

        let mut self_mut = self.as_mut().cxx_qt_ffi_rust_mut();
        self_mut.total = scripts.len();
        self_mut.scripts = scripts;
    }
}

