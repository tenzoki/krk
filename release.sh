#!/bin/sh
#
# Der Auslieferungsweg in einem Kommando mit einem Argument:
#
#     ./release.sh 0.2.0
#
# **Dies ist kein drittes Bauwerkzeug.** Das Projekt hat eines, xtask, und eine
# Huelle darum, das Makefile. Dieses Skript ist die zweite Huelle und traegt
# keine Logik: es prueft, dass genau ein Argument dasteht, und reicht weiter.
# Jede Schicht darunter fuegt genau eine Sache hinzu, und keine zweimal:
#
#     ./release.sh 0.2.0
#       └─ make ausliefern VERSION=0.2.0        Pfad zu cargo, Notarprofil,
#            ├─ cargo xtask version 0.2.0       Reihenfolge der zwei Schritte
#            └─ cargo xtask release             ← hier steht die ganze Logik
#
# Warum es unten zwei Kommandos sind und nicht eines, steht im Modulkopf von
# xtask/src/version.rs: xtask liest die Versionszahl beim Uebersetzen, also muss
# zwischen dem Setzen der Zahl und dem Bauen des Buendels ein Prozess enden.
#
# Was dieses Skript **nicht** tut: es faengt keinen Fehler ab und fasst keine
# Meldung zusammen. Die Meldungen von xtask nennen die Bedingung, die Zahl und
# den Handgriff; ein Skript, das sie durch ein eigenes "Auslieferung
# fehlgeschlagen" ersetzte, naehme dem Nutzer genau die Auskunft weg, um die
# jene Meldungen gebaut sind.

set -eu

if [ "$#" -ne 1 ]; then
    echo "Aufruf: ./release.sh <version>        z.B. ./release.sh 0.2.0" >&2
    echo "Genau ein Argument, die Versionszahl. Alles Weitere folgt daraus." >&2
    exit 2
fi

exec make -C "$(dirname "$0")" ausliefern VERSION="$1"
