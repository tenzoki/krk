# coder — Sechs offene Defekte im Umfeld von Vorschau und Oberfläche

**Status:** Complete
**Zeitpunkt:** 260806-1240
**Ausführender:** `coder`
**Auftrag:** Die sechs offenen Defekte aus `issues/` beheben, jeden für sich, minimal
und gezielt. Nicht committen, nicht in den Defektdateien vermerken.

---

## Ergebnis je Defekt

| # | Defekt | Ausgang |
|---|--------|---------|
| 1 | Vorschau liest Bilddateien ohne Größengrenze (`260806-0834`) | behoben |
| 2 | Vorschau lädt bei ausgeblendetem Fenster (`260806-0834`) | behoben |
| 3 | Größenformatierer schreibt "Zero KB" (`260805-1130`) | **offen**, Auflösung liegt nicht in Code-Dateien; neue Meldung `260806-1215` |
| 4 | Dateiliste leer während eines Stapel-Umbenennens (`260805-1337`) | behoben |
| 5 | Auswahlwiederherstellung umgeht die Wählbarkeitsprüfung (`260806-1123`) | behoben |
| 6 | macOS stellt ein AutoFill-Untermenü dazu (`260805-1455`) | behoben |

---

## 1. Größengrenze für Bilddateien

`crates/krk-ui/src/vorschaumodell.rs` bekommt neben `TEXTGRENZE` eine zweite Zahl,
`BILDGRENZE = 64 MB`, und `laden` prüft sie **vor** `std::fs::read`. Darüber fällt die
Anzeige auf die Metadaten zurück, denselben Weg, den eine Textdatei über 1 MB schon
ging und den `Inhalt::Bild` mit seinem mitgeführten `metadaten`-Feld ohnehin trug. Ein
neuer Mechanismus entsteht nicht; es ist dieselbe Regel mit einer zweiten Zahl. Eine
`const`-Zusicherung neben den beiden Zahlen hält beim Übersetzen fest, dass die
Bildgrenze über der Textgrenze liegt.

Dazu hält `Inhalt::Bild` seine Bytes jetzt in einem `Arc<Vec<u8>>`. Die Ansicht klont
den Inhalt des aktiven Tabs bei jedem Neuzeichnen, um die Ausleihe des Modells vor dem
ersten Objective-C-Aufruf zu beenden; mit einem blanken `Vec<u8>` war das jedes Mal
eine zweite vollständige Kopie der Bilddatei. `Arc` und nicht `Rc`, weil der
Arbeitsfaden den Wert baut und durch einen Kanal schickt.

**Die Wahl der Zahl betrifft den Nutzer und ist hier festgehalten.** 64 MB trennt die
gängigen Bildformate, die C6 mit ihrem Inhalt zusagt, von den Ausreißern: ein
Bildschirmfoto eines Retina-Schirms liegt bei wenigen MB, ein Kamera-JPEG unter 20 MB,
ein HEIC darunter, während ein TIFF- oder PSD-Export leicht über 100 MB liegt. Die
Textgrenze von 1 MB wäre zu eng gewesen und hätte jedes gewöhnliche Foto aus der
Anzeige genommen; C6 verlangt das Gegenteil.

**Was nicht behoben ist.** Der Defekt nennt als dritten Punkt den Lesefaden je
Auswahländerung: ein überholter Faden liest seine Datei zu Ende, nur sein `send`
scheitert still. Mit der Grenze ist jeder einzelne Lesevorgang gedeckelt, die Zahl der
Fäden ist es nicht. Ein Abbruchkennzeichen wäre ein zweiter Mechanismus neben dem
fallenden Empfänger, den der Modulkopf ausdrücklich als ausreichend beschreibt; das
gehört in eine eigene Entscheidung und nicht in diese Reparatur.

## 2. Kein Laden bei ausgeblendeter Vorschau

`AnwendungsIvars` bekommt ein Feld `vorschau_nachtrag: RefCell<Option<PathBuf>>`.
`vorschau_fuellen` fragt jetzt nach `modell.sichtbar(Bereich::Vorschau)`: bei
ausgeblendeter Vorschau wird der Pfad nur vermerkt und nicht gelesen. `bereich_umschalten`
holt ihn nach, sobald `Bereich::Vorschau` wieder sichtbar ist — mit demselben
`datei_anzeigen`, ein zweiter Weg in die Vorschau entsteht nicht. `zwischenablage_ansehen`
löscht den Vermerk, weil die Zwischenablage die neuere Quelle für denselben Tab ist und
sonst ein überholter Pfad nach dem nächsten Ein- und Ausblenden nachträte.

## 3. "Zero KB" — nicht behoben, und der Grund ist gemessen

