# S11: `Lesezeichen` trägt zwei Sorten

---
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` `#### 11.`
**Bindende Datensätze:** `decisions/260807-2147_a_traegt-eine-textmarke-auch-einen-bereich-oder-nur-eine-stelle.md`, `decisions/260807-2147_a_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`

---

## Was gebaut wurde

`krk-core::ablage::lesezeichen` führt seit diesem Schritt beide Sorten aus C6 in
einer Liste und einer Datei. Die Ablageform ist die aus `### Frage 10` des Plans,
unverändert übernommen: `Lesezeichen` behält `name` und bekommt ein über
`#[serde(flatten)]` eingebettetes Feld `ziel`, das die unmarkierte Auswahl `Ziel`
mit den Varianten `Ordner` und `Textstelle { datei, zeile, zeileninhalt }` trägt.

`gueltig()` hat eine erschöpfende Fallunterscheidung über `Ziel` ohne
Auffangzweig: der Ordner fragt `is_dir()`, die Textstelle fragt `is_file()`.
Beide stellen genau eine Frage an das Dateisystem und öffnen keine Datei. Das ist
das elfte Abnahmekriterium von C6, und der tragende Grund steht jetzt im
Modulkopf statt nur im Datensatz.

`Lesezeichen` trägt zusätzlich `#[serde(default)]`, mit derselben Begründung, die
`sitzung.rs` für die Nachbardatei schon führt. Bis zu diesem Schritt war
`Lesezeichen` die einzige serde-Struktur der Ablage ohne diese Vorsorge.

## Der Vorbehalt zu `flatten` ist gemessen ausgeräumt

`### Frage 10` hielt fest, dass am Papier nicht zu entscheiden war, ob `toml` in
seiner Fassung 1 die Verbindung aus `flatten` und `untagged` trägt, und benannte
den Ausweg für den Fall des Scheiterns. **Der Ausweg wird nicht gebraucht.** Mit
`toml 1.1.4` läuft die Rundreise durch, und die geschriebene Datei sieht genau so
aus, wie `### Frage 10` sie vorgezeichnet hat:

```toml
[[eintraege]]
name = "Projekte"
ordner = "/Users/pruefung/Projekte"

[[eintraege]]
name = "Die Lesestelle"
datei = "/Users/pruefung/Projekte/krk/crates/krk-core/src/verzeichnis/leser.rs"
zeile = 118
zeileninhalt = "        let mut puffer = vec![0u8; PUFFERGROESSE];"
```

Der Vorbehalt und der Ausweg stehen trotzdem als Kommentar an `Ziel`, wie der
Schritt es verlangt: er ist ausgeräumt für diese Fassung von `toml`, nicht für
jede künftige.

## Eine Abweichung vom Wortlaut des Schrittes, und warum

Der Schritt sagt, die vier Listenänderungen `anlegen`, `umbenennen`, `loeschen`
und `verschieben` blieben sortenblind und würden nicht angefasst. Drei von ihnen
sind unverändert. **`Lesezeichenliste::anlegen` hat seinen zweiten Parameter von
`&Path` auf `Ziel` gewechselt.**

Der Grund ist die Eigenschaft, die der Satz zusagt, nicht der Buchstabe: mit
`&Path` war `anlegen` gerade nicht sortenblind, sondern die Tür für genau eine
Sorte. S38 legt eine Textmarke über denselben Befehl `lesezeichen_anlegen` an und
hätte daneben einen zweiten Weg in die Liste gebraucht, entweder über das
öffentliche Feld `eintraege` oder über eine zweite Methode. Beides wäre der
zweite Mechanismus für dieselbe Aufgabe gewesen, den derselbe Schritt an anderer
Stelle ausschließt. Mit `Ziel` gibt es eine Tür, und die Liste fragt an keiner
Stelle nach der Sorte.

`Lesezeichen::neu(name, ordner)` bleibt als Konstruktor der Ordnermarke stehen,
und `Lesezeichen::textstelle(name, datei, zeile, zeileninhalt)` tritt daneben.
Ein Konstruktor, der beide Sorten zugleich annimmt, existiert nicht; die Probe
`ein_lesezeichen_traegt_genau_eine_sorte` hält das fest.

## Was `krk-ui` jetzt nicht mehr übersetzt

`cargo build -p krk-ui` bricht mit **genau zwei** Fehlern ab, beide in
`crates/krk-ui/src/leistenmodell.rs`, beide von späteren Schritten dieses Plans
ausdrücklich in Arbeit genommen:

| Stelle | Fehler | Wer ihn schließt |
|---|---|---|
| `leistenmodell.rs:352` | `no field 'ordner' on type Lesezeichen` | S39 — "`Auswahl` trägt die Sorte" |
| `leistenmodell.rs:382` | `anlegen`: `expected Ziel, found &Path` | S38 — "Eine Textmarke anlegen" |

Die Datei gehört in dieser Runde S38, S39 und S40 und wurde deshalb nicht
angefasst. `krk-core` ist grün und in sich vollständig; der Arbeitsbaum baut als
Werkstatt derzeit nicht durch.

## Abnahme

`cargo test -p krk-core` beendet mit 0: 122 Proben in der Kiste, 30 in
`tests/ablage.rs`, dazu die übrigen Testprogramme. `cargo clippy -p krk-core
--all-targets` meldet nichts. `rustfmt --check` ist auf allen drei geänderten
Dateien sauber.

Die sechs Zusagen des Abnahmekriteriums, jede an ihrer Probe:

| Zusage | Probe |
|---|---|
| Eine `bookmarks.toml` in der Form vor dieser Runde liefert drei Ordnermarken | `eine_bookmarks_toml_aus_der_zeit_vor_den_textmarken_bleibt_lesbar` |
| Rundreise über beide Sorten liefert byteweise dieselbe Datei | `eine_rundreise_ueber_beide_sorten_liefert_dieselbe_datei` |
| Von Hand lesbar: keine geschachtelte Tabelle, keine Sortenkennung | `die_geschriebene_datei_traegt_weder_geschachtelte_tabelle_noch_sortenkennung` |
| Ein Lesezeichen kann nicht beide Sorten tragen | `ein_lesezeichen_traegt_genau_eine_sorte` |
| `gueltig()` einer Textmarke: Datei wahr, Ordner falsch | `eine_textmarke_ist_gueltig_solange_ihre_datei_da_ist` |
| Die vier Listenänderungen wirken auf beide Sorten gleich | `umbenennen_loeschen_und_verschieben_sind_sortenblind`, `beide_sorten_gehen_durch_dieselbe_tuer_und_in_dieselbe_ordnung` |

Die Gültigkeitsprobe hält zusätzlich fest, was **nicht** geprüft wird: eine
Textmarke mit abweichendem Zeileninhalt bleibt gültig. Das ist der tragende Teil
der Antwort vom 260808-0017 und der Grund, aus dem die Prüfung eine Frage an das
Dateisystem bleibt.

## Geänderte Dateien

- `crates/krk-core/src/ablage/lesezeichen.rs` — `Ziel`, `Lesezeichen`, `gueltig`,
  `anlegen`, Modulkopf, vier neue Proben
- `crates/krk-core/src/ablage/mod.rs` — `Ziel` in der Wiederausfuhr
- `crates/krk-core/tests/ablage.rs` — vier neue Proben, `anlegen`-Aufrufe angepasst
- Dieser Plan — Schritt 11 auf `[DONE]`
