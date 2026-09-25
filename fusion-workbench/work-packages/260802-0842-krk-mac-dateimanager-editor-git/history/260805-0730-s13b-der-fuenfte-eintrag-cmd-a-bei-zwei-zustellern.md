# S13b: der fünfte Eintrag, cmd+a bei zwei Zustellern

**Agent:** ontocoder
**Datum:** 260805-0730
**Status:** Complete
**Auftrag:** den offengelassenen fünften Eintrag von Schritt 13b nachtragen, nachdem der Nutzer über `cmd+a` entschieden hat
**Vorlauf:** `history/260805-0637-s13b-menuekuerzel-in-die-auslieferungsbelegung.md`
**Nicht committet** (Auftrag)

---

## Was getan wurde

Eine Datei: `resources/default-keymap.toml`. `resources/Info.plist` war mit dem
ersten Durchgang fertig und ist committet; `crates/`, `xtask/`, Plan und Spec
sind unberührt.

### Der fünfte Eintrag

| `id` | `name` | `tasten` | `gehalten_von` |
|---|---|---|---|
| `text_alles_auswaehlen` | Alles auswählen | `["cmd+a"]` | `menue` |

Er steht am Dateiende im Abschnitt `C2: die Textbefehle des Menüs "Bearbeiten"`,
hinter `text_einfuegen`, also in der Reihenfolge, in der das Menü "Bearbeiten"
die vier Befehle führt. Der Kommentar, der beim ersten Durchgang an dieser
Stelle auf den Defekt verwies, ist ihm gewichen.

`alle_markieren` ist nicht angefasst und trägt `cmd+a` weiter als einzige
Kombination (`resources/default-keymap.toml:221-223`); der neue Block steht auf
`resources/default-keymap.toml:474-478`.

### Die Grundlage

Nutzerentscheid vom 260805,
`decisions/260805-0713_a_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`:
zwei Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination
tragen und denselben Zusteller haben. Der Zusteller steht in `gehalten_von`;
ohne das Feld stellt der Ereignisabgriff aus S7 zu, mit dem Wert `menue` das
Hauptmenü. Der Fokusvorbehalt aus S13 teilt jeden Tastendruck genau einem der
beiden zu, und deshalb begegnen sich `alle_markieren` und
`text_alles_auswaehlen` nie.

Der Plantext von S13b trägt die Regel seit dem Nachzug selbst; dieser Schritt
hat sie nicht erfunden und den Plan nicht angefasst.

### Zwei Nachträge im Kopfkommentar

Der Auftrag ließ offen, ob der Absatz über `cmd+a` noch etwas braucht. Er
brauchte etwas, und zwar an zwei Stellen:

1. **Ein neuer Absatz hinter dem Fokusvorbehalt.** Dass eine Kombination bei
   zwei Funktionen stehen darf, ist neu in dieser Datei und widerspricht dem,
   was ein Leser der Konflikterkennung unterstellt. Der Absatz steht bewusst
   direkt hinter dem Fokusvorbehalt und beginnt mit "Daraus folgt": die
   Doppelung ist keine zweite Regel neben ihm, sondern seine Folge. Er nennt die
   Regel in ihrer scharfen Form, hält fest, dass zwei vom Menü gehaltene
   Funktionen auf einer Kombination sehr wohl ein Konflikt bleiben, und weist
   `cmd+a` als den einen ausgelieferten Fall aus, mit Verweis auf den
   Entscheidungsdatensatz.
2. **Eine Zeile bei der Ein-Zeilen-Regel:** "Ausgeliefert sind 55 Funktionen mit
   zusammen 62 Kombinationen." Der Plan verlangt die Zahl, und sie hatte bis
   hierhin keinen Ort in der Datei; der alte Kommentar an der Fehlstelle trug
   sie mit dem falschen Wert 54.

Dazu der Kommentar über dem neuen Block selbst, der die Doppelung dort erklärt,
wo ein Leser sie trifft, ohne die Regel ein zweites Mal auszuschreiben.

Damit sind die vier Änderungen des ersten Durchgangs, die fünfte selbst
gefundene am C10-Kopf und diese beiden zusammen sieben Stellen im Kopfkommentar.

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `grep -c '^\[\[funktion\]\]'` | **55** |
| Kombinationen gesamt | **62** |
| Neue Blöcke gegenüber HEAD | **1** |
| Neue Blöcke gegenüber HEAD~1 | **5** (HEAD zählt 54, HEAD~1 zählt 50) |
| Geänderte Tastenliste eines vorhandenen Blocks | keine (`git diff -U0 \| grep '^-tasten = '` ist leer) |
| `alle_markieren` | unverändert `tasten = ["cmd+a"]`, ohne `gehalten_von` |
| Kombination bei zwei Funktionen | genau eine, `cmd+a`, mit verschiedenen Zustellern |
| Kombination bei zwei Funktionen desselben Zustellers | keine |
| Doppelte `id` | keine |
| `grep -F '"return"'`, `grep -F '"shift+delete"'` | beide ohne Treffer |
| Gültiges TOML | ja, siehe unten |

