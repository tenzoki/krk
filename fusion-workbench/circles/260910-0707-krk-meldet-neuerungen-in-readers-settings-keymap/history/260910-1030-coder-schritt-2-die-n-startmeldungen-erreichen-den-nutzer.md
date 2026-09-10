# Coder-Sitzung: Schritt 2, die n Startmeldungen erreichen den Nutzer

**Date:** 2026-09-10, 260910-1030
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 2
**Defekt:** `shared/issues/260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
**Entscheid:** `260910-0818_*_wie-erreichen-n-startmeldungen-den-nutzer-wenn-die-eine-zeile-nur-eine-traegt.md`, Möglichkeit 2
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Der gewählte Weg

Möglichkeit 2 des Entscheids, unverändert: **genau eine Startmeldung geht unverändert über
`meldung_zeigen` in die Statuszeile, ab der zweiten fährt ein Blatt herunter, das alle
aufführt.** Gebaut wie die Abschlussliste der übersprungenen Einträge —
`Blatt::mit_schaltflaechen` mit einer schließenden Schaltfläche auf der Eingabetaste,
`erlaeuterung_setzen` für die Liste. Ein dritter Anzeigeweg entsteht nicht.

Weder die Sätze verbunden noch eine Warteschlange mit Verweildauer gebaut; beide sind im
Entscheid geprüft und verworfen.

## Was getan wurde

### Das neue Blatt `crates/krk-ui/src/appkit/blaetter/startmeldungen.rs`

Drei Stücke, und die Entscheidung liegt im ersten:

- `Auskunft<'a>` — die drei Ausgänge: `Nichts`, `Zeile(&str)`, `Blatt { frage, liste }`.
- `auskunft(&[String]) -> Auskunft<'_>` — die reine Funktion, die den Weg wählt. Sie steht
  hier und nicht im Rumpf des Aufrufers, damit alle vier Fälle (null, eine, zwei, drei
  Meldungen) ohne AppKit und ohne Hauptfaden messbar sind; `libtest` gibt den Hauptfaden
  nicht her, und `krk-ui` hat kein Bibliotheksziel.
- `schaltflaechen()` und `zeigen(...)` — Bauplan und AppKit-Anteil, Zeile für Zeile die
  Form von `blaetter/uebersprungen.rs`.

**Der Fall „genau eine Meldung" bleibt Wort für Wort, was er war**: `Auskunft::Zeile` reicht
den Text unverändert heraus, der Aufrufer ruft damit dasselbe `meldung_zeigen` auf demselben
aktiven Dateifenster wie zuvor. Keine Kopfzeile, kein Zusatz, kein Blatt. Das fordert der
Defekt ausdrücklich.

Der Modulkopf trägt den Grund der Wahl, die begrenzte Abweichung von der Antwort vom
260804-0830 und die zwei verworfenen Wege mit ihren Gründen. Dazu den Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen`, den die zwei Proben in
`crates/krk-core/tests/baum.rs` von jeder AppKit-Datei mit Frameworkimport verlangen:
`NSWindow` seit 10.0, `MainThreadMarker` als Rust-Typ von `objc2` ohne macOS-Alter.

### `crates/krk-ui/src/appkit/anwendung.rs`

Die Schleife am Ende von `oberflaeche_aufbauen` ist eine vollständige Fallunterscheidung
über `Auskunft` geworden, **ohne Auffangzweig**: ein vierter Weg, eine Startmeldung zu
zeigen, hält damit den Bau an, statt still zu entstehen. Der Kommentar davor nennt den
Defekt und zeigt auf die reine Funktion.

Daneben steht die neue Methode `startmeldungen_zeigen`, gebaut wie der Rufer von
`uebersprungen::zeigen`: schwacher Griff auf den Delegierten im Rückruf, `blatt_geschlossen`
beim Schließen, `blatt_oeffnet` für den Griff nach `offenes_blatt` — sonst hätte das
stehende Blatt keinen Abbruchweg von außen.

Ohne Fenster passiert nichts: `ivars.fenster` ist zu diesem Zeitpunkt längst gesetzt (die
Zeile steht weit oben in derselben Funktion), und ohne Fenster gäbe es weder Blatt noch
sichtbare Statuszeile, weil beide darin hängen.

### `crates/krk-ui/src/appkit/blaetter/mod.rs`

`pub mod startmeldungen;` und der Modulkopf nachgezogen: „zehn" wird „elf", das elfte ist
benannt, und das Blatt außerhalb des Verzeichnisses (`belegungsansicht`) ist jetzt das
zwölfte statt des elften. Der Satz „zehn Blätter mit je eigenem Aufbau wären zehn Stellen"
zählt mit. Nicht angefasst: die Zeile mit dem Dateinamen des Defektdatensatzes
`260826-1336_*_der-modulkopf-der-blaetter-zaehlt-zehn-…` — ein Zitat, dessen Wortlaut die
Aussage selbst ist, und „das zehnte ist das erste mit einem eigenen Wächter", das weiter
richtig auf den Notizzettel zeigt.

## Der Wortlaut

Kopfzeile des Blattes: `Beim Start gab es {n} Meldungen`. Ein Singular fehlt mit Absicht —
`n` ist an dieser Stelle immer mindestens zwei. Die Liste sind die Meldungen selbst, eine je
Zeile, unverändert. Die Schaltfläche heißt „Schließen", wie bei der Abschlussliste.

Nutzersichtbarer Text trägt Umlaute (Naht vom 260907), Bezeichner und Kommentare die
Umschrift.

## Proben (im `#[cfg(test)]`-Modul neben dem Code)

