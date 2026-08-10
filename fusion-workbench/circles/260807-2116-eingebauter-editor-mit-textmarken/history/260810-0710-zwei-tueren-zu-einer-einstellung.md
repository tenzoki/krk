# Zwei Türen zu einer Einstellung: die Probe an der Textfläche

**Agent:** coder
**Status:** Complete
**Datum:** 260810-0710
**Auftrag:** Die beiden Befunde `260810-0416` und `260810-0417` zusammen beheben,
statt die Laufzeitprobe zweimal umzubauen.

---

## Was ich gemessen habe, bevor ich etwas gebaut habe

Der erste Datensatz nannte zwei Einstellungen der Form `set…Type:`, die die
bisherige Probe nicht sah. Die naheliegende Antwort wäre gewesen, die Namensform
zu erweitern. Ich habe stattdessen erst aufgezählt, **was** `NSTextView` in den
drei fraglichen Formen trägt, und dann gemessen, **was diese Einstellungen
zueinander sind**.

Aufzählung (`AnyClass::get(c"NSTextView").instance_methods()`, macOS 15.7.7,
Build 24G720): sechsundzwanzig Selektoren — zwölf `set…Enabled:`, dreizehn
`set…Type:`, ein `set…Behavior:`.

Kopplungsmessung (Swift-Programm im Kratzverzeichnis, KVC über
`setValue:forKey:`, je Paar eine frische `NSTextView`): **zehn der dreizehn
`set…Type:` sind derselbe Speicher wie ein `set…Enabled:` daneben.** Zwanzig
Messungen, je Paar in beide Richtungen, alle gekoppelt. `smartQuotesType` und
`automaticQuoteSubstitutionEnabled` legen einander um; dasselbe gilt für
`smartDashesType`, `textReplacementType`, `autocorrectionType`,
`smartInsertDeleteType`, `spellCheckingType`, `grammarCheckingType`,
`linkDetectionType`, `dataDetectionType` und `textCompletionType`.

Das änderte den Bau. Ohne die Messung hätte ich zwölf Zeilen in
`textflaeche_bauen` geschrieben, von denen zehn abgeschaltet hätten, was schon
aus ist.

## Was ich geändert habe

`crates/krk-ui/src/appkit/editor.rs`, drei Stellen:

**`textflaeche_bauen`** bekommt zwei Zeilen. Aus fünf abgeschalteten Automatiken
werden sieben:

```rust
text.setInlinePredictionType(NSTextInputTraitType::No);
text.setMathExpressionCompletionType(NSTextInputTraitType::No);
```

**Der Modulkopf** zieht mit: sieben statt fünf, die dritte Gruppe und ihr Grund,
der Fund über die zweite Tür, die drei Grenzen der Probe, und der Satz, dass
nicht die Proben die Zusage aus C4 tragen, sondern die Zeilen in
`textflaeche_bauen` und die Prüfung am laufenden Bündel.

**`mod tests`** trägt statt zweier flacher Aufstellungen eine Aufstellung mit
Antworten:

- `FORMEN` — die drei Namensformen.
- `Einordnung` — `Abgeschaltet`, `Geduldet`, `ZweiteTuerZu(&str)`,
  `NochOffen(&str)`.
- `EINSTELLUNGEN` — alle sechsundzwanzig mit ihrer Antwort.
- `keine_unbekannte_einstellung_steht_an_der_textflaeche` — die Probe, jetzt mit
  getrennten Richtungen.
- `jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung` — neu.
- `die_drei_nachgereichten_automatiken_stehen_unter_den_abgeschalteten` — löst
  `die_fuenfte_automatik_steht_unter_den_abgeschalteten` ab.

## Die Entscheidung zur Probe

Der zweite Datensatz stellte zur Debatte, ob eine Probe dieser Bauart der
richtige Ort ist. **Sie bleibt, aber als Stolperdraht und nicht als
Vollständigkeitsbeweis.** Drei Grenzen sind nicht zu schließen und stehen deshalb
ausformuliert im Modulkopf:

1. **Zur Übersetzungszeit nicht erzwingbar.** Rust sieht die SDK-Kopfdateien
   nicht, `objc2` bildet keine Verfügbarkeitsgrenze ab, und `AnyProtocol` führt
   in `objc2` 0.6 keine Mitgliederliste — geprüft an
   `objc2-0.6/src/runtime/mod.rs:1045-1090`. `NSTextInputTraits` wäre der
   sachliche statt des namensbasierten Schnitts; der Weg dorthin führte über
   rohes FFI und damit `unsafe`, das `krk-ui` außerhalb von `appkit/mod.rs`
   verbietet.
2. **Die Namensform ist nicht der Schnitt.** Die zehn Paare belegen es von der
   anderen Seite: zwei Namen, eine Sache.
3. **Sie misst das prüfende Gerät.** Eine Laufzeitaufzählung kann nichts anderes.

Nur die gefährliche Richtung hält den Bau an. Ein verschwundener Schalter wird
ein `eprintln!` und färbt keine grüne Reihe auf einem unterstützten System rot.

## Ein Fund, den kein Datensatz führte

`setContentType:` (`NSTextContentType`, der Hinweis für Ausfüllvorschläge) stand
in keiner Aufstellung. Eingeordnet als geduldet: ab Werk `nil`, und auch mit Wert
trägt erst die Wahl des Nutzers Zeichen ein.

## Was ich nicht angefasst habe

`writingToolsBehavior` (`260810-0512`). `EINSTELLUNGEN` führt es als `NochOffen`
mit dem Verweis auf den Datensatz — damit die Probe es nicht übersieht und die
Antwort trotzdem beim Nutzer bleibt. Die Einstellung selbst wird nicht gesetzt.

## Abnahme

`make check` — Rückgabewert 0. Beide Richtungen der Probe gegengeprüft: ein aus
der Aufstellung entfernter Eintrag bricht sie und nennt den Selektor; ein Eintrag
ohne Entsprechung an der Klasse läuft grün durch und schreibt den Hinweis.

Beide Datensätze auf `_c_`, mit `Resolved:`-Abschnitt.

## Offen für den nächsten

Der Spec trägt für "keine Automatik ändert den getippten Text" kein eigenes
Abnahmekriterium. Die Zusage steht als Festlegung unter C2 ("kein Weg darf eine
Datei beim Sichern verändern …") und lebt im Übrigen im Modulkopf. Das ist eine
Aussage für den Abschlussschritt S42, nicht für mich.
