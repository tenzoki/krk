# Abgleich 260820-2056 — Sitzungsabschluss ohne laufende Sitzung

**Status:** Complete
**Reconciler**, Domäne `code`, Baumstand `f5300f4`, Arbeitsbaum zu Beginn sauber, alles nach
`origin/main` geschoben.
**Anlass:** Schritt 3 von `/fusion:cleanup`. Kein Circle ist aktiv, `agentstate.yaml` und
`.active-circle` fehlen. Die Runde 14 (`circles/260819-2230-auswahl-und-kopieren-in-der-vorschau`)
ist am 260820-1045 kohärent geschlossen und hat ihren eigenen Abgleich vom 260820-0834 bereits
gefahren. Dieser Durchgang ist der breite über alle Speicher, mit zwei vom Nutzer ausdrücklich
benannten Stellen darin.

## Zur Domäne

**`code` kommt aus der Aufgabenstellung und aus dem Vorgabewert, nicht aus einer Messung.**
`agentstate.yaml` ist beim sauberen Sitzungsende gelöscht worden, also hat kein aufgezeichnetes
Verdikt den Schluss überlebt. Dieselbe Feststellung steht im Abgleich vom 260819-1440; sie gilt
unverändert, und dieser Durchgang übernimmt sie, statt sie neu zu erheben.

## Was geprüft wurde

| Gegenstand | Menge | Ergebnis |
|---|---|---|
| Planungsdateien, Marker gegen Kopfzeile `**Status:**` gegen Baum | 28 | eine umbenannt, vier Kopfzeilen berichtigt, ein Spec zum ersten Mal beurteilt |
| Entscheidungsdatensätze mit `_a_`, jeder einzeln am Baum gesucht | 14 | acht auf `_i_` gehoben, sechs begründet stehen gelassen |
| Offene Defekte, die `CLAUDE.md`-Aussagen beschreiben | 5 | alle fünf am heutigen `CLAUDE.md` nachgelesen, alle fünf geschlossen |
| Zitate auf jede umzubenennende Datei, auf tote Zeiger geprüft | 6 Namen | kein Zitat außerhalb der Ortsregel betroffen |
| Vollständiger Prüflauf `make check` gegen `f5300f4` | 1 | Rückgabewert 0, alle vier Kommandos grün |
| Durchsichtsdateien | 60 | keine Anmerkung nötig; die jüngste liegt vor dem letzten Codecommit, und das ist selbst ein Befund |

## Was berichtigt wurde

**Fünfzehn Umbenennungen.** Sechs Defekte auf `_c_`, acht Entscheidungen auf `_i_`, ein Plan auf `_c_`.

### Der Plan der Runde 14 war die einzige Ausnahme im ganzen Baum

`circles/260819-2230-…/planning/260819-2245_*_plan-auswahl-und-kopieren-in-der-vorschau.md`
**`_p_` → `_c_`**, Kopfzeile `**Status:**` von „Teilweise abgeschlossen" auf „Complete".

Der Abgleich vom 260820-0834 hatte zwei Gründe für `_p_` genannt. **Der erste ist entfallen**: die
Bündelabnahme aus `## Nutzerarbeit` hat der Nutzer am 260820-1030 gefahren (`70d914d`,
Schließungsnotiz des Circle-Datensatzes). **Der zweite hat für einen Plan nie gegolten**: die offene
Frage `shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-…` schließt Pläne im
eigenen Text aus — „Für Pläne trägt sie und ist im ganzen Baum befolgt […]. Für Specs schweigt sie."

Am Dateibestand nachgezählt: **dreizehn gefahrene Runden, dreizehn Pläne, und zwölf davon standen
auf `_c_`** — darunter neun, deren Abnahmelauf nie gefahren worden ist. Der Plan-Marker folgt in
diesem Baum ausnahmslos den Schritten und nicht der Abnahme; die Runde 14 war der einzige Abweichler.

### Fünf Defekte über `CLAUDE.md`, plus der Sammeldatensatz, der sie führt

Der Kuratorenlauf `260820-1119` hat sie erhoben und ausdrücklich nicht geschlossen, weil das Bewegen
eines Markers auf eine Baumprüfung hin nicht in seinen Auftrag gehört. Dieser Abgleich hat jede der
fünf Behauptungen **neu** gegen `CLAUDE.md` gehalten und nicht vom Kurator übernommen — die Datei ist
seit seiner Messung ein zweites Mal geändert worden (`7da3098`). Alle fünf halten.

