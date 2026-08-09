# S9: Das Einlesen und die Sicherungsform

**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 9 (Phase B)
**Bindende Grundlage:** `decisions/260808-0021_a_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`

## Was entstanden ist

| Datei | Inhalt |
|---|---|
| `crates/krk-core/src/text/datei.rs` (neu) | `einlesen`, `in_gehaltene_form`, `sicherungsform`, `sichern` |
| `crates/krk-core/src/text/mod.rs` | `pub mod datei;`, Schaubild und Kopf um die beiden Enden erweitert |
| `crates/krk-core/tests/text.rs` | `Pruefordner` und sieben Proben zu S9 |

Vier Funktionen, und zwischen ihnen genau eine Zusage:

```
Bytes ──> einlesen ──> in_gehaltene_form ──> Stand ──> sicherungsform ──> sichern
                              ^                                              │
                    jeder Text von anderswo (S37)                 ablage::atomar
```

**Der gehaltene Stand des Editors ist gültiges UTF-8 ohne Bytefolgenmarke und
mit `\n` als einzigem Zeilenende.** Das Einlesen stellt sie her, deshalb stellt
das Sichern sie nicht noch einmal her. `sicherungsform` wandelt keine
Zeilenenden; wer dort danach sucht, sucht eine Zeile zu spät, und der Modulkopf
sagt es ihm.

## Der Preis steht im Code, nicht nur im Datensatz

Der Nutzer hat am 260808-0043 Möglichkeit 2 gewählt und ist der Empfehlung des
Datensatzes nicht gefolgt. Der Modulkopf von `datei.rs` trägt einen eigenen
Abschnitt „Der Preis dieser Wahl, ausgeschrieben" mit beiden Folgen: das
Sichern ändert jede Zeile einer Windows-Datei, und eine fremde Datei aus einem
Windows-Projekt kommt verändert zurück. Dazu der Satz, der die nächste Sitzung
davon abhalten soll, hier einen Sonderfall einzuziehen: wer diesen Kopf liest,
weil ein Nutzer sich über genau diese Wirkung beschwert, hat den richtigen Ort
gefunden und die falsche Erwartung. Die Antwort wäre eine neue Frage an den
Nutzer, kein Zweig hier.

## Die Festlegungen, die der Plan offen ließ

- **`einlesen` nimmt Bytes und keinen Pfad, `sichern` nimmt einen Pfad.** Die
  Unwucht ist Absicht und im Kopf begründet: die Größen- und Typprüfung aus C2
  muss vor dem Lesen laufen, sonst steht eine zu große Datei doch im Speicher.
  Ein `lesen(pfad)` hier wäre die zweite Stelle, die eine Datei öffnet, und die
  erste ohne Prüfung. S10 setzt die Prüfung davor und macht daraus den einen
  Weg.
- **Der leere Stand bleibt leer.** Die Fallunterscheidung in `sicherungsform`
  hat drei Zweige, überschneidungsfrei und vollständig: leer bleibt leer (eine
  Datei ohne Zeile braucht keinen Zeilenabschluss, und ein angehängtes `\n`
  machte aus null Bytes eines), ein Stand auf `\n` geht unverändert hinaus, alles
  Übrige bekommt genau einen `\n`. Ohne den ersten Zweig wäre das
  Abnahmekriterium „byteweise unverändert" für die leere Datei nicht zu halten.
- **Hinten wird nicht aufgeräumt.** Ein Stand, der auf mehrere `\n` endet,
  behält sie. „Genau ein abschließender Umbruch" heißt, dass genau einer
  **angehängt** wird, nicht dass leere Zeilen am Dateiende verschwinden; die
  sind Text des Nutzers.
- **Abgeschnitten wird allein die führende Bytefolgenmarke.** Ein `U+FEFF`
  mitten im Text ist ein Leerzeichen ohne Breite und bleibt stehen.
- **`sichern` stellt keine Marke ab, es schreibt nur keine.** Der Datensatz sagt
  „nie eine Bytefolgenmarke schreiben", nicht „eine vorhandene entfernen". Der
  Stand trägt am Anfang keine, weil `einlesen` sie abgeschnitten hat; was der
  Nutzer dort selbst hinschreibt, ist sein Text.
- **`einlesen` liefert `Option<String>` und keinen Fehlerwert.** Der Fehler von
  `String::from_utf8` trägt nichts, was ein Aufrufer benutzt. Welchen Satz der
  Nutzer liest, entscheidet der Abweisungsgrund aus S10, und der ist die eine
  Stelle dafür.

## Der Hinweis aus S8 ist eingelöst

`text::suche` normalisiert nichts, und das bleibt so. Die eine Stelle, die
`\r\n` zu `\n` macht, ist `datei::in_gehaltene_form`, und sie ist öffentlich,
damit S37 den Ersatztext des Suchen-und-Ersetzens hindurchführen kann. Der
Modulkopf nennt S37 namentlich, und die Probe
`ein_ersatztext_geht_durch_dieselbe_stelle_wie_das_eingelesene` führt einen
Ersatztext mit `\r\n` durch `in_gehaltene_form` und danach durch
`suche::alle_ersetzen`.

