# S22: Vollständige Messreihe auf dem Referenzgerät

**Agent:** coder
**Datum:** 260806-0018
**Status:** Complete

## Auftrag

Planschritt S22 (`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, `#### 22.`): die vollständige Messreihe auf dem `MacBookPro15,1`, zweimal gefahren (ruhig und unter bekannter Last), der L4-Vergleich im Begleittext, CLAUDE.md nachgezogen. Wiederaufnahme nach einer unterbrochenen Sitzung; der frühere Hintergrundlauf hatte kein Ergebnis hinterlassen, diese Sitzung hat alles im Vordergrund neu gefahren.

## Ergebnis

- **Ruhige Reihe** (Abnahmelauf): `messungen/260805-2207-MacBookPro15-1-abnahme.txt` — fünf Runden, Systemlast { 1.90 2.16 2.30 } vor dem Lauf. Neun der zehn Zusagen halten in jeder Runde. **L9 verfehlt** den 95-%-Anteil (90/85/90/100/85 %); jede verpasste Eingabe liegt zwischen 17,2 und 23,4 ms, also im zweiten Bild. Entscheidungsdatensatz statt Lockerung: `decisions/260806-0014_o_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`.
- **Reihe unter bekannter Last**: `messungen/260805-2212-MacBookPro15-1-abnahme-unter-last.txt` — sechs `yes`-Endlosschleifen, Systemlast bis { 9.32 6.61 4.42 }. L1 hält fünfmal 100 %, L9 verfehlt erneut (3 von 5 Runden gehalten), L4 hebt sich auf 538,9–597,8 ms Runden-p95 mit Maximum 761,6 ms.
- **L4-Vergleich** (Begleittext `messungen/260805-2207-MacBookPro15-1-abnahme-begleittext.md`): ruhig liegen die fünf Runden-Perzentile innerhalb von 19,2 ms, unter Last verdreifacht sich die Spannweite und das Niveau steigt um rund 50 %. Die Streuung aus Defekt 260803-1845 kommt von außen; ein Startpfad-Defekt ist nicht angezeigt.
- **Befund zu Defekt 260805-2335**: L1 war Fremdlast (hält ruhig und unter reiner Rechenlast in allen Runden); L9 ist es nicht (verfehlt auch ruhig). Beide Defekte schließt der Orchestrator, nicht diese Sitzung.
- **L4 bleibt warm gemessen**: `purge` verlangt Rechte, die diese Sitzung nicht hat (geprüft: `Operation not permitted`); der Berichtskopf weist die Zahl als Untergrenze der Kaltstart-Zusage aus, wie schon in S21.

## Zwischenfall: /tmp-Prüfordner von der Systembereinigung beschnitten

Vor dem Lauf fehlten Prüfordner A genau seine 214 und dem 100.000er seine 2016 Unterordner; nach Neuerzeugung unter `/tmp` verloren beide binnen Minuten erneut alle Unterordner (auch B und der L6-Ordner), und ein laufender Fünf-Runden-Lauf brach deshalb korrekt mit "die Läufe messen nicht dasselbe" ab. Der Messplatz liegt jetzt unter `~/Library/Caches/krk-messplatz/` (A, B, gross je aus ihren Startwerten neu erzeugt und auf 10.000/10.000/100.000 Einträge geprüft, Kopierziel daneben, derselbe APFS-Datenträger). Defekt für die `/tmp`-Vorgaben im Makefile: `issues/260806-0014_o_pruefordner-unter-tmp-verlieren-leere-unterordner-an-die-systembereinigung.md`.

## Weitere Änderungen

- Plan: S22 `[IN PROGRESS]` → `[DONE]`. Stand: 34 von 36 Schritten, offen S6b und S23.
- `CLAUDE.md` `## Projektstand`: Prüfdatum 260806-0014, Zustandsbeschreibung auf den Navigator der Runde 1 gezogen, der veraltete 8/24-Planstand ersetzt (34/36, offen S6b und S23), Messreihen-Ergebnis samt L9-Entscheidungsverweis ergänzt, `messungen/`-Kommentar im Baum erweitert. `**Language:** de` und `## Sprache` unangetastet.
- Nutzer-`session.toml` vor dem Lauf gesichert und danach byte-identisch zurückgespielt (Prüfsumme 36ac815e…, verglichen). Kopierziel leer hinterlassen.
- `make check` grün (Bau, 26 Operationstests nach Wiederholung, fmt, clippy). Der einzelne Fehlschlag von `der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` trat nur unter der Restlast der Lastreihe auf und hält auf dem ruhigen Gerät; S22 hat keinen Quellcode angefasst.

## Nicht getan

- Kein Commit (der Orchestrator committet), keine Änderung an Zusagen, Abnahmemaßen oder `resources/*.toml`, keine Schließung der beiden Defekte 260803-1845 und 260805-2335.
