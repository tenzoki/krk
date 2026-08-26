Kein Prüfordner außer dem L6-Unterordner wird gegen seine zugesagte Eintragszahl gehalten

---

L3 sagt 400 ms für **10.000** Einträge zu, L10 4 s für **100.000**. Die Zahl ist Bestandteil der
Zusage und nicht Beiwerk. Der Gesamtlauf prüft von den drei Prüfordnern aber nur, dass sie
Verzeichnisse sind (`crates/krk-bench/src/messen.rs:1019-1026`), und die kopflose Strecke prüft
nur, dass alle zwanzig Läufe **dieselbe** Zahl lesen (`messen.rs:170-180`) — nicht, dass es die
zugesagte ist. Ein Prüfordner mit 3.000 Einträgen hält L3 mühelos, und nichts im Lauf sagt es.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Hoch
**Betroffen:** `crates/krk-bench/src/messen.rs`, `crates/krk-bench/src/bericht.rs`
**Cross-references:** `crates/krk-bench/src/messen.rs:1485-1509` (der Gegenmaßstab), `messungen/260810-1918-alle-zusagen.txt`

## Der Maßstab steht schon im Baum, und zwar eine Bildschirmseite tiefer

`unterordner_sicherstellen` (`messen.rs:1485-1509`) hält für den L6-Unterordner genau das ein,
was den drei anderen fehlt, und in beide Richtungen:

- `Some(brief) if brief.eintraege == EINTRAEGE_L6 => Ok(unterordner)` (`messen.rs:1492`) — die
  Zahl im Steckbrief muss die zugesagte sein, sonst bricht der Lauf ab.
- `None if unterordner.exists() => Err(… "auf unbekanntem Bestand misst L6 nicht" …)`
  (`messen.rs:1499-1503`) — ein Ordner **ohne** Steckbrief wird abgewiesen, statt auf ihm zu
  messen.

Für Prüfordner A, B und 100k gilt keine der beiden Regeln. `Gesamtlauf::fahren` prüft an
`messen.rs:1019-1026` allein `is_dir()`. Ein Ordner ohne Steckbrief kommt durch; der Bericht
schreibt dann über `ordner_beschreiben` (`messen.rs:2088-2101`) „kein Steckbrief daneben;
Startwert unbekannt" in den Kopf und misst weiter.

## Die tatsächlich gelesene Zahl erreicht den Abnahmebericht überhaupt nicht

`Messreihe` führt das Feld `eintraege` (`messen.rs:131`), also die Zahl, die die Läufe
wirklich gelesen haben. `eine_gesamtrunde` entnimmt der Reihe aber nur die Wertelisten
(`messen.rs:1208-1209` und `1217-1218`) und lässt `reihe_a.eintraege` und `reihe_gross.eintraege` fallen.
`Gesamtergebnis` trägt kein solches Feld, und `gesamt_verfassen` (`bericht.rs:206`) hat damit
keine Stelle, an der es sie ausweisen könnte. **Im Abnahmebericht über die zehn Zusagen steht
nirgends, wie viele Einträge tatsächlich gelesen wurden** — nur, was der Steckbrief behauptet.

Die kopflose Strecke ist da einen Schritt weiter und trotzdem nicht zu Ende: `verfassen`
schreibt beide Zahlen nebeneinander (`bericht.rs:100-106`, `„{} (laut Steckbrief: {})"`), aber
niemand vergleicht sie. Ein Auseinanderfallen steht im Bericht und hält den Lauf nicht an.

## Warum das mehr ist als Sorgfalt

Der letzte vollständige Abnahmelauf ist vom 260810 und liegt vor jeder seither geschlossenen
Runde. Wer ihn wiederholt, tut das auf Ordnern, die seit fünfzehn Tagen auf der Platte liegen.
Ein hineingerutschter `.DS_Store`, ein von Hand gelöschter Eintrag, ein aus einem älteren Lauf
stehengebliebener Ordner unter dem erwarteten Pfad — jeder dieser Fälle liefert zwanzig
übereinstimmende Läufe auf einem Bestand, den keine Zusage meint, und ein grünes Gate.

## Denkbarer Weg

Dieselbe Prüfung wie für L6, an derselben Stelle: in `Gesamtlauf::fahren` neben `is_dir()` den
Steckbrief lesen und A und B gegen 10.000, den dritten gegen 100.000 halten; und nach jeder
`Messreihe` die tatsächlich gelesene Zahl gegen den Steckbrief prüfen, statt sie fallen zu
lassen. Für `messen --kopflos` genügt der zweite Teil, weil dort keine feste Zahl zugesagt ist —
aber ein Abbruch bei Abweichung ist auch dort richtiger als eine Zeile im Kopf.

---
Resolved: 260826-2140 — pruefordner_pruefen traegt die zwei Regeln aus unterordner_sicherstellen an einer Stelle; Gesamtlauf::fahren ruft sie fuer A, B und 100k, Messreihe::fahren haelt die gelesene Zahl gegen den Steckbrief, wo einer daliegt, und Gesamtergebnis reicht sie in den Berichtskopf; Probe rot vor der Behebung (Plan 260826-1811 Schritt 6).
