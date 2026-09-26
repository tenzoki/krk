#!/bin/zsh
# Baut das Prüfprogramm und startet es. Wegwerf-Prüfcode, siehe README.md.
#
#   ./starten.sh vorgabe   der Feldeditor, den AppKit von sich aus einsetzt
#   ./starten.sh eigener   ein eigener Feldeditor aus windowWillReturnFieldEditor:toObject:
#   ./starten.sh eigenerundo  dazu allowsUndo ein
#   ./starten.sh eigenerfaengt dazu beantwortet der Feldeditor undo: und redo: selbst
#   ./starten.sh eigenerfaengt ohnepruefung  ohne eigene Menuepruefung
#   ./starten.sh alle      die vier nacheinander, eigenerfaengt ein zweites Mal ohne Menuepruefung, danach mehrzeilig.swift
#                          (die Form aus 4.2 mit dem eigenen Feldeditor)

set -e
verzeichnis=${0:A:h}
weg=${1:-alle}

cd "$verzeichnis"
echo "Baue rueckgaengig …"
swiftc -o rueckgaengig rueckgaengig.swift
swiftc -o mehrzeilig mehrzeilig.swift

if [[ $weg == alle ]]; then
    for d in vorgabe eigener eigenerundo eigenerfaengt; do ./rueckgaengig $d; done
    ./rueckgaengig eigenerfaengt ohnepruefung
    for d in feldeditormass feldeditorohnenote; do ./mehrzeilig $d; done
else
    ./rueckgaengig "$weg" $2
fi
