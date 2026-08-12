# Wie heißt die zur Seite gelegte Ablagedatei, und was geschieht, wenn schon eine dasteht?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-core/src/ablage/mod.rs:220-260` (`Ablage::laden`); `crates/krk-core/src/ablage/atomar.rs:22-40` (`NACHBARENDUNG`, `nachbarpfad`, und die Begründung gegen eine Laufnummer); `crates/krk-core/src/ablage/pfade.rs:70` (die vier Dateinamen); `crates/krk-core/tests/ablage.rs:159`

---

## Question

Festlegung D dieser Runde sagt: eine beschädigte Ablagedatei wird zur Seite gelegt statt überschrieben, und zwar für alle vier Dateien unter `~/Library/Application Support/KRK/`. Offen ist, wie die zur Seite gelegte Datei heißt und was geschieht, wenn beim übernächsten Start schon eine solche dasteht.

Der Fall tritt auf, und zwar an derselben Datei mehrfach: eine künftige Fassung von KRK, die `bookmarks.toml` in alter Form nicht mehr versteht, liest sie bei **jedem** Start als beschädigt. Legt sie jedes Mal zur Seite, ohne die vorige zu beachten, entsteht entweder eine wachsende Reihe oder die zweite Ablage überschreibt die erste, die als einzige die echten Lesezeichen trägt. Der zweite Fall wäre derselbe Datenverlust, gegen den die Festlegung gerichtet ist, nur einen Start später.

Das Projekt hat diese Abwägung schon einmal geführt, für die Nachbardatei des atomaren Schreibens. `nachbarpfad` (`atomar.rs:24`) leitet den Namen fest ab und trägt ausdrücklich **keine** Laufnummer: „Ein Absturz hinterlässt damit höchstens eine einzige liegengebliebene Datei statt einer wachsenden Reihe, und der nächste Schreibvorgang überschreibt sie." Der Grund trägt dort, weil niemand die Nachbardatei liest. Hier trägt er nicht: die zur Seite gelegte Datei ist genau das, was der Nutzer später lesen will.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **Fester Name, und eine bestehende Ablage wird nicht angerührt.** Etwa `bookmarks.toml.beschaedigt`. Steht sie schon da, bleibt sie, und die neue Beschädigung wird nicht ein zweites Mal gesichert.
   - Folge: es gibt höchstens eine zur Seite gelegte Datei je Ablagedatei, und es ist die **erste**, also die, die am ehesten noch die Arbeit des Nutzers trägt. Der Name ist ohne Nachschlagen zu verstehen. Es entsteht keine wachsende Reihe.
   - Preis: eine zweite, andersartige Beschädigung nach der ersten geht verloren. Das ist der seltenere Fall und der weniger wertvolle: die zweite Beschädigung entsteht aus dem Auslieferungszustand heraus, den KRK selbst geschrieben hat.

2. **Fester Name, und eine bestehende Ablage wird überschrieben.** Der Name ist derselbe, die jüngste Beschädigung gewinnt.
   - Folge: dieselbe Bauart wie `nachbarpfad`, ein Weg im Kern statt zweier.
   - Preis: der Datenverlust kehrt zurück, nur um einen Start verschoben. Beim zweiten Start überschreibt der Auslieferungszustand die zur Seite gelegte Datei mit den echten Lesezeichen. Wer den Fehler nicht am selben Tag bemerkt, hat nichts mehr.

3. **Name mit Zeitstempel, alle Ablagen bleiben stehen.** Etwa `bookmarks.toml.260812-1000.beschaedigt`.
   - Folge: nichts geht verloren, in keinem Fall.
   - Preis: eine wachsende Reihe in einem Ordner, den KRK selbst verwaltet und niemand aufräumt. Bei einer Fassung, die die Datei bei jedem Start als beschädigt liest, wächst sie pro Start um eine Datei. Genau diese Reihe hat das Projekt bei `nachbarpfad` ausdrücklich abgelehnt.

## Constraints

- Die vier Dateinamen stehen in `Datei::dateiname` (`pfade.rs:70`) und sind in `crates/krk-core/tests/ablage.rs:159` festgenagelt. Der abgeleitete Name muss sich aus ihnen ergeben und darf keine zweite Namensliste anlegen.
- Die zur Seite gelegte Datei darf keinen Namen tragen, den KRK selbst wieder als Ablagedatei liest. `Ablageort::datei` fragt nach `Datei`, und ein angehängtes Suffix fällt nicht darunter; ein vorangestelltes Präfix wäre gefährlicher.
- Der Vorgang gehört auf denselben Weg wie das atomare Schreiben, also in `crates/krk-core/src/ablage/`. Ein zweiter Schreibweg neben `atomar::schreiben` entsteht nicht.
- KRK legt heute genau eine Datei von sich aus an, `settings.toml`. Für die übrigen drei ist eine fehlende Datei der erste Start und keine Meldung wert (`Grund::NichtAnlegbar`, `mod.rs:90-98`). Das Zur-Seite-Legen darf diese Unterscheidung nicht verwischen: eine fehlende Datei wird nicht zur Seite gelegt.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie ist die einzige, die den Zweck der Festlegung wirklich einlöst: der Nutzer soll seine Lesezeichen zurückbekommen können, und was sie trägt, ist die **erste** zur Seite gelegte Fassung, nicht die letzte. Möglichkeit 2 verschiebt den Verlust um einen Start, Möglichkeit 3 legt eine Reihe an, die niemand abräumt.

Der Preis von Möglichkeit 1, dass eine zweite Beschädigung ungesichert bleibt, ist gering: nach der ersten Beschädigung arbeitet KRK auf dem Auslieferungszustand, und was danach kaputtgeht, ist Inhalt, den KRK selbst geschrieben hat.

`inference:` Wir schließen aus dem Wortlaut des Wunsches, „sicherstellen, dass lesezeichen erhalten bleiben", dass der Nutzer die Wiederherstellbarkeit meint und nicht die lückenlose Aufzeichnung jeder Beschädigung. Geprüft ist das nicht.


## Antwort 260812-1105

**Moeglichkeit 1.**

Die zur Seite gelegte Datei bekommt einen festen Namen, und eine schon dastehende wird **nicht**
ueberschrieben.

Sie ist die einzige Moeglichkeit, die den Zweck der Festlegung einloest: der Nutzer soll seine
Lesezeichen zurueckbekommen koennen, und was sie traegt, ist die **erste** zur Seite gelegte
Fassung, nicht die letzte. Ein Ueberschreiben verschoebe den Verlust um einen Start; eine
durchnummerierte Reihe legt einen Bestand an, den niemand abraeumt.

Der Preis, dass eine zweite Beschaedigung ungesichert bleibt, ist gering: nach der ersten arbeitet
KRK auf dem Auslieferungszustand, und was danach kaputtgeht, ist Inhalt, den KRK selbst geschrieben
hat.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:
