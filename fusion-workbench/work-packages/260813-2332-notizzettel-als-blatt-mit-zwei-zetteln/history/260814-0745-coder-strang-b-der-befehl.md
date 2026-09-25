# Coder — Strang B: der Befehl kommt auf zwei Wegen

**Date:** 260814-0745
**Agent:** coder
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Plan:** `planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, Schritte 6, 7 und 8
**Status:** Complete

---

## Auftrag

Strang B des Plans, und nur er: die Auslieferungsbelegung führt den Notizzettel
(Schritt 6), `Kommando::Notizzettel` zieht die zwei vollständigen
Fallunterscheidungen nach (Schritt 7), und drei Proben halten fest, dass die
Zulässigkeitsregel dabei unangetastet bleibt (Schritt 8).

Ausdrücklich nicht angefasst: `crates/krk-core/src/ablage/` und
`crates/krk-core/src/text/`. Dort arbeitet gleichzeitig ein zweiter coder an
Strang A.

## Was gebaut ist

### Schritt 6 — `resources/default-keymap.toml`

Ein `[[funktion]]`-Block mit `id = "notizzettel"`, `name = "Notizzettel
anzeigen"` und `tasten = ["f2", "cmd+k"]`. Er steht zwischen `belegung_ansehen`
und `weitere_instanz`, mit einem eigenen Abschnittskopf unmittelbar über
`# ── Eine weitere Instanz von KRK ──`. Der bestehende Abschnitt zu den
Textbefehlen des Menüs „Bearbeiten" bleibt damit als Block zusammen; die
Reihenfolge im Anwendungsmenü ist trotzdem die zugesagte, weil jene Textbefehle
einem anderen Funktionsbereich angehören.

Der Kommentar begründet die zwei Wege: F2 aus der Norton-Reihe, `cmd+k` als
Mac-Weg auf dieselbe Funktion, in **einer** Zeile nach der Ein-Zeilen-Regel aus
C3. Er nennt den Nutzerentscheid vom 260802-1409 mit seinem Datensatz
(`shared/decisions/260802-0842_*_f-tasten-unter-macos-systembelegung.md`, dort
als Nachtrag).

Die Kopfzeile ist von „82 Funktionen mit zusammen 88 Kombinationen" auf 83 und
90 nachgezogen.

**Die Freiprüfung vor dem Eintrag, die der Plan verlangt, ist gefahren.** Über
alle Tastenlisten der Datei ausgezählt: `f2` und `cmd+k` kamen kein einziges Mal
vor. Belegt ist allein `shift+cmd+k`, und das trägt `kopieren` neben `f5`. Die
Zählung danach ergibt 83 Funktionen und 90 Kombinationen, und die einzige
doppelt vorkommende Kombination ist `cmd+a` — sie stand schon vorher zweimal da,
mit zwei verschiedenen Zustellern, und die Datei schreibt an Ort und Stelle aus,
warum das kein Konflikt ist (Nutzerentscheid vom 260805).

### Schritt 7 — die Variante und die zwei Fallunterscheidungen

`crates/krk-core/src/tasten/belegung.rs`:

- Neue Variante `Kommando::Notizzettel` am Ende der Aufzählung. Ihr
  Doc-Kommentar nennt die zwei Kombinationen und hält fest, dass der Befehl den
  Zettel **nicht** schließt: bei stehendem Blatt weist ihn `zulaessig` ab, weil
  `waehrend_blatt_erlaubt` allein den Abbruch nennt.
- `KENNUNGEN` wächst von 76 auf 77 Paare, mit `(Kommando::Notizzettel,
  "notizzettel")`.
- `Kommando::wirkungsbereich` ordnet sie im Zweig „das Fenster als ganzes"
  `Wirkungsbereich::Ueberall` zu. Der Kommentar nennt den Grund: der Zettel
  fährt als Blatt herunter und gehört keinem der fünf Bereiche; ein
  Wirkungsbereich, der einen davon verlangte, schnitte die anderen vier ab.

`crates/krk-ui/src/belegungsmodell.rs`:

- `bereich_des_kommandos` ordnet sie `Funktionsbereich::Anwendung` zu, neben
  `BelegungAnsehen`, `Beenden` und `WeitereInstanz`. Der Kommentar sagt, warum
  kein eigener Funktionsbereich entsteht: er ergäbe ein Obermenü mit einem
  einzigen Eintrag.

