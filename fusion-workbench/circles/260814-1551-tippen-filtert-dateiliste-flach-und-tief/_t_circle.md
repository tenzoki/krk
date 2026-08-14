# Tippen filtert die Dateiliste, flach und über den ganzen Unterbaum

---
**Domain:** code
**Status:** active
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md
**Active session history:** shared/history/260814-1500-orchestrator-session.md

---

## Directive

Wer im Dateifenster Buchstaben ohne Zusatztaste tippt, blendet damit jeden Eintrag aus, dessen Name nicht passt. Der Filter gehört dem Tab und wird beim Ordnerwechsel geleert; `Esc` nimmt zuerst den Filtertext zurück, bevor es seine übrigen Bedeutungen bekommt. Steht ein Filtertext, dehnt ein Schalter in der Bereichsleiste die Suche auf den ganzen Unterbaum unter dem angezeigten Ordner aus. Die Treffer stehen dann in einer flachen Liste, und jeder Treffer nennt den Unterordner, in dem er liegt. Sie erscheinen bereits während des Durchlaufs, die Statuszeile zählt die gezeigten gegen die vorhandenen Einträge mit, und ein Tastendruck hält den laufenden Durchlauf an. In der flachen Suche bleiben Ordner stehen, damit die Navigation bei stehendem Filter nicht abbricht; in der tiefen Suche fallen sie weg, weil in einer tiefen Trefferliste ohnehin nicht navigiert wird.

**Was diese Runde ersetzt.** Das sechste Abnahmekriterium von C2 der Runde 1 lautet heute: „Tippt der Nutzer Buchstaben ohne Zusatztaste, springt die Auswahl auf den ersten Eintrag, dessen Name so beginnt. Nach einer Pause beginnt die Eingabe von vorn." Dieses Kriterium wird **ersetzt und nicht ergänzt**. Der Nutzer hat die Ersetzung am 260814 ausdrücklich so gewählt. Die Folge ist benannt und angenommen: wer heute `d` tippt, um zum ersten Eintrag mit `d` zu springen, sieht künftig statt eines Sprungs eine auf die `d`-Einträge verkürzte Liste. Der bisherige Sprung ist danach auf keinem Weg mehr zu haben, weder als Vorbelegung noch als zweiter Modus. Ein neues Kürzel und ein neues Bedienelement für den Einstieg in den Filter entstehen dabei nicht: das Tippen selbst ist der Einstieg, so wie es heute der Einstieg in die Sprungmarke ist.

**Was diese Runde fallen lässt.** Der Nutzer hat die tiefe Suche zunächst als Baum beschrieben, in dem die Treffer unter ihren Ordnern hängen, und diese Form nach der Kostenfrage fallen gelassen. Es entsteht deshalb **kein** hierarchisches Modell, keine `NSOutlineView` und keine zweite Tabellenklasse; `crates/krk-ui/src/appkit/tabelle.rs` bleibt eine flache `NSTableView` mit ihren vier Spalten. Die frühere Formulierung „jeder Treffer ist mit seinem Weg dorthin sichtbar" ist durch „jeder Treffer nennt den Unterordner, in dem er liegt" abgelöst. Das steht hier, damit es später niemand als Versehen liest und den Baum nachträglich einbaut.

**Was diese Runde nicht anfasst.** Sie setzt **keine elfte Zeitzusage und fasst keine der zehn aus C8 der Runde 1 an**. Die laufende Anzeige baut auf dem vorhandenen Lesevorgang auf, also auf dem eigenen Faden mit seinen Stapeln zu 1.024 Einträgen und dem Abbruch innerhalb von zwei Stapeln; eine zweite Lesemechanik daneben entsteht nicht. Die tiefe Suche bekommt weder einen Deckel auf die Trefferzahl noch eine Tiefengrenze. Suchen und Ersetzen über mehrere Dateien bleibt außerhalb dieser Runde und ist seit dem 260802 ein eigenes Vorhaben.

**Vorbelegungen, denen der Nutzer nicht widersprochen hat.** Der Filter gehört dem Tab und nicht dem Fenster. Der Schalter „Tief" wirkt erst, sobald ein Filtertext steht. `Esc` löscht zuerst den Filtertext. Die Statuszeile nennt die gezeigten gegen die vorhandenen Einträge.

## Grounding snapshot

Der Baum ist am 260814 auf dem Stand `43dfe90` gelesen worden. Was hier steht, ist an den genannten Stellen geprüft.

