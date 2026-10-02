# Der Plan der Runde 1 führt den Messstrecken-Defekt nicht mehr als offen (T1)

**Agent:** ontocoder
**Status:** Complete
**Quelle:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_der-plan-fuehrt-den-messstrecken-defekt-an-zwei-stellen-noch-als-offen.md`
**Aufgabe:** T1 aus `fusion-workbench/tasklist.md`
**Geändert:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_*_plan-navigator-geruest-runde-1.md`, sonst nichts

## Der Anlass in einem Satz

Der Plan der Runde 1 sagte im Kopf weiter, aus dem Nachzug 260807-0832 bleibe ein Defekt an der Messstrecke offen; der Commit `d569f8a` hat ihn sechzehn Minuten nach jenem Nachzug geschlossen.

## Was am Baum nachgeprüft ist, bevor geschrieben wurde

Alle vier Behauptungen des Defektdatensatzes halten, jede einzeln gelesen:

- `Abnahmemass::AnteilImBild` (`crates/krk-bench/src/messen.rs:395-410`) trägt die drei Felder `bildlaenge`, `mindestanteil_prozent` und `obergrenze_bilder`.
- `ANTEIL_IM_BILD_PROZENT` liefert im Codebaum null Treffer; die verbliebenen Vorkommen stehen sämtlich in Dokumenten der Workbench.
- `Zusage::gehalten_in` (ebd.:577-612) prüft Anteil und Obergrenze in derselben Runde, beide Hälften mit `anteil_haelt && grenze_haelt`.
- `issues/260807-0832_*_die-messstrecke-kann-die-neue-zweiteilige-fassung-von-l9-nicht-abnehmen.md` trägt den Marker `_c_`.

`d569f8a` ist am 260807-0856 entstanden, `git log` gelesen. Die Datumsangabe „sechzehn Minuten später" aus dem Defektdatensatz stimmt damit.

## Die drei Stellen im Plan

Der Datensatz vom 260807 nennt zwei Stellen, die Aufgabe eine dritte. Gesucht wurde nach dem Wortlaut, nicht nach der Zeilennummer; die Nummern aus dem Datensatz (23 und 264) sind auf 25 und 267 gewandert.

- **Zeile 25, Nachzug 260807-0832.** Stand offen, ist berichtigt.
- **Zeile 267, `### Frage 5`.** War schon berichtigt, und zwar mit dem Nachzug 260807-1900, Commit `f11b36d` vom 260807-1923. Nichts getan.
- **Zeile 1458, Abgleichseintrag 260807-1022.** Führt den Befund selbst und sagte „Zwei Stellen dieses Plans sagen aber …". Mit der Berichtigung von Zeile 25 wird der Satz falsch, also mitgezogen.

## Wie Zeile 1458 mitgezogen ist, und warum nicht anders

Der Eintrag ist ein datierter Abgleichsbefund und kein laufender Text. Ihn zu löschen hieße, die Aufzeichnung eines Standes zu tilgen, den es gab; ihn stehen zu lassen hieße, eine Gegenwartsaussage zu führen, die nicht mehr stimmt. Der Befund steht deshalb weiter, aber im Präteritum, und dahinter steht in einem Satz, dass beide Stellen inzwischen berichtigt sind, mit dem jeweiligen Commit und Datum. Wer den Abgleich später liest, sieht damit den Befund und seine Erledigung an derselben Stelle.

Dieselbe Zurückhaltung gilt für Zeile 25: der Absatz behält seine Aussage, dass der Nachzug den Defekt hinterlassen hat, und nennt nun im selben Satz seine Schließung. Ein Nachzug, aus dem nie etwas offen geblieben wäre, hätte den Datensatz `260807-0832` nie erklärt.

## Der Marker

Der Plan trägt `_c_` und behält ihn. Berichtigt ist der Inhalt, nicht der Zustand; der Dateiname ist unverändert.

## Prüfung

- `git diff --name-only -- …/planning/` nennt genau die eine Datei — exit 0.
- Die hinzugefügten Zeilen enthalten keinen ausgeschriebenen Zustandsmarker in einem Verweis: `git diff -U0 … | grep '^+' | grep -v '^+++' | grep -oE '(issues|decisions|planning|history|circles)/[0-9]{6}-[0-9]{4}_[a-z]_'` liefert nichts — exit 1, also kein Treffer. Beide neuen Verweise stehen in der Sternform `_*_`.
- Kein Bau nötig, es sind reine Textstellen.

## Was offen bleibt

Der Defektdatensatz steht auf `_p_` und wird vom Nutzer geschlossen; der `Resolved:`-Vermerk und der Commit gehören ihm. Weder `git add` noch `git commit` sind gelaufen.
