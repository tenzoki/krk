# Der Plan verbietet `y` und `z` und legt Rückgängig selbst auf `cmd+z`

---
**Domain:** planning
**Schwere:** Medium
**Gefunden von:** coder, bei der Umsetzung von S5 und S6
**Betroffen:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Befund 4, Frage 11 und das Abnahmekriterium von S6
**Cross-references:** `resources/default-keymap.toml`, `crates/krk-core/tests/belegung.rs`, `decisions/260808-0140_o_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

---

## Der Befund

Der Plan sagt an zwei Stellen, dass kein neuer Tastenbefehl dieser Runde auf `y`
oder `z` liegt, und legt an einer dritten Stelle zwei neue Kombinationen genau
dorthin.

**Befund 4, letzter Absatz:**

> **Kein neuer Tastenbefehl dieser Runde liegt auf `y` oder `z`.** … alle
> vorgeschlagenen Kombinationen unten benutzen `e`, `s`, `f`, `g`, `j` und `r`
> und sind davon nicht berührt.

**Abnahmekriterium von S6:**

> Eine Probe hält fest, dass die Datei 71 Funktionen führt und dass keine
> Tastenliste die Taste `y` oder `z` neu belegt.

**Die Tabelle in Frage 11**, dieselbe Tabelle, auf die sich der Satz aus Befund 4
mit "alle vorgeschlagenen Kombinationen unten" beruft, führt zwei Zeilen, die er
übersieht:

| `text_rueckgaengig` | `cmd+z`, `gehalten_von = "menue"` |
| `text_wiederholen` | `shift+cmd+z`, `gehalten_von = "menue"` |

Die Zählung "71 Funktionen" im selben Abnahmekriterium schließt beide ein. Ohne
sie sind es 69, und ohne sie hat der Editor kein Rückgängig — was S7 und die
Ableitung darüber ("ein Editor, den niemand benutzt") ausdrücklich ausschließen.

## Warum das zählt

Die drei Aussagen sind zusammen nicht erfüllbar. Wer S5 und S6 umsetzt, muss
zwischen ihnen wählen, und die Wahl ist nicht folgenlos: sie entscheidet, ob der
eingebaute Editor ein Rückgängig hat.

Der Grund für das Verbot trägt bei den beiden Menüeinträgen nicht. KRK belegt
nach C3 der Runde 1 den **virtuellen Tastencode**, also die Stelle auf der
Tastatur, und `kVK_ANSI_Y` und `kVK_ANSI_Z` tauschen zwischen der deutschen und
der amerikanischen Belegung den Platz. Ein Menükürzel schlägt dagegen über das
**Zeichen** nach: `NSMenuItem.keyEquivalent` nimmt eine Zeichenkette entgegen.
Befund 4 selbst führt das aus und nennt es den Grund, aus dem `cmd+c` und
`cmd+v` auf jeder Tastaturbelegung an der beschrifteten Stelle wirken. `cmd+z`
tut es aus demselben Grund.

Der Schnitt, der die drei Aussagen versöhnt, ist damit nicht "y und z sind
verboten", sondern: **keine Kombination, die KRK selbst über den Tastencode
zustellt, liegt auf einer der beiden wandernden Stellen.** Das ist eine
trennscharfe Grenze entlang des Zustellers, den die Belegung ohnehin führt
(`gehalten_von`), und kein Sonderfall für zwei Einträge.

## Was in S5 und S6 daraus geworden ist

Umgesetzt ist die Tabelle aus Frage 11, samt `cmd+z` und `shift+cmd+z`. Die
Zahl 71 stimmt damit, S7 ist nicht blockiert, und der Editor hat ein
Rückgängig. Zwei Proben tragen die genauere Regel:

- `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen`
  (`crates/krk-core/tests/belegung.rs`) prüft die elf Editor-Funktionen gegen
  beide Stellen und nimmt die vom Menü gehaltenen Funktionen mit Begründung aus.
- `die_y_kuerzel_liegen_auf_kvk_ansi_y_und_die_stelle_kvk_ansi_z_ist_unbelegt`
  hat dieselbe Ausnahme bekommen. Ihre zweite Aussage lautet seither: die Stelle
  `kVK_ANSI_Z` trägt keine Funktion, **die der Ereignisabgriff zustellt**. Der
  Nachschlag überspringt zugestellte Funktionen ohnehin
  (`Belegung::nachschlag`), die Probe blieb also in der Sache unverändert.

Der offene Datensatz zu den y-Tasten ist davon unberührt: er handelt von
`cmd+y` und `shift+cmd+y`, und die stellt KRK selbst über den Tastencode zu.

## Was zu tun ist

Eine Entscheidung des Nutzers, und danach eine Textänderung im Plan.

1. **Bestätigen oder umstoßen**, dass die Regel dem Zusteller folgt und nicht
   dem Buchstaben. Wer sie umstößt, nimmt in Kauf, dass der Editor kein
   Rückgängig über die Tastatur hat; ein anderer Buchstabe als `z` ist dafür
   keine Antwort, weil das Kürzel am `NSMenuItem` hängt und `cmd+z` der
   Mac-Standard ist.
2. Bei Bestätigung: den Satz in Befund 4 und das Abnahmekriterium von S6 um den
   Halbsatz ergänzen, der die Regel auf das einschränkt, was KRK selbst über den
   Tastencode zustellt. Zwei Sätze, keine Änderung an Frage 11.

---
Resolved: Am 260810-0822 geschlossen, aber nicht auf dem hier vorgeschlagenen Weg. **Der Schnitt entlang des Zustellers trägt nicht.** Geprüft am Code: seine Voraussetzung, KRK belege nach C3 der Runde 1 den virtuellen Tastencode, war schon beim Schreiben dieses Datensatzes falsch. S2 hatte den dritten Weg gebaut, und `Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) liefert seither für jeden einbuchstabigen Namen eine Zeichenkennung. Daraus folgt zweierlei. Erstens ist der vorgeschlagene Schnitt am heutigen Code leer: `y` und `z` sind einbuchstabig, ihre Kennung ist nie ein Code, und die Menge "von KRK über den Tastencode zugestellt und auf einer der beiden Stellen" ist für jede denkbare Belegung leer. Ein Kriterium, das keine Belegung verletzen kann, misst nichts. Zweitens trennt die Grenze `gehalten_von` an dieser Frage nichts mehr, weil beide Zusteller Buchstaben über das Zeichen nachschlagen.

Was stattdessen gilt, sagt `260809-1746_*_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md`: die Einschränkung auf `y` und `z` entfällt ersatzlos, weil seit S2 keine Stelle mehr wandert. Nachgezogen sind der Satz in `### Befund 4` und das Abnahmekriterium von S6 im Plan; die Tabelle in `### Frage 11` bleibt unverändert, und der Editor behält sein Rückgängig auf `cmd+z`. Die Begründung im Einzelnen steht im `## Reconciliation Log` des Plans, Abschnitt `### 260810-0822`. Zwei Kommentare in `resources/default-keymap.toml` führen die alte Begründung weiter und sind als `260810-0011_*_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md` für den `ontocoder` geführt.

Eine Nutzerentscheidung, die dieser Datensatz unter "Was zu tun ist" verlangt hat, war nicht mehr nötig: die Alternative, die zur Wahl gestanden hätte, hätte den Editor sein Rückgängig gekostet, und sie stützte sich auf einen Grund, den es nicht mehr gibt.
