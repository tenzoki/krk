# S13b: Menükürzel in die Auslieferungsbelegung

**Agent:** ontocoder
**Datum:** 260805-0637
**Status:** Complete, mit einem Vorbehalt — vier der fünf Einträge stehen, der fünfte ist auf eine Nutzerfrage blockiert
**Auftrag:** Schritt 13b aus `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`
**Nicht committet** (Auftrag)

---

## Was getan wurde

`resources/default-keymap.toml` und `resources/Info.plist`. Keine andere Datei
angefasst; `crates/`, `xtask/`, Plan und Spec bleiben unberührt.

### Vier von fünf Einträgen

| `id` | `tasten` | `gehalten_von` | Abschnitt |
|---|---|---|---|
| `fenster_schliessen` | `["shift+cmd+w"]` | — | C7, direkt hinter `fenster_einblenden` |
| `text_ausschneiden` | `["cmd+x"]` | `menue` | neuer Abschnitt am Dateiende |
| `text_kopieren` | `["cmd+c"]` | `menue` | ebenda |
| `text_einfuegen` | `["cmd+v"]` | `menue` | ebenda |

`text_alles_auswaehlen` auf `cmd+a` **fehlt**, siehe unten. Die Datei zählt
damit 54 Funktionen und 61 Kombinationen statt der zugesagten 55 und 62.

### Wo die neuen Einträge stehen

Zwei Orte statt eines, weil die Gliederung der Datei nach Fähigkeiten führt und
die fünf Einträge nicht einer Fähigkeit angehören.

`fenster_schliessen` steht im vorhandenen Abschnitt `C7: Sichtbarkeit und
Breiten der Bereiche`, unmittelbar hinter `fenster_einblenden`. Der Plan nennt
diesen Eintrag ausdrücklich als sein Vorbild, C7 trägt das Abnahmekriterium zu
Shift+Cmd+W, und die beiden sind Gegenstücke. Ein eigener Abschnitt für einen
Eintrag, der in einen vorhandenen passt, wäre eine Ausnahme ohne Gegenwert.

Die drei Textbefehle bekommen einen neuen Abschnitt `C2: die Textbefehle des
Menüs "Bearbeiten"` **am Ende der Datei**, hinter der Belegungsansicht. Der
Grund für das Ende und nicht für einen Platz bei den übrigen C2-Abschnitten:
sie sind die ersten Einträge der Datei, die der Ereignisabgriff nicht
ausführt. Zwischen `markierung_umkehren` und `sortierung_name` gestellt läsen
sie sich wie Funktionen des Dateifensters, und der Kopfkommentar des
Abschnitts, der die Zustellung über die Antwortkette erklärt, stünde mitten in
der Navigation. Als geschlossener Block am Ende ist die neue Art von Eintrag an
einer Stelle erklärt.

### Vier Änderungen am Kopfkommentar, wie vom Plan verlangt

1. **Aufbau je Eintrag** führt `gehalten_von` mit auf, samt der Abgrenzung, die
   der Plan verlangt: eine Funktion mit `gehalten_von = "menue"` bekommt nie
   ein Kommando, eine Funktion ohne das Feld und ohne Kommando wartet nur auf
   den Schritt, der ihres baut (`belegung_ansehen` bis S20).
2. **Der Absatz über die ab Werk freien Kombinationen** sagt jetzt zwei statt
   vier. `shift+delete` und `return` bleiben, `cmd+c` und `cmd+v` fallen
   heraus. Ein zweiter Absatz daneben hält fest, dass die Reservierung aus C3
   damit eingelöst und nicht gebrochen ist.
3. **Ein neuer Absatz zum Fokusvorbehalt**: eine Kombination kann im Textfeld
   etwas anderes bedeuten als im Dateifenster, mit `cmd+left` als vorhandenem
   Beispiel.
4. **Der Abschnitt für die neuen Einträge** in der Gliederung nach Fähigkeiten,
   siehe oben.

### Eine fünfte Änderung, die der Plan nicht nennt

Der Kopf des C10-Abschnitts schloss mit "Cmd+C und Cmd+V bleiben davon
unberuehrt und ab Werk frei". Die zweite Hälfte des Satzes wurde durch die
neuen Einträge falsch. Nachgezogen auf die Formulierung, die Spec C10 seit dem
260805-0000 selbst führt: von C10 unberührt, ab Werk nicht mehr frei, und genau
dadurch für die spätere Runde frei gehalten. Eine Aussage stehen zu lassen, die
die eigene Änderung widerlegt, wäre ein Widerspruch in derselben Datei.

### `Info.plist`

`NSDisabledCharacterPaletteMenuItem` und `NSDisabledDictationMenuItem`, beide
`true`, hinter `NSHighResolutionCapable` und vor dem Block der
TCC-Rückfragetexte. Der Kommentar darüber trägt die Begründung und die
Kennzeichnung `inference:` des Plans mit: gemessen ist der gleichgelagerte Fall
am Fenstermenü, nicht dieser. Der Platzhalter `__KRK_VERSION__` ist unberührt.

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `grep -c '^\[\[funktion\]\]'` | **54**, zugesagt sind 55 |
| Neue Blöcke | 4 statt 5, davon 3 mit `gehalten_von = "menue"` |
| Tastenliste eines vorhandenen Blocks geändert | keine (`git diff -U0 \| grep '^-tasten = '` ist leer) |
| Kombination bei zwei Funktionen, am vollständigen Eintrag geprüft | keine, über alle 54 Funktionen und 61 Kombinationen |
| `grep -F '"return"'`, `grep -F '"shift+delete"'` | beide ohne Treffer |
| `cargo test -p krk-core --test belegung` | **FAILED**, 3 von 26 bestanden |
| `plutil -lint resources/Info.plist` | OK |
| `plutil -extract NSDisabledCharacterPaletteMenuItem raw` | `true` |
| `plutil -extract NSDisabledDictationMenuItem raw` | `true` |
| `plutil -extract CFBundleShortVersionString raw` | `__KRK_VERSION__`, unberührt |

