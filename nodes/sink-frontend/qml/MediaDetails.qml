import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import sinkfrontend

Popup {
    property real imageMaxWidth: 0.65

    padding: 0
    rightPadding: 10
    background: Rectangle {
        color: palette.alternateBase
    }

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

    function setData() {
        let text = Object.assign({}, grid.currentItem.media);

        for (const [k,] of Object.entries(text)) {
            if (k.toLowerCase().includes('url')) {
                delete text[k];
            }
        }

        if (text.summary) {
            const trimAt = 100;
            const truncated = text.summary.length > trimAt ? `${text.summary.substring(0, trimAt)}...` : text.summary;
            text.summary = truncated;
        }

        details.text = renderText(text);
        media.media = grid.currentItem.media;
    }

    modal: true
    focus: true
    anchors.centerIn: Overlay.overlay
    width: ApplicationWindow.window.width * 0.7
    height: ApplicationWindow.window.height * 0.5
    onAboutToShow: setData()

    RowLayout {
        anchors.fill: parent
        focus: true

        Keys.onLeftPressed: grid.moveCurrentIndexLeft() || setData()
        Keys.onRightPressed: grid.moveCurrentIndexRight() || setData()
        Keys.onDownPressed: grid.moveCurrentIndexDown() || setData()
        Keys.onUpPressed: grid.moveCurrentIndexUp() || setData()

        Media {
            id: media
            Layout.alignment: Qt.AlignTop
            Layout.preferredWidth: Math.max(parent.width * imageMaxWidth,  parent.height)
            Layout.fillHeight: true

            media: {}
            focusEnabled: false
            mipmap: false
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
        }
    }
}
