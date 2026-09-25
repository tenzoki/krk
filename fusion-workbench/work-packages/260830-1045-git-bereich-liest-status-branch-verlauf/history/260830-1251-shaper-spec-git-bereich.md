# Shaper: der Spec der Runde 23

**Date:** 2026-08-30 12:51
**Status:** Complete
**Filed by:** shaper, Kai Stalmann <kai@stalmann.org>
**Circle:** 260830-1045-git-bereich-liest-status-branch-verlauf

## Auftrag

Den Spec der Runde 23 aus der bereits geschärften Directive schneiden. Die Directive war nicht noch einmal zu klären. Ein früherer Lauf hatte vier Fragen gestellt, die der Nutzer mit „1a 2a 3b 4b" beantwortet hat; sie kamen mit dem Auftrag herein.

## Gelesen

Die Directive und der Grounding snapshot des Circle-Datensatzes, die Machbarkeitsanalyse `260830-1006-gix-als-git-anbindung-stufe-a.md` in voller Länge, die vier beantworteten und die eine offene Entscheidung unter dem Stempel `260830-1006`, der Defekt `260830-1106_*_der-entscheid-zur-c-freiheits-zusage-nennt-fuenf-prosastellen-im-baum-stehen-sechs.md`, der Defekt `shared/issues/260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-…`, der Spec der Runde 22 als Vorbild für Form und Zuschnitt.

Dazu im Quellbaum, Stand `3266fb3`: `fenstermodell.rs` (`Bereich` samt seinen elf Fallunterscheidungen), `kommandos/fokus.rs` (`Fokus`, `wirkt`, die Tafel), `kommandos/zulaessigkeit.rs`, `spalten.rs`, `appkit/tabelle.rs` (die vier Spaltenfunktionen), `appkit/bereichsleiste.rs`, `appkit/statuszeile.rs`, `appkit/aufteilung.rs`, `auffrischung.rs`, `belegungsmodell.rs`, `krk-core/src/ablage/sitzung.rs`, `krk-core/src/tasten/belegung.rs` (`Wirkungsbereich`), `resources/default-keymap.toml`.

## Erhebungen

- **Die zwei Tasten sind frei.** `grep 'tasten = ' resources/default-keymap.toml` über alle Kombinationen nennt weder `opt+cmd+r` noch `shift+cmd+b`. Beide bleiben in ihrer Familie: `opt+cmd+<Buchstabe>` ist die Umschaltfamilie, `shift+cmd+<Buchstabe>` die Fokusfamilie.
- **92 Zählaussagen über Bereiche, Fokuswerte, Spalten und Schalter** in 21 Dateien werden von dieser Runde unrichtig. Das Erhebungskommando steht als Kriterium C9.4 im Spec, mit `messungen/`, `spikes/` und dem Tätigkeitsbericht ausgenommen.
- **`Spalte::ALLE` trägt dieselbe stille Stelle wie `Bereich::ALLE`.** Die Feldbreite `[Spalte; 4]` zwingt zu vier Einträgen und sagt nichts darüber, welche vier; der Modulkopf von `spalten.rs` zählt sieben Stellen auf, die eine fünfte Spalte erzwingt, und lässt die eine aus, die entscheidet, ob die Spalte überhaupt erscheint. Kein neuer Datensatz: `Spalte::ALLE` ist eine der elf Listen, die `shared/decisions/260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-…` über sein `grep` benennt. Der Spec trägt es als C5.1, C9.7 und als erste Bedingung.
- **Kein neunter `Wirkungsbereich` ist nötig.** Beide neuen Befehle tragen `Ueberall`, wie die vier Fokusbefehle und die fünf Umschalter. Die einzige offene Zelle der Tafel ist `Navigator × Git`, und sie steht auf `true`, weil die Directive die Pfeiltasten in der Verlaufsliste verlangt.

## Geschrieben

- `planning/260830-1251_o_spec-git-bereich-liest-status-branch-verlauf.md`: zehn Fähigkeiten, 90 Abnahmekriterien, davon 25 Nutzerarbeit am laufenden Bündel. Dreizehn Festlegungen des Nutzers (E1 bis E13) stehen von vierzehn Festlegungen des Specs (A1 bis A14) getrennt, weil die zweite Gruppe am Spec-Tor überstimmbar ist und die erste nicht.
- `decisions/260830-1251_o_haengt-der-gitbefund-zusaetzlich-an-einem-beobachter-auf-git.md`: die eine Frage, die der Spec mit einer Festlegung beantwortet und die den Nutzer angeht, weil sie ihn im Betrieb trifft.

## Keine Klärungsrunde

Vier Kandidaten für eine fünfte Frage sind geprüft und alle vier ohne Rückfrage entschieden. Der Zuschnitt der Statuszusammenfassung folgt aus dem ersten Satz der Directive („den Git-Zustand des **angezeigten Ordners**") und aus der Beschränkung über die Pfadmuster; die zwei Sonderzustände des HEAD sind vom Auftrag ausdrücklich zum Ausschreiben verlangt; die Auffrischung ist als Datensatz gefilt und im Spec vorbelegt. Alle vier stehen als A3, A6, A7, A8 und A9 und sind am Spec-Tor überstimmbar.

## Verification

`grep -c '^- \[ \]'` über den Spec: 95 Kästchen, davon 90 Abnahmekriterien und fünf offene Nutzerfragen; je Fähigkeit gezählt mit `grep -cE '^- \[ \] C<n>\.[0-9]+ '`. Jede Baumaussage des Specs ist am Stand `3266fb3` gelesen, dessen Quellbaum unterhalb von `crates/`, in `Cargo.toml` und in `CLAUDE.md` mit `d1fbaac` deckungsgleich ist. Kein Code, keine Daten, kein Plan und kein Circle-Datensatz sind angefasst.
