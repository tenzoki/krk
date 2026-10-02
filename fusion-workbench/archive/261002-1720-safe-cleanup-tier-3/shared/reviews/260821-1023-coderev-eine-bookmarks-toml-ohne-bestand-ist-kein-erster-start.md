# Durchsicht: eine `bookmarks.toml` ohne Bestand ist kein erster Start (`073448e`)

**Sender:** coderev
**Datum:** 260821-1023
**Reviewed-range:** `01d2365..e688238`
**Not-opened:** none
**Grundlage:**
`shared/issues/260820-2235_c_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`
(Messtabelle, fünf Zeilen),
`shared/decisions/260821-0142_o_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`,
`shared/issues/260821-0142_o_eine-nicht-lesbare-ablagedatei-wird-nicht-gesichert-und-vom-naechsten-schreibvorgang-ueberschrieben.md`

## Zusammenfassung

Die Änderung trifft die Frage richtig, die der Ausgangsdefekt gestellt hat, und sie trifft sie
an der schmalsten Stelle: zwei Hälften, beide in den schon bestehenden Zweig
`Grund::Beschaedigt`, kein zweiter Mechanismus, keine stillschweigende Verallgemeinerung auf
die drei übrigen TOML-Dateien. Alle fünf Zeilen der Messtabelle sind nachgemessen und halten,
die vier Randfälle aus dem Auftrag ebenfalls. **Ein Befund steht dagegen und ist eine
Verschlechterung gegenüber `01d2365`:** der neue Zweig legt eine Datei beiseite, die den
Bestand gar nicht tragen kann, und belegt damit den einen Sicherungsplatz gegen die spätere
Sicherung, die ihn enthielte. Sieben Prosastellen der Ablage stehen daneben.

## Zählung

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 0 |
| Niedrig | 1 |

## Was geprüft ist und hält

**Die Messtabelle des Ausgangsdefekts, alle fünf Zeilen, am Baumstand `e688238` nachgefahren**
mit einem eigenen Programm gegen `krk_core::ablage::Lesezeichenliste` über `toml::from_str`,
also über denselben Weg wie `Zugang::laden`:

| Eingabe | Dokument leer | `Lesezeichenliste` |
|---|---|---|
| `[[eintraege]]` mit `name`/`ordner` | nein | `Ok(1)` |
| oberster Schlüssel heißt `[[lesezeichen]]` | nein | `Err` — ``unknown field `lesezeichen`, expected `eintraege` `` |
| Datei ist 0 Bytes | **ja** | `Ok(0)` — und der neue Zweig greift vor `from_str` |
| Eintrag mit unbekanntem Feld `farbe` | nein | `Ok(1)`, unverändert |
| Eintrag ohne `ordner` / dritte Zielsorte | nein | `Err` |

Die zweite Zeile ist damit umgedreht, die dritte gefangen, und die **vierte steht wie
gemessen**: die Zusage im Doc-Kommentar an `Lesezeichen`
(`crates/krk-core/src/ablage/lesezeichen.rs:71-78`), dass ein später hinzugefügtes Feld eine
ältere Datei nicht ungültig macht, ist nicht gebrochen.

**Die Begründung für den Sitz von `deny_unknown_fields` hält.** `Lesezeichen::ziel` trägt
`#[serde(flatten)]` (`lesezeichen.rs:89`), und `deny_unknown_fields` schließt sich damit an
derselben Struktur aus. Am Eintrag wäre der Vermerk also technisch nicht zu haben, wie der
Doc-Kommentar sagt. Die Wirkung ist gemessen und nicht abgeleitet: das unbekannte Feld im
Eintrag wandert durch `flatten` in den Zwischenspeicher, `Ziel` ist `untagged` und übergeht
es, das Ergebnis ist `Ok`.

**`Datei::leerbefund` ist vollständig und ohne Auffangzweig** (`pfade.rs:234-241`): eine
siebte `Datei`-Variante hält den Bau an. Die Antwort je Datei ist die, die der Bugfixer
behauptet.

