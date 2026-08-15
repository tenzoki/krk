# Drei Kommentarbefunde der Durchsicht

**Status:** Complete
**Agent:** coder
**Anlass:** Aufgabe T4, Befunde 1, 2 und 4 der Durchsicht `shared/reviews/260815-1047-coderev-der-filtertext-uebersteht-jeden-ordnerwechsel.md`; Nutzerlinie vom 260815-1055: den Zustand ehrlich festhalten, das Verhalten nicht ändern

---

## Was geändert wurde

Reine Kommentar- und Verweisarbeit. Kein Verhalten, keine Signatur, keine Probe
angefasst. Der Rumpf von `Tabliste::ordner_setzen` steht unverändert da; geändert
sind der Doc-Kommentar darüber, ein Kommentar im Rumpf, zwei Doc-Kommentare an
Proben und vier Markerzitate in drei Dateien.

## 1. Der Sichtbarkeitsvorbehalt steht jetzt im Doc-Kommentar

Der Absatz behauptete: „Getragen wird sie davon, dass der stehende Filtertext zu
sehen ist: die Statuszeile nennt ihn samt Trefferzahl." Belegt war das an
`statuszeile::filterstand_text`, das den Satz **baut**, und nicht an
`statuszeile::zeile`, das über `Rang::ALLE` entscheidet, ob er die Zeile
erreicht.

An seiner Stelle steht ein eigener Abschnitt mit der Überschrift „Die
Sichtbarkeit des stehenden Filtertextes ist nicht zugesagt". Er nennt die
Rangfolge (Filterstand ist Rang 5 von 6), die Ordnung „erst der Rang, dann die
aktive Seite", die Fenstermeldung des inaktiven Dateifensters auf Rang 3 als den
Weg, auf dem der Filterstand dauerhaft verschwindet, und den Unterschied
zwischen Weg und Häufigkeit: der Weg besteht seit der Runde 10, was der 260815
ändert, ist die Häufigkeit. Der offene Datensatz
`shared/issues/260815-1047_o_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md`
ist am Ende des Abschnitts genannt.

## 2. Die Aufzählung der Löschwege ist geöffnet

Der Kommentar sagte: „Geloescht wird er allein vom Nutzer, mit `Esc` oder Zeichen
fuer Zeichen ueber die Rueckschritt-Taste." Der Baum kennt drei weitere Wege.

Der neue Abschnitt trägt die Überschrift „Die Wege, auf denen der Filtertext
verschwindet, sind nicht abschliessend aufgezaehlt" und folgt dem Wortlaut, den
C1.9 des Spec seit dem 260815 führt: kein Ordnerwechsel und keine Auffrischung
löscht ihn, der Nutzer nimmt ihn mit `Esc` oder der Rückschritt-Taste weg,
daneben fällt er mit dem Tab und mit der Sitzung. Die drei weiteren Wege stehen
namentlich da, jeder mit seiner Stelle:

```text
  Tabliste::schliessen            letzter Tab, frischer Tabinhalt
  Tabliste::verdeckten_tab_setzen Auswurf unter einem verdeckten Tab
  krk_core::ablage::sitzung::Tab  fuehrt den Filtertext nicht  ──> Neustart
```

`Tabliste::schliessen` und `verdeckten_tab_setzen` sind **nicht** angefasst.

## 3. Sieben Markerzitate berichtigt, vier Datensätze betroffen

Alle 17 ausgeschriebenen Markerzitate unter `crates/` und `xtask/` sind gegen den
Dateibestand gehalten worden. Sieben Fundstellen nannten einen Marker, den ihr
Ziel nicht mehr trägt:

| Fundstelle | zitiert | gesetzt |
|---|---|---|
| `crates/krk-ui/src/tabs.rs` (3 Stellen) | `260814-1830_a_bleibt-der-filtertext-…` | `_i_` |
| `crates/krk-ui/src/appkit/editor.rs:525` | `260807-2147_a_welche-dateien-oeffnet-der-editor-…` | `_i_` |
| `crates/krk-ui/src/appkit/editor.rs:1054`, `:2020` | `260810-0303_o_ein-ersetzen-und-ein-eingefuegtes-crlf-…` | `_c_` |
| `crates/krk-ui/src/appkit/anwendung.rs:2835` | `260814-2102_a_gehoert-die-fallunterscheidung-…` | `_i_` |

Die zehn übrigen stimmen und sind unverändert geblieben. Über `crates/` hinaus
tragen nur `spikes/` und `messungen/` solche Zitate; beide sind Aufzeichnungen
eines Standes und behalten nach der Ortsregel aus `CLAUDE.md` ihren damaligen
Marker.

**Abweichung vom Datensatz.** `shared/issues/260815-1047_o_vier-verweise-…`
empfiehlt unter „Was zu tun ist" die Sternform `_*_` statt des heutigen Markers,
weil ein ausgeschriebenes Zitat auf dem Weg `_o_` → `_a_` → `_i_` zweimal falsch
wird. Die Aufgabe T4 verlangt ausdrücklich den heute getragenen Marker, und so
ist es gemacht. Die Wahl zwischen beiden Formen bleibt damit offen.

## 4. Ein falscher Typname

`crates/krk-ui/src/tabs.rs`, Doc-Kommentar der Probe
`der_aufstieg_laesst_den_filtertext_stehen_wie_der_einstieg`: `Dateifenster`
existiert und hat kein `ordner_aufwaerts`; die Methode gehört
`DateifensterQuelle` (`crates/krk-ui/src/appkit/tabelle.rs:1386`). Berichtigt.

## 5. Der Datensatz zu `verdeckten_tab_setzen`

`circles/260814-1551-…/issues/260815-0020_o_verdeckten-tab-setzen-…` ist um einen
Nachtrag ergänzt und bleibt offen. Berichtigt sind zwei Aussagen: die
Befundtabelle führte für `ordner_setzen` „ja, wenn der Filter der Tiefe an ist"
(jetzt „ja, unbedingt", beide Zeilennummern auf dem heutigen Stand), und die
Einordnung als „unentschieden" stützte sich auf den alten Wortlaut von C1.10.
Mit der unbedingten Regel aus C1.9 ist der Weg ein Widerspruch zum Wortlaut. Die
Entwurfsfrage des Datensatzes, ob die vier Werte an einer Stelle übertragen
werden, ist unbeantwortet und bleibt es.

## Abnahme

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo test --workspace -- --skip ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` | 0 |
| `cargo test --workspace` | 0 |

Der volle Lauf ist **grün**, und das ist gegenüber der Aufgabenstellung eine
Abweichung: sie sagt, `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`
reiße die 15-Sekunden-Schranke in sechs Läufen von sechs
(`shared/issues/260815-1019_o_die-wettrennprobe-…`). In diesem Lauf ist die Probe
durchgelaufen (`test … ok`). Ein Lauf widerlegt keine Streuung; der Datensatz
bleibt offen, und die Beobachtung gehört dorthin.

## Was offen bleibt

- Die drei Datensätze der Durchsicht sind **nicht** auf `_c_` gezogen. Für
  Befund 1 verlangt Möglichkeit 3 des Datensatzes den Vorbehalt an zwei Stellen,
  im Doc-Kommentar und in den `## Constraints` des Entscheidungsdatensatzes
  `circles/260814-1551-…/decisions/260814-1830_i_bleibt-der-filtertext-…`. Die
  erste Stelle steht, die zweite ist Werkbankarbeit und nicht Teil von T4.
- Befund 3 der Durchsicht (Directive der Runde 10 und Planschritt B2) ist eine
  eigene Aufgabe und hier nicht angefasst.