Die Prüfung auf doppelte Kombinationen lief über einen geparsten Aufbau der
Datei, am vollständigen Eintrag und nicht als Teilzeichenkette: 55 Blöcke, 62
Kombinationen, 61 verschiedene. Die beiden Fallen, vor denen der Auftrag warnt,
sind sauber: `shift+cmd+v` (Verschieben) enthält `cmd+v`, `cmd+right`
(Einsteigen) enthält `cmd+r`. Eine Teilzeichenkettensuche hätte hier zwei
Fehlalarme geliefert und den echten Fall `cmd+a` unter ihnen begraben.

Beim zweiten vollständigen Durchgang durch die Datei ist **keine weitere**
Kombination aufgefallen, die bei zwei Funktionen steht. `cmd+a` bleibt die
einzige Doppelung im Auslieferungszustand.

### Gültiges TOML, obwohl der Testlauf rot ist

`cargo test -p krk-core --test belegung` endet weiter mit FAILED, 3 von 26
bestanden. Die Ursache ist unverändert die aus S13b bekannte und im Plantext
ausgeschriebene: `Eintrag` in
`crates/krk-core/src/tasten/belegung.rs:662-670` trägt
`#[serde(deny_unknown_fields)]` und kennt `gehalten_von` nicht.

```
unknown field `gehalten_von`, expected one of `id`, `name`, `tasten`, `reserviert_fuer`
```

Die Meldung ist zugleich der Nachweis, dass die Datei gültiges TOML ist: der
Parser kam bis zur Feldzuordnung und meldet einen Bereich (`span 17351..17363`)
auf das erste `gehalten_von`, er scheiterte nicht am Lexer. Das Abnahmekriterium
von S13b verlangt den grünen Lauf nicht mehr; der zugehörige Defekt
`260805-0637_c_das-abnahmekriterium-von-s13b-verlangt-einen-gruenen-test-den-erst-s13c-gruen-macht.md`
ist geschlossen, und dieser Schritt meldet ihn nicht ein zweites Mal.

Ein Parser für TOML steht in dieser Umgebung außerhalb des Rust-Baums nicht zur
Verfügung: `python3` ist 3.9.6, also ohne `tomllib`, und weder `toml` noch `yq`
sind installiert. Die Struktur wurde deshalb über einen Zeilenparser für den
Aufbau dieser Datei erhoben (`[[funktion]]`, `id`, `name`, `tasten`,
`reserviert_fuer`, `gehalten_von`, Kommentarzeilen ab Spaltenanfang). Er hat
keine Zeile ungeparst gelassen; die Vollständigkeit ist damit belegt und nicht
angenommen.

## Geänderte Dateien

- `resources/default-keymap.toml`

## Geschlossene Defekte

- `issues/260805-0637_c_cmd-a-liegt-schon-auf-alle-markieren-und-s13b-vergibt-es-ein-zweites-mal.md`
  (`_o_` → `_c_`, mit `Resolved:`). Der Defekt war die Meldung des ersten
  Durchgangs; ihn schließt der Nutzerentscheid samt seiner Umsetzung in den
  Daten.

Der `git mv` der Umbenennung hat die Änderung in den Index gestellt; sie ist
mit `git restore --staged` wieder herausgenommen. Der Index ist unberührt.

## Was offen bleibt

- **S13c** bringt `belegung.rs` den Zusteller bei, und zwar an drei Stellen:
  `Belegung::konflikte`, `Belegung::zuweisen` und `Belegung::nachschlag`. Die
  dritte ist die, ohne die die Regel nicht trägt: der Nachschlag darf nur
  Funktionen ohne `gehalten_von` sehen, sonst hinge das Verhalten an der
  Reihenfolge der Einträge in der Datei. Stünde `text_alles_auswaehlen` vor
  `alle_markieren`, wäre das Markieren aller Einträge still tot. Bis S13c ist
  der Baum rot.
- Der Marker `_a_` von
  `decisions/260805-0713_a_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`
  bleibt stehen. Der Datensatz führt `Implemented: <offen — S13c>`, und die
  Regel lebt erst mit dem Code aus S13c; dieser Schritt hat nur die Daten
  gestellt, auf die sie sich anwendet, und nicht committet.
- Dasselbe gilt für
  `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`,
  wie schon der erste Durchgang festhielt.
- Den `[DONE]`-Vermerk an S13b setzt der Auftraggeber, nicht dieser Schritt.
