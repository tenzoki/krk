# Die zweite Hälfte von S38 und S39: die Textmarke wird angelegt und angesprungen

---
**Agent:** coder
**Status:** Complete
**Anlass:** S38 (eine Textmarke anlegen), zweite Hälfte, und S39 (der Sprung auf eine Textmarke) aus `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`
**Umfang:** `crates/krk-ui/src/leistenmodell.rs`, `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/appkit/anwendung.rs`
**Ergebnis:** `make check` grün — Bau, Proben, `fmt --all --check` und `clippy --all-targets -D warnings`
**Geschlossen:** `issues/260809-1631_c_ein-markensprung-kann-zwei-meldungen-zugleich-haben-und-die-zeile-traegt-eine.md`

---

## Was gebaut wurde

### Der Editor-Zweig von `cmd+d` (die zweite Hälfte von S38)

`lesezeichen_anlegen` fragt seit diesem Schritt `anlegeziel`, und das ist die
**eine Stelle, an der der Fokus die Sorte wählt**. Sie liefert das fertige
`krk_core::ablage::Ziel` und den Namensvorschlag als ein Paar; alles danach ist
für beide Sorten dasselbe, nämlich dasselbe Eingabeblatt und dieselbe
Ausführung. Die Kette dahinter bis `bookmarks.toml` fragt seit dem 260810-0036
ohnehin nicht mehr nach der Sorte.

Die Zeile der Schreibmarke ist nicht neu gerechnet worden:
`Editorbereich::schreibmarkenzeile` liefert Nummer und Inhalt, und die Umrechnung
zwischen AppKits UTF-16-Einheiten und Byteversätzen steht in
`appkit/koordinaten.rs`. Das `#[allow(dead_code)]` an dieser Funktion ist mit
ihrem Aufrufer gefallen.

**Der Fokus im Editor ohne gehaltene Datei meldet und legt nichts an.** Auf den
Ordner des aktiven Dateifensters auszuweichen wäre der stillschweigend andere
Befehl gewesen: der Nutzer bekäme eine Ordnermarke, wo er eine Textmarke
verlangt hat. Gemeldet wird derselbe Satz, den `editorblatt_moeglich` seit S25
für diesen Fall führt.

Der Namensvorschlag ist `dateiname:zeile` — der Vorschlag, den
`issues/260810-0036` offen gelassen hatte.

### Die Auswahl trägt die Sorte (S39, die Leistenseite)

`Leistenmodell::gewaehlt` lieferte für eine Textmarke `None`, weil `Auswahl`
einen Ordner trug und eine Textmarke keinen hat. Der Platzhalter aus `65c8efa`
ist gefallen: `Auswahl` trägt jetzt das `Ziel` selbst.

**Ein eigener Sortenwert in `krk-ui` ist bewusst nicht entstanden.** Er wäre die
zweite Wahrheit darüber, was eine Zeile öffnet, und liefe von dem weg, was in
`bookmarks.toml` steht; `Leistenmodell::sinnbild` liest seit S40 dieselbe
Eigenschaft für die Anzeige. Ein Gerät und ein Standardort tragen `Ziel::Ordner`,
denn genau das öffnen sie — dieselbe Regel, die S40 dem Sinnbild gegeben hat.

Dazu `Auswahl::pfad`: die eine Stelle, die beide Sorten auf einen Pfad bringt.
Der Satz über ein fehlendes Ziel lautet für beide gleich, denn was fehlt, ist ein
Eintrag im Dateisystem, und welcher Art er wäre, sagt der Satz nicht.

An `appkit/leiste.rs` war **nichts** zu ändern, obwohl der Umfang sie nennt: sie
reicht die `Auswahl` durch und liest allein ihren Namen.

### Der Sprung (S39, die Editorseite)

`Editorbereich::marke_anspringen` fragt `krk_core::text::marke::wiederfinden` und
setzt die Schreibmarke über dieselbe `stelle_zeigen`, die der Zeilensprung aus
C5 benutzt — ohne Ausdehnung, denn eine Marke bezeichnet eine Stelle und keinen
Bereich. Gesucht wird im gehaltenen Stand und nicht in der Datei auf der Platte,
dieselbe Regel, die das neunte Abnahmekriterium von C5 der Suche gibt.