Der Defekt bleibt offen. Zwei Befunde stehen in der neuen Meldung
`issues/260806-1215_o_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`:

1. Der Defekt ist weiter gefasst als seine Beschreibung: **jede** Angabe unter 1.000
   Bytes kommt englisch, nicht nur die Null. Gemessen mit demselben
   `NSByteCountFormatter` und `CountStyle::File`, einmal ohne Bündel und einmal in einem
   Bündel mit `CFBundleLocalizations = de, en`: `Zero KB` / `1 byte` / `512 bytes`
   gegen `0 KB` / `1 Byte` / `512 Byte`.
2. Weg 2 des Defekts, `setAllowsNonnumericFormatting(false)`, löst die Sache nicht: er
   liefert `0 bytes`, also wieder Englisch. Weg 1, der Schlüssel in
   `resources/Info.plist`, löst sie vollständig. Ein Weg über Code besteht nicht:
   `AppleLanguages` über `registerDefaults:` ändert `preferredLocalizations` nicht,
   weil Foundation gegen die Sprachen des **Bündels** schneidet.

`resources/Info.plist` ist eine Bündelbeschreibung und gehört dem `ontocoder`; deshalb
eine Meldung und keine Reparatur.

## 4. Keine Auffrischung im Ordner des eigenen laufenden Vorgangs

`Vorgang` bekommt eine Methode `ordner()`, die die Ordner einer Operation aufzählt:
Quelle, dazu das Ziel beim Kopieren und Verschieben. Sie ist **die eine Stelle**, die
diese Frage beantwortet, und beide Aufrufer gehen über sie: der Abschluss in
`vorgang_beenden` frischt genau diese Ordner auf, und der Rückruf der Dateisystemwache
überspringt genau sie, solange der Vorgang läuft.

Der Vergleich steht als `auffrischung::gehoert_zu_vorgang` in der Datei, die die
Auffrischungsentscheidung ohnehin hält und ohne Fenster prüfbar ist; er benutzt
denselben `gleicher_ordner`, der auch `ordner_neu_lesen` trägt.

**Der Aufschub gilt allein für die Ordner des Vorgangs.** Eine fremde Änderung
anderswo frischt weiter ohne Zutun auf, wie C9 es zusagt. Eine fremde Änderung in
diesen Ordnern geht nicht verloren, sie erscheint mit der Auffrischung des Abschlusses.
Statt einer leeren Liste sieht der Nutzer während des Vorgangs den Stand von vorher.

Von den beiden Richtungen des Defekts ist das die erste. Die zweite, das Entprellen,
hätte einen Zeitgeber und damit einen zweiten Mechanismus gebraucht und hilft gegen
einen Meldungsstrom über mehrere Sekunden ohnehin nicht: die Liste bliebe die ganze Zeit
auf dem alten Stand und das Entprellen liefe nie ab.

## 5. Wählbarkeitsprüfung bei der Auswahlwiederherstellung

`Belegungsmodell::waehlbare_zeile` liefert die Stelle selbst, falls sie in der Liste
liegt und eine Funktion trägt, sonst die erste Funktionszeile. `nachziehen` in
`appkit/belegungsansicht.rs` geht durch sie hindurch. Reines Rust und deshalb im
Modell und nicht in der Ansicht: die Entscheidung ist ohne Fenster prüfbar, und eine
neue Prüfung hält sie fest. Der tragende Invariant steht jetzt an der Aufrufstelle
ausgeschrieben.

## 6. AutoFill-Untermenü

`systemzusaetze_unterdruecken` registriert einen dritten Schlüssel,
`NSAutoFillSystemInsertMenuEnabled`, und zwar auf **falsch** statt wie die beiden
anderen auf wahr. Der im Defekt vermutete Name `NSDisabledAutoFillMenuItem` wirkt
nicht; am gebauten Bündel gemessen, das Menü trug den Eintrag unverändert. Der
wirksame Name ist in den Zeichenketten des dyld-Zwischenspeichers gesucht und dann am
Bündel bestätigt worden.

---

## Geänderte Dateien

- `crates/krk-ui/src/vorschaumodell.rs` — Bildgrenze, `Arc` für die Bilddaten, drei neue Prüfungen
- `crates/krk-ui/src/appkit/vorschau.rs` — Begründung am Klon des Inhalts
- `crates/krk-ui/src/appkit/anwendung.rs` — `vorschau_nachtrag`, `vorschau_nachtragen`, `Vorgang::ordner`, `vorgangsordner`, Filter im Wachen-Rückruf
- `crates/krk-ui/src/auffrischung.rs` — `gehoert_zu_vorgang` und zwei neue Prüfungen
- `crates/krk-ui/src/belegungsmodell.rs` — `waehlbare_zeile` und eine neue Prüfung
- `crates/krk-ui/src/appkit/belegungsansicht.rs` — `nachziehen` geht durch `waehlbare_zeile`
- `crates/krk-ui/src/appkit/menue.rs` — dritter Vorgabeschlüssel

