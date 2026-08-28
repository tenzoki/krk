C6 der Runde 1 sagt eine Dreiteilung der Vorschau zu, und eine Datei hat seit der Runde 20 vier Wege

---

Das Kriterium C6 im Spec der Runde 1 teilt die Anzeige einer Datei in drei Wege: Text bis 1 MB, Bild bis 64 MB, alles Übrige als Metadaten. Seit der Runde 20 tritt ein vierter daneben: eine Datei mit der Endung `pdf` bis 64 MB reist als `Inhalt::Pdf` in die Ansicht und wird dort als Betrachter gezeigt. Die drei Wege der Runde 1 gelten unverändert; das Wort „Dreiteilung" trifft für die Anzeige als Ganzes nicht mehr zu. Der fremde Spec wird nicht angefasst; der Datensatz bleibt offen, weil die Schließung dem Nutzer gehört.

---

**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `4778c8a`, dazu der uncommittete Stand von `crates/krk-ui/src/vorschaumodell.rs` aus Schritt 5
**Affected:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md:307-317` (Kriterium C6, fünftes und sechstes Abnahmekriterium `:316-317`); `crates/krk-ui/src/vorschaumodell.rs:27-44` (Abschnitt `# Die Dreiteilung der Anzeige (C6)`)
**Cross-references:** `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/planning/260828-0649_*_spec-vorschau-rendert-pdf-als-betrachter.md:45-70` (Ursache, Abschnitt `## Wie der Betrachter in die Vorschau tritt`, und `:245`, zehnte Anweisung unter `## Open for Planner`); `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/planning/260828-0712_*_plan-vorschau-rendert-pdf-als-betrachter.md:179-181` (Entscheidung 10) und `:252-257` (Schritt 10); `circles/260802-0842-krk-mac-dateimanager-editor-git/_b_circle.md:139` (der Abnahmelauf der Runde 1 ist Nutzerarbeit und steht aus); Vorbild `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/issues/260827-1710_*_c2-5-der-runde-16-sagt-unveraendert-ueber-die-ganze-anzeige-und-die-waechst-seit-der-runde-19-um-drei-zeilen.md`

## Der Befund

Der Wortlaut von C6 (Spec der Runde 1, `:316-317`):

> Textdateien und Markdown-Dateien bis 1 MB erscheinen mit ihrem Inhalt, die gängigen Bildformate bis 64 MB. Oberhalb ihrer Grenze erscheint die Datei als Metadaten, so wie das folgende Kriterium sie für alles Übrige beschreibt.
>
> Alles andere, einschließlich Ordner und Dateien ohne darstellbaren Inhalt, erscheint als Metadaten: Name, vollständiger Pfad, Größe, Änderungsdatum, Rechte und Typ.

Zwei Aussagen stecken darin.

1. **Die drei Wege selbst.** Sie gelten. `laden` (`crates/krk-ui/src/vorschaumodell.rs`) verzweigt weiterhin nach Text bis 1 MB, Bild bis `BILDGRENZE` (`:201`, 64 MB) und Metadaten für alles Übrige, und der PDF-Zweig (`:824-835`) steht als eigene Abfrage `ist_pdfpfad` daneben, ohne einen der drei zu verändern. Was kein lesbares PDF ist, über der Grenze liegt oder ein Kennwort verlangt, fällt auf die Metadaten zurück, also auf den dritten Weg, wie ein Bild, das `NSImage` nicht liest.

2. **„Alles andere … erscheint als Metadaten."** Diese Aussage trifft für die Anzeige als Ganzes nicht mehr zu. Eine Datei mit der Endung `pdf` bis 64 MB ist seit der Runde 20 nicht „alles andere", sondern ein vierter Weg: sie reist als `Inhalt::Pdf { daten, metadaten }` (`vorschaumodell.rs:320`) in die Ansicht, die die Bytes PDFKit zum Deuten gibt. Aus der Dreiteilung ist für eine Datei eine Vierteilung geworden.

**Ursache** ist der Spec der Runde 20, `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/planning/260828-0649_*_spec-vorschau-rendert-pdf-als-betrachter.md`, dessen Diagramm unter `## Wie der Betrachter in die Vorschau tritt` (`:45-70`) die drei Wege der Runde 1 ausdrücklich als „unveraendert" zeichnet und den Betrachter als „der vierte Weg, neu" daneben stellt. Dieser Datensatz ist ein Befund über einen Text und nicht über Code: der gebaute Stand tut, was die Runde 20 verlangt.

**Der Modulkopf ist nachgezogen.** Schritt 5 des Plans hat den Abschnitt `# Die Dreiteilung der Anzeige (C6)` in `crates/krk-ui/src/vorschaumodell.rs:27-44` um einen Absatz erweitert; der uncommittete Stand lautet (`:36-44`):

> **Seit der Runde 20 hat eine Datei vier Wege, und der Abschnitt behaelt seinen Namen, weil C6 der Runde 1 so heisst.** Der vierte ist der Betrachter: eine Datei mit der Endung `pdf` bis 64 MB reist als [`Inhalt::Pdf`] mit ihren Bytes in die Ansicht, die sie PDFKit zum Deuten gibt. Die drei Wege der Runde 1 gelten unveraendert; der dritte traegt weiter alles Uebrige, und das PDF ueber der Grenze, das kein PDF ist oder ein Kennwort verlangt, faellt auf ihn zurueck wie ein Bild, das `NSImage` nicht liest (C2.1 bis C2.6 der Runde 20). Die Grenze ist [`BILDGRENZE`] und keine zweite Zahl daneben.

Der Abschnittsname bleibt „Dreiteilung", weil er den Namen des Kriteriums trägt und nicht die Zahl der Wege; die Zusammenfassung der Runde 16 hat die Zählung schon einmal berührt, ohne sie zu ändern (`:73-80`, „teilt die Dreiteilung nicht in vier, sondern besetzt einen Teil des dritten"). Der Betrachter tut, was jene nicht tat: er besetzt keinen Teil des dritten Weges, sondern tritt als eigener daneben.

**Der Spec der Runde 1 wird nicht angefasst.** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md` ist der freigegebene Wortlaut einer geschlossenen Runde, und ein fremder Spec ist nicht der Ort, an dem die Runde 20 ihre Wirkung einträgt. Die Buchung geschieht durch dieses Zitat, nicht durch eine Änderung dort (Plan der Runde 20, `:179-181`), wie die Runde 19 es für C2.5 der Runde 16 getan hat.

## Schließbedingung

Der Datensatz bleibt offen, weil der Abnahmelauf der Runde 1 aussteht (`_b_circle.md:139`: er verlangt KRK im Vordergrund) und die Schließung dem Nutzer gehört. Wer C6 abnimmt, liest die drei Wege als das Kriterium und nimmt für die Formel „alles andere" diesen Datensatz als Erklärung: ein PDF im Betrachter statt in den Metadaten ist kein Verstoß gegen C6, sondern die Runde 20. Geschlossen wird er mit jenem Abnahmelauf oder mit dem der Runde 20 (Plan, Schritt 11), sobald der Nutzer den Betrachter neben den drei Wegen abgenommen hat.
