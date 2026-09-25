# Ontocoder: Schritt 6 der Runde 19 — der Kommentarteil der Auslieferungsfassung

**Datum:** 260827
**Plan:** `planning/260827-1322_p_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`, Schritt 6
**Kriterien:** C3.4, C3.9, C3.10
**Status:** Complete

## Was geändert ist

`resources/default-readers.toml`, allein im Kommentarteil, 39 Zeilen hinzu, keine entfernt:

- Abschnitt „Die vier Bausteine", bei `zaehlung` hinter dem Beispiel „Offene Defekte": die zwei freiwilligen Schlüssel. `typ` mit den drei Werten `"datei"`, `"ordner"`, `"verknuepfung"` in Umschrift (Festlegung 5 des Plans), ohne ihn zählt jeder Typ, ein vierter Wert kostet die ganze Datei; `versteckt = true` setzt die Klammer (`14 (3)`, die Zahl davor schließt die versteckten ein, Klammer auch bei null), ohne den Schlüssel oder mit `false` keine Klammer; über der Eintragsschranke entfällt die Klammer ganz und es steht der „mindestens"-Satz. Dazu ein zweites Beispiel „Dateien" mit `zaehlung = { typ = "datei", versteckt = true }` — als Kommentarzeile (C3.9).
- Neuer Abschnitt „Das eingebaute Default-Profil" hinter „Welches Profil gewinnt": die drei Zählzeilen „Dateien", „Ordner", „Verknüpfungen" sind in KRK eingebaut, stehen in keinem Block der Datei, lassen sich weder anpassen noch abschalten, bleiben auch bei geleerter Datei; ein treffendes Profil verdrängt sie; wer sie selbst beschreiben will, schreibt ein eigenes Profil mit denselben Schlüsseln (C3.10).
- Kein `[[profil]]`- und kein `[[profil.zeile]]`-Block angefasst (C3.4); `git diff HEAD --stat` zeigt nur Einfügungen.

## Nicht gemacht, mit Grund

- Der Einleitungssatz „Trifft keines zu, bleibt die Metadatenanzeige, wie sie war." (Zeile 18 f.) und Schritt 3 unter „Welches Profil gewinnt" („die gewohnte Metadatenanzeige") sind seit dieser Runde ungenau: die Metadaten bleiben, die drei Zählzeilen treten darunter. Der Dispatch nennt allein die zwei Stellen aus Schritt 6; der neue Abschnitt richtet die Aussage, ohne die alten Sätze zu ändern. Für den Orchestrator als Nachfrage vermerkt.
- Nicht committet; kein baumweites git-Kommando.

## Verifikation

- `make check` — exit 0 (alle vier grün; `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt ... ok`).
