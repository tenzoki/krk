# Concept Evaluation: Plan Vier Tastenbefehle für Pfade, das Öffnen und Cmd+W

**Date:** 2026-08-11 17:04
**Target:** `fusion-workbench/circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/planning/260811-1648_o_plan-vier-tastenbefehle-pfade-kopieren-oeffnen.md`
**Verdict:** acceptable
**Diagrams evaluated:** 2  |  **Validation:** by-tool (mmdc 11.16.0 aus dem npx-Zwischenspeicher, beide Blöcke nach PNG gerendert und angesehen)

## Spruch

Der Entwurf steht, und beide Bilder tragen ihn; zwei Kanten stimmen nicht, und beide sind einzeln benennbar. Der Aufbaugraph zeigt die tragende Trennung dieser Runde an der Stelle, an der sie entschieden wird: die drei neuen Zweige sitzen sichtbar im Kasten `appkit/tabelle.rs, DateifensterQuelle`, und ein Knoten für den Anwendungsdelegierten kommt im Bild nicht vor. Kein Leser kann dem Bild entnehmen, die vier Befehle hingen am Delegierten. Der Schrittgraph ist ein sauberer gerichteter Graph ohne Kreise, und die Sonderstellung von Cmd+W steht darin nicht als Beschriftung, sondern als Struktur: `S5` hat keine eingehende Kante von `S1`, und damit sagt das Bild von selbst, dass dieser Befehl keine der drei neuen Belegungen braucht. Der Befund, den die Bewertung des Specs unter ihrem zweiten Punkt geführt hat, wiederholt sich nicht.

Zwei Befunde halten den Spruch bei **acceptable** statt bei **clean**, und keiner von beiden ist Kosmetik. Im ersten Bild läuft die Kante `pfadtext, pfadzeilen --> zwischenablage::text_schreiben` aus dem Kasten "ohne AppKit prüfbar" in den Kasten "Die Hüllen um das System", also in genau die Richtung, die Frage 6 des Plans ausschließt. Im zweiten Bild fehlt die Kante `S2 --> S3`: der Öffner ruft `nichts_betroffen()`, und diese Funktion entsteht in S2. Beides sind Aussagen des Bildes über den Entwurf und nicht Eigenschaften des Entwurfs. Der Code, den der Plan beschreibt, hat weder die eine noch die andere Schwäche.

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Beschriftet | Spruch |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `flowchart TD`, vier `subgraph` | 16 | 17 | 1,06 | 3 (`KA`) | 3 (`KA`) | 0 | erklärt, im Rendern nicht tragend | 0 | 8 von 17 | acceptable |
| 2 | `flowchart TD`, ohne `subgraph` | 6 | 6 | 1,00 | 2 (`S1`) | 3 (`AB`) | 0 | bei sechs Knoten nicht nötig | 0 | 4 von 6 | acceptable |

Beide Blöcke übersetzen fehlerfrei; die eckigen Klammern in `AB["Abschluss: alle Schritte [DONE], make check grün"]` halten den Übersetzer nicht an. Der höchste Ausgangsgrad im ersten Bild liegt bei 3, verteilt auf drei verschiedene Ziele. Ein Gott-Knoten ist in keinem der beiden Graphen zu sehen, und die Dichte von 1,06 bei 16 Knoten ist die eines Baumes mit einer Handvoll Zusammenführungen.

Im gerenderten ersten Bild stehen die vier Kästen in der Reihenfolge Hüllen, dann `operationen.rs` neben den Zugängen, dann die Quelle. Das ist die Umkehrung der Deklarationsreihenfolge. Sechs Kanten laufen dabei aufwärts.

## Die vier Prüfpunkte des Auftrags

**Die Trennung zwischen Quelle und Delegiertem trägt, aber sie ist einseitig gezeichnet.** Der Kasten "appkit/tabelle.rs, DateifensterQuelle" hält `kommando_ausfuehren, drei neue Zweige` und alle Methoden, auf die Befund 1 sich beruft. Ein Knoten für `appkit/anwendung.rs` fehlt im ganzen Bild, und damit fehlt auch die Gegenaussage: dass dort genau ein Zweig entsteht und der Cmd+W gehört. Ein Bild, das nur eine Seite zeigt, kann einen Unterschied nicht zeigen. Es dementiert ihn allerdings auch nicht, und das war die Sorge des Auftrags.

**Der Doppelklick ist von der Taste an zwei von vier Stellen unterschieden.** Getrennt ist der Eingang: `E4` läuft nicht auf `kommando_ausfuehren`, sondern auf `doppelklick(zeile)`. Getrennt ist die Wirkungsmenge, und zwar über die beiden Beschriftungen `"alle betroffenen"` an der Taste und `"sonst diese eine Zeile"` am Doppelklick. Genau diese Unterscheidung hat die Bewertung des Specs unter ihrem ersten Punkt vermisst; der Plan trägt sie nach. Ungezeichnet bleiben die Herkunft aus AppKit und die Löschung der vorigen Befehlsantwort aus Befund 9.

**Cmd+W steht nicht wie eine vierte neue Belegung da.** Im ersten Bild kommt es nicht vor, im zweiten steht `S5` ohne eingehende Kante neben einem `S1`, das ausdrücklich "Drei Funktionen in der Belegung" heißt. Wer die beiden Knoten nebeneinander liest, entnimmt dem Graphen, dass S5 keine der drei braucht. Das ist die richtige Aussage, und sie steht als Struktur da, wo im Spec eine Beschriftung gefehlt hatte.

**Die Abhängigkeiten stimmen an vier von fünf Stellen.** `S1 --> S2`, `S1 --> S3` und `S3 --> S4` decken sich mit der Prosa der Schritte, und `S5` ohne Vorbedingung ebenfalls. Die fünfte Abhängigkeit steht in der Prosa und fehlt im Bild; sie ist Befund 2.

## Befunde

### 1. Eine Kante läuft aus der AppKit-freien Schicht in eine AppKit-Hülle (substanziell)

Die Kante `PT -->|"ein Pfad je Zeile"| PB` verbindet `pfadtext, pfadzeilen` im Kasten "kommandos/operationen.rs, ohne AppKit prüfbar" mit `zwischenablage::text_schreiben` im Kasten "Die Hüllen um das System, appkit/". Gelesen als das, was die Vorbemerkung über dem Bild ankündigt, nämlich "wer wen ruft", behauptet sie eine Abhängigkeit von `kommandos/` nach `appkit/`. Frage 6 desselben Plans schließt sie aus: "Das Verzeichnis `kommandos` nennt keine `objc2`-Kiste." Die Kante `WS -->|"angenommen oder nicht"| ME` läuft in dieselbe Richtung zurück und behauptet, die Systemhülle rufe die Meldungsfunktion.

Der wirkliche Rufer ist in beiden Fällen die Quelle. Der Plan sagt es an zwei Stellen wörtlich: S2 c) lässt den Befehlszweig `pfadtext` an `text_schreiben` geben, und S3 c) lässt `mit_standardprogramm_oeffnen` die Hülle rufen und danach die Meldung setzen. Sechs der 17 Kanten sind auf diese Weise Datenfluss und nicht Aufruf: `OR --> PT`, `BT --> PT`, `PT --> PB`, `PT --> ME`, `WS --> ME` und `ME --> ST`. Sie tragen dieselbe Pfeilform wie die elf echten Aufrufkanten, und die Vorbemerkung nennt nur eine der beiden Bedeutungen.

Die Folge ist im gerenderten Bild zu sehen und nicht bloß erschlossen. Die sechs Kanten ketten die reinen Hilfsfunktionen zu einer Reihe `BE → BT → PT → ME → ST`, die im Code nicht existiert, und diese Reihe endet wieder in der Quelle. Der Umbruchalgorithmus setzt den Kasten der Quelle deshalb ganz nach unten, die Hüllen ganz nach oben und die Zugänge dazwischen. Die deklarierte Richtung `TD` trägt im fertigen Bild keine Leserichtung mehr: sechs Kanten laufen aufwärts, drei davon als lange Bögen um die Kästen herum. Wer das Bild ansieht, verfolgt Kurven, statt eine Schichtung zu lesen.

