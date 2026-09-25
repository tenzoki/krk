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

---
Resolved: `textflaeche_bauen` setzt beide auf `No`, die Aufstellung unter
`mod tests` ordnet sie als abgeschaltet ein, und die Probe erreicht jetzt die
Namensform, in der sie stehen. Modulkopf und Proben ziehen mit.

**Beide Zeilen stehen.** `crates/krk-ui/src/appkit/editor.rs`,
`textflaeche_bauen`:

```rust
text.setInlinePredictionType(NSTextInputTraitType::No);
text.setMathExpressionCompletionType(NSTextInputTraitType::No);
```

`NSTextInputTraitType::No` ist die Absage; der Vorgabewert `Default` überließ die
Wahl dem System. Aus fünf abgeschalteten Automatiken sind damit sieben geworden.

**Die zweite Frage des Datensatzes ist beantwortet, und die Antwort fiel anders
aus als die Frage sie stellte.** Der Datensatz fragte, ob die Probe künftig eine
zweite Namensform trägt. Sie trägt jetzt drei — `Enabled:`, `Type:`, `Behavior:`
—, aber der tragende Fund liegt daneben: **die Form `set…Type:` ist zum größten
Teil keine zweite Menge von Einstellungen, sondern eine zweite Tür zu denselben.**

`NSTextView` trägt auf macOS 15.7.7 sechsundzwanzig Einstellungen der drei
Formen. Zehn der dreizehn `set…Type:` sind derselbe Speicher wie ein
`set…Enabled:` daneben:

| zweite Tür | erste Tür |
|---|---|
| `smartQuotesType` | `automaticQuoteSubstitutionEnabled` |
| `smartDashesType` | `automaticDashSubstitutionEnabled` |
| `textReplacementType` | `automaticTextReplacementEnabled` |
| `autocorrectionType` | `automaticSpellingCorrectionEnabled` |
| `smartInsertDeleteType` | `smartInsertDeleteEnabled` |
| `spellCheckingType` | `continuousSpellCheckingEnabled` |
| `grammarCheckingType` | `grammarCheckingEnabled` |
| `linkDetectionType` | `automaticLinkDetectionEnabled` |
| `dataDetectionType` | `automaticDataDetectionEnabled` |
| `textCompletionType` | `automaticTextCompletionEnabled` |

Gemessen, nicht der Dokumentation entnommen: ein Swift-Programm auf demselben
Gerät (macOS 15.7.7, Build 24G720) legt je Paar die eine Tür um und liest die
andere, in beiden Richtungen und für jedes Paar einzeln. Alle zwanzig Messungen
sind gekoppelt. Wer die zweite Tür noch einmal zuschließt, schaltet zehnmal ab,
was schon aus ist — deshalb hat `textflaeche_bauen` zwei neue Zeilen bekommen
und nicht zwölf.

Ohne Zwilling stehen genau drei: die beiden dieses Datensatzes und
`writingToolsBehavior`.

**Ein Fund kommt hinzu, den dieser Datensatz nicht führte.** `setContentType:`
(`NSTextContentType`, der Hinweis für Ausfüllvorschläge) stand in keiner
Aufstellung. Er ist als geduldet eingeordnet: an KRKs Fläche steht er ab Werk auf
`nil`, es gibt also nichts vorzuschlagen, und auch mit Wert trägt erst die Wahl
des Nutzers Zeichen ein — derselbe Schnitt wie bei der Textvervollständigung.

**Die erste Frage des Datensatzes bleibt offen und ist Nutzerarbeit.** Ob die
beiden an KRKs Fläche überhaupt gegriffen hätten, ist am laufenden Bündel zu
messen. Das Abschalten macht die Frage in der sicheren Richtung gegenstandslos:
sie greifen jetzt nicht, ob sie es vorher taten oder nicht.

**Die Proben tragen neue Namen und eine neue Aufstellung**, alle in
`crates/krk-ui/src/appkit/editor.rs` unter `mod tests`:

- `EINSTELLUNGEN` — jede der sechsundzwanzig mit ihrer Antwort auf C4:
  abgeschaltet, geduldet, zweite Tür zu, oder noch offen.
- `keine_unbekannte_einstellung_steht_an_der_textflaeche` — Nachfolgerin von
  `keine_unbekannte_automatik_steht_an_der_textflaeche`.
- `jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung` — neu, damit keine
  zweite Tür auf eine zweite Tür oder ins Leere zeigt.
- `die_drei_nachgereichten_automatiken_stehen_unter_den_abgeschalteten` —
  Nachfolgerin von `die_fuenfte_automatik_steht_unter_den_abgeschalteten`, jetzt
  mit allen dreien, die nachträglich hereinkamen.

Die Zusicherung wurde gegengeprüft: ein aus der Aufstellung entfernter Eintrag
bricht die Probe und nennt den Selektor beim Namen.

**Was die Probe nicht kann, steht jetzt im Modulkopf** statt zwischen den Zeilen;
die drei Grenzen sind mit `260810-0417` zusammen beantwortet und dort ausgeführt.
