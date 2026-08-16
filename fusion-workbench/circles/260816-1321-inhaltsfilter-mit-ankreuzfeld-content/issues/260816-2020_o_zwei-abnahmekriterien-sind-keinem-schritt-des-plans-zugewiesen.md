Zwei Abnahmekriterien sind keinem Schritt des Plans zugewiesen, und beide halten trotzdem
---
Von den 57 Abnahmekriterien des Spec
`shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` nennen die Felder
`Erfüllt:` der zwölf Planschritte 55. **Zwei stehen in keinem davon: C4.3 und C6.2.** Sie
kommen auch in keinem der elf Sitzungsprotokolle unter `history/` vor.

```sh
grep -c "C4\.3\b" fusion-workbench/circles/260816-1321-*/planning/260816-1359_o_plan-*.md   # 0
grep -rc "C6\.2\b" fusion-workbench/circles/260816-1321-*/history/                          # 0 in jeder Datei
```

**Beide halten am Baum, und beide sind belegt.** Der Befund ist damit kein Fehlverhalten der
Anwendung, sondern eine Lücke in der Zuordnung: niemand hat sie als Zusage dieser Runde
gemessen, und ohne G1 hätte sie niemand gemessen.

**C4.3 — „`Esc` räumt den Filtertext weg und beendet damit den Durchlauf. Die Liste ist danach
wieder vollständig."** Der Weg steht vollständig da und geht durch drei Stellen:
`Anwendungsdelegierter::abbrechen` (`crates/krk-ui/src/appkit/anwendung.rs:4614-4615`, der
dritte Rang) ruft `DateifensterQuelle::filter_leeren`
(`crates/krk-ui/src/appkit/tabelle.rs:2191-2202`), das nach `Ordnermodell::filter_leeren` in
`nach_filteraenderung` (`tabelle.rs:1339`) geht, und dort steht `durchlauf_nachziehen` an
erster Stelle. `Tabliste::durchlauf_nachziehen_an` (`crates/krk-ui/src/tabs.rs:878-880`) setzt
`durchlauf = None` **unbedingt und als erste Zeile**, bevor irgendeine Bedingung geprüft wird;
das `Drop` des Laufs setzt das Abbruchkennzeichen. Ein neuer Lauf entsteht nicht, weil
`filter_steht()` danach falsch ist. Die Probe
`ohne_seine_drei_bedingungen_beginnt_kein_durchlauf` (`tabs.rs:2092`, Abschnitt „Ohne
Filtertext", Zeilen 2109-2116) hält den zweiten Teil fest: nach `filter_leeren` gibt
`durchlauf_nachziehen` falsch zurück und `arbeitet_noch` ebenfalls.

**Die Zuordnungslücke hat einen erkennbaren Grund.** Der Weg stammt vollständig aus der
Runde 10 und trägt seit D1 keine Zeile mehr, die diese Runde geschrieben hätte. Wer die Schritte
nach dem fragt, was sie **ändern**, findet C4.3 in keinem; wer den Spec nach dem fragt, was
gelten soll, findet es. Genau diesen Unterschied soll G1 auffangen.

**C6.2 — „Der Filtertext wird einmal je Suche kleingeschrieben und nicht einmal je gelesener
Datei."** `Ordnermodell::filter_uebernehmen`
(`crates/krk-core/src/verzeichnis/modell.rs:906-907`) ist die eine Stelle, an der
`filter_klein` entsteht, und ihr Doc-Kommentar sagt es. `Tabliste::durchlauf_nachziehen_an`
reicht den fertigen Wert einmal je Lauf herein (`crates/krk-ui/src/tabs.rs:913`),
`Durchlauf::starten` führt ihn als `String` mit, und `traegt_der_inhalt`
(`crates/krk-core/src/verzeichnis/inhalt.rs:133-139`) nimmt ihn als `&str` entgegen, ohne ihn
anzufassen. Kleingeschrieben wird je Datei allein der **Inhalt**, in `traegt_die_folge`
(`filter.rs:114`), und das ist der vom Plan unter `## Risks & Mitigations` benannte und
angenommene Preis. Die erste Hälfte der Zusage trägt die Probe
`der_kleingeschriebene_filtertext_laeuft_mit`
(`crates/krk-core/tests/verzeichnis.rs:1012`), die zweite ist am Diff abzulesen: eine Suche
nach `to_lowercase` und `to_ascii_lowercase` über `filter.rs`, `inhalt.rs`, `durchlauf.rs`,
`modell.rs` und `tabs.rs` nennt genau zwei Stellen, `modell.rs:907` für den Filtertext und
`filter.rs:114` für den verglichenen Text.
---
Gefunden bei Schritt G1 des Plans
`planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, beim Zählen der 57 Kriterien
gegen die `Erfüllt:`-Felder der zwölf Schritte und gegen die elf Sitzungsprotokolle.

**Warum das aufgeschrieben gehört, obwohl nichts kaputt ist.** Ein Kriterium, das kein Schritt
beansprucht, wird bei der Abnahme leicht für erfüllt gehalten, weil die Liste der Schritte
vollständig aussieht. Die Runde 8 hat den umgekehrten Fall aufgeschrieben — neun Kriterien,
die eine Probe versprechen und keine haben
(`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/issues/260813-1345_o_neun-abnahmekriterien-tragen-probe-und-haben-keine.md`)
—, und die Runde 7 denselben
(`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0647_o_neun-abnahmekriterien-versprechen-eine-probe-und-haben-keine.md`).
Dieser Datensatz führt die dritte Form derselben Lücke: das Kriterium, das kein Schritt nennt.

**Beide stehen in der Abnahmeliste** `messungen/260816-abnahme-inhaltsfilter.md` mit ihrem
Nachweis und als solche gekennzeichnet, damit der Lauf sie nicht überspringt.

**Ein Vorschlag, und er gehört nicht in diese Runde.** Ein Schritt, der die `Erfüllt:`-Felder
eines Plans gegen die nummerierten Kriterien seines Spec abgleicht, findet diese Lücke vor dem
Bau statt nach ihm. Er wäre eine Zählprobe wie die im Filter, nur über die Datensätze statt
über den Quelltext.
