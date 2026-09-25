# Welche Antworten bietet das Konfliktblatt, wenn das Zip genau eine Zieldatei hat?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_a_circle.md` (Directive, Zip-Teil); `crates/krk-ui/src/appkit/blaetter/konflikt.rs`; `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-1832_*_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`

---

## Question

Der Nutzer hat für den Namenskonflikt beim Zip „dieselbe Rückfrage wie beim Kopieren“ gewählt und sie als überschreiben, danebenlegen oder abbrechen beschrieben. Das gebaute Konfliktblatt bietet vier Antworten, nämlich Überschreiben, Überspringen, Umbenennen und Abbrechen, dazu die Wahl „für alle weiteren übernehmen“. Ein Zip erzeugt genau eine Zieldatei, und in dieser Lage hat „für alle weiteren“ keinen Gegenstand, während Überspringen und Abbrechen dasselbe bewirken: der Vorgang endet ohne Archiv. Vor dem Bau muss feststehen, welche Schaltflächen der Nutzer sieht, denn ein Blatt mit einer Antwort ohne Wirkung und einer doppelten Antwort ist die Art von Sonderfall, die dieses Projekt vermeidet.

## Options

1. **Das Blatt unverändert übernehmen** — vier Antworten und das Ankreuzfeld, genau wie beim Kopieren.
   - Pro: Kein Sonderfall im Blatt, eine Fassung für alle Vorgangsarten, keine neue Verzweigung.
   - Contra: Der Nutzer sieht ein wirkungsloses Ankreuzfeld und zwei Schaltflächen, die dasselbe tun.
2. **Auf drei Antworten kürzen: Überschreiben, Umbenennen, Abbrechen** — das Ankreuzfeld und Überspringen entfallen, wenn der Vorgang genau eine Zieldatei hat.
   - Pro: Trifft genau das, was der Nutzer in Runde 2 beschrieben hat. Jede sichtbare Antwort hat eine eigene Wirkung.
   - Contra: Das Blatt bekommt eine Fallunterscheidung nach der Zahl der Ziele, also eine zweite Gestalt, die eine eigene Probe braucht.
3. **Ohne Blatt: der Vorgang legt immer daneben** — bei einem belegten Namen entsteht `Projekte 2.zip`, gemeldet in der Statuszeile.
   - Pro: Keine Rückfrage bei einer Handlung, die nichts überschreibt und leicht zu wiederholen ist.
   - Contra: Widerspricht der Wahl des Nutzers aus Runde 2 und nimmt ihm das absichtliche Überschreiben eines veralteten Archivs.

## Constraints

Die Reihenfolge der Schaltflächen und die Tastenbelegung des Blattes sind aus einem Sicherheitsgrund gesetzt: die Eingabetaste liegt auf Überspringen und nicht auf der ersten Schaltfläche, damit ein reflexhaftes Bestätigen nichts löscht, und die Escape-Taste fällt auf Abbrechen. Fällt Überspringen weg, muss die Eingabetaste einen neuen Träger bekommen, und Überschreiben darf es nicht sein. Das Namensfeld des Blattes ist nicht der Ersthelfer, weil sonst der Ereignisabgriff jede Taste an AppKit weitergäbe; der Eingabewächter des Blattes hängt an derselben Stelle.

## Recommendation

Möglichkeit 2, mit der Eingabetaste auf Abbrechen. Sie liefert, was der Nutzer beschrieben hat, und die Fallunterscheidung ist billig: sie hängt an einer Zahl, die der Aufrufer ohnehin kennt, und die Wahl der Vorgabeschaltfläche steht bereits an einer prüfbaren Stelle.

---
Answered: shared/history/260824-2120-orchestrator-session.md:34 — Moeglichkeit 2, auf Ueberschreiben, Umbenennen und Abbrechen kuerzen; Eingabetaste auf Abbrechen.
Implemented: 8b5a5ce — blaetter::konflikt::schaltflaechen(genau_ein_ziel) baut beide Gestalten als eine Angabe; bei genau einem Ziel drei Antworten, Eingabetaste auf Abbrechen, kein Ankreuzfeld. Je eine Tafelprobe haelt beide.
Deferred:
Superseded by:
Retired:
