# KRK trägt eine Statusleiste mit Schaltern für die fünf Bereiche

---
**Domain:** code
**Status:** anticipated
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** (none yet)
**Active session history:** (none yet)

---

## Directive

KRK trägt am unteren Fensterrand eine Leiste über die volle Fensterbreite, die für jeden der fünf Bereiche der Fensterzeile einen Schalter führt: Lesezeichen- und Geräteleiste, linkes Dateifenster, rechtes Dateifenster, Vorschau und Editor. Jeder Schalter zeigt an, ob sein Bereich steht, und schaltet ihn per Mausklick oder über die Tastatur um. Jede Änderung der Sichtbarkeit teilt die Fensterzeile neu auf, und zwar proportional zur zuletzt sichtbaren Aufteilung: zwei Bereiche, die im Verhältnis 2:1 zueinander standen, stehen nach dem Einblenden eines dritten weiterhin in diesem Verhältnis. Die Breiten, aus denen dieses Verhältnis entsteht, halten dabei die Ziehbewegung des Nutzers, und der gemeldete Rückfall der Vorschaubreite beim Navigieren in der Dateiliste ist mit dieser Runde behoben.

## Grounding snapshot

Vorläufig. Ein vorgesehener Circle trägt noch keine erhobene Grundlage; dieser Abschnitt hält fest, was beim Lesen des Baums am 260811-1304 sichtbar war, und wird bei der Aktivierung ersetzt.

### Woher das Vorhaben kommt

Der Nutzer hat den Entwurf am 260811 diktiert, im selben Zug mit dem Defekt `shared/issues/260811-1245_o_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`, den er am 260811-1240 gemeldet hat. Der Defekt und der Entwurf liegen in derselben Maschinerie, und der Defektdatensatz sagt das von sich aus: eine proportionale Neuaufteilung auf einer Grundlage, die die Ziehbewegung des Nutzers nicht hält, verteilt die falschen Anteile.

### Was schon steht, am Code geprüft am 260811-1304

**Die Fensterzeile trägt seit der Editor-Runde fünf Bereiche**, je einer in einem `NSBox`, und `crates/krk-ui/src/appkit/aufteilung.rs` legt sie aus. `Bereich` (`crates/krk-ui/src/fenstermodell.rs:62`) zählt sie auf, und die Aufzählung ist vollständig ohne Auffangzweig: ein sechster Bereich hält den Bau an.

**Die Breitenregel steht einmal**, in `crates/krk-ui/src/fenstermodell.rs:609`, `bereichsbreiten(verfuegbar, breiten, sichtbar)`. Sie ist reines Rust ohne AppKit und damit ohne Fenster prüfbar; `aufteilung.rs` setzt nur um, was dort herauskommt, und ruft sie an zwei Stellen: nach einer Änderung von Breite oder Sichtbarkeit im Fenstermodell, und wenn AppKit die Bereiche neu auslegen lässt.

**Die heutige Regel ist nicht proportional, und das ist der zentrale Befund für diese Runde.** Sie arbeitet in zwei Stufen:

```
verfügbare Breite
   │
   ├─ feste Bereiche in der Reihenfolge Lesezeichen → Vorschau → Editor:
   │     jeder bekommt seine gespeicherte Breite in Punkten,
   │     gedeckelt auf das, was den Dateifenstern ihr Mindestmaß lässt
   │
   └─ Rest an die sichtbaren Dateifenster,
         im Verhältnis ihrer gespeicherten Breiten
```

Ein Verhältnis gilt heute also allein zwischen den beiden Dateifenstern. Die drei festen Bereiche tragen absolute Punktzahlen, und wer in der Reihenfolge von `Bereich::ALLE` vorn steht, behält seine Wunschbreite, wenn es eng wird. Der Dokumentationskommentar an `bereichsbreiten` benennt das als Zusage und nicht als Zufall: die Lesezeichenleiste steht vor dem Editor, also weicht sie nicht, wenn beide zugleich stehen, und die Dateifenster rücken zusammen. Diese Vorrangordnung ist eine Festlegung des Nutzers vom 260808.

Daraus folgt: **der Entwurf verlangt eine neue Fassung der einen Breitenregel und nicht eine Ergänzung daneben.** Sein Beispiel, zwei Bereiche im Verhältnis 2:1 behalten dieses Verhältnis beim Einblenden eines dritten, trifft heute nur zu, wenn beide Bereiche Dateifenster sind. Steht ein fester Bereich darunter, ändert sich das Verhältnis, weil der feste seine Punktzahl behält und der bewegliche allein abgibt. Was mit der Festlegung vom 260808 geschieht, ist die erste der offenen Fragen unten.

**`Breiten` und `Sichtbarkeit`** (`crates/krk-core/src/ablage/sitzung.rs:181` und `:211`) liegen in `session.toml`. `Breiten` führt fünf `Option<f64>`, eines je Bereich; ein fehlender Wert fällt auf `Bereich::anfangsbreite()` zurück. `Sichtbarkeit` führt vier `bool` und **kein Feld für das linke Dateifenster**. Der Kommentar dort begründet die Lücke: C7 lässt immer mindestens ein Dateifenster stehen, und ein Feld, das nie `false` werden darf, wäre eine Zusage, die niemand einhält. `Fenstermodell::umschalten` weist `Bereich::Links` mit `return false` ab, und `sichtbar_in` liefert für ihn fest `true`.

**Vorschau und Editor teilen sich zeitlich eine Fläche.** `Bereich::teilt_flaeche_mit` ist die eine Stelle des gegenseitigen Ausschlusses aus C1 der Editor-Runde: wird einer der beiden sichtbar, weicht der andere. Zwei der fünf Schalter können damit nie zugleich an sein.

**Mindestbreiten gibt es**, `Bereich::mindestbreite`: Lesezeichen 120, Dateifenster je 240, Vorschau 160, Editor 320. Sie sind an der Fläche gerechnet und nicht am Inhalt, weil der Rahmen aus C9 jedem Randbereich vier Punkte Inhaltsbreite nimmt. Was geschieht, wenn ihre Summe die Fensterbreite übersteigt, steht heute im Code und nirgends als Regel: die festen Bereiche werden auf `(rest − Mindestmaß der Dateifenster).max(0.0)` gedeckelt und können bei `0.0` landen, während sie als sichtbar gelten.

**Die Statuszeile aus C1 der Runde 1 sitzt nicht dort, wo die neue Leiste hin soll.** `crates/krk-ui/src/appkit/statuszeile.rs` beschreibt sie als "die Statuszeile am Fuß eines Dateifensters", und `aufteilung.rs:379` legt sie je Dateifenster innerhalb des Bereichs an. Es gibt also zwei davon und keine über die Fensterbreite. Sie trägt fünf Ränge: Befehlsantwort, Vorgangsanzeige, Fenstermeldung, Tabmeldung, Markierungsstand. Eine neue Leiste über die volle Breite ist damit eine **zweite Fläche und nicht dieselbe**; ob sie auch Meldungen trägt, entscheidet, ob C1 gebrochen wird.

**Das Ein- und Ausblenden per Tastatur gibt es schon.** `resources/default-keymap.toml` führt `leiste_umschalten`, `zweites_fenster_umschalten` und `vorschau_umschalten`; alle drei tragen den Wirkungsbereich `Ueberall`. Für den Editor gibt es kein Gegenstück mit diesem Namen, sondern die Einstiege `bearbeiten` und `editor_aus_vorschau` sowie `editor_schliessen`. Die Tastaturhälfte des Entwurfs ist damit zu drei Vierteln gebaut, und was fehlt, ist ein Umschalter für den Editor und die Frage nach dem linken Dateifenster.

**Mausbedienung ist im Baum vorhanden, aber schmal.** `crates/krk-ui/src/appkit/vorschau.rs:121` überschreibt `mouseDown:`, und `belegungsansicht.rs` baut zwei `NSButton` über `buttonWithTitle_target_action`. Ein anklickbares Bedienelement in der Fensterzeile selbst gibt es heute nicht.

**Der Fokus kennt fünf Werte** (`crates/krk-ui/src/kommandos/fokus.rs:75`), und jeder Wechsel geht durch die Überschreibung von `makeFirstResponder:` in `appkit/fenster.rs`. `fokusanzeige_nachziehen` schreibt dort ausschließlich die fünf Rahmenfarben und den Fenstertitel und ruft weder `anwenden` noch `setHidden`; der Grund steht im Modulkopf und ist eine Falle, die schon eine Sitzung gekostet hat. Ob die neue Leiste einen sechsten Fokuswert bekommt, hängt an der Antwort auf Frage 2 unten.

### Die vier Aufzählungen, die der Bau anhält

Wer diese Runde plant, rechnet mit Übersetzerfehlern statt mit stiller Aufnahme. Vier vollständige Fallunterscheidungen ohne Auffangzweig sind betroffen, sobald ein Bereich, ein Kommando oder ein Fokuswert dazukommt: `Wirkungsbereich` und `Kommando` (`crates/krk-core/src/tasten/belegung.rs`), `Bereich` (`crates/krk-ui/src/fenstermodell.rs`) und `Fokus` (`crates/krk-ui/src/kommandos/fokus.rs`). Der Übersetzer nennt die Stellen genauer als jede Aufzählung hier.

### Offene Fragen

Sieben Fragen sind bei der Klärung aufgekommen und liegen als eigene Datensätze in `decisions/` dieses Circles, nicht in diesem Abschnitt. Sie sind die Eingabe für die Klärungsrunde bei der Aktivierung; jede trägt ihre Möglichkeiten samt Folgen und, wo es eine gibt, eine Empfehlung.

| Datei in `decisions/` | Worum es geht |
|---|---|
| `260811-1305_o_was-heisst-proportional-zur-letzten-aufteilung.md` | Ob die eine Breitenregel neu geschrieben wird und was aus der Vorrangordnung vom 260808 wird. Die entscheidende Frage: sie bestimmt den Umfang der Runde. |
| `260811-1305_o_traegt-das-linke-dateifenster-einen-schalter.md` | Ob die Leiste vier oder fünf Schalter trägt, und ob `Sichtbarkeit` ein fünftes Feld bekommt. |
| `260811-1305_o_wie-zeigen-zwei-schalter-eine-flaeche-die-nur-einer-haben-kann.md` | Wie die Leiste den gegenseitigen Ausschluss von Vorschau und Editor zeigt. |
| `260811-1305_o_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md` | Ob die neue Fläche auch Meldungen trägt und damit C1 der Runde 1 anfasst. |
| `260811-1305_o_welchen-anteil-bekommt-ein-bereich-der-noch-nie-sichtbar-war.md` | Woraus der erste Anteil eines nie sichtbaren Bereichs entsteht. Entfällt, wenn die erste Frage auf die heutige Regel fällt. |
| `260811-1305_o_was-geschieht-wenn-die-mindestbreiten-nicht-mehr-hineinpassen.md` | Was ein Schalter tut, dessen Bereich nicht mehr hineinpasst. |
| `260811-1305_o_wird-der-vorschaubreiten-defekt-in-dieser-runde-behoben.md` | Ob der gemeldete Defekt in dieser Runde oder in einer eigenen davor behoben wird. |

### Was diese Runde nicht festlegt

Womit die Schalter gezeichnet werden, ist offen und gehört in den Plan. Der Circle legt kein Bedienelement fest, weder `NSButton` noch `NSSegmentedControl` noch eine eigene Ansicht.

## Dependencies

`260802-0842-krk-mac-dateimanager-editor-git` — die Runde 1, beschränkt abgeschlossen. Sie bindet an drei Stellen. **C7** ist die Fähigkeit, die diese Runde fortschreibt: sie sagt zu, dass Breiten und Sichtbarkeit einen Neustart überleben, dass das Wiedereinblenden die vorherige Breite wiederherstellt und dass mindestens ein Dateifenster stehen bleibt. Die dritte Zusage ist der Grund, warum `Sichtbarkeit` kein Feld für das linke Dateifenster führt, und damit die Ursache der zweiten offenen Frage unten. **C1** legt die Statuszeile als den einen Weg fest, auf dem KRK dem Nutzer eine laufende Meldung zeigt; eine zweite Meldefläche wäre ein Bruch damit. **C8** führt zehn Zeitzusagen, darunter L9 zum ersten Bild; eine zusätzliche Leiste am Fensterfuß liegt auf dem Weg des Bildaufbaus.

`260807-2116-eingebauter-editor-mit-textmarken` — die Editor-Runde, beschränkt abgeschlossen. Ihr **C1** trägt den gegenseitigen Ausschluss von Vorschau und Editor und damit die dritte offene Frage. Sie hat außerdem den fünften Bereich, den fünften Fokuswert und die fünf Rahmen gebaut, auf denen diese Runde aufsetzt.

`shared/issues/260811-1245_o_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md` — der Defekt in derselben Maschinerie. Ob er in dieser Runde oder vor ihr behoben wird, ist die vierte offene Frage; der Datensatz selbst benennt die zwei möglichen Bruchstellen und verlangt, dass zuerst gemessen wird, welche es ist.

`260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — vorgesehen, nicht gefahren. **Er bindet diese Runde nicht.** Der Betrachter lebt in einem Tab des Vorschaufensters und fasst weder die Breitenregel noch die Sichtbarkeit an. Eine Berührung bleibt und ist klein: ein gerenderter Web-Inhalt braucht plausibel mehr als die 160 Punkte Mindestbreite, die die Vorschau heute trägt. Wer jenen Circle aktiviert, prüft die Zahl; wer diesen plant, muss nicht auf ihn warten.

## Turn log

(noch keiner)