Der Entwurf hat diesen Fehler nicht. Die Abhängigkeitsrichtung im Code ist einseitig, von `tabelle.rs` nach `operationen.rs` und nach den beiden Hüllen, und `operationen.rs` ruft nichts davon zurück. Zu berichtigen ist die Zeichnung, nicht der Bau.

### 2. Dem Schrittgraphen fehlt die Kante `S2 --> S3` (substanziell)

S3 ruft `nichts_betroffen()`. Der Plan sagt es in S3 c): "bei leerer Menge meldet sie `nichts_betroffen()` und tut sonst nichts." Die Funktion entsteht in S2, und zwar dort namentlich in der Dateiliste und im Änderungstext b). Die Dateiliste von S3 führt für `operationen.rs` allein `oeffnungsmeldung`. Im Baum gibt es die Funktion heute nicht: `grep -rn "nichts_betroffen" crates/` findet allein den Namen einer bestehenden Probe (`crates/krk-ui/src/kommandos/operationen.rs:824`), keine Funktion dieses Namens.

Das Bild sagt damit eine Ordnung zu, die nicht baut. Nach dem gezeichneten Graphen ist die Reihenfolge S1, S3, S4, S2, S5 zulässig, denn S3 hängt darin allein an S1. In dieser Reihenfolge fehlt S3 die Funktion, die es ruft, und der Schritt endet mit einem roten `make check`. Genau diese Zusage macht der Plan zwei Absätze über den Schritten: "Jeder der fünf Schritte baut für sich und ist für sich prüfbar; nach jedem sind die vier Abnahmekommandos grün."

Der Befund ist derselbe, den Befund 2 des Plans für die Hülle und ihren ersten Aufrufer schon einmal richtig gezogen hat. `nichts_betroffen` ist eine gemeinsame Hilfsfunktion mit zwei Aufrufern in zwei Schritten, und die Tabelle in Frage 6 sagt das auch so: "gemeinsam für beide Befehle". Der Schnitt selbst ist in Ordnung, es fehlt allein die Kante, die ihn absichert.

### 3. Der Anwendungsdelegierte fehlt im Aufbaubild, und mit ihm die Hälfte der tragenden Aussage (substanziell)

Die Vorbemerkung sagt, das erste Bild zeige, "wo die neuen Teile wohnen". Ein neuer Teil dieser Runde wohnt in `crates/krk-ui/src/appkit/anwendung.rs`: ein Zweig in `kommando_ausfuehren` und die Methode `tab_schliessen(fokus)` daneben, beides in S5. Beide fehlen im Bild.

Der Verlust ist nicht die Vollständigkeit, sondern der Beleg. Befund 1 des Plans steht und fällt mit einer Gegenüberstellung: drei Zweige hier, ein Zweig dort. Im Bild ist nur die eine Hälfte zu sehen, und die andere ist am Abnahmetor nur aus der Prosa zu holen. Ein fünfter Kasten mit dem Titel `appkit/anwendung.rs, Anwendungsdelegierter`, darin ein Knoten "kommando_ausfuehren, ein neuer Zweig: Cmd+W" und einer für `tab_schliessen(fokus)`, macht die Zahlen im Bild sichtbar und kostet zwei Knoten.

Wir führen den Befund als substanziell, weil er die Stelle betrifft, an der die Runde ihren Zuschnitt begründet. Er ist zugleich der einzige der drei substanziellen, bei dem ein Weglassen vertretbar wäre: wer das erste Bild ausdrücklich auf die drei neuen Befehle beschränkt, muss den Delegierten nicht zeigen. Dann gehört die Beschränkung aber in die Vorbemerkung und in die Aufschrift des Zugangskastens, und das ist Befund 4.

### 4. Zwei verschiedene Vieren stehen im selben Dokument (geringfügig)