`leistenauswahl_ausfuehren` verzweigt nach der Sorte, und `textmarke_anspringen`
öffnet die Datei über `Editorbereich::datei_oeffnen` — **derselbe eine Weg wie
F4 und wie der Übergang aus der Vorschau**, und damit dieselbe Größen- und
Typprüfung aus `krk_core::text::datei::oeffnen`. Alles, was auf das Öffnen
folgt, erbt dieser Weg von `editorausgang_behandeln`, ohne eine Zeile dafür:
das Hervorholen des ausgeblendeten Editors, den Fokus, den Titel, die
Abweisungsmeldung und die Nachfrage aus C4 beim Wechsel auf eine andere Datei.

## Die drei Entscheidungen, die dabei zu treffen waren

### Der Sprung geschieht beim Ladeausgang und nicht beim Befehl

`datei_oeffnen` kehrt seit S24 zurück, ohne gelesen zu haben, und wohin die
Schreibmarke gehört, entscheidet sich am Text. Die gemerkte Stelle wartet
deshalb in `AnwendungsIvars::vorgemerkte_marke`, und
`editorausgang_behandeln` nimmt sie beim ersten Ausgang heraus — derselbe
Zuschnitt wie `editor_aus_sitzung` daneben: ein Feld, ein Schreiber je Anlass,
ein Leser, und der Leser verbraucht es.

Sie überlebt genau **einen** Umweg, und das ist die Rückhaltung aus C4:

```text
  Auswahl ──> vormerken ──> datei_oeffnen
                                 │
                                 ├── Geoeffnet / SchonOffen ──> herausnehmen, springen
                                 ├── Abgewiesen ─────────────> herausgenommen, fällt
                                 └── Zurueckgehalten ────────> zurücklegen, Nachfrage
                                                                    │
                                            sichern / verwerfen ────┤──> Geoeffnet (oben)
                                            abbrechen ──────────────┘──> mit der Datei fallen
```

Das Herausnehmen an einer Stelle statt drei Löschstellen ist die tragende Wahl:
eine abgewiesene Datei braucht dafür keine eigene Zeile. Die zwei Zeilen, die
es gibt, stehen beide dort, wo auch die zurückgehaltene Datei behandelt wird —
sie zurückzulegen im Zweig `Zurueckgehalten`, sie fallen zu lassen in
`anlass_unterbleibt`. Bliebe sie dort stehen, spränge das nächste F4 auf eine
Stelle, die niemand verlangt hat.

Gesprungen wird als **Letztes** im Zweig, nach `fokus_holen`:
`scrollRangeToVisible:` holt einen Bereich ins Bild, und der Editor muss dafür
auf dem Schirm stehen.

### Zwei Auskünfte, ein Satz — der offene Punkt aus S21

`issues/260809-1631` hielt fest, dass `Markensprung` zwei verschiedene Auskünfte
trägt, dass Rang 1 der Statuszeile eine Meldung hält, und dass S39 der erste und
einzige Aufrufer ist, bei dem beide an einem Wert hängen. Der Befund schlug einen
Satz für den zusammengesetzten Fall vor, statt einer zweiten Vorrangregel.

**Der Vorschlag hält, und seine Begründung ist beim Bauen stärker geworden: die
beiden Auskünfte sind nicht unabhängig.** `wiederfinden` liefert
`Fund::Getroffen` und `Fund::Verschoben` allein für eine Nummer, deren Zeile es
im heutigen Text gibt — `Zeilenindex::inhalt_der_zeile` beantwortet jede andere
mit `None`, und der gelieferte Sprung entsteht aus derselben Nummer. Eine von
`Zeilenlage::Getroffen` verschiedene Lage kommt deshalb **nur** mit
`Fund::NichtGefunden` vor; von neun Paarungen sind fünf erreichbar, und die
zweite Auskunft kann nie für sich stehen.

Eine Vorrangregel wäre damit nicht nur unnötig, sondern falsch: sie täte so, als
könnten beide Meldungen einzeln auftreten. Ein dritter, zusammengesetzter Wert
wäre der zweite Weg zu demselben Sachverhalt. Gebaut ist deshalb **ein** Wert mit
zwei Feldern: `Editormeldung::MarkenstelleGeaendert` trägt neben der
Zeilennummer die `Zeilenlage`. Der Fund entscheidet, **ob** gemeldet wird, die
Lage, **wohin** die Schreibmarke gekommen ist.

