# Was tut der Ordnersprung, wenn es keine angezeigte Datei oder keinen erreichbaren Ordner gibt?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/vorschaumodell.rs:434` (`aktiver_pfad`, liefert `Option<PathBuf>`); `crates/krk-ui/src/editormodell.rs:621` (`pfad`, liefert `Option<&Path>`); `crates/krk-ui/src/fenstermodell.rs:191` (`Bereich::teilt_flaeche_mit`); `crates/krk-ui/src/kommandos/operationen.rs:858` (`nichts_betroffen`, die Meldungsvorlage der Runde 4)

---

## Question

Der Befehl setzt voraus, dass es eine angezeigte Datei gibt und dass ihr Ordner lesbar ist. Beides kann fehlen, und zwar auf vier verschiedene Weisen. Die Fallunterscheidung muss trennscharf und vollständig sein, wie es die übrigen dieses Programms sind, sonst fällt ein Fall stumm durch.

Die vier Fälle, am Bestand abgelesen:

1. **Weder Vorschau noch Editor ist sichtbar.** `Bereich::teilt_flaeche_mit` schließt aus, dass beide zugleich stehen, aber nicht, dass beide fehlen: der Nutzer kann sie über die Bereichsleiste der Runde 5 beide abschalten.
2. **Der sichtbare Bereich zeigt keine Datei.** Ein Vorschau-Tab kann leer sein, Metadaten zeigen, einen Hinweis tragen oder Text aus der Zwischenablage halten; in allen vier Lagen liefert `aktiver_pfad` den Wert `None`. Der Editor liefert `None`, solange er keine Datei hält.
3. **Die angezeigte Datei ist verschwunden.** Der Pfad steht, die Datei ist inzwischen gelöscht oder verschoben. Der Ordner darüber besteht noch.
4. **Der Ordner über der Datei ist verschwunden oder nicht lesbar.** Der Lesevorgang beginnt und scheitert.

Der vierte Fall braucht keine eigene Antwort: er ist der gewöhnliche Fehlschlag eines Lesevorgangs, und den behandelt das Dateifenster seit der Runde 1 über seine Statuszeile. Zu entscheiden sind die ersten drei.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **Alle drei Fälle melden dasselbe: der Befehl tut nichts, und die Statuszeile sagt, dass keine Datei angezeigt wird.** Im dritten Fall springt der Ordnersprung trotzdem, weil der Ordner ja besteht, und allein die Auswahl bleibt aus.
   - Folge: ein Satz für die Fälle 1 und 2, das gewöhnliche Verhalten für Fall 3. Der Nutzer bekommt bei Fall 3 den Ordner zu sehen, was die Auskunft ist, die er wollte; dass die Datei fort ist, sieht er daran, dass sie nicht dasteht.
   - Preis: die Fälle 1 und 2 bekommen dieselbe Meldung, obwohl sie verschiedene Ursachen haben. Wer den Editor abgeschaltet hat, liest „keine Datei angezeigt" und sucht den Fehler bei der Datei.

2. **Drei Meldungen, eine je Fall.** „Kein Editor und keine Vorschau sichtbar", „der angezeigte Inhalt ist keine Datei", „die Datei besteht nicht mehr, der Ordner wird trotzdem gezeigt".
   - Folge: der Nutzer weiß in jedem Fall, was zu tun ist. Die Unterscheidung ist billig, weil die drei Fälle im Code ohnehin auseinanderfallen.
   - Preis: drei Meldungstexte für einen Befehl, den es heute nicht gibt. Die Runde 4 hat für ihre vier Befehle zusammen zwei Sätze gebraucht (`nichts_betroffen` mit einem eingesetzten Verb), und das war ausdrücklich die sparsame Bauart.

3. **Der Befehl wird gar nicht erst angeboten, solange keine Datei angezeigt wird.** Das Kommando meldet sich als nicht ausführbar, und der Menüeintrag ist grau.
   - Folge: kein Meldungstext nötig, und die Oberfläche sagt die Antwort, bevor der Nutzer fragt.
   - Preis: KRK kennt heute keinen grauen Menüeintrag und keine Zustandsprüfung vor der Ausführung. Der Weg dahin ginge über `validateMenuItem:` und wäre ein neuer Mechanismus, den kein anderer Befehl dieses Programms benutzt. Der Tastenweg bliebe davon ohnehin unberührt und bräuchte weiterhin eine Meldung.

## Constraints

- Die Fallunterscheidung muss vollständig sein und ohne Auffangzweig auskommen, wie `Vorschaumodell::zeigt_dateitext` (`crates/krk-ui/src/vorschaumodell.rs:451`) und die übrigen dieser Art. Ein sechster Vorschauinhalt soll den Bau anhalten und nicht stumm im falschen Zweig landen.
- Die Meldung gehört in die Statuszeile des betroffenen Dateifensters. Der Weg dorthin ist gebaut; der Kern gibt nichts aus (`crates/krk-core/src/ablage/mod.rs`, Abschnitt „Der Kern gibt nichts aus").
- Der Befehl trägt einen Wirkungsbereich aus `Wirkungsbereich` (sieben Werte, `crates/krk-core/src/tasten/belegung.rs`). Welcher es ist, entscheidet mit, aus welchem Fokus heraus er überhaupt anläuft, und damit, wie oft Fall 1 auftritt.

## Recommendation

**Wir empfehlen Möglichkeit 1 mit einer Ergänzung:** ein Satz für die Fälle 1 und 2 zusammen, aber formuliert vom Ergebnis her und nicht von der Ursache, etwa „keine angezeigte Datei, zu der gesprungen werden könnte". So gelesen ist es keine falsche Auskunft an den, der den Editor abgeschaltet hat, sondern eine richtige.

Fall 3 sollte springen und nicht abbrechen. Der Nutzer hat den Befehl gedrückt, weil er den Ordner sehen will; dass die Datei darin fehlt, ist eine Auskunft und kein Grund, ihm den Ordner vorzuenthalten.

Möglichkeit 3 empfehlen wir nicht: sie führt einen Mechanismus ein, den dieses Programm nirgends benutzt, und löst den Tastenweg nicht.


## Antwort 260812-1105

**Moeglichkeit 1 mit Ergaenzung.**

Ein Satz in der Statuszeile, **vom Ergebnis her formuliert und nicht von der Ursache**: „keine
angezeigte Datei, zu der gesprungen werden koennte". So gelesen ist er auch fuer den richtig, der
den Editor abgeschaltet hat, statt ihm eine falsche Auskunft zu geben.

**Fall 3 springt und bricht nicht ab.** Ist der Ordner da und die Datei darin verschwunden, zeigt
KRK den Ordner. Der Nutzer hat den Befehl gedrueckt, weil er den Ordner sehen will; dass die Datei
fehlt, ist eine Auskunft und kein Grund, ihm den Ordner vorzuenthalten.

Ein eigener Mechanismus fuer die Abweisung ist abgelehnt: das Programm benutzt nirgends einen, und
er loest den Tastenweg nicht.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:
