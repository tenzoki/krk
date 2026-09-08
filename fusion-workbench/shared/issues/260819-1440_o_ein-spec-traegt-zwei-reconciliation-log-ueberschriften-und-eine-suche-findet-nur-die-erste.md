Der Spec der Runde 10 trägt zwei Überschriften `## Reconciliation Log`, und eine Suche findet nur die erste

---

`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md` führt die Überschrift `## Reconciliation Log` zweimal, bei `:515` und bei `:524`. Der zweite Abgleich hat eine eigene Überschrift geschrieben, statt unter die bestehende zu schreiben.

---

**Gefunden am:** 260819-1440, Baumstand `77dcd48`
**Gefunden von:** reconciler, beim Beurteilen der offenen Spec-Marker
**Schwere:** gering. Keine Aussage der Datei ist falsch; verloren geht der Zugriff.
**Betroffen:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`
**Domain:** code

## Was es kostet

Wer den Abgleichsstand einer Planungsdatei abfragt, nimmt üblicherweise die Überschrift als Anker:

```sh
awk '/^## Reconciliation Log/,0' <datei>
```

Über dieser Datei liefert das den Block ab `:515` und hört bei `:524` nicht auf, sondern zieht die zweite Überschrift als Text mit — bei einem Muster, das beim ersten Treffer abbricht, dagegen fehlt der zweite Block ganz. Genau so ist er in diesem Durchgang zunächst übersehen worden.

**Die zwei Blöcke sagen Verschiedenes und ergänzen einander.** Der erste (`:515`) führt vier datierte Einträge vom 260814 und 260815 zur Entstehung und Nachbesserung des Spec. Der zweite (`:524`) ist der Abgleich vom 260815-1216 und prüft die zwei Kriterien C1.9 und C1.10 gegen den Baum. Keiner ersetzt den anderen, also ist das Zusammenführen und nicht das Streichen die Behebung.

## Was eine Behebung tun müsste

Die zweite Überschrift entfernen und ihren Inhalt als datierten Eintrag unter die erste ziehen, in derselben Form wie deren vier vorhandene Einträge. Der Text beider Blöcke bleibt dabei unangetastet: es ist eine Umstellung und keine Berichtigung.

## Abgrenzung

Über alle 28 Planungsdateien des Baums erhoben, am 260819-1440: diese Datei ist die einzige mit einer doppelten Überschrift, und `## Reconciliation Log` ist in ihr die einzige doppelte Überschrift überhaupt. Der Befund ist damit ein Einzelfall und keine Gestalt.

```sh
cd fusion-workbench
for f in shared/planning/*.md circles/*/planning/*.md; do
  d=$(grep '^## ' "$f" | sort | uniq -d)
  [ -n "$d" ] && echo "$f: $d"
done
```

## Ablage

Gemeinsamer Speicher, und die Herkunftsregel entscheidet das ohne Ermessen: der Befund ist in einem Abgleich ohne aktiven Circle entstanden, also nicht aus der Directive der Runde 10, in deren Spec er sitzt. Dass sein Gegenstand in einem Circle liegt, ist nach derselben Regel kein Ablagegrund — Reichweite wird zitiert und nicht abgelegt.

---
Stand 260908-1539: **dieser Datensatz bleibt offen, und das ist der beschlossene Zustand und
keine ausstehende Arbeit.**

Der Nutzer hat am 260907-1301 zu
`260906-0509_*_deckt-die-nachsatzregel-auch-eine-umstellung-im-bestandstext-ab.md`
Moeglichkeit 1 gewaehlt: die Nachsatzregel vom 260906 deckt eine **Umstellung** im
Bestandstext nicht. Eine Ueberschrift umzubenennen oder zwei Abschnitte zusammenzuziehen ist
ein Eingriff in den Wortlaut, und die Regel sagt „nicht im Text" ohne Ausnahme. Der Gewinn ist
eine Regel, die nach dem Ort entscheidbar bleibt; der Preis ist genau dieser eine offene
Datensatz.

Der Nachsatz, den die vierte Behebungsschleife am 260906 in den Spec der Runde 10 gesetzt hat,
nennt beide Abschnitte und ihre Stelle. Wer maschinell nach dem Abgleichsstand sucht, findet
ueber ihn den Hinweis auf den zweiten Block. Das ist weniger als eine Behebung und mehr als
nichts.

Der Befund selbst besteht unveraendert: die Datei traegt weiter zwei Ueberschriften
`## Reconciliation Log`, und ein Muster, das beim ersten Treffer abbricht, sieht den zweiten
Block nicht.