Der Kasten heißt "Die vier Zugänge" und hält `opt+cmd+c`, `shift+cmd+c`, `return` und den Doppelklick. Die Überschrift des Plans heißt "Vier Tastenbefehle für Pfade, das Öffnen und Cmd+W" und meint die drei neuen Kombinationen samt Cmd+W. Die beiden Mengen haben drei Glieder gemeinsam und unterscheiden sich in genau den zwei Gliedern, deren Sonderstellung dieser Auftrag prüfen lässt.

Ein Leser, der die Zahl abgleicht, kommt auf eine der beiden falschen Zuordnungen: entweder hält er den Doppelklick für einen der vier Tastenbefehle, oder er sucht Cmd+W unter den vier Zugängen. Eine Aufschrift wie "Die vier Zugänge zu den drei neuen Befehlen" nennt beide Zahlen und trennt sie.

### 5. Die Löschung der vorigen Befehlsantwort ist der vierte Unterschied und fehlt (geringfügig)

Befund 9 des Plans beschreibt eine Asymmetrie, die der Runde eigens einen umgeschriebenen Doc-Kommentar wert ist: der Tastenweg wird vor jedem Befehl von `anwendung.rs:2009-2011` geräumt, der Doppelklick räumt selbst, weil er kein Kommando ist. Im Bild haben `KA` und `DK` beide keine Kante zur Statuszeile, und der Unterschied ist nicht zu sehen. Eine Kante `DK -->|"löscht die Antwort auf den vorigen Befehl"| ST` trüge ihn, und sie stünde an der einzigen Stelle des Graphen, an der die beiden Eingänge dieselbe Fläche berühren.

### 6. Der Weg des Doppelklicks in die Anwendung ist die einzige unbeschriftete Zugangskante (geringfügig)

`E1` bis `E3` tragen Beschriftungen, `E4 --> DK` trägt keine. Damit sieht die Zustellung des Doppelklicks aus wie die eines Tastendrucks, obwohl sie über `setDoubleAction:` am Tabellendelegierten läuft und den Fokusvorbehalt nicht berührt. Eine Beschriftung an dieser einen Kante stellt die Symmetrie der vier Eingänge wieder her und sagt zugleich, worin sie sich unterscheiden.

### 7. Zwei kosmetische Beobachtungen

`direction LR` im Kasten "Die vier Zugänge" bleibt wirkungslos: im gerenderten Bild stehen die vier Knoten untereinander, nicht nebeneinander. Der Umbruchalgorithmus verwirft die Richtungsangabe eines Teilgraphen, sobald dessen Knoten Kanten nach außen tragen. Die Zeile schadet nicht, sie hält aber ein Versprechen an den Autor nicht ein.

Der Knoten `ME` heißt "kopiermeldung, oeffnungsmeldung" und nennt damit zwei der drei Meldungsfunktionen aus der Tabelle in Frage 6. Die dritte, `nichts_betroffen`, fehlt. Sie ist dieselbe, die in Befund 2 die fehlende Kante trägt.

## Nicht zu beanstanden

**Kein Kreis, kein Gott-Knoten, keine Verfilzung.** Beide Graphen sind azyklisch, und wir haben die Kreissuche über beide gefahren. Der höchste Ausgangsgrad liegt bei 3 im ersten und bei 2 im zweiten Bild. Die Dichte von 1,06 bei 16 Knoten liegt weit unter jeder Schwelle, ab der eine fehlende Schicht zu vermuten wäre.

**Kein verwaister Knoten.** Alle 16 Knoten des ersten und alle 6 des zweiten Bildes tragen mindestens eine Kante. Drei Knoten des ersten Bildes sind Blätter: `PB`, `ST` und `EIN`. Die ersten beiden sind Ergebnisse und gehören ans Ende. Bei `EIN` hatte die Bewertung des Specs eine Asymmetrie geführt, weil dort der Ordnerzweig des Doppelklicks vor dem Ziel endete. In diesem Bild ist das Blatt richtig: `in_zeile_einsteigen` ist die Grenze dessen, was die Runde neu baut, und der Weiterweg über `ordner_lesen` ist Bestand. Wir litigieren den Befund nicht neu.

