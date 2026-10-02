# Vollbaum-Durchsicht R7: der Anwendungsdelegierte (`crates/krk-ui/src/appkit/anwendung.rs`)

**Reviewed-range:** `004ff72..7ac511a`
**Not-opened:** none
**Sender:** coderev
**Datum:** 260826-1325

Vollbaum-Durchsicht ohne Codeänderung im Bereich: die Commits des Bereichs tragen nur Werkbankdateien (`git diff --stat 004ff72..7ac511a -- crates xtask resources Cargo.toml` ist leer). Gelesen ist die ganze Datei, 9.626 Zeilen, dazu die Gegenstellen in `tabelle.rs`, `leiste.rs`, `vorschau.rs`, `ereignisse.rs`, `fenster.rs`, `menue.rs`, `blaetter/mod.rs`, `blaetter/stapelumbenennen.rs`, `kommandos/zulaessigkeit.rs`, `kommandos/fokus.rs`, `kommandos/operationen.rs`, `kommandos/rueckschritt.rs`, `kommandos/kontextmenue.rs` und `krk-core/src/tasten/belegung.rs`. Jede Zeilenangabe ist am Baum abgelesen und ein zweites Mal gegengelesen.

## Summary

Der Delegierte hält, was `CLAUDE.md` über ihn zusagt: alle neun Zusagen aus dem Auftrag treffen am Baum zu, und die Auffangzweig-Frage ist beantwortet — 52 der 79 Kommandos haben hier einen eigenen Zweig, 27 fallen in `bereichskommando`, und alle 27 haben in `Tabelle::kommando_ausfuehren` einen eigenen Zweig. Was die Durchsicht gefunden hat, sitzt nicht im Zulässigkeitsgerüst, sondern an seinen Rändern: sechs Blätter ohne Griff in `offenes_blatt`, von denen eines (Stapel-Umbenennen) einen ausdrücklich bedienbaren Nicht-Textfeld-Fokus hat und dort `Esc` an `abbrechen()` verliert; eine Erfolgsmeldung, die eine Fehlermeldung überschreibt; zwei stumme `bool`-Antworten ohne `#[must_use]`; fünf verwaiste Zahlen in der Prosa.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 2 |

Vier Defektdatensätze unter `shared/issues/260826-1325_o_*`.

## Die Auffangzweig-Frage

`Anwendungsdelegierter::kommando_ausfuehren` (`anwendung.rs:3120-3408`), das `match` bei `3170-3402`, endet auf `andere => self.bereichskommando(fokus, andere)` (`3401`).

- **52 Varianten mit eigenem Zweig** (gezählt über `Kommando::… =>` in `3170-3402`, gegen die 79 Varianten aus `awk '/^pub enum Kommando/,/^}/' belegung.rs`).
- **27 Varianten im Auffang:** AlleMarkieren, AuswahlHoch, AuswahlRunter, EintragspfadKopieren, Listenanfang, Listenende, MarkierungAufheben, MarkierungUmkehren, MarkierungUmschalten, MitStandardprogrammOeffnen, Oeffnen, OrdnerAufwaerts, OrdnerpfadKopieren, Pfadeingabe, SeiteHoch, SeiteRunter, SortierrichtungUmkehren, SortierungDatum, SortierungGroesse, SortierungName, SortierungTyp, TabNaechster, TabNeu, TabVoriger, Umbenennen, VersteckteUmschalten, ZwischenablageSpringen.
- **Belegt für alle 27:** jede hat einen eigenen Zweig in `Tabelle::kommando_ausfuehren` (`tabelle.rs:1679-1712`, Auffang dort `_ => return false` bei `1714`). `Leiste::kommando_ausfuehren` führt davon AuswahlHoch und AuswahlRunter (`leiste.rs:345-349`), `Vorschaufenster::kommando_ausfuehren` TabNeu, TabNaechster, TabVoriger (`vorschau.rs:997-1013`).
- **Kein stiller Durchfall mit dem Fokus im Editor:** `bereichskommando` antwortet für `Fokus::Editor` `false` (`3460`), aber keine der 27 trägt `Ueberall`, `Dateibereiche` oder `Editor`: 22 tragen `Dateifenster`, AuswahlHoch/AuswahlRunter `Navigator`, TabNeu/TabNaechster/TabVoriger `Tabbereich` (`belegung.rs`, `fn wirkungsbereich`), und `fokus::wirkt` weist alle drei für `Editor` ab (`fokus.rs:346`, `356-358`, `364-366`). Der Zweig `Fokus::Editor => false` ist damit heute für kein Kommando erreichbar, das die Regel durchlässt; er hält die Form, nicht ein Verhalten.

