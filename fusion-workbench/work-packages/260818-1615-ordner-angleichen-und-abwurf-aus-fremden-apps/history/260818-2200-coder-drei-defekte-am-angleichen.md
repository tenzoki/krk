# Coder — Drei Defekte am Angleichen: die Trennung, die Nadeln, der Rückgabewert

**Datum:** 260818-2200
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Defektsätze:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/issues/260818-2127`, `-2128`, `-2129`, `-2130`
**Baumstand beim Beginn:** `79f52af`; zwei weitere Agenten arbeiten gleichzeitig an
`appkit/zwischenablage.rs` und `kommandos/abwurfregel.rs`
**Geänderte Datei:** `crates/krk-ui/src/appkit/anwendung.rs`, und keine zweite

## Was der Auftrag war

Die drei Befunde der Durchsicht aus Runde 13 beheben, die den Befehl
`ordner_angleichen` betreffen. Einer trägt eine Entscheidung des Nutzers vom
260818, die beiden anderen sind in sich abgeschlossen. Keine der Fragen wurde
offengelassen; die Defektsätze bleiben offen, der Nutzer schließt sie selbst.

## Defekt 1: Hervorholen und Stellen sind getrennt (2128 und 2129)

Der Spec widersprach sich für eine Lage: das andere Dateifenster ist
ausgeblendet **und** zeigt schon denselben Ordner. C1 sagte „es geschieht
nichts", C2 sagte „einblenden und stellen". Das Flussdiagramm entschied
stillschweigend für C1, der Rumpf folgte ihm, und das Ergebnis war eine
Statuszeile, die über einen Bereich berichtete, den der Nutzer nicht sah.

**Der Nutzer hat die beiden Handlungen getrennt.** Sie beantworten
verschiedene Fragen, und keine folgt aus der anderen:

```
                    Ordner gleich?
                       nein          ja
Bereich   sichtbar   stellen        nichts, und die Zeile sagt es
          verdeckt   holen+stellen  nur holen, und die Zeile sagt beides
