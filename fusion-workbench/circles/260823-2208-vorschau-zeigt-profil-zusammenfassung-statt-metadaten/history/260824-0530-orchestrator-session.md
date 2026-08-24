# Orchestrator-Sitzung — 260824-0530

**Directive:** die Directive des aktiven Circles `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (Abschnitt `## Directive` seines Datensatzes)
**Modus:** (noch nicht aufgelöst — Phase 0 steht aus)
**Status:** Setup abgeschlossen

## Aufnahme beim Start

| Größe | Stand |
|---|---|
| Git HEAD | `278a008` |
| Aktiver Circle | `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (seit dieser Sitzung, `_t_`) |
| Offene Defekte im Circle | 0 |
| Offene Defekte, gemeinsamer Speicher | 54 |
| Offene Planschritte im Circle | 0 (kein Spec, kein Plan) |
| Offene Planschritte, gemeinsamer Speicher | 5 Dateien |
| Offene Entscheidungsfragen im Circle | 2 |
| Offene Entscheidungsfragen, gemeinsamer Speicher | 14 |
| Circles | 1 aktiv, 5 kohärent, 10 beschränkt, 2 zurückgestellt |
| Turn-Budget | 12 |
| Domäne | `code` |

## Vorgeschichte dieser Sitzung

Die vorige Sitzung (`shared/history/260823-2119-orchestrator-session.md`) hat den Backlogeintrag
`shared/backlog/260823-2136_o_readerconventions-profile-fuer-dateizugriff.md` aufgenommen, den
Shaper über `/fusion:direct` den Circle anlegen lassen und ihn über `/fusion:next` aktiviert.
Der Backlogeintrag bleibt offen: der Circle nimmt nur die Zusammenfassungs-Hälfte, die zweite
Hälfte steht noch darin.

## Zwei offene Entscheidungsfragen im Circle

- `decisions/260823-2208_o_gilt-ein-profil-nur-fuer-ordner-oder-auch-fuer-einzelne-dateien.md`
- `decisions/260823-2208_o_liefert-krk-ein-fertiges-fusion-workbench-profil-mit.md`

Beide gehören in die Klärung dieser Runde und binden die Planung.

## Vom Playmaker offen gelassen

Zwei Rückstands-Operationen sind vorgeschlagen und nicht ausgeführt (ein Aufteilen des
ReaderConventions-Eintrags, ein Schließen von `260813-2033`). Die explizite Form von
`/fusion:next` legt sie dem Nutzer nicht vor; sie stehen im Portfolio bis zum nächsten
Lauf ohne Argument.

Zwei Warnungen aus dem Portfolio: `CLAUDE.md` sagt weiter, es gebe keinen vorgesehenen Circle,
und der Datensatz `260816-2255-befehle-absetzen-und-makros-speichern` trägt `(offen)` über
seiner ausgeschriebenen Schließungsnotiz.

## Beantwortete Fragen (Nutzer, 260824-0530)

**Gilt ein Profil nur für Ordner, oder auch für einzelne Dateien?** Nur Ordner
(Möglichkeit 1). Dateien bleiben bei der Dreiteilung aus C6 der Runde 1: Text bis 1 MB,
Bild bis 64 MB, sonst Metadaten. Die Dateifrage bleibt einer späteren Runde überlassen und
verlangt keinen Rückbau an der Erkennungsregel.

**Liefert KRK ein fertiges fusion-workbench-Profil mit?** Mitgeliefert und wirksam
(Möglichkeit 1). `resources/default-readers.toml` wird über `include_str!` eingebettet und
beim ersten Start wörtlich angelegt, denselben Weg wie `settings.toml`. Der Preis ist eine
Pflegeaufgabe: ändert fusion seine Ablagekonventionen, zieht das Profil nach. Ein veraltetes
Profil verschlechtert nichts, weil ohne Treffer die heutige Metadatenanzeige stehen bleibt.

**Weg dieser Sitzung:** erst Spec über den Shaper, dann Plan. Der Nutzer sieht die
Abnahmekriterien vor der Planung.

## Beantwortete Fragen des Shapers (Nutzer, 260824-0555)

**Wie zieht der Baustein „ein Feld aus einer Datei" seinen Wert?** Regulärer Ausdruck mit
Fanggruppe (Möglichkeit 3). Trägt alle sechs skizzierten Fälle vollständig, einschließlich der
JSON-Felder aus `.fusion-setup`. Der Nutzer nimmt dafür zwei genannte Kosten in Kauf: eine
fremde Kiste, die der Baum heute nicht führt, und die Rückkehr einer Ausdruckssprache, die er
am 260823 für die Profilregeln abgelehnt hatte. Die Ablehnung galt dem Bausteinsatz als
Ganzem; der reguläre Ausdruck bleibt auf diesen einen Baustein beschränkt.

**Was heißt „die jüngsten zehn", und was ist ihr Titel?** Nach Änderungsdatum sortiert, Titel
ist die erste Überschriftenzeile (Möglichkeit 2). Kosten laut Shaper: zehn Dateiöffnungen je
Zusammenfassung, der Zustandsmarker verschwindet aus der Liste, und ein nachträglich
bearbeiteter alter Datensatz rutscht nach vorn. Die Dateiöffnungen berühren die Zeitzusage L7.

**Was zeigt die Zusammenfassung, wenn ein Baustein ins Leere greift?** Die Zeile steht mit
einem Platzhalter (Möglichkeit 2). Das Veralten eines Profils bleibt ablesbar.

## Beantwortete Fragen der zweiten Klärungsrunde (Nutzer, 260824-0610)

**Welche Form hat das Pfadmuster?** Regulärer Ausdruck auf dem vollen Pfad (Möglichkeit 1).
Dieselbe Form wie beim Feldbaustein, also eine Mustersprache in `readers.toml` statt zweier.
Die mitgelieferte Datei bliebe bei etwa fünf Profilen.

**Titel der jüngsten zehn.** Erste nicht leere Zeile (Möglichkeit 1). Das berichtigt die
Antwort vom 260824-0555, deren Überschriftenzeile keinen einzigen Defektdatensatz erreicht
hätte: 82 Dateien in `shared/issues/` und 157 im größten Circle-Speicher tragen kein `#`.
Die Sortierung nach Änderungsdatum bleibt, wie am 260824-0555 entschieden.

