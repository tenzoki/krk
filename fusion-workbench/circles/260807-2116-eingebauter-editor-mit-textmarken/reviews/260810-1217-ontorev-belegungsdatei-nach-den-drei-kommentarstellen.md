# Durchsicht: `resources/default-keymap.toml` nach den drei Kommentarstellen

**Durchgesehen von:** ontorev
**Am:** 260810-1217
**Gegenstand:** `git diff 38a02b2..HEAD -- resources/default-keymap.toml`, 61 geänderte Zeilen
**Circle:** `circles/260807-2116-eingebauter-editor-mit-textmarken`

---

## Zusammenfassung

Die geänderten Kommentartexte sind inhaltlich richtig, stimmen mit dem Code überein, den sie beschreiben, und widersprechen einander nicht. Keine Belegungszeile ist angefasst worden; das ist am Diff und an einem Vergleich der Datei ohne ihre Kommentare belegt. Die drei Befunde dieser Durchsicht stehen alle **neben** den geänderten Zeilen: zwei alte Kommentarbehauptungen der Datei stimmen nicht mit dem Bestand, und die zwei Zählstände im Dateikopf sind heute richtig, aber durch keine Probe gehalten.

## Zahlen

| Schwere | Anzahl |
|---------|--------|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 2 |

Kein Befund betrifft eine geänderte Zeile. Kein Befund betrifft eine Belegungszeile.

---

## 1. Keine Belegungszeile ist geändert — bestätigt

Der Diff berührt ausschließlich Zeilen, die mit `#` beginnen. Das ist nicht nur am Diff gelesen, sondern gegengeprüft: die Datei ohne ihre Kommentar- und Leerzeilen ist vor und nach der Änderung Byte für Byte dieselbe.

```sh
git show 38a02b2:resources/default-keymap.toml | grep -vE '^\s*#' | grep -vE '^\s*$' > alt.txt
grep -vE '^\s*#' resources/default-keymap.toml | grep -vE '^\s*$' > neu.txt
diff alt.txt neu.txt          # kein Unterschied
wc -l < neu.txt               # 290
```

290 Nutzzeilen, unverändert. Die Behauptung hält.

---

## 2. Die drei Kommentartexte gegen den Code

### Der Dateikopf, Zeile 42

> Funktionstasten schlaegt KRK ueber den Tastencode nach, und F3 mit gehaltener fn-Taste erzeugt denselben Tastencode wie ein nacktes F3

Richtig, und die Schlussfolgerung trägt weiter. `Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) legt einen einbuchstabigen ASCII-Namen auf `Tastenkennung::Zeichen` und **jeden anderen** Namen auf `Tastenkennung::Code`. Der Name `f3` ist zwei Zeichen lang und fällt damit auf `Code`. Der Absatz über die fn-Taste und F3 spricht also über genau die Sorte, die noch über den Tastencode geht, und die Ableitung aus `spikes/fn-tasten/messung-A.txt` bleibt gültig.

Der Kopf ist damit **enger** als die tatsächliche Regel: über den Tastencode gehen außer den Funktionstasten auch der Pfeilblock und die Steuertasten. Das ist kein Widerspruch, sondern die für diesen Absatz nötige Teilaussage. Der Absatz braucht nur F3, und er behauptet nichts über die Buchstaben.

### Der Editor-Block, Zeilen 484-499

> Ein einbuchstabiger Tastenname in dieser Datei benennt die **Aufschrift** und keine Stelle auf der Tastatur: Buchstaben und Ziffern werden ueber das gemeldete Zeichen nachgeschlagen, alles uebrige ueber den virtuellen Tastencode.

Richtig und deckungsgleich mit `Taste::kennung` (`parser.rs:192-198`), einschließlich der Einschränkung auf ASCII-Kleinbuchstaben und ASCII-Ziffern. Die Stellensuche in `Kombination::aus_tastendruck` (`parser.rs:569-576`) filtert jede Taste mit Zeichenkennung aus, wie der Text es voraussetzt.

Die drei Verweise stimmen alle:

- Der Abschnitt `# Zwei Nachschlagarten, und warum es zwei sein muessen` steht in `parser.rs:18`.
- Der Entscheidungsdatensatz `260808-0140_i_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md` liegt im aktiven Circle, trägt `**Status:** answered` und den Marker `_i_` (umgesetzt). Der Text nennt ihn zu Recht einen Nutzerentscheid.
- Das Datum 260808-0155 im Kommentar deckt sich mit der Zeile `Answered:` des Datensatzes: „Entschieden vom Nutzer am 260808-0155."

Der Satz „kein Eintrag wandert mit der Tastaturbelegung, und keiner meidet einen Buchstaben" ist ausdrücklich auf „die Eintraege hier" beschränkt, also auf die elf Editor-Funktionen. Die benutzen `e`, `s`, `f`, `g`, `j` und `r`, alle über das Zeichen nachgeschlagen. Für diese Menge hält die Aussage ohne Vorbehalt.

