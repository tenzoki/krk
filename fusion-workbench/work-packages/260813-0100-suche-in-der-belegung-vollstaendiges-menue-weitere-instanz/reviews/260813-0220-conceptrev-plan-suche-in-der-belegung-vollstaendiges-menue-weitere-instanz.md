# Concept Evaluation: Implementierungsplan Suche in der Belegung, vollständiges Menü, weitere Instanz

**Date:** 2026-08-13 02:20
**Target:** `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`
**Verdict:** acceptable
**Diagrams evaluated:** 4  |  **Validation:** by-tool (mmdc 11.16.0 über `npx`, alle vier Blöcke nach PNG gerendert und angesehen)
**Vorgänger:** `260813-0109-conceptrev-…` (Spruch `tangled`) und `260813-0144-conceptrev-…` (Spruch `acceptable`), beide zum Spec

## Spruch

**Der Plan ist tragfähig, und ein einziger Punkt gehört vor die Ausführung: die Zahl drei in der Zählprobe zu `ersthelfer_gehoert_appkit`.** Die vier Bilder parsen, sie sind sauber geschichtet, sie tragen keinen Zyklus, keinen Gott-Knoten und keine Waise, und die Typwahl stimmt viermal. Die zwei Punkte, die der zweite Durchgang der Planung mitgegeben hat, sind beide umgesetzt und nicht nur behauptet: der dritte Ausgang des Nachschlags steht als eigener Knoten mit eigener Wache im ersten Bild, und der Zustandsautomat trägt seine Wächter.

Die Wache selbst ist richtig gebaut. Ihre Buchführung ist es nicht. Der Regelkasten sagt „eine Funktion, drei Frager", der Graph zeichnet zwei Kanten in denselben Knoten, und der Entwurf, den S2 und S6 beschreiben, ergibt zwei Aufrufstellen und keine drei. S2 nimmt die Drei trotzdem als Abnahmekriterium auf. Eine Zählprobe, die gegen den eigenen Entwurf zählt, schlägt entweder fehl, oder jemand baut die dritte Aufrufstelle nach, um sie grün zu bekommen. Der zweite Ausgang ist der schlechtere: er stellt dieselbe Frage ein zweites Mal, und genau das verhindert diese Runde an anderer Stelle.

Zwei weitere Befunde ändern keine Regel, kosten aber einen Ausführenden Zeit oder eine Fundstelle. Im dritten Bild steht eine Funktion des Kerns im Kasten von `krk-ui`. Im ersten Bild ist die Bedingung der Suchstation enger gezeichnet als S10 sie beschreibt.

## Die drei Aussagen des Auftrags, nachgeprüft

| Aussage | Befund |
|---|---|
| 1 · Die Wache vor dem Sprungmarkenpuffer steht im Bild, und die Zahl der Frager stimmt mit der Zählprobe | **Halb.** Die Wache steht: `NACH` hat drei Ausgänge, der mittlere führt auf `WACHE`, und `WACHE -.->\|fragt\| ERSTH` zeichnet den eigenen Zugriff auf dieselbe Funktion. Die Zahl stimmt nicht. Siehe Befund 1. |
| 2 · Menüaufbau und Ausgrauung sind eine Einheit | **Zutrifft.** Im vierten Bild ist `S6` **ein** Knoten und trägt alle drei Gegenstände in seiner Aufschrift. Kein Bild legt eine Trennung nahe. Siehe „Nicht zu beanstanden". |
| 3 · Die zwei Sperren sind sauber getrennt geblieben | **Zutrifft.** `RECHT` und `DURCH` sind zwei Knoten mit zwei Sperrdateien, zwei Lebensdauern und zwei beschrifteten Kanten auf denselben Fremdaufruf. Kein Knoten trägt beide Mechanismen. |

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Spruch |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `flowchart TD` | 19 | 23 | 1,21 | 3 (`NACH`, `A1`, `A2`, `WACHE`) | 3 (`WEITER`) | 0 | ja, zwei `subgraph` | 0 | acceptable |
| 2 | `stateDiagram-v2` | 4 Zustände (+1 Verbund, 2 Pseudo) | 8 | 2,00 | 2 | 2 | 2 (gewollt) | ja, zwei Regionen | 0 | clean |
| 3 | `flowchart TD` | 9 | 10 | 1,11 | 2 | 5 (`DURCH`) | 0 | ja, zwei `subgraph`, `FLOCK` daneben | 0 | acceptable |
| 4 | `flowchart TD` | 15 | 14 | 0,93 | 3 (`S2`) | 3 (`S6`) | 0 | nein, kein `subgraph`; vier Stränge stehen als Spalten | 0, aber zwei Komponenten | acceptable |

