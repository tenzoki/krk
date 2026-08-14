Zwei Stellen des Plans sind mit dem Nachtrag vom 260814-0941 nicht mitgezogen worden

---

Der Nachtrag des Plans vom 260814-0941 nennt sechs nachgezogene Stellen: die Schritte 10 bis
14, den Kasten `zettel_sichern` im Bild der Sicherungsmomente und die Risikozeile zu zwei
Instanzen. Zwei weitere Stellen desselben Dokuments tragen den Stand vor dem Nachtrag.

| Stelle | Was dort steht | Was seit dem 260814-0925 gilt |
|---|---|---|
| Die `**Decidability:**`-Zeile im Kopf | die drei Eingaben seien „dem beim Öffnen gelesenen Text, dem Stand der Textfläche und dem offenen Zettel" | Die Frage „in welche Datei" ist seit dem Nachtrag über **beide** Zettel zu beantworten. `Zettelmodell::zu_sichern` (`crates/krk-ui/src/zettelmodell.rs:248`) läuft über `Zettel::ALLE`, und der gehaltene Stand des **nicht** offenen Zettels steht im Modell und nicht in der Textfläche. Die Textfläche allein reicht als zweite Eingabe nicht mehr. |
| `## Testing Strategy`, Absatz „Am Modell, ohne Fenster" | „Das Zettelmodell trägt sein eigenes Prüfmodul: Wechsel, Änderungserkennung, ‚derselbe Tab schreibt nichts'." | Drei Gegenstände, und der vierte fehlt: die Regel „der getippte Stand gewinnt" mit den drei Proben, die Schritt 14 seit dem Nachtrag verlangt — `das_oeffnen_setzt_den_abweichenden_stand_nicht_zurueck` (`:417`), `ein_sauberer_zettel_bekommt_den_neuen_dateiinhalt` (`:438`), `jeder_abweichende_zettel_steht_zur_sicherung_an` (`:456`). |

---

**Schwere:** niedrig. Kein Bau, kein Verhalten, und **kein Widerspruch zum Spec**: die
tragende Aussage der Decidability-Zeile hält unverändert, denn alle Eingaben liegen im selben
Augenblick vor und keine wird vorhergesagt. Beide Stellen sagen weniger, als der Bau leistet,
und nicht etwas anderes.

**Warum es trotzdem aufgeschrieben ist.** Die Decidability-Zeile ist die eine Zeile im
Plankopf, die nach `rules/critical-stance.md` ausdrücklich dafür da ist, dass ein Mensch am
Freigabe-Tor die tragende Frage sieht. Eine Eingabeliste, die eine Eingabe zu wenig nennt,
gibt am nächsten Tor eine engere Frage zur Prüfung als die, die der Bau beantwortet. Der
Absatz in `## Testing Strategy` ist die Aufstellung, an der ein späterer Leser abliest, was
das Prüfmodul deckt; die drei Proben, die den hohen Befund der Durchsicht von Turn 1 halten,
kommen darin nicht vor.

**Was zu tun ist.** In der Decidability-Zeile die dritte Eingabe auf „den gehaltenen Stand
beider Zettel" ziehen; im Absatz zur Prüfstrategie den vierten Gegenstand nachtragen. Beides
ist je eine Zeile und gehört dem Planer.

**Kontext**

- Gefunden beim Abgleich der Runde 9, `history/260814-1002-reconciliation.md`.
- Der Nachtrag selbst: Kopfnotiz des Plans vom 260814-0941, gebaut in `79dab20`; Anlass sind
  die zwei Defektdatensätze `issues/260814-0908_*` und `issues/260814-0909_*`, beide
  geschlossen.
