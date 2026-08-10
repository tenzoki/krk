# Der Stolperdraht sieht drei der vier Schreibwerkzeug-Einstellungen nicht, darunter ein Mitglied von NSTextInputTraits selbst

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:2616` (`FORMEN`), `:2665-2735` (`EINSTELLUNGEN`), Modulkopf `:129-186`
**Cross-references:** `issues/260810-0416_c_zwei-weitere-textveraendernde-automatiken-stehen-an-und-die-probe-sieht-sie-nicht.md`, `issues/260810-0417_c_die-laufzeitprobe-bindet-den-bau-an-die-macos-version-des-pruefenden-geraets.md`, `issues/260810-0512_o_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`, Spec C4

---

## Der Befund

Der Commit erweitert die Filterform um `Behavior:`, ausdrücklich damit
`setWritingToolsBehavior:` in Reichweite kommt. Die Erweiterung fängt genau
**eine** der vier Einstellungen, die `NSTextView` zu den Schreibwerkzeugen
trägt. Die anderen drei liegen weiter außerhalb aller drei Formen und stehen in
keiner Zeile von `EINSTELLUNGEN`:

| Selektor | Form | in `FORMEN`? | in `EINSTELLUNGEN`? |
|---|---|---|---|
| `setWritingToolsBehavior:` | `Behavior:` | ja | ja (`NochOffen`) |
| `setAllowedWritingToolsResultOptions:` | `Options:` | **nein** | **nein** |
| `setWritingToolsAllowedInputOptions:` | `Options:` | **nein** | **nein** |
| `setAllowsWritingToolsAffordance:` | `Affordance:` | **nein** | **nein** |

Die erste der drei fehlenden wiegt am schwersten: `allowedWritingToolsResultOptions`
ist **Mitglied des Protokolls `NSTextInputTraits`**, das der Modulkopf als den
„sachlichen Schnitt" benennt. Das Protokoll führt vierzehn Merkmale;
`EINSTELLUNGEN` trägt dreizehn davon.

Gemessen, nicht der Dokumentation entnommen: ein ObjC-Programm auf demselben
Gerät (macOS 15.7.7, Build 24G720) liest `protocol_copyPropertyList` von
`NSTextInputTraits` und `class_copyMethodList` von `NSTextView`:

```
NSTextInputTraits, properties=14
  autocorrectionType  spellCheckingType  grammarCheckingType  smartQuotesType
  smartDashesType  smartInsertDeleteType  textReplacementType  dataDetectionType
  linkDetectionType  textCompletionType  inlinePredictionType
  mathExpressionCompletionType  writingToolsBehavior
  allowedWritingToolsResultOptions        <-- fehlt in EINSTELLUNGEN
```

## Warum das die Aussage des Commits berührt

Der Modulkopf sagt zu: „verlangt, dass jeder Fund in `EINSTELLUNGEN` … eine
Antwort hat", und nennt drei Grenzen dieser Zusage. Die Grenze, die hier greift,
steht dort schon — „Die Namensform ist nicht der Schnitt, den die Sache
verlangt" —, aber sie steht als abstrakte Einschränkung. Sie hat einen
konkreten, heute belegbaren Fall, und der liegt unmittelbar neben der
Einstellung, für die die Form überhaupt erweitert wurde. Die Erweiterung um
`Behavior:` hat die Aufstellung **breiter** gemacht, nicht **vollständiger**.

`writingToolsBehavior` steht als `NochOffen` und wartet auf eine Lesart von C4
durch den Nutzer (`260810-0512`). Diese Lesart betrifft die drei fehlenden
mit: wer entscheidet, dass die Schreibwerkzeuge auszuschließen sind, schließt
sie über `writingToolsBehavior` allein nicht aus.

## Was heute hält

Kein gemessener Textverlust. KRK ruft keine der drei auf, und
`allowedWritingToolsResultOptions` steht an KRKs Fläche ab Werk auf `0`
(gemessen). Der Befund ist eine Lücke in der Zusage der Aufstellung, keine
beobachtete Fehlwirkung.

## Vorschlag

Die drei Selektoren in `EINSTELLUNGEN` aufnehmen und einordnen — mindestens
`setAllowedWritingToolsResultOptions:`, weil es Mitglied desselben Protokolls
ist wie die dreizehn geführten. Ob die Filterform noch weiter wachsen soll, ist
die falsche Frage: der Schnitt über das Protokoll ist erreichbar (siehe
`260810-0749`) und fängt dieses eine ohne jede weitere Namensform.

---
Resolved: Der Vorschlag ist gefahren, und zwar über den Schnitt und nicht über
eine weitere Namensform allein — genau in der Reihenfolge, die der letzte Absatz
dieses Datensatzes vorgibt.

`keine_unbekannte_einstellung_steht_an_der_textflaeche` zählt jetzt aus **zwei**
Quellen auf:

1. **Die Pflichtmerkmale des Protokolls `NSTextInputTraits`**, über
   `objc2::ffi::protocol_copyPropertyList`, und **ohne jede Namensform**: wer
   Mitglied dieses Protokolls ist, ist eine Texteingabe-Einstellung, gleich wie
   der Selektor endet. Aus dem Merkmalsnamen wird der Setzer nach der Regel, die
   Objective-C selbst anwendet. Vierzehn sind es auf diesem Gerät —
   nachgemessen, die Zahl dieses Datensatzes stimmt, und
   `setAllowedWritingToolsResultOptions:` war die eine der vierzehn, die fehlte.
2. **Die Namensformen über der ganzen Vererbungskette**, für alles, was
   `NSTextView` neben dem Protokoll führt.

Die beiden übrigen Schreibwerkzeug-Einstellungen sind **keine**
Protokollmitglieder, wie die Messung in diesem Datensatz zeigt. Für sie sind
`Options:` und `Affordance:` zu `FORMEN` gekommen, dazu `Types:` für die
Sammeltür aus `260810-0746`; sechs Formen statt drei. Damit sind alle vier
Schreibwerkzeug-Einstellungen eingeordnet, alle vier als `NochOffen` und alle
vier auf denselben Entscheidungsdatensatz
`decisions/260810-0959_o_schliesst-c4-die-schreibwerkzeuge-aus.md` — denn die
Feststellung dieses Datensatzes trägt: wer über `writingToolsBehavior` allein
entscheidet, hat die anderen drei nicht mitentschieden.

Was der Datensatz an der Erweiterung um `Behavior:` bemängelt — sie habe die
Aufstellung breiter, nicht vollständiger gemacht —, ist damit beantwortet: die
erste Quelle braucht keine Form und wird von der zweiten nur ergänzt.
