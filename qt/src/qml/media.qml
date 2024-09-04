import QtQuick 2.12
import QtQuick.Window 2.12
import QtQuick.Controls 2.12

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


    GridView {
        anchors.fill: parent
        id: view

        cellWidth: 128
        cellHeight: 128

        model: mediaModel
        delegate: Image {
            required property url url
            width: view.cellWidth
            height: view.cellHeight
            asynchronous: true
            cache: false
            fillMode: Image.PreserveAspectCrop
            source: url
        }

        boundsBehavior: Flickable.StopAtBounds
        flickDeceleration: 5000
        maximumFlickVelocity: 5000

        Behavior on contentY {
            NumberAnimation {
                duration: animDuration
                easing.type: Easing.Linear
            }
        }

        ScrollBar.vertical: ScrollBar {}

        CustomScroll {
            target: view
        }
    }
}