**Der Diagrammtyp passt beide Male.** Ein gerichteter `flowchart` mit Kästen für die Module ist die Wahl für Aufbau und Fluss, und ein `flowchart TD` ist die vorgesehene Form für eine Schrittabhängigkeit. Ein dritter Graph fehlt nicht: die 62 Abnahmekriterien, die neun Meldungstexte und die elf Befunde sind Aufzählungen, und ihre Tabellenform ist die richtige.

**Der Schrittgraph ist im Rendern sauber.** `S1` steht oben, `AB` unten, alle sechs Kanten laufen abwärts, keine kreuzt eine andere. Ein Teilgraph fehlt bei sechs Knoten nicht.

**Zwei Vorwürfe, die wir geprüft und verworfen haben.** Die Kette `BE → BT → PT → ME → ST` sieht nach einer Verletzung der Schichtung aus, weil sie zweimal zwischen AppKit und der reinen Schicht wechselt. Am Code nachgesehen ist der Wechsel legitim: `operationen.rs` ist keine tiefere Schicht, sondern ein reines Hilfsmodul, das die Quelle an zwei Punkten ruft, vor dem Systemaufruf und nach ihm. Ein Kreis entsteht daraus nicht. Und die fehlende Kante `S1 --> S4` ist keine Lücke, sondern die richtige Sparsamkeit eines gerichteten Graphen: S4 erreicht S1 über S3.

## Was ein sauberer Nachzug verlangt

Der Spruch ist **acceptable**, deshalb steht hier keine Umgestaltung des Entwurfs. Drei Änderungen bringen die Bilder mit dem Plan zur Deckung, und keine von ihnen hält die Abnahme auf.

Die sechs Datenflusskanten des ersten Bildes bekommen entweder eine eigene Form, etwa `-.->` mit einem Wort in der Vorbemerkung, oder sie laufen über ihren wirklichen Rufer. Der zweite Weg ist der lehrreichere: `KA --> PB` statt `PT --> PB`, und `OE --> ME` statt `WS --> ME`. Danach steht keine Kante mehr von `kommandos/` nach `appkit/`, die Quelle steht als der Ort da, der die Reihenfolge bestimmt, und die Kästen ordnen sich im Rendern von selbst, weil die künstliche Kette von fünf Gliedern verschwindet.

Das zweite Bild bekommt die Kante `S2 -->|"nichts_betroffen"| S3`. Ebenso gültig wäre, `nichts_betroffen` in die Dateiliste von S3 zu ziehen und in S2 nur den Aufruf zu lassen; dann fällt die Kante fort, und der Text der beiden Schritte muss es sagen. Welcher der beiden Wege gewählt wird, ist eine Frage an den Planner und nicht an diese Bewertung. Ungeklärt darf sie nicht bleiben, weil ein Ausführer die zulässige Reihenfolge aus dem Bild nimmt.

Das erste Bild bekommt einen fünften Kasten für `appkit/anwendung.rs` mit dem einen neuen Zweig, oder seine Vorbemerkung und die Aufschrift "Die vier Zugänge" sagen, dass es die drei neuen Befehle zeigt und Cmd+W im zweiten Bild steht. Beide Wege lösen zugleich Befund 4.

---

Abgleichsvermerk 260811-2157 (`reconciler`): die Auflage dieser Durchsicht ist eingeloest. Der Kopf
des betroffenen Dokuments nennt die Nachbesserung mit Datum, und die Diagramme im Baum tragen sie.
Der Befund des Plan-Durchgangs, die fehlende Kante `S2 → S3` sei ein echter und kein zeichnerischer
Mangel, ist am Code bestaetigt: `mit_standardprogramm_oeffnen` (`crates/krk-ui/src/appkit/tabelle.rs:940`)
ruft `operationen::nichts_zu_oeffnen`, das im selben Zug wie `nichts_zu_kopieren` aus S2 entstanden
ist. Eine Reihenfolge S3 vor S2 haette den Baum rot stehen lassen.