Die Falle aus `CLAUDE.md` ist also am heutigen Baum geschlossen, und keine Probe hält sie — `angleichproben::der_befehl_steht_vor_dem_auffangzweig` (`8584-8598`) prüft ein einzelnes Kommando, nicht die Menge. Das ist kein neuer Befund; es ist der Stand, den `shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-…` als Lücke für die achtzigste Variante schon beschreibt.

## Die neun Zusagen aus dem Auftrag, gegen den Baum

| Zusage | Befund |
|---|---|
| Auffangzweig in `kommando_ausfuehren` | trifft zu, `3401`; Zahlen oben |
| Vier Kommandos bei stehendem Blatt; `blatt_steht` hält beide Stellen | trifft zu. `zulaessig` (`zulaessigkeit.rs:177-185`) mit `waehrend_blatt_erlaubt` (`operationen.rs:283-285`, genau `Abbrechen`) und `immer_erreichbar` (`zulaessigkeit.rs:202-207`, genau die drei). `blatt_steht` (`3005-3011`) hat drei Rufer: `lage()` (`3071`), `fokusanzeige_nachziehen` (`5063`), `beenden_erlauben` (`7606`). `eingabe_ausfuehren` liest `lage.blatt_steht` (`2967`) aus derselben Erhebung — dieselbe Abfrage, ein Weg |
| Eine Zulässigkeitsfrage, zwei Frager, dieselbe `Lage` | trifft zu. `kommando_ausfuehren` (`3139-3142`) und `validateMenuItem:` (`899-906`) rufen beide `zulaessigkeit::zulaessig(kommando, self.lage())`; keine Vorprüfung davor. Die einzige Abweichung ist gewollt: der Menüweg gibt `None` als Anschlag (`844`) |
| `ist_eigene_textflaeche`: genau zwei, Objektgleichheit, hier und nicht in `ereignisse.rs` | trifft zu. Zwei `isEqual`-Vergleiche (`2583-2596`), gereicht als Abschluss an `ersthelfer_gehoert_appkit` (`3072-3075`); `ereignisse.rs:702-718` kennt weder Editor noch Vorschau und fragt danach nur `isKindOfClass` |
| Melder: erst `aktives_dem_ersthelfer_nachziehen`, dann `fokusanzeige_nachziehen`; das zweite ruft weder `anwenden` noch `setHidden` | trifft zu. Reihenfolge `1225-1230`; Rumpf `5057-5070` schreibt `rahmen_setzen` und `titel_nachziehen`; `fokusnachzugproben` (`8691-8723`) hält es. Der eine Auslösepunkt ist `Hauptfenster::makeFirstResponder:` (`fenster.rs:226-235`) samt `becomeKeyWindow`/`resignKeyWindow` (`241-256`) |
| Kontextmenü: drei Werte, Verzweigung ohne Auffang | trifft zu. `Kontextbefehl` hat drei Varianten (`kontextmenue.rs`), `kontextbefehl_ausfuehren` (`6205-6211`) verzweigt vollständig; `kontextproben` (`9490-9529`) hält Zweig und Wirkung |
| Rückschritt-Regel mit genau einem Rufer; der `Anschlag` reist bis in den Zweig | trifft zu. Der eine Rufer außerhalb der Proben ist `papierkorb_oder_zeichen_zurueck` (`5229-5233`); `Option<Anschlag>` kommt durch `kommando_ausfuehren(kommando, anschlag)` (`3120`) in den Zweig `Kommando::InPapierkorb` (`3179`); Menü (`844`) und Bereichsleiste (`1246`) geben `None` |
| Untergrenzen-Abschnitt; nichts über macOS 15 | trifft zu. Abschnitt `168-194`. Jüngste Berührung ist `NSApplication::activate` (macOS 14, `4674`); `attachedSheet`, `firstResponder`, `isDescendantOf:`, `replyToApplicationShouldTerminate:`, `isActive`, `performClose:`, `timerWithTimeInterval:…` liegen alle unter 15 |
| Kein eigenes `#![allow(unsafe_code)]` | trifft zu. Die zwei `unsafe`-Blöcke (`1132`, `7782-7792`) laufen unter dem `#![allow(unsafe_code)]` von `appkit/mod.rs:1`; `grep -rn 'allow(unsafe_code)' crates/krk-ui/src` findet allein diese Zeile |

