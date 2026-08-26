# Schlussabgleich der Sitzung 260825-1659 — 260826-1024

**Status:** Complete
**Bereich:** `e5ec81a..c95f28b`, sieben Commits
**Baumstand beim Abgleich:** `c95f28b`
**Domäne:** code
**Kein Circle aktiv.** Die Sitzung ist die Runde 18; ihre Arbeit liegt nach der Herkunftsregel
unter `shared/`.

Der Abgleich der Runde 18 selbst liegt in `shared/history/260826-0157-reconciliation.md` und ist
**nicht gedoppelt**. Was hier steht, setzt darauf auf und deckt allein den wiederaufgenommenen
Bereich.

## Eine Berichtigung an der Auftragsangabe

Der Auftrag nennt sechs Commits; `git rev-list --count e5ec81a..c95f28b` liefert **sieben**. Die
Tabelle im Auftrag führt sie alle sieben auf, es fehlt allein die Zahl davor. Gemeint und
geprüft ist der ganze Bereich.

## Was geprüft wurde und wie

Gelesen wurde gegen den Baum und nicht gegen die Berichte, die Erledigung behaupten. Die
Sitzungsdatei `shared/history/260825-1659-orchestrator-session.md` ist unvollständig und war
deshalb nicht die Grundlage; an ihre Stelle traten `fusion-workbench/agentstate.yaml`, der
Ereignisstrom `orchestrator-events.jsonl` und die Commits.

Selbst gefahren am 260826-1017: `make check` über `c95f28b`, Ausstiegscode 0, „alle vier gruen".

| Größe | Zahl |
|---|---|
| Pläne durchgesehen / geändert | 1 / 1 |
| Defektdatensätze umbenannt `_o_` → `_c_` | 5 |
| Defektdatensätze mit `Revised by:` versehen, ohne Umbenennung | 1 |
| Entscheidungsdatensätze nachgeprüft / umbenannt | 44 aktive, davon 7 auf `_i_` nachgemessen / 0 |
| Durchsichten geprüft / vermerkt | 2 / 2 |
| Neu abgelegte Defektdatensätze | 2 |

## Die vier Aufträge über den gewöhnlichen Abgleich hinaus

### 1. Der Defektdatensatz zur Runde 18 ohne Circle: gekürzt und geschlossen

`shared/issues/260826-0149_*_die-runde-18-hat-keinen-circle-datensatz-…` ist auf Anweisung des
Nutzers auf seine Möglichkeit 3 gekürzt: die Möglichkeiten 1, 2 und 4 fallen, Möglichkeit 3
steht jetzt als Abschnitt „Was zu tun war", der Absatz über das Leseprofil ist auf das
eingegrenzt, was der Nutzer feststellt, und der Schlusssatz der Empfehlung über die „Einheit der
Arbeit oder der Ablage" ist gefallen.

**Der Befund trägt und ist behoben.** `fb50fcd` sagt es an vier Stellen aus, jede am 260826-1017
einzeln aufgeschlagen: `CLAUDE.md:11` (der Satz über die Untergrenze, samt Verweis auf den
Datensatz), `:32` (die Tabellenzeile 18 mit einem Strich in der Circle-Spalte), `:71` („Beide
Listen lassen jede Runde ohne Circle-Datensatz aus") und `:85` (der Absatz über den Abnahmelauf
nennt die Runde 18 neben dem Glob). Der Datensatz steht auf `_c_`.

**Der Leseprofil-Mechanismus stand nie zur Debatte, und der Datensatz sagt das jetzt.** Das
Profil „fusion-Werkbank: alle Runden" erkennt `fusion-workbench/circles$` und zählt seine sechs
Zustandszeilen über `ordner = "*"`; ein Circle ist ein Verzeichnis darunter. Der Leser führt
beide Zahlen an zwei Orten: die offenen Defekte des gemeinsamen Speichers am Profil
„Projektwurzel mit fusion-Werkbank", die über alle Runden am Rundenprofil. Beide Stellen sind in
`resources/default-readers.toml` nachgeschlagen; die zwei **Zahlen** sind auftragsgemäß weder
nachgeprüft noch irgendwo festgeschrieben.

**Das Kürzen ist eine Ausnahme vom Zuschnitt dieses Agenten** und steht hier deshalb ausdrücklich:
der Abgleich ändert sonst keine Beschreibung eines Defekts. Die Anweisung des Nutzers wiegt
schwerer.

### 2. Die drei Marker, die niemand nachgezogen hat

Jeder einzeln am Baum geprüft, keiner nach dem Bericht.