Zu Diagramm 1: zwei Quellen (`E` und `KLICK`, beide bewusst), vier Senken (`ZUW`, `SUCH`, `TIPP`, `ERST`), alle 19 Knoten erreichbar. Unbeschriftet sind 3 von 23 Kanten (`WEITER --> MENUE`, `KLICK --> A2`, `MENUE --> A2`), keine davon mit Bedeutungsverlust.

Zu Diagramm 3: der Eingangsgrad 5 an `DURCH` ist kein Gott-Knoten, sondern die Bauform eines Schreibtors, und wir beanstanden ihn ausdrücklich nicht. Er stimmt zugleich mit der Zählaussage in S12 überein, wonach der Übersetzer an fünf Stellen aus diesem Bild plus `messmodus.rs:301-315` anhält.

Zu Diagramm 4: die 15 Schritte zerfallen in zwei Zusammenhangskomponenten, S1 bis S10 und S11 bis S15. Für einen Abhängigkeits-DAG ist das kein Fehler, sondern die Aussage, dass der Strang der weiteren Instanz unabhängig läuft. Die Prosa nennt ihn selbst die Naht, an der die Runde sich teilen ließe. Die Kante `S2 --> S6` ist über `S3` transitiv enthalten und damit redundant; sie schadet nicht.

Die zwei Zyklen in Diagramm 2 sind die Selbstübergänge und das Paar `keine Aufnahme` und `Aufnahme`. In einem Zustandsautomaten sind das Zustandswechsel und keine Abhängigkeitszyklen.

## Befunde

### 1. Die Zählprobe zu `ersthelfer_gehoert_appkit` verlangt drei Aufrufstellen, der Entwurf liefert zwei (substanziell, Diagramm 1)

Der Knoten `ERSTH` trägt die Aufschrift „eine Funktion, drei Frager". In den Knoten laufen zwei Kanten: `REGEL -.->|"Bestandteil (2)"| ERSTH` und `WACHE -.->|fragt| ERSTH`. Der Graph zeichnet also zwei, die Aufschrift behauptet drei, und der Graph hat recht.

Wir haben es am Baum und am Entwurf durchgerechnet. Heute gibt es genau eine Aufrufstelle, `crates/krk-ui/src/appkit/ereignisse.rs:488`, dazu die Definition ab `:536`. Nach S2 sind es diese beiden:

```
behandeln (ereignisse.rs)   ── ruft ──> ersthelfer_gehoert_appkit    Wache vor Nachschlag::Sprungmarke
lage()    (anwendung.rs)    ── ruft ──> ersthelfer_gehoert_appkit    Bestandteil (2) der Regel

kommando_ausfuehren ──┐
                      ├── ruft self.lage() ── und damit dieselbe eine Stelle
validateMenuItem:   ──┘
```

S2 legt `lage()` ausdrücklich als **eine** Methode am Delegierten an, „damit die drei Eingaben nicht an drei Orten zusammengetragen werden", und S6 lässt `validateMenuItem:` über `zulaessig(kommando, self.lage())` fragen. Beide Frager der Regel teilen sich damit eine Aufrufstelle. Zwei, nicht drei.

