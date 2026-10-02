# Playmaker-Lauf 260821-2115 (orchestrator-phase4)

**Status:** Complete
**Auslöser:** nicht-interaktiver Phase-4-Auftrag nach dem Abschluss von
`260821-1644-veroeffentlichen-als-achte-station` (`_t_` → `_c_`, 260821-2110)
**Baumstand:** `66e480b`
**Domain-Bias:** `code` (aus der Zeile `**Domain:** code` des Auftrags)
**Mandat:** ranken, `portfolio.md` neu erzeugen, Vermerke schreiben. Keine der vier
bestätigungspflichtigen Rückstandsoperationen; der Lauf hielt keine Bestätigung.

## Bestand

| Marker | Bedeutung | Zahl |
|---|---|---|
| `_a_` | vorgesehen | 1 |
| `_t_` | aktiv | 0 |
| `_c_` | kohärent geschlossen | 5 |
| `_b_` | beschränkt geschlossen | 10 |
| `_s_` | überholt | 0 |
| `_d_` | zurückgestellt | 1 |

17 Circle-Verzeichnisse, 15 gefahrene Runden. `fusion-workbench/.active-circle` fehlt, und kein
Datensatz trägt `_t_`: der reguläre Zustand nach einem Abschluss, keine Warnung.

## Rangfolge der vorgesehenen Circles

**Rang 1 von 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.** Einziger
Kandidat, zum dreizehnten Mal in Folge. Ein offener Entscheidungsdatensatz bindet ihn (die
Verfügbarkeitsprüfung für Schnittstellen ab macOS 26), seine eine Abhängigkeitskante führt auf
die Runde 1.

**Abweichung von der Vorbedingungsregel, bewusst und ausgeschrieben.** Die Kante zeigt auf einen
beschränkt (`_b_`) geschlossenen Circle und bekäme mechanisch eine Marke. `CLAUDE.md` weist
diese Lesart für dieses Projekt zurück; zehn der fünfzehn gefahrenen Runden tragen `_b_`, weil
der Abnahmelauf KRK im Vordergrund verlangt und Nutzerarbeit ist. Kein Abzug angesetzt.

## Prüfung des Grounding-Schnappschusses, vom Nutzer beauftragt

Ergebnis: die Runde 15 hat nichts gealtert, und die Prüfung hat trotzdem einen Befund.

- Gemessen an den fünf Commits der Runde 15 (`72f7a5d`, `465330b`, `94855a7`, `4e810f9`,
  `26212b1`): betroffen sind `xtask/`, `README.md` und Werkstattdatensätze, kein `crates/`-Pfad.
  Die vier Alterungspunkte der Runde 14 aus dem Vermerk vom 260820-1044 gelten unverändert.
- Befund: der Schnappschuss sagt seit dem 260804 nichts darüber, was KRKs eigenes Bündel
  braucht, um `http:` selbst anzuzeigen. `resources/Info.plist` führt keinen Schlüssel
  `NSAppTransportSecurity`, im Baum kommt der Name nicht vor. Eine `.entitlements`-Datei gibt es
  nicht, signiert wird mit `--options runtime` (`xtask/src/sign.rs:226`, seit `d577295`), und
  seit dem 260820 prüft die siebte Station die Härtung nach (`xtask/src/beglaubigung.rs`).
  Nicht gemessen, benannt: beides ist Sache der Untersuchung des Darstellungsmittels.
- Vorschlag zur Neuschärfung, im Vermerk ausgeschrieben und nicht ausgeführt: vier Stücke des
  Schnappschusses wären nachzuziehen, darunter der Absatz zur Technikwahl, eine vierte offene
  Frage nach der Netzrichtlinie und die seit dem 260818 ausstehende Berichtigung aus
  `shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`.

## Zyklen und Weitergabe

- **Kein Abhängigkeitszyklus.** Der Graph über die nicht-terminalen Circles hat einen Knoten und
  keine Kante innerhalb dieser Menge. Kein `## Dependency warning` geschrieben.
