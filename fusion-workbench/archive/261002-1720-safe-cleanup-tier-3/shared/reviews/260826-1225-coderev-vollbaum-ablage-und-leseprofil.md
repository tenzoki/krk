# Vollbaum-Durchsicht: `crates/krk-core/src/ablage/` und `crates/krk-core/src/leseprofil/`

**Reviewed-range:** `004ff72..004ff72`
**Not-opened:** none

Kein Commit-Bereich: Vollbaum-Durchsicht von `crates/krk-core/src/{ablage,leseprofil}/` am
Baumstand `004ff72`. Die beiden Pflichtfelder darüber stehen in der Form, die
`bin/fusion-review-coverage` liest; die drei Schwesterdurchsichten derselben Sitzung tragen
dieselbe.

**Sender:** coderev
**Gelesen:** 12 von 12 Dateien, 5.875 Zeilen
**Baumstand:** `004ff72`
**Nachgemessen:** `cargo clippy -p krk-core --all-targets` läuft ohne Warnung durch, mit
eigenem `CARGO_TARGET_DIR` außerhalb des Projektbaums, damit drei parallel laufende Prüfer
nicht behindert werden.

## Summary

Beide Modulgruppen sind in ungewöhnlich gutem Zustand: die Fehlerpfade sind vollständig
ausgeführt, die Fallunterscheidungen tragen fast durchweg keinen Auffangzweig, und die
Abnahmeproben decken auch die schwer erreichbaren Lagen ab — ein Prozess, der zwischen
Schreiben und Umbenennen stirbt, zwei Prozesse am Sitzungsrecht, eine Zusammenfassung mit einem
freien Deskriptor. Kein Befund dieser Durchsicht betrifft ein Fehlverhalten zur Laufzeit, und
keiner ist ein Auslieferungshindernis. Fünf Defekte sind gefiltert, davon einer mittlerer
Schwere; dazu ein Entscheidungsdatensatz für eine Frage, die der Baum offen lässt und die keine
Behebung beantworten kann. Die zwei Defekte, die die Aufgabe zur Prüfung vorgelegt hat, halten
beide unverändert; an einem ist eine seiner zwei genannten Hürden inzwischen gefallen.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 4 |
| Entscheidung statt Defekt | 1 |

## Was die Aufgabe zur Prüfung vorgelegt hat

### `Datei::ALLE` führt jede geschriebene Ablagedatei, und kein Schreiber schreibt daran vorbei

Nachgemessen. `Datei::ALLE` (`crates/krk-core/src/ablage/pfade.rs:226-234`) führt sieben Werte;
`readers.toml` steht als `Datei::Leser` darin (`:210`), die Zählung stimmt. Jeder Schreibweg in
den Ablageordner geht durch `atomar::schreiben` und bildet seinen Pfad über `Ablageort::datei`,
das ausschließlich `Datei` entgegennimmt (`pfade.rs:451-453`). Welche Dateien des ganzen Baums
`atomar::schreiben` überhaupt erreichen können, zählt die Probe
`nur_benannte_dateien_erreichen_das_atomare_schreiben` (`crates/krk-core/tests/baum.rs:178-208`)
namentlich ab; sie führt sechs Dateien, davon drei in der Ablage.

Zwei abgeleitete Namen liegen im selben Ordner und stehen absichtlich nicht in der Aufzählung:
die Nachbardatei `*.neu` und die Sicherung `*.beschaedigt` (`ablage/atomar.rs:46-49`). Beide
werden über `mit_endung` aus einem Ziel gebildet, das selbst aus `Datei` stammt; sie sind damit
kein Weg an der Aufzählung vorbei, sondern Ableitungen aus ihr. Die zwei Sperrdateien
`schreiben.lock` und `sitzungsrecht.lock` (`ablage/sperre.rs:80-83`) tragen keinen Inhalt, den
irgendjemand liest, und gehören schon deshalb nicht in eine Aufzählung des Bestands; ihre
Vollständigkeit hält die eigene Probe `ueber_der_ablage_stehen_genau_zwei_absprachen`
(`tests/baum.rs:216-241`).

**Kein Befund.**

### `260812-1204` — semantisch widersprüchliche `keymap.toml` wird nicht zur Seite gelegt

