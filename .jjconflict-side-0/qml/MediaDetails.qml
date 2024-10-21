import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12

Popup {
    // dont even ask
    // @TODO remove this when json.parse is removed from MediaList.qml
    property var current: JSON.parse('{"title": "", "uri": "", "type": "Image", "width": 0, "height": 0, "history": []}')
    property real imageMaxWidth: 0.65

    modal: true
    focus: true
    anchors.centerIn: Overlay.overlay
    width: window.width * 0.7
    height: window.height * 0.5
    onAboutToShow: {
        // @TODO fix markdown injection
        let text = [];
        text.push(`Title: *${current.title}*`);

        switch (current.type) {
            case 'Image':
                text.push(`Width: *${current.width}*`);
                text.push(`Height: *${current.height}*`);
                break;
            case 'PDF':
                text.push(`Author: *${current.author}*`);
                const trimAt = 100
                const truncated = current.summary.length > trimAt ? `${current.summary.substring(0, trimAt)}...` : current.summary;
                text.push(`Summary: *${truncated.trim()}*`);
                break;
        }

        text.push('History:<br />' + current.history.map(f => `*${f}*`).join('<br />'));

        if (current.tags.length > 0) text.push('Tags:<br />' + current.tags.map(f => `*${f}*`).join('<br />'));

        details.text = text.join('<br /><br />');
        media.media = current;
    }

    RowLayout {
        anchors.fill: parent

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
            Layout.fillHeight: true
            Layout.maximumWidth: parent.width * (1 - imageMaxWidth)
            verticalAlignment: Text.AlignVCenter
            horizontalAlignment: Text.AlignRight
            textFormat: Text.MarkdownText
            wrapMode: Text.Wrap
            clip: true
        }
    }
}