Die Prüfung auf doppelte Kombinationen lief am vollständigen Eintrag über einen
geparsten Aufbau der Datei und nicht als Teilzeichenkette. Die beiden Fälle,
vor denen der Auftrag warnt, sind mit diesem Schritt echt betroffen und beide
sauber: `shift+cmd+v` (Verschieben) enthält `cmd+v`, `cmd+right` (Einsteigen)
enthält `cmd+r`. Eine Teilzeichenkettensuche hätte hier zwei Fehlalarme
geliefert.

## Der Test ist rot, und warum das zwei verschiedene Dinge sind

`cargo test -p krk-core --test belegung` endet nicht mit 0. Zwei Ursachen,
beide gemeldet:

**Der Parser weist `gehalten_von` ab.** `Eintrag` in
`crates/krk-core/src/tasten/belegung.rs:662-670` trägt
`#[serde(deny_unknown_fields)]`. Die Meldung im Wortlaut:

```
unknown field `gehalten_von`, expected one of `id`, `name`, `tasten`, `reserviert_fuer`
```

Sie kommt aus `belegung.rs:66`, wo `AUSLIEFERUNG` die eingebettete Datei über
`include_str!` liest und bei einem Lesefehler abbricht. Damit fallen 23 der 26
Prüfungen, nicht eine: jede, die `Belegung` anfasst. Die Meldung belegt zugleich,
dass die Datei **gültiges TOML** ist — der Parser kam bis zur Feldzuordnung, er
scheiterte nicht am Lexer. Der Auftrag hat diesen Ausgang vorweggenommen; die
Behebung gehört S13c, dessen Plantext das Feld bereits vorsieht.

**Die Prüfung `die_ab_werk_freien_kombinationen_kommen_nicht_vor` führt `cmd+c`
und `cmd+v` als frei.** Diese Zusage hat der Nutzerentscheid vom 260805-0000
abgelöst. Auch das gehört S13c, dessen Dateiliste
`crates/krk-core/tests/belegung.rs` als erweitert führt.

Gemeldet als
`issues/260805-0637_o_das-abnahmekriterium-von-s13b-verlangt-einen-gruenen-test-den-erst-s13c-gruen-macht.md`.

## Der Blocker: cmd+a

Der fünfte Eintrag ist nicht geschrieben. `cmd+a` gehört seit dem Anlegen der
Datei in S9 (Commit `d1a8ab1`) der Funktion `alle_markieren`; die Zeile hat nie
`ctrl+a` getragen. Der Plan hält in S13b das Gegenteil fest und stützt darauf
seine Zusage, alle fünf Kombinationen seien frei.

Ihn trotzdem zu schreiben hätte drei Zusagen zugleich gebrochen: das
Abnahmekriterium von S13b (keine Kombination bei zwei Funktionen), das
Abnahmekriterium von C3 (die Auslieferungsbelegung ist in sich konfliktfrei)
und, maschinell, `Belegung::bauen`, das bei einem Konflikt abbricht. `cmd+a`
umzuhängen hätte die andere Hälfte desselben Abnahmekriteriums gebrochen (keine
Änderung an der Tastenliste eines vorhandenen Blocks) und eine Belegung
angefasst, die der Nutzer am 260803-2110 angenommen hat.

Der Konflikt ist im Datenbestand nicht auflösbar, ohne einen Wert zu erfinden.
Drei Wege stehen offen, keiner davon ohne Preis; sie sind mit ihren Kosten in
`issues/260805-0637_o_cmd-a-liegt-schon-auf-alle-markieren-und-s13b-vergibt-es-ein-zweites-mal.md`
ausgeschrieben. Die Wahl braucht einen Entscheidungsdatensatz.

Die Lücke steht als Kommentar an der Stelle in der Datei, an der der Eintrag
hingehört, mit Verweis auf den Defekt. Sie ist damit weder still noch beim
Weiterarbeiten übersehbar.

## Geänderte Dateien

- `resources/default-keymap.toml`
- `resources/Info.plist`

## Angelegte Defekte

- `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-0637_o_cmd-a-liegt-schon-auf-alle-markieren-und-s13b-vergibt-es-ein-zweites-mal.md`
- `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-0637_o_das-abnahmekriterium-von-s13b-verlangt-einen-gruenen-test-den-erst-s13c-gruen-macht.md`

## Was offen bleibt

- Der Nutzer entscheidet über `cmd+a`. Danach ist S13b mit einem Eintrag und
  einer gestrichenen Kommentarzeile fertig.
- S13c bringt dem Parser `gehalten_von` bei und zieht die Prüfung der ab Werk
  freien Kombinationen nach. Bis dahin ist der Baum rot.
- Der Marker `_a_` von
  `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`
  bleibt stehen. Die Entscheidung ist erst mit S13c umgesetzt, und dieser
  Schritt hat nicht committet, also gibt es keinen Hash zu zitieren.
