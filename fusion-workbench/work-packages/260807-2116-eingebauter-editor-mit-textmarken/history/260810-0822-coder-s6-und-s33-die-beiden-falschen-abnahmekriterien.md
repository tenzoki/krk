# S6 und S33: die beiden falschen Abnahmekriterien berichtigt

**Status:** Complete
**Domäne:** code
**Ausführender:** `coder`
**Aktiver Circle:** `260807-2116-eingebauter-editor-mit-textmarken` (`_t_`)
**Grundlage:** `history/260810-0810-reconciliation.md`, Abschnitt `## Der Planstand, am Code geprüft`; drei Defektdatensätze auf `_p_`
**Abnahme:** `make check` mit Rückgabewert 0. Vier Kommandos grün, `clippy` ohne Meldung, `fmt` sauber.

**Ein Hinweis zur Ausstattung dieses Laufs:** `fusion-rules coder` gibt weder `chat-voice-de.yaml` noch `default-voice-de.yaml` aus. Beide Profile sind unmittelbar aus `fusion-workbench/stilwerk/` gelesen und angewandt worden. Die Artefaktsprache ist Deutsch nach `CLAUDE.md`, Zeile `**Language:** de`, ohne abweichende Artefaktsprache. Derselbe Befund steht schon im Abgleich vom 260810-0810 für den `reconciler`, dort allerdings mit ausgegebenem Chat-Profil; für den `coder` fehlen beide.

---

## Was zu tun war

Der Abschluss-Abgleich hatte zwei Planschritte benannt, die `[DONE]` tragen, obwohl der Code ihr Abnahmekriterium nicht einlöst. In beiden Fällen war das Kriterium falsch und nicht der Code. Drei Defektdatensätze standen dazu auf `_p_`.

## Teil 1: S6 und die beiden wandernden Tastenstellen

### Der vorgeschlagene Schnitt trägt nicht

`issues/260809-1527` hatte vorgeschlagen, die Regel auf das einzuschränken, was "KRK selbst über den Tastencode zustellt". Der Vorschlag ist an drei Stellen geprüft und hält an keiner.

**Seine Voraussetzung war schon beim Schreiben falsch.** Er beruft sich darauf, KRK belege nach C3 der Runde 1 den virtuellen Tastencode. S2 hat am 260809-1746 den dritten Weg gebaut. `Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) liefert seither für jeden einbuchstabigen Namen aus einem ASCII-Kleinbuchstaben oder einer Ziffer eine Zeichenkennung, und `Belegung::nachschlag` (`crates/krk-core/src/tasten/belegung.rs:821-838`) vergleicht Maske und Kennung statt Maske und Code.

**Am heutigen Code gelesen ist der Vorschlag leer.** `y` und `z` sind einbuchstabig, ihre Kennung ist damit immer eine Zeichenkennung und nie ein Code. Die Menge "von KRK über den Tastencode zugestellt und auf `kVK_ANSI_Y` oder `kVK_ANSI_Z`" ist für jede denkbare Belegung leer. Ein Kriterium, das keine Belegung verletzen kann, misst nichts, und eine Probe darauf bestünde auch dann, wenn jede Editor-Funktion auf `cmd+y` läge.

**Die Grenze, entlang der er schneidet, trennt nichts mehr.** Der Vorschlag zieht sie an `gehalten_von`. Beide Zusteller schlagen Buchstaben seit S2 über das Zeichen nach, das Hauptmenü über `NSMenuItem.keyEquivalent` und der Ereignisabgriff über `charactersByApplyingModifiers:`. Für diese Frage ist der Zusteller ohne Unterschied.

### Was stattdessen gilt

Der Weg aus `issues/260809-1746`: die Einschränkung auf `y` und `z` entfällt ersatzlos, weil seit S2 keine Stelle mehr wandert. Nachgezogen sind zwei Planstellen.

- **`### Befund 4`**, letzter Absatz. Er sagt jetzt, dass kein Schritt an der Frage hängt und keiner der neuen Tastenbefehle `y` oder `z` meidet, und er nennt S2 als den Grund. Ein zweiter Absatz hält fest, was bis zum 260810-0822 dort stand und warum es fiel.
- **Das Abnahmekriterium von S6.** Es nennt jetzt allein, was die Probe `die_auslieferungsbelegung_fuehrt_einundsiebzig_funktionen` hält: 71 Funktionen und die dreizehn neuen Kennungen. Ein zweiter Aufzählungspunkt am Schritt hält die Streichung samt Grund fest.

