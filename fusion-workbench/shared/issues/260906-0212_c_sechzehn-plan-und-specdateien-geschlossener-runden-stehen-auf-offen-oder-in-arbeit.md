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

---
Stand 260908-1539: **einer von sechzehn ist gezogen, fuenfzehn bleiben stehen, und der
Datensatz bleibt offen.**

**Gezogen ist `260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`** (`_p_` → `_c_`,
Kopfzeile `**Status:**` auf `Complete`, neue Kopfzeile `**Abnahme:** offen`, urspruenglicher
Wortlaut als datierter Nachsatz am Dateiende). Er ist der einzige der sechzehn, an dem die
Nutzerantwort vom 260907-0823 ohne Vorbehalt greift: die Runde 18 hat keinen
Circle-Datensatz, der Plan liegt im gemeinsamen Planungsspeicher und damit ausserhalb einer
geschlossenen Runde. Alle zehn Schritte stehen auf `[DONE]`, gegen die Schrittmarken der
Datei selbst geprueft.

**Die uebrigen drei Plaene bleiben stehen, und nicht aus Unschluessigkeit.** Sie liegen in den
`planning/`-Speichern geschlossener Runden, und wie weit die Regel vom 260907-0823 in solche
Dokumente zurueckreicht, ist offen
(`260907-2340_*_wie-weit-reicht-die-neue-regel-fuer-den-zustand-eines-anforderungsdokuments-in-den-bestand-zurueck.md`).
Namentlich: `260824-0640_*_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`
(`_p_`), `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (`_p_`) und
`260816-2307_*_plan-befehle-absetzen-und-makros-speichern.md` (`_o_`, Runde zurueckgestellt).
Die zwoelf Specs haengen unveraendert an
`260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`.

**Eine Zahl dieses Datensatzes stimmt nicht.** Der erste Absatz sagt „Vier davon stehen auf
`_p_`"; es sind fuenf. Nachgezaehlt am 260908-1539 mit
``find fusion-workbench/shared/planning fusion-workbench/circles/*/planning -name '*_p_*.md'``
→ `260824-0640`, `260830-1251`, `260830-1317`, `260819-2216`, `260825-1725`. Der fuenfte ist
der Spec der Runde 14 im gemeinsamen Speicher, den die Aufzaehlung unter der Tafel selbst
nennt und den der Einleitungssatz nicht mitzaehlt. Nach dem Zug von heute sind es vier.

---
Stand 260913-1519: **zwei weitere sind gezogen, dreizehn bleiben stehen.** Der Plan und das
Anforderungsdokument der zurückgestellten Runde tragen `_d_`:
`260816-2307_*_plan-befehle-absetzen-und-makros-speichern.md` und
`260816-2240_*_spec-befehle-absetzen-und-makros-speichern.md`.

**Der Absatz darüber, der den Plan als bewusst stehen gelassen führt, ist damit überholt.**
Er stützte sich auf die Antwort in
`260907-2340_*_wie-weit-reicht-die-neue-regel-fuer-den-zustand-eines-anforderungsdokuments-in-den-bestand-zurueck.md`,
die beide Dateien auf `_o_` festhielt. Deren Grund („keine Bauarbeit") trägt gegen `_c_` und
nicht gegen `_d_`; der Nutzer hat das am 260913-1519 nachgezogen, und der Nachsatz in jenem
Datensatz schreibt es aus. Für die verbliebenen zwei Pläne innerhalb geschlossener Runden
(`260824-0640_*` und `260830-1317_*`, beide `_p_`) ändert das nichts: bei ihnen geht es um
gefahrene Bauarbeit und damit um `_c_`, nicht um eine Zurückstellung.

Die Zahl der Anforderungsdokumente, die an
`260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md` hängen, fällt
damit von zwölf auf elf.

---
**Berichtigung am 260913-1547 zum Absatz darüber.** Der Satz „zwei weitere sind gezogen,
dreizehn bleiben stehen" ist falsch. Er ist aus dem Stand vom 260908-1539 fortgerechnet
worden, statt den Dateibestand zu lesen. Zwischen jener Notiz und heute hat `68b76de`
(260909, „vierzehn Plandokumente tragen den Zustand ihrer Bauarbeit") vierzehn Dateien
gezogen, und niemand hat es hier eingetragen. Nach den zwei Zügen von heute steht keine
einzige mehr offen.

**Erhoben am 260913-1547** über alle `planning/`-Verzeichnisse außerhalb von `archive/`:
41 Dateien tragen `_c_`, zwei tragen `_d_`, keine trägt `_o_` oder `_p_`.

---
Resolved: alle sechzehn Dateien der Tafel tragen den Marker, den
`260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md` festlegt, und
keine Plan- oder Specdatei des Bestands steht mehr auf `_o_` oder `_p_`. Sie sind in drei
Zügen gezogen worden, je namentlich belegt durch `git log --diff-filter=R`:

- `85bcbad` (260908): `260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`.
- `68b76de` (260909): vierzehn, nämlich `260807-2147_*`, `260811-0753_*`, `260811-1130_*`,
  `260811-1552_*`, `260813-2348_*`, `260814-1830_*`, `260824-0613_*`, `260824-0640_*`,
  `260830-1251_*`, `260830-1317_*`, `260813-0053_*`, `260816-1310_*`, `260819-2216_*` und
  `260821-1115_*`.
- `ba11852` (260913): `260816-2240_*_spec-befehle-absetzen-und-makros-speichern.md` auf `_d_`
  statt `_c_`, weil ihr keine Bauarbeit zugrunde liegt; im selben Zug der Plan derselben
  Runde, `260816-2307_*`, den die Notiz vom 260908 zusätzlich zur Tafel genannt hat.

Die Abnahmebedingung dieses Datensatzes verlangt daneben, dass die Pläne `**Status:**`
tragen oder ihren offenen Rest im Kopf nennen. Erhoben am selben Tag: alle 41 geschlossenen
Dateien tragen eine `**Status:**`-Kopfzeile, sechzehn davon daneben die eigene Zeile
`**Abnahme:**`, die die Festlegung vom 260907-0823 verlangt. Die übrigen sind vor jener
Festlegung geschlossen worden und werden nach der Ortsregel nicht nachgeführt.

Die Zahl „vier auf `_p_`" aus dem Einleitungssatz und ihre eigene Berichtigung auf fünf sind
damit beide gegenstandslos: es steht keine mehr auf `_p_`.
