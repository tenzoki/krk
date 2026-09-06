Sechzehn Plan- und Specdateien geschlossener Runden stehen auf `_o_` oder `_p_`

---

Zehn Plan- und Specdateien in den `planning/`-Speichern beschränkt geschlossener Runden und sechs im gemeinsamen Speicher tragen einen Marker, der Arbeit ankündigt, die längst gefahren ist. Vier davon stehen auf `_p_` — „ein Agent arbeitet gerade daran" — und keiner tut es.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** beim Nachziehen des Feldes `**Active spec/plan:**` der Runde 16, das auf einen Pfad mit `_o_` zeigte, während die Datei `_p_` trägt.

## Gemessen am 260906-0212

Über alle Circles mit einem `_b_`- oder `_c_`-Datensatz und über `shared/planning/`:

| Circle | Marker der Datei | Datei |
|---|---|---|
| `260807-2116-…` `_b_` | `_o_` | `260807-2147_*_spec-eingebauter-editor-mit-textmarken.md` |
| `260809-2040-…` `_b_` | `_o_` | `260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md` |
| `260809-2040-…` `_b_` | `_o_` | `260811-1130_*_abnahmeanleitung-tastenbelegung-als-markdown.md` |
| `260811-1257-…` `_b_` | `_o_` | `260811-1552_*_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md` |
| `260813-2332-…` `_b_` | `_o_` | `260813-2348_*_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` |
| `260814-1551-…` `_b_` | `_o_` | `260814-1830_*_spec-tippen-filtert-dateiliste-flach-und-tief.md` |
| `260823-2208-…` `_b_` | `_o_` | `260824-0613_*_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` |
| `260823-2208-…` `_b_` | **`_p_`** | `260824-0640_*_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` |
| `260830-1045-…` `_b_` | **`_p_`** | `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md` |
| `260830-1045-…` `_b_` | **`_p_`** | `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` |

Dazu im gemeinsamen Speicher: `260813-0053_*_spec-suche-…` (`_o_`), `260816-1310_*_spec-inhaltsfilter-…` (`_o_`), `260816-2240_*_spec-befehle-absetzen-…` (`_o_`, Runde zurückgestellt), `260819-2216_*_spec-auswahl-und-kopieren-…` (**`_p_`**), `260821-1115_*_spec-artefakt-und-release.md` (`_o_`), `260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (**`_p_`**, Runde 18 ohne Circle-Datensatz).

## Zwei Hälften mit verschiedenem Stand

**Die Pläne sind eindeutig und einfach nachzuziehen.** `rules/fusion-workbench-conventions.md` `## Inline State Tracking` sagt für eine Plandatei: sind alle Schritte `[DONE]`, geht der Kopf auf `**Status:** Complete` und der Dateiname auf `_c_`. Für die vier Pläne oben ist das je eine Prüfung gegen ihre eigenen Schrittmarken und keine Auslegung.

**Die Specs hängen an einer offenen Frage.** `260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md` fragt genau danach: heißt `_c_` an einem Spec „gebaut" oder „abgenommen"? In diesem Projekt fallen die zwei auseinander, weil der Abnahmelauf Nutzerarbeit ist und die meisten Runden deshalb beschränkt schließen. Solange die Frage offen ist, ist ein Spec auf `_c_` zu ziehen eine Aussage, deren Inhalt nicht feststeht.

## Was daran zählt

Der Marker ist die Auskunft, die eine Suche über den Dateibestand liefert, und `CLAUDE.md` erklärt genau diesen Bestand für verbindlich. Ein `_p_` sagt „ein Agent arbeitet daran", und eine Wiederaufnahme, die danach sucht, findet vier Dateien, an denen niemand arbeitet. Ein `_o_` an einem Spec, dessen Runde beschränkt geschlossen ist, sagt „noch nicht angefangen".

Der Fehler ist daneben schon einmal weitergereicht worden: das Feld `**Active spec/plan:**` im `_b_circle.md` der Runde 16 zeigte auf `…/260824-0640_o_plan-….md`, weil es beim Übergang `_o_` → `_p_` nicht mitgezogen wurde. Es ist am 260906 auf `_p_` nachgezogen, also auf einen Marker, der selbst falsch ist.

## Abnahme

Die vier Pläne tragen `_c_` und `**Status:** Complete`, oder ihr offener Rest steht namentlich in ihrem Kopf. Für die zwölf Specs: entweder sie tragen den Marker, den `260819-1440_*` festlegt, oder dieser Datensatz nennt sie als bewusst stehen gelassen mit dem Grund.
