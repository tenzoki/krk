# Das andere Dateifenster zieht nach, und fremde Anwendungen dürfen abwerfen

---
**Domain:** code
**Status:** closed
**Filed by:** orchestrator
**Active spec/plan:** shared/planning/260818-1510_*_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md
**Active session history:** shared/history/260818-1117-orchestrator-session.md

---

## Directive

KRK stellt nach dieser Runde das andere Dateifenster auf einen Tastendruck hin auf den Ordner des aktiven, und es nimmt Dateien und Ordner entgegen, die eine fremde Anwendung in eine seiner Dateilisten wirft. Der Abwurf landet in dem Ordner, den der Zeiger beim Loslassen benennt: über einer Ordnerzeile in diesem Ordner, sonst in dem, den die Liste zeigt. Kopiert wird in der Vorgabe, verschoben mit `cmd`; welcher der beiden Vorgänge gilt, beantwortet das System und nicht KRK. Was KRK nicht ausführen kann, weist es schon während des Ziehens ab, damit der Zeiger es vor dem Loslassen zeigt.

## Grounding snapshot

Erhoben am 260818 gegen den Baumstand `8d5baf6`, Version 0.5.2. Die dreizehnte gefahrene Runde des Projekts.

### Die Zählung der Runde, gezählt und nicht übernommen

Der Bestand führt vierzehn Circle-Datensätze. Zwei davon haben nie einen Turn gefahren: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` steht auf `_a_`, und `260816-2255-befehle-absetzen-und-makros-speichern` steht auf `_d_`, zurückgestellt am 260817 zugunsten der zwölften Runde, mit leerem `## Turn log`. Bleiben zwölf gefahrene Runden, und diese ist die dreizehnte.

**Die Prosa in `CLAUDE.md` ist an dieser Stelle hinterher und nennt zehn Runden.** Wer die Zahl braucht, zählt die Datensätze und ihre Turn-Protokolle, wie es die Datei an ihrer eigenen Tabelle selbst verlangt. Der Spec dieser Runde zählt bereits richtig: er nennt den Abnahmelauf vom 260810 als vor den Runden 5 bis 13 liegend.

### Der Baum kennt kein Ziehen, in keine Richtung

Am 260818 über `crates/` gemessen: `registerForDraggedTypes:`, `draggingEntered:`, `validateDrop:`, `acceptDrop:`, `beginDraggingSessionWithItems:event:source:`, `tableView:pasteboardWriterForRow:` und `NSFilePromiseReceiver` haben zusammen null Fundstellen. KRK ist heute weder Ziehquelle noch Abwurfziel.

Das ist für den Zuschnitt der Runde bedeutsam, weil es die Rückfallposition streicht. Es gibt keine halbfertige Ziehannahme, an die anzuschließen wäre, und keine bestehende Sortenanmeldung, deren Verhalten den Plan einengte. Die Runde baut die Annahme ganz, und der Spec nimmt die Abgabe ausdrücklich heraus (`## Nicht Gegenstand dieser Runde`).

### Die Einstiegspunkte, gegen den Baum gelesen

| Stelle | Was sie heute tut |
|---|---|
| `crates/krk-ui/src/appkit/tabelle.rs:853`, `DateifensterQuelle::ordner_lesen` | stellt ein Dateifenster auf einen Ordner. Die eine Klasse, die beide Gegenstände dieser Runde berühren |
| `crates/krk-ui/src/appkit/anwendung.rs:4428`, `Anwendungsdelegierter::uebertragen` | löst auf, welches das andere Dateifenster ist, und trägt heute den Weg zu Kopieren und Verschieben |
| `crates/krk-ui/src/appkit/anwendung.rs:3862`, `Anwendungsdelegierter::bereich_einblenden` | der Weg für Befehle, die einen Bereich brauchen, statt ihn umzuschalten |
| `crates/krk-ui/src/appkit/anwendung.rs:5302`, `auftrag_stellen` | nimmt seine Quellen aus der Auswahl des aktiven Dateifensters und passt für einen Abwurf deshalb nicht |
| `crates/krk-ui/src/appkit/anwendung.rs:5348`, `vorgang_laeuft_schon` | beantwortet die Frage nach dem einen laufenden Vorgang |
| `crates/krk-ui/src/appkit/anwendung.rs:5368`, `auftrag_starten` | nimmt einen fertigen `Auftrag`; der gemeinsame Teil, den heute zwei Wege benutzen |

