import QtQuick 2.12
import QtQuick.Controls 2.12
import fenlu

Image {
    required property var data

    //MediaDetails {
    //    id: popup
    //    data: data
    //}

    asynchronous: true
    fillMode: Image.PreserveAspectCrop
    source: data.uri

    Label {
        horizontalAlignment: Text.AlignHCenter
        anchors.top: parent ? parent.top : undefined;
        width: parent ? parent.width : 0;
        topPadding: 5
        bottomPadding: 5

        text: data ? data.title : ""
        color: "white"

        background: Rectangle {
            opacity: 0.7
            color: "black"
        }
    }

    MouseArea {
        anchors.fill: parent ? parent : undefined;
        //onClicked: popup.open();
        onDoubleClicked: FenluMedia.open(data.uri);
    }
}
