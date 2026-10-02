# Shaper: Befehle absetzen und Makros speichern

**Datum:** 2026-08-16 22:40
**Modus:** user-direct, ohne aktiven Circle
**Ausgabe:** `fusion-workbench/shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md`

## Auftrag

Den Spec der zwölften Runde schreiben. Alle Klärungsrunden waren gefahren; der Auftrag brachte die vollständige Fassung von elf Nutzerantworten, den Umfang von vier Fähigkeiten in fester Baureihenfolge, neun bestätigte Festlegungen aus dem Baum und die Abgrenzungen mit. Keine Frage war offen, also lief keine weitere Klärungsrunde.

## Grundlage

Gelesen und geprüft am 260816 gegen den Baumstand `627b5f4`:

- `shared/consult/260815-1354-befehlslauf-und-makros-in-krk.md`, vollständig, samt Quellenliste und offenen Fragen.
- `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` als Formvorbild der elften Runde.
- `crates/krk-core/src/ablage/pfade.rs:104-183` — sechs Ablagedateien, und der Doc-Kommentar an `Datei::format` nennt die siebte ausdrücklich als bauanhaltend.
- `crates/krk-core/src/verzeichnis/sys.rs:1-60` — fünf Schnittstellen, neun gebundene Funktionen.
- `crates/krk-core/src/tasten/belegung.rs` — `Kommando` trägt 79 Varianten, `Wirkungsbereich` sieben.
- `crates/krk-ui/src/belegungsmodell.rs:74-190` — neun Funktionsbereiche, eine Gliederung mit drei Abnehmern.
- `crates/krk-ui/src/menuemodell.rs:1-45` — ein Obermenü je besetztem Funktionsbereich, keine zweite Ordnung.
- `crates/krk-ui/src/appkit/statuszeile.rs:200-245` — sechs Ränge, ein siebter hält den Bau an.
- `crates/krk-ui/src/appkit/anwendung.rs:4591-4616` — die drei Ränge des Abbruchs, und `vorgang: RefCell<Option<Vorgang>>` als die eine laufende Operation.
- `crates/krk-ui/src/vorschaumodell.rs:1-30, 121, 419-455` — die eine Regel der Tabs, `TEXTGRENZE`, `zwischenablage_anzeigen`.
- `crates/krk-ui/src/kommandos/operationen.rs:1-96` — Vermittlerfaden, Bündelung ohne Takt, Vorgangszeile.
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs:1-80` — eine Frage, zwei Frager.
- `resources/default-keymap.toml:1-60, 343-410` — Aufbau eines Eintrags, die Form der leeren Tastenliste, 85 ausgelieferte Funktionen.
- Alle offenen Defekt- und Entscheidungsdatensätze in `shared/` und in jedem Circle, über das `find` aus `CLAUDE.md`.

## Was der Spec zusätzlich entscheidet

Fünf Stellen waren aus den elf Antworten nicht unmittelbar zu lesen und sind im Spec unter `## Abgeleitet und nicht gefragt` benannt, damit sie am Gate widersprechbar bleiben:

1. **Höchstens ein angehefteter Tab, und er gehört dem Befehlslauf.** Damit ist die Tabelle unter `## Welcher Tab die Ausgabe nimmt` vollständig und überschneidungsfrei, und die Tab-Leiste wächst nicht mit jedem Lauf. Der Preis, die Ausgabe eines Laufs überlebt den nächsten nicht, steht im Spec.
2. **Freie Argumente werden im Makroeintrag erklärt und nicht aus unbekannten Klammern erraten.** `awk '{print $1}'` ist der Fall, an dem die Erratungsregel scheitert.
3. **Die dreizehn Kommandos tragen `Wirkungsbereich::Ueberall`**, mit derselben Begründung, die `Kommando::Abbrechen` schon trägt.
4. **Ein Makro, das die Auswahl verlangt und nichts vorfindet, läuft nicht**, nach dem Muster von `nichts_zu_kopieren` und `nichts_zu_teilen`.
5. **Der Ausgabe-Tab trägt keinen Pfad** und gilt damit nicht als angezeigte Datei.

## Berührungen mit dem übrigen Speicher

- `shared/decisions/260813-0053_o_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md` wird faktisch in Richtung seiner Empfehlung beantwortet und ausdrücklich nicht geschlossen.
- `shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md` trifft diese Runde nicht, weil alle dreizehn Funktionen ohne Kombination ausgeliefert werden.
- Die Abgrenzung „KRK als Kommandozentrale für Fusion" aus der Runde 1 wird genannt und nicht aufgehoben.
- Die Reihenfolge gegenüber dem vorgesehenen Circle `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` ist als bewusste Festlegung aufgenommen.

## Ergebnis

54 Abnahmekriterien über vier Fähigkeiten, ein Mermaid-Diagramm des Laufwegs, eine Tabelle für die Tab-Regel. Keine ausstehende Nutzerentscheidung, kein neuer Entscheidungs- und kein neuer Defektdatensatz. Kein Circle angelegt und kein Marker umbenannt; das ist der ausdrückliche Auftrag des Nutzers.
