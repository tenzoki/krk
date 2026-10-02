# Der Weg zu einer neuen Profildatei steht im README

**Agent:** coder
**Datum:** 2026-08-25, 20:52 bis 20:58
**Aufgabe:** T-9, Schritt 9 des Plans
`fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`,
Strang 3 — „Der Weg zu einer neuen Profildatei steht im `README.md`"
**Status:** Complete

## Was entstanden ist

`README.md` trägt einen neuen Abschnitt `## Neue Leseprofile übernehmen`, eingefügt als
Zeilen 44 bis 73, unmittelbar hinter dem Abschnitt zum Installieren und **vor** dem
Trennstrich, ab dem sich der Text an den richtet, der KRK baut und ausliefert. Der Ort ist
die Aussage: der Handgriff ist Nutzerarbeit und keine Bauarbeit.

Vier Absätze, in dieser Reihenfolge:

1. Was ein Leseprofil überhaupt tut und wo die Datei liegt. Der Begriff kam im `README.md`
   bis heute nicht vor; ohne den Satz stünde der Handgriff ohne Gegenstand da.
2. **Was ohne den Handgriff geschieht.** `anlegen_falls_fehlt` schreibt die
   Auslieferungsfassung nur bei fehlender Datei; wer KRK schon einmal gestartet hat, sieht
   nach der Installation weiter die Profile von vorher, ohne jede Meldung. Der Absatz sagt
   auch, warum keine Meldung am Platz wäre: eine unveränderte Datei ist nicht beschädigt.
3. Die drei Schritte als nummerierte Liste: beenden, beiseitelegen, starten.
4. **Was das Löschen kostet.** Eigene Profile und eigene Änderungen an den ausgelieferten
   stehen in der alten Datei und in keiner zweiten; die neu angelegte kennt nur die
   Auslieferungsfassung.

## Zwei Entscheidungen beim Schreiben

**Der Abschnitt nennt keine Zahl.** Der Plan spricht von „den drei neuen Profilen dieser
Runde", der Nutzer hat nach der Freigabe vier Profile für flight nachbestellt, und
`resources/default-readers.toml` führt seit `5595026` zwölf statt fünf. Eine Zahl an dieser
Stelle wäre mit der nächsten Runde falsch, und die Gewohnheit dieses Projekts gegen solche
Zahlen ist in `CLAUDE.md` mehrfach ausgeschrieben. Der Abschnitt spricht deshalb von „neuen
Profilen" und lässt das Zählen der Datei.

**Der Text lehnt sich in Wortwahl und Begründung an den Abschnitt zum Installieren an.**
„Was KRK sich merkt, liegt außerhalb des Bündels, und ein Handgriff, der es mitnimmt, hat
es genommen" ist dieselbe Regel wie „die alte Fassung vorher nicht löschen", auf einen
zweiten Gegenstand angewandt; der Abschnitt sagt das ausdrücklich („derselbe Grund wie beim
Installieren"), statt die Begründung ein zweites Mal von vorn aufzubauen.

## Was ausdrücklich nicht angefasst wurde

`RELEASETEXT` in `xtask/src/veroeffentlichung.rs`. Er trägt die Installationsregel, und
jede seiner Aussagen hängt an einer eigenen Behauptung der Probe
`der_releasetext_traegt_jede_seiner_aussagen`. Der Handgriff hier ist ein anderer
Gegenstand und gehört nicht auf die Releaseseite.

Ebenso unberührt: `resources/`, `crates/`, `Cargo.toml`, `CLAUDE.md` und der
Analysespeicher, in dem parallel Schritt 10 schreibt.

## Abnahme

Die drei Kriterien des Plans:

| Kriterium | Stand |
|---|---|
| voller Pfad, drei Schritte in dieser Reihenfolge, ausdrücklich was ohne den Handgriff geschieht | erfüllt, `README.md:44-73` |
| sagt, was verloren geht, wenn der Nutzer löscht statt beiseitelegt | erfüllt, letzter Absatz |
| `make check` grün | erfüllt |

`Verification: make check — exit 0` (mit `PATH="$HOME/.cargo/bin:$PATH"`; alle vier
Kommandos grün, `cargo fmt --all --check` schreibt nichts).

## Was offen bleibt

**Die Vorbedingung des Plans ist damit nicht erfüllt, sondern nur beschreibbar geworden.**
Die Schließungsbedingung verlangt, dass der Nutzer den Handgriff an seinem Gerät ausgeführt
und die neuen Profile in KRK gesehen hat. Kein Agent kann das tun. Bis dahin zeigt KRK auf
dem Entwicklungsgerät die fünf Profile vom 260824.

Möglichkeit 2 des Datensatzes
`shared/decisions/260825-1725_a_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`
— ein Menübefehl „Leseprofile auf die Auslieferungsfassung zurücksetzen" — ist vom Nutzer
am 260825-1740 ausdrücklich einer späteren Runde zugewiesen und bleibt es.
