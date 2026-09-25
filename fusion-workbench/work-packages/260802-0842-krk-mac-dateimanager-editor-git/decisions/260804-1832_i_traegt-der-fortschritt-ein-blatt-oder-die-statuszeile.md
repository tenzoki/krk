# Trägt der Fortschritt einer Dateioperation ein Blatt oder die Statuszeile?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1814_o_ein-modales-blatt-widerspricht-der-zusage-dass-die-oberflaeche-bedienbar-bleibt.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1814_o_ein-blatt-braucht-360-ms-bis-es-steht-und-l8-sagt-200-ms-zu.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C1, C4, C8)

---

## Question

S16 zeigt den Fortschritt einer laufenden Dateioperation als Blatt am Dateifenster. Die Abnahme am 260804 hat zwei Befunde geliefert, die dieselbe Wahl von zwei Seiten treffen. Erstens ist ein Blatt fenstermodal: es sperrt genau die Oberfläche, die C4 während einer laufenden Operation bedienbar zusagt. Zweitens braucht macOS auf dem Referenzgerät 354 bis 403 ms, bis ein Blatt angehängt ist, gleich was es zeigt, während L8 den Fortschritt 200 ms nach Start sichtbar zusagt. KRKs eigener Anteil liegt bei 152 bis 154 ms und wäre für sich genommen kein Problem. Die Frage ist damit nicht, ob KRK schneller wird, sondern wo der Fortschritt erscheint.

## Options

1. **Die Zusage anders lesen** — "sichtbar" heißt, dass KRK das Blatt in Auftrag gegeben hat und die Einblendung läuft.
   - Pro: kein Umbau, S16 bleibt in jeder Hinsicht stehen.
   - Contra: löst den ersten Befund nicht, das Blatt sperrt weiter. Der Zeitpunkt, an dem der erste Bildpunkt des Blattes erscheint, ist mit den Mitteln dieses Projekts nicht messbar; die Zusage hinge an einer Auslegung statt an einer Messung.
2. **Den Verzug verkürzen** — die 150-ms-Regel auf rund 40 ms herunternehmen.
   - Pro: kleiner Eingriff, eine Konstante.
   - Contra: bringt das angehängte Blatt auf rund 400 ms und erreicht die 200 ms nicht. Zusätzlich schwächt es die Zusage, dass eine kleine Kopie kein Fenster aufblitzen lässt. Der erste Befund bleibt ebenfalls ungelöst.
3. **Den Fortschritt in die Statuszeile stellen** — die Zeile am Fuß des Dateifensters, die seit S12 ohnehin da ist.
   - Pro: löst beide Befunde in einem. Das Fenster bleibt bedienbar, die Zusage aus C4 hält in ihrer starken Lesart, und eine Zeile erscheint ohne Einblendung mit dem nächsten Zeichendurchgang, also rund 17 ms nach dem Setzen bei 60 Hz. L8 liegt damit bei rund 170 ms und wird auf demselben Weg gemessen wie L1, L5, L6 und L7. Keine neue Ansicht entsteht.
   - Contra: eine Zeile am Fuß ist leichter zu übersehen als ein Blatt in der Mitte, und der Abbruch verliert seine Schaltfläche. Die Zeile trägt dann Fortschritt **und** Fehlermeldungen, also einen dritten Schreiber.

## Constraints

- C4 sagt zu, dass die Oberfläche während einer länger laufenden Operation bedienbar bleibt, den Fortschritt zeigt und sich abbrechen lässt.
- L8 sagt den Fortschritt 200 ms nach Start sichtbar zu. Die zehn Zahlen aus C8 sind vom Nutzer bestätigt; eine verfehlte Zusage führt zu einem eigenen Datensatz und nicht zu einer stillschweigenden Lockerung.
- Die Statuszeile hat seit S14 eine Vorrangregel für zwei Schreiber. Ein dritter ohne Regel wäre derselbe Fehler, den jene Regel behoben hat.
- Die Maxime "supersimpel" schließt eine Lösung aus, die eine Fähigkeit mit eigener Sonderregel, eigener Ausnahme und eigenem Rückfallweg erkauft.

## Recommendation

Möglichkeit 3, und zwar mit einer ausgeschriebenen Antwort auf ihren Haken. Die beiden Nachteile sind lösbar, ohne einen Sonderfall einzuführen. Die Sichtbarkeit trägt der Text selbst: die Zeile nennt den Abbruchbefehl, statt sich allein auf ihre Farbe zu verlassen. Die Rangfolge wächst von zwei auf drei Ränge und behält ihr Ordnungsprinzip, nämlich das Alter der Aussage: eine laufende Operation ist neuer als ein Ereignis am Fenster, und ein Ereignis ist neuer als der Zustand eines Ordners. Die Vorgangsanzeige bekommt dabei ein eigenes Feld und teilt sich keines mit der Fenstermeldung, weil ihre Lebensdauer die entgegengesetzte ist: eine Fenstermeldung soll beim Ordnerwechsel verschwinden, eine laufende Anzeige muss ihn überleben.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` → Schritt 16b — der Nutzer hat am 260804-1832 Möglichkeit 3 gewählt. Der Fortschritt geht in die Statuszeile des Dateifensters, das die Operation begonnen hat; der Abbruch bleibt auf `esc` und steht im Text der Zeile. Konfliktblatt, Löschbestätigung und Abschlussliste bleiben Blätter. L8 bleibt bei 200 ms, und die neun übrigen Zahlen aus C8 sind unberührt. Umgesetzt wird der Umbau in S16b; S16 bleibt abgenommen und trägt eine Notiz, die die Abweichung benennt.

---
Implemented: `5a2f05d` (S16b) und `c89ea66` (vier Ränge) — der Fortschritt steht in der Statuszeile mit eigenem Rang, der Abbruch auf `esc` im Text der Zeile; L8 mit p95 169,777 ms unter der 200-ms-Zusage abgenommen (`messungen/260805-2207-MacBookPro15-1-abnahme.txt`).