`expect`/`unwrap`/`panic!` außerhalb der Proben: fünf Stellen (`1676`, `1684`, `2899`, `7711`, `8174`), jede durch die Aufbaureihenfolge oder eine vorangehende Abfrage gedeckt; keine mit echtem Fehlerfall.

## Findings by theme

### 1. Blätter ohne Griff in `offenes_blatt` — Medium

`shared/issues/260826-1325_o_esc-im-stapel-umbenennen-blatt-mit-fokus-in-der-vorschautabelle-…`

Sechs Blätter, die `anwendung.rs` öffnet, liefern keinen `Blattgriff` und legen nichts in `offenes_blatt` ab: `namenseingabe` (`2011`, `2165`, `5698`), `stapelumbenennen` (`5828`), `zeilennummer` (`7172`), `suche` (`7207`). `Blatt::zeigen` lässt den Griff fallen (`blaetter/mod.rs:765`). Solange der Ersthelfer ein Textfeld ist, deckt `ersthelfer_gehoert_appkit` das ab. Das Stapel-Umbenennen-Blatt hat einen ausdrücklich bedienbaren Fokus in seiner Vorschautabelle (`stapelumbenennen.rs:35-40`); dort ist `Abbrechen` zulässig (`zulaessigkeit.rs:177-185`, `fokus.rs:345`), `abbrechen()` (`3648-3673`) findet kein Blatt und fällt auf den laufenden Vorgang oder den Filtertext des Tabs **hinter** dem Blatt, und der Abgriff schluckt `Esc` (`3407`). Die Kette ist am Baum belegt; welche Ansicht AppKit mit dem Fokus in der Tabelle als Ersthelfer führt, ist nicht am Bündel gemessen.

### 2. Erfolgsmeldung über einer Fehlermeldung — Medium

`shared/issues/260826-1325_o_lesezeichen-anlegen-meldet-angelegt-auch-wenn-das-sichern-gescheitert-ist-…`

`lesezeichen_anlegen_ausfuehren` (`2135-2148`) schreibt nach `lesezeichen_aendern` unbedingt „Lesezeichen „…“ angelegt". `lesezeichen_aendern` (`1902-1970`) hat in zwei Ausgängen (`1922-1931`, `1964-1969`) gerade eine Fehlermeldung in dieselbe Befehlsantwort gestellt; `befehlsantwort_zeigen` ersetzt (`tabelle.rs:3306-3309`). Der Nutzer liest „angelegt" über einer nicht geschriebenen Datei — und die Lesezeichen sind die Datei, deren Verlust dieses Projekt am 260820 dreizehn Beweisstücke gekostet hat.

### 3. Stumme `bool`-Antworten ohne `#[must_use]` — Low

`shared/issues/260826-1325_o_fokus-setzen-und-auftrag-starten-tragen-kein-must-use-…`

`fokus_setzen` (`2400-2420`) scheitert laut eigenem Kommentar still und trägt kein Attribut; drei Rufer werfen die Antwort nackt weg (`3210`, `4468`, `4584`), und zwei davon sind genau die Rangmitnahmen vom 260825. `auftrag_starten` hat sechs Rufer in zwei Schreibweisen (`5868` nackt, fünf mit `let _ =`). Zum Vergleich trägt `bereich_einblenden` (`4340`) das Attribut mit demselben Grund.

