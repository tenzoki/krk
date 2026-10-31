#!/bin/zsh
# Baut die zwei Pruefprogramme und startet sie. Wegwerf-Pruefcode, siehe
# messungen/261031-1300-quicknote-textdidchange.txt.

set -e
verzeichnis=${0:A:h}

cd "$verzeichnis"
for name in mit_fenster ohne_schluesselfenster; do
  echo "== $name"
  swiftc -O -o "$name" "$name.swift"
  "./$name"
done
