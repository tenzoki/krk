# KRK trägt eine Statusleiste mit Schaltern für die fünf Bereiche

---
**Domain:** code
**Status:** bounded
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_c_bereichsleiste-und-proportionale-breitenregel.md
**Active session history:** shared/history/260812-0306-orchestrator-session.md

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

- Turn 1 (Sitzung 260812-0306): Commits 5e17c9e..8ffaac2 — Schritte 1 bis 3 des Plans
  (proportionale Breitenregel, Abweisung an den Mindestbreiten, ausblendbares linkes
  Dateifenster). Ein Defekt der Runde selbst behoben (260812-0439), zwei neue abgelegt
  (260812-0512 sowie der Beifund 260812-0415 des Planers). Coherence-Urteil: Durchsicht lief
  noch. Sitzungsprotokoll: shared/history/260812-0306-orchestrator-session.md
- Turn 2 (Sitzung 260812-0306): Commit 90b02d4 — Schritte 4 bis 7 in **einem** Commit, weil der
  Baum zwischen ihnen nicht grün wird: Schritt 4 trägt fünf Funktionen in die Belegung ein, deren
  Kommandos erst Schritt 7 baut. Belegung auf 79 Funktionen, `Kommando` auf 73 Kennungen, `Spalte`
  als reine Aufzählung umgezogen, Spaltensichtbarkeit in Modell und Ablage. Zwei Defekte
  geschlossen (260812-0533, 260812-0548). Erste Durchsicht über 5aa22df..8ffaac2: vier Befunde,
  zwei mittel. Coherence-Urteil: offen bis Phase 3.
- Turn 3 (Sitzung 260812-0306): Commits 026c665, 0342445, 15d7bbe — die vier Befunde der ersten
  Durchsicht behoben (die zwei mittleren durch einen neuen Schnitt an der Rückrechnung, nicht
  durch Flicken), Schritt 8 gebaut, vierzehn Entscheidungen auf umgesetzt. `make bundle` läuft,
  `target/KRK.app` ist gebaut und signiert. Coherence-Urteil: offen bis Phase 3.
- Turn 4 (Sitzung 260812-0306): Commit caeaa18 — die vier Befunde der zweiten Durchsicht über
  8ffaac2..0342445. Drei im Code, einer im Text: drei Stellen sagten zu, die Spaltenbefehle
  stünden in der Markdown-Ausgabe, und das stand gegen den Nutzerentscheid vom 260811-0110.
  Berichtigt sind die Zusagen, nicht der Code. Coherence-Urteil: offen bis Phase 3.

## Parent grounding stale

**Festgestellt am:** 260811-2223
**Playmaker-Lauf:** 260811-2223-playmaker-direct-dispatch
**Beschränkt abgeschlossenes Kind:** `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`,
geschlossen am 260811-2210

Die Runde 4 hat eine harte Vorbedingung dieses Circles aufgelöst, und drei Stellen des Datensatzes
stehen seither auf einer Grundlage, die sich bewegt hat. Keine davon hält die Aktivierung auf; im
Gegenteil, der Wegfall der Vorbedingung ist der Grund, warum dieser Circle in diesem Lauf auf
Rang 1 steht. Alle drei gehören in die Klärungsrunde.

**Zur Auslösebedingung, offen benannt.** Die Regel verlangt, dass der Abschnitt
`## Grounding snapshot` des Elterndatensatzes den Verzeichnisnamen des abgeschlossenen Kindes oder
den in seiner `## Closure note` genannten Artefakt zitiert. Dieser Datensatz nennt weder das eine
noch das andere. Er zitiert den Defekt `shared/issues/260811-1245_*_…`, und die Runde 4 hat genau
diesen Defekt behoben; ihre `## Closure note` adressiert den Befund unter der Überschrift „Für die
Nachfolger" ausdrücklich an diesen Circle. Der Vermerk steht deshalb hier, obwohl die wörtliche
Bedingung nicht greift. Wer anders entscheidet, sieht an dieser Stelle, worauf.

### 1. Die Directive sagt eine Behebung zu, die bereits anderswo gefallen ist

Zeile 14 schließt mit: „der gemeldete Rückfall der Vorschaubreite beim Navigieren in der Dateiliste
ist mit dieser Runde behoben." Die Behebung liegt seit dem 260811-2130 im Baum, gefahren in der
Runde 4 unter dem Commit `1ea5a3d`. Gemessen war es die erste der beiden im Defektdatensatz
genannten Bruchstellen: `kommando_ausfuehren` rief `aufteilung_nachziehen()` nach jedem Befehl,
bevor jemand die gezogene Breite nachmaß. Die Behebung ist `bildschirmbreiten_uebernehmen()` am
Kopf von `kommando_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs:2048`, Funktion bei
`:2577`), am Baum gelesen im Abgleich
`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/history/260811-2157-reconciliation.md`.
Bruchstelle 2 trifft nicht zu, C7 der Runde 1 war also nie gebrochen.

Für diese Runde ist das eine Entlastung und keine Korrektur: die proportionale Regel entsteht auf
einer Grundlage, die die Ziehbewegung des Nutzers hält. Der Satz in der Directive beschreibt danach
einen Zustand, der schon eingetreten ist, und nicht mehr eine Leistung dieser Runde. Wer den Spec
schreibt, formuliert ihn um.

### 2. Die siebte offene Frage ist gegenstandslos geworden

Der Datensatz
`decisions/260811-1305_*_wird-der-vorschaubreiten-defekt-in-dieser-runde-behoben.md` steht seit dem
260811 auf beantwortet, und geantwortet hat die Lage und nicht der Nutzer: der Defekt ist in einer
eigenen Runde vor diesem Circle gefallen, im Ergebnis Möglichkeit 2 seiner drei Möglichkeiten. Zwei
Stellen dieses Datensatzes kennen das noch nicht. Zeile 68 zählt sieben offene Fragen, offen sind
sechs. Zeile 77 führt die Frage in der Tabelle unter ihrem damaligen Marker `_o_`. Der Playmaker
ändert keine Zitate und keine Zählung; beide Zeilen bleiben, wie sie stehen.

Ob der beantwortete Datensatz bei der Aktivierung auf umgesetzt (`_i_`) geht oder als überholt
(`_s_`) gilt, entscheidet, wer den Circle aktiviert. Er selbst hält das ausdrücklich offen.

### 3. Ein Beifund der Runde 4 trifft die Bereichsschalter unmittelbar

`MINDESTGROESSE` (`crates/krk-ui/src/appkit/fenster.rs`) steht auf 780 Punkten und deckt damit die
vier Bereiche der Runde 1. Der Editor braucht 320 statt der 160 Punkte der Vorschau, der Fünfersatz
summiert sich auf 920. Zwischen 780 und 920 Punkten Fensterbreite wird der Editor unter sein
Mindestmaß gedrückt.

Der Befund fällt mit der sechsten offenen Frage dieses Circles zusammen, was geschieht, wenn die
Mindestbreiten nicht mehr hineinpassen
(`decisions/260811-1305_*_was-geschieht-wenn-die-mindestbreiten-nicht-mehr-hineinpassen.md`). Der
Datensatz jener Frage kennt die Zahl 920 nicht; sie ist erst am 260811-2130 bei der Behebung
aufgefallen. Wer die Frage beantwortet, rechnet mit ihr.

## Activation proposal

**Vorgeschlagen am:** 260811-2223
**Playmaker-Lauf:** 260811-2223-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** sofort, nach einer Klärungsrunde über den Zuschnitt

Dieser Circle ist der empfohlene nächste Kandidat und steht auf Rang 1 von zwei. Der zweite ist
`260804-0933-eingebauter-web-betrachter-im-vorschaufenster`. Die Rangfolge kehrt die der beiden
Läufe vom 260811-1326 und 260811-1415 um, in denen dieser Circle hinter der Runde 4 auf Rang 2
stand; die Runde 4 ist geschlossen, und damit ist das Feld auf zwei geschrumpft.

**Der Ausschlag ist der Wegfall der harten Vorbedingung.** Der Defekt der Vorschaubreite ist am
260811 in der Runde 4 gemessen und behoben worden, und der Abschnitt `## Parent grounding stale`
oben schlüsselt auf, was das für diesen Datensatz heißt. Damit entfällt der Grund, aus dem dieser
Circle in beiden früheren Läufen hinter einem anderen stand. Die proportionale Regel entsteht auf
einer Grundlage, die die Ziehbewegung des Nutzers hält, und der Messschritt, den der Defektdatensatz
verlangte, ist gefahren.

**Die Grundlage ist am Baum erhoben, und die tragende Stelle ist eine einzige Funktion.**
`bereichsbreiten(verfuegbar, breiten, sichtbar)` (`crates/krk-ui/src/fenstermodell.rs:609`) ist die
eine Breitenregel. Sie ist reines Rust ohne AppKit und damit ohne Fenster prüfbar; `aufteilung.rs`
setzt nur um, was dort herauskommt. Eine Runde, die die Regel neu fasst, kann ihre Arbeit an dieser
Funktion und ihren Proben abnehmen, ohne KRK im Vordergrund zu brauchen. Das unterscheidet sie von
den vier gefahrenen Runden, deren Abnahmelauf sämtlich am Vordergrund hängt, und es ist das
stärkste Einzelsignal für diesen Circle.

**Gegen die Empfehlung spricht der Zuschnitt, und er ist seit dem Anlegen gewachsen.** Sechs
Entscheidungsdatensätze in `decisions/` dieses Circles tragen `_o_`, und die erste von ihnen,
`260811-1305_*_was-heisst-proportional-zur-letzten-aufteilung.md`, bestimmt den Umfang der ganzen
Runde: sie fragt, ob die eine Breitenregel neu geschrieben wird und was aus der Vorrangordnung vom
260808 wird, nach der die Lesezeichenleiste vor dem Editor nicht weicht. Dazu kommt seit dem
260811-1732 eine Erweiterung, die kein Defekt ist, sondern ein Nachtrag des Nutzers:
`issues/260811-1732_*_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md` verlangt
neben den fünf Bereichsschaltern drei Schalter für die Spalten Größe, Datum und Typ. Der Nachtrag
selbst hält fest, warum er mehr ist als eine längere Liste: ein Bereichsschalter ändert die
Aufteilung und löst die proportionale Neuverteilung aus, ein Spaltenschalter ändert den Inhalt
beider Dateifenster und die Aufteilung gar nicht. Er wirft vier eigene Fragen auf, darunter die
einzige mit Folgen über das Verbergen hinaus: was mit der Sortierung geschieht, wenn die Spalte
weggeschaltet wird, nach der sortiert ist. Ob beide Sorten in eine Runde gehören, ist selbst eine
Frage und gehört an den Anfang der Klärungsrunde.

**Der Zählwert der offenen Entscheidungen spricht gegen diesen Circle, und er misst hier wie schon
zweimal zuvor die falsche Größe.** Sechs offene Datensätze gegen einen beim Web-Betrachter ist für
die Gewichtung `code` ein schlechter Wert. Die sechs sind Zuschnittfragen: jede führt ihre
Möglichkeiten samt Folgen, jede ist in einer Klärungsrunde mit dem Nutzer beantwortbar, und keine
verlangt eine Messung oder eine Untersuchung. Der eine Datensatz beim Web-Betrachter ist eine
ungemessene technische Frage zur Verfügbarkeitsprüfung für macOS-26-Schnittstellen, und derselbe
Circle hält daneben fest, dass auch das Mittel der Darstellung von Web-Inhalt offen ist und „in eine
eigene Untersuchung vor dem Plan" gehört. Eine Untersuchung vor dem Plan ist teurer als sechs
Fragen an den Nutzer.

**Zur Abhängigkeitslage, die bei diesem Projekt nichts mehr unterscheidet.** Beide Abhängigkeiten
dieses Circles sind beschränkt abgeschlossen (`_b_`) und nicht kohärent (`_c_`), also trägt er nach
der Rangheuristik das Kennzeichen der unerfüllten Vorbedingung. Der andere Kandidat trägt es
ebenso, und die vier gefahrenen Runden sind sämtlich `_b_`, sämtlich aus demselben Grund: der
Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Das Kennzeichen unterscheidet in diesem Projekt keine zwei Kandidaten mehr; die Rangfolge entsteht
aus den übrigen Signalen. Inhaltlich tragen beide Bindungen hier leicht: die Runde 1 stellt C7 mit
den gesicherten Breiten, C1 mit der Statuszeile und C8 mit den zehn Zeitzusagen, die Editor-Runde
den fünften Bereich und den gegenseitigen Ausschluss von Vorschau und Editor. Keine der beiden
Beschränkungen liegt auf einem Bauteil, das diese Runde anfasst.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Closure note

**Geschlossen am:** 260812-0820 als **beschränkter Abschluss** (`_b_`)
**Sitzungsprotokoll:** `shared/history/260812-0306-orchestrator-session.md`
**Abgleich:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0801-reconciliation.md`
**Urteil der drei Kanten:** `bounded-closure-proposed`, Abschnitt `## Coherence` des Sitzungsprotokolls

### Warum beschränkt und nicht kohärent

Aus demselben Grund wie die vier Runden davor, und es ist eine Eigenschaft dieses Projekts und
keine Häufung von Fehlschlägen: dreizehn Abnahmekriterien dieser Runde (C1.1, C1.2, C1.4, C2.1
bis C2.5, C3.1, C3.2, C3.4, C5.1, C6.3) sind nur am laufenden `KRK.app` im Vordergrund zu sehen,
und kein Agent kann ihn fahren
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Was ein Agent abnehmen kann, ist abgenommen: 27 Kriterien einzeln gegen den Baum gelesen,
`make check` mit Exit 0 über vierzehn Prüfziele, `make bundle` gebaut und signiert. „Gebaut" ist
damit die richtige Aussage über diese Runde und „abgenommen" nicht.

### Was die Runde gebaut hat

Elf Commits zwischen `6b6ea3c` und dem Abschluss. Die Breitenregel verteilt Anteile statt
Punktzahlen an alle fünf Bereiche; das linke Dateifenster ist ausblendbar, solange eines bleibt;
ein Bereich, dessen Mindestbreite nicht mehr hineinpasst, geht nicht auf; am Fensterfuß steht die
`Bereichsleiste` mit acht Ankreuzfeldern, fünf für die Bereiche und drei für die Spalten Größe,
Datum und Typ. `Kommando` ist von 68 auf 73 Kennungen gewachsen, die Auslieferungsbelegung von
74 auf 79 Funktionen.

Vierzehn Entscheidungsfragen sind beantwortet und umgesetzt, zwölf Defekte geschlossen, davon
acht in dieser Runde selbst gefunden. Zwei Durchsichten decken den Codebereich
`5aa22df..0342445` lückenlos.

### Der Bounded-Closure-Artefakt: was gelernt wurde, das die Directive nicht erreicht hat

**Die Anteilsregel hat eine Nutzerfestlegung überstimmt, und das ist die wichtigste Einzelfolge
dieser Runde.** Die Regel vom 260808 — die Lesezeichenleiste weicht dem Editor nicht — trägt
unter der Anteilsregel nicht mehr. Der Orchestrator hat sie in der Klärungsrunde autonom fallen
lassen, gedeckt durch die Weisung „mache autonom", und hat das zunächst mit zwei Gründen
begründet, von denen der Abgleich einen widerlegt hat: die Frage „wer weicht, wenn es eng wird"
löst sich **nicht** auf, sie wird nur anders beantwortet (die Mindestbreite entscheidet statt der
Reihenfolge in `Bereich::ALLE`). Tragfähig bleibt der erste Grund allein. Der Nutzer kann das
umstoßen; es kostet `bereichsbreiten` samt Proben ein zweites Mal.

**Die Naht zwischen gespeicherter Zahl und Bildschirmpunkt ist die verletzlichste Stelle dieser
Runde.** Drei der acht in ihr gefundenen Defekte sitzen dort, und die erste Durchsicht hat zwei
davon gefunden, die niemand vermutet hatte. Der tragende Schnitt heißt jetzt: vom Schirm wird nur
zurückgelesen, was die Regel nicht selbst ausgelegt hat. Ob AppKit die Rahmen nach dem Auslegen
unverändert stehen lässt, ist ohne Fenster nicht prüfbar und bleibt die eine ungemessene Annahme
darunter.

**Vier Defekte bleiben offen**, jeder mit einem benannten Weg: der Breitenschritt neben einem
gedeckelten Bereich (`260812-0700`), F4 am schmalen Fenster (`260812-0512`), `Spalte` und
`Schluessel` als zwei Aufzählungen derselben vier Dinge (`260812-0415`), die Zahl 39 im Kopf der
Belegungsdatei (`260812-0810`), dazu zwei Modulköpfe mit einem toten Verweis (`260812-0801`).
Keiner hält den Abschluss auf, keiner ist ein Betriebsfehler.

### Für die Nachfolger

Wer den Abnahmelauf fährt, sieht als Erstes nach, ob die acht Schalter bei 780 Punkten
Fensterbreite nebeneinander passen — überschlagen sind es rund 540 Punkte, gerechnet und nicht
gemessen. Danach, ob `refusesFirstResponder` den Ersthelferrang bei eingeschalteter vollständiger
Tastaturbedienung wirklich fernhält; steht das nicht, gibt `ersthelferbereich` eine falsche
Auskunft über den Fokus. Und L9 aus C8 ist nachzumessen: die Leiste nimmt der Fensterzeile 18
Punkte Höhe, und L9 liegt auf dem Weg des Bildaufbaus. Diese Runde setzt keine neue Zahl.