```

Umgesetzt durch Umstellen der beiden Zweige im Rumpf: die Sichtbarkeitsfrage
steht jetzt vorn und blendet unbedingt ein, der Ordnervergleich steht dahinter
und entscheidet allein über den Lesevorgang.

**Die Abweisung des zu schmalen Fensters hält weiterhin auch das Stellen an.**
Das ist keine Rückkehr zur Kette, sondern C2, zweites Kriterium: „bleibt das
andere Dateifenster ausgeblendet **und** auf seinem bisherigen Ordner". Ein
Lesevorgang dorthin kostete den Zieltab Auswahl und Bildlaufposition für eine
Anzeige, die niemand sieht.

**Die neue Meldung, und warum es zwei sind.** „das andere Dateifenster zeigt
diesen Ordner bereits" stimmt seit der Trennung nur noch, wenn wirklich nichts
geschah. Für die hervorgeholte Lage steht daneben „das andere Dateifenster
wurde eingeblendet und zeigt diesen Ordner bereits". Eine einzige Meldung für
beide Lagen ginge nur in eine Richtung schief: der Satz ohne das Einblenden
verschwiege genau die Änderung, die der Tastendruck bewirkt hat, und der Satz
mit dem Einblenden wäre in der sichtbaren Lage falsch. Der Unterschied ist eine
Klausel, und er steht als `if` am Aufrufwert von `antwort_zeigen`, nicht als
zweiter Zweig.

**Defekt 2129 ist mit demselben Griff erledigt, aber am Doc-Kommentar.** Der
alte Text nannte den durchgerutschten zweiten Lesevorgang „folgenlos". Das
stimmt nicht: `ordner_lesen` geht durch `Tabliste::ordner_setzen`, und das
ersetzt den stehenden Tab, statt ihn nachzulesen. Sortierung, „Deep",
Inhaltsfilter, die Anzeige ausgeblendeter Einträge und der Filtertext gehen von
Hand mit, Auswahl und Bildlaufposition nicht — genau darum besteht
`Tabliste::aktiven_neu_lesen` daneben. Der Doc-Kommentar nennt jetzt den Preis
und die drei Wege, auf denen zwei Schreibweisen eines Ordners entstehen. Der
Verzicht auf `canonicalize` bleibt; er ist nur nicht mehr kostenlos begründet.

## Defekt 2: Die Nadeln nennen jetzt, was der Baum ruft (2127)

`das_angleichen_ruehrt_weder_fokus_noch_sichtbarkeit_an` suchte nach
`aktiv_setzen(`. Der Setzer des Delegierten heißt `aktives_setzen`
(`anwendung.rs:4115`), und `aktiv_setzen(` ist davon keine Teilzeichenfolge.
`fokus_setzen(` und `fokus_holen(`, die beiden Wege, auf denen der Fokus in
dieser Datei überhaupt wechselt, standen gar nicht erst da. Von drei Nadeln
konnten zwei nicht anschlagen, und der Fokusteil der Probe maß nichts.

Aus drei Nadeln sind fünf geworden: `aktives_setzen(`, `fokus_setzen(`,
`fokus_holen(`, `bereich_umschalten(`, `ausblenden(`.

**Jede einzelne am Baum gegengeprüft**, nach dem Muster, mit dem in Runde 13
schon der Zweigreihenfolge-Probe nachgegangen wurde: den verbotenen Aufruf
versuchsweise in den Rumpf gesetzt, die Probe laufen lassen, den Aufruf wieder
entfernt.

| Nadel | eingesetzter Aufruf | Probe |
|---|---|---|
| `aktives_setzen(` | `self.aktives_setzen(aktiv);` | FAILED |
| `fokus_setzen(` | `let _ = self.fokus_setzen(Fokus::Dateifenster);` | FAILED |
| `fokus_holen(` | `let _ = self.fokus_holen(Fokus::Dateifenster);` | FAILED |
| `bereich_umschalten(` | `let _ = self.bereich_umschalten(Bereich::von_seite(aktiv));` | FAILED |
| `ausblenden(` | `self.editor_ausblenden();` | FAILED |

Der Baum steht danach byteweise wieder auf dem Stand vor dem Versuch; geprüft
mit `diff` gegen eine vorher angelegte Sicherung.

**Was die fünf weiterhin nicht sehen**, und der Modulkopf sagt es schon: eine
Wirkung, die aus diesem Rumpf in eine später gerufene Hilfsfunktion wandert.
Die Probe liest den Rumpf und nicht den Aufrufbaum darunter. Der Defektsatz
schlägt die stärkere Bauform von `zettelproben` vor, eine Zählung über den
ganzen Quellbaum; sie beantwortet hier aber eine andere Frage — „wie oft wird
gerufen" statt „ruft dieser Rumpf" — und ist nicht umgesetzt.

## Defekt 3: Der Rückgabewert heißt „hat gewirkt" (2130)

Erst geprüft, welche der beiden Seiten falsch ist, dann geändert.

**Der Vertrag steht am Kopf des `match` in `kommando_ausfuehren`**
(`anwendung.rs:2889-2894`): über die Zuständigkeit ist vorher entschieden, und
der Wert trägt allein die zwei Nachwirkungen `aufteilung_nachziehen` und
`sitzung_vormerken`. Die Helfer dieser Datei folgen ihm: `spalte_umschalten`
(`:3886`), `bereich_umschalten` (`:3865`) und `bereich_einblenden` (`:3962`)
liefern `false`, wenn nichts geschah. Der Satz „der Befehl war zuständig" nennt
also eine Bedeutung, die dieser Baum dem Wert nicht gibt.

**Hier ist der Wert tragend und nicht kosmetisch, und das gab den Ausschlag
für die Codeänderung.** `nach_dem_sichtbarkeitswechsel` (`:4024`) legt die
Fensterzeile **nicht** neu aus; es zieht die Dateisystemwache nach, rettet den
Fokus aus einem ausgeblendeten Randbereich und trägt die Vorschau nach. Ein
hervorgeholtes Dateifenster bekommt seinen Nachzug allein über
`kommando_ausfuehren`. Damit ist der neue Zweig „war ausgeblendet, steht jetzt
da, Ordner stimmt schon" ein Zweig, der `true` liefern **muss**, und die
beiden, in denen nichts geschah, liefern `false`:

- zu schmales Fenster: `false`
- sichtbares Ziel auf demselben Ordner: `false`
- hervorgeholt, Ordner stimmte schon: `true`
- gestellt, mit oder ohne Hervorholen: `true`

**Geprüft, dass der Wert nur einen Abnehmer hat.** `ordner_angleichen` wird an
genau einer Stelle gerufen (`anwendung.rs:3077`), im `match`, das `gewirkt`
bildet. Ein `false` erreicht AppKit nicht: `kommando_ausfuehren` liefert am Ende
unbedingt `true`.

**Was nicht mitgezogen wurde.** `ordner_der_datei_zeigen` liefert auf seinem
Leerweg weiter `true`, und dieselbe Fossilie steht nach dieser Änderung noch an
sechs Stellen der Datei (`grep -n "war zustaendig"`: `:1773`, `:1861`, `:1880`,
`:2945`, `:2964`, `:3442`). `terminal_oeffnen` (`:1861`) und
`weitere_instanz_starten` (`:1880`) begründen ihr `true` sogar mit einem Satz,
den Runde 7 überholt hat („ein `false` gäbe den Tastendruck an AppKit weiter") —
`kommando_ausfuehren` liefert seit damals unbedingt `true`, und der innere Wert
erreicht AppKit gar nicht. Alle sechs nachzuziehen ist der größere Umbau aus
Möglichkeit 2 des Defektsatzes und liegt außerhalb dieser Runde. Der Doc-Kommentar von `ordner_angleichen` nennt die Abweichung
ausdrücklich, damit der Baum nicht zwei Lesarten eines Wertes stillschweigend
nebeneinander trägt.

## Was unverändert blieb

Alles, was die Durchsicht als richtig bestätigt hat:

- Die Sichtbarkeit steht in ihrem eigenen `let`. Die kompakte `if`-Form stürzte
  ab, weil ein Temporary in einer `if`-Bedingung bis zum Ende der ganzen
  Bedingung lebt und `bereich_einblenden` über `sichtbarkeit_aendern` an
  `borrow_mut()` greift. Der Kommentar dazu steht unverändert.
- Die drei Bedeutungen des `false` von `bereich_einblenden` bleiben getrennt;
  nur die echte Abweisung meldet.
- Die Meldung geht an das auslösende Dateifenster.
- `die_sichtbarkeit_wird_vor_dem_einblenden_gefragt` misst weiterhin, was sie
  soll: `sichtbar(bereich)` steht vor `bereich_einblenden(bereich)`.

## Abnahme

`make check` — Beendigungsstatus `0`. Bau, Proben, Formatierung und Clippy
unter `-D warnings` grün, ohne eine einzige Warnung. Die drei Angleichproben
laufen mit.

Vor dem Lauf geprüft, dass weder `/tmp` noch `$TMPDIR` eine
`krk-messplan-*.toml` hält: kein Treffer, also hat der Lauf keinem Messlauf den
Plan weggeräumt.

## Eine Nebenwirkung, die zu melden ist

Der erste Formatierungslauf ging als `cargo fmt --all` über den ganzen
Workspace und hat damit möglicherweise `crates/krk-ui/src/appkit/zwischenablage.rs`
mitformatiert, die Datei eines gleichzeitig arbeitenden Agenten. Ob sie
tatsächlich angefasst wurde, ist im Nachhinein nicht mehr feststellbar; der
Inhalt jener Datei wurde nicht geändert, und die kanonische Form ist die, die
deren eigenes `make check` ohnehin verlangt. Für die weiteren Läufe wurde nur
noch `cargo fmt --all --check` verwendet, das nichts schreibt.
