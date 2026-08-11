Der Doppelklick räumt die Befehlsantwort nur an seiner eigenen Fensterseite weg

---

`DateifensterQuelle::doppelklick` ruft `self.befehlsantwort_loeschen()` (`crates/krk-ui/src/appkit/tabelle.rs:943`), also an genau der Quelle, in der geklickt wurde. Die Regel, auf die sich der Doc-Kommentar daneben ausdrücklich beruft, räumt beide Seiten: `Anwendungsdelegierter::kommando_ausfuehren` läuft über `for seite in Fensterseite::ALLE` (`crates/krk-ui/src/appkit/anwendung.rs:2009-2010`).

Der Kommentar in `tabelle.rs:936-940` sagt: "Es ist dieselbe Regel und keine zweite: was KRK auf die letzte Handlung geantwortet hat, gilt bis zur nächsten." Umgesetzt ist sie halb so breit.

---

**Der Fall, an dem es sichtbar wird.** Jede Fensterseite hat ihre eigene Statuszeile (`Statuszeile` als Feld der `DateifensterQuelle`, `tabelle.rs:269`), und die Befehlsantwort ist der oberste ihrer fünf Ränge.

1. Fokus im linken Dateifenster, `shift+cmd+c` auf sieben markierten Einträgen. Die linke Statuszeile zeigt "7 Pfade kopiert".
2. Doppelklick auf einen Ordner im **rechten** Dateifenster. Rechts wird eingestiegen, rechts ist die Antwort geräumt (es stand keine).
3. Links steht "7 Pfade kopiert" weiter, obwohl der Nutzer inzwischen etwas anderes getan hat.

Der Grund, aus dem der Delegierte beide Seiten räumt, steht bei ihm im Kommentar (`anwendung.rs:2003-2008`): "weil es genau einen letzten Befehl gibt und nicht einen je Seite". Für eine Handlung mit der Maus gilt derselbe Satz.

**Zwei Wege, und der zweite ist der kleinere.** Entweder räumt der Doppelklick beide Seiten — dann braucht die Quelle einen Weg an ihre Nachbarin, den sie heute nicht hat, und der Aufruf gehörte eher an den Delegierten als an die Quelle. Oder der Kommentar sagt, was der Code tut, und nennt die Einschränkung: der Doppelklick räumt seine eigene Seite, die Antwort der anderen bleibt bis zum nächsten Tastenbefehl stehen. Welcher der beiden richtig ist, hängt daran, ob die Regel "eine letzte Handlung im Fenster" oder "eine letzte Handlung je Seite" heißen soll; heute sagen die beiden Aufrufer Verschiedenes.

Kein Abnahmekriterium des Specs ist davon berührt: C3 sagt zum Doppelklick nichts über die Statuszeile der anderen Seite.

Gefunden vom `coderev` am 260811 bei der Durchsicht des Turns 1 dieses Circles.

---
Resolved: **Der Kommentar ist geaendert, nicht das Verhalten**, und die Begruendung traegt.

Der breitere Weg braeuchte einen dritten Rueckruf von der `DateifensterQuelle` zum
Anwendungsdelegierten; die Quelle hat heute zwei und keinen Weg zur Nachbarseite. Das waere ein
neuer Mechanismus fuer eine Zeile Anzeige.

Der Kommentar traegt jetzt die Regel, die beide Aufrufer wirklich teilen: **geraeumt wird so
weit, wie die Handlung reicht.** Ein Kommando reicht ueber beide Dateifenster, `Kopieren`
schreibt in das unbeteiligte, und der Delegierte raeumt darum beide; der Doppelklick reicht ueber
die eine angeklickte Zeile und raeumt die eine Statuszeile, an der er sitzt. Dieselbe Regel, und
die Reichweite ist der Grund und nicht der Preis.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1. Abgenommen mit `make check`, exit 0.
