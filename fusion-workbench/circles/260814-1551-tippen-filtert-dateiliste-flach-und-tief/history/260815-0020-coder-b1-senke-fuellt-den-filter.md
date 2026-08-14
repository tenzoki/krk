# B1 — Die Senke füllt den Filter des sichtbaren Tabs

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang B, Schritt B1
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.1, C1.8, C1.13, C2.7
**Verification:** `make check` — exit 0

## Was umgesetzt ist

**Das Tippen hat ein neues Ziel, und der Weg dorthin ist derselbe geblieben.**
`DateifensterQuelle::sprungmarke_tippen` heißt jetzt `filterzeichen_tippen` und schreibt
über `Tabliste::aktiver_mut().modell_mut().zeichen_anhaengen` in das `Ordnermodell` des
sichtbaren Tabs. Der Rückgabewert ist der alte: ob KRK das Zeichen verbraucht hat. Die
Zeichenregel `krk_core::verzeichnis::sprungmarke::traegt_ein_dateiname` wird dabei am
Aufrufer gefragt und nicht im Kern, weil `zeichen_anhaengen` keinen Rückgabewert trägt
(C1.4).

**Der Ivar `sprungmarke: RefCell<Sprungmarke>` ist ersatzlos gefallen**, mit ihm die
Zeile in `neu`, die Zeile in `nach_lesebeginn`, die in `tab_gewechselt` und die in
`umsortiert`. Der Import ist von `sprungmarke::{self, Sprungmarke}` auf
`sprungmarke::traegt_ein_dateiname` zurückgegangen; `std::time::Instant` hatte danach
keinen Leser mehr in der Datei und ist aus der `use`-Zeile gefallen. Das Modul
`sprungmarke.rs` selbst ist unberührt — sein Abbau ist A2.

**C1.11 ist eine reine Funktion geworden.** `crate::kommandos::navigation::ersatzzeile`
steht neben `zielzeile` und beantwortet aus drei Werten — ob das Modell einen Eintrag als
ausgewählt führt, welche Zeile er in der neuen Sicht hat, wie viele Zeilen sie trägt —,
ob eine Ersatzzeile fällig ist. Vier Proben in `mod tests` derselben Datei, ohne
`NSTableView`.

**Zwei Randfälle sind dabei entschieden und stehen als Prosa an der Funktion:** ohne
bestehende Auswahl entsteht keine, weil ein Sprung C1.1 widerspräche; und in einer leeren
Sicht bleibt die Auswahl leer, was `kommandos::operationen::betroffene` schon trägt.

## Wie die Anzeige nachzieht

Neu ist die private `DateifensterQuelle::nach_filteraenderung`: sie ruft `umsortiert`
(also `reloadData` und `auswahl_anzeigen`) und stellt danach die Frage aus C1.11 an der
**neuen** Sicht. Fällt eine Ersatzzeile an, geht sie über `zeile_setzen` und nicht über
`zeile_auswaehlen`, damit die Auswahl auch im Modell steht.

**`letztes_filterzeichen_weg` aus C2 ruft jetzt dieselbe Stelle** statt `umsortiert`
unmittelbar. Der Ersatzzweig kann dort nicht greifen — ein Zeichen weniger nimmt der
Sicht keine Zeile —, aber zwei Wege für dieselbe Änderung wären zwei Gelegenheiten,
auseinanderzulaufen.

**`umsortiert` selbst hat seine Bedeutung behalten.** Der Datensatz
`issues/260814-2357_o_c2-nennt-zwei-dateien-…` hatte erwartet, B1 werde den Auswahlnachzug
dort einbauen; das hätte auch `verstecke_umschalten` und die beiden Sortierbefehle aus der
Runde 1 geändert, die C1.11 nicht meint. Der Nachzug sitzt deshalb eine Ebene darüber, und
die beiden Filtermethoden erben ihn trotzdem, wie der Datensatz es vorhergesagt hat.

## Was der Anwendungsdelegierte bekommen hat

Der Zeichenzweig in `eingabe_ausfuehren` hat seine Form behalten und allein den Namen der
gerufenen Methode gewechselt. Der Fokusvorbehalt aus dem Defekt `260809-1648` steht
unverändert, ebenso die Abweisung bei stehendem Blatt und bei einem Ersthelfer, der
AppKit gehört. Vier Prosastellen, die die Sprungmarke als gegenwärtig beschrieben, nennen
jetzt den Filtertext und die Sprungmarke als abgelöst: der Modulkopf, der Kopf von
`eingabe_ausfuehren` und zwei Kommentare im Zweig selbst.

**Kein neues Kürzel, kein neues Bedienelement** (C1.13). `resources/default-keymap.toml`
ist nicht angefasst; der Nachschlag fällt für eine unbelegte Taste ohne Zusatztaste
weiter auf `Nachschlag::Sprungmarke`. Die Variante behält ihren Namen — ihre Umbenennung
gehört, wenn überhaupt, zu A2.

## Ob ein getipptes Zeichen wirklich im Modell landet

**Ja.** Die Kette ist vollständig und hat keine Lücke mehr:
`ereignisse::behandeln` → `Nachschlag::Sprungmarke` → `Eingabe::Zeichen` →
`Anwendungsdelegierter::eingabe_ausfuehren` (Fokus `Dateifenster`) →
`DateifensterQuelle::filterzeichen_tippen` → `Ordnermodell::zeichen_anhaengen` →
`filter_uebernehmen` → `sicht_neu_aufbauen`. `filter_steht` antwortet danach `true`, und
damit ist die Fallunterscheidung der Rückschritt-Taste aus C2 zum ersten Mal wirksam.

**Zwei Einschränkungen gehören dazu.** Erstens ist die Kette am Bündel nicht gefahren —
das bleibt Nutzerarbeit, wie in diesem Projekt jede Abnahme am laufenden KRK. Zweitens
hat `krk-ui` kein Bibliotheksziel, und `filterzeichen_tippen` hängt an einer
ObjC-Klasse; geprüft ist deshalb die reine Funktion `ersatzzeile` und, aus A1, die
Modellseite (`crates/krk-core/tests/verzeichnis.rs:917-956`). Die beiden Hops dazwischen
sind je ein direkter Aufruf und tragen der Übersetzung nach.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/kommandos/navigation.rs` — vom Plan im Fließtext von B1 verlangt,
  in seiner Zeile `Files:` nicht genannt; angehängt an
  `issues/260814-2357_o_c2-nennt-zwei-dateien-…`, statt einen vierten Datensatz anzulegen.

## Verification

```
make check — exit 0
```

Alle vier Abnahmekommandos grün, `clippy` unter `-D warnings`. Die vier neuen Proben
laufen als `kommandos::navigation::tests::eine_sichtbare_auswahl_bleibt_stehen`,
`…::eine_weggefallene_auswahl_geht_auf_die_erste_zeile`,
`…::ohne_bestehende_auswahl_entsteht_keine` und
`…::eine_leere_sicht_bekommt_keine_auswahl`.
