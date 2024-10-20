import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12

Popup {
    // dont even ask
    property var current: JSON.parse('{"title": "", "uri": "", "type": "Image", "width": 0, "height": 0, "history": []}')
    property real imageMaxWidth: 0.65

    modal: true
    focus: true
    anchors.centerIn: Overlay.overlay
    width: window.width * 0.7
    height: window.height * 0.5
    onAboutToShow: {
        let text = [];
        text.push(`Title: *${current.title}*`);

        switch (current.type) {
            case 'Image':
                text.push(`Width: *${current.width}*`);
                text.push(`Height: *${current.height}*`);
                break;
            case 'PDF':
                text.push(`Author: *${current.author}*`);
                text.push(`Summary: *${current.summary}*`);
                break;
        }

        text.push('');
        text.push('History:');
        text = text.concat(current.history.map(f => `*${f}*`));

        if (current.tags.length > 0) {
            text.push('');
            text.push('Tags:');
            text = text.concat(current.tags.map(f => `*${f}*`));
        }

        details.text = text.join('<br />');
        media.media = current;
    }

    RowLayout {
        anchors.fill: parent
        clip: true

        Media {
            media: current
            id: media
            Layout.preferredWidth: Math.max(parent.width * imageMaxWidth,  parent.height)
            Layout.maximumHeight: parent.height
            fillMode: Image.PreserveAspectFit
        }

        Text {
            id: details
            Layout.fillWidth: true
            textFormat: Text.MarkdownText
            Layout.fillHeight: true
            clip: true
            Layout.maximumWidth: parent.width * (1 - imageMaxWidth)
            verticalAlignment: Text.AlignVCenter
            horizontalAlignment: Text.AlignRight
        }
    }
}
