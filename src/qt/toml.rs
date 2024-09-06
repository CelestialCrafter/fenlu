#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qmap.h");
        type QMapPair_QString_QVariant = cxx_qt_lib::QMapPair_QString_QVariant;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QMapPair_QString_QVariant, map)]
        #[qproperty(QString, path)]
        type FenluToml = super::Toml;
    }

    impl cxx_qt::Constructor<()> for FenluToml {}
}

use std::{fs, pin::Pin};

use cxx_qt_lib::{QMapPair, QMapPair_QString_QVariant, QString, QVariantValue};
use toml::{Table, Value};

#[derive(Default)]
pub struct Toml {
    map: QMapPair_QString_QVariant,
    path: String
}

impl qobject::FenluToml {
    pub fn load_table(self: Pin<&mut Self>, table: Table) {
        let mut qmap = QMapPair_QString_QVariant::default();
        table.iter().for_each(|(k, v)| {
            let variant = match v {
                Value::Float(f) => QVariantValue::construct(&f),
                Value::String(s) => QVariantValue::construct(&QString::from(s)),
                Value::Integer(i) => QVariantValue::construct(&i),
                Value::Boolean(b) => QVariantValue::construct(&b),
                _ => QVariantValue::construct(&false)
            };
            qmap.insert(k.into(), variant);
        });

        self.set_map(qmap);
    }
}

impl cxx_qt::Initialize for qobject::FenluToml {
    fn initialize(self: Pin<&mut Self>) {
        self.load_table(
            toml::from_str(
                fs::read_to_string(self.path).expect("should be able to read toml file").as_str()
            ).expect("should be able to parse toml")
        );
    }
}