**Sitzungsinfo der Wurzelzusammenfassung.** Aus `orchestrator-live.md` (Möglichkeit 1).
`agentstate.yaml` steht in dieser Werkbank nicht da und ist in `.gitignore` geführt. Der
Preis: der Ausdruck hängt an einer Zeilenform, die fusion ändern kann; dann setzt dieser eine
Baustein seinen Platzhalter.

## Spec-Tor (Nutzer, 260824-0625)

**Freigegeben, A1 bis A7 stehen.** Der Nutzer hat die sieben abgeleiteten Festlegungen ohne
Widerspruch bestätigt, A7 eingeschlossen: der Zustand eines Circles wird über den Baustein
„Vorhandensein" mit je einer Zeile für vorgesehen, aktiv und geschlossen ausgedrückt, statt
den festen Bausteinsatz um einen fünften Baustein zu erweitern.

Spec: `circles/260823-2208-.../planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`
Sechs Fähigkeiten, 56 Abnahmekriterien. Für L7 abzählbare Grenzen statt einer Zeitmessung.

## Plan-Tor (Nutzer, 260824-0705)

**Freigegeben.** Der Nutzer nimmt `regex` 1.x als fremde Kiste auf. Der Grund steht im
Plankopf unter `**Decidability:**`: ob ein Ausdruck aus der `readers.toml` die Vorschau
anhält, ist aus dem Text des Ausdrucks nicht entscheidbar, also wechselt der Plan den
Mechanismus statt ihn anzunähern. `fancy-regex` mit Schrittgrenze wäre die Näherung gewesen.

Plan: `circles/260823-2208-.../planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`
13 Schritte in fünf Bündeln: elf `coder`, einer `ontocoder`, einer `analyst`.

Das Kopffeld `**Active spec/plan:**` des Circle-Datensatzes zeigt seit diesem Tor auf den
Plan, und der Abschnitt `## Directive` trägt an Stelle der Prosa den festen Zeiger auf dieses
Feld.
