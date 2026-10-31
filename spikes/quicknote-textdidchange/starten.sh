#!/bin/zsh
# Baut die zwei Pruefprogramme und startet sie. Wegwerf-Pruefcode, siehe
# messungen/261031-1300-quicknote-textdidchange.txt.
#
# Die Programme landen in einem frischen Ordner unter $TMPDIR und nicht im
# Spike-Verzeichnis: so bleibt der Arbeitsbaum sauber, und `cargo xtask
# release` schlaegt nach einem Lauf nicht an (`stand_pruefen`).

set -e
verzeichnis=${0:A:h}
ausgabe=$(mktemp -d "${TMPDIR:-/tmp}/krk-spike-quicknote.XXXXXX")
trap 'rm -rf "$ausgabe"' EXIT

for name in mit_fenster ohne_schluesselfenster; do
  echo "== $name"
  swiftc -O -o "$ausgabe/$name" "$verzeichnis/$name.swift"
  "$ausgabe/$name"
done
