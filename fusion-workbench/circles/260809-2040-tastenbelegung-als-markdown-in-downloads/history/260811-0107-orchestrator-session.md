# Orchestrator Session — 260811-0107

**Directive:** Die geltende Tastenbelegung als Markdown-Datei im Downloads-Ordner.
**Mode:** custom (Circle-Aktivierung)
**Status:** In Arbeit

## Aktivierung

Der Circle `260809-2040-tastenbelegung-als-markdown-in-downloads` ist am 260811-0107 auf
ausdrückliche Wahl des Nutzers aktiviert worden: der Datensatz ging von `_a_` auf `_t_`, und
`fusion-workbench/.active-circle` trägt den Verzeichnisnamen. Damit zeigen alle `OUT_*` in den
Circle; die `SCAN_*` decken den Circle und den gemeinsamen Speicher ab.

**Kein Plane-Push.** `plane.config.yaml` ist die unveränderte Vorlage (`plane.example.com`,
`your-workspace-slug`, Null-UUID), und `$PLANE_API_KEY` ist nicht gesetzt. Der Mirror ist damit
nicht eingerichtet, und der Push entfällt — nicht weil er fehlschlug, sondern weil es nichts zu
spiegeln gibt.

## Vorabprüfung zur fünften Frage

Der Datensatz
`decisions/260809-2040_*_welche-belegung-schreibt-die-ausgabe-bei-offener-belegungsansicht.md`
trägt eine ausdrückliche Aufforderung: ob ein Menüeintrag bei stehendem Blatt noch anschlägt, sei
nicht gemessen und vor der Antwort nachzusehen. Nachgesehen am 260811-0107:

- Die Belegungsansicht wird über `beginSheetModalForWindow_completionHandler`
  (`crates/krk-ui/src/appkit/blaetter/mod.rs:508`) gezeigt, also **dokumentmodal** und nicht über
  `runModal`. Eine eigene Ereignisschleife bringt sie nicht mit.
- Eine eigene `validateMenuItem`-Überschreibung gibt es im ganzen Baum nicht; die Suche liefert
  null Treffer.

`inference:`, nicht gemessen: ein dokumentmodales Blatt lässt die Menüleiste bedienbar, und ohne
eigene Prüfung der Menüeinträge schlägt ein solcher Eintrag an. Damit ist die fünfte Frage beim
Menüweg **nicht** gegenstandslos, sondern zu beantworten. Beim Belegungsweg bleibt sie es, weil
der Ereignisabgriff bei stehendem Blatt allein `abbrechen` durchlässt.

## Verlauf

- 260811-0107 — Circle aktiviert, Pfade neu aufgelöst, fünf offene Fragen dem Nutzer vorgelegt.

## Die sieben Klärungsfragen des Spec, beantwortet am 260811-0115

Der `shaper` hat sie zurückgegeben, statt sie selbst zu stellen: als Unteragent hat er kein
`AskUserQuestion` und hat das benannt, statt ein Werkzeug zu behaupten. Seine Vorarbeit liegt in
`history/260811-0446-shaper-klaerungsrunde-tastenbelegung-ausgabe.md`.

| # | Frage | Antwort |
|---|---|---|
| 1 | Dateiname | `KRK-Tastenbelegung.md` |
| 2 | Kopfzeile | nur eine Überschrift, kein Zeitstempel |
| 3 | Beschriftung der dritten Spalte | ausgeschrieben, ohne Legende |
| 4 | dritte Spalte bei den sechs Textbefehlen | „Textfelder und Editor" |
| 5 | Erfolgsmeldung | mit vollem Pfad |
| 6 | Überschreiben melden? | nein, eine Meldung für beide Fälle |
| 7 | gesicherten Stand melden? | **nein** |

**Zwei Antworten tragen einen Vorbehalt, und der gehört in den Spec statt in die Erinnerung.**

Antwort 4 steht auf einer Ableitung. Dass die sechs Textbefehle „in Textfeldern und im Editor"
wirken, ist aus dem Aufbau der Antwortkette geschlossen und **nicht gemessen**. Der Spec
kennzeichnet es als Annahme, und wer den Planschritt baut, prüft es zuerst — eine Spalte, die
eine falsche Zusicherung gibt, ist schlimmer als eine leere. Dieselbe Fehlerform hat die
Durchsicht in der vorigen Sitzung zweimal gefunden.

Antwort 7 weicht von der Empfehlung ab, und der Preis ist damit angenommen: **wer bei offener
Belegungsansicht ausgibt, bekommt eine Datei, die dem Schirm widerspricht, ohne es zu
erfahren.** Die Abweichung besteht nur bis zum Sichern und entsteht allein durch eine eigene
Handlung des Nutzers. Der Spec schreibt den Fall aus, statt ihn wegzulassen.

**Was der Spec ohne Rückfrage festhält**, vom `shaper` am Code erhoben: die Schreibweise der
Kombinationen kommt aus `anzeige()` (`crates/krk-ui/src/belegungsmodell.rs:527`), also
`Shift+Cmd+K` und `F3` — eine abweichende Schreibweise wäre die zweite Aufbereitung, die die
Directive ausschließt. Keine der vier bestehenden vollständigen Fallunterscheidungen wächst; die
Belegung bleibt bei 71 Funktionen, weil der Menüeintrag kein Kommando mitbringt. Neu hinzu kommt
dagegen **eine** vollständige Fallunterscheidung: die Beschriftung der sieben Wirkungsbereiche,
zu bauen ohne Auffangzweig, wie `Funktionsbereich::name()` es vormacht.

