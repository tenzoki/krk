# Die Kopplung der zehn Paare trägt den Commit und ist im Baum durch nichts gehalten

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:2628-2632` (`Einordnung::ZweiteTuerZu`), `:2699-2731` (die zehn Einträge), `:2826-2848` (`jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung`)
**Cross-references:** `issues/260810-0416_c_zwei-weitere-textveraendernde-automatiken-stehen-an-und-die-probe-sieht-sie-nicht.md`, `spikes/fn-tasten/beobachter.swift` (der Umgang mit einem früheren Messprogramm), `messungen/`

---

## Die Behauptung ist geprüft und hält

Zuerst der Teil, der stimmt. Ich habe die Kopplung unabhängig nachgemessen, auf
demselben Gerät (macOS 15.7.7, Build 24G720), je Paar einzeln, in beiden
Richtungen, mit einem eigenen ObjC-Programm über `valueForKey:`/`setValue:forKey:`:

```
smartQuotesType       / automaticQuoteSubstitutionEnabled  boolNO->type=1  typeNO->bool=0
smartDashesType       / automaticDashSubstitutionEnabled   boolNO->type=1  typeNO->bool=0
textReplacementType   / automaticTextReplacementEnabled    boolNO->type=1  typeNO->bool=0
autocorrectionType    / automaticSpellingCorrectionEnabled boolNO->type=1  typeNO->bool=0
smartInsertDeleteType / smartInsertDeleteEnabled           boolNO->type=1  typeNO->bool=0
spellCheckingType     / continuousSpellCheckingEnabled     boolNO->type=1  typeNO->bool=0
grammarCheckingType   / grammarCheckingEnabled             boolNO->type=1  typeNO->bool=0
linkDetectionType     / automaticLinkDetectionEnabled      boolNO->type=1  typeNO->bool=0
dataDetectionType     / automaticDataDetectionEnabled      boolNO->type=1  typeNO->bool=0
textCompletionType    / automaticTextCompletionEnabled     boolNO->type=1  typeNO->bool=0
```

`1` ist `NSTextInputTraitTypeNo`. Alle zehn Paare koppeln in beide Richtungen.
Die Schlussweise des Commits — „wer die erste Tür zuschließt, hat die zweite
mit zugeschlossen, also genügen zwei neue Zeilen statt zwölf" — ist damit für
alle zehn Paare belegt und nicht nur für die gemessenen Richtungen. **Der
Commit steht in dieser Sache.**

## Der Befund

Diese Messung ist die tragende Behauptung des Commits: an ihr hängen zehn der
sechsundzwanzig Einträge und die Entscheidung, `textflaeche_bauen` **nicht** um
zehn Zeilen zu ergänzen. Im Baum hält sie nichts.

- Das Swift-Programm, das die zwanzig Messungen erzeugt hat, ist nirgends
  abgelegt — geprüft: `find . -name '*.swift'` findet allein
  `spikes/fn-tasten/beobachter.swift`, und `git log --all -- '*.swift'` nennt
  keinen weiteren. Für die Fn-Tastenfrage wurde das Prüfprogramm aufgehoben;
  hier nicht.
- Unter `messungen/` liegt kein Bericht dazu. Fünfzehn Berichte stehen dort,
  keiner nennt die Kopplung.
- `jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung` prüft die
  Aufstellung **gegen sich selbst**: kein Name doppelt, jedes Ziel trägt eine
  eigene Antwort. Sie fasst AppKit nicht an. Sie liefe grün weiter, wenn keines
  der zehn Paare mehr koppelte.
- `keine_unbekannte_einstellung_steht_an_der_textflaeche` prüft Namen, nicht
  Wirkung. Auch sie sieht eine gelöste Kopplung nicht.

Damit gilt: entkoppelt Apple in einem späteren macOS ein Paar, oder war eine
der zwanzig Messungen falsch abgelesen, stehen bis zu zehn Einstellungen offen,
die der Commit für geschlossen hält — und jede Probe im Baum bleibt grün. Das
ist dieselbe Bauart von Lücke, gegen die die Probe von `260809-1650` überhaupt
angetreten ist, eine Ebene höher.

## Was heute hält

Alles. Die Kopplung ist heute nachgemessen und gilt. Der Befund ist eine
Aussage über die Haltbarkeit der Aussage, nicht über den heutigen Zustand.

## Vorschlag

Die Messung braucht ein Artefakt, das sich auf macOS 26 erneut fahren lässt.
Der billigste Weg ist der, den das Projekt für die Fn-Tastenfrage schon
gegangen ist: das Messprogramm unter `spikes/` ablegen und einen Bericht unter
`messungen/` schreiben, mit Gerät, Build und den zwanzig Zeilen.

Eine Probe im Baum wäre stärker, ist aber nicht umsonst zu haben: sie braucht
eine `NSTextView`-Instanz und damit den Hauptfaden. Das Muster steht schon in
derselben Datei — `editor.rs:2521` baut sich einen `MainThreadMarker` über
`new_unchecked()`, innerhalb von `mod tests`. Ob das für den Bau einer
AppKit-Ansicht tragfähig ist, ist eigens zu prüfen und nicht aus dem
vorhandenen Fall zu schließen; der Verweis sagt nur, dass die Hürde nicht dort
liegt, wo `260810-0417` sie vermutet hat (siehe `260810-0749`).

---
Resolved: Der Befund hält in jedem Punkt, und die Lösung ist die stärkere der
beiden angebotenen: nicht das Ablegen eines Messprogramms unter `spikes/`,
sondern die Probe im Baum.

**Die Hürde ist geprüft und liegt nicht dort, wo sie vermutet wurde.** Eine
`NSTextView` lässt sich in `cargo test` bauen und über `valueForKey:` /
`setValue:forKey:` befragen; dieser Datensatz nennt die Frage ausdrücklich als
eigens zu prüfen, und sie ist jetzt geprüft. `jede_zweite_tuer_und_ihre_erste_legen_einander_um`
fährt alle zehn Paare, je Paar in beiden Richtungen, je Richtung auf einer
eigenen Fläche, damit die zweite Messung nicht auf dem Ergebnis der ersten
sitzt.

**Die tragende Änderung ist die Richtung der Abhängigkeit.** Die Probe liest die
zehn Paare **aus `EINSTELLUNGEN`**. Damit ist die Aufstellung die Vorlage der
Messung und nicht ihr Nachtrag: eine Zeile, die eine Kopplung behauptet, die es
nicht gibt, hält den Bau an. Entkoppelt eine spätere Fassung von macOS ein Paar,
wird die Reihe rot, statt grün zu bleiben — genau der Fall, den dieser Datensatz
beschreibt. Dasselbe Muster tragen jetzt drei weitere Proben: die sieben
abgeschalteten Automatiken, die Sammeltür und der Vorgabewert der
Schreibwerkzeuge lesen ihre Namen ebenfalls aus der Aufstellung.

Ein Bericht unter `messungen/` ist damit nicht mehr die Sicherung, sondern
höchstens eine Zweitschrift. `spikes/` und `messungen/` lagen außerhalb der
Dateigrenze dieses Durchgangs; sollte der Nutzer die Zweitschrift trotzdem
wollen, ist sie ohne die Probe nachzutragen.

**Der Preis ist benannt und hat seinen eigenen Datensatz.** Die vier neuen Proben
behaupten über `MainThreadMarker::new_unchecked` den Hauptfaden, den `libtest`
ihnen nicht gibt. Sechs vollständige Läufe von `cargo test --workspace` ohne
Absturz sind gemessen, und der Doc-Kommentar von `an_einer_flaeche` sagt, was
das belegt und was nicht:
`issues/260810-1001_o_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`.