### Die zwei Bedeutungen eines `false`

`Fenstermodell::einblenden` (`crates/krk-ui/src/fenstermodell.rs:735`) liefert `false` in zwei verschiedenen Lagen, und der Rumpf zeigt beide: `if self.sichtbar(bereich) { return false; }` für den Bereich, der schon dasteht, und danach das `false` aus `umschalten`, wenn die Mindestbreiten bei der aktuellen Fensterbreite nicht mehr nebeneinander passen. Nur die zweite ist eine Abweisung.

Das `#[must_use]` an derselben Zeile benennt die eine Hälfte der Falle und nicht die andere: „wer sie nicht liest, haelt einen Bereich fuer hervorgeholt, den das Modell nicht eingeblendet hat". Wer den Rückgabewert liest, aber die Lagen nicht trennt, baut die andere Hälfte ein und meldet dem Nutzer ein zu schmales Fenster, während das Dateifenster längst dasteht. C2 des Specs verlangt deshalb, dass die Sichtbarkeit **vor** dem Einblenden gefragt wird.

### Was der Übersetzer einfordert, am 260818 gezählt

`enum Kommando` (`crates/krk-core/src/tasten/belegung.rs`) trägt 78 Varianten, `KENNUNGEN` ebenso 78, und beide wachsen um eins. `Kommando::wirkungsbereich` und `bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs`) sind vollständige Fallunterscheidungen ohne Auffangzweig und halten den Bau an, bis die Zeile steht. `resources/default-keymap.toml` führt 84 Blöcke `[[funktion]]` und wächst auf 85. `opt+cmd+s` ist ab Werk unbelegt, gegen die Datei nachgezählt.

Der Abwurf bringt **keine** neue Auftragsart mit. Er mündet in `Art::Kopieren` oder `Art::Verschieben`, also in dieselben zwei Werte, die F5 und F6 heute stellen; `schiebt_auffrischung_auf` bekommt keine Zeile.

### Die Untergrenze macOS 15 hält ohne Ausnahme

Jede Klasse und jede Methode, die die Runde anspricht, steht weit darunter, am SDK gelesen: `registerForDraggedTypes:` und `NSDraggingInfo` seit 10.0, `readObjectsForClasses:options:` seit 10.6, `NSFilePromiseReceiver` seit 10.12, `NSPasteboardTypeFileURL` seit 10.13; die vier Tabellenmethoden tragen im Kopf gar keine Verfügbarkeitsangabe. Das ist keine Formalie: `objc2` führt keine Verfügbarkeitsangaben mit, der Übersetzer hält die Untergrenze nicht, und eine Methode darüber gibt keine Warnung, sondern einen Absturz auf dem Referenzgerät.

### Die zehn Zeitzusagen bleiben zehn

Keine der zehn Zusagen aus C8 der Runde 1 ist berührt, und die Runde setzt keine elfte. Der Spec ordnet sie einzeln gegen die Kennungen in `crates/krk-bench/src/messen.rs` zu; kein Ziehvorgang und kein Ordnerwechsel im **anderen** Dateifenster kommt darin vor.

An die Stelle einer Zahl treten zwei ohne Messstrecke prüfbare Kriterien, in derselben Bauart, die die Runde 2 dafür gewählt hat: die erste Bildschirmseite im nachgezogenen Dateifenster vor dem Rest des Ordners, und eine Liste, die während eines stehenden Ziehvorgangs bildlauffähig bleibt. Beides ist Nutzerarbeit.

Der Abnahmelauf der zehn Zusagen ist zuletzt am 260810 gefahren und liegt damit vor den Runden 5 bis 13. Diese Runde ändert daran nichts und verlangt keinen neuen Lauf.

### Was der Spec offen an den Planner gibt

