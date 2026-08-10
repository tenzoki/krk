# Zwei weitere textverändernde Automatiken stehen an, und die Probe kann sie nicht sehen

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`textflaeche_bauen`, `ABGESCHALTET`, `GEDULDET`, `keine_unbekannte_automatik_steht_an_der_textflaeche`, Modulkopf)
**Cross-references:** `issues/260809-1650_c_die-fuenfte-textveraendernde-automatik-smart-insert-delete-bleibt-an.md`, `issues/260810-0512_o_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`, Spec C4 („der gesicherte Stand ist der getippte"), Commit `f7ef6c5`

---

## Der Befund

`NSTextView` trägt auf dem Mindest-Zielsystem macOS 15 zwei weitere
Einstellungen, die Zeichen in den Text bringen, die der Nutzer nicht getippt
hat. Beide stehen auf ihrem Vorgabewert, beide sind nicht abgewählt, und beide
liegen **außerhalb** der Form `set…Enabled:`, auf die die Probe
`keine_unbekannte_automatik_steht_an_der_textflaeche` prüft:

| Einstellung | ab | Vorgabewert an KRKs Fläche |
|---|---|---|
| `inlinePredictionType` | macOS 14 | `0` (`NSTextInputTraitTypeDefault`, das System entscheidet) |
| `mathExpressionCompletionType` | macOS 15 | `0` (`NSTextInputTraitTypeDefault`, das System entscheidet) |

Gemessen, nicht der Dokumentation entnommen: ein kleines Swift-Programm auf
demselben Gerät (macOS 15.7.7, Build 24G720) baut eine `NSTextView` und liest
die beiden Werte sowie die Selektoren aus. Die Ausgabe lautet

```
inlinePredictionType: 0
mathExpressionCompletionType: 0
setInlinePredictionType: vorhanden: true
setMathExpressionCompletionType: vorhanden: true
set…Enabled: an NSTextView: [12 Selektoren, genau die zwölf aus
                             ABGESCHALTET und GEDULDET]
```

Die Deklarationen stehen in
`MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSTextView.h:485`
und `:488`.

## Warum sie nicht unter die Begründung von 260810-0512 fallen

Der Modulkopf trennt die abgewählten Automatiken von den Schreibwerkzeugen aus
macOS 15 mit dem Satz: „Sie unterscheiden sich von den fuenf darin, dass der
Nutzer sie eigens aufruft." Für diese beiden gilt das gerade nicht:

- **Die Vorhersage im Textfluss** schlägt die Fortsetzung eines Wortes grau vor
  und trägt sie ein, sobald der Nutzer die Leer- oder die Tabulatortaste
  drückt. Ein Aufruf durch den Nutzer findet nicht statt; er tippt weiter.
- **Die Auswertung von Rechenausdrücken** ersetzt beim Tippen von `=` den
  Ausdruck davor durch sein Ergebnis. In Prosa ist das gemeint. In einer
  Konfigurationsdatei mit `wert=1+2` ist es eine Änderung, die niemand getippt
  hat, und damit dieselbe Sorte, gegen die die fünf abgeschalteten stehen.

Damit ist die Aussage des Modulkopfs, „dass es bei fuenf bleibt, haelt eine
Probe fest und nicht die Aufmerksamkeit des naechsten Lesers", heute nicht
gedeckt: die Probe misst eine Namensform, und zwei Einstellungen derselben
Wirkung tragen eine andere.

## Was zu entscheiden ist

Zwei Fragen hängen daran, und dieser Defekt beantwortet keine:

1. **Greifen die beiden an KRKs Fläche überhaupt?** `setRichText(false)`,
   abgeschaltete Rechtschreibkorrektur und eine Fläche ohne
   Sprachvorgabe könnten sie im Ergebnis wirkungslos machen. Das ist am
   laufenden Bündel zu messen und nicht am Kopf zu entscheiden — dieselbe
   Nutzerarbeit, die der Modulkopf für die fünf Zeilen ohnehin ansetzt.
2. **Trägt die Probe künftig eine zweite Form?** Eine Aufzählung, die neben
   `set…Enabled:` auch `set…Type:` und `set…Behavior:` einordnet, fängt die
   drei heute bekannten Fälle (die beiden hier und `writingToolsBehavior` aus
   `260810-0512`). Sie fängt nicht die nächste Form, und der Schnitt „alles,
   was den Textspeicher anfassen kann" ist an einer Namensform überhaupt nicht
   entscheidbar. Ob die Probe dann noch das richtige Werkzeug ist, gehört mit
   zur Antwort.

## Was heute hält

Kein gemessener Textverlust. Der Befund ist eine offene Flanke gegen die Zusage
aus C4 und keine beobachtete Fehlwirkung.
