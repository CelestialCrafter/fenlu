import QtQuick

Rectangle {
    property bool focused: true
    property bool minor: false

    border.color: palette.highlight
    border.width: !minor * focused * 2

    Rectangle {
        anchors.fill: parent
        color: minor ? palette.mid : palette.highlight
        opacity: focused * 0.5 + (minor * 0.5)
    }
}

