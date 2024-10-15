import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12

Popup {
    // dont even ask
    property var current: JSON.parse('{"title": "", "uri": "", "width": 0, "height": 0, "history": []}')

    modal: true
    focus: true
    anchors.centerIn: Overlay.overlay
    width: window.width * 0.9
    height: window.height * 0.9

    RowLayout {
        anchors.fill: parent
        Media {
            id: media
            media: current
        }

        ColumnLayout {
            ListView {
                Layout.fillHeight: true
                Layout.fillWidth: true
                id: history
                model: ListModel {}
                delegate: Text {
                    required property string location
                    text: location
                }
            }
        }
    }

    onAboutToShow: {
        media.media = current;
        history.model.clear();
        for (let i = 0; i < current.history.length; i++) {
            history.model.append({ location: current.history[i] });
        }
    }

}