**Der Defekt hält unverändert.** `tasten::belegung::laden`
(`crates/krk-core/src/tasten/belegung.rs:1492-1512`) baut seine `Ersetzung` weiterhin eine Ebene
über `Zugang::laden` und trägt `Beiseite::Nicht`; der Code verweist inzwischen selbst auf den
Datensatz (`belegung.rs:1507-1509`). Beide dort offengelassenen Wege stehen noch offen:
`Zugang::beiseite_legen` ist weiter privat (`ablage/mod.rs:862`), und die semantische Prüfung
wohnt weiter im Tastenmodul. Nachtrag am Datensatz eingetragen.

### `260812-1529` — Ablagedatei mit ungültigem UTF-8 wird nicht zur Seite gelegt

**Der Defekt hält, und eine seiner zwei genannten Hürden ist gefallen.** `Zugang::laden` liest
weiter mit `fs::read_to_string` (`ablage/mod.rs:628`); jeder Fehler, `InvalidData`
eingeschlossen, landet im selben Zweig mit `Grund::NichtLesbar` und `Beiseite::Nicht`
(`:636-646`). Der Datensatz verwarf seinen ersten Behebungsweg damals mit dem Satz „nur nimmt
`atomar::schreiben` heute ein `&str`, also bräuchte der Weg eine Byte-Fassung daneben, und ein
zweiter Schreibweg ist durch den Datensatz vom 260812-1105 ausgeschlossen". Seit der Runde 9
nimmt `atomar::schreiben` einen `&mut impl Read` (`ablage/atomar.rs:174`), und
`Zugang::beiseite_legen` reicht bereits einen `&[u8]` als Leser herein (`mod.rs:672`). Der Weg
zu den zwei Zetteldateien geht ihn schon: `Textstand::Unlesbar` reicht seinen **offenen
Deskriptor** an dieselbe Funktion (`mod.rs:772-773`), und `Unlesbarkeit::KeinText` wird dort
ausdrücklich gesichert (`:776-778`). Die Lücke besteht damit nur noch für die fünf
TOML-Dateien, und sie ist heute billiger zu schließen als am 260812. Nachtrag am Datensatz
eingetragen.

### `260821-0142` — die strenge Bestandsregel für `session.toml` und `keymap.toml`

Die Frage ist nicht zu beantworten, und dieser Bericht beantwortet sie nicht. **Was der Code
heute tut:** `Datei::leerbefund` (`ablage/pfade.rs:303-312`) gibt `Leerbefund::Beschaedigt`
allein für `Datei::Lesezeichen` zurück; `keymap.toml`, `session.toml`, `settings.toml`,
`readers.toml` und die zwei Zettel tragen `Leerbefund::Vorgabe`. `Sitzung`
(`ablage/sitzung.rs:319-321`) trägt weiterhin `#[serde(default)]` und **kein**
`deny_unknown_fields`; `Lesezeichenliste` (`ablage/lesezeichen.rs:349-351`) trägt beides. Die
Messung, die die dritte Randbedingung des Datensatzes verlangt — schreibt KRK je eine
`session.toml` ohne obersten Schlüssel —, ist nicht gefahren. Der Marker steht zu Recht auf
offen.

