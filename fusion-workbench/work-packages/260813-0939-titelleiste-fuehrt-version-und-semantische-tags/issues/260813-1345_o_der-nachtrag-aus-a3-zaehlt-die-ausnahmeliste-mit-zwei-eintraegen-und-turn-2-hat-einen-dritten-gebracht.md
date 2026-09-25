Der Nachtrag aus A3 zählt die Ausnahmeliste mit zwei Einträgen, und Turn 2 hat einen dritten gebracht

---

Schritt A3 hat dem Defekt der Runde 6
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_o_die-blattregel-sieht-den-freigabedialog-nicht.md`)
einen Abschnitt angehängt, der festhält, was die neue Schlüsselfensterbedingung für ihn leistet
und was nicht. Der Nachtrag sagt dort (`:91-92`):

> ausser den beiden der Ausnahmeliste, `beenden` und `fenster_schliessen`

Turn 2 hat `Kommando::FensterEinblenden` hinzugefügt
(`crates/krk-ui/src/kommandos/zulaessigkeit.rs:198-201`, Commit `ed0388e`). Die Liste führt seit
diesem Commit **drei** Befehle.

---

**Schwere:** niedrig. Kein Verhalten, kein Bau. Der Nachtrag ist von Turn 1 und war beim
Schreiben richtig.

**Warum es trotzdem hierher gehört und nicht in den Speicher der Runde 6.** Der Nachtrag ist
Erzeugnis dieser Runde, in Schritt A3 ausdrücklich beauftragt; die Aussage, die er trägt, ist
eine Aussage über den Bau dieser Runde. Der Datensatz, an dem er hängt, gehört der Runde 6 und
steht ausserhalb der beiden Speicher dieses Circles — ein Abgleich darf ihn nicht bearbeiten.

**Die Sache, die der Nachtrag festhält, bleibt richtig.** Die neue Bedingung schliesst jedes
fremde **Fenster** und erreicht eine Verfolgungsschleife nicht; der Freigabewähler entsteht über
`showRelativeToRect_ofView_preferredEdge` (`crates/krk-ui/src/appkit/teilen.rs:222`) und ist
keines. Der Defekt der Runde 6 steht darum weiter offen, und die Beobachtung, die ihn entscheidet
(Shift+Cmd+S öffnen, dabei Cmd+W drücken), steht in Planschritt E2. Falsch ist allein die Zahl
der Ausnahmen in einem Nebensatz.

**Was zu tun ist**

Den Halbsatz auf drei Einträge bringen, sobald jemand den Datensatz der Runde 6 ohnehin öffnet
— also spätestens, wenn die Beobachtung aus E2 gefahren ist und er geschlossen oder mit einem
gemessenen Befund versehen wird. Für `fenster_einblenden` gilt in dieser Lage dasselbe wie für
die beiden anderen: es kommt vor einem fremden Schlüsselfenster durch.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Der dritte Eintrag ist die Behebung des einen hohen Befunds der Durchsicht:
  `260813-1258_c_fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar.md`,
  geschlossen.