| Datensatz | Berichtigt durch |
|---|---|
| `shared/issues/260816-2138_*_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md` | L01 (`5886d04`), L03 und L04 (`7da3098`) |
| `shared/issues/260816-1232_*_claude-md-sagt-den-tag-setze-der-nutzer-….md` | L07 (`5886d04`) |
| `shared/issues/260818-0028_*_claude-md-says-the-bundle-ships-as-v0-4-1-….md` | L03 (`5886d04`) |
| `shared/issues/260818-1635_*_claude-md-nennt-zwei-nachzuziehende-stellen-je-kommando-….md` | L09 (`5886d04`) |
| `circles/260816-1321-…/issues/260816-1935_*_claude-md-nennt-zwei-filterregeln-….md` | L10, L11 (`5886d04`), L03 (`7da3098`) |

Der fünfte trägt **vier** Behauptungen und nicht eine, wie die Vorlage des Kurators annahm; alle vier
sind geprüft. Dazu ist
`shared/issues/260820-1119_*_fuenf-offene-defektdatensaetze-…` selbst auf `_c_` gegangen: sein
Abschnitt `## Fix` ist wörtlich das, was dieser Durchgang getan hat.

### Acht beantwortete Entscheidungen sind am Baum eingelöst

Alle acht mit Datei, Zeile und Commit belegt; die Belege stehen in den Datensätzen selbst.

| Datensatz | Beleg |
|---|---|
| Filter passt auf jede Stelle des Namens | `filter.rs:122`, `9e1892d` |
| Tiefe Suche steigt nicht in Verknüpfungen hinab | `durchlauf.rs:573`, `32fd038` |
| Ausgeblendete Markierung steht in der Statuszeile | `statuszeile.rs:330,447-453`, `2d3d971` |
| „Deep" bekommt keine Tastenkombination | `default-keymap.toml:459-462`, `d73be91` |
| Rückschritt nimmt ein Zeichen zurück | `kommandos/rueckschritt.rs:99-135`, `2ff4b5a` |
| Gehaltener Rückschritt hört an der Grenze auf | dieselbe Tafel, dritte Größe `merker`, `2ff4b5a` |
| Ein Faden, ein Kanal je Tab, Tiefe 1.024 | `durchlauf.rs:262-267`, `leser.rs:50`, `2cdd299` |
| Die 1 MB der Vorschau gelten für den Inhaltsfilter | `tabs.rs:929`, `09baffd` |

**Sieben der acht lagen im Speicher der Runde 10.** Der Abgleich vom 260819-1440 hat sie nicht
gesehen, und das ist kein Versäumnis, sondern eine Eigenschaft des Werkzeugs: ohne aktiven Circle
liefert `bin/fusion-paths` für `SCAN_DECISIONS` allein den gemeinsamen Speicher, also sind die
Speicher der geschlossenen Runden aus dem Suchbereich gefallen. **Wer einen breiten Abgleich fährt,
listet die Circle-Speicher von Hand dazu**; dieser Durchgang hat es getan.

### Vier Kopfzeilen `**Status:**` standen gegen den Baum

Kein Dateimarker ist dabei bewegt worden. Geändert ist allein die eine Zeile.

| Datei | Vorher | Jetzt |
|---|---|---|
| `shared/planning/260819-2216_p_spec-auswahl-und-kopieren-in-der-vorschau.md` | „15 der 39 Abnahmekriterien […] sind ungefahren" | gefahren am 260820-1030, mit dem Grund, warum der Marker `_p_` bleibt |
| `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` | „Entwurf" | gebaut und belegt, Abnahmelauf nicht gefahren |
| `circles/260813-2332-…/planning/260813-2348_o_spec-notizzettel-….md` | „Entwurf" | gebaut und belegt, Abnahmelauf am 260814 gefahren, 8 von 29 Bündelkriterien |
| `circles/260814-1551-…/planning/260814-1830_o_spec-tippen-filtert-….md` | „Entwurf" | gebaut und belegt, Abnahmelauf nicht gefahren |

Die erste Zeile war seit dem Abnahmelauf des Nutzers falsch. Die drei „Entwurf" waren es länger: eine
Runde, die vor Tagen geschlossen hat, ist in keinem Sinn ein Entwurf.

### Der Spec der Runde 11 ist zum ersten Mal beurteilt

