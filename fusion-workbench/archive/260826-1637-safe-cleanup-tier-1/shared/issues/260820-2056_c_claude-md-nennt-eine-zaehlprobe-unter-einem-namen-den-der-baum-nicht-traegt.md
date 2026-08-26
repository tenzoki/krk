CLAUDE.md nennt eine Zählprobe unter einem Namen, den der Baum nicht trägt

---

`CLAUDE.md:131` verweist für die Rufer der drei Filterregeln ausdrücklich auf eine Probe statt auf
eine Zahl: „Wie viele Rufer jede hat, sagt die Zählprobe
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_…` in `crates/krk-core/tests/verzeichnis.rs`
und nicht diese Zeile." Diesen Namen trägt im Baum keine Probe. Sie heißt seit der Runde 11
`die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`
(`crates/krk-core/tests/verzeichnis.rs:3095`).

---

**Schwere:** gering für die Sache, mittel für die Form. Die Aussage darüber stimmt, und die Probe
tut, was der Satz ihr zuschreibt. Kaputt ist der Zeiger, und er ist an einer Stelle kaputt, an der
`CLAUDE.md` den Leser bewusst vom eigenen Text wegschickt. Wer dem Verweis folgt, findet nichts und
muss die Auskunft neu erheben — also genau die Arbeit tun, die der Verweis ihm ersparen sollte.

**Gefunden von:** reconciler, Abgleich `shared/history/260820-2056-reconciliation.md`, beim Prüfen
des Kriteriums C6.3 aus dem Spec der Runde 11.
**Betroffen:** `CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß", Absatz „Das
Tippen im Dateifenster filtert seit der Runde 10".
**Domain:** code

## Gemessen, an `f5300f4`

```
$ grep -c 'die_zeichenregel_und_der_vergleich' crates/ -r --include='*.rs'
0

$ grep -rn 'fn die_zeichenregel' crates/
crates/krk-core/tests/verzeichnis.rs:3095:fn die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei() {
```

Der alte Name stammt aus der Runde 10, wo die Probe zwei Regeln mit je zwei Rufern hielt. Die
Runde 11 hat sie auf drei Rufer für den Vergleich nachgezogen und dabei umbenannt; der Name trägt
die Zahlen seither selbst. `CLAUDE.md` nennt die drei Regeln richtig und den Namen der Probe falsch.

**Die Kürzung mit Auslassungszeichen macht es schlimmer und nicht besser.** Der zitierte Name endet
auf `…`, also liest ein Suchender ihn als Präfix und sucht danach; `die_zeichenregel_` allein trifft,
`die_zeichenregel_und_der_vergleich` trifft nicht mehr. Das ist dieselbe Gestalt, die
`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`
für Datensatzverweise beschreibt, hier an einem Probennamen.

## Was zu tun ist

Den Namen in `CLAUDE.md` durch den heutigen ersetzen. Der Rest des Satzes bleibt richtig und ist
nicht anzufassen. Das ist Kuratorarbeit an einer der drei normativen Flächen, nicht Codearbeit.

**Der Spec der Runde 11 nennt denselben alten Namen** (Kriterium C6.3,
`shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md:169`). Dort bleibt er stehen: ein
Spec zeichnet auf, was zugesagt war, und die Umbenennung ist nach der Sache eine Einlösung von C6.3
und kein Bruch — die Probe behält ihre namentliche Liste, nennt den dritten Rufer
(`krk-core/src/verzeichnis/inhalt.rs`) und ist nicht durch eine bloße Zahl ersetzt.

---
Resolved: 90f8ac1 — `CLAUDE.md:143` nennt die Probe seit dem 260820 unter ihrem heutigen Namen
`die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`, ungekürzt und ohne Auslassungszeichen;
der Rest des Satzes ist unangetastet, wie dieser Datensatz es verlangt. Der Baum trägt sie unter
diesem Namen (`crates/krk-core/tests/verzeichnis.rs:3244`), der alte Name kommt unter `crates/`
nicht mehr vor, und `make check` über `c95f28b` fährt sie grün. Am 260826-1017 gegen den Baum
gelesen. **Der Marker ist rund sechs Tage nach der Behebung nachgezogen**: der Kuratorenlauf
`260826-0818` hat den Fall benannt und den Marker dem `reconciler` zugewiesen, abgelegt als
`shared/issues/260826-0923_*_drei-behobene-claude-md-datensaetze-stehen-weiter-offen-und-niemand-ist-dafuer-beauftragt.md`.
Der Spec der Runde 11 nennt den alten Namen weiter und bleibt zu Recht unangetastet.
