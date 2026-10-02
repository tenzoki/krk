# Der Filtertext übersteht jeden Ordnerwechsel

**Status:** Complete
**Agent:** coder
**Anlass:** Aufgabe T2, Nutzerentscheid vom 260815-0955 zu `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_a_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md` (Möglichkeit 2)

---

## Was gebaut wurde

`Tabliste::ordner_setzen` (`crates/krk-ui/src/tabs.rs`) überträgt den Filtertext
jetzt unbedingt in das neue `Ordnermodell`. Bisher hing die Übertragung am
Kennzeichen „Deep":

```text
  vorher:  tief == true  ──> Filtertext geht hinüber
           tief == false ──> String::new()

  jetzt:   Filtertext geht hinüber
```

Damit stehen vier Übertragungen in derselben Bauart nebeneinander: Sortierung,
Verstecke, der Filter der Tiefe und der Filtertext. Keine trägt eine Bedingung.

## `filtertext_ueberlebt` ist ersatzlos entfallen

Die Variable war der Platzhalter für die offene Antwort: `let
filtertext_ueberlebt = tief;`, mit einem Kommentar darüber, der den Fall „stehen
lassen" vorwegnahm. Mit der einen Regel trüge sie nur noch ein `true` und liesse
eine Fallunterscheidung vermuten, die es nicht mehr gibt. Der Kommentar an ihrer
Stelle nennt den Nutzerentscheid und sagt ausdrücklich, dass die Variable
entfallen ist — so findet der nächste Leser die Herkunft, ohne dass eine
Bedingung im Code stehen bleibt, die nie falsch wird.

## Drei Kommentare sind nachgezogen

**Der Doc-Kommentar von `ordner_setzen`.** Die Überschrift lautete „Die eine
Stelle, an der ein Ordnerwechsel den Filter **entscheidet**"; entschieden wird
dort nichts mehr, also heißt sie jetzt „…den Filter **trägt**". Der Abschnitt
schreibt die Regel aus, nennt den Datensatz unter seinem neuen Marker (`_a_`),
hält fest, dass sie erst seit dem 260815-0955 gilt, und nennt die Bedingung, an
der sie hängt: die Statuszeile zeigt den stehenden Filtertext samt Trefferzahl.

**Der Vorbereitungskommentar im Rumpf.** Er beschrieb den vergangenen Zustand
(„Der Plan fährt auf ‚geleert'") und ist auf die gefallene Antwort umgeschrieben.

**Der Doc-Kommentar von `aktiven_neu_lesen`.** Er sagte, die Regel aus
`ordner_setzen` greife bei einer Auffrischung nicht, „gleich ob der Filter der
Tiefe an ist" — eine Formulierung, die die alte Fallunterscheidung
mittransportierte. Jetzt steht dort der eigentliche Grund: eine Auffrischung
wechselt den Ordner nicht, also fällt das `Ordnermodell` gar nicht erst.

## Die Proben

| Probe | Was sie prüft |
|---|---|
| `ein_ordnerwechsel_laesst_den_filtertext_stehen_wenn_die_tiefe_suche_aus_ist` | C1.9, flacher Fall. Umgeschrieben: sie prüfte bis zum 260815 das Gegenteil (Name war `…leert_den_filtertext…`). |
| `der_aufstieg_laesst_den_filtertext_stehen_wie_der_einstieg` | **Neu.** C1.9, Aufstieg. Fährt ihn wie `Dateifenster::ordner_aufwaerts`: mit `krk_core::verzeichnis::aufwaerts` gerechnet, der verlassene Ordner als `auswahl`. Prüft daneben, dass die Wunschauswahl weiter auf dem verlassenen Ordner steht. |
| `mit_tiefer_suche_ueberlebt_der_filtertext_den_ordnerwechsel` | C1.10, tiefer Fall. Unverändert im Rumpf; der Doc-Kommentar sagt jetzt, dass es kein eigener Fall mehr ist, sondern derselbe wie C1.9. |

Unangetastet geblieben sind `die_tiefe_suche_geht_auch_ohne_filtertext_hinueber`,
`eine_auffrischung_laesst_den_filtertext_stehen` und
`der_filtertext_gehoert_dem_tab_und_nicht_dem_fenster`. Alle drei prüfen
Aussagen, die die neue Regel nicht berührt.

Eine Probe, die das bisherige Leeren festschrieb, gibt es außerhalb von
`tabs.rs` nicht. Gesucht wurde über `Ordnerwechsel`, `Filtertext`, `geleert` und
`leert` im ganzen Baum. Der Kommentar in `crates/krk-core/tests/verzeichnis.rs`
über „Schritt F2" („trägt sie den Filtertext über den Ordnerwechsel") ist unter
der neuen Regel unverändert richtig und blieb stehen.

## Was ausdrücklich nicht angefasst wurde

`Ordnermodell::sichtbar` (`crates/krk-core/src/verzeichnis/modell.rs`), das
Ankreuzfeld selbst, die Rangfolge von `Esc` und der Durchlauf über den
Unterbaum. Die zwei anderen Datensätze, die der Baum unter dem Marker `_o_`
zitiert (`…gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster` und
`…an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext`), stehen
weiterhin offen; ihre Zitate sind richtig und blieben unverändert.

## Abnahme

| Kommando | Exit | Anmerkung |
|---|---|---|
| `make check` | 0 (Lauf 1), 2 (Lauf 2) | schwankt allein an der Probe unten |
| `cargo test --workspace -- --skip ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` | 0 | jede andere Probe des Baums läuft durch |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | keine Warnung |
| `cargo fmt --all --check` | 0 | |

**Der schwankende Ausfall stammt nicht aus dieser Arbeit.**
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`
(`crates/krk-core/tests/text.rs`) fällt im Profil `debug` an einer Zeitschranke
von 15 s aus, nicht an dem Verhalten, das die Meldung nennt. Der Ausfall besteht
am unveränderten Baum: mit `git stash` auf `f8297b6` zurückgesetzt fällt dieselbe
Probe genauso aus, und im Profil `release` läuft sie in 4,66 s durch. Aufgenommen
als
`shared/issues/260815-1019_o_die-wettrennprobe-des-oeffnens-faellt-im-profil-debug-an-der-15-sekunden-schranke-aus.md`.

**`make check` ist damit kein grüner Beleg für diese Aufgabe**, solange die Probe
nicht berichtigt ist. Was diese Aufgabe trägt, belegt die zweite Zeile der
Tabelle zusammen mit `clippy` und `fmt`.
