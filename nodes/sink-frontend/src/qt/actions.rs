#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qvector.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qmap.h");
        type QMap_QString_QVariant = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(QMap_QString_QVariant, actions)]
        type Actions = super::ActionsRust;

        #[qinvokable]
        fn execute(self: Pin<&mut Actions>, media: QMap_QString_QVariant, action: QString);
    }

    impl cxx_qt::Constructor<()> for Actions {}
}

use std::{pin::Pin, process::{Command, Stdio}, thread};

use cxx_qt_lib::QVariantValue;
use qobject::{QMap_QString_QVariant, QString, QUrl, QVariant};

use crate::{config::CONFIG, media::{self}};

#[derive(Default)]
pub struct ActionsRust {
    actions: QMap_QString_QVariant
}

fn convert_option_variant<T, E>(option: Option<QVariant>, conv: impl FnOnce(E) -> T) -> T
where T: Default, E: QVariantValue {
    option.map(|variant| conv(variant.value_or_default::<E>())).unwrap_or_default()
}

impl qobject::Actions {
    fn execute(self: Pin<&mut Self>, media: QMap_QString_QVariant, action: QString) {
        let action_command: String = match self.actions.get((&action).into()) {
            Some(action) => action,
            None => return eprintln!("action {:?} does not exist", action),
        }.value_or_default::<QString>().to_string();

        let media = media::Media {
            url: convert_option_variant::<_, QUrl>(media.get(&"url".into()), |u| u.to_string()).to_string(),
            essential_metadata: media::EssentialMetadata {
                title: convert_option_variant::<_, QString>(media.get(&"title".into()), |s| s.into()),
                ..Default::default()
            },
            ..Default::default()
        };

        let media_json = serde_json::to_string(&media).expect("could not serialize media");

        let (shell, flag) = if cfg!(windows) {
            ("cmd", "/c")
        } else {
            ("sh", "-c")
        };

        let arg = action_command.replace("%", &media_json);
        let result = Command::new(shell)
            .args([flag, arg.as_str()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn();
        if let Err(error) = result {
            eprintln!("could not execute action: {}", error)
        }
    }
}

impl cxx_qt::Initialize for qobject::Actions {
    fn initialize(mut self: Pin<&mut Self>) {
        while let None = CONFIG.get() {
            thread::yield_now();
        }

        let mut actions = QMap_QString_QVariant::default();

        CONFIG
            .get()
            .unwrap()
            .actions
            .clone()
            .into_iter()
            .map(|action| (action.name.into(), (&QString::from(action.command)).into()))
            .for_each(|(name, command)| actions.insert(name, command));

        self.as_mut().set_actions(actions);
    }
}