Die Drei stammt aus einer richtigen Beobachtung des zweiten Durchgangs, die dort auf der logischen Ebene stand: die Teilfrage nach dem Ersthelfer hat drei Interessenten. Auf der Ebene der Aufrufstellen, auf der die Probe zählt, sind es zwei. Der Plan trägt die Zahl an drei Stellen als prüfbare Zusage: in S2 („`ersthelfer_gehoert_appkit` hat genau drei Aufrufstellen"), in der Risikotabelle („Die Zählprobe hält fest, dass `ersthelfer_gehoert_appkit` drei Frager hat") und in der Aufschrift des Knotens.

C2.16 des Spec verlangt diese Zahl nicht. Sein Wortlaut ist „Die Zulässigkeitsfrage steht an genau einer Stelle, und beide Frager rufen sie", und die zugehörige Probe ist die über die Aufrufer von `zulaessig`. Die stimmt: zwei, und das erste Bild zeichnet sie als zwei gestrichelte Kanten in `REGEL`. Die Zahl drei ist ein Zusatz des Plans.

Warum es vor die Ausführung gehört: ein Ausführender, dessen Schritt mit einer roten Zählprobe endet, hat zwei Auswege. Er meldet die Zahl, oder er stellt die dritte Aufrufstelle her. Der zweite Ausweg ist der wahrscheinlichere, weil er billiger aussieht, und er baut genau den Doppelbau, den diese Runde für die Kommandos gerade beseitigt: eine zweite Stelle, an der dieselbe Frage gestellt wird. Die Korrektur ist eine Zahl an drei Stellen und keine Umplanung.

### 2. Eine Funktion des Kerns steht im Kasten von `krk-ui` (mittel, Diagramm 3)

Der `subgraph` heißt `krk-ui · die Aufrufer` und enthält fünf Knoten. Einer davon gehört nicht dorthin: `belegung::fuer_den_betrieb` liegt in `crates/krk-core/src/tasten/belegung.rs:1310`, also im Kern selbst. Der Plan schreibt die Fundstelle in seiner Ausgangslage korrekt hin; das Bild widerspricht ihr.

Der Preis ist konkret. S12 sagt, der Übersetzer halte „an jeder Aufrufstelle von `Ablage::laden` und `Ablage::sichern`" an, „das sind die fünf Stellen aus dem Aufrufbild oben plus `messmodus.rs:301-315`". Wer die fünf im Bild abliest und dessen Kastentitel glaubt, sucht sie in `krk-ui` und findet vier. Die fünfte liegt eine Kiste tiefer.

Dazu kommt eine Aussage, die das Bild in dieser Form nicht mehr macht: dass `Ablage::durchgang` einen Aufrufer innerhalb der eigenen Kiste hat. Der Kastentitel behauptet eine reine Richtung von `krk-ui` nach `krk-core`, und die stimmt für vier der fünf.

Ein zweiter Knoten desselben Kastens ist zweideutig. `Sitzungsschreiber vormerken` nennt einen Typ aus `crates/krk-core/src/ablage/sitzung.rs:417`, während die Beschreibung in S13 auf die Methode `sitzung_vormerken` in `crates/krk-ui/src/appkit/anwendung.rs:4837` zielt. Die Tabelle der Signaturänderungen führt beide Wege in einer Zeile. Welcher der beiden im Bild steht, entscheidet, ob der Kastentitel für ihn stimmt.

### 3. Die Suchstation im ersten Bild ist enger gezeichnet als S10 sie baut (mittel, Diagramm 1)

Der Knoten `SUF` fragt „Belegungsansicht steht und das Zeichen trägt ein Dateiname?" und hat zwei Ausgänge: ja führt auf „Suchtext ergänzen, auf den Treffer springen", nein auf den Nachschlag. S10 beschreibt dieselbe Station weiter: sie „prüft auf ein Suchzeichen, auf die Eingabetaste und auf die Rücktaste und gibt sie an die `Belegungsquelle`".

Im Bild fallen Eingabetaste und Rücktaste durch die Station hindurch in den Nachschlag. Der Zustandsautomat daneben zeichnet beide als Übergänge der Suche (`B --> B: Zeichen hängt an · Eingabetaste zum nächsten Treffer · Rücktaste kürzt`). Zwei Bilder desselben Plans geben damit zwei Antworten darauf, wer die Eingabetaste bekommt, während die Belegungsansicht steht.