`### Frage 11` ist unverändert. Die Tabelle legt `text_rueckgaengig` und `text_wiederholen` weiterhin auf `cmd+z` und `shift+cmd+z`, und der Editor behält sein Rückgängig.

### Die Probe ist gefallen

`keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen` ist aus `crates/krk-core/tests/belegung.rs` entfernt. Umgebaut auf den vorgeschlagenen Schnitt hätte sie eine leere Menge geprüft; stehengelassen verbot sie künftigen Runden zwei Buchstaben ohne Grund. An ihrer Stelle steht ein Kommentar, der sagt, was sie hielt und wer es seither hält:

- `auf_einer_deutschen_tastatur_findet_die_aufschrift_y_die_vorschau` misst beide Tastendrücke, wie der Abgriff sie meldet, und die Gegenprobe dazu.
- `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte` hält für jede ausgelieferte Kombination fest, dass Buchstaben und Ziffern über das Zeichen und alles übrige über den Code gehen.

Beide messen die Sache selbst statt einer Vorsichtsregel. `cargo test -p krk-core --test belegung` läuft mit 42 bestandenen Proben durch.

## Teil 2: S33 und die vorübergehenden Merkmale

Der gebaute Code ist richtig. Falsch war die Annahme, vorübergehende Merkmale des Layoutverwalters trügen alle vier zugesagten Wirkungen. `NSLayoutManager.h:351` sagt, für das Zeichnen erkannt werde allein, was die Auslegung nicht ändert. Schriftgröße, Schriftschnitt, feste Schrift und Absatzeinzug ändern sie und täten als vorübergehendes Merkmal gar nichts.

### Ein neuer Entscheid, der alte auf überholt

Der Entscheid `260808-0140_*_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md` stand auf umgesetzt und ist deshalb nicht zurückgedreht worden. Nach `rules/fusion-workbench-conventions.md`, `## State Markers — decisions`, ist er auf überholt gezogen und trägt eine Zeile `Superseded by:` mit Grund.

Der neue Datensatz ist `decisions/260810-0822_i_wie-die-formatansicht-ihre-auszeichnung-setzt-und-warum-an-zwei-orten.md`. Er beschreibt den gebauten Zustand, ist also nicht offen, sondern unmittelbar umgesetzt, und nennt `41309cc` in seiner Zeile `Implemented:`. Er trägt die gemessene Eigenschaft von AppKit als Grundlage und stellt drei Möglichkeiten gegenüber, von denen die gebaute die einzige ist, die alle vier zugesagten Wirkungen zeigt.

**Die Wahl des Nutzers vom 260808-0155 ist davon nicht berührt.** Die Auszeichnungszeichen bleiben stehen, die ausgezeichneten Stellen bekommen ihre Wirkung, und der Stand in der Ansicht ist Zeichen für Zeichen der Stand der Datei. Abgelöst ist die Begründung, nicht die Antwort.

### Zwei Planstellen auf dieselbe Aussage gezogen

- **`### Frage 7`** trägt jetzt die Fallunterscheidung "wirkt auf die Auslegung oder nicht", die Zeile aus dem SDK-Kopf als Beleg und die Zusage an der Stelle, die sie wirklich trägt: der Sicherungsweg liest `NSTextView::string` und damit die Zeichen der Fläche, kein Merkmal.
- **S33**, Änderungszeile und Abnahmekriterium. Das Kriterium verlangt jetzt beide Merkmalswege und die Grenze zwischen ihnen, nennt `editor.rs:1099` als die Stelle, an der der Sicherungsweg die Zeichen liest, und ist vom gebauten Code eingelöst.

## Zwei weitere Stellen im Plan, mitgezogen

Beide führten dieselbe falsche Begründung.

