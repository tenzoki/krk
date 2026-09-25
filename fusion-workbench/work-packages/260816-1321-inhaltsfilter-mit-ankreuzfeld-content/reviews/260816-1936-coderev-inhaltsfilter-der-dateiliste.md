# Durchsicht: Der Inhaltsfilter der Dateiliste (elfte Runde)

**Sender:** coderev
**Reviewed-range:** `9f5ced5..b9ab8ae`
**Not-opened:** `crates/krk-core/tests/belegung.rs`, `crates/krk-ui/src/appkit/menue.rs`, `crates/krk-ui/src/belegungsausgabe.rs`, `crates/krk-ui/src/belegungsmodell.rs`, `crates/krk-ui/src/appkit/anwendung.rs`
**Datum:** 2026-08-16
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Spec:** `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` (sechs Fähigkeiten, 57 Abnahmekriterien)
**Plan:** `planning/260816-1359_c_plan-inhaltsfilter-der-dateiliste.md` (zwölf Schritte, alle `[DONE]`)

**Zur Angabe `Not-opened:`.** Die fünf genannten Dateien sind über ihren vollen Diff im
Bereich gelesen und nicht als ganze Datei geöffnet; ihr Anteil an dieser Runde ist die
Eintragung eines Kommandos und das Nachziehen von vier Zahlen, und beides ist am Diff
vollständig. `crates/krk-core/tests/text.rs` und `crates/krk-core/tests/verzeichnis.rs`
sind abschnittsweise geöffnet (die Proben der Lesehülle, der Abbruchgrenze und des
Deskriptormangels) und im Übrigen über den Diff gelesen; sie stehen deshalb nicht in der
Liste. Alle übrigen Dateien des Bereichs sind ganz geöffnet.

## Zusammenfassung

Die Runde hält, was sie als teuerste Zusage benannt hat: **kein Weg im Baum öffnet eine
Datei, deren Name die Folge schon trägt, unterhalb der Schwelle wird gar nicht gelesen, und
eine Datei über 1 MB wird nicht gelesen.** Alle drei sind am Baum nachgelesen und nicht
angenommen. Der Deskriptorhaushalt hält ebenso: ein Verzeichnisdeskriptor und höchstens ein
Dateideskriptor, und der zweite fällt vor dem nächsten Kandidaten. `make check` läuft grün,
Clippy und `fmt` sind sauber.

Sechs Befunde stehen dagegen, und **keiner davon ist eine falsche Antwort des Kerns.** Fünf
liegen an der Naht zwischen Kern und Oberfläche oder an der Prosa; einer ist eine
Kostenstelle, die niemand gewogen hat. Der schwerste ist eine Lücke in C2.9: das
Ausschalten von „Content" wirkt für eine Datei sofort und für einen Ordner erst nach einem
ganzen neuen Unterbaumlauf.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch (Auslieferungssperre, Sicherheit, Datenverlust) | 0 |
| Hoch (falsches Ergebnis, gebrochener Ablauf) | 0 |
| Mittel (Korrektheitsrisiko, unbenannte Kosten, Wartbarkeit) | 4 |
| Niedrig (Prosa, Kosmetik) | 2 |

## Was am Baum nachgelesen ist und hält

Die vier Punkte, auf die es der Durchsicht besonders ankommen sollte, halten. Sie stehen
hier mit ihrem Beleg, damit die Zusagen nachprüfbar bleiben und nicht nur behauptet sind.