| Lage | Satz |
|---|---|
| `Getroffen` | „die gemerkte Stelle hat sich geändert; die Marke führt auf Zeile 118“ |
| `HinterDerLetzten` | „die gemerkte Stelle hat sich geändert; die Datei hat keine Zeile 500 mehr; die Schreibmarke steht am Dateiende“ |
| `VorDerErsten` | „die gemerkte Stelle hat sich geändert; Zeilen zählen ab 1; die Schreibmarke steht am Dateianfang“ |

Die dritte Zeile ist keine Vorsorge: eine gemerkte Nummer 0 kommt aus keinem
Anlegen, wohl aber aus einer von Hand geänderten `bookmarks.toml`, wie
`krk-core` es in `eine_gemerkte_nummer_null_fuehrt_an_den_textanfang_und_sucht_trotzdem`
schon festhält.

Die Meldung der Zeilenlage beim Zeilensprung aus C5 steht unverändert daneben
und teilt sich mit dieser keinen Rang: sie ist die Antwort auf `cmd+j` und kommt
nie im selben Tastendruck wie ein Markensprung.

### Die Sorte wohnt im `Ziel` und nicht daneben

Siehe oben unter „Die Auswahl trägt die Sorte". Der Preis ist, dass `Auswahl`
für ein Gerät ein `Ziel::Ordner` baut, das nie in einer Datei stand; der Gewinn
ist eine Aufzählung statt zweier und eine vollständige Fallunterscheidung beim
Aufrufer, die eine dritte Sorte anhielte.

## Beide Platzhalter aus `65c8efa` sind abgelöst

| Platzhalter | Ablöser | Stand |
|---|---|---|
| `Leistenmodell::anlegen` verpackt den Pfad fest in `Ziel::Ordner` | S38 | gefallen am 260810-0036 |
| `Leistenmodell::gewaehlt` liefert für eine Textmarke `None` | S39 | gefallen mit diesem Schritt |

Dazu gefallen: die beiden `#[allow(dead_code)]` an
`Editormeldung::MarkenstelleGeaendert` und `Editormeldung::markenstelle`, deren
Vermerk S39 als Ablösepunkt nannte, und das an
`Editorbereich::schreibmarkenzeile`, das S38 nannte. **In `appkit/editor.rs` und
in `leistenmodell.rs` steht danach kein `#[allow(dead_code)]` mehr**, und der
Kopf von `Editormeldung` sagt es: jeder Wert der Aufzählung hat seinen Auslöser.

## Die Proben

Sechs neue, alle grün im ersten Lauf.

| Datei | Probe | Was sie hält |
|---|---|---|
| `leistenmodell.rs` | `eine_textmarke_liefert_ihre_stelle_und_nicht_nichts` | die Ablösung des Platzhalters: die Auswahl trägt Datei, Nummer und gemerkten Inhalt |
| `leistenmodell.rs` | `eine_textmarke_auf_eine_fehlende_datei_liefert_eine_ungueltige_auswahl` | der erste der drei Ausgänge: fehlende Datei, ungültige Auswahl, Pfad für den Satz |
| `appkit/editor.rs` | `allein_die_nicht_wiedergefundene_markenstelle_meldet_sich` (erweitert) | getroffen und verschoben melden nichts; die neue Lage steht im Wert |
| `appkit/editor.rs` | `eine_marke_auf_eine_gekuerzte_datei_meldet_beide_auskuenfte_in_einem_satz` | genau das Beispiel aus `marke.rs:96-98`, beide Auskünfte in einem Satz |
| `appkit/editor.rs` | `eine_gemerkte_nummer_null_meldet_den_dateianfang` | die dritte Lage bekommt ihren eigenen Satz |
| `appkit/editor.rs` | `die_drei_lagen_des_markensprungs_tragen_drei_verschiedene_saetze` | keine Lage bekommt still den Satz einer anderen |

