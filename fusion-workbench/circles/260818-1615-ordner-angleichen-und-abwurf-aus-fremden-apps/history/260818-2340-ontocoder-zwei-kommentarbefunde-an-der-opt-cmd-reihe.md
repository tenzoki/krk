# Ontocoder — Zwei Kommentarbefunde an der opt+cmd-Reihe

**Datum:** 260818-2340
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Befunde:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/issues/260818-2131_o_the-keymap-now-carries-two-enumerations-of-the-opt-cmd-row-and-the-older-one-is-short-by-two.md` und `…/issues/260818-2132_o_the-letter-choice-cites-the-third-rule-without-recording-that-the-first-two-were-checked.md`
**Baumstand beim Beginn:** `8d5baf6`. `resources/default-keymap.toml` unverändert; im Arbeitsbaum lagen zwei Umbenennungen von Entscheidungsdatensätzen aus einem früheren Auftrag dieser Sitzung. Kein zweiter Agent lief.

## Was der Auftrag war

Zwei Befunde der Durchsicht beheben, beide reine Kommentararbeit an
`resources/default-keymap.toml`. Kein `[[funktion]]`-Block, keine
`tasten`-Zeile, keine Kombination durfte sich ändern, und keine andere Datei.

## Die Reihe selbst nachgezählt

Vor jeder Änderung gegen die Datei gezählt, nicht gegen die Befunde:

```
grep '^tasten' resources/default-keymap.toml | grep -o 'opt+cmd+[a-z]*' | sort -u
```

Elf Kombinationen: opt+cmd+b, +c, +d, +e, +g, +l, +left, +n, +o, +right, +s.
Geprüft, dass keine `tasten`-Zeile eingerückt steht und `opt+cmd` außerhalb von
Kommentaren und `tasten`-Zeilen nirgends vorkommt; beide Gegenproben leer. Die
ältere Aufzählung nannte acht, die neuere zehn.

## Befund 1: die zwei Aufzählungen

**Entschieden: es überlebt keine von beiden.** Nicht "eine behalten, die andere
verweisen lässt", sondern beide weg.

Der Grund steht in derselben Datei, fünfzehn Zeilen über der neueren
Aufzählung und siebenhundert darunter. Dieselbe Prüfung ist hier schon zweimal
festgehalten worden, und beide Male ohne Liste:

- `:275` für opt+cmd+o: „ab Werk frei; am 260812 nachgezaehlt, keine
  Tastenliste dieser Datei nennt es."
- `:1016` für opt+cmd+n: „Nachgezaehlt am 260813 ueber alle Tastenlisten dieser
  Datei … keine davon nennt opt+cmd+n."

Beide halten den **Befund** fest und nicht die Menge, gegen die er erhoben
wurde. Die Runde 13 ist von dieser Form abgewichen und hat die Liste
ausgeschrieben; damit stand sie neben der älteren im Kopf der Reihe. Die
Aufzählung war also nicht die Lösung eines Problems, sondern selbst die
Neuerung.

Dazu kommt die Eigenschaft der Menge: die opt+cmd-Reihe wächst mit fast jeder
Runde, und CLAUDE.md legt für genau diesen Fall handgeführte Zahlen ab, statt
sie zu pflegen — die Variantenzahl von `Kommando`, die Quote der
Untergrenzen-Abschnitte. Eine Aufzählung über eine wachsende Menge geht mit der
nächsten Ergänzung wieder falsch, gleich wie viele Kopien es gibt. Eine Kopie
ist nur seltener falsch als zwei.

`48bb57f` („der Kopf des Norton-Blocks verspricht nur noch, was der Block
haelt") behielt eine Zahl, und das ist kein Gegenbeispiel: die Norton-Reihe hat
sechs Einträge und wächst nicht.

**Geändert:**

- `:266-272` (der Kommentar bei `ordner_der_datei`): Der Vorsatz „Die
  opt+cmd-Reihe traegt …, was einen Ordner herstellt oder liefert" ist gefallen.
  Er war für opt+cmd+e und opt+cmd+n falsch, und das ist genau das Versprechen,
  das `48bb57f` aus dem Norton-Block genommen hat. An seiner Stelle steht, dass
  die Reihe keine einzige Familie ist: Ordnerbefehle, die Umschaltfamilie, und
  Einzelne, die zu keiner von beiden gehören. Keine Zahl davor, weil auch die
  Zahl der Sorten wachsen kann. Der Nachbarschaftsgrund für diesen Eintrag
  (unmittelbar neben opt+cmd+c) steht unverändert.
- Neu an derselben Stelle: der `grep`, der die Reihe aus der Datei liest, dazu
  ein Vermerk, was hier bis zum 260818 stand und warum es weg ist. Der Vermerk
  steht einmal und nur hier.
- `:293-296` (der Kommentar bei `ordner_angleichen`): Die Zehnerliste ist
  gefallen. Der Freiheitsbefund bleibt und trägt jetzt die Form seiner beiden
  Nachbarn, die er ausdrücklich nennt.

## Befund 2: die drei Wahlregeln

Die drei Regeln stehen in
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`
und sind gelesen worden, nicht zitiert.

