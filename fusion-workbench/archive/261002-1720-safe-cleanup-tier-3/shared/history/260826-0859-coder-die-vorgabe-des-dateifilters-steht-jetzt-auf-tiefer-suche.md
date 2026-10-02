# coder — die Vorgabe des Dateifilters steht jetzt auf tiefer Suche

**Status:** Complete
**Agent:** coder
**Datum:** 2026-08-26 08:59

## Auftrag

Die Vorgabe des Ankreuzfelds „Deep“ von `false` auf `true` umstellen. Vorher prüfen, ob `crates/krk-core/src/verzeichnis/modell.rs` wirklich die einzige Stelle ist, an der die Vorgabe steht.

## Was geprüft wurde, bevor geändert wurde

**Die gesicherte Sitzung führt den Stand nicht mit.** `krk_core::ablage::sitzung::Tab` trägt fünf Felder (`ordner`, `auswahl`, `verstecke_ausgeblendet`, `sortierung`, `bildlauf`) und keines davon ist die tiefe Suche oder der Inhaltsfilter. `session.toml` kann die Vorgabe damit nicht überschreiben; sie greift bei jedem Start, auch bei einem Nutzer mit gesicherter Sitzung.

**Ein neuer Tab erbt nichts vom Geschwistertab.** Jeder Weg zu einem neuen `Tabinhalt` — `Tabliste::aus_zustand`, `oeffnen`, `schliessen` beim letzten Tab, `verdeckten_tab_setzen` — geht durch `Tabinhalt::aus_zustand`, und das ruft `Ordnermodell::neu`. Übernommen werden dort ausschließlich Sortierung und der Stand der versteckten Einträge. Der Ordnerwechsel innerhalb eines Tabs (`Tabliste::ordner_setzen`) trägt den Stand dagegen hinüber; das ist die bestehende Regel und wurde nicht angefasst. Der Datensatz `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_o_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md` bleibt offen und wurde nicht beantwortet.

**Das Ankreuzfeld zieht seinen Anfangszustand aus demselben Wert.** `Anwendungsdelegierter::bereichsleiste_nachziehen` liest `DateifensterQuelle::tiefe_suche_steht`, und das ist `tabs.aktiver().modell().tief()`. Ein zweiter Halter existiert nicht; die Leiste zeigt an und hält nichts. Der erste Ruf läuft am Ende von `oberflaeche_aufbauen` über `aufteilung_nachziehen`. Modell auf `true` und Häkchen leer kann es damit nicht geben.

**Keine der zehn Zeitzusagen aus C8 misst eine andere Strecke.** Gelesen wurde `crates/krk-bench/src/messen.rs` und `crates/krk-ui/src/messmodus.rs`. Keine der beiden Strecken setzt je einen Filtertext: die kopflose Strecke fährt `Ordnermodell::neu`, liest und sortiert; die Sitzungsstrecke drückt Pfeiltasten (L1 und L9 sind je „Pfeil ab“), wechselt Tabs, Fenster und Ordner und startet eine Kopie. Ohne Filtertext verlässt `Ordnermodell::zeilengrund_von` den Prüfschritt, bevor die Frage nach der Tiefe fällt, und `Tabliste::durchlauf_nachziehen` stößt keinen Durchlauf an. Gemessen wurde nichts und keine Zahl behauptet; der Befund ist am Quelltext gelesen.

## Was geändert wurde

`crates/krk-core/src/verzeichnis/modell.rs`: `tief: false` → `tief: true` in `Ordnermodell::neu`, dazu ein Abschnitt im Doc-Kommentar von `neu`, der beide Vorgaben, ihre Einzigkeit und ihre Trägheit ohne Filtertext ausschreibt, sowie ein Verweis am Feld `tief`.

`crates/krk-core/tests/verzeichnis.rs`: die neue Probe `die_tiefe_suche_ist_die_vorbelegung` hält die Vorgabe selbst und die mitverschobene Schwelle. Die beiden Helfer `gefiltert` und `handmodell` schalten die tiefe Suche jetzt ausdrücklich ab, ebenso `der_filter_wirkt_schon_beim_anhaengen_eines_stapels`: diese Proben messen den flachen Zweig und sollen ihn nennen, statt auf einer Vorgabe zu reiten.

`crates/krk-ui/src/tabs.rs`: `ein_ordnerwechsel_laesst_den_filtertext_stehen_wenn_die_tiefe_suche_aus_ist` setzt den Stand jetzt ausdrücklich; die neue Probe `ein_neuer_tab_traegt_die_vorbelegung_der_tiefen_suche` hält den Befund, dass ein neuer Tab bei der Vorgabe beginnt.

Elf Proben sind an der Umstellung rot geworden, zehn in `krk-core/tests/verzeichnis.rs` und eine in `krk-ui/src/tabs.rs`. Keine davon hat die Vorgabe geprüft; jede hat sich auf sie verlassen, während sie etwas anderes maß.

## Was offen bleibt

Die Schwelle des Inhaltsfilters hat sich mitverschoben: `filter::inhaltsschwelle(tief)` gibt fünf statt drei, und das gilt ab Werk. Die Schwelle wurde nicht angefasst. Der Datensatz `shared/decisions/260826-0859_o_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md` legt die Frage mit drei Möglichkeiten vor.

`CLAUDE.md` wurde auftragsgemäß nicht angefasst. Keine Aussage dort wird durch die Umstellung falsch; der Absatz über den Filter (Zeile 143) nennt `inhaltsschwelle` ohne Zahlen und die Regel des Ordnerwechsels, und beides gilt weiter. Was dort fehlt, ist die Vorgabe selbst.

## Abnahme

```
cargo test --workspace              exit 0
cargo clippy --workspace --all-targets  exit 0
cargo fmt --all --check             exit 0
```

Nichts committet, nichts gestaged.
