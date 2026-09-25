# S40 ganz und S38 zur Hälfte: die Leiste zeigt beide Sorten

---
**Agent:** coder
**Status:** Complete
**Anlass:** S38 (eine Textmarke anlegen) und S40 (die Leiste zeigt beide Sorten) aus `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`
**Umfang:** `crates/krk-ui/src/leistenmodell.rs`, `crates/krk-ui/src/appkit/leiste.rs`, `crates/krk-ui/src/appkit/anwendung.rs` (drei Stellen: die Aufrufkette des Anlegens)
**Ergebnis:** `make check` grün — Bau, Proben, `fmt --check` und `clippy -D warnings`

---

## Was gebaut wurde

### Eine Tür für beide Sorten (die Hälfte von S38, die der Leiste gehört)

`Leistenmodell::anlegen` nahm bisher einen `&Path` und verpackte ihn fest in
`Ziel::Ordner`; der Platzhalter aus `65c8efa` nannte S38 als seinen Ablöser.
Seither nimmt die Funktion das fertige `krk_core::ablage::Ziel` entgegen und
fragt an keiner Stelle nach der Sorte. Dieselbe Form haben
`Leistenquelle::lesezeichen_anlegen` und
`Anwendungsdelegierter::lesezeichen_anlegen_ausfuehren` bekommen. Die Kette vom
Tastendruck bis in `bookmarks.toml` ist damit sortenblind, so wie
`Lesezeichenliste::anlegen` es seit S11 ist.

Der zweite Platzhalter, `Leistenmodell::gewaehlt` mit seinem `None` für eine
Textmarke, steht unverändert: er gehört S39 und wurde nicht angefasst.

### Das Sinnbild (S40)

`Leistenmodell::sinnbild(stelle) -> Option<Sinnbild>` nennt die **Sorte** und
kein Bild; `appkit/leiste.rs` macht daraus `folder` oder `doc.text` über
`NSImage::imageWithSystemSymbolName:accessibilityDescription:`. Die Trennung
folgt der Zusage des Modulkopfs von `leistenmodell.rs`: dort steht keine Zeile
AppKit, und ein Systembildname wäre eine.

**Die Regel des Sinnbilds gilt allen wählbaren Zeilen: es sagt, was die Zeile
öffnet.** Ein Gerät öffnet einen Ordner und trägt deshalb dasselbe Sinnbild wie
eine Ordnermarke; eine Überschrift öffnet nichts und trägt keines. Zwei Regeln
nebeneinander, eine für die Lesezeichen und eine für die Geräte, hätten zwei
Ausnahmen, sobald der untere Teil einmal etwas anderes als Ordner führt. Der
sichtbare Nebeneffekt: die Spalte des Sinnbilds steht in jeder wählbaren Zeile,
auch wenn das System das Bild nicht liefert, und alle Beschriftungen rücken
gleich weit ein.

Das Sinnbild ist das zweite Kennzeichen neben der Farbe, aus demselben Grund wie
der Zusatz "(fehlt)" bei einer ungültigen Marke: eine Farbe allein ist bei
Farbfehlsichtigkeit kein Kennzeichen. Das Sinnbild einer ungültigen Marke wird
mit `contentTintColor` so gedämpft wie ihr Text daneben, damit die Zeile als eine
gelesen wird.

### Was nicht zu ändern war

An `Leistenmodell::beschriftung` und `Leistenmodell::ungueltig` **keine Zeile**,
obwohl die Dateiliste von S40 beide nennt. Beide lesen die Marke, die
`Gemerkt::nachpruefen` gesetzt hat, und `Lesezeichen::gueltig` beantwortet seit
S11 beide Sorten mit je einer Frage an das Dateisystem. Die Zusage von S40, dass
die Gültigkeitsprüfung kein Lesevorgang wird, hält damit von selbst; zu tun war
sie zu messen, nicht sie herzustellen.

## Die Proben

Sechs neue in `crates/krk-ui/src/leistenmodell.rs`, alle grün:

