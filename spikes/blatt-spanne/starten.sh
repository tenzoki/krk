#!/bin/zsh
# Baut das Prüfprogramm und startet es. Wegwerf-Prüfcode, siehe README.md.
#
#   ./starten.sh griff    schließt das Blatt über endSheet(_:returnCode:)
#   ./starten.sh klick    schließt es über performClick(nil), wie ein Nutzer

set -e
verzeichnis=${0:A:h}
weg=${1:-griff}

cd "$verzeichnis"
echo "Baue spanne …"
swiftc -o spanne spanne.swift

echo "Messe Weg $weg. Das Programm beantwortet sein Blatt selbst und beendet sich."
./spanne "$weg"