Ausgeführt schadet die Fassung des Bildes nicht, sie leistet nur weniger: die Belegungsansicht ist ein Blatt, `blatt_steht()` ist wahr, und Bestandteil (1) der Regel weist den nachgeschlagenen Befehl ab. Verloren gingen C1.1 in seinem zweiten Teil, die Eingabetaste zum nächsten Treffer, und das Kürzen über die Rücktaste. Der Text von S10 trägt beides, also ist es kein Loch im Entwurf, sondern eine Lücke im Bild, das ein Ausführender zuerst liest. Eine dritte Kante an `SUF` oder eine erweiterte Bedingung in seiner Aufschrift schließt es.

### 4. Der Regelknoten wird im Bild vom Abgriff gefragt, im Code von der Senke (geringfügig, Diagramm 1)

`A1 -.->|fragt| REGEL` hängt an einer Raute innerhalb des Kastens `Ereignisabgriff`. S2 sagt das Gegenteil: „Der Zweig `Nachschlag::Funktion` reicht das Kommando unverändert weiter; die Frage stellt die Senke." Der Aufrufer von `zulaessig` ist `kommando_ausfuehren`, also der Knoten `TUN` am Fuß des Bildes, und nicht der Abgriff.

Für die Aussage des Bildes ist die Verortung vertretbar, denn die Antwort entscheidet, was der Abgriff schluckt, und genau das ist S3. Nur trägt dasselbe Bild die Zählzusage „`zulaessig` genau zwei Aufrufer". Wer die zwei Aufrufer aus den zwei gestrichelten Kanten abliest, setzt einen davon in `ereignisse.rs`. Der Text ist eindeutig, das Bild ist es nicht.

### 5. Die Prosa zählt vier Wächter, der Automat trägt fünf (geringfügig, Diagramm 2)

`[keine Aufnahme]` steht an fünf Übergängen: an den vier innerhalb der Suchregion und am Ausgang `Belegungsansicht --> [*]`. Die Prosa darunter sagt „Die vier Wächter". Ihre eigene Begründung nennt beide Fälle, den Suchtext während einer Aufnahme und das nackte `esc`, und der zweite ist der fünfte Wächter. Eine Zahl.

### 6. Die Aussage „nur an einer Stelle" trifft auf den Graphen nicht zu (geringfügig, Diagramm 4)

Die Prosa sagt „Vier Stränge, die einander nur an einer Stelle berühren." Der Graph zeigt zwei Berührungspunkte innerhalb der ersten Komponente, `S6` (Zulässigkeit trifft Menü) und `S10` (Zulässigkeit trifft Suche), und null zwischen dieser Komponente und dem Strang der weiteren Instanz. Gemeint ist erkennbar, dass je zwei Stränge sich an je einem Punkt berühren. So gelesen stimmt es; wörtlich gelesen nicht, und wer aus dem Satz auf einen einzigen Verflechtungspunkt schließt, plant die Reihenfolge falsch.

## Nicht zu beanstanden

**Alle vier Blöcke parsen, und wir haben die Bilder angesehen.** `mmdc` 11.16.0 hat alle vier nach PNG erzeugt. Kein Syntaxbefund. Der Bericht des Planners, er habe gerendert und nachgebessert, deckt sich mit dem, was wir sehen.

**Die zwei Punkte des zweiten Durchgangs sind an der Sache umgesetzt.** Der dritte Ausgang des Nachschlags steht im Bild und deckt sich mit `crates/krk-core/src/tasten/belegung.rs:994-1002`, wo `Nachschlag` genau die drei Varianten `Funktion`, `Sprungmarke` und `Unbelegt` führt. Der Zustandsautomat trägt seine Wächter und sagt damit nicht mehr das Gegenteil von C1.15. Der Vorrang der zwei `esc`-Bedeutungen ist entschieden und steht als Wächter am Ausgang.

