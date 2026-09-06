# Werden Defektdatensätze über eingefrorene Spec- und Plantexte geschlossen, oder bleiben sie für immer offen?

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** Werkbankführung
**Baumstand:** `2fa1d0e`

## Die Frage

Ein Teil der offenen Defektdatensätze in den Rundenverzeichnissen trifft keine Aussage über den lebenden Baum, sondern über den Wortlaut eines **freigegebenen Spec- oder Plantextes einer geschlossenen Runde**. Solche Texte sind nach `CLAUDE.md`, Abschnitt „Aufzeichnungen eines Standes", nicht nachzuführen. Der Datensatz ist damit richtig, und der Bau ist es auch — nur ist der Datensatz auf keinem Weg zu schließen, den ein Ausführer gehen dürfte.

Heute laufen diese Datensätze mit den behebbaren in derselben Liste (`find … -name '*_o_*.md'`) und blähen jede Erhebung offener Arbeit auf. Sie sind auch keine Arbeit: niemand wird sie je anfassen.

## Warum die Frage jetzt kommt

Eine Behebungsschleife in den Rundenverzeichnissen (260906, ausschließlich `crates/krk-ui/`) hat 69 offene Datensätze mit alleinigem Bezug auf jene Kiste durchgesehen. 23 waren am lebenden Baum behebbar und sind geschlossen. Unter den übrigen bildet diese Gruppe den größten zusammenhängenden Block, und für jeden ihrer Datensätze gilt derselbe Satz: der Text bleibt, wie er ist.

Die Gruppe im Einzelnen, alle mit alleinigem Bezug auf `crates/krk-ui/`:

| Datensatz | Der eingefrorene Text |
|---|---|
| `circles/260813-0939-…/issues/260813-1345_o_neun-abnahmekriterien-tragen-probe-und-haben-keine.md` | Spec der Runde 8, neun Kriterien mit **(Probe)** |
| `circles/260813-2332-…/issues/260814-1002_o_zwei-in-c3-zugesagte-proben-stehen-nicht-im-baum.md` | Spec und Plan der Runde 9 |
| `circles/260813-2332-…/issues/260814-1002_o_zwei-stellen-des-plans-sind-mit-dem-nachtrag-vom-0941-nicht-mitgezogen-worden.md` | Plan der Runde 9, `Decidability`-Zeile und Prüfstrategie |
| `circles/260814-1551-…/issues/260814-2303_o_e1-und-e3-nennen-drei-dateien-der-weg-an-das-tabmodell-fuehrt-durch-eine-vierte.md` | Plan der Runde 10, Dateilisten von E1 und E3 |
| `circles/260818-1615-…/issues/260818-1704_o_der-plan-sagt-die-proben-blieben-nach-schritt-1-gruen-sie-fallen-zu-51.md` | Plan der Runde 13, Abnahmezeile von Schritt 1 |
| `circles/260818-1615-…/issues/260818-2228_o_step-9-of-the-plan-calls-the-new-caller-the-third-and-its-own-current-state-counts-three-already.md` | Plan der Runde 13, Schritt 9 und `## API Changes` |
| `circles/260819-2230-…/issues/260820-0646_o_der-plan-schreibt-zaehlerwartungen-ohne-sie-gegen-den-baum-zu-halten-dreimal-in-einer-runde.md` | Plan der Runde 14, drei Zählerwartungen |
| `circles/260827-2028-…/issues/260828-0712_o_der-spec-nennt-make-tasten-fuer-die-markdown-ausgabe-der-belegung-die-aus-dem-menue-kommt.md` | Spec der Runde 20, C1.3 und C3.6 |
| `circles/260828-2349-…/issues/260829-0006_o_drei-baumaussagen-des-specs-der-runde-22-stimmen-mit-dem-baum-nicht-ueberein.md` | Spec der Runde 22, A2, C5.1, C5.5 |

Zwei weitere Datensätze stehen daneben und sind **nicht** dieselbe Sorte, obwohl sie auch einen fremden Spec zitieren: `circles/260827-0310-…/issues/260827-1710_*` (C2.5 der Runde 16) und `circles/260827-2028-…/issues/260828-0744_*` (C6 der Runde 1) nennen ihre Schließbedingung selbst, nämlich den ausstehenden Abnahmelauf des Nutzers. Sie gehören zur Nutzerarbeit und nicht hierher.

## Möglichkeiten

1. **So lassen.** Der Datensatz bleibt `_o_`, weil die Aussage zutrifft. Preis: jede Erhebung offener Arbeit zählt Arbeit mit, die keine ist, und jeder künftige Durchgang liest dieselben neun ein zweites Mal, um dasselbe festzustellen.

2. **Ein eigener Marker für „richtig und nicht behebbar".** Das Vokabular kennt ihn nicht; einen einzuführen träfe jede Erhebung im ganzen Werkzeugkasten und ist keine Frage dieses Projekts allein.

3. **Als `_c_` schließen, mit einer `Resolved:`-Zeile, die die Unbehebbarkeit ausschreibt.** Der Marker hieße dann nicht mehr „behoben", sondern „erledigt"; der Datensatz bleibt lesbar und fällt aus der Liste offener Arbeit. Das ist die billigste Lösung und die, die den Marker am weitesten dehnt.

4. **Ins Archiv.** Sie sind Aufzeichnungen über einen Text, der sich nicht mehr ändert, und genau das archiviert `/fusion:cleanup --only archive` sonst auch. Preis: die Aussage steht danach weiter weg von dem Spec, über den sie spricht.

Keine Möglichkeit ist hier gewählt. Der Ausführer hat die neun unangetastet auf `_o_` gelassen, wie sein Auftrag es vorgibt.

## Was daran nicht die Frage ist

Ob der Text berichtigt wird. Er wird es nicht; `CLAUDE.md` entscheidet das, und dieser Datensatz rührt daran nicht. Gefragt ist allein, wie der Befund darüber im Bestand geführt wird.
