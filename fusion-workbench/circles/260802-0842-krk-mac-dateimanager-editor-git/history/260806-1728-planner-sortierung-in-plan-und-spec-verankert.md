# Planner 260806-1728: die Sortierung in Plan und Spec verankert

**Status:** Complete
**Agent:** planner
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Auftrag:** Nachzug an Plan und Spec, damit der Entscheidungsdatensatz zur Sortierung an den Stellen steht, für die er sich bindend erklärt. Kein Codeeingriff, kein Eingriff an Entscheidungs- oder Defektdateien, kein Commit.

## Anlass

Der Abgleich vom 260806-1647 fand, dass `decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md` sich selbst für Schritt 12 bindend erklärt, S12 seit dem 260804-1040 `[DONE]` trägt und weder Plan noch Spec den Datensatz an einer einzigen Stelle nennt. Gemeldet als `issues/260806-1647_*_die-sortierfrage-bindet-s12-und-steht-in-keiner-planstelle.md`. Der Nutzer hat die Frage am 260806 mit Möglichkeit 1 beantwortet, Commit `16e4558` hat sie umgesetzt.

## Geändert

Zwei Dateien, beide unter `planning/`, beide behalten ihren Marker `_o_`; kein Schritt verliert sein `[DONE]`.

`260802-1428_o_plan-navigator-geruest-runde-1.md`:

- Datumszeile um die drei Nachzüge vom 260806 ergänzt, die sie überging.
- Neuer Absatz **Nachzug 260806-1728** im Kopf.
- `### Frage 2`: neuer Absatz zum Sortierschlüssel als Kollationsschlüssel. Der vorhandene Absatz war die einzige Planstelle, die den Schlüssel überhaupt berührte, und begründete die Vorberechnung mit den Kosten eines sprachsensitiven Vergleichs.
- S2: Abweichungsnotiz mit dem Nutzerentscheid, der Umsetzung und den nachgemessenen Zahlen für L3 und L10.
- S12: Notiz zur Fundstelle, mit dem wörtlichen Bindungssatz des Datensatzes.
- `## Datenstrukturen`: `Eintrag` um `endungsschluessel` und `endung_ab` erweitert, dazu ein erläuternder Absatz.
- `## Angelegte Defekte und Entscheidungen`: zwei neue Einträge, der Sortierdatensatz und die Folgefrage nach der Sprache der Ordnung.

`260802-1036_o_spec-navigator-geruest.md`:

- Datums- und Statuszeile fortgeschrieben.
- Neuer Absatz **Stand 260806-1728** im Kopf.
- C2: das Abnahmekriterium zur Sortierung sagt jetzt, wonach beide Sortierungen ordnen; dazu eine neue Festlegung mit dem Datensatz, der CLDR-Wurzelordnung und der offenen Folgefrage.
- C8: neuer Absatz `L3 und L10 decken das Sortieren mit`, mit den nachgemessenen Zahlen.
- `## Offene Nutzerentscheidungen`: neuer Absatz zum Entscheid vom 260806 und zur offenen Folgefrage; die Behauptung "Keine offene Frage an den Nutzer" ist auf den heutigen Stand gezogen.

## Befund nebenbei

Der Datensatz nennt in seinen Cross-references den Abschnitt **C1** des Specs. Die Sortierung steht dort nicht und stand dort nie; sie gehört seit der ersten Fassung des Specs zu C2. Wer der Angabe folgte, fand in C1 nichts über Sortierung und keinen Ort für einen Nachtrag.

Fünf weitere offene Entscheidungsdatensätze nennen einen Planschritt oder eine Fähigkeit des Specs, ohne dort genannt zu werden: `260805-1730_o_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor` (S18, C5, C7), `260805-1845_o_wann-eine-von-hand-geaenderte-settings-toml-wirkt` (S18c, C3), `260805-2216_o_tastenweg-des-fokus-in-das-vorschaufenster` (S19, C6, C10), `260805-2252_o_entfernen-einer-einzelnen-kombination-in-der-belegungsansicht` (S20, C3) und `260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund` (S21, S22, C8). Keiner von ihnen schreibt sich eine Bindung zu, wie der Sortierdatensatz es tut; die Asymmetrie ist damit eine Stufe schwächer, aber dieselbe.

Zwei dieser Datensätze schreiben in ihren Cross-references den Marker aus (`260805-1623_a_…`, `260802-1428_o_…`), gegen die Regel `## Wie dieser Plan auf Datensätze verweist`.

Die Defektdatei ist nicht angefasst und nicht umbenannt; der Nutzer schließt sie selbst.
