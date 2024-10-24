import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    function renderText(obj) {
        const output = [];
        for (let [k, v] of Object.entries(obj)) {
            k = k.toString();
            // uppercase first letter
            const sk = k.charAt(0).toUpperCase() + k.slice(1);
            const renderValue = v => `*${v.toString().trim()}*`;

            let sv = '';
            if (v?.map) {
                if (v.length < 1) continue;
                sv += '<br/>' + v.map(renderValue).join('<br/>');
            } else {
                sv = ' ' + renderValue(v);
            }

            output.push(`${sk}:${sv}`);
        }

        return output.join('<br/><br/>');
    }

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
        let text = {};
        text.title = current.title;

        switch (current.type) {
            case 'Image':
                text.width = current.width;
                text.height = current.height;
                break;
            case 'PDF':
                text.author = current.author;
                const trimAt = 100;
                const truncated = current.summary.length > trimAt ? `${current.summary.substring(0, trimAt)}...` : current.summary;
                text.summary = truncated;
                break;
        }

        text.history = Object.keys(current.history);
        text.tags = current.tags;

        details.text = renderText(text);
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
