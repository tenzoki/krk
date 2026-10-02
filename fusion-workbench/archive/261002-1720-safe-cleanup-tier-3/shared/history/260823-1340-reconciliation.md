# Abgleich zum Abschluss der Sitzung `260823-0442`

**Bereich:** `ab11eb8..616ad5e`, neun Commits, dazu der nicht eingecheckte Arbeitsbaum
**Domäne:** `code`
**Aktiver Circle:** keiner (`fusion-workbench/.active-circle` fehlt); alles im gemeinsamen Speicher
**Gefahren von:** reconciler, 260823-1340
**Status:** Abgeschlossen

## Was geprüft ist

| Gegenstand | Zahl | davon geändert |
|---|---|---|
| Defektdatensätze gelesen (gemeinsamer Speicher, 260820 bis 260823) | 35 | 4 ergänzt |
| Entscheidungsdatensätze gelesen | 42 aktive (`_o_` und `_a_`) von 161 | 2 ergänzt |
| Pläne und Specs gelesen (`shared/planning/`) | 6 | 0 |
| Durchsichten gelesen | 2 aus dieser Sitzung | 2 ergänzt |
| Neue Defektdatensätze abgelegt | 2 | — |

`make check` ist am 260823-1336 gefahren und gibt 0 zurück; alle vier Kommandos grün,
`krk-core`-Proben und `xtask`-Proben durch, `fmt` und `clippy` ohne Befund.

## Die elf geschlossenen Defektdatensätze halten

Der Auftrag nannte zehn; es sind elf, und jeder trägt genau eine `Resolved:`-Zeile. Jede
Behauptung ist einzeln gegen den Baum gelesen und keine über den Commit-Text geglaubt:

- `260820-1034` **F4 setzt den Fokus nur dann in den Editor** — `df8163d`, vom Nutzer am
  260823-0942 von Hand abgenommen. Der Vermerk sagt beides, die Korrektur und die Abnahme.
- `260820-1034` **cmd+e bleibt in der Vorschau wirkungslos** — `df8163d`, dieselbe Wurzel. Der
  Vermerk hält die Dateilisten-Hälfte ausdrücklich als vom Nutzer überholt fest und schließt
  allein mit dem behobenen Defekt.
- `260823-0730` **drei Prosastellen** — `52fba42`. An `bildschirmbreiten_uebernehmen` steht heute
  eine Regel statt einer Aufzählung, und sie nennt die zwei offenen Befunde beim Namen.
- `260823-0733` **Probe zur Editorfortsetzung** — `52fba42`. `die_editorfortsetzung_misst_als_erste_anweisung`
  und der Helfer `erste_anweisung` stehen im Baum; der Datensatz belegt die Auslösung durch
  einen versuchsweisen Umbau.