| Probe | Abnahmekriterium |
|---|---|
| `ohne_meldung_geschieht_nichts` | ohne Startmeldung kein Blatt und keine Zeile |
| `eine_meldung_geht_unveraendert_in_die_zeile` | bei genau einer Meldung Wort für Wort das heutige Verhalten |
| `zwei_meldungen_stehen_beide_im_blatt` | bei zwei Meldungen erreicht jede den Nutzer |
| `drei_meldungen_stehen_alle_drei_im_blatt` | bei drei Meldungen erreicht jede den Nutzer |
| `der_bauplan_traegt_die_eine_schliessende_schaltflaeche` | genau eine Schaltfläche, und sie lässt liegen |

Die dritte und die vierte stehen beide da, obwohl die Grenze bei zwei liegt: eine
Fallunterscheidung, die allein bei zwei richtig rechnete, fiele mit nur einer der beiden
nicht auf. Die vierte hält daneben die Zeilenzahl, damit ein Verbinder der Sätze — genau die
verworfene Möglichkeit 1 — rot würde.

Die Zählprobe `jedes_blatt_nennt_seine_liegenlassende_schaltflaeche`
(`crates/krk-ui/src/appkit/blaetter/mod.rs`) sieht das neue Blatt von selbst: sie läuft über
den Quellbaum und nicht über eine Aufzählung.

## Was ausdrücklich nicht angefasst ist

**Der Defektdatensatz.** Der Auftrag behält die abschließende `Resolved:`-Zeile dem Nutzer
vor; der Marker steht deshalb weiter auf `_o_`, und das Umbenennen gehört zu derselben
Handlung. Der Plan sieht beides in diesem Schritt vor — hier weicht die Ausführung dem
Auftrag und nicht dem Plan.

**Der Entscheidungsdatensatz** `260910-0818_*_wie-erreichen-n-startmeldungen-…` steht
weiter auf `_a_`. Die `Implemented:`-Zeile zitiert den Commit-Hash, und committet wird
nicht in dieser Sitzung.

Nicht angefasst sind ferner `crates/krk-core/`, `xtask/`, `resources/`, CLAUDE.md und die
Schritte 3 bis 10 des Plans. Insbesondere gibt es weiter keinen Merker der gemeldeten
Fassung, keinen Aufruf von `neuerungen::erheben` beim Start, kein Kommando und kein Blatt
auf Abruf.

## Was offen bleibt und keine Schuld dieses Schrittes ist

**Der Messmodus.** Ein stehendes Blatt sperrt jeden Tastenbefehl bis auf vier, und die
Zeitzusage L4 misst bis zur bedienbaren Sitzung. Der Entscheid hält fest, dass es dabei
bleibt: der Messmodus liest eine Prüfsitzung und keine gewöhnliche Ablage und erzeugt
darum im Regelfall gar keine Meldung. Hier ist dafür nichts eingebaut worden — keine
Sonderbehandlung, kein Schalter.

## Prüfung

- `make check` — exit 0 („alle fünf gruen": build, test, fmt-check, lint, doc)

Der erste Lauf war an `cargo fmt --all --check` rot, an einer Zeilenumbruchstelle der
`use`-Liste in `anwendung.rs`; `cargo fmt --all` hat sie gesetzt, der zweite Lauf ist grün.
Die fünf neuen Proben laufen im Probenlauf des Binärziels `krk` mit.
