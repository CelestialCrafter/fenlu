import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12

Popup {
    // dont even ask
    property var current: JSON.parse('{"title": "", "uri": "", "type": "Image", "width": 0, "height": 0, "history": []}')
    property list<string> details: []

    modal: true
    focus: true
    anchors.centerIn: Overlay.overlay
    width: window.width * 0.7
    height: window.height * 0.3
    onAboutToShow: {
        media.media = current;

        details = [];
        details.push(`Title: ${current.title}`);
        if (current.tags.length > 0) details.push(`Tags: ${current.tags.join(', ')}`);
        details.push(`History: ${current.history.join(', ')}`);

        switch (current.type) {
            case 'Image':
                details.push(`Width: ${current.width}`);
                details.push(`Height: ${current.height}`);
                break;
            case 'PDF':
                details.push(`Author: ${current.author}`);
                details.push(`Summary: ${current.summary}`);
                break;
        }
    }

    RowLayout {
        anchors.fill: parent
        states: [
            State {
                when: width / 2 < height
                PropertyChanges {
                    target: media
                    Layout.preferredWidth: parent.width * 0.5
                    Layout.preferredHeight: parent.width * 0.5
                }
            },
            State {
                when: width / 2 >= height
                PropertyChanges {
                    target: media
                    Layout.preferredWidth: parent.height
                    Layout.preferredHeight: parent.height
                }
            }
        ]

        Media {
            media: current
            id: media
        }

        Text {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            text: details.join('\n')
            verticalAlignment: Text.AlignVCenter
            horizontalAlignment: Text.AlignRight
        }
    }
}
