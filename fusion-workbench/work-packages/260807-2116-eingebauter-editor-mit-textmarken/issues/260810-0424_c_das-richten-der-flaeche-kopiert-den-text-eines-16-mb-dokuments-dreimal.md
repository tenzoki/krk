# Das Richten der Fläche kopiert den Text eines 16-MB-Dokuments dreimal

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:1197-1209` (`flaeche_richten`), `crates/krk-core/src/text/datei.rs:404` (`versatz_nach_der_wandlung`)
**Cross-references:** `issues/260810-0215_c_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`, `issues/260810-0054_o_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md`, Commit `d5993f1`, C8

---

## Der Befund

Ein einziges eingefügtes `\r\n` löst in einer Datei nahe der 16-MB-Grenze diese
Kette aus:

```
  text_zurueckschreiben   text.string().to_string()     ~16 MB  UTF-16 -> UTF-8
  bearbeiten              in_gehaltene_form(stand)      ~16 MB  Kopie mit Wandlung
  flaeche_richten         text.string().to_string()     ~16 MB  UTF-16 -> UTF-8, zum zweiten Mal
  versatz_nach_der…       rest.to_owned()               bis 16 MB  Kopie des Restes
  versatz_nach_der…       in_gehaltene_form(rest)       bis 16 MB  zweite Kopie des Restes
  stand_einsetzen         NSString::from_str(stand)     ~16 MB  UTF-8 -> UTF-16
  stand_erneuern          darstellung_nachziehen()      ganze Datei neu einfärben
```

Der zweite Durchlauf durch `text.string()` ist im Code kommentiert und als Preis
angenommen. Die beiden Kopien in `versatz_nach_der_wandlung` sind es nicht:
`rest.to_owned()` legt den Rest des Textes auch dann an, wenn `in_gehaltene_form`
ihn danach unverändert zurückgibt — und das ist der Regelfall, denn hinter der
eingefügten Stelle steht meist nichts mehr zu wandeln.

## Warum das hier steht und nicht als Behebung

C8 sagt für das Einfügen keine Zeit zu, und der Weg läuft nicht je Tastendruck.
Der Befund ist deshalb Low. Er steht trotzdem, weil `260810-0054` schon eine
gemessene Verzögerung beim Tippen in großen Dateien führt und die Einfärbung
am Ende dieser Kette ein zweites Mal angestoßen wird.

## Was zu prüfen wäre

`in_gehaltene_form` nimmt ein `String` und gibt eines zurück; eine Fassung über
`Cow<str>` ließe den kurzen Weg ohne Kopie durchlaufen und käme weiterhin ohne
eine zweite Formulierung der Wandlungsregeln aus — das ist die Zusage, die der
Doc-Kommentar von `versatz_nach_der_wandlung` ausdrücklich hält und die eine
Zählung der weggefallenen Zeichen brechen würde.

Ob das die Signatur von `in_gehaltene_form` verändert, gehört in die Antwort:
sie wird als „die **eine** Stelle" geführt und hat drei weitere Aufrufer —
`datei::einlesen:333`, `Editormodell::bearbeiten` (`editormodell.rs:918`) und
den Ersatztext in `editormodell.rs:1154`. Der erste reicht sie als
Funktionswert an `Option::map` weiter und ist damit an ihre heutige Signatur
gebunden.

---
## Gemessen am 260810-1044, und die Behebung liegt in `krk-core`

**Der Befund ist bestaetigt, und die Zahl steht jetzt daneben.** Gezaehlt wurden
die Anlagen ab 1 MB (also die Kopien der ganzen Datei) mit einem zaehlenden
Allokator, an einem Text von 16,0 MB, in den vorn ein `\r\n` eingefuegt wurde,
Schreibmarke dahinter — der Regelfall, den dieser Datensatz nennt: hinter der
Schreibmarke steht fast die ganze Datei, und sie ist in gehaltener Form.

```
  bearbeiten: in_gehaltene_form(stand)                1 Kopie,  16,0 MB
  versatz_nach_der_wandlung, Fassung im Baum          1 Kopie,  16,0 MB
  dieselbe Rechnung ueber Cow<str>                    0 Kopien,  0,0 MB
  ─────────────────────────────────────────────────────────────────────
  Gegenfall (der Rest traegt selbst ein \r\n):
  Fassung im Baum                                     2 Kopien, 32,0 MB
  Fassung ueber Cow<str>                              2 Kopien, 32,0 MB