### Der Rückgängig-Block, Zeilen 622-643

> Die beiden sind die einzigen Kombinationen dieser Runde auf dem Buchstaben z, und das ist unauffaellig: einen Buchstaben schlagen **beide** Zusteller ueber das **Zeichen** nach.

Beides geprüft und richtig.

Der Buchstabe `z` steht in der ganzen Datei in genau zwei Tastenlisten, `cmd+z` (Zeile 676) und `shift+cmd+z` (Zeile 685), beide mit `gehalten_von = "menue"`. Der Buchstabe `y` steht in keiner.

Die beiden Codeverweise stimmen: `zeichen_der_taste` steht in `crates/krk-ui/src/appkit/menue.rs:405`, `Kombination::aus_tastendruck` in `crates/krk-core/src/tasten/parser.rs:569`. Beide gehen für einen Buchstaben über das Zeichen, und damit ist die Begründung nicht mehr „nur das Menü schlägt über das Zeichen nach", sondern „beide tun es". Genau das sagt der Text.

### Sagen die drei dasselbe?

Ja, mit einer Abstufung, die keinen Widerspruch trägt:

```
Dateikopf, Z. 42     Funktionstasten          → Tastencode
Editor-Block, Z. 485 Buchstaben und Ziffern   → Zeichen
                     alles uebrige            → Tastencode
Rueckgaengig, Z. 623 Buchstaben, beide Zusteller → Zeichen
```

Der Editor-Block trägt die vollständige Regel, der Dateikopf ihre eine Hälfte für den Fall F3, der Rückgängig-Block ihre andere Hälfte für den Fall `cmd+z` und ergänzt sie um den zweiten Zusteller. Keine der drei Stellen sagt mehr, als der Code belegt.

### Kleinigkeit ohne Defektdatensatz

Im Rückgängig-Block hängt der Schlusssatz „Was hier steht, ist die Anzeige- und Konfliktseite dieser Kuerzel und nicht der Nachschlag, der sie ausloest" ohne Leerzeile am historischen Absatz „Bis zum 260810 …". Er gehört der Sache nach zum ganzen Block und nicht zur Fußnote über den alten Stand. Eine leere Kommentarzeile davor stellte das her. Das ist Absatzform, kein Sachfehler, und darum steht es hier und nicht in einem Defektdatensatz.

---

## 3. Die Zahlen im Dateikopf

`resources/default-keymap.toml:30`:

> Ausgeliefert sind 71 Funktionen mit zusammen 79 Kombinationen.

**Beide Zahlen stimmen.** Nachgezählt am 260810-1217 über den Bestand der Datei: 71 `[[funktion]]`-Einträge, 79 Einträge über alle `tasten`-Listen zusammen. Kein Eintrag ohne `tasten`, keine `id` doppelt.

Die vier Aufzählungen, die die Editor-Runde in `crates/` erweitert hat, sind hier ohne Wirkung geblieben, weil keine von ihnen in dieser Datei gezählt wird. Der Dateikopf zählt Einträge, nicht Varianten.

Was fehlt, ist die Sicherung. Die Zahl steht in einem Kommentar, und ein Kommentar hält keinen Bau an. Siehe Befund 3 unten.

---

## 4. Die innere Geschlossenheit der Datei

### Jeder Eintrag trägt ein Kommando, das die Aufzählung kennt

Geprüft über alle 71 Einträge gegen `Kommando::KENNUNGEN` (`crates/krk-core/src/tasten/belegung.rs:493 ff.`):

| Sorte | Anzahl |
|-------|--------|
| trägt ein Kommando aus der Aufzählung | 65 |
| `gehalten_von = "menue"`, trägt nach `Funktion::kommando` nie eines | 6 |
| `reserviert_fuer` gesetzt | 0 |
| benannt, nicht zugestellt, ohne Kommando | 0 |

Die 65 Kennungen sind genau die 65 Varianten der Aufzählung `Kommando`, in beide Richtungen ohne Rest: keine Kennung ohne Eintrag, kein Eintrag ohne Kennung. Die sechs zugestellten sind `text_ausschneiden`, `text_kopieren`, `text_einfuegen`, `text_alles_auswaehlen`, `text_rueckgaengig` und `text_wiederholen`; `Funktion::kommando` (`belegung.rs:712-717`) gibt für sie vor dem Nachschlag `None` zurück, wie der Dateikopf es zusagt. Keiner der sechs steht zugleich in `KENNUNGEN`, die Zusage ist also nicht nur eingehalten, sondern auch nicht bloß zufällig erfüllt.

Die vorhandene Probe `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (`belegung.rs`) sichert die eine Richtung. `cargo test -p krk-core --lib tasten` läuft durch: 21 Proben, keine gescheitert.

### Keine Tastenkombination doppelt vergeben

Über alle 79 Kombinationen gibt es genau **eine** Doppelung:

```
cmd+a  →  alle_markieren        (Ereignisabgriff)
       →  text_alles_auswaehlen (gehalten_von = "menue")
