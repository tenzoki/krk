Der Satz für die leere Menge sagt nicht, dass nichts zu kopieren war

---

Ein Abnahmekriterium von C2 lautet: "Ist der Ordner leer und steht damit weder eine Markierung noch eine Auswahl, bleibt die Zwischenablage unverändert, und die Statuszeile sagt, **dass nichts zu kopieren war**. Kommentarlos nichts zu tun ist nicht zulässig."

`nichts_betroffen()` liefert "nichts markiert und nichts ausgewählt" (`crates/krk-ui/src/kommandos/operationen.rs:833-835`). Der Satz nennt die Lage und nicht die Folge; das Wort "kopieren" kommt darin nicht vor, und die Probe `der_satz_fuer_die_leere_menge_gilt_beiden_befehlen` (`operationen.rs:1436-1443`) hält ausdrücklich fest, dass es nicht vorkommt.

---

**Die Abweichung ist gemeldet und begründet, der Rest ist eine Abnahmefrage.** Der Plan schlägt in Frage 7 zwei Texte vor, "nichts zu kopieren: der Ordner ist leer" und "nichts zu öffnen: der Ordner ist leer", typisiert die Funktion in Frage 6 aber gemeinsam für beide Befehle. Beides zugleich geht nicht, und der Doc-Kommentar von `nichts_betroffen` trägt die Auflösung: ein Verb machte den Satz für einen der beiden Befehle falsch, und "der Ordner ist leer" wäre daneben unwahr, weil eine leere Menge auch in einem vollen Ordner entsteht — während eines Lesevorgangs, nachdem `Ordnermodell::ersatz_einloesen` Markierung und Auswahl geleert hat.

Die Begründung trägt. Was offen bleibt, ist die Zusage von C2 im Wortlaut: der Nutzer liest bei einem leeren Ordner nicht, dass nichts kopiert wurde, sondern warum. Der Verstoß gegen "kommentarlos nichts zu tun ist nicht zulässig" liegt nicht vor.

**Drei Wege, absteigend nach Kosten.**
1. Zwei Texte statt eines, also den gemeinsamen Zuschnitt aufgeben. Kostet eine zweite Funktion und macht den Doc-Kommentar hinfällig.
2. Ein Argument, das den Befehl nennt, etwa `nichts_betroffen(Verb::Kopieren)`. Eine Aufzählung mit zwei Werten für zwei Sätze.
3. Das Kriterium von C2 auf die Lage statt auf die Folge umstellen und den heutigen Satz stehen lassen. Kostet eine Zeile im Spec und ist die Wahl, die der Baum schon getroffen hat.

**Der Nutzer entscheidet das bei der Abnahme.** Der Befund steht hier, damit die Frage bei der Durchsicht der 62 Kriterien nicht neu gestellt werden muss.

Gefunden vom `coderev` am 260811 bei der Durchsicht des Turns 1 dieses Circles.

---
Resolved: C2 verlangt im Spec, dass die Statuszeile sagt, **dass nichts zu kopieren war**. Aus
der einen verbfreien Funktion sind zwei geworden, `nichts_zu_kopieren()` und
`nichts_zu_oeffnen()`, beide ueber einen privaten Rumpf `nichts_betroffen(verb)`, der die
gemeinsame Haelfte an einer Stelle haelt. Kein neuer Typ, keine neue Aufzaehlung, kein
Auffangzweig — also Weg 2 dieses Datensatzes ohne den dort befuerchteten Preis.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1. Abgenommen mit `make check`, exit 0.

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behauptung traegt.**
`crates/krk-ui/src/kommandos/operationen.rs` fuehrt `nichts_zu_kopieren()`, `nichts_zu_oeffnen()`
und den privaten Rumpf `nichts_betroffen(verb)` bei Zeile 858. Beide Aufrufer stehen in
`appkit/tabelle.rs`: `eintragspfad_kopieren` bei `:907`, `mit_standardprogramm_oeffnen` bei `:942`.
Kein neuer Typ, keine Aufzaehlung, kein Auffangzweig.

**Eine Folge fuer den Plan, die kein Defekt ist:** die Tabelle unter `## Frage 6` des Umsetzungsplans
fuehrt noch `nichts_betroffen() -> String` als eine gemeinsame oeffentliche Funktion. Der Baum
traegt zwei oeffentliche und einen privaten Rumpf. Der Plan ist damit an dieser Zeile ueberholt,
und die Ursache ist genau dieser Datensatz. Vermerkt im Reconciliation Log des Plans.
