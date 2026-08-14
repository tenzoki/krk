# Schritt A1 — ein Prüfschritt, zwei Frager, vier Felder am Ordnermodell

**Datum:** 260814-2145
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang A, Schritt A1
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`

---

## Was umgesetzt ist

`crates/krk-core/src/verzeichnis/modell.rs`

- `Befund` als Aufzählung mit drei Werten — `Unentschieden`, `Treffer`, `KeinTreffer` — neben `Markierungsstand`, ohne Auffangzweig.
- `Ordnermodell` bekommt vier Felder: `filtertext: String`, `filter_klein: String`, `tief: bool`, `befund: Vec<Befund>`. Der Befundvektor läuft parallel zu `eintraege`, in derselben Bauart und aus demselben Grund wie `markiert`, und fällt in `ersatz_einloesen` mit Auswahl und Markierung.
- `Ordnermodell::sichtbar(index) -> bool` als **der eine** Prüfschritt, das erste Spec-Bild Zweig für Zweig: versteckt und ausgeblendet, kein Filtertext, Name trägt die Teilzeichenfolge, ist es ein Ordner, ist „Deep" an, liegt ein Treffer darunter. Der letzte Zweig ist ein `match` über alle drei `Befund`-Werte ohne `_ =>`.
- `anhaengen` und `sicht_neu_aufbauen` rufen ihn; die beiden bisherigen wortgleichen Fassungen der Versteckt-Regel sind weg. `sicht_neu_aufbauen` nimmt die Sichtliste mit `mem::take` heraus, damit `sichtbar` sich `self` ausleihen kann und die Zuteilung stehen bleibt.
- Der Vergleich ist derselbe wie in `Belegungsmodell::zeile_traegt`: `name.to_lowercase().contains(&filter_klein)`, mit der Umschreibung des Suchtexts einmal je Änderung in `filter_uebernehmen` und nicht einmal je Zeile.
- Setzer: `filtertext_setzen`, `zeichen_anhaengen`, `letztes_zeichen_weg` (`#[must_use]`, liefert ob etwas wegzunehmen war), `filter_leeren`, `tief_setzen`, `befund_setzen`, `befund_zuruecksetzen`. Leser: `filtertext`, `filter_klein`, `filter_steht`, `tief`, `befund`.
- Der Modulkopf trägt einen neuen Abschnitt mit dem Bild der sechs Zweige und der Begründung, warum die Regel an einer Stelle steht.
- Sortierschlüssel, Markierung, `betroffene` und `Eintrag` sind unangetastet.

`crates/krk-core/tests/verzeichnis.rs`

Vierzehn neue Proben im Abschnitt „Der Filter aus C1 und C2": Teilzeichenfolge an jeder Stelle und in jeder Schreibung, keine Umlautfaltung (`apfel` findet `Äpfel` nicht), Ordner bleiben bei flacher Suche stehen, Name oder Befund bei tiefer Suche, ein namentlich passender Ordner steht auch ohne Treffer darunter, Verknüpfungen zählen für die Sichtbarkeit als Ordner, Filter und Verstecke gehen durch denselben Prüfschritt, der Filter wirkt schon beim Anhängen eines Stapels, die eingestellte Sortierung bleibt die Ordnung, die ausgefilterte Auswahl kommt zurück, die Markierung besteht fort und wirkt wieder, die Markierbefehle behalten ihren Zuschnitt, ein Zeichen zurück lässt die Liste wachsen, der Befund fällt bei jeder Änderung der Frage zurück, der kleingeschriebene Filtertext läuft mit.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün: Bau, 31 Proben in `tests/verzeichnis.rs`, Clippy unter `-D warnings`, Formatprüfung.

## Datensätze

- Geschlossen: `issues/260814-2102_c_der-pruefschritt-fuer-die-sichtbarkeit-steht-im-ordnermodell-zweimal-wortgleich-da.md`
- Neu abgelegt: `issues/260814-2145_o_die-begruendung-des-spec-fuer-verknuepfungen-in-der-sichtbarkeit-haelt-am-baum-nicht.md`
- Neu abgelegt: `issues/260814-2145_o_befund-setzen-baut-die-ganze-sicht-neu-auf-und-der-durchlauf-ruft-es-je-ordner.md`
- Plan: Schritt A1 auf `[DONE]`

## Nicht angefasst

`verzeichnis/mod.rs` (kein `pub use Befund` — A1 nennt zwei Dateien und `tests/verzeichnis.rs` erreicht `krk_core::verzeichnis::modell::Befund` unmittelbar), `sprungmarke.rs` (Strang A2), alles unter `krk-ui`, `crates/krk-bench/src/messen.rs`.

Nicht committet — das ist Sache des Nutzers.