- **S36**, Suchen. Die Begründung lautete, Suchen und Ersetzen bezögen sich auf den Text und nicht auf die Darstellung, "weil die Einfärbung nach S33 im Layoutverwalter liegt und den Textspeicher nicht anfasst". Der Textspeicher trägt jetzt Merkmale. Der Grund, der trägt: S33 setzt Merkmale und keine Zeichen, und die Suche läuft über den gehaltenen Stand.
- **`## Wie dieser Plan die Maxime "supersimpel" einlöst`.** Der Punkt führte die Einfärbung im Layoutverwalter als Grund für den einen Textbestand. Er nennt jetzt denselben Grund wie S36.

## Zwei Stellen außerhalb der Schreibgrenze

`resources/default-keymap.toml` führt die falsche Aussage von S6 an zwei Kommentarblöcken weiter, `:484-491` und `:617-627`. Die Datei ist TOML und gehört dem `ontocoder`. Ein Datensatz dafür bestand bereits: `issues/260810-0011_o_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md`, angelegt am 260810-0011 bei der Abnahme von S41. Er sagt selbst, er sei mit den beiden hier geschlossenen zusammen zu erledigen, und er bleibt als einziger der drei offen. Ein zweiter Datensatz zur selben Sache ist angelegt und wieder entfernt worden, bevor er Bestand wurde.

## Verweise auf die Sternstelle gezogen

Die drei Umbenennungen hätten Verweise in lebenden Dokumenten veralten lassen. Sie tragen jetzt die Sternstelle: `crates/krk-ui/src/hervorhebung.rs`, `issues/260809-1642_c_...`, `issues/260810-0011_o_...` sowie die beiden geschlossenen Datensätze selbst. Verweise in Sitzungshistorien sind bewusst nicht angefasst.

Der Modulkopf von `hervorhebung.rs` sagte außerdem, Plan und Datensatz nennten beide eine Mechanik. Das gilt seit dieser Arbeit nicht mehr; die Stelle steht jetzt im Präteritum und nennt den Schließungsstand.

## Der Marker von Plan und Spec

**Der Plan bleibt auf offen, und der Grund hat gewechselt.** Der Abgleich vom 260810-0805 hat ihn mit den zwei gebrochenen Kriterien begründet; die sind weg. Die Kopfzeile sagt das jetzt und stellt die Umbenennung als neu zu entscheiden hin. Die Entscheidung gehört dem Orchestrator, weil sie die Zeile `**Active spec/plan:**` im Circle-Datensatz nachzieht.

**Am Spec ist nichts geändert.** Geprüft: der Spec macht keine Aussage über die Meidung von `y` und `z` und keine über vorübergehende Merkmale. Seine Aussage bei C10, Ansicht und Datei deckten sich bei Markdown zeilenweise, hängt an der Antwort des Nutzers und nicht an der abgelösten Begründung; sie gilt unverändert. Der Spec bleibt aus dem Grund auf offen, den der Abgleich nennt: 110 Abnahmekriterien sind unabgehakt, und der Abnahmelauf ist Nutzerarbeit.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` | Kopfzeile, `### Befund 4`, `### Frage 7`, S6, S33, S36, `## Wie dieser Plan die Maxime "supersimpel" einlöst`, `## Angelegte Datensätze`, neuer Abschnitt im `## Reconciliation Log` |
| `crates/krk-core/tests/belegung.rs` | die Probe entfernt, ein Kommentar an ihre Stelle |
| `crates/krk-ui/src/hervorhebung.rs` | Modulkopf: zwei Verweise nachgezogen |
| `fusion-workbench/.../decisions/260810-0822_i_wie-die-formatansicht-ihre-auszeichnung-setzt-und-warum-an-zwei-orten.md` | neu, unmittelbar umgesetzt |
| `fusion-workbench/.../decisions/260808-0140_s_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md` | `_i_` → `_s_`, Kopf und `Superseded by:` |
| `fusion-workbench/.../issues/260809-1527_c_...` | `_p_` → `_c_`, Abschlussnotiz |
| `fusion-workbench/.../issues/260809-1746_c_...` | `_p_` → `_c_`, Abschlussnotiz |
| `fusion-workbench/.../issues/260810-0053_c_...` | `_p_` → `_c_`, Abschlussnotiz |
| `fusion-workbench/.../issues/260809-1642_c_...`, `260810-0011_o_...` | Verweise auf die Sternstelle gezogen |

Nicht committet, wie beauftragt.