Der Plan sagt in S37 „hier ist nichts Eigenes zu tun als es nicht zu brechen".
Das stimmt für den Stand; für den **Ersatztext**, der aus einem Eingabefeld
kommt und ein hineinkopiertes `\r` tragen kann, stimmt es nur, wenn er durch
diese Stelle geht. Der Kopf von `datei.rs` sagt das, damit S37 es nicht selbst
herleiten muss.

## Wie das abgeleitete Abnahmekriterium abgedeckt ist

Das Kriterium: „Beim Sichern schreibt der Editor ausschließlich `\n` als
Zeilenende, hängt genau einen abschließenden `\n` an, wenn der Stand keinen
trägt, und schreibt keine Bytefolgenmarke an den Dateianfang, unabhängig von der
Form, die die geöffnete Datei mitbrachte."

`die_drei_abweichungen_verschwinden_beim_lesen_und_kommen_nicht_zurueck` legt
eine Prüfdatei an, die alle drei Abweichungen zugleich trägt (`EF BB BF`, CRLF,
kein abschließender Umbruch), prüft zuerst, dass die Probe selbst trägt, was sie
zu tragen behauptet, liest ein, sichert **ohne jede Änderung am Stand** und
prüft die Bytes auf der Platte auf alle vier Zusagen: kein `0x0D`, letztes Byte
`0x0A`, die letzten zwei Bytes nicht `0x0A 0x0A`, die ersten drei nicht
`EF BB BF`. Dazu der vollständige Bytevergleich und die Kontrolle, dass die
Nachbardatei aus `ablage::atomar` nicht liegengeblieben ist.

Die zweite geforderte Probe,
`die_zielform_ueberlebt_die_rundreise_byteweise`, führt vier Dateien in der
Zielform durch Einlesen und Sichern und vergleicht byteweise: eine gewöhnliche
mit Umlauten und Emoji, eine mit leeren Zeilen am Ende, die leere und eine
einzeilige.

Fünf Proben stehen daneben, weil sie Zweige decken, die die beiden geforderten
nicht erreichen: der einzelne Wagenrücklauf alter Mac-Dateien und die gemischte
Datei, die Marke mitten im Text, ungültiges UTF-8, der Ersatztext aus S37 und
die drei Zweige von `sicherungsform` ohne eine einzige Datei.

## Die vier Abnahmekommandos

Gefahren am 260809-1105, alle grün:

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 15 Testziele, `tests/text.rs` jetzt 12 statt 5 Proben |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |
| `cargo fmt --all --check` | sauber |

## Was mitgezogen wurde und was liegenbleibt

**Mitgezogen, im Umfang:** der Kopf von `text/mod.rs`. Sein Schaubild kannte die
beiden Enden nicht, und der Satz „Kein Zustand und kein Dateisystem. Jede
Funktion dieses Verzeichnisses … liest keine Datei und schreibt keine" wurde mit
`datei.rs` falsch. Er heißt jetzt „Kein Zustand, und das Dateisystem an genau
einer Stelle" und benennt `datei` als die Ausnahme.

Der Kopf von `tests/text.rs` sagte „Keine Datei und kein Prüfordner". Er führt
jetzt beide Fallgruppen und begründet, warum die Fälle ab 6 einen Prüfordner
brauchen.

**Gleichzeitig gearbeitet:** S12 hat während dieses Schrittes `pub mod marke;`
in dieselbe `text/mod.rs` eingetragen. Beide Änderungen stehen nebeneinander,
nichts wurde überschrieben. Der Kopf beschreibt `marke` noch nicht; das gehört
S12 und wurde hier nicht angefasst.

**Liegengeblieben, außerhalb des Umfangs:** die sieben Probenordner in
`crates/krk-ui/src/vorschaumodell.rs` tragen feste Namen im
Temporärverzeichnis, gegen die Form, die `CLAUDE.md` für Prüfordner festlegt.
Zwei Proben daraus scheiterten in einem Lauf während dieser Sitzung und waren
davor und danach grün. Datensatz:
`shared/issues/260809-1106_o_die-probenordner-der-vorschau-tragen-feste-namen-im-temporaerverzeichnis.md`.
Er hält ausdrücklich fest, dass der naheliegende Weg (zwei gleichzeitige
`cargo test` in derselben `$TMPDIR`) sich in 240 Wiederholungen **nicht**
erzwingen ließ und deshalb Verdacht und nicht Ursache ist.

## Was S10 hier vorfindet

`text/datei.rs` ist die Datei, die S10 erweitert. Es findet vor: die vier
Funktionen, einen Kopf mit dem Abschnitt „Was hier nicht steht", der den Platz
der Prüfung schon benennt, und `einlesen`, das Bytes nimmt — die Prüfung schiebt
sich davor, ohne dass eine Zeile zu ändern wäre.
