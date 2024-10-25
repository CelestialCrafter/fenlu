import QtQuick

Rectangle {
    property bool focused: true
    property bool minor: false

    border.color: "#d7827e"
    border.width: !minor * focused * 2

    Rectangle {
        anchors.fill: parent
        color: minor ? "gray" : "#d7827e"
        opacity: focused * 0.5 - (minor * 0.1)
    }
}

