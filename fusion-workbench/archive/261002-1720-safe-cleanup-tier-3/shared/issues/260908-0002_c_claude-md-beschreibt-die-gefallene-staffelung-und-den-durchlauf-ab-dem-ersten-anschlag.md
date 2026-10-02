# CLAUDE.md beschreibt die gefallene Staffelung und den Durchlauf ab dem ersten Anschlag

---
Der Absatz „Das Tippen im Dateifenster filtert seit der Runde 10 …" (CLAUDE.md,
Abschnitt `## Was man nicht sieht, wenn man es nicht weiß`) trägt vier Aussagen über den
Filter, die seit der Umsetzung von `260826-0859` und `260826-0923` falsch sind. Die
Vorgabe des Auftrags war ausdrücklich, CLAUDE.md nicht anzufassen und den Befund
abzulegen.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was dasteht und was gilt

| Zitiert in CLAUDE.md | Stand im Baum |
|---|---|
| „`inhaltsschwelle` sagt, ab welcher Länge des Filtertexts auch der Inhalt gelesen wird" | die Funktion heißt `ZEICHENSCHWELLE`, ist eine Konstante und gilt für Unterbaum **und** Inhalt |
| „Seit dem 260826 … stößt schon der erste Anschlag im Dateifenster den Durchlauf über den Unterbaum an" | der Durchlauf beginnt ab drei getippten Zeichen; ein oder zwei Anschläge stoßen keinen an |
| „An demselben Wert hängt die Schwelle des Inhaltsfilters, die `inhaltsschwelle` nach dem Stand der tiefen Suche staffelt" | die Staffelung ist gefallen; die Schwelle hängt an keinem der beiden Schalter |
| „ob die Kopplung so bleibt, ist offen (`260826-0859_*_…`)" | beantwortet und umgesetzt; der Datensatz trägt `_i_` |

Die dritte Regel des Moduls ist damit auch anders benannt und anders geschnitten, als der
Absatz sagt: sie heißt jetzt „ab welcher Länge des Filtertexts der Filter über den
angezeigten Ordner und über die bloßen Namen hinausgreift".

## Abnahmetest

`grep -n 'inhaltsschwelle\|erste Anschlag im Dateifenster' CLAUDE.md` gibt nichts aus, und
der Absatz nennt die eine Schwelle mit ihrem heutigen Namen samt dem Erhebungskommando
`awk '/pub const ZEICHENSCHWELLE/' crates/krk-core/src/verzeichnis/filter.rs`.

## Kontext

Umgesetzt in der Sitzung `260908-0002-eine-zeichenschwelle-fuer-unterbaum-und-inhalt.md`.
Die zwei Entscheidungsdatensätze sind
`260826-0859_*_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md`
und
`260826-0923_*_bekommt-der-tiefe-durchlauf-eine-eigene-zeichenschwelle-jetzt-wo-ein-anschlag-ihn-ab-werk-ausloest.md`.

---
Resolved: Alle vier Aussagen stehen auf dem heutigen Stand.

`grep -n 'inhaltsschwelle\|erste Anschlag im Dateifenster' CLAUDE.md` gibt nichts
mehr aus. Die dritte Regel des Moduls heißt im Absatz jetzt `ZEICHENSCHWELLE` und
trägt ihren heutigen Schnitt („ab welcher Länge des Filtertexts der Filter über
den angezeigten Ordner und über die bloßen Namen hinausgreift"), dazu das
Erhebungskommando
``awk '/pub const ZEICHENSCHWELLE/' crates/krk-core/src/verzeichnis/filter.rs``
statt einer Zahl.

Der Durchlauf über den Unterbaum beginnt im Text ab `ZEICHENSCHWELLE` getippten
Zeichen und nicht mehr mit dem ersten Anschlag; die Vorbelegung des Ankreuzfelds
„Deep" auf ein bleibt als eigene Aussage stehen, weil sie richtig ist
(`Ordnermodell::neu`, `tief: true`), und ist von der Schwelle getrennt. Die
Staffelung ist als gefallen bezeichnet, mit beiden Entscheiden zitiert und ohne
Verweis auf eine offene Frage. Genannt ist auch die eine Zählstelle,
`Ordnermodell::schwelle_erreicht`, die beide Frager bedient.

Belegt am Baum vor der Änderung:
`crates/krk-core/src/verzeichnis/filter.rs:257` trägt
`pub const ZEICHENSCHWELLE: usize = 3;`, eine Funktion `inhaltsschwelle` gibt es
nicht mehr, und `crates/krk-core/src/verzeichnis/modell.rs:1229` ist ihr einziger
Rufer.