**Der Filtermechanismus ist vorhanden und trägt heute einen einzigen Prüfschritt.** `Ordnermodell` baut seine Sichtreihenfolge in `anhaengen` und in `sicht_neu_aufbauen` auf, und der einzige Filter darin ist `verstecke_ausblenden` (`crates/krk-core/src/verzeichnis/modell.rs:195-207` und `:246-258`). Ein Namensfilter ist an dieser Stelle ein Prüfschritt mehr und keine zweite Sicht daneben.

**Der Eintrag trägt keinen Pfad.** `Eintrag` (`crates/krk-core/src/verzeichnis/eintrag.rs:29-60`) führt `name`, zwei Sortierschlüssel, Größe, Änderungszeit, den Versatz der Endung, den Typ und das Versteckt-Kennzeichen. Für die Antwort „jeder Treffer nennt seinen Unterordner" kommt ein Feld hinzu. Daran hängt eine Stelle, die es heute nicht braucht: `kommandos::operationen::betroffene` baut die Pfade der betroffenen Einträge als `ordner.join(&eintrag.name)` (`crates/krk-ui/src/kommandos/operationen.rs:167-192`). Für einen Treffer aus einem Unterordner ist das der falsche Pfad, und die Stelle muss den neuen Wert lesen.

**Der Lesevorgang trägt die laufende Anzeige schon.** Ein Arbeitsfaden schickt Stapel zu 1.024 Einträgen an den Hauptfaden, der Kanal hat die Kapazität eines Stapels, und daraus folgt der Abbruch innerhalb von zwei Stapeln (`crates/krk-core/src/verzeichnis/leser.rs:1-37`). Der erste Stapel trägt heute die Zusage L2.

**Der Lesevorgang leert sein Ordnermodell nicht vorab**, sondern ersetzt es mit dem ersten gelieferten Stapel (`Ordnermodell::lesevorgang_beginnen`, mit dem Auffangfall in `abschliessen`). Wer in dieser Spanne den Bestand befragt, sieht den vorigen Ordner. Für eine tiefe Suche, die während ihres Durchlaufs eine Trefferzahl anzeigt, ist das die Stelle, an der die Zahl falsch werden kann.

**Die zu ersetzende Sprungmarke** steht in `crates/krk-core/src/verzeichnis/sprungmarke.rs:119-127`: `erste_zeile_mit` sucht in der Sichtreihenfolge mit `starts_with` ohne Rücksicht auf Groß- und Kleinschreibung, und der Puffer davor beginnt nach einer Sekunde Pause von vorn. Der Weg dorthin führt über `Nachschlag::Sprungmarke` im Kern und den Zeichenzweig in `crates/krk-ui/src/appkit/ereignisse.rs:547`.

**Eine Funktion aus diesem Modul hat einen zweiten Nutzer und darf nicht mitfallen.** `krk_core::verzeichnis::sprungmarke::traegt_ein_dateiname` entscheidet auch in der Tippsuche der Belegungsansicht aus der Runde 7, welche Zeichen aufgenommen werden (`crates/krk-ui/src/belegungsmodell.rs:72` und `:669-670`). Die Zeichenregel bleibt eine einzige; ersetzt wird die Wirkung der Sprungmarke, nicht ihre Zeichenprüfung.

**Die Bereichsleiste trägt acht Ankreuzfelder und keinen Ersthelferrang.** Fünf für die Bereiche der Fensterzeile, drei für die schaltbaren Spalten (`crates/krk-ui/src/appkit/bereichsleiste.rs:1-49`). Jeder Schalter trägt `setRefusesFirstResponder(true)`, weil `Fokus` sonst eine falsche Auskunft gäbe. Der Schalter „Tief" wäre das neunte Feld und der erste Zuwachs seit der Runde 5.

**Die Statuszeile ist eine und hat fünf Ränge.** `Rang` (`crates/krk-ui/src/appkit/statuszeile.rs:197-240`) zählt Befehlsantwort, Vorgangsanzeige, Fenstermeldung, Tabmeldung und Markierungsstand, in dieser Rangfolge, als vollständige Fallunterscheidung ohne Auffangzweig. Der Modulkopf sagt ausdrücklich, dass die Zeile heute weder den Lesefortschritt noch die Zahl der Einträge trägt und beides „in einer späteren Runde in dieselbe Zeile und nicht in eine zweite daneben" käme. Diese Runde ist jene spätere Runde. Ein sechster Rang hält den Bau an und erzwingt die Antwort darauf, wo er einzuordnen ist.

