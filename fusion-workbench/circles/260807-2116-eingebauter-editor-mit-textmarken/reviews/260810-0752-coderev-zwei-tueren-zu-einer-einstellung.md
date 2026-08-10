# Durchsicht Runde 2: zwei Türen zu einer Einstellung

**Sender:** coderev
**Umfang:** `git diff e6b76ab..HEAD -- crates/` — eine Datei,
`crates/krk-ui/src/appkit/editor.rs`, +310/−87, Commit `d9fc2c8`
**Gerät der Messungen:** macOS 15.7.7, Build 24G720
**Stand des Baums:** selbst gefahren — `cargo test --workspace` grün (308 Proben
in `krk-ui`, darunter die drei neuen), `cargo fmt --all --check` grün,
`cargo clippy --workspace --all-targets` ohne Warnung.

---

## Zusammenfassung

Der Commit tut, was er zusagt: die beiden fehlenden Automatiken sind
abgeschaltet, die Probe sagt jetzt getrennt, was sie hält und was nicht, und
die Aufstellung ist von zwei flachen Listen zu einer beantworteten Tabelle
geworden. Die tragende Behauptung — zehn der dreizehn `set…Type:` sind eine
zweite Tür zu einem `set…Enabled:` — habe ich unabhängig nachgemessen; **sie
hält für alle zehn Paare in beiden Richtungen**. Sieben Befunde stehen dagegen,
keiner davon am ausgeführten Code: vier betreffen die Reichweite der Probe,
zwei die Haltbarkeit und Genauigkeit der Messaussage, einer eine Begründung,
die schlicht nicht zutrifft.

**Totals:** Critical 0 · High 0 · Medium 5 · Low 2

---

## Antwort auf die drei Fragen

### 1. Trägt die Kopplungsmessung?

**Ja, und ich habe sie nachgemessen statt sie zu übernehmen.** Ein eigenes
ObjC-Programm legt je Paar die eine Tür um und liest die andere, für jedes der
zehn Paare einzeln und in beiden Richtungen:

```
boolNO  -> type = 1 (NSTextInputTraitTypeNo)   fuer alle zehn
typeNO  -> bool = 0                            fuer alle zehn
```

Der Schluss gilt damit **für alle zehn Paare**, nicht nur für die gemessenen
Richtungen — es waren beide Richtungen, und meine Messung ist von der des
Ausführenden unabhängig. „Legt einander um" ist als Beleg für den operativen
Schluss ausreichend: `bool = NO` nagelt den Typ auf `No` fest, die zweite Tür
noch einmal zuzuschließen ändert nichts. Zwei neue Zeilen statt zwölf sind
richtig.

Zwei Einschränkungen bleiben, beide als Befund abgelegt:

- „Derselbe Speicher" ist eine Stufe stärker als gemessen. Der Wahrheitswert
  ist eine verlustbehaftete Sicht auf den dreiwertigen Typ: `Default` lässt
  sich über die erste Tür nicht herstellen, und ihr Lesen löst `Default` in
  eine Systemvorgabe auf, die je Einstellung verschieden ausfällt (gemessen:
  acht Paare → `1`, `linkDetectionType` und `dataDetectionType` → `0`).
  Befund `260810-0750`, Low.
- Die Messung ist im Baum durch nichts gehalten. Das Swift-Programm ist nicht
  abgelegt, unter `messungen/` steht kein Bericht, und
  `jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung` prüft die Tabelle
  nur gegen sich selbst — sie fasst AppKit nicht an. Entkoppelt Apple auf
  macOS 26 ein Paar, stehen zehn Einstellungen offen und jede Probe bleibt
  grün. Befund `260810-0748`, Medium.

### 2. Ist die Asymmetrie richtig begründet, und trägt die Erweiterung um `Behavior:`?

**Die Asymmetrie ist richtig.** Eine Einstellung, die es nicht mehr gibt, kann
keine Zeichen ändern; eine grüne Reihe auf einem unterstützten System rot zu
färben wäre falsch. Der Schnitt ist sauber gelegt.

**Die Umsetzung sagt aber mehr zu, als sie hält.** Der `eprintln!`-Zweig läuft
genau dann, wenn der Test **nicht** fehlschlägt — und `libtest` gibt die Ausgabe
eines erfolgreichen Tests nicht aus. Nachgestellt mit einem minimalen Test:
ohne `--nocapture` erscheint nichts, mit `--nocapture` erscheint alles. Kein
Kommando des Projekts setzt das Flag, `make test` am wenigsten. Der Hinweis
geht heute auf allen begangenen Wegen ins Leere. Befund `260810-0747`, Medium.

**Und ja, ein neuer textverändernder Schalter kann weiter still durchrutschen.**
Die Erweiterung um `Behavior:` macht die Aufstellung breiter, nicht
vollständiger, und das ist nicht als Möglichkeit, sondern belegt:

- `setWritingToolsBehavior:` ist eine von **vier** Schreibwerkzeug-Einstellungen
  an `NSTextView`. Die Erweiterung fängt diese eine. Die drei anderen —
  `setAllowedWritingToolsResultOptions:`, `setWritingToolsAllowedInputOptions:`,
  `setAllowsWritingToolsAffordance:` — liegen weiter draußen. Die erste davon
  ist **Mitglied desselben Protokolls `NSTextInputTraits`**, das der Modulkopf
  als den sachlichen Schnitt benennt: das Protokoll führt vierzehn Merkmale,
  `EINSTELLUNGEN` trägt dreizehn. Befund `260810-0745`, Medium.
- `setEnabledTextCheckingTypes:` ist eine **dritte** Tür, außerhalb aller drei
  Formen (`Types:`, nicht `Type:`). Gemessen: nach KRKs fünf Zeilen macht ein
  einziger Aufruf mit `NSTextCheckingAllTypes` fünf der sieben abgeschalteten
  Automatiken wieder an und setzt `smartQuotesType` von `No` auf `Yes` zurück.
  Der Modulkopf zählt zwei Türen; es sind drei. Befund `260810-0746`, Medium.
- Die Aufzählung sieht nur `NSTextView` selbst. `instance_methods()` ist
  `class_copyMethodList` und liefert keine ererbten Methoden; fünf Selektoren
  der drei Formen sitzen heute schon an `NSView` und `NSResponder` und laufen
  ungesehen mit. Befund `260810-0751`, Low.