Weder `belegungsansicht.rs` noch `belegungsausgabe.rs` noch `menuemodell.rs`
sind angefasst. Dass die drei Flächen die Zeile trotzdem führen, belegen die
bestehenden Proben, die grün durchlaufen —
`jede_kennung_hat_einen_funktionsbereich`,
`jede_kennung_ohne_kommando_wird_vom_menue_zugestellt` und
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`.

### Schritt 8 — drei Proben zur Zulässigkeit

`crates/krk-ui/src/kommandos/zulaessigkeit.rs`:

- `der_notizzettel_kommt_bei_stehendem_blatt_nicht_durch` — drei Zusicherungen
  nennen die Herleitung (`Wirkungsbereich::Ueberall`, nicht auf
  `immer_erreichbar`, nicht in `waehrend_blatt_erlaubt`), danach läuft eine
  Schleife über alle fünf Fokuswerte und hält beide Richtungen fest: ohne Blatt
  wirkt der Befehl aus jedem Fokus, mit stehendem Blatt aus keinem.
- `die_ausnahmeliste_fuehrt_dieselben_drei_befehle_wie_vor_dieser_runde` — der
  Durchgang geht über alle Kommandos aus `KENNUNGEN` und zählt, welche
  `immer_erreichbar` bejaht. Erst Länge 3 und die drei namentlichen
  Zusicherungen zusammen sagen „genau diese drei".
- `im_textfeld_eines_blattes_ist_auch_der_abbruch_abgewiesen` ist unverändert
  geblieben und hat einen Absatz im Doc-Kommentar bekommen: an genau dieser
  Abweisung hängt der Weg zurück aus dem Zettel.

`crates/krk-ui/src/kommandos/operationen.rs`:

- `waehrend_eines_blattes_bleibt_es_bei_dem_einen_abbruch` — derselbe Durchgang
  über alle Kommandos, mit `vec![Kommando::Abbrechen]` als Erwartung, dazu eine
  eigene Zeile für den Notizzettelbefehl. Sie steht da, obwohl der Durchgang ihn
  schon deckt: ein Eintrag für ihn wäre der naheliegende Griff, um den Zettel
  mit derselben Taste wieder zu schließen, und genau das lässt diese Runde sein.

`immer_erreichbar`, `waehrend_blatt_erlaubt` und `zulaessigkeit::zulaessig`
selbst sind unverändert.

## Was bewusst offen bleibt

**`Kommando::Notizzettel` tut noch nichts.** Der eigene Zweig in
`Anwendungsdelegierter::kommando_ausfuehren` ist Schritt 12 des Plans und
gehört zu Strang C; er hängt an vier Vorgängern aus drei Strängen. Bis dahin
fällt der Befehl durch den Auffangzweig `andere => self.bereichskommando(fokus,
andere)` und bewirkt nichts, und der Übersetzer sagt dazu kein Wort — der Plan
hält das als eigene Feststellung fest, und die Ausgangslage nennt es die
wichtigste der Stellen, an denen der Bau **nicht** anhält.

**Ein Nutzer mit eigener `keymap.toml` bekommt den Notizzettel unbelegt.**
`Belegung::bauen` setzt für eine Funktion, die die Nutzerdatei nicht nennt,
`tasten: Vec::new()`. Das ist der gefilte Defekt
`shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`
und nicht Gegenstand dieser Runde.

## Prüfung

`make check` fährt Bau, Proben, `clippy` und `fmt` in einem Zug.

**Im Arbeitsbaum war der Lauf nicht aussagekräftig**, und zwar aus einem Grund,
der nichts mit diesen Schritten zu tun hat: der zweite coder arbeitet
gleichzeitig an Strang A, und `crates/krk-core/src/text/datei.rs` stand zu jedem
Zeitpunkt meiner Läufe in einem Zwischenstand — erst ein ungenutztes `Seek` im
`use`, dann eine `fmt`-Abweichung, dann vier Übersetzungsfehler an der noch
nicht umgestellten Signatur von `atomar::schreiben`. Keiner der drei Befunde
berührt eine Datei dieses Strangs.

**Geprüft ist deshalb in einem eigenen Baum.** Ein `git worktree` auf `HEAD`
(`edea4d9`), darin allein die fünf Dateien dieses Auftrags, sonst nichts:

```
make check   →  exit 0,  "alle vier gruen"
```

Die drei neuen Proben laufen dort namentlich grün, ebenso die vier bestehenden,
die an den geänderten Zählständen und Zuordnungen hängen:

```
tasten::belegung::tests::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch ... ok
tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung ... ok
belegungsausgabe::tests::jede_kennung_ohne_kommando_wird_vom_menue_zugestellt ... ok
belegungsmodell::tests::jede_kennung_hat_einen_funktionsbereich ... ok
kommandos::operationen::tests::waehrend_eines_blattes_bleibt_es_bei_dem_einen_abbruch ... ok
kommandos::zulaessigkeit::tests::der_notizzettel_kommt_bei_stehendem_blatt_nicht_durch ... ok
kommandos::zulaessigkeit::tests::die_ausnahmeliste_fuehrt_dieselben_drei_befehle_wie_vor_dieser_runde ... ok
```

Der Prüfbaum ist danach wieder abgeräumt (`git worktree remove`); `git worktree
list` führt allein das Projektverzeichnis.

**Was diese Prüfung nicht leistet:** sie sagt nichts darüber, ob Strang A und
Strang B zusammen grün sind. Das entscheidet der erste `make check` im
Arbeitsbaum, nachdem der zweite coder seinen Strang abgeschlossen hat.

Kein `make bundle`, kein `cargo xtask bundle`, kein `cargo xtask release` — wie
beauftragt.

## Geänderte Dateien

- `/Users/k1/Projects/productive/krk/resources/default-keymap.toml`
- `/Users/k1/Projects/productive/krk/crates/krk-core/src/tasten/belegung.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/belegungsmodell.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/kommandos/zulaessigkeit.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/kommandos/operationen.rs`
- `/Users/k1/Projects/productive/krk/fusion-workbench/circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`
  (Schritte 6, 7 und 8 auf `[DONE]`)
