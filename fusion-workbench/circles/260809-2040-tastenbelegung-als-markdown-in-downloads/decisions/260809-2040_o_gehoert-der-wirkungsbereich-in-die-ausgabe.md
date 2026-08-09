# Gehört der Wirkungsbereich mit in die Ausgabe, also wo ein Befehl wirkt?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_a_circle.md` (Directive und Grounding), `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_was-steht-in-der-ausgabe-und-wonach-ist-sie-gegliedert.md` (Umfang und Ordnung, getrennt gefragt), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_i_welcher-bereich-den-fokus-fuer-die-zwischenablage-befehle-haben-muss.md`

---

## Question

KRK führt zu jedem Kommando eine Eigenschaft, die sagt, welcher Bereich den Eingabefokus haben muss, damit der Befehl wirkt: `Kommando::wirkungsbereich` in `crates/krk-core/src/tasten/belegung.rs`, mit sieben Werten von `Dateifenster` über `Editor` bis `Ueberall`. Der Vorbehalt ist **stumm**: ein Kommando, das daran scheitert, tut nichts und meldet nichts, und der Tastendruck geht unverändert weiter.

Genau diese Stummheit macht die Frage erheblich. Wer `Cmd+Backspace` im Editor drückt und nichts geschieht, hat keinen Weg herauszufinden, warum, außer im Quelltext nachzusehen. Eine Ausgabe, die den Wirkungsbereich führt, wäre der einzige Ort, an dem KRK diese Regel je erklärt.

Dagegen steht, dass die Belegungsansicht am Bildschirm ihn heute **nicht** zeigt. Sie hat zwei Spalten, Funktion und Belegung. Der Wirkungsbereich aufzunehmen hieße also, dass die Datei mehr zeigt als der Schirm, und das ist genau die Abweichung, die die dritte Frage dieses Circles ausschließen will.

## Options

1. **Nein: die Ausgabe zeigt, was die Bildschirmansicht zeigt, also Funktion und Belegung.**
   - Pro: eine Wahrheit über die Belegung in zwei Ausgabeformen. Der Zuschnitt bleibt klein, und die Ausgabe braucht nichts, was es nicht schon gibt.
   - Contra: die stumme Regel bleibt stumm. Der Nutzer erfährt nirgends, warum ein Befehl an einer Stelle nichts tut.
2. **Ja, als dritte Spalte je Funktion.** Neben Funktion und Belegung steht, wo der Befehl wirkt.
   - Pro: beantwortet die Frage genau dort, wo sie aufkommt, nämlich neben der Taste. Die Eigenschaft liegt fertig im Kern und ist vollständig.
   - Contra: sie hat heute **keine deutschen Namen**. `Wirkungsbereich` trägt keinen `impl`-Block, also müssen sieben Beschriftungen entstehen, und das ist eine neue Namensliste, die niemand außer dieser Ausgabe braucht. Die sechs vom Menü zugestellten Funktionen haben kein Kommando und damit keinen Wirkungsbereich; ihre Zellen blieben leer und verlangten eine Erklärung. Und die Datei zeigt mehr als der Schirm.
3. **Ja, aber nicht je Zeile: ein erklärender Abschnitt am Ende der Datei**, der die sieben Bereiche benennt und sagt, welche Funktionsbereiche in welchem wirken.
   - Pro: erklärt die Regel, ohne jede Zeile zu verbreitern, und bleibt dabei eine Erklärung statt einer zweiten Datenspalte.
   - Contra: dieselben sieben Beschriftungen wie in Möglichkeit 2, und dazu erklärender Fließtext, der von Hand gepflegt werden muss und mit einem achten Wert veraltet, ohne dass der Übersetzer es anhält.

## Constraints

- `Kommando::wirkungsbereich` ist eine vollständige Fallunterscheidung ohne Auffangzweig, und `CLAUDE.md` führt sie als eine der drei, die ein neues Kommando anhalten. Jede Antwort, die die Werte beschriftet, muss diese Eigenschaft behalten: die Beschriftungen gehören in dieselbe vollständige Form, nicht in eine Tabelle mit Rückfall.
- Der Wirkungsbereich und der Funktionsbereich der Ansicht sind zwei verschiedene Gliederungen und dürfen nicht vermengt werden. Der Modulkopf von `belegungsmodell.rs` sagt es ausdrücklich: der Wirkungsbereich beantwortet, welcher Bereich den Fokus haben muss, und wirft Fenster-, Fokus- und Anwendungsbefehle in einen Topf, den kein Nutzer als Ordnung wiedererkennt.
- Die sechs vom Hauptmenü zugestellten Funktionen haben kein Kommando (`Funktion::kommando` liefert `None`) und damit keinen Wirkungsbereich. Jede Antwort mit einer dritten Spalte braucht für sie eine Zelle.

## Recommendation

**Wir empfehlen Möglichkeit 1 für diesen Circle** und halten Möglichkeit 3 für den besseren zweiten Schritt, falls der Nutzer die stumme Regel als Problem erlebt.

Der Grund ist der Zuschnitt, nicht der Wert der Information. Die Directive sagt, die Ausgabe entstehe aus derselben Belegung wie die Bildschirmansicht und stelle keine zweite Aufbereitung daneben. Sieben neue Beschriftungen, eine Regel für die sechs Funktionen ohne Kommando und eine Spalte, die es am Schirm nicht gibt, sind zusammen eine zweite Aufbereitung, auch wenn jeder Teil für sich klein aussieht.

**Was daran eine Auslegung ist:** wie oft der Nutzer über die stumme Regel stolpert. Wer sich seine Belegung umbaut und Befehle an Stellen legt, an denen sie nicht wirken, stolpert häufiger als wer bei der Auslieferung bleibt. Das kann nur er sagen.

---
Answered:
Implemented:
Deferred:
Superseded by:
