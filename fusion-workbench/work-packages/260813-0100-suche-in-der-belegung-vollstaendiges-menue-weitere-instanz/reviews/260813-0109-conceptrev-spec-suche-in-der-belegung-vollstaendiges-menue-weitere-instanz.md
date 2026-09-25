# Concept Evaluation: Spec Suche in der Belegungsansicht, vollständiges Menü, weitere Instanz

**Date:** 2026-08-13 01:09
**Target:** `fusion-workbench/shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md`
**Verdict:** tangled
**Diagrams evaluated:** 2  |  **Validation:** by-tool (mmdc 11.16.0 über `npx`, beide Blöcke nach SVG und PNG gerendert, beide Bilder angesehen)

## Spruch

Beide Bilder sind metrisch sauber und beide widersprechen an ihrer tragenden Stelle dem Text, den sie bebildern. Der Spruch **tangled** kommt nicht aus der Dichte: 14 Knoten mit 16 Kanten im ersten Diagramm, 8 Knoten mit 8 Kanten im zweiten, keine Zyklen, kein Gott-Knoten, keine Waisen, beide Typen richtig gewählt, beide parsen. Er kommt aus zwei Knoten, und beide tragen die Zusage ihrer Fähigkeit.

Im ersten Diagramm behauptet der Knoten `ZUL2` ("dieselbe Zulässigkeitsfrage") eine Nämlichkeit, die der Graph zwei Zeilen darüber selbst widerlegt: er zeichnet `V` ("Fokusvorbehalt") und `ZUL` ("Zulässig hier? (Blatt, Fokus)") als **zwei** Rauten mit zwei verschiedenen Fragen, und drei Kanten münden in `APP`, von denen nur eine durch `ZUL` gekommen ist. Auf den beiden anderen Wegen kann `ZUL2` mit "ja" antworten, obwohl der Abgriff die Taste eben an AppKit abgegeben hat. Der Satz unter dem Bild, eine durchgereichte Taste könne im Menü nichts auslösen, gilt für ein Drittel der Eingänge. Das Abnahmekriterium C2.6 verlangt genau den Fall, den die Regel aus C2.5 nicht abdeckt, und wir haben ihn am Baum bis zum konkreten Gegenbeispiel verfolgt.

Im zweiten Diagramm trägt ein einziger Knoten `SP` ("Sperre über dem Ablageordner") zwei verschiedene Mechanismen, und die Aufschrift des Kastens `I1` macht den Widerspruch sichtbar: "Instanz 1 (hält die Sperre seit dem Start)". Hielte Instanz 1 diese Sperre dauerhaft, könnte die Kante `L2 --> SP` nicht bestehen, denn Instanz 2 schriebe nie. Hält sie sie nicht dauerhaft, verliert die Zuständigkeit für die Sitzung ihr Merkmal, das der Entscheidungsdatensatz ausdrücklich als das entscheidbare gegen die unentscheidbare Frage gesetzt hat. Ein Wort, zwei Mechanismen, ein Knoten.

Wir halten fest, dass in beiden Fällen der Entwurf und nicht die Zeichnung nachzuziehen ist. Die Bilder sind an diesen zwei Stellen ehrlicher als die Prosa: sie zeigen die Lücke, die der Text zudeckt.

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Spruch |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `flowchart TD` | 14 | 16 | 1,14 | 3 (`F`) | 3 (`APP`) | 0 | nein, keine `subgraph`-Blöcke | 0 | tangled |
| 2 | `flowchart LR` | 8 (+2 Kästen) | 8 | 0,89 | 4 (`SP`) | 3 (`SP`) | 0 | ja, zwei `subgraph`-Blöcke | 0 | tangled |

Diagramm 1 hat eine Quelle (`E`) und vier Senken (`Z`, `S`, `GRAU`, `AUS`); alle 14 Knoten sind von `E` aus erreichbar. Unbeschriftet sind 5 von 16 Kanten: `E --> A`, `A --> F`, `APP --> M`, `M --> ZUL2`, `KLICK --> AUS`.

Diagramm 2 hat vier Quellen (`S1`, `L1`, `L2` und der Kasten `I2`) und vier Senken, die vier Ablagedateien. Unbeschriftet sind 4 von 8 Kanten, nämlich alle vier von `SP` zu den Dateien.