Sieben Fragen, alle technisch: wo die Ziehannahme wohnt, wie die Wahl zwischen Kopieren und Verschieben ohne AppKit prüfbar wird, womit KRK das Schreibrecht eines Ordners feststellt, wie die Ablage des Ziehvorgangs die eine Hülle um `NSPasteboard` erreicht, wie der Abwurf in die Operationsmaschine kommt, wie der Befehl den Zielordner setzt und einblendet, und wo die Proben stehen. Zwei Grenzen sind dabei gesetzt und nicht verhandelbar: eine zweite Hülle um `NSPasteboard` entsteht nicht, und die Frage nach dem laufenden Vorgang wird nicht zweimal beantwortet.

### Ein gemessener Abweichungspunkt

Der Spec nennt unter C3 den Datensatz `shared/issues/260815-1047_*_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md` und sagt, er bleibe offen. **Am Bestand trägt er `_d_`**, also als Lage angenommen und nicht offen. An der Sache ändert das nichts: der Zustand am Code besteht fort, und die Runde macht ihn weder besser noch schlimmer. Wer den Marker beim Aktivieren nachzieht, zieht ihn im Spec nach und nicht am Datensatz.

## Dependencies

Kein anderer Circle blockiert diesen. Die folgenden Runden binden ihn über ihre Datensätze; zitiert wird, wo sie liegen, kopiert wird nichts.

- `circles/260802-0842-krk-mac-dateimanager-editor-git` — die Runde 1. Sie hat die Operationsmaschine mit Fortschritt, Abbruch und Konfliktrückfrage gebaut, in die der Abwurf mündet, die Selbstauffrischung des angezeigten Ordners aus C9, auf die C4 sich stützt, und die zehn Zeitzusagen aus C8.
- `circles/260811-1304-statusleiste-mit-bereichsschaltern` — die Runde 5. Sie hat `bereich_einblenden` und die Mindestbreiten-Abweisung gebaut, auf denen C2 vollständig steht. Bindend: `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_*_was-geschieht-wenn-die-mindestbreiten-nicht-mehr-hineinpassen.md` und `.../260812-0415_*_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`, beide umgesetzt.
- `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief` — die Runde 10. C3 erbt ihre Regel unverändert, statt eine zweite danebenzusetzen. Bindend: `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_*_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`, umgesetzt.
- `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` — die Runde 4. Sie hat `appkit/zwischenablage.rs` von der Quelle zum Ziel erweitert und damit festgelegt, dass es genau eine Hülle um `NSPasteboard` gibt. Bindend: `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/decisions/260811-1552_*_welche-sorten-legt-der-pfadkopierer-in-die-zwischenablage.md`, umgesetzt.
- `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` — die Runde 12. Sie hat `Kommando` und `KENNUNGEN` auf die 78 gebracht, von denen diese Runde ausgeht, und ihre Haltung zu zerstörerischen Wegen ist im tragenden Entscheidungsdatensatz unten als Maßstab herangezogen.

**Bindende Datensätze:**

- `shared/decisions/260818-1453_*_welche-zusatztaste-macht-aus-einem-abwurf-ein-verschieben.md` — beantwortet am 260818, der tragende Datensatz dieser Runde. Er hält fest, dass nicht KRK die Frage nach Kopieren oder Verschieben beantwortet, sondern das System über `draggingSourceOperationMask`, und dass die erste Nutzerantwort (`shift`) daran gescheitert wäre. C5 des Specs steht darauf.
- `shared/issues/260815-1047_*_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md` — als Lage angenommen (`_d_`). Berührt C3 in der Anzeige, nicht im Verhalten; die Runde hebt ihn nicht auf.
- `shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md` — offen. Diese Runde bringt eine neue Funktion in die Belegung und läuft damit erneut in seine Wirkung.

## Turn log

