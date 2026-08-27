# Portfolio

**Generated:** 260827-1927 (by playmaker session 260827-1927-playmaker-orchestrator-phase4)
**Domain bias:** code

Bestand: 0 vorgesehen, 0 aktiv, 6 kohärent geschlossen, 12 beschränkt
geschlossen, 0 überholt, 2 zurückgestellt. Summe 20 Circle-Datensätze. Die Runde
18 ist ohne Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Der reguläre
Zustand nach dem Abschluss der Runde 19 am 260827-1920.

## Anticipated (_a_) — ranked

(none)

Es gibt keinen vorgesehenen Circle. Der nächste entsteht aus der Ablage: siehe
`## Backlog — ranked`.

## Backlog — ranked

Recommended to shape: `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md` — der einzige offene Eintrag, dessen Sache nicht gebaut ist; Bilder zeigt die Vorschau seit der Runde 1, PDF nirgends.
`/fusion:direct shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md`

**1. `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md`**
(`_p_`, in diesem Lauf empfohlen) — die Vorschau soll beim Navigieren jpg, png
und vor allem PDF rendern. Die Hälfte davon steht: `Inhalt::Bild` und
`ist_bildpfad` (`crates/krk-ui/src/vorschaumodell.rs`) nehmen png, jpg, jpeg,
gif, tiff, heic, heif, bmp und icns bis `BILDGRENZE` von 64 MB an, und die
Vorschau zeigt sie über eine `NSImageView` (`crates/krk-ui/src/appkit/vorschau.rs`),
seit C6 der Runde 1. PDF liest kein Weg im Baum. Wir empfehlen, den Eintrag
auszuarbeiten und den Gegenstand dabei auf PDF zu verengen; ein Spec, der die
Bildanzeige noch einmal verlangt, beschriebe Gebautes. Die zwei Fragen, die der
Shaper stellen wird: ob PDF über `PDFKit` (eine weitere AppKit-Klasse mit
Untergrenzen-Abschnitt) oder als gerasterte Seite über `NSImage` ankommt, und
ob die Auswahl- und Kopierzusage der Runde 14 (`circles/260819-2230-auswahl-und-kopieren-in-der-vorschau`)
für Seiten eines PDFs gilt.

**2. `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`**
(`_o_`) — verlangt eine zweite Kombination neben `f4` für den Editor-Einstieg.
Gebaut: `editor_rundweg` auf `cmd+e` öffnet seit dem 260823 denselben
ausgewählten Eintrag (`resources/default-keymap.toml`, Kommentar bei
`bearbeiten`).
  `close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e (editor_rundweg) öffnet seit dem 260823 im Dateifenster denselben ausgewählten Eintrag wie f4`

**3. `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`**
(`_o_`) — verlangt Leseprofile in einer Definitionsdatei unter
`~/Library/Application Support/KRK/`. Gebaut von der Runde 16 als `readers.toml`
mit zwölf Auslieferungsprofilen (`resources/default-readers.toml`), seit der
Runde 19 dazu das eingebaute Default-Profil.
  `close shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md — die Runde 16 hat die Leseprofile als readers.toml gebaut, mit zwölf ausgelieferten Profilen für die Werkbank`

Die zwei Schließungen sind Vorschläge, zum zweiten Mal nach dem Lauf 260827-0403.
Dieser Lauf ist ein Phase-4-Dispatch ohne Nutzer und hält für keine der beiden
eine Bestätigung; ausgeführt hat er allein die Umbenennung des ersten Eintrags
auf `_p_`. Ein interaktiver Lauf über `/fusion:next` legt die zwei Zeilen zur
Bestätigung vor.

## Recently closed (_c_ / _b_)

1. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`,
   260827-1920) — die Vorschau zählt den Inhalt eines Ordners in einem
   eingebauten Default-Profil. Ein Turn, acht Planschritte, Abnahmelauf vom
   Nutzer gefahren, Abgleich `coherent`. Zwei Low-Befunde der Durchsicht bleiben
   als offene Defekte für eine Folgerunde.
2. `260825-0711-kontextmenue-traegt-zip-unzip-finder` (`_b_`, 260825-1422) —
   das Kontextmenü trägt Zip, Unzip und Finder neben dem Teilen. Der Datensatz
   trägt keine Schließungsnotiz; die Begründung steht in `git:2a77012`. Siehe
   `## Warnings`.
3. `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (`_b_`,
   260824-1810) — `readers.toml` als siebte Ablagedatei, die Erkennung in zwei
   Durchgängen, die vier Bausteine mit ihrem Haushalt. Beschränkt, weil sieben
   Abnahmekriterien KRK im Vordergrund verlangen.
4. `260821-1644-veroeffentlichen-als-achte-station` (`_c_`, 260821-2110) —
   Veröffentlichen als achte Station der Auslieferungskette, mit gefahrenem
   Abnahmelauf des Nutzers über fünfzehn Kriterien.
5. `260819-2230-auswahl-und-kopieren-in-der-vorschau` (`_c_`, 260820-1045) —
   die Vorschaufläche wird auswählbar, kopiert wird der Quelltext.

## Archived (_s_ / _d_)

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`,
  260821-2202) — KRK zeigt Web-Seiten in einem eigenen Betrachter. Abgesagt, nicht
  verschoben: der Nutzer hat das Abgeben an den Systembrowser gewählt
  (`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`).
  Das Vokabular kennt für eine Absage keinen eigenen Marker.
- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, 260817-0445) — KRK
  setzt Befehle ab und führt gespeicherte Makros aus. Nichts ist gebaut. Hier
  heißt der Marker „später": die Runde war aktiv und ist der Löschabsicherung
  gewichen.

Überholte Runden (`_s_`) gibt es nicht.

## Warnings

- Der Datensatz der Runde 17,
  `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_b_circle.md`, trägt
  keinen Abschnitt `## Closure note` und ein leeres Turn-Protokoll. Der
  Abschluss am 260825 war eine reine Umbenennung (`git:2a77012`); was die Runde
  erreicht hat und warum sie beschränkt geschlossen ist, steht allein in der
  Commit-Nachricht. Unverändert seit dem Lauf 260827-0403.
- Drei weitere terminale Datensätze tragen ein leeres Turn-Protokoll:
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`,
  `260819-2230-auswahl-und-kopieren-in-der-vorschau` und, mit einer
  Platzhalterzeile statt Einträgen, die zwei zurückgestellten Runden.
- Zwei Ablageeinträge sind gebaut und stehen offen, weil außer der Promotion
  durch den Shaper kein Weg einen Eintrag schließt; der dritte war zur Hälfte
  gebaut, als der Nutzer ihn gefilet hat. Die zwei Schließungen stehen oben als
  Zeilen zur Bestätigung.
- Die Runde 19 lässt zwei Low-Befunde der Durchsicht als offene Defekte zurück:
  `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/issues/260827-1911_*_drei-saetze-im-kommentarteil-der-auslieferungsfassung-beschreiben-den-stand-vor-der-runde-19.md`
  (Ontocoder) und
  `…/issues/260827-1911_*_erkennung-rs-sagt-none-heisse-die-heutige-metadatenanzeige-und-das-ist-seit-der-runde-19-der-rueckfallzweig.md`
  (Coder). Beide sind Aufräumarbeit für eine Folgerunde und keine
  Vorbedingung.
- Kein Zeigerfehler, kein Abhängigkeitszyklus, keine veraltete Grundlage: es
  gibt keinen nicht-terminalen Circle, und der Abschluss der Runde 19 ist
  kohärent, also keine Bounded-Closure-Propagation.