**Gelesen wird nur, was gelesen werden soll.** Der einzige Weg zu `traegt_der_inhalt` ist
`datei_entscheiden` (`crates/krk-core/src/verzeichnis/durchlauf.rs:388-403`), und der hat
genau zwei Rufer. Im flachen Zweig steht davor der Kurzschluss des Namens in der
Auftragsliste: `auftraege` filtert auf `!name_traegt_den_filter`
(`crates/krk-ui/src/tabs.rs:1084`), also bekommt eine namentlich passende Datei gar keinen
Auftrag. Im tiefen Zweig steht er im Rumpf: `traegt_die_folge(&kandidat.name, …)` kehrt
mit `Some(true)` zurück, bevor der Typzweig erreicht ist
(`durchlauf.rs:497-505`). Die Schwelle wird an genau einer Stelle geprüft,
`Ordnermodell::inhalt_wirkt` (`crates/krk-core/src/verzeichnis/modell.rs:838-841`), und
sowohl die Auftragsart als auch die Grenze leiten sich aus ihr ab
(`tabs.rs:1089` und `:920-922`) — unterhalb der Schwelle ist `inhaltsgrenze` `None`, und
`durchlauf.rs:516-518` steigt dann über jede Datei hinweg. Die 1 MB hält
`bis_zur_grenze_lesen` zweimal: einmal am `fstat` vor dem Lesen und einmal am
`take(grenze + 1)` danach (`crates/krk-core/src/text/datei.rs:616-634`).

**Der Deskriptorhaushalt.** `Lesestand` hält den einen `Schwungleser`
(`durchlauf.rs:568-584`), die vorgemerkten Ordner stehen als Pfad auf `offen`
(`durchlauf.rs:464`, `:514`). `bis_zur_grenze_lesen` bindet die geöffnete Datei an eine
lokale Bindung und gibt sie beim Verlassen frei (`text/datei.rs:606-634`) — der
Dateideskriptor lebt damit genau so lange wie ein Aufruf und ist vor dem nächsten Kandidaten
weg.

**Der Abbruch (C4.7).** Die Prüfung steht an drei Stellen, und alle drei stehen unmittelbar
vor einer Einheit, die dauern kann: vor dem nächsten Stapel eines Ordners
(`durchlauf.rs:482`), vor der nächsten gelesenen Datei im Unterbaum (`durchlauf.rs:523`) und
vor der gelesenen Datei im flachen Zweig (`durchlauf.rs:342`). Die Zählprobe
`die_abbruchgrenze_steht_vor_jedem_stapel_und_vor_jeder_datei`
(`crates/krk-core/tests/verzeichnis.rs:2128-2156`) hält die Verteilung fest und benennt
selbst, was sie nicht entscheidet. **Ein Weg an ihnen vorbei besteht nicht:** jeder Aufruf
von `traegt_der_inhalt` läuft durch `datei_entscheiden`, und beide Rufer prüfen davor.

**Die vier Aussagen in einer Zelle.** Farbe und Schrift werden in **jedem** Durchgang
gesetzt, nicht nur im markierten Fall (`crates/krk-ui/src/appkit/tabelle.rs:2922-2936`), und
die Fallunterscheidung über die Farbe ist überschneidungsfrei und vollständig: markiert →
orange, sonst Inhaltstreffer → gedämpft, sonst Grundfarbe. Die Ausschließlichkeit der
beiden Treffergründe leistet der Kurzschluss und keine zusätzliche Regel:
`steht_wegen_des_inhalts` schließt einen Namenstreffer ausdrücklich aus
(`modell.rs:706-708`). Die Rangfolge zwischen Markierung und Dämpfung ist entschieden und
im Kommentar belegt (`tabelle.rs:2893-2905`, Datensatz `260816-1359_i_welche-aussage-…`).