### 4. Verwaiste Zahlen in der Prosa — Low

`shared/issues/260826-1325_o_fuenf-zahlen-in-der-prosa-des-anwendungsdelegierten-…`

„vier Bereiche" bei `15`, `80-81`, `1135` (es sind fünf); „Drei Aufrufer" von `fokus_setzen` bei `2390` (fünf Rufstellen); „Vier Anlaesse" von `titel_nachziehen` bei `5080` (sechs Rufstellen).

## Die zwei offenen Befunde aus früheren Runden

**`260826-1223_o_kennungen-ist-die-programmweite-kommandoliste-…`** gilt unverändert; der Quelltext hat sich seit `004ff72` nicht bewegt, `KENNUNGEN` steht weiter als `[(Kommando, &'static str); 79]` (`belegung.rs:697`). Von den elf Fundstellen in `krk-ui` liegt **keine** in `anwendung.rs`; die Datei nennt `KENNUNGEN` nur in Prosa (`880`, `902`) und erreicht die Liste mittelbar über `menue::kommando_zum_tag` (`832`, `900`) und `tag_des_kommandos`. Der Zweig `None => false` bei `905` ist die Stelle, an der ein Kommando ohne Eintrag hier grau würde — folgerichtig, und ohne Absturz.

**`260826-1223_o_die-nutzerdatei-setzt-den-zusteller-frei-…`** gilt unverändert. Aus Sicht dieser Datei: ein Eintrag mit fremdem `gehalten_von` erreicht `kommando_ausfuehren` über den Abgriff nicht (Nachschlag überspringt ihn), und über das Menü auch nicht — `menue.rs:395-411` baut den Posten ohne Aktion und ohne `tag`, sobald `Funktion::kommando()` `None` liefert (`belegung.rs:1169-1174`). Die Aussage des Datensatzes, dass auch der Menüweg kein Kommando liefert, ist am Baum belegt.

## Cross-cutting observations

- **Der Fokusvorbehalt trägt Blätter mit, die nie dafür gebaut wurden.** Sechs Blätter verlassen sich darauf, dass ihr Ersthelfer ein Textfeld ist; die Zulässigkeitsregel weiß von ihnen nichts. Die sechs registrierten Blätter gehen den anderen Weg. Zwei Zuschnitte für dieselbe Sache, und der Unterschied steht an keiner Stelle ausgeschrieben.
- **Das `must_use`-Muster ist in dieser Datei dreimal gesetzt und zweimal nicht**, obwohl `fokus_setzen` die Bedingung aus `CLAUDE.md` wörtlich erfüllt. Die Kern-Datensätze vom 260826-1221/1223 zeigen dasselbe Gefälle in `krk-core`.
- **Die Zählproben halten, was sie zählen, und die Prosa daneben nicht.** `aktives_setzen`, `zettel_sichern`, `kontextmelder_setzen` haben ihre Zahl in einer Probe; `fokus_setzen` und `titel_nachziehen` haben sie nur im Kommentar, und beide sind falsch.

## Recommended sequencing

1. Befund 1 (Esc im Stapelblatt) — vor der nächsten Auslieferung am Bündel prüfen; die Behebung (Griff zurückgeben, in `offenes_blatt` legen) ist mechanisch.
2. Befund 2 (Lesezeichen „angelegt") — mit derselben Auslieferung; klein und an der teuersten Datei.
3. Befund 3 und 4 — Aufräumen, kein Blocker.

**Verification:** Datei `crates/krk-ui/src/appkit/anwendung.rs` vollständig gelesen (1-9626); Zweige mit `grep -oE 'Kommando::[A-Za-z]+ =>'` über `3170-3402` gegen `awk '/^pub enum Kommando/,/^}/'` gezählt; jede Zeilenangabe mit `sed -n`/`grep -n` am Baum abgelesen und gegengelesen; kein Kommando übersetzt, kein Quelltext geändert.