## Turn 1 — S1 bis S3 gebaut, mit einer widerlegten Ableitung

### Die Messung aus S1, und sie ist das Ergebnis dieses Turns

S1 sollte prüfen, ob die Beschriftung „Textfelder und Editor" für die sechs vom Hauptmenü
zugestellten Textbefehle trägt. Der Plan hatte den Mechanismus gewechselt, statt zu nähern:
gemessen wird über `AnyClass::responds_to` am Objective-C-Laufzeitsystem — ohne Instanz, ohne
Hauptfaden, ohne Vordergrund. Damit wurde aus Nutzerarbeit Agentenarbeit.

| Selektor | antwortet an | trägt die Methode | Befund |
|---|---|---|---|
| `cut:` | NSTextView | NSText | bestätigt |
| `copy:` | NSTextView | NSText | bestätigt |
| `paste:` | NSTextView | NSText | bestätigt |
| `selectAll:` | **NSTableView** und NSTextView | **NSTableView** bzw. NSText | **widerlegt** |
| `undo:` | NSWindow | NSWindow | nicht entscheidbar |
| `redo:` | NSWindow | NSWindow | nicht entscheidbar |

`NSTextField`, `NSScrollView` und `NSApplication` beantworten keinen der sechs.

**Der Verdachtsfall, den der `shaper` beim Schreiben des Specs benannt hatte, ist eingetreten.**
`NSTableView` beantwortet `selectAll:` aus einer eigenen Methode, und die Lesezeichenleiste ist
eine. Die Zelle für `text_alles_auswaehlen` bleibt deshalb leer. Der volle Befund samt dem, was
die Messung ausdrücklich **nicht** entschieden hat, steht in
`issues/260811-0930_*_die-ableitung-textfelder-und-editor-bricht-fuer-alles-auswaehlen-*.md`;
drei Proben halten die Messung im Baum.

**Das ist der Wert dieser Runde.** Der Spec hat eine Vermutung als Vermutung gekennzeichnet und
ihre Prüfung erzwungen, bevor sie als Zusicherung in eine Datei geriet. Ohne dieses Kriterium
stünde heute in `~/Downloads/KRK-Tastenbelegung.md` eine falsche Angabe.

### Dieser Abschnitt ist zugleich die Antwort auf einen Befund

Die Durchsicht hat vermerkt, dass es zu S1 bis S3 keinen Sitzungsbericht gibt, obwohl S1 einen
als Abnahmekriterium führt (`issues/260811-0959_*_…`). **Die Ursache liegt bei mir:** meine
Aufgabenstellung an den `coder` verbot jede Datei unter `fusion-workbench/`, und das schloss sein
Historienprotokoll mit ein. Dieselbe Form wie am 260810
(`shared/issues/260810-1907_*_die-durchsicht-von-turn-2-hat-kein-durchsichtsdokument-hinterlassen.md`).

**Nachträglich erzeugt wird kein Bericht des `coder`** — ein Protokoll über eine Arbeit, das
jemand schreibt, der sie nicht getan hat, ist ein Beleg über die Arbeit statt der Arbeit selbst.
Die Substanz steht stattdessen hier, im Defektdatensatz `260811-0930`, in den Commit-Nachrichten
und im Modulkopf von `menue.rs`. Das Kriterium ist damit erfüllt, und der Weg dorthin steht
dabei.

### Was Turn 1 sonst ergeben hat

Sechs Commits: `39687f3` (S1), `33cc083` (S2), `f1ce0f5` (Circle-Datensatz), `fd863e3` (S3),
dazu die Durchsichtsbefunde. Die Durchsicht hat sechs Defekte abgelegt, von denen zwei mittleren
Gewichts eine Entscheidung des Nutzers verlangten:

- **Der Auffangzweig war erreichbar**, und die Begründung des `coder` trug nicht — die Durchsicht
  hat den Gegenfall gemessen. Der Nutzer hat Weg b) gewählt: der Zweig trägt jetzt „(von KRK
  nicht eingeordnet)" statt einer leeren Zelle, die schon an `text_alles_auswaehlen` vergeben
  war. **Die Ungleichheit zwischen `bereich` und `wirkung` bleibt bestehen** und bleibt erfasst.
- **Der Nutzerentscheid vom 260811-0935 stand allein im Programmtext.** Auch das geht auf mich
  zurück: ich habe die Frage im Chat gestellt, die Antwort bekommen und sie in Commit-Nachrichten
  und Kommentare geschrieben, aber in keinen Datensatz. Nachgetragen als
  `decisions/260811-1010_a_was-traegt-die-dritte-spalte-bei-rueckgaengig-und-wiederholen.md`, und
  C3 im Spec ist auf den gemessenen Stand berichtigt.

**Der gewichtigste Nebenfund betrifft nicht den Code, sondern eine Zusage an den Nutzer.** Der
Text zu `NSDownloadsFolderUsageDescription` in `resources/Info.plist` nannte das Schreiben nicht.
Der `coder` hat am System belegt, dass der Schlüssel wirkt (`TCC.framework`), und dabei gesehen,
dass TCC **einmal je Programm und Dienst** fragt: KRK löst die Rückfrage schon beim Anzeigen des
Downloads-Ordners aus, seit Runde 1. Der Satz beschaffte damit Zustimmung für eine Handlung, die
er nicht nannte. Er ist ergänzt.