| Datensatz | Marker | Beleg im Baum |
|---|---|---|
| `260823-1336_*_claude-md-nennt-einen-empfaenger-…` | `_o_` → `_c_` | `CLAUDE.md:141` nennt beide Empfänger in ihrer Laufreihenfolge und beschränkt die Zusage „ruft weder `anwenden` noch `setHidden`" auf `fokusanzeige_nachziehen`; der Rumpf des Melders steht in `anwendung.rs:1225-1230` wie zitiert |
| `260823-1649_*_claude-md-sagt-die-version-sei-…-gestiegen` | `_o_` → `_c_` | `CLAUDE.md:46` trägt den Halbsatz über die tägliche Steigerung nicht mehr |
| `260820-2056_*_claude-md-nennt-eine-zaehlprobe-…` | `_o_` → `_c_` | `CLAUDE.md:143` nennt `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`; der Baum trägt sie unter diesem Namen (`crates/krk-core/tests/verzeichnis.rs:3244`), der alte Name kommt unter `crates/` nicht mehr vor |

Der dritte trägt **nicht** `fb50fcd`, sondern `90f8ac1` vom 260820: die Aussage war schon sechs
Tage vorher gerichtet, und allein der Marker stand nach. Der Datensatz, der das gemeldet hat
(`260826-0923_*_drei-behobene-claude-md-datensaetze-…`), ist damit geschlossen.

Die zwei, die er zu Recht offen nennt, bleiben offen. `260825-1859_*` hängt an
`Cargo.toml:157`, das unverändert „mit dem einen Merkmal `deflate-flate2`" schreibt, während die
Merkmalsliste zwei Zeilen weiter zwei führt; das ist Arbeit des `coder`. `260826-0149_*` ist
geschlossen, aber über den Weg aus Auftrag 1 und nicht über diesen.

### 3. Die Falschzuschreibung in ihrer Quelle

`260823-1336` schreibt zweimal „Runde 14", und `76ceb68` landete am 2026-08-19 um 11:20, also
zwischen dem Schluss der Runde 13 (08:12, `c09ff3a`) und dem Beginn der Runde 14 (22:31,
`258bd7c`), ohne ein einziges Artefakt unter `circles/`.

**Nach der Konvention ist der `Revised by:`-Vermerk die richtige Form, und die Ersetzung im Rumpf
wäre die falsche.** `rules/fusion-workbench-conventions.md` `## Inline State Tracking` sagt für
eine Begründung, die ein späterer Commit umgezogen hat: den Vermerk anhängen, den bestehenden
Text unberührt lassen, den Marker nicht bewegen. Der Grund steht dort mit: der Rumpf zeichnet
auf, was damals gemessen wurde, und eine Ersetzung löschte die Umkehrung, statt auf sie zu
zeigen. Dazu kommt der Zuschnitt dieses Agenten, der Beschreibungen fremder Datensätze nicht
ändert.

Der Vermerk nennt beide Stellen des Rumpfs ausdrücklich und den Commit, der `CLAUDE.md` gerichtet
hat. Damit hat ein späterer Lauf die Berichtigung vor Augen, sobald er den Datensatz öffnet. Der
Marker `_c_` steht dort wegen der Behebung und nicht wegen der Berichtigung; das schreibt der
Vermerk selbst aus. Der meldende Datensatz `260826-0923_*_claude-md-schreibt-den-zweiten-…` ist
geschlossen.

**Ein Restrisiko bleibt und ist keines, das eine andere Form beseitigte.** Wer den Rumpf liest
und den Schluss nicht, kann die Zuschreibung ein zweites Mal übernehmen. Die Konvention nimmt
das in Kauf, weil die Alternative — den Rumpf umschreiben — die Aufzeichnung zerstört.

### 4. Die zwei Zahlen des Nutzers

Auftragsgemäß **nicht nachgeprüft und nirgends festgeschrieben.** Was geprüft ist: dass die zwei
Orte im Leser existieren und je eine der zwei Auskünfte tragen.

## Der gewöhnliche Abgleich

### Die neun Befunde der zwei Durchsichten

Zwei sind mit diesem Abgleich erledigt, sieben stehen weiter offen. Keiner trägt der Sache nach
das falsche Vokabular: die sieben Defekte sind je „geh und richte es", die zwei
Entscheidungsdatensätze je eine Wahl zwischen benannten Möglichkeiten mit Constraints und
Empfehlung.

| Befund | Stand |
|---|---|
| `260826-0902_o_*` keine Probe hält die vier flight-Zahlen | offen. `grep -rn 'flight' crates/ xtask/ README.md` ist am 260826-1024 leer |
| `260826-0903_o_*` die Zeichengleichheit der zwei Werkbankpaare | offen, unberührt |
| `260826-0904_o_*` der Doppelungshinweis „vor der Ortsangabe" | offen. Die vier Halbsätze stehen in `resources/default-readers.toml:298`, `:637`, `:724`, `:812` |
| `260826-0923_o_*` die falsche Runden-Zuschreibung | **geschlossen** (`c95f28b`, dazu der `Revised by:`-Vermerk) |
| `260826-0923_o_*` drei behobene Datensätze stehen offen | **geschlossen** (die drei Marker sind nachgezogen) |
| `260826-0923_o_*` die Ablage-Aufzählung neben ihrem Zeiger | offen |
| `260826-0923_o_*` die Pfadregel ist für Zeile 18 nicht total | offen |
| `260826-0859_o_*` die Schwelle des Inhaltsfilters (Entscheidung) | offen und unbeantwortet |
| `260826-0923_o_*` eine eigene Zeichenschwelle für den Durchlauf (Entscheidung) | offen und unbeantwortet |