```

Das ist der eine Fall, den der Dateikopf in Zeile 83-88 als den einzigen benennt, mit dem Nutzerentscheid vom 260805 dahinter. Verschiedene Zusteller, damit nach der Konfliktdefinition des Kopfes kein Konflikt. Kein weiterer Fall, und keine Doppelung bei gleichem Zusteller.

---

## Befunde

### Befund 1 — Medium: der Editor-Abschnitt zählt fünf e-Tasten und hat vier

`resources/default-keymap.toml:479` behauptet, fünf der elf Editor-Funktionen teilten sich den Buchstaben `e`. Es sind vier: `cmd+e`, `shift+cmd+e`, `opt+cmd+e`, `ctrl+cmd+e`. Der Absatz nennt selbst vier Ebenen der Systematik, und die vier Funktionen belegen sie genau. Eine fünfte Kombination auf `e` gibt es in der ganzen Datei nicht.

Die Zahl war nie richtig: `git log -L 479,482:resources/default-keymap.toml` liefert genau einen Treffer, `e1acc68`, und schon dort tragen vier Kombinationen den Buchstaben `e`.

Nicht in den geänderten Zeilen, sondern zwei Zeilen darüber.

Datensatz: `issues/260810-1217_o_der-editor-abschnitt-der-belegung-zaehlt-fuenf-e-tasten-und-hat-vier.md`

### Befund 2 — Low: der Dateikopf nennt `belegung_ansehen` als Funktion ohne Kommando

`resources/default-keymap.toml:25-26` begründet, warum das Feld `gehalten_von` existiert, und stützt sich dafür auf eine Funktion, „deren Kommando ein spaeterer Schritt erst baut, wie es `belegung_ansehen` unten ist". Dieser Schritt ist gebaut: `Kommando::BelegungAnsehen` steht in `belegung.rs:409`, die Kennung in `belegung.rs:493`. Von dieser Sorte gibt es in der Datei keinen Eintrag mehr (0 von 71).

Datensatz: `issues/260810-1218_o_der-dateikopf-der-belegung-nennt-belegung-ansehen-als-funktion-ohne-kommando.md`

### Befund 3 — Low: die zwei Zahlen im Dateikopf wachsen nicht mit der Datei

Die Zahlen 71 und 79 in Zeile 30 stimmen heute. Nichts hält sie: sie stehen in einem Kommentar, und ein hinzugefügter Eintrag löst keinen Hinweis aus. Die beiden anderen Befunde dieser Durchsicht sind aus derselben Lücke entstanden. Vorgeschlagen ist eine Probe neben den bestehenden in `belegung.rs`, die beide Zählstände an der eingebetteten Belegung festnagelt und im Kommentar auf Zeile 30 verweist.

Datensatz: `issues/260810-1219_o_die-zwei-zahlen-im-kopf-der-belegungsdatei-wachsen-nicht-mit-ihr.md`

---

## Empfohlene Reihenfolge

1. **Befund 1** zuerst, weil er eine falsche Zahl über die Systematik der Editor-Belegung trägt und jeden irreleitet, der die Belegung erweitert. Eine Wortänderung.
2. **Befund 3** danach, weil die Probe die Stelle absichert, an der Befund 1 und Befund 2 entstanden sind. Sie fasst die Datei nicht an.
3. **Befund 2** zuletzt. Er beschädigt nichts; er lässt eine Begründung ohne lebendes Beispiel stehen.

Alle drei sind unabhängig voneinander. Befund 1 und Befund 2 gehören `ontocoder` (Kommentarzeilen in einer TOML-Datei), Befund 3 gehört `coder` (eine Probe in `crates/krk-core`).

---

## Was diese Durchsicht nicht geprüft hat

- **Die Zeitzusagen und die Geschwindigkeit.** Keine Zeile dieses Diffs betrifft Laufzeit.
- **Ob die Kombinationen am laufenden Bündel auslösen.** Das verlangt KRK im Vordergrund und ist Nutzerarbeit; der Grund steht in `CLAUDE.md` unter „Was man nicht sieht".
- **Die Belegung auf einer nicht-deutschen und nicht-amerikanischen Tastatur.** Der Zeichennachschlag macht die Aussage plausibel, dass keine Kombination wandert, aber gemessen ist sie an keinem anderen Layout. `inference:`, nicht geprüft.
- **Die übrigen Kommentarblöcke der Datei außerhalb der drei genannten Stellen** sind auf Behauptungen über den Nachschlag durchsucht (`grep` auf „Tastencode", „Stelle", „Zeichen", „Aufschrift", „kVK") und tragen keine weitere. Sie sind nicht Satz für Satz gegen den Code gelesen.