- **Kein `parent-grounding-stale`-Ereignis nach der mechanischen Regel.** Sie knüpft an ein
  beschränkt (`_b_`) schließendes Kind; die Runde 15 hat kohärent geschlossen, und kein Circle
  hat in diesem Lauf nach `_b_` gewechselt. Der geschriebene Vermerk beruht auf dem
  Nutzerauftrag und sagt das in seinem ersten Absatz.

## Rückstandsspeicher

Gelesen: ein Eintrag, `_p_`. Keine `_o_`, keine `_c_`, keine `_d_`.

- Enthaltene Ideen: eine. Keine Doppelung, kein Zusammenführungskandidat.
- An `## Warnings` abgegeben: eine Hälfte des Rumpfes beschreibt einen möglichen Defekt am
  Kommentar zu `bearbeiten` in `resources/default-keymap.toml` gegen den Nutzerentscheid vom
  260802-1409. Der Playmaker filt keinen Defekt; die Stelle steht als Punkt 14 im Portfolio.
- **Rang 1 von 1:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
  — einzige lebende Idee, Datensätze auf der Platte, heute shapebar.

**Durchgeführte Schreibvorgänge: keiner.** Der Marker bleibt auf `_p_`; die Rangfolge ändert sich
nicht, also gibt es nichts umzubenennen.

**Vorgeschlagen und nicht durchgeführt, mangels Bestätigung:**

- `defer shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md until shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md beantwortet ist`
  — Grund: Eintrag und offene Frage greifen auf denselben knappen Vorrat an
  Tastenkombinationen. Möglichkeit 2 der Frage legt einen neuen Befehl auf eine erst zu
  findende freie Kombination, Möglichkeit 3 macht `f3` zur Umschalttaste und nennt die
  Nachbarschaft zu `f4` als Argument. Die Zurückstellung ist eine der vier
  bestätigungspflichtigen Operationen, und ein Phase-4-Auftrag hält keine Bestätigung.

## Geschriebene Abschnitte an Circle-Datensätzen

- `## Parent grounding stale` an
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`
- `## Activation proposal` an denselben Datensatz

Der Datensatz trägt damit 23 Playmaker-Abschnitte aus dreizehn Läufen auf 1558 Zeilen. Die
Länge ist als Punkt 15 im Portfolio vermerkt.

## Warnungen im Portfolio

15 Punkte. Neu oder geändert gegenüber dem Lauf vom 260820-1044:

- Punkt 5: der Marker `_c_` trägt jetzt fünf Lesarten; die Runde 15 kommt mit einem nicht
  prüfbaren Kriterium hinzu und hat den Marker an ihrem Spec bewusst auf `_o_` gelassen.
- Punkt 6: die Aufzeichnungslücke der Runde 14 hat sich nicht wiederholt. Die Runde 15 hat ihre
  Abnahme als Durchsicht abgelegt.
- Punkt 8: die falsche Aussage über den Ereignisabgriff ist seit `7da3098` aus `CLAUDE.md`
  verschwunden; die Rundentabelle führt weiterhin vierzehn statt fünfzehn Zeilen.
- Punkt 9: das Auslieferungstor steht wieder offen, zwei Dokumentationscommits hinter `v0.5.6`.
- Punkt 12: 152 offene Defekte statt 145, davon 44 im gemeinsamen Speicher statt 37.
- Punkt 13: 36 offene Entscheidungen statt 33. Die Zahl der beantwortet-und-nicht-umgesetzten
  fiel von gemeldeten 14 auf gemessene 6, und git kennt seit dem 260820 nur zwei passende
  Umbenennungen. Acht Datensätze sind nicht erklärt; der Punkt sagt das, statt eine der beiden
  Zahlen zu bevorzugen.

Unverändert fortgeführt: kein Zyklus (1), der unverortete Bereich Editor/Vorschau (2), der
Vermerk ohne Auslösebedingung (3), die halb überholte Grundlage (4), die nicht committeten
Abschlussdatensätze (7), der ausstehende Zusagenlauf (10), die drei offenen Abnahmeläufe (11),
der mögliche Defekt im Rückstandseintrag (14), die Länge des Circle-Datensatzes (15).

## Ausgabe

`fusion-workbench/portfolio.md`, vollständig neu erzeugt, 271 Zeilen.