**Der zweite der drei Ausgänge aus dem Abnahmekriterium ist in `krk-ui` nicht
prüfbar, und das ist eine Einschränkung.** Die abgewiesene Datei hängt an einem
laufenden Fenster mit Editorbereich; eine Probe dafür müsste eines bauen. Die
Prüfung dahinter deckt `krk-core` seit S10 ab, und dass die Marke dabei gültig
bleibt, folgt aus `Lesezeichen::gueltig`, das allein `is_file` fragt und den
Sprung nicht kennt. Der Ausgang steht deshalb unten unter der Nutzerarbeit.

## Was offen bleibt

Nichts an S38 und S39. Zwei Beobachtungen, die daneben liegen:

- **Ein gelungenes Öffnen im Editor merkt die Sitzung nicht vor.** Weder F4 noch
  der Übergang aus der Vorschau noch der Markensprung rufen
  `sitzung_vormerken`; `kommando_ausfuehren` tut es nicht, und
  `editorausgang_behandeln` auch nicht. Der Sprung erbt das Verhalten der beiden
  bestehenden Wege, statt eine dritte Regel danebenzustellen. Ob das richtig ist,
  gehört zu S30 und S31 und nicht hierher; festgehalten, damit es nicht als
  Nebenwirkung dieses Schrittes gelesen wird.
- **Der Defekt `issues/260810-0215`** — Stand und Text der Fläche laufen nach
  einem eingefügten `\r\n` auseinander — trifft `schreibmarkenzeile` und damit
  auch das Anlegen einer Marke unmittelbar nach einem solchen Einfügen. Er steht
  unverändert und ist nicht Gegenstand dieses Schrittes.

## Was der Nutzer selbst prüfen muss

Am laufenden Bündel, mit KRK im Vordergrund.

1. **Anlegen.** F4 auf eine Textdatei, Schreibmarke in eine Zeile, `cmd+d`. Das
   Blatt fragt nach einem Namen und schlägt `dateiname:zeile` vor. Die Marke
   steht danach unten in der Leiste, mit dem Dokument-Sinnbild.
2. **Eine Zeile und kein Bereich.** Mehrere Zeilen auswählen, dann `cmd+d`: die
   Marke liegt auf der Zeile am **Anfang** der Auswahl.
3. **Beide Sorten am selben Befehl.** Fokus in ein Dateifenster, `cmd+d` legt
   weiterhin eine Ordnermarke auf den angezeigten Ordner an.
4. **Der Sprung bei ausgeblendetem Editor.** Editor mit `opt+cmd+e` schließen,
   dann die Textmarke in der Leiste auswählen: der Editor kommt hervor, hat den
   Fokus, und die Schreibmarke steht auf der gemerkten Zeile.
5. **Eine verschobene Stelle.** Die Datei von außen um zehn Zeilen nach unten
   schieben, etwa mit einem eingefügten Kopf, dann die Marke wählen: sie trifft
   die verschobene Zeile, und die Statuszeile meldet nichts.
6. **Eine zu weit verschobene Stelle.** Dasselbe mit hundert Zeilen: die Marke
   landet auf der gemerkten Nummer, und die Statuszeile sagt „die gemerkte
   Stelle hat sich geändert".
7. **Eine gekürzte Datei.** Die Datei auf weniger Zeilen kürzen, als die Marke
   nennt: ein Satz nennt beides, die geänderte Stelle und das Dateiende.
8. **Die fehlende Datei.** Die Datei löschen: die Marke trägt „(fehlt)" in der
   Leiste, und ihre Auswahl meldet den Grund, ohne den Editor anzufassen.
9. **Die abgewiesene Datei.** Die Datei durch etwas Nichttextliches oder etwas
   über 16 MB ersetzen: die Auswahl meldet den Grund der Abweisung, springt
   nicht, und die Marke bleibt **ohne** „(fehlt)".
10. **Der ungesicherte Stand.** Im Editor etwas tippen, ohne zu sichern, dann
    eine Textmarke auf eine **andere** Datei wählen: die Nachfrage aus C4 kommt.
    Mit „sichern" oder „verwerfen" öffnet die Datei und der Sprung geschieht;
    mit „abbrechen" bleibt alles stehen, und ein anschließendes F4 auf eine
    dritte Datei springt **nicht** auf die gemerkte Stelle.