Neu im Issue-Speicher: `260806-1215` (Umfang des Größenformatierers, für `ontocoder`)
und `260806-1235` (der Sitzungslauf der Abnahmestrecke bricht bei L5-Tab ab).

## Nachweis am gebauten Bündel

Alle Zahlen auf MacBookPro15,1, macOS 15.7.7, mit `make bundle` gebaut und signiert.
"vorher" heißt: dieselbe Messung mit `git stash` auf `crates/`, also unverändertem
`main`.

**Defekt 1** — zwanzig synthetische Pfeil-ab-Ereignisse über KRKs eigene
Ereignisschlange (`--messmodus spannen`) über einem Ordner mit 40 Bilddateien von je
65 MB, höchster belegter Speicher des Prozesses:

| Startordner | vorher | nachher |
|---|---|---|
| 40 Textdateien | 53 MB | 53 MB |
| 40 Bilddateien je 65 MB | **438 MB** | **54 MB** |

Dazu am laufenden Fenster: eine Sitzung mit Auswahl auf `bild-07.tiff` (68,2 MB) zeigt
im Vorschaufenster die sechs Metadatenzeilen aus C6 statt eines Bildes, und eine
Auswahl auf einer gültigen PNG-Datei unter der Grenze zeigt weiterhin das Bild. C6
bleibt damit gehalten.

**Defekt 2** — eine Sitzung mit Auswahl auf einer 60-MB-Bilddatei, also unter der
Grenze, einmal mit eingeblendeter und einmal mit ausgeblendeter Vorschau:

| Vorschaufenster | vorher | nachher |
|---|---|---|
| eingeblendet | 234 MB | 173 MB |
| ausgeblendet | 231 MB | **49 MB** |

Ausgeblendet liest KRK die Datei nicht mehr; eingeblendet liest es sie weiter, und die
Differenz von 61 MB gegenüber vorher ist genau die Kopie, die Defekt 1 entfernt hat.

**Defekt 6** — `make menue` am signierten Bündel. Vorher trug das Menü "Bearbeiten"
einen Trenner und darunter `eintrag="AutoFill" … selektor=submenuAction:`. Nachher
stehen dort genau die vier eigenen Einträge mit unveränderten Kürzeln, kein Trenner
und kein AutoFill.

**Zeitzusagen aus C8** — die Durchstichstrecke, drei Runden, mit und ohne die
Änderungen:

| Größe | vorher | nachher | Abnahme |
|---|---|---|---|
| L1 (Anteil im Bild) | 85,0 %, in 0 von 3 Runden gehalten | 85,0 %, in 1 von 3 Runden gehalten | ≥ 95 % |
| L2 | 47,5 ms | 47,8 ms | p95 ≤ 100 ms, gehalten |
| L3 | 149,1 ms | 148,9 ms | p95 ≤ 400 ms, gehalten |
| L4 | 316,6 ms | 292,2 ms | p95 ≤ 1000 ms, gehalten |
| L10 | 57,1 ms | 56,4 ms | p95 ≤ 100 ms, gehalten |

L1 verfehlt den Anteil auf diesem Gerät heute **mit und ohne** die Änderungen; die
Zahlen liegen mit ihnen eher besser. Der Lauf fand unter der Last dieser Sitzung
statt. Die sechs Größen der Sitzungsstrecke (L1, L5, L6, L7, L8, L9) sind nicht
gemessen, weil der Sitzungslauf auf `main` bei L5-Tab abbricht; siehe die neue Meldung
`260806-1235`.

**Defekt 4 und 5** sind am Bündel nicht vorgeführt. Defekt 5 ist heute nicht
auslösbar, dafür gibt es nichts zu zeigen. Defekt 4 bräuchte ein Stapel-Umbenennen über
5.000 Einträge und damit einen Tastendruck, den die feste Schrittliste des Messmodus
nicht kennt; belegt ist die Entscheidung durch zwei neue Prüfungen in
`auffrischung.rs`.

## Abnahme

`make check` grün: `cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`.
Sechs neue Prüfungen. Die Modulgrenze ist unverändert: `vorschaumodell.rs`,
`belegungsmodell.rs` und `auffrischung.rs` nennen keine `objc2`-Kiste, und
`#![deny(unsafe_code)]` steht weiter mit den zwei bekannten Ausnahmen.

**Nicht committet.** Der Baum trägt die Änderungen an sieben Dateien unter `crates/`.
