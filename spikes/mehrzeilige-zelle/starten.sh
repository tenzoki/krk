#!/bin/zsh
# Baut das Prüfprogramm und startet es. Wegwerf-Prüfcode, siehe README.md.
#
#   ./starten.sh ohne          tippt und misst, sonst nichts
#   ./starten.sh note          nach jeder Zeile noteHeightOfRowsWithIndexesChanged:
#   ./starten.sh intrinsisch   davor invalidateIntrinsicContentSize am Feld
#   ./starten.sh jeanschlag    beides bei jedem Anschlag aus controlTextDidChange:
#   ./starten.sh feldeditormass wie jeanschlag, Höhe am Text des Feldeditors gemessen
#   ./starten.sh feldeditorohnenote  wie feldeditormass, ohne noteHeightOfRows…
#   ./starten.sh alle          die sechs nacheinander, dazu `ohne` mit weichem Feld
#
# Ein zweites Argument `weich` lässt den vertikalen Stauchwiderstand des Feldes
# auf seiner Vorgabe.

set -e
verzeichnis=${0:A:h}
weg=${1:-alle}

cd "$verzeichnis"
echo "Baue zelle …"
swiftc -o zelle zelle.swift

if [[ $weg == alle ]]; then
    ./zelle ohne weich
    for d in ohne note intrinsisch jeanschlag feldeditormass feldeditorohnenote; do ./zelle $d; done
else
    ./zelle "$weg" $2
fi