**Für `session.toml`, `keymap.toml` und `settings.toml` ändert sich nachweislich nichts**, und
zwar aus einem Grund, den man an der Zeile ablesen kann: die Prüfung steht als
`welche.leerbefund() == Leerbefund::Beschaedigt && ohne_obersten_schluessel(&text)`
(`mod.rs:566`). Der linke Operand ist für die drei `Leerbefund::Vorgabe`, also fällt der
rechte weg — die drei laufen weder in den neuen Zweig noch in den zusätzlichen Parselauf.
Verhaltensgleich, und nicht bloß verhaltensähnlich. Die Probe
`eine_leere_datei_meldet_bei_den_drei_uebrigen_toml_dateien_nichts` schreibt es aus.

**`ohne_obersten_schluessel` hält gegen die vier Randfälle des Auftrags**, gemessen:

| Eingabe | Antwort | Folge für `bookmarks.toml` |
|---|---|---|
| nur Kommentare | ja | beschädigt |
| nur Leerzeilen | ja | beschädigt |
| BOM allein | ja | beschädigt |
| BOM + `eintraege = []` | nein | still angenommen |
| BOM + echter Bestand | nein | ein Eintrag, still angenommen |
| `eintraege = []` | nein | still angenommen |
| ungültiges TOML | nein | fällt in den Parse-Zweig darunter |

Die Bytefolgenmarke fällt damit nicht durch: die Kiste `toml` zieht sie ab, bevor sie zählt,
und die drei BOM-Zeilen verhalten sich wie ihre Gegenstücke ohne. Die ausdrücklich leere Liste
des Nutzers, `eintraege = []`, geht still durch — das ist die Gegenprobe, ohne die die neue
Regel gefährlicher wäre als der Defekt, und sie steht als eigene Probe im Baum.

**Kein zweiter Schreibweg.** `nur_benannte_dateien_erreichen_das_atomare_schreiben`
(`crates/krk-core/tests/baum.rs:178-206`) zählt unverändert fünf Dateien; die zwei geänderten
Quelldateien `lesezeichen.rs` und `pfade.rs` rufen `atomar::schreiben` nicht, die zwei anderen
standen schon auf der Liste. Nachgefahren: `cargo test -p krk-core --test baum`, 4 Proben grün.

**Abnahme unabhängig nachgefahren:** `cargo test -p krk-core --test ablage` → 70 grün, 5
ignoriert; `cargo clippy -p krk-core --all-targets` → keine Warnung; `cargo fmt --all --check`
→ sauber.

**Die Untergrenzen-Angabe ist nicht berührt**, wie der Auftrag sagt: sie gilt für
`krk-ui/src/appkit/`, und alle vier geänderten Dateien liegen in `krk-core`.

## Befunde

### Hoch — der neue Zweig belegt den einen Sicherungsplatz mit einer Datei ohne Bestand

**Datensatz:**
`shared/issues/260821-1023_o_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`

**Umfang:** `krk-core`, allein `bookmarks.toml` — und jede weitere Datei, die
`Leerbefund::Beschaedigt` bekäme, falls die offene Frage vom 260821-0142 mit Option 2 oder 3
beantwortet wird.

Die Eintrittsbedingung des neuen Zweigs ist „null oberste Schlüssel". Eine Datei, die sie
erfüllt, kann den Schlüssel `eintraege` nicht tragen und deshalb **nie** einen Bestand. Der
Zweig ruft trotzdem `beiseite_legen` (`mod.rs:567`), und weil es je Ablagedatei genau einen
Sicherungsplatz gibt, dessen erste Belegung unangetastet bleibt (`atomar.rs:68-86`), sperrt er
den Platz gegen jede spätere Sicherung.

Gemessen gegen die öffentliche Schnittstelle, Ausgabe gekürzt:

```
Start 1 (0 Bytes)        -> Gesichert(".../bookmarks.toml.beschaedigt"), laenge=0
Start 2 (echter Bestand) -> SchonVorhanden(".../bookmarks.toml.beschaedigt"), Inhalt=""
  Meldung: Die bisherige Fassung liegt seit einem frueheren Start unter ... und bleibt dort
  bookmarks.toml nach dem naechsten Schreibvorgang: "eintraege = []\n"
```

Der Bestand aus Start 2 ist fort und aus der Sicherung nicht zurückzuholen. **Vor `073448e`
wäre er gesichert worden**, weil eine leere `bookmarks.toml` damals keine `Ersetzung` erzeugte
und den Platz frei ließ. Der Zweig, der gegen den stillen Verlust gebaut ist, macht ihn in
dieser Reihenfolge unwiederbringlich.

Die Regel, die dabei bricht, ist nicht `SchonVorhanden`, sondern ihre Voraussetzung: der
Datensatz vom 260812-1105 begründet den einen Platz mit „die **erste** Fassung ist die
wertvollere". Für eine Fassung ohne obersten Schlüssel gilt der Satz nicht.

Daneben verspricht die Meldung das Gegenteil dessen, was dasteht: `Beiseite::Gesichert` über
eine 0-Byte-Datei erzeugt „Die bisherige Fassung liegt unter …", und der Doc-Kommentar an
`Beiseite` (`mod.rs:272-277`) sagt ausdrücklich zu, „dass keine Meldung eine Datei verspricht,
die es nicht gibt".

**Vorschlag:** der neue Zweig gibt `Beiseite::Nicht` zurück, statt `beiseite_legen` zu rufen.
Er hat nichts zu sichern, `Nicht` sagt genau das, und der Platz bleibt frei. Der Preis — der
Wortlaut einer Datei aus lauter Kommentaren bleibt nicht erhalten — steht im Datensatz mit der
Gegenrechnung. Eine Probe für die Reihenfolge „Leerbefund, dann echter Bestand" fehlt in jedem
Fall.

### Niedrig — sieben Prosastellen der Ablage

**Datensatz:**
`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`

**Umfang:** `krk-core/src/ablage/`, Doc-Kommentare.

Fünf Stellen sagen „vier Dateien" über `Ablage`, `Ablage::pfad` und `Zugang::pfad`, die alle
sechs annehmen (`mod.rs:45`, `:425`, `:427`, `:467`, `:508`); `Ablageort::datei` sagt es
richtig (`pfade.rs:354`). Diese fünf sind **älter als der Turn** — `git log -S` nennt
`3caa2b7` vom 260813, die zweite Zetteldatei kam mit der Runde 9 und hat sie stehen lassen.
Drei benachbarte Stellen sind richtig (`mod.rs:59`, `:513`, `:599`, alle über die vier
TOML-Dateien) und dürfen nicht mitgezogen werden.

Zwei Stellen gehören zum Turn: `mod.rs:142` sagt „Die drei übrigen tragen
`Leerbefund::Vorgabe`", es sind fünf — `pfade.rs:224` und die Auflösung des Ausgangsdefekts
sagen beide fünf, `mod.rs` ist die eine engere Zahl. Und der Modulkopf von `pfade.rs:1-2`
zählt auf, was das Modul beantwortet, ohne die neue dritte Frage zu nennen, obwohl er für die
zweite einen eigenen Abschnitt führt.

## Übergreifende Beobachtungen

**Der Auftrag hat die Prosa an der richtigen Stelle verdächtigt, und die Prosa hat gehalten.**
Jede Zahl, die der Modulkopf von `ablage/mod.rs` neu setzt, ist nachgezählt und stimmt: „drei
der vier TOML-Dateien tragen `deny_unknown_fields`" — `Belegungsdatei`
(`tasten/belegung.rs:1587`), `Einstellungsdatei` (`ablage/einstellungen.rs:125`),
`Lesezeichenliste` (`ablage/lesezeichen.rs:350`), und `Sitzung` (`ablage/sitzung.rs:319-321`)
trägt ihn nicht. `CLAUDE.md` nennt für die Ablage keine Zahl und ist deshalb nicht betroffen.
Die falschen Zahlen, die die Durchsicht gefunden hat, stehen an Stellen, die der Turn nicht
angefasst hat.