Ein Nebenbefund dazu, gemessen und nicht geraten: `Sitzung` trägt `fenster: [Dateifenster; 2]`,
und ein Feld fester Länge weist eine abweichende Zahl von `[[fenster]]`-Blöcken über `serde`
als Fehler ab. Die Zusage am Feld („Ein `session.toml` mit einer anderen Zahl ist beschädigt und
führt zum Auslieferungszustand", `sitzung.rs:379-380`) hält also, unabhängig von der offenen
Frage.

### `260825-1725` — wie neue Auslieferungsprofile einen Nutzer erreichen

**Umgesetzt wie zugesagt.** Der Datensatz ist auf Möglichkeit 1 beantwortet und mit `d04e50f`
als umgesetzt markiert. `README.md:44-73` trägt den Abschnitt `## Neue Leseprofile übernehmen`
mit dem vollen Pfad, den drei Schritten in der zugesagten Reihenfolge, der ausdrücklichen
Aussage, dass ohne den Handgriff nichts sichtbar wird und keine Meldung kommt, und der
Begründung für „beiseitelegen und nicht löschen". `ablage::leseprofile::anlegen_falls_fehlt`
(`ablage/leseprofile.rs:128-134`) ist unverändert und hält C1.2: eine vorhandene Datei wird
nicht angefasst. Der im README empfohlene Ausweichname `readers.toml.alt` kollidiert mit
nichts — `Ablageort::datei` liest keinen abgeleiteten Namen als Ablagedatei
(`ablage/atomar.rs:81-84`), und die Sicherung heißt `readers.toml.beschaedigt`.

**Kein Befund.**

### `#![deny(unsafe_code)]` und `#[must_use]`

Keine der zwölf Dateien trägt `#![allow(unsafe_code)]`, und keine spricht einen Fremdaufruf
unmittelbar an; die `flock`-Aufrufe der Sperre gehen über `verzeichnis::sys`
(`ablage/sperre.rs:77`), also über die eine Datei des Kerns mit der Ausnahme. Die Zusage hält.

Die `#[must_use]`-Regel hält **nicht durchgehend**; siehe den ersten Befund unten.

## Findings by theme

### Verlorene Meldungen: `Geladen<T>` ohne `#[must_use]` — Medium

`shared/issues/260826-1225_o_geladen-traegt-kein-must-use-und-vier-der-fuenf-ladewege-koennen-ihre-ersetzung-still-fallen-lassen.md`

`Geladen<T>` (`ablage/mod.rs:474-480`) trägt das Feld `ersetzung`, und das ist die **einzige**
Auskunft darüber, dass eine Ablagedatei beschädigt war, unter `atomar::beiseitepfad` zur Seite
gelegt wurde und durch den Auslieferungszustand ersetzt ist. Weder der Typ noch einer der vier
Ladewege trägt `#[must_use]`. `zugang.laden::<Sitzung>(Datei::Sitzung);` als Anweisung ohne
Bindung baut grün, und der Nutzer erfährt nie, dass sein Bestand ersetzt worden ist.

Die Auslassung ist nicht als Abwägung zu lesen, weil der unmittelbare Nachbar sie nicht macht:

```rust
// ablage/leseprofile.rs:100-103   MIT
#[must_use = "die zweite Haelfte des Paares sind die Meldungen ueber abgewiesene …"]
pub fn laden(zugang: &Zugang<'_>) -> (Geladen<Profile>, Vec<String>) {

// ablage/einstellungen.rs:149     OHNE
pub fn laden(zugang: &Zugang<'_>) -> Geladen<Einstellungen> {
```

Beide sind der Ladeweg einer von Hand gepflegten Ablagedatei, beide liefern eine `Ersetzung`.
Die Behebung ist **ein** Vermerk am Typ und nicht fünf an den Funktionen: er deckt
`Zugang::laden`, `Zugang::text_laden`, `einstellungen::laden`, `belegung::laden` und die erste
Hälfte des Paares aus `leseprofile::laden` auf einmal ab. `Geladen::mit_meldung` (`:493`)
braucht einen eigenen, denn sein `(T, Option<String>)` ist kein `Geladen` mehr.

### Drei Prosastellen, die jedes bisherige Suchmuster übersehen musste — Low

`shared/issues/260826-1225_o_drei-prosastellen-der-ablage-nennen-die-zahl-der-dateien-falsch-und-jedes-bisherige-suchmuster-musste-sie-uebersehen.md`

`sperre.rs:3-4`, `sperre.rs:40` und `einstellungen.rs:1` nennen die Zahl der Ablagedateien
beziehungsweise die Zahl der von Hand gepflegten falsch. Die dritte widerspricht der
Nachbardatei wörtlich: `einstellungen.rs:1` sagt „die **eine** Ablagedatei, die der Nutzer von
Hand pflegt", `leseprofile.rs:1-2` sagt „die **zweite**".

Der Befund ist nicht die Zahl, sondern warum vier Erhebungen seit dem 260814 sie nicht gefunden
haben. Gemessen am Stand `79dab20`, den die zweite Erhebung selbst nennt:

```sh
git grep -nE "vier Dateien|vier Ablagedateien|vier Lade- und Schreibmethoden" 79dab20 -- crates | wc -l
# 17 — dieselbe Zahl, die der Datensatz nennt
git grep -nE "vier Dateien|vier Ablagedateien" 79dab20 -- crates/krk-core/src/ablage/sperre.rs
# keine Zeile
```

Das Wortpaar steht in `sperre.rs` über einen Zeilenumbruch verteilt (`tr '\n' ' '` liefert
„dieselben vier //! Dateien"), ist also für jede zeilenweise Suche unsichtbar. Die zweite
Stelle sagt „Nutzdateien", die dritte „eine" — beide wären auch von einer zeilenübergreifenden
Suche nach diesem Muster nicht erfasst worden. Es ist der zweite Fall derselben Art wie der,
den CLAUDE.md schon führt („Wer eine Erhebung fährt, erweitert das Muster, bevor er zählt").

### Eine Invariante, die nur ein Kommentar behauptet — Low

`shared/issues/260826-1225_o_die-merkliste-der-lesungen-begruendet-ihre-form-mit-einer-schranke-die-der-code-nicht-haelt.md`

Der Doc-Kommentar von `Lauf::staende` (`leseprofil/bausteine.rs:342-344`) begründet die
Listenform mit „es sind hoechstens so viele Eintraege, wie `HOECHSTENS_LESELAEUFE` zulaesst".
`stand_am` (`:380-393`) merkt jeden **verschiedenen Ort**, auch die, an denen wegen erschöpftem
Haushalt gar nicht gelesen wurde — was zwei Zeilen darüber (`:338`) selbst ausgeschrieben ist.
Die Probe `dreizehn_zaehlbausteine_erreichen_die_grenze_und_der_rest_traegt_den_platzhalter`
(`tests/leseprofil.rs:2673`) fährt 13 Zeilen mit je eigenem Ort plus den erkannten Ordner: 14
Einträge bei einer Schranke von 12. Kein Verhalten ist betroffen, die Begründung ist zu
berichtigen.

### `juengste` mit `anzahl = 0` — Low

`shared/issues/260826-1225_o_juengste-mit-anzahl-null-wird-still-angenommen-und-kann-nie-etwas-sagen.md`

`gekappte_anzahl` (`leseprofil/datei.rs:524-528`) deckelt nach oben und kennt keine untere
Schranke. `anzahl = 0` kommt ohne Meldung durch, `kandidaten.truncate(0)`
(`bausteine.rs:645-648`) leert die Liste, und die Zeile trägt in jeder Zusammenfassung den
Platzhalter. Das Modul kennt den Satz für genau diesen Fall bereits und wendet ihn eine Ebene
höher an: ein Profil ohne beide Erkennungsmuster wird abgewiesen, weil es „nie treffen" könnte
(`datei.rs:363-370`). Eine Zeile, die nie etwas sagen kann, fällt in keine der drei Reichweiten,
die der Modulkopf aufzählt.

### Zwei tote öffentliche Zugänge — Low

`shared/issues/260826-1225_o_zwei-oeffentliche-zugaenge-der-ablage-haben-im-ganzen-arbeitsbereich-keinen-rufer.md`

`Lesezeichenliste::eintrag` (`ablage/lesezeichen.rs:368-370`) und `Nachbardatei::ziel`
(`ablage/atomar.rs:123-125`) haben in `crates/` und `xtask/` keinen einzigen Rufer, auch keine
Probe. Beide sind `pub` in einer Bibliothekskiste, also warnt der Übersetzer nicht. Der
Nachbar `Nachbardatei::nachbarpfad` hat zwei Rufer, und die Liste wird überall über das offene
Feld `eintraege` gelesen — der prüfende Zugang daneben ist damit nicht nur ungerufen, sondern
auch nicht die Form, die der Baum benutzt. Dieselbe Bauart hat eine Schwesterdurchsicht in
anderen Modulen gefunden
(`shared/issues/260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`);
dort hat ein Teil der Namen wenigstens eine eigene Probe, diese zwei haben auch die nicht.

### Zwei Schreibweisen des Deutschen in einer Statuszeile — Entscheidung, nicht Defekt

`shared/decisions/260826-1225_o_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`

`leseprofil/mod.rs` schreibt an `:726` „(Lesung bei 2000 **Einträgen** abgebrochen)" und an
`:530` „**traegt** ein leeres **Stueck**", letzteres in einer Funktion, deren Doc-Kommentar
ausdrücklich sagt „so wie ihn die Statuszeile zeigt". Beide Sätze können in derselben Sitzung in
derselben Zeile erscheinen. Im Umfang dieser Durchsicht tragen rund fünfzehn nutzersichtbare
Zeichenketten die Umschrift; der übrige Baum hält es umgekehrt (`text/datei.rs:263-273`
„lässt sich nicht im Editor öffnen", `tasten/belegung.rs:325` „Geräteleiste`",
`krk-ui/src/spalten.rs:87` „Größe", `krk-ui/src/menuemodell.rs:117` „Über KRK").

**Das ist als Entscheidung und nicht als Defekt gefiltert**, weil die Behebungsrichtung nicht
aus dem Code folgt: nichts im Baum und nichts in CLAUDE.md sagt, welche der beiden Schreibweisen
für nutzersichtbare Prosa gilt. Erst nach der Antwort ist der Durchgang mechanisch — und auch
dann nicht über ein Wortmuster, denn „traegt" ist in einem Doc-Kommentar richtig und in einer
Meldung falsch.

## Cross-cutting observations

**Die `#[must_use]`-Regel ist über die ganze Kiste ungleich angewandt, und vier Prüfer haben es
in derselben Sitzung unabhängig voneinander an vier Modulgruppen gefunden.** Neben dem Befund
oben stehen `shared/issues/260826-1221_*_must-use-traegt-sieben-praedikate-des-verzeichnisbaums-…`
(`verzeichnis/`), `…_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-…`
(`operation/`, `stapelumbenennen/`) und
`shared/issues/260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-…` (`tasten/`,
`text/`). Vier unabhängige Befunde derselben Regel an vier Modulgruppen sind kein Zufall der
Durchsicht, sondern die Auskunft, dass die Regel seit ihrer Setzung am 260811-2140 nie in einem
Zug über die Kiste gezogen worden ist. **Sie gehören als ein Durchgang geräumt und nicht als
vier**; dieser hier ist der einzige der vier, bei dem der verlorene Wert kein gerechnetes
Ergebnis ist, sondern eine Meldung an den Nutzer über einen bereits geschriebenen
Datenverlust — und der einzige, der mit einem einzigen Vermerk erledigt ist.

**Die Zählerhebungen dieses Projekts messen wiederholt dieselben zwei bis drei Dateien.** Vier
Erhebungen zur Zahl der Ablagedateien haben `ablage/mod.rs`, `ablage/pfade.rs` und
`tests/ablage.rs` gelesen und jedes Mal die Zahl bestätigt bekommen, die sie schon kannten;
`sperre.rs` und `einstellungen.rs` kamen in keiner vor. Zusammen mit dem schon geführten Fall
der Kurzform-Verweise (`shared/issues/260810-1851_*`) ist das die zweite Erhebung dieses
Projekts, die an ihrem eigenen Suchmuster gescheitert ist. Die Gegenmaßnahme ist nicht ein
schärferes Muster, sondern der Umfang: über das Verzeichnis suchen und nicht über die Dateien
der vorigen Erhebung.

**Die Sicherung einer beschädigten Datei hängt weiter an einem einzigen Zweig, und drei Wege
gehen an ihm vorbei.** `Zugang::beiseite_legen` wird allein aus `Grund::Beschaedigt` gerufen.
Vorbei gehen: ungültiges UTF-8 (`260812-1529`, hält), die semantisch widersprüchliche
`keymap.toml` (`260812-1204`, hält) und die nicht lesbare Datei
(`shared/issues/260821-0142_*`, hält). Alle drei sind seit langem geführt, alle drei teilen
dieselbe Ursache, und der Modulkopf schreibt sie inzwischen selbst aus (`ablage/mod.rs:167-176`).
Das ist keine Häufung von Versäumnissen, sondern eine Naht, an der drei Datensätze auf dieselbe
Entscheidung warten — und für den ersten der drei ist die damals genannte Hürde inzwischen
gefallen.

## Was ausdrücklich nicht gefiltert ist

- **`atomar::vorbereiten` synchronisiert die Datei und nicht ihr Verzeichnis** (`atomar.rs:164`).
  Nach dem `rename` bleibt der Verzeichniseintrag ungeschrieben, bis das Dateisystem ihn
  ausschreibt. `speculation:` Ob das auf APFS eine Rolle spielt, ist in diesem Baum durch
  nichts gemessen, und die Zusage des Modulkopfs — ein Leser sieht den alten Inhalt ganz oder
  den neuen ganz — ist davon nicht berührt. Ein Datensatz ohne Messung wäre eine Vermutung im
  Bestand; er ist deshalb nicht gefiltert, sondern hier benannt.
- **Ein Platzhalterlauf kann bis zu 2.000 Verzeichnisse öffnen** und bucht dafür einen einzigen
  Leselauf (`bausteine.rs:444-477`). Das ist bereits geführt
  (`shared/issues/260825-1953_*_ein-platzhalterlauf-oeffnet-bis-zu-zweitausend-verzeichnisse-…`)
  und wird hier nicht doppelt gefiltert. Nachgemessen ist dabei: der Lauf hängt am Arbeitsfaden
  eines ausgewählten Eintrags und nicht am Hauptfaden
  (`krk-ui/src/vorschaumodell.rs:1430`, Zählprobe), die Oberfläche steht also nicht still.
- **`Dateifenster::tabs` trägt am Typ die Zusage „Nie leer, siehe C1"** (`sitzung.rs:145`), und
  der Typ hält sie nicht. Gehalten wird sie beim Verbraucher: `Tabliste::aus_zustand`
  (`krk-ui/src/tabs.rs:412-427`) legt einen Tab an, wenn die Liste leer ankommt, mit
  ausgeschriebener Begründung. Die Zusage steht damit an der falschen Stelle, aber sie steht;
  ein Defekt daraus wäre eine Aussage über den Zuschnitt und keine über ein Fehlverhalten.
- **`einzeilig` ist nicht auf jeden Meldungstext angewandt** (`ablage/mod.rs:642`, `:668`,
  `einstellungen.rs:168`, `leseprofile.rs:114`). Nachgemessen und **kein Befund**: die
  einzigen mehrzeiligen Quellen sind `toml::de::Error` und `regex::Error`, und beide gehen
  durch eine der zwei Stellen, die `einzeilig` rufen. `io::Error` und `Belegungsfehler`
  (`tasten/belegung.rs:1584-1604`) sind einzeilig.

## Recommended sequencing

Nichts davon hält eine Auslieferung auf. In der Reihenfolge, in der es sich lohnt:

1. **`#[must_use]` an `Geladen<T>` und `mit_meldung`** — ein Vermerk, und er schließt den einen
   Weg dieser Durchsicht, auf dem ein Nutzer von einem geschriebenen Datenverlust nichts
   erführe. Am besten zusammen mit den drei Schwesterdatensätzen als ein Durchgang über die
   Kiste.
2. **Die Entscheidung zur Schreibweise** — sie kostet nichts zu beantworten und blockiert die
   Räumung von rund fünfzehn Zeichenketten.
3. **Die drei Prosastellen und die zwei toten Zugänge** — Aufräumarbeit, jederzeit.
4. **`anzahl = 0` und die Begründung an `Lauf::staende`** — beides eine Zeile.

## Verification

Zwölf von zwölf Dateien vollständig geöffnet und gelesen. Jede Zeilenangabe dieses Berichts ist
am Baumstand `004ff72` nachgesehen. `cargo clippy -p krk-core --all-targets` läuft ohne
Warnung durch, gefahren mit eigenem `CARGO_TARGET_DIR` außerhalb des Projektbaums. Die
serde-Verhalten von `Lesezeichenliste` (fehlendes Ziel, verschriebener Schlüssel, fremder
oberster Schlüssel, leere Datei) sind an einer Wegwerf-Kiste gegen `krk-core` gemessen und
nicht angenommen; die Vermutung eines stillen Rückfalls auf `Ziel::default()` hat sich dabei
**nicht** bestätigt und ist deshalb kein Befund. Keine Datei des Quellbaums ist verändert
worden, `~/Library/Application Support/KRK/` ist nicht angefasst worden.
