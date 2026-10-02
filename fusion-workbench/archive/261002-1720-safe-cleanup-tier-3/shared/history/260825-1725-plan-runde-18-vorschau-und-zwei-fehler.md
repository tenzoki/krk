# Planung der Runde 18: die Vorschau vertieft, und zwei Fehler

**Status:** Complete
**Agent:** planner
**Datum:** 2026-08-25
**Kein aktiver Circle**, also liegt alles im gemeinsamen Speicher (Herkunftsregel).

## Auftrag

Der Nutzer hat einen Rohauftrag gegeben und das Schärfen ausdrücklich übersprungen, mit der
Bitte, die offenen Fragen selbst und überschneidungsfrei zu beantworten. Fünf Erweiterungen der
Profil-Zusammenfassung aus der Runde 16 und zwei Fehler, die er als Regressionen bezeichnet
hat.

## Ergebnis

**Plan:** `shared/planning/260825-1725_o_plan-vorschau-vertieft-und-zwei-fehler.md`, zehn
Schritte in drei Strängen, acht an `coder`, einer an `ontocoder`, einer an `analyst`.

**Sechs Entscheidungsdatensätze**, alle in `shared/decisions/` und alle mit einer Empfehlung:

| Datei (Stempel `260825-1725`) | Empfehlung |
|---|---|
| `_o_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner` | ein Platzhalter in der Ortsangabe, kein fünfter Baustein und keine Tiefenangabe |
| `_o_liest-eine-zusammenfassung-denselben-unterordner-einmal-oder-je-zeile` | ein Ort, eine Lesung; kehrt eine Festlegung der Runde 16 um |
| `_o_wie-kommt-ein-aenderungsdatum-in-eine-profilzeile` | `zeigt` an `juengste`, kein fünfter Baustein und kein siebter `Wert` |
| `_o_wo-wohnt-die-umrechnung-von-systemtime-in-buergerliche-ortszeit` | `localtime_r(3)` als sechste Schnittstelle der Systemschicht |
| `_o_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist` | für jeden Ordner, nicht nur für die Projektwurzel |
| `_o_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat` | Handgriff jetzt, Befehl in einer späteren Runde |
| `_o_nimmt-ein-klick-auf-die-tableiste-des-anderen-dateifensters-den-ersthelferrang-mit` | ja, aber als eigener Schritt |

Kein Defektdatensatz. Die zwei Fehler des Auftrags sind bereits abgelegt oder keine Neufunde:
der Zip-Zeitstempel steht als `circles/260825-0711-…/issues/260825-0838_o_…` und wird von
Schritt 3 abgearbeitet; der Klick-Fokus bekommt keinen eigenen Datensatz, weil der Plan ihn in
demselben Zug behebt.

## Was die Planung gemessen hat

Zwei Untersuchungen liefen parallel zur Planung, beide mit Belegen am Baum.

**Der Klick-Fokus ist keine Regression.** Die Entkopplung ist so alt wie der Tab-Befehl
(`537fda5`, 260804); kein Commit der Runden 14 bis 17 hat den Klickweg angefasst. KRK führt
zwei Fokusgrößen, `Fenstermodell::aktiv` und den Ersthelferrang von AppKit, und der Tab-Zweig
(`anwendung.rs:3172`) schreibt die erste ohne die zweite. Danach ist der Melderweg tot, den
`76ceb68` am 260819 hinzugefügt hat: an einem Nachbau gemessen wird `makeFirstResponder:` gar
nicht erst gerufen, wenn die geklickte Ansicht den Rang schon hält. Die Behebung ist eine
Zeile, und ihr Beleg ist eine Zählprobe am Quelltext nach dem Muster der `zettelproben`.

**Zwei der drei Vorschläge des Zip-Datensatzes tragen nicht.** Das Merkmal `time` von `zip`
liefert die Uhrzeit des Packens statt des Änderungsdatums der Quelle und bringt an Umrechnung
nur `TryFrom<PrimitiveDateTime>`, also eine Zeit ohne Zone; gebraucht wird es nicht, weil
`DateTime::from_date_and_time` und `last_modified_time` merkmalsfrei sind. Und das erweiterte
Zeitfeld 0x5455 allein genügt auf macOS nicht: `ditto(1)` übergeht es nachweislich und liest
0x5855. Gemessen an vier Archiven über dieselbe Quelle wirkt nur die Kombination aus richtigem
MS-DOS-Feld, 0x5455 und 0x5855; das dafür nötige Merkmal ist `unreserved` und nicht `time`, es
ist als `unreserved = []` deklariert und bringt weder eine Abhängigkeit noch C-Code mit. Diese
drei Befunde trägt Schritt 3 als Nachtrag in den vorhandenen Datensatz ein.

## Die tragende Entwurfsentscheidung

Die Auskunft „wie viele offene Defekte gibt es über alle Runden" kostet eine Verzeichnisöffnung
je Runde, und die Zahl der Runden wächst. Ein fester Deckel auf Verzeichnisleseläufe kann sie
deshalb nie dauerhaft tragen: heute wären 39 nötig, bei hundert Runden 201, gegen einen Deckel
von zwölf.

Der Ausweg ist nicht ein größerer Deckel, sondern eine andere Einheit. Ein Platzhalter-Lauf
bucht **einen** Leselauf und wird durch `HOECHSTENS_EINTRAEGE` begrenzt, also durch die Zahl
der gelesenen Einträge statt der geöffneten Verzeichnisse. An der heutigen Werkbank ist die
Auskunft damit exakt (568 von 2.000 Einträgen); bei rund hundert Runden wird sie unvollständig
und sagt es selbst, über die Vokabel `Wert::UeberGrenze`, die die Runde 16 dafür geschrieben
hat. Mit dieser Änderung und der Merkung je Ort passen alle acht Profile unter die Zwölf, ohne
dass eine Zusage steigt.

## Was der Nutzer als Nächstes tut

Er liest den Plan und die sechs Datensätze und entscheidet. Nicht ausgeführt wird nichts: die
Planung endet hier, und die Umsetzung beginnt erst auf seine Freigabe.

Ein Punkt gehört dabei vor die Freigabe und nicht danach: **eine Änderung an
`resources/default-readers.toml` erreicht ihn nicht**, weil KRK die Nutzerdatei nach ihrer
Anlage nie überschreibt. Ohne den Handgriff aus Schritt 9 zeigt KRK nach dieser Runde die fünf
Profile von gestern, und zwar ohne Meldung.