- `260823-1030` **umbenannte Kennung weist jede `keymap.toml` ab** — **als Lage angenommen, nicht
  behoben**, und der Datensatz sagt das in seiner ersten Zeile. Der Mechanismus besteht
  unverändert fort; `crates/krk-core/src/tasten/belegung.rs:1423` bricht beim ersten unbekannten
  Bezeichner mit `return Err` ab, nachgelesen und zutreffend, ebenso `:1446-1447` und `:1660`.
  Der Datensatz trägt darunter das nach den Konventionen vorgeschriebene `Revised by:`, weil die
  Schließungsbegründung („es gibt noch keine Nutzer") am 260823-1140 falsch belegt war und durch
  die geprüfte ersetzt wurde („auf keiner der beiden Maschinen liegt eine `keymap.toml`"). Der
  Marker bleibt richtigerweise `_c_`; die Konvention verlangt genau das.
- `260823-1031` **zweite Fokuserhebung** — `52fba42`. `fn editor_rundweg(&self, fokus: Fokus)`,
  der Zweig ruft `self.editor_rundweg(fokus)`, und `self.fokus()` hat wieder genau fünf Aufrufer.
- `260823-1032` **zwei Zahlen im Modulkopf** — `52fba42`. Beide gestrichen statt korrigiert. Die
  verbliebene Zahl „Zehn Module" stimmt: `crates/krk-ui/src/kommandos/` trägt zehn Module neben
  `mod.rs`, und die Aufzählung unmittelbar darunter ist ihr eigener Beleg.
- `260823-1033` **drei Stellen zum `false`** — `52fba42`. Die vier genannten Stellen sagen jetzt,
  der Rückgabewert entscheide über den Nachzug und nicht über das Weiterlaufen des Tastendrucks.
  Die Aussagen auf dem Zeichenweg sind unverändert und richtig; `tabelle.rs:1719-1722` führt sie
  weiter, und der Datensatz nimmt sie ausdrücklich aus.
- `260823-1034` **`vorschau_danach` ungeprüft** — `52fba42`. `mod rundwegproben` steht ab
  `anwendung.rs:8412` mit den vier benannten Proben.
- `260823-1035` **Rückweg blendet immer ein** — `52fba42`, **ohne Verhaltensänderung**, und das
  ist der Sonderfall des Auftrags. Geändert ist allein die überzogene Begründung im Code; die
  Sachfrage hat der Nutzer am 260823-1235 als Möglichkeit 1 entschieden. Der Datensatz führt in
  seinem Rumpf noch den Absatz „Dieser Datensatz bleibt in Arbeit" und darunter den `Resolved:`.
  Das ist kein Widerspruch, sondern die Chronologie eines Datensatzes, der als Frage begann; die
  Konvention verlangt, den älteren Text stehen zu lassen.
- `260823-1036` **Zuschreibung zweier Proben** — `52fba42`. Beide Doc-Kommentare tragen den
  richtigstellenden Absatz, `resources/default-keymap.toml:825` führt die Umbenennung.

## Die zwei Entscheidungsdatensätze auf `_i_` halten

- `260820-1034` **wie kommt eine Taste zum Umschalten** — `Answered:` und `Implemented: 28cbb7b`.
  Am Baum nachgelesen und zutreffend: `crates/krk-ui/src/kommandos/rundweg.rs` steht als reine
  Funktion mit genau einem Rufer, `Wirkungsbereich::Vorschau` ist gefallen und
  `Wirkungsbereich::Dateibereiche` an seine Stelle getreten, die Aufzählung trägt weiterhin
  sieben Werte, und die Kennung heißt `editor_rundweg`. Der Datensatz trägt seit dem 260823-1320
  die Abnahme des Nutzers; sie ist zum Zeitpunkt dieses Abgleichs noch nicht eingecheckt.
- `260823-1137` **holt der Rückweg die Vorschau zurück** — `Answered:` (Nutzer, 260823-1235,
  Möglichkeit 1) und `Implemented: 52fba42`. Der Marker `_i_` ist gerechtfertigt, obwohl kein
  Verhalten geändert wurde: umgesetzt ist die Prosa, und der Datensatz sagt das ausdrücklich.
  Der Sonderfall ist sauber geführt und nicht kaschiert.

## Die offen gebliebenen Datensätze sind zu Recht offen

- `260823-0731` **Klick nimmt eine Ziehbewegung zurück.** Der Sachverhalt steht unverändert:
  `aktives_setzen` ruft `aufteilung_nachziehen` ohne vorherige Messung, und
  `bildschirmbreiten_uebernehmen` hat weiterhin genau zwei Rufer, von denen keiner
  `aktives_setzen` ist. Die Vorfrage, die der Datensatz selbst stellt, ist unbeantwortet.
- `260823-0732` **Nachzug vor dem Fokusumzug.** Beide Gründe bestehen fort: die AppKit-Frage ist
  ohne laufendes Bündel nicht entscheidbar, und die Zeithälfte (L1 mit Umschaltbefehlen in der
  Reihe) verlangt den Abnahmelauf. Der Nutzer hat `f4` und den `cmd+e`-Rundweg von Hand
  abgenommen, die L1-Messung **nicht**; die Zusage bleibt damit auf der Liste der späteren
  Messrunde, neben L7 aus `shared/decisions/260819-2216_*`.
- `260823-1210` **ein `make check` von neun mit Rückgabewert 2.** Ein zehnter Lauf ist im Rahmen
  dieses Abgleichs gefahren und grün, mit erhaltener Ausgabe. Das entkräftet den Datensatz nicht
  und ist als Beleg ergänzt; ohne die Ausgabe des roten Laufs bleibt er nicht diagnostizierbar.
- **Die älteren offenen Entscheidungen.** 35 tragen heute `_o_`, sieben `_a_`, über den
  gemeinsamen Speicher und alle Circles hinweg. Diese Sitzung hat keine davon nebenbei erledigt.
  Einer ist jedoch stichhaltig überholt und steht seit zehn Tagen offen, siehe unten.

## Was der Abgleich gefunden hat

### 1. Eine dritte Serie falsch gewordener Stellen steht da

Der Auftrag hat danach gefragt, und die Antwort ist ja. Sie liegt nicht im Code, sondern in der
Workbench: **jede** Zeilenangabe nach `crates/krk-ui/src/appkit/anwendung.rs`, die vor `52fba42`
geschrieben wurde, zeigt ins Leere. `52fba42` hat die Datei um rund 220 Zeilen verlängert, und
der Versatz liegt zwischen 59 und 168 Zeilen, also ungleichmäßig. Fünfzehn Angaben sind
stichprobenweise geprüft; keine einzige traf. Betroffen sind die zwei offen gebliebenen
Datensätze — die einzigen, die noch gelesen werden — und beide Durchsichtsberichte.

Abgelegt als
`shared/issues/260823-1336_o_die-zeilenzitate-der-zwei-offen-gebliebenen-befunde-und-beider-durchsichten-zeigen-nach-52fba42-ins-leere.md`.
Die zwei offenen Datensätze haben zusätzlich eine Umrechnungstafel bekommen, damit sie heute
benutzbar bleiben; die Tafel ist ausdrücklich als für heute gültig gekennzeichnet.

**Die Gestalt ist dieselbe wie bei den ersten beiden Serien**, und der Baum hat für sie zweimal
dieselbe Antwort gewählt: eine Regel statt einer Zahl. Für Zeilenangaben in der Workbench ist
diese Antwort noch nicht getroffen.

### 2. `CLAUDE.md` nennt einen Empfänger der Ersthelfermeldung, der Baum trägt zwei

**Keine Aussage von `CLAUDE.md` ist durch diese neun Commits falsch geworden.** Einzeln geprüft:
`Wirkungsbereich` trägt weiterhin sieben Werte (der Tausch `Vorschau` → `Dateibereiche` ändert
die Zahl nicht, und `CLAUDE.md` nennt die Werte nicht), `Bereich` fünf, `Fokus` fünf, für
`Kommando` steht dort richtigerweise keine Zahl (der Baum trägt heute 79).
`ist_eigene_textflaeche` hält weiterhin genau zwei Flächen, Editor und Vorschau. Beide
`kommando_ausfuehren` enden weiterhin auf einen Auffangzweig. Die Probe
`waehrend_eines_blattes_kommen_genau_diese_vier_durch` sagt weiterhin vier. Die eine Hülle um
`NSPasteboard` ist von keinem dieser Commits berührt.

Eine ältere Lücke ist dabei aufgefallen und als eigener Datensatz abgelegt
(`shared/issues/260823-1336_o_claude-md-nennt-einen-empfaenger-der-ersthelfermeldung-*`): der
Absatz über den Ereignisabgriff sagt „Empfänger ist `fokusanzeige_nachziehen`", während am Melder
seit dem 260819 zwei hängen und der ungenannte über `aktives_setzen` bis `anwenden` durchgeht.
Entstanden mit `76ceb683` in der Runde 14, nicht in dieser Sitzung. Sie ist die stillschweigende
Voraussetzung des offenen Befunds `260823-0732`. `CLAUDE.md` ist nicht angefasst worden; der
Auftrag verbietet es, und die Datei gehört dem `curator`.

### 3. Ein Entscheidungsdatensatz steht seit zehn Tagen offen, obwohl der Baum ihn trägt

`shared/decisions/260813-0053_o_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`
fragt, ob der Ereignisabgriff den zulässigen oder den ausgeführten Befehl schluckt. Möglichkeit 1
steht seit `9da33bc` vom 260813 im Baum: der Diff stellt `let ausgefuehrt = match kommando` auf
`let gewirkt` um und die Rückgabe auf `true`, und am heutigen Stand schließt
`fn kommando_ausfuehren` unverändert auf `if gewirkt { … } true`. Der Planschritt S3 der Runde 7
steht auf `[DONE]`.

**Der Marker ist trotzdem nicht gezogen worden**, und das ist eine bewusste Enthaltung: der
Datensatz fragt den Nutzer, und die Runde ist ausdrücklich auf der Empfehlung gefahren, statt
sie beantwortet zu bekommen. Gebaut zu sein und beantwortet zu sein ist hier zweierlei, und ein
Abgleich, der aus dem einen das andere macht, fälscht eine Zustimmung. Der Datensatz hat statt
des Markers die Belege bekommen; was fehlt, ist eine Zeile vom Nutzer. Die Nebenfrage
`circles/260813-0100-*/decisions/260813-0320_*_esc-im-editor-*` liegt in derselben Lage und
sagt es in ihrem eigenen Nachtrag ausdrücklich.

### 4. Sieben tote Verweise an einem Tag

Die Sitzung hat in ihren eigenen Datensätzen sieben Verweise mit ausgeschriebenem Marker
hinterlassen, deren Ziel dieselbe Sitzung anschließend umbenannt hat. Fünf davon stehen nicht in
einer `**Cross-references:**`-Zeile, sondern in Fließtext, in einem `Resolved:`-Vermerk und in
einer `Answered:`-Zeile. Das ist Sachstand für die offene Frage
`shared/decisions/260818-0201_*_does-a-cross-references-line-*` und dort als Messung ergänzt;
eine Antwort, die allein die Kopfzeile umstellt, ließe fünf der sieben stehen.

### 5. Ein einunddreißigster Fall der leeren Vorlagenzeile

`shared/decisions/260823-1137_*` trägt die leere Vorlagenzeile vor der gefüllten. Der Bestand
liegt heute bei 28 statt der dreißig, die der Datensatz nennt, und die Differenz ist kein
Fortschritt: zwei sind am 260820 ins Archiv gewandert, korrigiert ist keiner. Als `Also seen:`
an `shared/issues/260820-2056_*_dreissig-entscheidungsdatensaetze-*` ergänzt.

## Nichts gefunden zu

- **Falsch gesetzte Marker.** Keine Umbenennung dieser Sitzung ist unbelegt; `471d801` hat den
  einen Fehler, der passiert war, noch in der Sitzung geheilt.
- **Fehlende `Resolved:`-Zeilen.** Alle elf tragen genau eine.
- **Pläne mit falschem Stand.** Die sechs Dateien unter `shared/planning/` tragen alle ihre
  Begründung im Kopf; keine ist durch diese Sitzung berührt.
- **Als Defekt abgelegte Entscheidungen.** Kein Fall in dieser Sitzung; `260823-1035` ist der
  Grenzfall, und er ist richtig behandelt worden, nämlich durch Abspaltung der Sachfrage in einen
  eigenen Entscheidungsdatensatz.

## Was nicht Sache dieses Abgleichs war

Der Arbeitsbaum trägt beim Abschluss dieses Abgleichs nicht eingecheckte Änderungen: die
Abnahmenotiz an `260820-1034_i_*`, das Sitzungsprotokoll des Orchestrators, `.asset-provenance`,
die vier ersetzten Stilprofile und das Ereignisprotokoll. Das Einchecken gehört dem
Orchestrator; hier steht es nur als Aufnahme des Zustands, gegen den geprüft wurde.