### `CLAUDE.md` nach `fb50fcd` und `c95f28b`

**Keine der geprüften Aussagen ist überholt.** Die neun Belege des Kuratorenlaufs sind vom
`coderev` einzeln nachgefahren; dieser Abgleich hat die drei nachgemessen, an denen ein Marker
hing (die zwei Empfänger, der Tagbestand, der Probenname), und alle drei tragen.

**Die Vorgabe der tiefen Suche aus `20c9833` steht nicht in `CLAUDE.md`, und der `coder` hat
recht: das ist eine Lücke und keine Falschaussage.** Selbst gefahren:
`grep -no 'Deep[^,.]*\|tiefe[nr]* Suche\|Vorbelegung\|inhaltsschwelle' CLAUDE.md` liefert die
Zeilen 24, 25 und 143. Die zwei Tabellenzeilen benennen die Ankreuzfelder, der Filterabsatz
nennt `inhaltsschwelle` ohne Zahl und die Regel des Ordnerwechsels; keine der drei behauptet
einen Anfangszustand, und keine wird durch `20c9833` falsch.

Was fehlt, wiegt trotzdem: `Ordnermodell::neu` setzt `tief: true`
(`crates/krk-core/src/verzeichnis/modell.rs:374`), und `Tabliste::durchlauf_nachziehen_an` stößt
den Durchlauf über den Unterbaum an, sobald ein Filtertext steht (`crates/krk-ui/src/tabs.rs:897`)
— also beim **ersten** Anschlag. Bis zum 260826 verlangte derselbe Weg einen Klick des Nutzers.
`CLAUDE.md` beschreibt die Kosten des Durchlaufs in einem eigenen Absatz und sagt nicht, dass ihn
jetzt eine Taste auslöst. Abgelegt als
`shared/issues/260826-1024_o_claude-md-sagt-nicht-dass-die-tiefe-suche-ab-werk-steht-….md`.

### Die sieben Entscheidungen auf `_i_`

**Keine trägt ihre `Implemented:`-Zeile zu Unrecht.** Achtzehn Fundstellen sind am 260826-1017
einzeln aufgeschlagen und treffen jede ihre Zusage: `bausteine.rs:361`, `:377`, `:541`, `:86`,
`:788`; `tabelle.rs:488`, `:2189`, `:1546`; `anwendung.rs:8863`, `:8957`; `leseprofil/mod.rs:456`,
`:507`, `:360`; `datei.rs:270`, `:290`; `sys.rs:1088`, `:1133`; `README.md:44`. Keine
Zeilennummer ist durch die sieben Commits verschoben worden.

Die eine Umsetzung ohne Code — der Weg zu einer neuen Profildatei im `README.md` — trägt ihre
Einschränkung selbst: die zweite Hälfte der Antwort, ein Befehl in KRK, ist ausdrücklich
Gegenstand einer späteren Runde.

### Der Plan

Kein Commit des Bereichs berührt einen der zehn Planschritte. Der Plan hat einen zweiten Eintrag
im Reconciliation Log bekommen und in seiner Statuszeile den neuen Baumstand. **Der Dateimarker
bleibt auf `_p_`**: die zwei Nutzerbedingungen aus „Where this Circle stops" sind weiter
ungefahren.

## Neu abgelegte Datensätze

- `shared/issues/260826-1024_o_claude-md-sagt-nicht-dass-die-tiefe-suche-ab-werk-steht-und-ein-anschlag-den-unterbaum-ablaeuft.md`
- `shared/issues/260826-1024_o_acht-offene-defektdatensaetze-tragen-eine-leere-resolved-zeile-und-antworten-jeder-suche-als-geschlossen.md`

Der zweite ist ein Nebenbefund dieses Abgleichs und war in keinem Auftrag genannt. Acht
Datensätze mit dem Marker `_o_` tragen im Rumpf eine leere `Resolved:`-Zeile zwischen zwei
`---`-Trennern; ein `grep -l '^Resolved:'` gibt sie als erledigt zurück. Alle acht stammen vom
260823 und 260824. Der Datensatz `260818-0710_*` beschreibt die andere Richtung desselben
Fehlers, und diese hier ist die teurere: ein übersehener Abschluss kostet eine Nachprüfung, ein
vorgetäuschter kostet sie nicht, weil niemand sie ansetzt.

Ein neunter ist mit diesem Abgleich weggefallen: `260823-1649` hat seine leere Zeile ausgefüllt
bekommen, weil dort ohnehin eine Notiz hingehörte.

## Nichts fehlabgelegt

Kein offener Defektdatensatz dieses Bereichs ist der Sache nach eine Entscheidung, und kein
Entscheidungsdatensatz ein Defekt.