- Runde 1 (Sitzung 260818-1117): Commits `b47355e..71413c3`; Bündel A gebaut, also die
  Fähigkeiten C1 bis C3, `make check` grün. Durchsicht `reviews/260818-2133-coderev-round-13-turn-1-ordner-angleichen.md`,
  sechs Datensätze gefilt, keiner hält eine Auslieferung auf. Ein Widerspruch zwischen C1
  und C2 des Specs ist dabei aufgefallen und vom Nutzer entschieden: ein ausgeblendetes
  Zieldateifenster wird auch dann eingeblendet, wenn es den Ordner schon führt, aber nicht
  neu gelesen. Kohärenzverdikt: coherent. Sitzungsprotokoll:
  `shared/history/260818-1117-orchestrator-session.md`

- Runde 2 (Sitzung 260818-1117): Commits `a6b3818..801d594`, acht Stück; Bündel B gebaut, also die
  Fähigkeiten C4 bis C7, dazu die Berichtigungen beider Durchsichten. Durchsicht
  `reviews/260818-2340-coderev-round-13-turn-2-abwurf-aus-fremden-apps.md`, fünf Datensätze,
  alle behoben. Der schwerste kam dabei nicht aus dieser Runde: der textuelle Pfadvergleich
  hätte über `ziel_klaeren` und „Überschreiben" eine Datei löschen können, deren zweite
  Schreibweise das Ziel war. Behoben in `krk-core` über `(st_dev, st_ino)`. Kohärenzverdikt:
  coherent. Abgleich: `history/260819-0102-reconciliation.md`.

## Closure note

**Geschlossen kohärent (`_c_`) am 260819, nach gefahrenem Abnahmelauf des Nutzers.**

Beide Fähigkeiten stehen. `opt+cmd+s` stellt das andere Dateifenster auf den Ordner des
aktiven und holt es dafür hervor, wenn es ausgeblendet ist; der Zieltab behält seine eigene
Sicht. Eine Dateiliste nimmt Dateien und Ordner aus fremden Anwendungen entgegen, mit dem
Ordner unter dem Zeiger oder dem angezeigten als Ziel, Kopieren als Vorgabe, `cmd` für das
Verschieben und `opt` für das erzwungene Kopieren.

**Der Abnahmelauf ist gefahren, und das unterscheidet diese Runde von zehn der zwölf vorigen.**
Der Nutzer hat am 260819 die zehn Prüfungen am gebauten Bündel 0.5.2 abgenommen, darunter alle
Kriterien zu C4 bis C7, die einen Ziehvorgang aus einer zweiten Anwendung verlangen, und die
zwei aus C2, die das Fenster an seiner Breite zu ziehen verlangen. Damit schließt die Runde
kohärent statt beschränkt. Sie ist die **dritte**, die kohärent schließt, und nach der Runde 8
die zweite, die den Abnahmelauf dafür wirklich gefahren hat: die Runde 12 hat kohärent ohne
einen solchen Lauf geschlossen. Die Liste der zehn abgenommenen Prüfungen steht in
`history/260819-0810-abnahmelauf.md`; ohne sie stünde die Behauptung des Laufs allein in
dieser Notiz, die von ihm abhängt.

**Was die Runde über ihren Auftrag hinaus gefunden hat.** Der Abwurf machte einen Weg
erreichbar, der älter ist als er: `operation::ziel_klaeren` beantwortet „Überschreiben" mit
`loeschen::baum_entfernen`, also einem echten `remove_file` und nicht dem Papierkorb. War das
Ziel unter zweiter Schreibweise die Quelle, löschte das die Datei des Nutzers. Gegen den
unreparierten Baum gefahren endete es mit „die Quelle ist weg: NotFound"; derselbe textuelle
Schutz ließ einen Ordner 139 Einträge weit in seinen eigenen Baum absteigen. Behoben an der
entscheidbaren Stelle, `operation::zielpfad` über `(st_dev, st_ino)`, mit `lstat` und `stat`
für die zwei verschiedenen Fragen.

**Was offen bleibt.** Drei Datensätze im Speicher dieser Runde: die falsche Zusage des Plans
zum grünen Zwischenstand nach Schritt 1, der Abwurf, der sein Ziel zugleich als Quellordner
durchreicht und den Abschluss denselben Ordner zweimal lesen lässt, und die „dritter Rufer"-
Formulierung des Plans. Keiner hält etwas auf.

Sitzungsprotokoll: `shared/history/260818-1117-orchestrator-session.md`