## Befunde

### 1. `ZUL2` behauptet eine Nämlichkeit, die der Graph selbst widerlegt (substanziell, Diagramm 1)

Drei Kanten münden in `APP` ("unverändert an AppKit"), und nur eine von ihnen kommt aus `ZUL`:

```
V   --|Ersthelfer gehört AppKit|--> APP     ZUL wurde nie gefragt
N   --|kein Treffer|-------------> APP     ZUL wurde nie gefragt
ZUL --|nein|---------------------> APP     ZUL hat "nein" gesagt
                                    │
                                    v
                                   M --> ZUL2 --|ja|--> KLICK --> AUS
```

Die Prosa unter dem Bild sagt: "Weil beide dieselbe Antwort bekommen, kann eine Taste, die der Abgriff durchreicht, im Menü nichts auslösen." Für den dritten Eingang stimmt das. Für den ersten stimmt es nicht, und der erste ist der gefährliche.

Der Fokusvorbehalt fragt, wem die Taste gehört. `fokus::wirkt` fragt, ob der Wirkungsbereich zum Fokuswert passt. `CLAUDE.md` führt diesen Unterschied bereits als eine der Eigenschaften, die schon eine Sitzung gekostet haben, und das Diagramm zeichnet ihn korrekt als zwei Rauten. Die Beschriftung von `ZUL2` nimmt ihn wieder zurück.

Das Gegenbeispiel ist am Baum nachgesehen und keine Herleitung. Beim Umbenennen direkt in der Liste hält der Feldeditor eines `NSTextField` der Namensspalte den Ersthelferrang (`crates/krk-ui/src/appkit/tabelle.rs:2342`, `feld.setEditable(true)`, dort als "das Umbenennen 'direkt in der Liste' aus C4" bezeichnet). Der Fokusvorbehalt reicht jeden Tastendruck dann unverändert an AppKit weiter (`crates/krk-ui/src/appkit/ereignisse.rs:29`). Es steht dabei **kein** Blatt, C2.7 greift also nicht. Und `fokus()` antwortet für diesen Feldeditor `Fokus::Dateifenster`; der Doc-Kommentar sagt es ausdrücklich (`crates/krk-ui/src/appkit/anwendung.rs:3528`: "Für den Feldeditor eines Textfeldes im Dateifenster lautet die Antwort vorher wie nachher `Dateifenster`"). Damit liefert `wirkt` für jeden Befehl mit `Wirkungsbereich::Ueberall`, `Dateifenster`, `Navigator` oder `Tabbereich` ein `true` (`crates/krk-ui/src/kommandos/fokus.rs:334-353`). Nach der Regel aus C2.5 ist der Eintrag also freigegeben, und `resources/default-keymap.toml` bindet unter anderem `up`, `down`, `return`, `space` und `tab` ohne Zusatztaste.

Das Ergebnis ist genau der Schaden, den die Runde verhindern will, nur an einer anderen Stelle als der untersuchten: der Nutzer benennt eine Datei um, drückt `up`, und der freigegebene Menüeintrag bewegt die Auswahl in der Liste. Das dritte Abnahmekriterium von C2.6 verlangt für Textfelder ausdrücklich das Gegenteil, und die Regel aus C2.5 kann es nicht liefern, weil sie die Frage des Fokusvorbehalts nicht stellt.

Ein zweites Symptom desselben Befundes steht im Knoten `KLICK` ("bedienbar per Mausklick"). Der Ja-Zweig von `ZUL2` nennt allein die Maus, obwohl die Gefahr dieser Runde am Kürzel hängt und nicht am Zeigegerät. Wer nur das Bild liest, hält den Ja-Zweig für harmlos.

### 2. Ein Knoten `SP` trägt zwei Sperren, und die Aufschrift von `I1` macht sie unvereinbar (substanziell, Diagramm 2)