Der Abgleich vom 260819-1440 hat festgestellt, dass zwei Specs nie einen `## Reconciliation Log`
bekommen haben und ihr `_o_` deshalb „nicht gesetzt, sondern stehen geblieben" ist. Der Circle der
Runde 11 führt daneben gar kein Abgleichsprotokoll. **Für den Spec der Runde 11 ist die Lücke jetzt
geschlossen**, mit einer Tafel über die Kriterien, die sich ohne laufendes Bündel entscheiden lassen:
C6.1, C6.3, C6.4, C6.5, C6.8, C1.7 und die Schwelle aus dem Directive-Satz. Alle halten, eines nur
der Sache und nicht dem Namen nach (siehe unten). Die übrigen der 57 sind Nutzerarbeit und stehen
ungefahren in `messungen/260816-abnahme-inhaltsfilter.md`.

**Der Spec der Runde 7** (`shared/planning/260813-0053_o_spec-suche-in-der-belegung-…`) ist der
zweite nie beurteilte und bleibt es. Er ist in der Aufgabenstellung nicht genannt, und eine
Beurteilung seiner Kriterien ist Arbeit von eigenem Umfang.

## Was nur gekennzeichnet und nicht bewegt wurde

### Die zwei Specs, die der Nutzer benannt hat — und warum nur einer davon ein Rückstand war

**`shared/planning/260819-2216_p_spec-auswahl-und-kopieren-in-der-vorschau.md` bleibt `_p_`.** Von den
drei Gründen, die der Abgleich vom 260820-0834 für diesen Marker genannt hat, ist einer entfallen
(die 15 ungefahrenen Kriterien) und einer steht: `_c_` an einem Spec entschiede die offene Frage
`shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md` durch
vollendete Tatsache. **Die zwei Lesarten fallen an dieser Datei tatsächlich auseinander**, und das
ist am Baum gemessen und nicht angenommen: nach der Lesart der belegten Bauarbeit stünde sie auf
`_c_` (acht von acht Planschritten belegt), nach der Lesart der Abnahmekriterien nicht — C2.12
verlangt vier Wege, zwei davon (Ziehen und Dienste) sind vom Befund
`circles/260819-2230-…/issues/260820-0733_o_` am Baum widerlegt, und C2.3 und C2.4 tragen die
Kennzeichnung **(Probe)** ohne Probe.

**`shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` bleibt `_o_`**, aus demselben
Grund und mit derselben Begründung. Der Rückstand an dieser Datei war nicht der Marker, sondern die
fehlende Beurteilung; die ist jetzt nachgeholt.

**`_p_` ist dabei selbst unwahr, und das ist die Rechnung der offenen Frage.** Die Konvention liest
`_p_` als „In progress — agent is actively working on it". An diesem Spec arbeitet kein Agent, und
die Runde ist seit dem 260820-1045 geschlossen. Von den vier Markern nimmt `_p_` als einziger die
Frage nicht vorweg und behauptet als einziger eine Tätigkeit, die es nicht gibt.

### Was der offenen Frage an Bestand zugewachsen ist

Der Datensatz `260819-1440_o_` hat drei Ergänzungen bekommen, alle gemessen und keine wählend:

1. **Die Runde 14 ist der erste Fall, in dem die zwei Lesarten auseinanderfallen, obwohl der
   Abnahmelauf gefahren ist.** Bei den sieben Specs seiner Tafel trennt sie die Frage „gefahren oder
   nicht". Hier trennt sie feiner, und genau deshalb ist die Lesart A teurer anzuwenden, als ihre
   Kostenrechnung annimmt.
2. **`_p_` ist als Ausweichmarker in Gebrauch gekommen**, an zwei Dateien, beide aus der Runde 14.
3. **Ein Preis, den die Kostenrechnung nicht führt: `_c_` macht einen Spec archivierbar.** Am
   260819-1613 hat der Archivschritt zwei `_c_`-Specs aus `shared/planning/` in den Archivspeicher
   verschoben. Nach der Lesart B nähmen die sieben offenen Specs beim nächsten Aufräumen denselben
   Weg, und die Kurzform-Zitate auf sie zeigten ins Leere, ohne dass eine Suche es meldete.

### Sechs Entscheidungen bleiben auf `_a_`, jede mit eigenem Grund

Vier davon sind nicht bewegbar, und bei dreien ist der Grund derselbe und neu benannt: **ihre Antwort
ist eine Abwesenheit.** „Die Mindestbreite bleibt", „die Frage löst sich mit der Bauform auf", „es
kommt keine elfte Zeitzusage" — für keine dieser Antworten gibt es einen Commit, den die Zeile
`Implemented:` zitieren könnte, und die Vokabel kennt keinen anderen Ausgang. Als eigener Befund
abgelegt. Die zwei übrigen sind die der Runde 14, vom Abschluss jener Runde begründet auf `_a_`
belassen; dieser Abgleich hat beide nachgeprüft und folgt der Einordnung.