**Regel 1** („wo der Mac für dieselbe Sache ein Kürzel kennt"): Der Finder hat
kein zweites Dateifenster. Die Sache gibt es dort nicht, also auch kein Kürzel
dafür. Regel 1 geht leer aus.

**Regel 2** („wo Norton Commander oder Total Commander eine Form haben, die auf
dem Mac frei ist"): Total Commander erreicht die Sache über ctrl+left und
ctrl+right. Frei ist das Paar auf dem Mac nicht, und zwar zweifach: macOS
schaltet damit zwischen den Schreibtischen um, und diese Datei vergibt es an
`bereich_verbreitern` (`:604`) und `bereich_verschmaelern` (`:609`). Der
zweite Halbsatz ist im Baum nachgelesen, der erste nicht.

Regel 2 fragt nach der **Form** und nicht nach einer mac-tauglichen Abwandlung;
ihre drei Beispiele — `tab`, `space`, `shift+f6` — sind unveränderte
Übernahmen. `ctrl+cmd+left`/`ctrl+cmd+right` aus der Liste des Shapers wäre
eine Abwandlung und fällt damit nicht unter Regel 2. **Regel 2 geht leer aus,
Regel 3 entscheidet, und opt+cmd+s bleibt unangetastet.** Der Fall „Regel 2
sticht und die Kombination ist wieder offen", den der Befund als Abbruchgrund
vorgesehen hat, ist nicht eingetreten.

Das steht jetzt ausgeschrieben im Kommentar, mitsamt der Angabe, woher die
Total-Commander-Form stammt: aus der dokumentierten Belegung jenes Programms
und nicht aus einer Messung an einem laufenden Total Commander. Für Norton
Commander ist keine Form benannt, weil keine zu belegen war.

**Die zweite Lesart ist eingeordnet, nicht getilgt.** „s liest sich als selber
Ordner" aus dem Spec steht jetzt als Merkhilfe im Kommentar, ausdrücklich nicht
als Grund. Gewählt hat den Buchstaben Regel 3 über das Verb „stellen"; der
bindende Datensatz lässt keine zweite Wahlbegründung zu.

## Prüfung

```
make check   →  Exit 0   („alle vier gruen")
```

Vor dem Lauf geprüft, dass weder `/tmp` noch `$TMPDIR` eine
`krk-messplan-*.toml` hält; beide leer.

Kein lokaler TOML-Leser stand zur Verfügung (`python3` ist 3.9.6, ohne
`tomllib` und ohne `toml`). Das ist kein Loch: die Datei geht über
`include_str!` in `krk_core::tasten::belegung::AUSLIEFERUNGSTEXT` (`:159`), und
`crates/krk-core/tests/belegung.rs` liest sie mit 98 Proben. Darunter
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`, die die
Kopfzeile liest und selbst nachzählt — grün heißt, dass die 85 Funktionen und
90 Kombinationen im Kopf weiter mit dem Inhalt übereinstimmen, also von dieser
Arbeit nichts angefasst wurde.

Der Diff ist gegengelesen: jede geänderte Zeile beginnt mit `#`. Die Gegenprobe

```
git diff -U0 resources/default-keymap.toml | grep -E '^[-+]' | grep -vE '^(\+\+\+|---)' | grep -vE '^[-+]#'
```

liefert nichts. Genau eine Datei ist geändert.

## Was offen bleibt

- Die Befundsätze sind **nicht** geschlossen und der Stand ist **nicht**
  eingecheckt; beides macht der Nutzer.
- `:791` (bei `editor_schliessen`) zählt die Umschaltfamilie auf: opt+cmd+l, +d,
  +b, +left, +right. Das ist keine dritte Aufzählung der Reihe, sondern eine der
  Familie, sie ist vollständig und richtig, und sie trägt dort die Begründung
  des Buchstabens. Nicht angefasst, außerhalb des Auftrags. Wächst die
  Umschaltfamilie, hat sie dieselbe Eigenschaft wie die beiden getilgten Listen.
- Die Total-Commander-Form ist die eine Aussage dieser Arbeit, die nicht am Baum
  und nicht an einer zitierbaren Quelle des Projekts hängt. Der Kommentar sagt
  das an Ort und Stelle.
