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

    CustomScrollGridView {
        property int spacing: 4

        anchors.fill: parent
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

	function onRecvNew(media) {
	    model.append({ media })
	}
    }
}