## Vier neue Datensätze

Drei Defekte und eine Ergänzung an einer offenen Entscheidung.

- **`shared/issues/260820-2056_o_claude-md-nennt-eine-zaehlprobe-unter-einem-namen-den-der-baum-nicht-traegt.md`**
  — `CLAUDE.md:131` schickt den Leser für die Zahl der Rufer ausdrücklich an eine Probe statt an den
  eigenen Text und nennt sie `die_zeichenregel_und_der_vergleich_stehen_je_einmal_…`. Diesen Namen
  trägt keine Probe; sie heißt `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`
  (`crates/krk-core/tests/verzeichnis.rs:3095`). Null Treffer für den zitierten Namen. Kuratorarbeit.
- **`shared/issues/260820-2056_o_dreissig-entscheidungsdatensaetze-tragen-eine-leere-vorlagenzeile-vor-der-gefuellten.md`**
  — über alle 158 Datensätze erhoben: **30 Dateien, 46 Schlüsselfälle**, in denen die **erste**
  Fundstelle eines Schlüssels leer ist und eine spätere gefüllt. Ein `grep -m1 'Superseded by:'`
  meldet dort „nicht überholt". Der Kurator hat die Gestalt zweimal als Kandidat vorgelegt und dabei
  nur `Superseded by` gemessen, also zwei von 46. **Sie folgt aus der Regel und nicht aus
  Unachtsamkeit**: die Vorlage setzt den Block ans Ende, die Fortschreibungsregel hängt darunter an.
  Dieser Durchgang hat sie beim Heben der acht Datensätze achtmal selbst reproduziert.
- **`shared/issues/260820-2056_o_drei-beantwortete-datensaetze-koennen-nie-umgesetzt-werden-weil-ihre-antwort-eine-abwesenheit-ist.md`**
  — siehe oben. Der Sache nach eine Frage an fusion und kein Defekt dieses Projekts.
- **`shared/decisions/260815-1812_o_der-eine-codecommit-…-ohne-durchsicht-…`** hat eine Ergänzung
  bekommen: `f5300f4` ist ein ungedeckter Codecommit, und `bin/fusion-review-coverage` kann ihn nicht
  melden. Ohne `agentstate.yaml` fehlt der Anker, und das Werkzeug antwortet `verdict=unchecked` — in
  einem Bericht liest sich das wie „nichts zu beanstanden". Das ist ein **zweiter** Fehlerweg neben
  dem, den der Datensatz beschreibt: dort eine falsche Zuordnung innerhalb einer gemessenen Spanne,
  hier eine Arbeit ganz außerhalb jeder Spanne.

## Zwei Dinge, die dieser Durchgang bewusst nicht getan hat

**Die zwei Marker der zurückgestellten Runde 12 sind nicht bewegt worden.** Der Abgleich vom
260819-1440 hat den Fall geprüft und den Preis benannt: der Circle-Datensatz zitiert beide Dateien
mit ausgeschriebenem `_o_`, eine Umbenennung erzeugte dort zwei tote Zeiger, und den Circle-Datensatz
darf ein Abgleich nicht anfassen. Die Lage ist unverändert, die Abwägung auch.

**Kein `## Coherence` ist an eine Sitzungsdatei angehängt worden.** Das jüngste Sitzungsprotokoll,
`shared/history/260819-2026-orchestrator-session.md`, trägt bereits sein eigenes `## Coherence` vom
260820-0834 und steht auf `Complete`. Ein zweiter Abschnitt darin schriebe das Verdikt dieses
Durchgangs einer abgeschlossenen fremden Sitzung zu und erzeugte daneben genau die doppelte
Überschrift, die `shared/issues/260819-1440_*_ein-spec-traegt-zwei-reconciliation-log-ueberschriften-…`
als Defekt führt. Der Abgleich vom 260819-1440 hat so entschieden und es begründet; dieser folgt ihm.
Das Verdikt steht deshalb hier.

## Coherence

**Verdikt:** review-needed

**Kanten:**

- **Artifact↔Grounding: 15 Marker gegen den Baum nachgezogen, 4 Kopfzeilen berichtigt, 3 neue
  Defekte, 143 offene Defekte im Bestand.** Der Code selbst ist sauber: `make check` läuft am
  260820-2050 gegen `f5300f4` mit Rückgabewert 0 durch, alle vier Kommandos grün, keine Probe rot,
  keine Warnung unter `-D warnings`. **Alle drei neuen Befunde betreffen die Form der Werkbank und
  nicht den Code** — ein toter Zeiger in `CLAUDE.md`, 46 Schlüsselfälle mit leerer Vorlagenzeile, und
  drei Datensätze ohne erreichbaren Endzustand. Die 143 offenen Defekte sind kein Zuwachs dieses
  Durchgangs: sechs sind geschlossen, drei dazugekommen.
