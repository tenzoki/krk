# Planner: Nacharbeit am Plan der Runde 4, zwei Auflagen des Nutzers

**Datum:** 2026-08-11, 17:21
**Status:** Complete
**Agent:** planner
**Bearbeitete Datei:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/planning/260811-1648_o_plan-vier-tastenbefehle-pfade-kopieren-oeffnen.md` (Marker bleibt `_o_`)
**Grundlage:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/reviews/260811-1704-conceptrev-plan-vier-tastenbefehle-pfade-kopieren-oeffnen.md`, Spruch `acceptable`; Abnahme des Nutzers am 260811-1710 mit zwei Auflagen

## Was der Auftrag war

Der Nutzer hat den Plan abgenommen und drei Nacharbeiten verlangt, ausdrücklich und ausschließlich diese: die fehlende Kante `S2 → S3` im Schrittgraphen samt der Prosa, die sie nennt; die sechs Datenflusskanten im ersten Bild, deren Vorbemerkung Aufrufbeziehungen ankündigt; und die Nutzerantwort auf die Frage nach einer Schwelle vor dem Öffnen, die der Plan bis dahin als offen führte. Kein Code, kein Spec, kein Circle-Datensatz, kein Commit.

## Auflage 1: die Kante `S2 → S3`

Der Befund der Bewertung trägt am Code nachgeprüft: `nichts_betroffen` entsteht in S2 und wird von S3 gerufen, und `grep -rn "nichts_betroffen" crates/` findet im Baum allein den Namen einer bestehenden Probe. Die Kante ist eingezogen, die transitive Abkürzung `S2 --> AB` dafür gefallen. Der Graph bleibt bei 6 Knoten und 6 Kanten.

Von den drei Prosastellen, die der Nutzer prüfen ließ, waren zwei unvollständig und sind mitgezogen: die Vorbemerkung über dem zweiten Bild, die von drei Abhängigkeitskanten sprach und nun vier einzeln benennt, und die Kopfzeile von S3, die nun `S1 und S2` führt und den Grund nennt. Die Zuordnungstabelle brauchte keine Änderung; die Begründung steht im Bericht an den Nutzer.

## Auflage 2: das erste Bild

Gewählt ist der Weg, das Bild an die Vorbemerkung anzupassen statt umgekehrt. Alle sechs Datenflusskanten laufen jetzt über ihren wirklichen Rufer, die `DateifensterQuelle`. Ein Knoten ist dazugekommen, `die beiden Kopiermethoden daneben`, den die Dateiliste von S2 bereits führt; ohne ihn hätte `kommando_ausfuehren` einen Ausgangsgrad von sieben getragen.

Messwerte vorher 16 Knoten, 17 Kanten, Dichte 1,06, Ausgangsgrad höchstens 3; nachher 17 Knoten, 18 Kanten, Dichte 1,06, Ausgangsgrad höchstens 6. Beide Blöcke sind mit `mmdc` 11.16.0 nach PNG gerendert und angesehen: alle Kanten laufen abwärts, keine kreuzt, die aufwärts laufenden Bögen der vorigen Fassung sind fort. Keine Kante hat mehr ihren Ursprung im Kasten `kommandos/operationen.rs`; dessen drei Knoten sind Senken, womit die Zusage aus Frage 6 als Struktur im Bild steht.

## Die beantwortete Frage

`decisions/260811-1648_*_fragt-krk-nach-bevor-return-viele-eintraege-oeffnet.md` steht seit dem 260811-1710 auf beantwortet: keine Nachfrage. Drei Stellen des Plans führten sie als offen und tun es nicht mehr, die Zeile in der Risikotabelle, der Eintrag unter `## Angelegte Datensätze` und der Punkt unter `## Offene Fragen`, der ersatzlos entfällt. Der Kopf des Plans führte sechs bindende Datensätze und führt nun sieben. In S3 kam die Frage nicht vor. Kein Abnahmekriterium hat sich geändert.

## Zwei Beobachtungen, die nicht angefasst sind

Der Defekt `issues/260811-1648_o_fuenf-entscheidungsdatensaetze-tragen-im-rumpf-noch-den-stand-offen.md` zählt fünf von sechs; mit dem siebten Datensatz, der im Rumpf ebenfalls `**Status:** open` trägt, sind es sechs von sieben. Die Verweise auf die beiden Defektdateien unter `## Angelegte Datensätze` schreiben den Zustandsmarker aus, statt die Sternstelle der eigenen Verweisregel des Plans zu setzen.

## Setup-Notiz

`bin/fusion-rules planner` hat für diesen Lauf keinen Pfad auf ein Stilprofil ausgegeben, weder `chat-voice-de.yaml` noch `default-voice-de.yaml`, obwohl beide unter `fusion-workbench/stilwerk/` liegen. Beide sind unmittelbar gelesen und angewandt.