**Die Markierung besteht auch dort, wo der Nutzer sie nicht sieht, wirkt aber nicht.** `Ordnermodell::markierungsstand` zählt über alle Einträge (`modell.rs:350-374`), `alle_markieren` und `markierung_umkehren` wirken auf die sichtbaren, `markierung_aufheben` auf alle. `betroffene` dagegen läuft allein über die sichtbaren Zeilen: „eine Markierung, die der Nutzer beim Drücken der Taste nicht vor sich hatte, gehört nicht in den Auftrag" (`operationen.rs:162-192`). Ein Filter macht diesen bisher seltenen Fall zum Regelfall.

**Das Stapelumbenennen braucht dafür keine eigene Regel.** Es bekommt seine Namen aus derselben `betroffene`-Auswahl und prüft Kollisionen gegen `alle_namen`, das über `eintraege()` läuft und damit auch die ausgeblendeten Einträge sieht (`crates/krk-ui/src/appkit/anwendung.rs:4480-4497`, `crates/krk-ui/src/appkit/tabelle.rs:1455-1466`). Die Kollisionsprüfung bleibt vom Filter unberührt.

**Der Ordnersprung erreicht den ausgewählten Treffer heute nicht.** `Kommando::OrdnerDerDatei` aus der Runde 6 fragt `angezeigtedatei::welche`, und das kennt allein die Datei der Vorschau und die des Editors (`crates/krk-ui/src/angezeigtedatei.rs:1-58`). Die ausgewählte Zeile der Dateiliste ist keine seiner vier Eingaben.

**Der Sortierschlüssel entsteht einmal beim Lesen** und trägt die Kollation als Bytefolge. Er trägt die Zusagen L3 und L10 und darf nicht in einen paarweisen Vergleich zurückfallen.

**Jede neu angesprochene AppKit-Klasse braucht ihre Untergrenze im Modulkopf**, weil `objc2` keine Verfügbarkeitsangaben mitführt und der Übersetzer die Untergrenze macOS 15 deshalb nicht hält.

**Die Belegung führt am 260814 83 Einträge**, gezählt mit `grep -c '^\[\[' resources/default-keymap.toml`; `Kommando` trägt 77 Varianten. Ein Befehl für „Tief" wäre die 78.

Sechs Fragen sind bei dieser Klärung offen geblieben und liegen als Entscheidungsdatensätze unter `decisions/` dieses Circles. Keine davon hält die Planung auf, jede bindet sie.

## Dependencies

- `260802-0842-krk-mac-dateimanager-editor-git` — die Runde 1. Ihr Spec `planning/260802-1036_c_spec-navigator-geruest.md` trägt in C2 das sechste Abnahmekriterium, das diese Runde ersetzt, und in den Festlegungen darunter die Regel, dass eine freie Taste ohne Zusatztaste im Dateifenster auf die Sprungmarke durchfällt. Dieselbe Runde setzt in C8 die zehn Zeitzusagen, die diese Runde nicht anfasst.
- `260811-1304-statusleiste-mit-bereichsschaltern` — die Runde 5. Sie hat die Bereichsleiste gebaut, in die der Schalter „Tief" als neuntes Ankreuzfeld käme, samt der Begründung, warum kein Schalter den Ersthelferrang annimmt.
- `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` — die Runde 6. Sie hat die eine Statuszeile über die volle Fensterbreite mit ihren fünf Rängen gebaut und den Ordnersprung `OrdnerDerDatei`.
- `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` — die Runde 7. Ihre Tippsuche in der Belegungsansicht benutzt `traegt_ein_dateiname` aus dem Sprungmarken-Modul, und ihr Hauptmenü führt jede Funktion; ein neuer Befehl für „Tief" gehört dort hinein.

## Turn log

- Turn 1 (Sitzung 260814-1500): Commits ba4af5f..50742a6, zwoelf Stueck. Alle vierzehn Planschritte auf [DONE], der Baum nach jedem Schritt gruen ueber `make check` (Bau, Proben, Clippy unter -D warnings, Formatpruefung). Ein Nachzug ausserhalb des Plans (E2b), weil E1 und E2 eine Zusicherung teilen, die eine Probe haelt und keiner allein halten kann. Coherence-Urteil: noch nicht gefahren. Vor der Ausfuehrung haben zwei Diagrammpruefungen des Spec je einen echten Fehler gefunden: einen fehlenden Ausgang im Durchlauf, der drei Abnahmekriterien nicht tragen konnte, und zwei Widersprueche zwischen Abnahmekriterien. Zehn Defekte und fuenf Fragen bleiben offen. Der Abnahmelauf am laufenden Buendel ist Nutzerarbeit und steht aus; die Liste dafuer ist `history/260815-0400-abnahmeliste-g2.md`. Sitzungsprotokoll: shared/history/260814-1500-orchestrator-session.md