```

Beide Fassungen liefern denselben Versatz; das haelt die Messung fest, damit die
Zahl nicht von einer anderen Rechnung kommt.

**Der `Cow<str>`-Vorschlag traegt also genau im Regelfall und nur dort**, und das
ist die Haelfte, die dieser Datensatz bemaengelt: `rest.to_owned()` legt den Rest
auch dann an, wenn `in_gehaltene_form` ihn unveraendert zurueckgibt. Im
Gegenfall muss die Wandlung eine neue Zeichenkette bauen, und dann bleibt es bei
zwei Kopien — daran ist nichts zu holen.

Die ganze Kette, Kopien der vollen Laenge, im Regelfall:

| Schritt | Stelle | heute | mit dem Vorschlag |
|---|---|---|---|
| `text_zurueckschreiben`: `text.string().to_string()` | `editor.rs` | 1 | 1 |
| `bearbeiten`: `in_gehaltene_form` | `krk-core` | 1 (gemessen) | 1 |
| `flaeche_richten`: `text.string().to_string()` | `editor.rs` | 1 | 1 |
| `versatz_nach_der_wandlung` | `krk-core` | 1 (gemessen) | 0 (gemessen) |
| `stand_einsetzen`: `NSString::from_str` | `editor.rs` | 1 | 1 |
| **Summe** | | **5** | **4** |

## Was in `editor.rs` allein nicht zu holen ist

Die eine wegfallende Kopie steht in `krk-core/src/text/datei.rs` und nicht in
`editor.rs`; dieser Durchgang hatte `crates/krk-core/**` nicht in der Hand und
hat den Vorschlag deshalb nur gemessen, nicht gebaut.

**Der zweite Durchlauf durch `text.string()` in `flaeche_richten` ist in
`editor.rs` ebenfalls nicht abzustellen**, und der Grund gehoert dazu:
`text_zurueckschreiben` hat seine Abschrift an `Editormodell::bearbeiten`
abgegeben, das sie **verbraucht** (`stand: String`). Wer sie danach noch braucht,
klont sie vorher — dieselbe Kopie, nur an einer anderen Stelle. Zwei Wege gaebe
es, und beide liegen ausserhalb dieser Datei oder tragen nicht:

- **`bearbeiten` nimmt `&str`.** Dann fiele der zweite Durchlauf ganz weg. Das
  ist `krk-ui/src/editormodell.rs` und eine Aenderung an der Schnittstelle.
- **Nur den Rest hinter der Schreibmarke zurueckholen**, ueber
  `substringFromIndex:`, und ihn mit `versatz = 0` in
  `versatz_nach_der_wandlung` geben (dieselbe Rechnung: die Funktion sieht
  ohnehin allein `vorher[versatz..]`). Gerechnet lohnt das nur, wenn die
  Schreibmarke hinter der Mitte der Datei steht, und in genau diesem Fall steht
  sie vorn. Es waere ausserdem eine Kopie **mehr** und nur weniger Bytes, und es
  brachte die Frage nach einem halbierten Ersatzzeichenpaar mit.

Die Behebung gehoert also in einem Zug nach `krk-core`: `in_gehaltene_form`
ueber `Cow<str>` (oder ein `&str`-Helfer darunter, den beide Formen rufen),
womit `versatz_nach_der_wandlung` ohne die Eingangskopie auskommt. Die drei
weiteren Aufrufer, die dieser Datensatz nennt, sind dabei mitzuziehen; der erste
(`datei::einlesen:333`) reicht die Funktion als Wert an `Option::map` und ist an
ihre heutige Signatur gebunden.

---
Resolved: Der `&str`-Helfer darunter ist gebaut, nicht die geaenderte Signatur.
`text::datei::gehaltene_form(&str) -> Cow<'_, str>` traegt jetzt die Regeln der
Wandlung und gibt einen Text in gehaltener Form geliehen zurueck;
`in_gehaltene_form(String) -> String` bleibt unveraendert in der Signatur und ist
eine Fallunterscheidung darueber (`Cow::Borrowed` heisst "nichts zu wandeln",
also geht die uebernommene Zeichenkette zurueck). Damit bleiben die drei weiteren
Aufrufer unberuehrt, `datei::einlesen` voran, der die Funktion als Wert an
`Option::map` reicht, und `krk-ui` musste nicht angefasst werden.
`versatz_nach_der_wandlung` ruft jetzt `gehaltene_form(rest)` statt
`in_gehaltene_form(rest.to_owned())`.

**Nachgezaehlt mit einem eigenen zaehlenden Allokator**, Anlagen ab 1 MB, an
einem Text von 16.777.198 Bytes mit einem `\r\n` vorn und der Schreibmarke
dahinter. Die Probe steht dauerhaft in `crates/krk-core/tests/textkopien.rs` und
fuehrt die Fassung von vorher als `versatz_mit_kopie` mit, damit die Zahl ein
Vorher **und** ein Nachher hat; dass beide denselben Versatz liefern, prueft sie
mit.

```
  Regelfall (der Rest hinter der Schreibmarke ist in gehaltener Form):
    vorher    1 Anlage,  16.777.196 Bytes
    nachher   0 Anlagen,          0 Bytes

  Gegenfall (der Rest traegt selbst ein \r\n):
    vorher    2 Anlagen, 33.554.396 Bytes
    nachher   1 Anlage,  16.777.198 Bytes
```

**Der Gegenfall faellt besser aus, als die Messung vom 260810-1044 oben
vorhergesagt hat.** Sie fuehrt dort fuer beide Fassungen zwei Kopien und
schliesst mit "daran ist nichts zu holen"; gemessen an der gebauten Fassung sind
es eine gegen zwei, denn die Eingangskopie `rest.to_owned()` faellt in **beiden**
Faellen weg und nur die Anlage der Wandlung selbst bleibt. Vermutlich hat die
Vorwegmessung eine Fassung geprueft, die den Text weiter uebernahm; welche, ist
nicht mehr feststellbar. Die Zahl im Regelfall — eins gegen null — stimmt genau,
wie sie dort steht.

**Der Rest der Kette bleibt offen und liegt in `krk-ui`.** Die Summe der Kopien
voller Laenge geht damit von fuenf auf vier, wie die Tabelle oben es fuehrt. Die
drei verbleibenden Stellen in `krk-ui/src/appkit/editor.rs` —
`text_zurueckschreiben`, `flaeche_richten` und `NSString::from_str` — sind nicht
angefasst; der Abschnitt "Was in `editor.rs` allein nicht zu holen ist" oben sagt,
was dafuer noetig waere. **In `krk-core` steckt kein Rest mehr:** die vierte
Kopie ist die Wandlung in `bearbeiten` selbst, also die eine Anlage, die
`gehaltene_form` bauen **muss**, wenn ein `\r\n` zu wandeln ist. Ein eigener
Datensatz dafuer entsteht nicht, weil dort nichts zu holen ist.

Abnahme gruen: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets`, `cargo fmt -p krk-core -- --check`.