- **Artifact↔Directive: keine Directive ist in Kraft, und ein Commit dieser Spanne beantwortet
  keine.** `agentstate.yaml` fehlt, `.active-circle` fehlt, die Runde 14 ist geschlossen. Von den
  vier Commits seit ihrem Abschluss (`2beb1de..HEAD`) sind drei Buchführung: `7da3098` (Kuratorlauf
  an `CLAUDE.md`), `5d363de` (Version 0.5.5), `c586a43` (Sitzungsprotokoll). **Der vierte,
  `f5300f4`, ist Arbeit von der Größe einer halben Runde** — `xtask/src/beglaubigung.rs` mit 665
  neuen Zeilen, ein neues Skript, vier geänderte Dateien, zwölf neue Proben — und er gehört zu
  keinem Circle und trägt keine Durchsicht. Er läuft der Directive dieses Projekts nicht entgegen:
  er liegt im Gebiet der Runde 8 (`xtask release`, Versionstags) und löst einen Fehlschlag des
  Auslieferungslaufs vom selben Tag. **Geflaggt ist nicht seine Richtung, sondern dass es keine
  Directive gibt, an der er zu messen wäre.**
- **Grounding↔Directive: 39 aktive Datensätze über alle Speicher (33 offen, 6 beantwortet), null
  widersprüchlich, einer gekoppelt und teuer.** 109 stehen auf umgesetzt, acht davon seit diesem
  Durchgang. Kein aktiver Datensatz widerspricht der Directive dieses Projekts. **Einer bindet
  weiter und kostet inzwischen messbar:**
  `shared/decisions/260819-1440_o_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`
  hält acht Planungsdateien in einem Marker fest, den kein Abgleich bewegen darf, und hat seit
  gestern einen zweiten Nebeneffekt — `_p_` als Ausweichmarker an zwei Dateien, der eine Tätigkeit
  behauptet, die es nicht gibt. Drei weitere stehen dauerhaft auf `_a_`, weil ihre Antwort eine
  Abwesenheit ist.

**Rebalance-Empfehlung:** revise Directive

**Zur Empfehlung.** Die geflaggte Kante mit dem höchsten Hebel ist Artifact↔Directive, und die
mechanische Zuordnung liefert deshalb „revise Directive". Sie ist beratend, und die Sache dahinter
ist konkret und klein: **`f5300f4` braucht eine Directive, oder es braucht die Feststellung, dass
Arbeit aus einem unmittelbaren Auftrag des Nutzers keine braucht.** Drei Wege stehen offen — ein
eigener Circle für den Auslieferungsweg, ein Rückstandseintrag, oder die ausdrückliche Regel, dass
ein unmittelbar beauftragter Coder außerhalb des Circle-Mechanismus arbeiten darf. Die Wahl gehört
dem Nutzer; der Abgleich hat nichts davon vorweggenommen und `f5300f4` nicht angefasst.

**Zwei Dinge, die das Verdikt ausdrücklich nicht sagt.** Es sagt nicht, dass die Runde 14 unfertig
wäre — sie ist gebaut, abgenommen und geschlossen, und ihr Plan steht seit diesem Durchgang zu Recht
auf `_c_`. Und es sagt nicht, dass am Code etwas fehlt: die vier Prüfkommandos laufen grün, und
keiner der drei neuen Befunde berührt ausgelieferten Code.

## Für den nächsten Durchgang

- **Die offene Frage nach dem Spec-Marker ist reif.** Sie hält acht Dateien fest, hat einen
  Ausweichmarker erzeugt und ist um drei gemessene Befunde reicher, darunter die Archivfolge. Sie
  liegt beim Nutzer und bei keinem Agenten.
- **Der Spec der Runde 7 ist der letzte nie beurteilte** und braucht denselben Durchgang, den der
  Spec der Runde 11 heute bekommen hat.
- **Wer einen breiten Abgleich fährt, listet die Circle-Speicher von Hand dazu.** Ohne aktiven Circle
  liefert `bin/fusion-paths` allein den gemeinsamen Speicher; sieben Datensätze der Runde 10 sind
  deshalb vier Wochen lang aus jedem Suchbereich gefallen.
