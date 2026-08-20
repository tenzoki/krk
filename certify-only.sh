#!/bin/sh
#
# Ein bereits gebautes target/KRK.app allein beglaubigen, in einem Kommando mit
# einem Argument:
#
#     ./certify-only.sh 0.5.5
#
# **Dies ist kein drittes Bauwerkzeug.** Das Projekt hat eines, xtask, und eine
# Huelle darum, das Makefile. Dieses Skript ist die zweite Huelle und traegt
# keine Logik: es prueft, dass genau ein Argument dasteht, und reicht weiter.
# Dieselbe Schichtung wie bei ./release.sh, und jede Schicht fuegt genau eine
# Sache hinzu:
#
#     ./certify-only.sh 0.5.5
#       └─ make beglaubigen VERSION=0.5.5     Pfad zu cargo, Notarprofil
#            └─ cargo xtask beglaubigen 0.5.5 ← hier steht die ganze Logik
#
# **Wofuer dieser Weg da ist.** Fuer den Lauf, der erst an der Beglaubigung
# gescheitert ist — etwa am Zeitueberlauf des Uploads zu Apple —, waehrend das
# universelle, mit Developer-ID und gehaerteter Laufzeitumgebung signierte
# Buendel fertig unter target/KRK.app liegt und allein das Ticket fehlt. Ein
# zweites ./release.sh <zahl> braeche in dieser Lage an Station 1 ab, weil der
# Tag v<zahl> nach dem Lauf nicht mehr allein auf HEAD steht.
#
# **Und darin liegt seine Grenze.** Dieser Weg prueft weder Tag noch
# Arbeitsbaum; ein so beglaubigtes Buendel ist nicht durch die Vorpruefungen
# der Auslieferungskette gegangen. Wer von Grund auf ausliefert, nimmt
# ./release.sh <version>.
#
# Was dieses Skript **nicht** tut: es faengt keinen Fehler ab und fasst keine
# Meldung zusammen; die Meldungen von xtask nennen die Bedingung, die Zahl und
# den Handgriff. Derselbe Grund wie bei ./release.sh, und dort steht er
# ausgeschrieben.

set -eu

if [ "$#" -ne 1 ]; then
    echo "Aufruf: ./certify-only.sh <version>   z.B. ./certify-only.sh 0.5.5" >&2
    echo "Genau ein Argument, die Versionszahl des gebauten Buendels." >&2
    exit 2
fi

exec make -C "$(dirname "$0")" beglaubigen VERSION="$1"