Dreimal in Folge — `260809-1650`, `260810-0416`, und jetzt `260810-0746` — ist
die Aufzählung an einer Namensform stehengeblieben und die nächste Einstellung
derselben Wirkung trug eine andere. Das ist kein Einzelfund mehr. Der Modulkopf
sagt es selbst richtig („Die Namensform ist nicht der Schnitt, den die Sache
verlangt"), zieht daraus aber weiter die Konsequenz, die Form zu **erweitern**
statt zu **wechseln**.

### 3. Trägt „zur Übersetzungszeit nicht erzwingbar"?

**Zwei der drei Punkte halten, der dritte nicht.**

- *Rust sieht die SDK-Kopfdateien nicht* — hält. Die Bindungen sind
  handgeschrieben, keine Stelle im Baum liest eine Kopfdatei.
- *`objc2` bildet keine Verfügbarkeitsgrenze ab* — hält. Geprüft an
  `objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1642-1665`: die beiden neuen
  Setzer tragen als einzige Bedingung ein Cargo-Merkmal, keine
  Verfügbarkeitsangabe.
- *`AnyProtocol` führt in `objc2` 0.6 keine Mitgliederliste* — hält für die
  **sichere** Schnittstelle, geprüft an `objc2-0.6.4/src/runtime/mod.rs:1045ff.`
  Der Schluss daraus trägt aber nicht.

**Der Weg ohne `unsafe` außerhalb von `appkit/mod.rs` ist nicht nötig, weil die
Grenze, auf die sich die Begründung beruft, nicht existiert.**
`appkit/mod.rs:1-8` schreibt selbst, dass die Lint-Regel in den ganzen Teilbaum
`src/appkit/` durchschlägt; `xtask/src/release.rs:78` setzt
`AUSNAHME = "crates/krk-ui/src/appkit"`, ebenfalls den Teilbaum; und
`editor.rs` nennt `unsafe` bereits vierundzwanzigmal, eine davon (`:2521`)
innerhalb desselben `mod tests`, zwanzig Zeilen über der Probe. Dazu führt
`objc2` 0.6.4 die Aufzählung in `src/ffi/protocol.rs:49,57`.

Zum Beleg habe ich den Schnitt gefahren — er findet auf Anhieb die eine
Einstellung, die der Namensschnitt nicht sieht (Frage 2, `260810-0745`). Er ist
also erreichbar und für die Merkmale, die er abdeckt, überlegen.

Zur Ehrlichkeit die Gegenrechnung: der Protokollschnitt **allein** wäre enger
als der heutige. Er kennt weder die dreizehn `set…Enabled:`, die `NSTextView`
für sich trägt, noch `setEnabledTextCheckingTypes:`. Wer ihn will, will die
Vereinigung, nicht den Ersatz — der Modulkopf sagt heute „statt", und das trägt
nicht. Befund `260810-0749`, Medium.

---

## Was der Commit richtig macht

Nicht unerwähnt, weil es die Befunde einordnet:

- Die zwei Zeilen in `textflaeche_bauen:2121-2122` stehen richtig, mit dem
  richtigen Wert (`No`, nicht `Default`), und beide Methoden liegen auf oder
  unter dem Zielsystem — die Notiz dazu im Modulkopf `:204-207` ist die richtige
  Stelle.
- Die vier Antworten in `Einordnung` sind disjunkt und decken die
  sechsundzwanzig geführten Fälle vollständig ab. `NochOffen` als eigener Wert,
  statt den Fall aus der Tabelle zu lassen, ist die richtige Entscheidung: die
  Nutzerfrage `260810-0512` bleibt sichtbar, ohne dass die Probe sie beantwortet.
- `jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung` schließt eine echte
  Lücke — eine zweite Tür auf eine zweite Tür wäre ohne sie unbemerkt geblieben.
- Der Modulkopf sagt jetzt, was die Zusage aus C4 **trägt** (die sieben Zeilen
  und die Prüfung am laufenden Bündel) statt die Probe dafür auszugeben. Das
  war der Kern von `260810-0417` und ist erledigt.

---

## Befunde, geordnet

| # | Befund | Schwere | Datei |
|---|---|---|---|
| 1 | Drei von vier Schreibwerkzeug-Einstellungen außerhalb, darunter ein `NSTextInputTraits`-Mitglied | Medium | `260810-0745_o_…` |
| 2 | `setEnabledTextCheckingTypes:` ist eine dritte Tür, außerhalb aller Formen | Medium | `260810-0746_o_…` |
| 3 | Der `eprintln!`-Hinweis wird von libtest verschluckt | Medium | `260810-0747_o_…` |
| 4 | Die Kopplungsmessung hat kein Artefakt und keine Nachprüfung im Baum | Medium | `260810-0748_o_…` |
| 5 | Die Begründung über `unsafe` trifft nicht zu; der Protokollschnitt ist erreichbar | Medium | `260810-0749_o_…` |
| 6 | „Derselbe Speicher" ist stärker als die Messung | Low | `260810-0750_o_…` |
| 7 | Die Aufzählung sieht keine Oberklassen | Low | `260810-0751_o_…` |

*(`260810-0748` und `260810-0750` betreffen dieselbe Messung von zwei Seiten —
ihre Haltbarkeit und ihre Formulierung — und stehen deshalb getrennt.)*

**Querschnitt.** Vier der sieben sind derselbe Fehlermodus: eine Aufzählung
schneidet an einem Namen, und die Sache liegt quer dazu. Die Runde hat ihn
zweimal behoben und zweimal an derselben Stelle wieder eingebaut. Die
Konsequenz ist nicht die nächste Namensform, sondern ein Schnitt über das
Protokoll **zusätzlich** zu den Namensformen.

## Reihenfolge

**Nichts hiervon ist ein Abnahmeblocker.** Kein Befund betrifft das Verhalten
des laufenden Bündels, keiner die zehn Zeitzusagen aus C8, und die vier
Abnahmekommandos laufen grün. Der Abnahmelauf am laufenden Bündel kann fahren.

Für danach, in dieser Reihenfolge:

1. `260810-0747` — zwei Zeilen Text, macht eine falsche Zusage weg.
2. `260810-0749` — Begründung berichtigen, sonst ist der bessere Weg dauerhaft
   mit einem falschen Argument verstellt.
3. `260810-0745` + `260810-0746` — die vier fehlenden Selektoren einordnen; die
   Frage nach dem Protokollschnitt hängt daran.
4. `260810-0748` — Messprogramm ablegen, damit die Zahl auf macOS 26 wieder
   fahrbar ist.
5. `260810-0750`, `260810-0751` — Kosmetik am Kopf.

---

## Anhang: das Messprogramm

Die Messungen dieser Durchsicht stammen aus einem ObjC-Programm, übersetzt mit
`clang -fobjc-arc -framework Cocoa`. Es gehört nicht in `crates/` und ist
deshalb hier abgelegt statt eingecheckt; `260810-0748` schlägt vor, eine
gepflegte Fassung unter `spikes/` abzulegen.

```objc
#import <Cocoa/Cocoa.h>
#import <objc/runtime.h>

int main(void) { @autoreleasepool {
    [NSApplication sharedApplication];

    // 1. Der Schnitt ueber das Protokoll — 14 Merkmale, eines mehr als EINSTELLUNGEN.
    Protocol *p = objc_getProtocol("NSTextInputTraits");
    unsigned int pn = 0;
    objc_property_t *props = protocol_copyPropertyList(p, &pn);
    for (unsigned int i = 0; i < pn; i++) printf("prop %s\n", property_getName(props[i]));
    free(props);

    // 2. Die Kopplung, je Paar in beide Richtungen (hier ein Paar von zehn).
    NSTextView *v = [[NSTextView alloc] initWithFrame:NSMakeRect(0,0,100,100)];
    [v setValue:@(NO) forKey:@"automaticQuoteSubstitutionEnabled"];
    printf("boolNO  -> smartQuotesType=%ld\n", (long)[[v valueForKey:@"smartQuotesType"] integerValue]);
    [v setValue:@(1) forKey:@"smartQuotesType"];           // NSTextInputTraitTypeNo
    printf("typeNO  -> bool=%d\n", [[v valueForKey:@"automaticQuoteSubstitutionEnabled"] boolValue]);

    // 3. Die dritte Tuer.
    NSTextView *w = [[NSTextView alloc] initWithFrame:NSMakeRect(0,0,100,100)];
    [w setRichText:NO];
    [w setAutomaticQuoteSubstitutionEnabled:NO];
    [w setAutomaticDashSubstitutionEnabled:NO];
    [w setAutomaticTextReplacementEnabled:NO];
    [w setAutomaticSpellingCorrectionEnabled:NO];
    [w setSmartInsertDeleteEnabled:NO];
    [w setEnabledTextCheckingTypes:NSTextCheckingAllTypes];
    printf("nach der dritten Tuer: quote=%d smartQuotesType=%ld\n",
           w.isAutomaticQuoteSubstitutionEnabled,
           (long)[[w valueForKey:@"smartQuotesType"] integerValue]);

    // 4. Die Oberklassen, die instance_methods() nicht sieht.
    for (Class c = class_getSuperclass([NSTextView class]); c; c = class_getSuperclass(c)) {
        unsigned int n = 0; Method *ms = class_copyMethodList(c, &n);
        for (unsigned int i = 0; i < n; i++) {
            NSString *s = NSStringFromSelector(method_getName(ms[i]));
            if ([s hasPrefix:@"set"] && ([s hasSuffix:@"Enabled:"] || [s hasSuffix:@"Type:"] || [s hasSuffix:@"Behavior:"]))
                printf("%s: %s\n", class_getName(c), s.UTF8String);
        }
        free(ms);
    }
    return 0;
} }
```

---

## Abgleichvermerk 260810-0805

**Alle sieben Befunde stehen offen**, als `issues/260810-0745` bis `260810-0751`: fünf mittlere, zwei niedrige, keiner am ausgeführten Code — so, wie der Bericht es sagt. Zwischen dem Bericht und diesem Abgleich hat sie niemand angefasst; sie sind die jüngste Fracht dieser Runde und gehen ungeteilt in die Zeit nach dem Abnahmelauf.

Die tragende Messung des Berichts, die zehn Paare `set…Type:` gegen `set…Enabled:`, ist im Abgleich nicht nachgemessen worden. Der Bericht sagt, er habe sie unabhängig geprüft; ein zweiter Durchgang durch dieselbe Laufzeitprobe hätte nichts hinzugefügt, was der Bericht nicht schon trägt. `issues/260810-0748` hält dagegen fest, dass die Kopplung im Baum durch nichts gehalten wird.

Am Bericht selbst ist nichts geändert.
