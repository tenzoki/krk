# Lösen die Zifferntasten des Zehnerblocks dieselbe Funktion aus wie die der oberen Reihe?

**Domain:** Tastenbelegung
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/issues/260826-1223_*_der-grund-fuer-den-ausschluss-des-zehnerblocks-traegt-seit-der-zeichenkennung-nicht-mehr.md`

## Question

Der Zehnerblock ist im Nachschlag **halb** angeschlossen, und niemand hat das entschieden. Seine Zifferntasten lösen jede Belegung aus, die auf einer Ziffer liegt, ununterscheidbar von der oberen Reihe; seine Eingabetaste, sein Komma und seine vier Rechenzeichen lösen nichts aus, auch nicht das, was `return` auslöst. Bleibt es dabei, oder wird der Zehnerblock ganz angeschlossen oder ganz herausgehalten?

Der Stand ist nicht gewählt, sondern gefallen: bis zur Runde 2 schlug der Abgriff über den Tastencode nach, und die Codes des Zehnerblocks stehen nicht in `TASTEN` — damit war der Block draußen. Seit der Runde 2 fragt der Nachschlag für Buchstaben und Ziffern das gemeldete **Zeichen** (`crates/krk-core/src/tasten/parser.rs:393-396`, Nutzerentscheid vom 260808-0155), und der Tastencode wird für sie nicht mehr angesehen. Die Ziffern sind dadurch hereingekommen, ohne dass die Runde 2 es gesagt oder gemessen hätte; die Prosa in `parser.rs:237-238` und `:302-303` beschreibt bis heute den Zustand davor.

`inference:` Dass AppKit für eine Zifferntaste des Zehnerblocks über `charactersByApplyingModifiers(empty)` die Ziffer meldet, ist am Referenzgerät **nicht gemessen**. Am Quelltext allein entscheidbar ist der Rest der Kette. Wer diese Frage beantwortet, misst zuerst — ein Durchgang mit `--tasten-protokoll` genügt und ist Nutzerarbeit, weil KRK dafür im Vordergrund stehen muss.

## Options

**1. Es bleibt, wie es ist, und die Prosa wird nachgezogen.** Kostet nichts am Code. Die zwei Sätze in `parser.rs` nennen den Grund, der trägt (kein Name in der Schreibweise, also nicht von Hand belegbar und in der Belegungsansicht nicht zuweisbar), statt des Grundes, der nicht mehr trägt (eigene Tastencodes). Der Nutzer behält eine Tastatur, auf der `1` und die `1` des Blocks dasselbe tun und die Eingabetaste des Blocks nichts.

**2. Der Zehnerblock kommt ganz herein.** `TASTEN` bekommt die Gruppe, wie der Kopf von `parser.rs:88-99` es verlangt — „um ganze Tastengruppen, nie um einzelne Tasten" — also siebzehn Einträge mit eigenen Namen (`num0`…`num9`, `numenter`, `numplus`, …). Die Zifferntasten blieben dann **trotzdem** über ihr Zeichen erreichbar, denn die Kennungsregel (`Taste::kennung`, `parser.rs:192-198`) macht aus einem einbuchstabigen Namen ein Zeichen und aus jedem anderen eine Stelle; `num5` wäre eine Stelle, das Zeichen `'5'` bliebe bei der oberen Reihe. Damit wären die Rechenzeichen und die Eingabetaste des Blocks belegbar, ohne die Ziffern zu verändern. Kostet siebzehn Zeilen und eine Zeile im Kopf der Belegungsdatei; ändert am heutigen Verhalten nichts, sondern fügt hinzu.

**3. Der Zehnerblock kommt ganz heraus.** Der Nachschlag lehnt jeden Druck mit gesetztem `NSEventModifierFlagNumericPad` ab. Verlangt, dass `normalisieren` das Bit **nicht** mehr löscht, sondern durchreicht, und dass eine Stelle vor dem Nachschlag es liest. Das bricht mit der Begründung von `normalisierung.rs:34-37` und mit dem Zuschnitt „die Maske trägt vier Bits und keine fünf".

## Constraints

- Die Maxime lautet „Steuerung über die Tastatur", und der Zehnerblock ist Tastatur. Eine Antwort, die dem Nutzer eine funktionierende Taste wegnimmt, braucht mehr als Ordnungsliebe.
- `NSEventModifierFlagNumericPad` ist nach dem AppKit-Kopf auch bei den **Pfeiltasten** gesetzt (`normalisierung.rs:37-40` nennt die Frage ausdrücklich als im Projekt **ungemessen**). Möglichkeit 3 hinge damit an einer Messung, die es nicht gibt, und träfe im Fehlfall den Pfeilblock — also die Kernnavigation aus C2.
- Das Referenzgerät ist ein MacBook. Es hat keinen Zehnerblock; die Frage betrifft den Nutzer mit externer Tastatur und ist am Referenzgerät nicht abnehmbar.
- `TASTEN` wächst nach seinem eigenen Kopf nur um ganze Gruppen. Möglichkeit 2 ist die einzige, die sich an diese Regel hält.

## Recommendation

Möglichkeit 1 jetzt, Möglichkeit 2 als eigene Runde, wenn ein Nutzer mit Zehnerblock danach fragt.

Der Befund, der zu richten ist, ist die falsche Prosa, und die ist unabhängig davon zu richten, wie die Verhaltensfrage ausgeht — der Defektdatensatz oben trägt sie. Möglichkeit 3 scheidet aus: sie zahlt mit einer ungemessenen Annahme über den Pfeilblock für eine Ordnung, die niemand vermisst hat. Möglichkeit 2 fügt nur hinzu und nimmt nichts weg, kostet aber siebzehn Einträge für Tasten, die auf dem Abnahmegerät nicht vorkommen und deren Belegung deshalb nicht abzunehmen wäre.

---
Abgleich 260829-1252, am Baum `b9d9cbc`: **weiter offen, und eine Voraussetzung der Frage hat sich verschoben.** Der Datensatz sagt, die vier Rechenzeichen des Zehnerblocks lösten nichts aus. Seit der Runde 20 (`2aee690`, `1df8b8d`) tragen `plus` und `minus` die Codes `kVK_ANSI_KeypadPlus` (69) und `kVK_ANSI_KeypadMinus` (78) und lösen über das gemeldete Zeichen `cmd+plus`/`cmd+minus` aus (`crates/krk-core/src/tasten/parser.rs:362-365`, `:283-290`; Probe `:775-800`). Zwei der vier Rechenzeichen sind damit angeschlossen, Eingabetaste, Komma, Stern und Schrägstrich des Blocks weiter nicht. Keine Antwort auf die Frage gefunden; `shared/analyses/` und die Specs der Runden 19–22 nennen den Zehnerblock nicht als Gegenstand.
