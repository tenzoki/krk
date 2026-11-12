# Schuldet das Paket der drei Schaltflächen im Protokoll einen Abnahmelauf gegen die zehn Zusagen aus C8?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-2242-minutes-editor-traegt-kopieren-singlestrip-format.md, 261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md, 261017-0955-schuldet-das-sftp-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261102-1313-schuldet-das-hauptfadenpaket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261111-0852-schuldet-das-menue-und-minutes-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md

---

## Question

Der Umbau berührt einen gemessenen Weg an genau einer Stelle: `Anwendungsdelegierter::lage` wird je Tastenbefehl erhoben, auch für die Befehle, an denen L1 bis L10 messen, und bekommt ein Feld `protokolldatei` dazu. Der Wert steht seit dem Laden der Datei im `Editormodell`; gelesen wird ein Wahrheitswert hinter einem `RefCell`-Borgen, ohne Systemaufruf und ohne Speicheranforderung, nach derselben Bauart wie `Lage::eintragsdatei` und `Lage::csv`. Die Schaltflächenreihe entsteht erst, wenn der Editor zum ersten Mal eine Protokolldatei zeigt, und liegt damit nicht auf dem Startweg von L4. Lesevorgang, Sortierung, Vorschau und Operationsmaschine fasst das Paket nicht an. Seit dem 261017 stehen alle zehn Zusagen auf den Gegenständen der späteren Messrunde; der Lauf verlangt KRK im Vordergrund und ist Nutzerarbeit. Das Paket liefert aus, also steht die Frage vor der Auslieferung.

## Options

1. **Messen** — L1 bis L10 am Bündel nach dem letzten Codeschritt, vor `./release.sh`.
   - Pros: die Auslieferung trägt gemessene Zahlen.
   - Cons: Nutzerarbeit vor einer Auslieferung, die der Nutzer ohne Rückfrage bestellt hat; der Anlass ist ein gelesener Wahrheitswert.
2. **Nicht messen** — die zehn Zusagen bleiben auf den Gegenständen der späteren Messrunde, wie bei jedem Paket seit dem sftp-Paket.
   - Pros: keine Nutzerarbeit; die Lage der Zusagen ändert sich nicht.
   - Cons: die Messrunde schiebt ein weiteres Paket vor sich her.

## Constraints

Kein Agent kann den Lauf fahren. Keine der zehn Zahlen wird geändert.

## Recommendation

Möglichkeit 2. Der Umbau legt dem gemessenen Weg einen Feldzugriff auf, und die Pakete, die dieselbe Stelle um `eintragsdatei` und `csv` erweitert haben, sind ebenso verfahren. Die Auslieferung wartet nach dem Plan nicht auf diese Antwort; lautet sie „messen“, ist der Lauf eine Nutzerarbeit nach der Auslieferung.

---
Answered: 261112, vom Nutzer im Chat („2:nein“) — Möglichkeit 2: kein Abnahmelauf; die zehn Zusagen bleiben auf den Gegenständen der späteren Messrunde; ruled by user, Kai Stalmann <ks@qantr.com>