**Menüaufbau und Ausgrauung stehen als eine Einheit, und kein Bild legt eine Trennung nahe.** `S6` ist ein Knoten. Dass seine Aufschrift drei Gegenstände nennt, ist sonst ein Anzeichen für einen teilbaren Schritt; hier ist es begründet, und die Begründung steht als eigener Absatz im Schritt. Im ersten Bild liegen `MENUE` und `A2` beide in Schicht 2, also im selben Augenblick, und `GRAU` trägt die Aufschrift „für Kürzel und Maus zugleich". Das Bild sagt damit dasselbe wie der Text: die Ausgrauung ist Teil desselben Mechanismus und keine spätere Zutat.

**Die zwei Sperren sind zwei Mechanismen geblieben.** `RECHT` und `DURCH` tragen je ihre Lebensdauer in der Aufschrift, je ihre Sperrdatei an der Kante nach `FLOCK`, und die gestrichelte Kante `SITZ -.-> RECHT` sagt, wer wovon abhängt. Der gemeinsame Fremdaufruf steht als eigener Knoten außerhalb beider Kästen, und das ist die richtige Form: er ist die geteilte Systemschicht und kein dritter Mechanismus. Der Befund des ersten Durchgangs zum Spec, ein Knoten trage beide Mechanismen, kommt hier nicht wieder.

**Die Typwahl stimmt viermal.** Ein gerichteter Fluss für den Weg eines Tastendrucks, ein Zustandsautomat für die Betriebsarten der Belegungsansicht, ein gerichteter Fluss mit Kästen für die Sperren, ein `flowchart TD` als Abhängigkeits-DAG der Schritte. Die Typtafel der Regel sieht genau diese vier Zeilen vor.

**Der DAG deckt sich mit den Abhängigkeitszeilen der fünfzehn Schritte.** Wir haben jede Kante gegen die Zeile „Abhängigkeiten" ihres Schrittes gehalten. Sie stimmen alle, mit der einen redundanten Kante `S2 --> S6`, die über `S3` ohnehin gilt und für die es einen sachlichen Grund gibt: `validateMenuItem:` in S6 braucht `lage()` aus S2.

## Was vor die Ausführung gehört

**Ein Punkt, und er kostet eine Zahl.** Befund 1: die Zusage „drei Aufrufstellen" in S2, in der Risikotabelle und in der Aufschrift des Knotens `ERSTH` ist gegen den Entwurf des Plans falsch. Zwei ist die Zahl, die S2 und S6 zusammen ergeben. Wer sie nicht korrigiert, hinterlässt eine Abnahmeprobe, die entweder rot endet oder mit einer zweiten Aufrufstelle grün gemacht wird, und die zweite Aufrufstelle ist der Doppelbau, den die Runde beseitigen will.

Wenn stattdessen die logische Lesart gemeint ist, also drei Interessenten an der Teilfrage, dann gehört das in die Aufschrift und in den Doc-Kommentar und nicht in eine Zählprobe. Eine Probe zählt Aufrufstellen und keine Interessenten.

**Alles Übrige darf während der Ausführung nachgezogen werden.** Befund 2 und 3 kosten einen Ausführenden Zeit oder eine Fundstelle, aber keine Regel: die krk-core-Fundstelle steht in der Ausgangslage des Plans richtig, und S10 beschreibt seine zwei Stationen vollständig. Die Befunde 4 bis 6 sind Sache der Zeichnung.

**Der Entwurf selbst trägt.** Die Zulässigkeitsfrage ist eine Funktion mit zwei Aufrufern und einer eigens versorgten Wache für den dritten Zweig, das Menü und seine Ausgrauung sind ein Schritt, die zwei Sperren sind zwei Mechanismen mit zwei Lebensdauern über einem Fremdaufruf, und die fünfzehn Schritte stehen in einem zyklenfreien DAG mit vier ablesbaren Strängen. Der Spruch ist beratend; die Entscheidung liegt beim Nutzer.
