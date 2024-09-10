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
        #[qproperty(usize, sources_total)]
        #[qproperty(usize, transforms_total)]
        #[qproperty(usize, filters_total)]
        type FenluScripts = super::Scripts;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn get_source(self: &FenluScripts, index: usize) -> QString;
        #[qinvokable]
        fn get_transform(self: &FenluScripts, index: usize) -> QString;
        #[qinvokable]
        fn get_filter(self: &FenluScripts, index: usize) -> QString;
    }

    impl cxx_qt::Constructor<()> for FenluScripts {}
}

use std::pin::Pin;
use cxx_qt_lib::QString;
use glob::{glob, Paths, PatternError};
use qobject::FenluScripts;

impl qobject::FenluScripts {
    pub fn get_source(self: &FenluScripts, index: usize) -> QString {
        match self.sources.get(index as usize) {
            Some(name) => name.clone(),
            None => QString::default()
        }
    }

    pub fn get_transform(self: &FenluScripts, index: usize) -> QString {
        match self.transforms.get(index as usize) {
            Some(name) => name.clone(),
            None => QString::default()
        }
    }

    pub fn get_filter(self: &FenluScripts, index: usize) -> QString {
        match self.filters.get(index as usize) {
            Some(name) => name.clone(),
            None => QString::default()
        }
    }
}

#[derive(Default)]
pub struct Scripts {
    sources: Vec<QString>,
    sources_total: usize,

    transforms: Vec<QString>,
    transforms_total: usize,

    filters: Vec<QString>,
    filters_total: usize
}

impl cxx_qt::Initialize for FenluScripts {
    fn initialize(mut self: Pin<&mut Self>) {
        let glob_to_qstrings = |glob: Result<Paths, PatternError>| -> Vec<QString> {
            glob
                .expect("glob should be valid")
                .map(|path| path.expect("path read should succeed"))
                .map(|path| path.file_name().unwrap().to_os_string().into_string().expect("path should be utf-8"))
                .map(|name| QString::from(&name))
                .collect()
        };

        let sources = glob_to_qstrings(glob("scripts/*-source.fnl"));
        let transforms = glob_to_qstrings(glob("scripts/*-transform.fnl"));
        let filters = glob_to_qstrings(glob("scripts/*-filter.fnl"));

        let mut self_mut = self.as_mut().cxx_qt_ffi_rust_mut();

        self_mut.sources_total = sources.len();
        self_mut.transforms_total = transforms.len();
        self_mut.filters_total = filters.len();

        self_mut.sources = sources;
        self_mut.transforms = transforms;
        self_mut.filters = filters;
    }
}