**Die Zusagen der Runde 10.** Der Tabwechsel beendet den Durchlauf jetzt, und die
Begründung ist **ersetzt und nicht gelöscht**: `tabs.rs:834-848` schreibt die alte
Begründung aus, sagt, warum sie mit dem Inhaltsfilter anders wiegt, nennt den
Nutzerentscheid vom 260816-1410 samt Datensatz und benennt den Preis. Der Zuschnitt des
Einzugstakts, der über alle Tabs fragt, ist ausdrücklich als „gegenstandslos statt falsch"
eingeordnet (`tabs.rs:850-854`). Eine stillschweigend mitgefallene Zusage der Runde 10 ist
mir nicht begegnet; C3.14 („je Tab nie mehr als ein Durchlauf") und C4.5 halten unverändert,
und `crates/krk-bench/` ist von keinem der Commits angefasst, die zehn Zeitzusagen sind also
unberührt.

**Die Hausregeln.** `#[must_use]` steht an jedem neuen Rückgabewert, dessen stilles
Fallenlassen unbemerkt bliebe (`traegt_der_inhalt`, `inhaltsschwelle`,
`steht_wegen_des_inhalts`, `inhalt`, `inhalt_wirkt`). Die Fallunterscheidungen über
`Auftragsart`, `Inhaltsbefund`, `Lesehindernis` und `Befund` haben keinen Auffangzweig, und
die eine unerreichbare Paarung `(Auftragsart::Inhalt, None)` ist ausgeschrieben statt
weggelassen (`durchlauf.rs:348-354`) — das ist die richtige Wahl, ein Auffangzweig hätte
sie stillschweigend negativ entschieden. Der einzige neue `unsafe`-Block trägt seine
Begründung (`crates/krk-ui/src/appkit/bereichsleiste.rs:333-334`). Der Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen` steht in 38 der 40 Dateien unter
`crates/krk-ui/src/appkit/`; die zwei Fehlenden sind `koordinaten.rs` und `mod.rs`, also die
zwei begründeten Ausnahmen.

## Befunde nach Thema

### Thema 1: Die Naht zwischen Schalter und Befundvektor

**M1 — Das Ausschalten von „Content" bei eingeschaltetem „Deep" lässt Ordnerzeilen auf
einem veralteten Inhaltsbefund stehen.** Mittel. Betrifft `krk-core`.
Datensatz: `issues/260816-1930_o_content-ausschalten-laesst-ordnerzeilen-auf-einem-veralteten-inhaltsbefund-stehen.md`

`Ordnermodell::inhalt_setzen` (`modell.rs:806-822`) setzt den Befundvektor nur beim
Einschalten zurück, mit der Begründung „weil ihn dann **für eine Datei** niemand liest".
Für einen Ordner liest ihn der Prüfschritt weiter, und dort hängt er an `tief` und nicht an
`inhalt` (`modell.rs:621-632`). Ein Ordner, der allein wegen eines Inhaltstreffers unter
sich stand, bleibt deshalb nach dem Ausschalten stehen, bis der neue Lauf ihn neu
entscheidet — nach dem eigenen Text der Runde bis zu Minuten (`tabs.rs:839-841`). C2.9
verlangt „sofort".

**Die zugehörige Prosastelle ist unabhängig davon falsch:** `tief_setzen` begründet mit
„weil ihn dann niemand liest" (`modell.rs:787`), und seit dieser Runde liest ihn der
Dateizweig, gleich wie `tief` steht.

### Thema 2: Was gelesen wird, ohne dass jemand es gewogen hat

**M2 — Der Inhaltsfilter liest versteckte Dateien, deren Zeile nie stehen kann, und steigt
in versteckte Ordner ab.** Mittel. Betrifft `krk-ui` (Auftragsliste) und `krk-core`
(Abstieg).
Datensatz: `issues/260816-1931_o_der-inhaltsfilter-liest-versteckte-dateien-und-steigt-in-versteckte-ordner-ab.md`

`auftraege` filtert nicht auf `versteckt` (`tabs.rs:1077-1098`), und der Kommentar darüber
begründet das mit einer Erwägung der Runde 10, als ein Befund einen Metadatengang kostete
und nur für Ordner anfiel. Jetzt kostet er je versteckter Datei ein `open(2)` und bis zu
1 MB gelesene Bytes. Der Abstieg kennt ebenfalls kein Versteckt-Kennzeichen
(`durchlauf.rs:513-532`): ein Quellbaum mit „Deep" und „Content" liest `.git` mit, also
lauter zlib-gepackte Objekte, die am `String::from_utf8` herausfallen. In Spec, Plan und
allen zwölf Sitzungsprotokollen kommt der Fall nicht vor.

**M3 — Die Auftragsliste legt je Tastendruck einen Namen je nicht passender Datei an, auf
dem Hauptfaden.** Mittel. Betrifft `krk-ui`.
Datensatz: `issues/260816-1933_o_die-auftragsliste-legt-je-tastendruck-einen-namen-je-datei-an-auf-dem-hauptfaden.md`

`nach_filteraenderung` (`tabelle.rs:1339-1342`) ruft bei jedem Zeichen
`durchlauf_nachziehen`, das `auftraege` über den ganzen Bestand laufen lässt. Je Eintrag
schreibt `name_traegt_den_filter` den Namen einmal klein, je überlebendem Eintrag kommt
`eintrag.name.clone()` dazu. Bis zur Runde 10 überlebte diesen Filter nur ein Ordner und
auch das nur bei „Deep"; seither jede Datei, sobald „Content" wirkt. Am großen Prüfordner
sind das rund 100.000 zusätzliche Zeichenketten je Tastendruck, dazu ein Fadenstart, den
der nächste Tastendruck wieder abbricht. Die Runde hat entschieden, dafür keine elfte
Zeitzusage zu setzen; die Entscheidung galt der Dauer des Durchlaufs auf dem Arbeitsfaden,
und diese Kostenstelle liegt auf dem Hauptfaden und ist nirgends benannt.

### Thema 3: Ein Ausgang, den die Oberfläche nicht kennt

**M4 — Ein Deskriptormangel beendet den ganzen Durchlauf still, und die Statuszeile nimmt
dabei genau den Hinweis zurück, der die Liste als unfertig auswies.** Mittel. Betrifft
`krk-ui`.
Datensatz: `issues/260816-1932_o_ein-deskriptormangel-beendet-den-durchlauf-still-und-die-statuszeile-nimmt-den-lesehinweis-zurueck.md`

`Inhaltsbefund::Unentschieden` wird zu `None` (`durchlauf.rs:401`), und `None` beendet den
ganzen Faden. Am Hauptfaden räumt `befunde_einziehen` den Lauf weg (`tabs.rs:1145-1159`),
`liest_inhalt` fällt auf `false`, und der Satzteil „, Inhalt wird gelesen" verschwindet
(`statuszeile.rs:430-434`). Was danach dasteht, ist von einem fertigen Filterstand nicht zu
unterscheiden. Der Kern antwortet richtig — die Kindprobe unter `ulimit -n 64` belegt es —,
die Oberfläche hat für diesen Ausgang keinen Zweig.

### Thema 4: Prosa gegen Baum

**L1 — Sechs Prosastellen im Baum beschreiben den Stand vor der elften Runde.** Niedrig.
Datensatz: `issues/260816-1934_o_sechs-prosastellen-im-baum-beschreiben-den-stand-vor-der-elften-runde.md`

`sys.rs:802` (ein Aufrufer der Hülle statt zwei), `verweisziel.rs:42` (die Vorschau in
`krk-ui` als Leseweg), `text/datei.rs:598` (`/dev/zero` als Begründung einer Schranke, die
es nie erreicht), `text/datei.rs:646` (zwei Stellen für `String::from_utf8`, es sind drei),
`tabs.rs:661` („die vierte Übertragung", es ist die fünfte), `tabs.rs:825` und
`tabelle.rs:1325` (beide nennen „Deep" als einzigen Anlass eines Durchlaufs).

**L2 — CLAUDE.md nennt zwei Filterregeln, zwei Rufer je Regel und eine Lesehülle in
`krk-ui`.** Niedrig.
Datensatz: `issues/260816-1935_o_claude-md-nennt-zwei-filterregeln-und-eine-huelle-in-krk-ui-beides-hat-die-elfte-runde-abgeloest.md`

`CLAUDE.md:127`, `:131`, `:135` und die Rundentabelle.

## Übergreifende Beobachtungen

**Die Prosa läuft an den Nähten weg und nicht in der Mitte.** Von den elf Prosastellen
dieser Durchsicht steht keine in einer Datei, die die Runde umgebaut hat — sie stehen in
Dateien, die die Runde **berührt hat, ohne sie umzubauen** (`sys.rs`, `text/datei.rs`), in
einer, die sie gar nicht angefasst hat (`verweisziel.rs`), und in Zeilenkommentaren neben
frisch nachgezogenen Doc-Kommentaren (`tabs.rs:650-654` ist nachgezogen, `tabs.rs:661` nicht).
Der Nachzug `b9ab8ae` hat dieselbe Verteilung gefunden. Ein wirksamer Griff wäre nicht mehr
Sorgfalt, sondern eine Liste: wer eine Funktion verschiebt, sucht ihren alten Namen und
ihren alten Ort im ganzen Baum, statt die Dateien nachzuziehen, die er ohnehin offen hat.

**Drei der vier Mittel-Befunde teilen eine Ursache: eine Erwägung der Runde 10 ist mit
verändertem Preisschild weitergereicht worden.** Der versteckte Eintrag im Auftrag (M2), die
Namenskopie je Eintrag (M3) und der Befundvektor, den beim Ausschalten „niemand liest" (M1)
waren alle drei richtig, solange ein Befund einen Metadatengang kostete und nur Ordner
betraf. Der Inhaltsfilter hat den Preis je Auftrag um Größenordnungen gehoben und die
Menge der Aufträge von den Ordnern auf alle Einträge erweitert; die Begründungen sind
mitgewandert, ohne neu geprüft zu werden. Wer die drei einzeln behebt, behebt dreimal
dasselbe Versäumnis.

**Die Runde ist bei dem, was sie gemessen hat, ungewöhnlich ehrlich.** Die Abnahmeliste
`messungen/260816-abnahme-inhaltsfilter.md` benennt sechs ungemessene Stellen von selbst,
darunter den Kontrast der Dämpfung gegen die blaue Auswahlfläche und die Grenze der
`/dev/zero`-Probe. Keiner meiner Befunde steht dort schon; die Liste hat mir umgekehrt zwei
Verdachtsfälle abgenommen, die ich sonst gemeldet hätte.

## Reihenfolge

**Vor der Abnahme durch den Nutzer:** keiner. Alle sechs Befunde lassen die 57 Kriterien
prüfbar; M1 verschiebt die Grenze von C2.9 und macht sie nicht unprüfbar, sondern anders
prüfbar — wer den Abnahmelauf fährt, sollte beim Ausschalten von „Content" mit gesetztem
„Deep" ausdrücklich auf die **Ordnerzeilen** sehen.

**Vor dem nächsten Ausliefern:** M1, weil eine Liste, die einem gerade betätigten Schalter
widerspricht, der Sorte Fehler angehört, die der Nutzer als Defekt meldet und nicht als
Verzögerung.

**In der nächsten Sitzung, zusammen:** M2 und M3, aus dem oben genannten Grund — sie sind
dieselbe Frage an zwei Stellen, nämlich welche Einträge einen Auftrag verdienen. M4 gehört
zu `issues/260816-1710_o_…` und ist mit ihm zusammen zu entscheiden; eine Anzeige „dieser
Filterstand ist unvollständig" deckte beide ab.

**Aufräumen:** L1 und L2. L2 fällt üblicherweise beim Abschluss der Runde an.

## Was diese Durchsicht nicht entscheidet

Sie hat kein Bündel gestartet. Alles, was am laufenden KRK zu sehen ist — die Lage der
Schalter auf dem Schirm, die Lesbarkeit der Dämpfung, ob die Anwendung während eines Laufs
bedienbar bleibt, die Dauer der Abbruchspanne —, ist hier nicht geprüft, sondern am Code
gelesen. Die 28 Beobachtungen der Abnahmeliste bleiben Nutzerarbeit, und M1 gibt ihnen eine
Zeile mehr.