Der Spec und sein Entscheidungsdatensatz benutzen das Wort "Sperre" für zwei Dinge. C3.7 verlangt sie je Schreibvorgang: "Jeder Schreibvorgang an den vier Dateien … geht unter einer Sperre über den Ablageordner." C3.9 verlangt sie einmalig beim Start: "Die Sitzung schreibt genau die Instanz, die die Sperre beim Start bekommen hat." Der Datensatz nennt als Begründung, dass "wer die Sperre hält" eine entscheidbare Tatsache sei und deshalb die unentscheidbare Frage nach der Zugehörigkeit der Sitzung ersetzen könne.

Das Diagramm zeichnet **einen** Knoten dafür, und die Aufschrift des Kastens legt ihn auf die dauerhafte Lesart fest. Beide Lesarten kosten etwas:

- Hält Instanz 1 die Sperre vom Start bis zum Ende, dann kann keine zweite Instanz je schreiben, und die drei Kanten `L2 --> SP`, `L1 --> SP` und `S1 --> SP` können nicht alle gelten. Der Graph enthält dann eine unmögliche Kante.
- Nimmt und gibt jeder Schreibvorgang die Sperre, dann hält sie nach dem ersten Schreiben niemand, und "wer die Sperre hält" beantwortet die Frage nach der Sitzung nicht mehr. Der tragende Grund des Datensatzes fällt weg.

Beide Lesarten sind wiederherstellbar, wenn es zwei Mechanismen gibt: einen kurzlebigen wechselseitigen Ausschluss je Schreibvorgang und ein dauerhaft gehaltenes Merkmal der Sitzungszuständigkeit. Der Entwurf braucht dann zwei Namen und zwei Knoten. Wir halten das für den eigentlichen Befund und nicht für eine Zeichenfrage: eine Runde, die eine Sperre neu in einen Kern einführt, der bisher ohne auskam, darf nicht zwei Lebensdauern unter einem Wort führen. Die offene Nutzerfrage zur Ablage ist der richtige Ort, um beide zu benennen.

### 3. Die verneinende Kante von `I2` sagt im Bild das Gegenteil (mittel, Diagramm 2)

Die Kante `I2 -.->|schreibt die Sitzung nicht| D1` ist eine Verneinung in Kantenform, und eine Kante kann nicht verneinen. Im gerenderten Bild haben wir nachgesehen, was daraus wird: die gepunktete Linie beginnt nicht am Rand des Kastens, sondern am Knoten `Lesezeichen` innerhalb von Instanz 2, und sie läuft an `SP` **vorbei** direkt auf `session.toml`. Ein Leser sieht damit dreierlei, das nicht gemeint ist: dass Instanz 2 die Sitzung anfasst, dass ihr Lesezeichen-Knoten es tut, und dass ein Schreibweg an der Sperre vorbeiführt, obwohl C3.7 genau das ausschließt.

Die Aussage steht ohne die Kante bereits im Bild, und zwar durch das Fehlen eines zweiten `Sitzungsschreiber`-Knotens im Kasten `I2`. Ein Vermerk unter dem Diagramm trüge sie besser als ein Pfeil.

### 4. `SP` löscht die Zuordnung von Schreiber zu Datei, und zwei Dateien haben keinen Erzeuger (mittel, Diagramm 2)

Alle vier Kanten von `SP` zu den Dateien sind unbeschriftet, und `SP` steht zwischen allen Schreibern und allen Zielen. Aus dem Graphen allein liest man, jeder Schreiber schreibe jede Datei. Der Spec ist genauer: der Sitzungsschreiber schreibt `session.toml`, der Lesezeichenbefehl `bookmarks.toml`, die Belegungsansicht `keymap.toml`, und `settings.toml` wird nur beim allerersten Start geschrieben.

Für `keymap.toml` und `settings.toml` steht kein Erzeuger im Bild. Das fällt hier stärker ins Gewicht als sonst, denn `keymap.toml` schreibt die Belegungsansicht beim Verlassen mit Änderung, und die Belegungsansicht ist der Gegenstand von C1. Zwei der drei Fähigkeiten dieser Runde treffen sich an dieser Datei, und das Bild zeigt es nicht.

Der hohe Ausgangsgrad von `SP` selbst beanstanden wir ausdrücklich **nicht**. Eine Sperre ist ein Nadelöhr von Beruf, und ein Knoten mit vier Ausgängen bei acht Knoten ist kein Gott-Knoten.

### 5. Das Bild zeigt die Abhilfe und nicht die Gefahr (mittel, Diagramm 2)

Der schwerere der beiden Befunde der Ausgangslage ist die gemeinsame Nachbardatei: `atomar::nachbarpfad` leitet ihren Namen fest ab und trägt bewusst keine Laufnummer, weshalb zwei Instanzen dieselbe benutzen und das `rename` ein Gemisch veröffentlichen kann. Im Diagramm der Ablage kommt sie nicht vor. Wer nur das Bild sieht, hält die Sperre für einen Schutz gegen verlorene Änderungen; dass sie einen Schutz gegen eine beschädigte Datei ist, steht allein in der Prosa. Ein Knoten für die Nachbardatei zwischen `SP` und den vier Dateien zeigte beides zugleich.

### 6. Der Ja-Zweig von `ZUL` landet im Bild unter dem Menü (mittel, Diagramm 1)

Wir haben das gerenderte Bild angesehen. Die Kante `ZUL -->|ja| AUS` läuft über die gesamte rechte Seite nach unten und endet in `AUS` ("ausführen und schlucken"), das dadurch **unter** `Hauptmenü` und `ZUL2` zu liegen kommt. Das ist die einzige Kante, die der Leserichtung merklich zuwiderläuft, und sie kehrt die Aussage um: schluckt der Abgriff, sieht das Menü das Ereignis nie, im Bild steht das Schlucken aber nach dem Menü.

Die Zusammenführung selbst ist richtig und trägt C2.14, wonach Taste und Menüeintrag über dieselbe Stelle ausführen. Die Beschriftung trägt sie nicht: ein Mausklick führt aus und schluckt nichts. Ein getrennter Knoten für das Ausführen und ein zweiter für das Schlucken lösten beides, und sie berührten die vierte offene Nutzerfrage, die genau hier hängt.

### 7. Der Fänger `F` hat drei Ausgänge, von denen zwei sich überschneiden (geringfügig, Diagramm 1)

Die Zweige "nimmt auf" und "Suchzeichen" sind nicht überschneidungsfrei: ein während der Aufnahme getipptes Zeichen erfüllt beide. C1.15 entscheidet den Vorrang ("Während der Aufnahme nimmt die Suche nichts auf"), das Bild zeigt ihn nicht. Zwei hintereinandergeschaltete Rauten statt einer mit drei Ausgängen brächten die Reihenfolge ins Bild, und sie entspräche dem Code, in dem der Fänger und die Suche zwei Stationen sind.

### 8. Diagramm 1 trägt keine `subgraph`-Blöcke (geringfügig)

Die Schichtung ist im gerenderten Bild trotzdem ablesbar, weil die Kanten fast durchweg nach unten laufen. Vier Schichten liegen der Sache nach vor: Abgriff, Vorbehalt, Nachschlag mit Zulässigkeit, Menü. Ein Kasten je Schicht machte den Befund 1 auf einen Blick sichtbar, denn dann sähe man, dass die Kante aus dem Vorbehalt zwei Schichten überspringt und die Zulässigkeitsschicht umgeht. Der Spec der Runde 4 hat diese Form verwendet, und sie hat dort getragen.

### 9. Für die Betriebsarten der Belegungsansicht fehlt ein Diagramm (mittel)

Die Ansicht trägt nach dieser Runde drei Betriebsarten, und `esc` bedeutet in zweien etwas anderes: während der Aufnahme bricht es ab, sonst verlässt es und sichert, und einen Suchtext löscht es nie (C1.13). Dazu kommen der Suchtext, der so lange lebt wie die Ansicht (C1.12), die Rücktaste, die bei leerem Text nichts tut (C1.8), und drei Schaltflächen, deren Tasten an einer offenen Nutzerfrage hängen (C1.16). Das ist ein Zustandsautomat, und `stateDiagram-v2` ist dafür der Typ, den `rules/design-diagrams.md` vorsieht. Er prüfte zugleich, ob die Fallunterscheidung vollständig ist; heute steckt sie in sechzehn Kriterien, die man einzeln gegeneinander lesen muss.