**Der Befund oben ist die dritte Gestalt desselben Verlusts**, und die beiden vorliegenden
Datensätze decken ihn nicht ab. `260821-0142` (nicht lesbare Datei) hängt an
`Grund::NichtLesbar` und `Beiseite::Nicht`; dieser hängt an `Grund::Beschaedigt` und
`Beiseite::SchonVorhanden`. Was alle drei teilen, ist der Schlusssatz: **der nächste
gewöhnliche Schreibvorgang schreibt, ohne zu fragen, ob der gelesene Wert aus der Datei kam
oder aus dem Auslieferungszustand.** `lesezeichen_aendern` unterscheidet allein nach dem
Ausgang der Änderung. Solange das so ist, entscheidet allein die Sicherung darüber, ob ein
Bestand zurückzuholen ist — und deshalb wiegt jede Regel, die den einen Sicherungsplatz
verbraucht, schwerer als ihre Zeilenzahl vermuten lässt.

**Die Meldung ist der schwächste Teil der ganzen Kette, und das ist gemessen.** Der Befund
oben erzeugt eine Startmeldung; `shared/issues/260820-2235_*_die-startmeldungen-ueberschreiben-einander-…`
misst, dass von n Startmeldungen die n-te ankommt. Eine Zusage der Form „der Nutzer erfährt
davon" trägt in diesem Baum derzeit nicht.

**Die Abnahmezeile „No regressions introduced" im Bugfix-Protokoll
(`shared/history/260821-0142-bugfix-bookmarks-toml-ohne-bestand.md:82-84`) nennt drei
Bestandsproben und ist so weit richtig.** Der Befund oben liegt außerhalb ihres Blicks, weil
er an einer **Reihenfolge** hängt und nicht an einem Wert: er tritt erst beim zweiten
Ladevorgang auf, und keine Probe im Baum lädt zweimal. Die sechs neuen Proben prüfen je einen
Ladevorgang gegen eine Datei; die Sicherung wird darin nur als Ergebnis dieses einen Vorgangs
gelesen.

**Die Wegskizze in der Untersuchung
(`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md:84-99`) ist mit
`073448e` überholt** — die Zweige „Datei leer (0 Byte)" und „oberster Schlüssel passt nicht"
enden dort auf „leer, KEINE Meldung" und tun das nicht mehr. **Sie ist trotzdem nicht zu
berichtigen:** `CLAUDE.md` legt unter „Bindende Grundlage" fest, dass Aufzeichnungen eines
Standes ihren damaligen Stand behalten, und die Ortsregel nennt `analyses/` ausdrücklich. Die
Skizze ist der gemessene Stand von `01d2365` und soll das bleiben.

**Ein Nebeneffekt ohne Befund:** für `bookmarks.toml` wird der Text jetzt zweimal geparst,
einmal als `toml::Table` in `ohne_obersten_schluessel` und einmal als `Lesezeichenliste`. An
`bookmarks.toml` hängt keine der zehn Zeitzusagen, und die Datei ist klein. Wer die zweite
Fahrt einmal loswerden will, parst einmal in eine `toml::Table`, fragt sie nach der Leere und
gibt sie über `try_into` weiter; das ist Aufräumen, kein Befund.

## Reihenfolge

1. **Vor der nächsten Auslieferung:** der Befund „Hoch". Er ist eine Verschlechterung
   gegenüber dem Stand, den die Auslieferung `v0.5.5` trägt, und der Vorschlag ist kürzer als
   der heutige Code.
2. **Vor der Antwort auf `260821-0142` (strenge Lesart für `session.toml`):** derselbe Befund.
   Wird `Datei::Sitzung` auf `Leerbefund::Beschaedigt` gesetzt, gilt er dort mit, und
   `session.toml` wird im Takt geschrieben.
3. **Beim nächsten Aufräumen:** die sieben Prosastellen.
