#!/bin/zsh
# Baut das Prüfprogramm und startet es. Wegwerf-Prüfcode, siehe README.md.
# Schreibt messung.txt neben das Programm.

set -e
verzeichnis=${0:A:h}

cd "$verzeichnis"
echo "Baue quicknote …"
swiftc -o quicknote quicknote.swift
./quicknote