| Probe | Was sie hält |
|---|---|
| `eine_angelegte_textmarke_haengt_unten_an_und_ist_ausgewaehlt` | `anlegen` nimmt beide Sorten, die Textmarke hängt unten an |
| `eine_gemischte_liste_behaelt_ihre_reihenfolge_und_zeigt_beide_sorten` | eine Ordnung, keine Sortierung nach Sorte, je Zeile das richtige Sinnbild |
| `eine_textmarke_ist_ungueltig_wenn_die_datei_fehlt_und_sonst_nie` | geänderter Zeileninhalt bleibt ohne Kennzeichen, fehlende Datei trägt "(fehlt)" |
| `die_vier_lesezeichenbefehle_wirken_auf_eine_textmarke` | Anlegen, Umbenennen, Verschieben, Löschen sind sortenblind |
| `zehn_textmarken_kosten_je_eine_frage_und_keinen_lesevorgang` | das elfte Abnahmekriterium von C6, siehe unten |
| (erweitert) `Pruefordner::datei` | Prüfdateien für die Textmarken, im selben selbsträumenden Ordner |

**Die Zählprobe misst zwei Aussagen und keine Zahl, und das ist eine
Einschränkung.** Die Zahl der Systemaufrufe ist von innerhalb des Prozesses
nicht zu zählen. Gemessen wird stattdessen, was von außen entscheidbar ist:

- **Je Marke eine eigene, aktuelle Antwort.** Zehn Marken auf fünf vorhandene und
  fünf fehlende Dateien ergeben fünf gültige; eine danach gelöschte Datei ändert
  genau ihre Marke, und `gueltigkeit_pruefen` meldet die Änderung. Eine
  gemeinsame oder eine gemerkte Antwort fiele hier auf.
- **Kein Lesevorgang.** Eine Marke auf eine Datei ohne Leserecht (`0o000`) bleibt
  gültig. Wer sie öffnete, bekäme `EACCES` und müsste sie für ungültig erklären.
  Nachgeprüft, dass die Bedingung trägt: als dieser Nutzer ist die Datei
  `stat`-bar und nicht lesbar. Unter `root` sagt diese Hälfte weniger, weil dort
  auch das Öffnen gelingt; falsch anschlagen kann sie deshalb nicht.

## Was offen bleibt

**S38 ist nicht erledigt** und trägt im Plan keinen `[DONE]`-Marker. Der
Editor-Zweig des Befehls `cmd+d` fehlt, und er hängt an einer Auskunft, die es in
`appkit/editor.rs` nicht gibt: Nummer und Inhalt der Zeile, in der die
Schreibmarke steht. `Editorbereich` gibt `pfad()`, `haelt_datei()` und
`textflaeche()` heraus, aber nichts über die Stelle der Schreibmarke.

Sie im Aufrufer zu rechnen war die Versuchung und wäre falsch gewesen: die
Umrechnung von AppKits UTF-16-Einheiten in Byteversätze steht schon als
`anfaenge_in_utf16` in `appkit/nummernspalte.rs`, und ein zweiter Rechenweg für
dieselbe Umrechnung müsste die Zusage über Zeichengrenzen aus dem Modulkopf von
`krk_core::text` doppelt tragen. `appkit/editor.rs` und `appkit/nummernspalte.rs`
gehören zudem einem parallel laufenden Bündel und waren für diesen Schritt
gesperrt.

Der Zuschnitt der fehlenden Funktion, die vier Eigenschaften, die sie tragen
muss, und die dabei offene Frage — welches Ende einer mehrzeiligen Auswahl die
Schreibmarke ist, wo AppKit über `selectedRange()` nur den kleineren Versatz
nennt — stehen in
`issues/260810-0036_o_dem-editor-fehlt-die-auskunft-ueber-die-zeile-der-schreibmarke.md`.

Bis dahin legt `cmd+d` mit dem Fokus im Editor weiterhin eine Ordnermarke auf den
Ordner des aktiven Dateifensters an. Das Kommando trägt
`Wirkungsbereich::Ueberall`, der Rumpf ist also erreicht; er kennt den zweiten
Zweig nur noch nicht.

## Was der Nutzer selbst prüfen muss

Am laufenden Bündel, mit KRK im Vordergrund:

1. Beide Sorten stehen sichtbar unterscheidbar in **einer** Leiste. Eine
   Textmarke lässt sich heute nur von Hand in `bookmarks.toml` eintragen, weil
   S38 offen ist.
2. Die Beschriftungen aller Zeilen fluchten, auch die der Geräte.
3. Eine Textmarke auf eine gelöschte Datei trägt "(fehlt)" und ist gedämpft, mit
   dem Sinnbild.
4. Textmarken überleben Beenden und Neustart, in ihrer Reihenfolge.