## Nicht zu beanstanden

**Beide Blöcke parsen, und wir haben sie gerendert.** `mmdc` 11.16.0 aus dem `npx`-Zwischenspeicher hat beide nach SVG und PNG erzeugt, und wir haben beide Bilder angesehen. Kein Syntaxbefund.

**Die Typwahl stimmt zweimal.** Ein gerichteter Fluss für den Weg eines Tastendrucks, ein `LR`-Fluss mit Kästen für die Ablage: beides sind die Zeilen, die die Typtafel der Regel dafür vorsieht. Eine Sequenz wäre für das erste falsch, denn gezeigt wird eine Fallunterscheidung und kein Verlauf über die Zeit.

**Keine Zyklen, in keinem der beiden Graphen.** Wir haben beide auf Rückkanten durchsucht; es gibt keine. In einem Entwurf, der eine Sperre einführt, ist das die Auskunft, auf die es ankommt.

**Die Dichte ist in beiden Fällen niedrig.** 1,14 und 0,89 Kanten je Knoten, keine kreuzende Verfilzung, jeder Knoten angeschlossen. Wer den Spruch **tangled** als Aussage über die Menge der Striche liest, liest ihn falsch: beide Befunde sitzen an einer Beschriftung und an einem Knoten, der zwei Dinge zugleich ist.

**Zwei Diagramme sind für diesen Spec die richtige Zahl, und sie zeigen die richtigen zwei Sachen.** Die Naht der Runde liegt zwischen den beiden Bildern, und dass es zwei sind und nicht eines, ist selbst eine Aussage über den Zuschnitt. Für das Menü mit neun Obermenüs braucht es kein drittes: dass `nach_bereichen` einen dritten Abnehmer bekommt, trägt die Prosa.

## Was ein sauberer Nachzug verlangt

Der Spruch ist beratend, und die Entscheidung liegt beim Nutzer. Was folgt, sind die zwei Änderungen am **Entwurf**, ohne die die Bilder nicht zur Deckung mit dem Text kommen. Beide gehören vor die Planung und nicht in eine Nachbesserung der Zeichnung.

**Erstens: die Zulässigkeitsfrage aus C2.5 braucht die Frage des Fokusvorbehalts als dritten Bestandteil.** Heute nennt sie zwei: kein Blatt, und `fokus::wirkt` sagt ja. Der Fall des Feldeditors fällt durch beide hindurch, und C2.6 verlangt ihn. Die Erweiterung ist klein, denn die Antwort liegt schon vor: `ersthelfer_gehoert_appkit` in `crates/krk-ui/src/appkit/ereignisse.rs` beantwortet sie für den Abgriff. Sie ein zweites Mal zu stellen, hält die Zusage "eine Frage, zwei Frager, eine Stelle" ein; sie wegzulassen bricht sie. Im Bild verschwindet damit die Kante `V --> APP` als Sonderweg: der Vorbehalt wird zu einem Bestandteil der einen Raute, und alle Wege ins Menü kommen aus derselben Prüfung. Zu prüfen bleibt der zweite Eingang, `N --|kein Treffer|--> APP`: eine Funktion ohne Kommando hat auch keinen Menüeintrag, weshalb wir diesen Weg für ungefährlich halten, aber der Spec sollte den Satz führen.

**Zweitens: die Ablage braucht zwei Namen für zwei Sperren, oder einen benannten Verzicht auf eine davon.** Die Schreibsperre je Vorgang und das dauerhafte Merkmal der Sitzungszuständigkeit haben verschiedene Lebensdauern und verschiedene Zwecke, und der Entscheidungsdatensatz stützt seine Empfehlung auf die dauerhafte Lesart, während C3.7 die kurzlebige verlangt. Solange beide "die Sperre" heißen, wird der Plan eine von ihnen bauen und glauben, er habe beide. Im Bild sind es zwei Knoten, und die Aufschrift von `I1` nennt dann das Merkmal und nicht die Schreibsperre.

Die übrigen sieben Befunde ändern den Entwurf nicht. Sie ändern, was ein Leser aus den Bildern mitnimmt, und sie sind Sache des Planners oder einer Nachbesserung des Specs.
