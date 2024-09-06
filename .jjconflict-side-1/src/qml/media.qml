import QtQuick 2.12
import QtQuick.Window 2.12

import com.github.CelestialCrafter.fenlu 1.0

Window {
    height: 480
    title: qsTr("Fenlu")
    visible: true
    width: 640
    id: window

    FenluMedia {
        id: media
    }

    ListModel {
        id: mediaModel
    }

    Connections {
        target: media
        property var previousTotal: 0

        function onTotalChanged() {
            for (let i = previousTotal; i < media.total; i++) {
                mediaModel.append({ url: media.item(i) });
            }

            previousTotal = media.total;
        }
    }


    CustomScrollGridView {
        property var columns: 5

        anchors.fill: parent
        id: grid

        cellWidth: grid.width / columns
        cellHeight: grid.cellWidth
        model: mediaModel
        cacheBuffer: grid.cellHeight * columns

        delegate: Image {
            required property url url
            width: grid.cellWidth
            height: grid.cellHeight
            asynchronous: true
            cache: false
            fillMode: Image.PreserveAspectCrop
            source: url
        }
    }
}
