#!/bin/zsh
# Baut das Prüfprogramm und startet es. Wegwerf-Prüfcode, siehe README.md.
#
#   ./starten.sh A     Durchgang A, Ergebnis in messung-A.txt
#   ./starten.sh B     Durchgang B, Ergebnis in messung-B.txt

set -e
verzeichnis=${0:A:h}
etikett=${1:-A}

echo "Baue beobachter …"
swiftc -o "$verzeichnis/beobachter" "$verzeichnis/beobachter.swift"

echo "Starte Durchgang $etikett. Beenden mit Cmd+Q im Fenster oder Ctrl+C hier."
"$verzeichnis/beobachter" "$etikett"
