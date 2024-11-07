import QtQuick
import QtQuick.Controls
import QtQuick.Window
import sinkfrontend

ApplicationWindow {
    height: 480
    width: 640
    title: qsTr("sink-frontend")
    visible: true

    ListModel {
        id: model
    }

    ActionsContextMenu {
        id: contextMenu
    }

    MediaDetails {
        id: mediaDetails
    }

    SpinBox {
        id: offset
        stepSize: MediaList.render_amount
        to: 0
        onValueModified: {
            MediaList.offset = value;
            MediaList.rerender();
        }
    }

    Text {
        id: total
        anchors.top: offset.bottom
        anchors.horizontalCenter: offset.horizontalCenter
        text: "0"
    }

    CustomScrollGridView {
        property int spacing: 4

        height: parent.height
        anchors.horizontalCenter: parent.horizontalCenter
        width: Math.floor(parent.width / cellWidth) * cellWidth

        id: grid
        activeFocusOnTab: true
        focus: true
        clip: true
        cellWidth: 296
        cellHeight: cellWidth
        cacheBuffer: cellHeight
        model: model

        delegate: Media {
            width: grid.cellWidth - grid.spacing
            height: grid.cellHeight - grid.spacing
            focused: GridView.isCurrentItem
        }

        highlight: Item {}
    }

    Connections {
	target: MediaList

	function onAppend(media) {
	    model.append({ media });
	}

        function onTotalChanged() {
            total.text = MediaList.total.toString();
            offset.to = Math.floor(MediaList.total / MediaList.render_amount) * MediaList.render_amount;
        }

        function onClear() {
            model.clear();
        }
    }
}
