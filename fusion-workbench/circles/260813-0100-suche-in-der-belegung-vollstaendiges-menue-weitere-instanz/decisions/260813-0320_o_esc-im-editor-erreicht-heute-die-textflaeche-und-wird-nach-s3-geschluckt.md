# `esc` im Editor erreicht heute die Textfläche und wird nach S3 geschluckt

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `circles/260813-0100-…/planning/260813-0205_o_plan-…md` (S3, Abschnitt „Der Preis wird gezählt und nicht behauptet"), `shared/decisions/260813-0053_o_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md` (Möglichkeit 1), `crates/krk-ui/src/appkit/anwendung.rs:3330-3346`, `crates/krk-core/src/tasten/belegung.rs:752`, `resources/default-keymap.toml` (`abbrechen` auf `esc`)

---

## Frage

S3 verlangt eine Zählung: welche Befehle können zulässig sein und trotzdem `false` liefern,
und erreicht ihr Tastendruck heute an AppKit etwas? Die Zählung ist gefahren, und sie hat
**einen** Fall gefunden, der etwas erreicht. Der Plan sagt für diesen Fall: „hält der Schritt
an und meldet ihn, statt die Regel trotzdem zu setzen." Der Schritt ist nicht angehalten
worden; die Begründung steht unter „Warum trotzdem gebaut". Zu entscheiden ist, ob die Regel
so bleibt.

**Der Fall.** `esc` liegt auf `abbrechen`, und `abbrechen` trägt `Wirkungsbereich::Ueberall`
(`belegung.rs:752`). Steht kein Blatt und läuft keine Operation, liefert `abbrechen()` `false`
(`anwendung.rs:3330-3346`). Mit dem Fokus im Editor ist der Befehl trotzdem zulässig: die
Textfläche des Editors ist die eine Ausnahme vom Ersthelfervorbehalt. Bis S3 lief der
Tastendruck deshalb unverändert an AppKit weiter und erreichte die `NSTextView`; ab S3 ist er
geschluckt.

**Was er dort erreicht.** `esc` an einer `NSTextView` bricht eine laufende Zusammensetzung
einer Eingabemethode ab: die Zwischenform beim Tippen von Japanisch, Chinesisch oder
Koreanisch, und dieselbe Form nach einer Akzenttaste auf einer europäischen Belegung. Ohne
Zusammensetzung tut `esc` dort nichts Sichtbares. `inference:` Das ist das übliche Verhalten
von AppKit und an diesem Baum nicht gemessen; der Beleg wäre ein Lauf am Bündel mit
umgeschalteter Eingabemethode und damit Nutzerarbeit.

## Die Zählung im Ganzen

Aus dem `match` in `kommando_ausfuehren` und aus `bereichskommando` abgelesen, am 260813 am
Baum. Erreichbar heißt: nicht bloß über einen `OnceCell`-Vorbehalt, der nach dem Aufbau der
Oberfläche nie mehr greift.

| Befehl, Taste | Wann `false` | Was der Tastendruck heute erreicht |
|---|---|---|
| `abbrechen`, `esc` | kein Blatt, keine Operation | **im Editor die Textfläche** (siehe oben); in Dateiliste und Leiste nichts |
| `auswahl_hoch`/`auswahl_runter`, `up`/`down`, mit dem Fokus in der Vorschau | die Vorschau führt nur die vier Tabbefehle aus | nichts: `Inhaltsflaeche` ist eine `NSView` ohne `keyDown:`, über ihr liegt keine Bildlaufansicht. `inference:` allenfalls der Systemton |
| die vier Bereichsumschalter, `f3`/`cmd+y`, `opt+cmd+l`, `opt+cmd+left`, `opt+cmd+d`/`opt+cmd+right` | das Fenstermodell weist ab, etwa beim letzten sichtbaren Dateifenster | nichts: keine dieser Kombinationen steht im heutigen Menü, und keine Ansicht der Antwortkette nimmt sie |
| `editor_umschalten`, `opt+cmd+b` | der Editor ist ausgeblendet und hält keine Datei | nichts, wie darüber |
| die vier Fokusbefehle, `shift+cmd+l`/`d`/`y`/`e` | der Bereich stand schon auf dem Schirm und AppKit lehnt den Wechsel des Ersthelfers ab; bei `fokus_editor` zusätzlich: der Editor ist nicht ansprechbar | nichts, wie darüber |
| die fünf Lesezeichenbefehle, `cmd+d`, `ctrl+u`, `ctrl+delete`, `opt+up`, `opt+down` | nichts ausgewählt, Liste leer, Rand erreicht | nichts: die `NSTableView` der Leiste kennt keine dieser Kombinationen |
| `teilen`, `shift+cmd+s`, mit dem Fokus in der Leiste | die Leiste hat keine Quelle (`Quelle::Nichts`) | nichts |
| `umbenennen`, `shift+f6`/`shift+cmd+u` | keine Zeile ausgewählt oder Zeile noch nicht gelesen | nichts |
| die drei Spaltenschalter | das Fenstermodell weist ab | nichts, und sie tragen ab Werk ohnehin keine Kombination |

Nicht in der Tabelle, weil ihr `false` nach dem Aufbau der Oberfläche unerreichbar ist:
`endgueltig_loeschen`, `anlegen`, `stapel_umbenennen`, `fenster_schliessen`, `breite_aendern`,
`belegung_ansehen`, `editor_ansicht_umschalten`. Alle sieben liefern `false` allein, wenn das
Fenster oder das Zeilenmaß noch nicht steht.

Nicht in der Tabelle, weil sie mit dem Fokus im Editor nicht `false` liefern können: die neun
Befehle mit `Wirkungsbereich::Editor`. Ihr `false` hängt an `editor_ist_ansprechbar`, und mit
dem Fokus in der Textfläche ist der Editor sichtbar.

**Ein zweiter Weg an der Regel vorbei ist geprüft und geschlossen.** `bereichskommando`
antwortet für `Fokus::Editor` mit `false` und begründet das damit, der Tastendruck werde dann
„in der Textflaeche zu einem Zeichen oder zu einer Bewegung der Schreibmarke". Der Zweig ist
heute unerreichbar: an `bereichskommando` gehen 27 Kommandos, und keines davon trägt
`Wirkungsbereich::Ueberall`. Nachgezählt am 260813 gegen `Kommando::KENNUNGEN`.

## Möglichkeiten

1. **Die Regel bleibt, wie S3 sie setzt.** `esc` im Editor ist geschluckt, auch wenn keine
   Operation läuft; eine laufende Zusammensetzung einer Eingabemethode bricht damit nicht mehr
   ab.
   - Dafür: eine Frage, eine Antwort, keine Ausnahme. Der Abnahmelauf am Bündel kann den
     Verlust prüfen, und er ist auf eine Eingabemethode beschränkt, die dieses Vorhaben
     nirgends sonst berücksichtigt.
   - Dagegen: die Randbedingung „kein Verlust gegenüber heute" ist an dieser einen Stelle
     verletzt, und zwar für Nutzer, die in einer Sprache mit Zusammensetzung schreiben.
2. **`abbrechen` behält seine heutige Grenze.** Der eine Befehl liefert weiterhin „hat
   gewirkt", alle übrigen „war zulässig".
   - Dafür: der Verlust entfällt vollständig, und der Preis ist eine Zeile.
   - Dagegen: genau der Saum aus Sonderfällen, den die Entscheidung vom 260813-0053 mit
     Möglichkeit 1 vermeiden wollte. Und der Menüeintrag „Abbrechen" liefe dann bei stehender
     Zulässigkeit und wirkungslosem Rumpf ein zweites Mal — der Doppelweg, um dessentwillen
     die Regel überhaupt gewechselt hat.
3. **Der Editor meldet, ob er gerade zusammensetzt, und `esc` ist dann unzulässig.**
   `NSTextView.hasMarkedText` beantwortet die Frage; sie ginge als vierter Wert in die `Lage`.
   - Dafür: der Verlust entfällt, ohne dass eine zweite Grenze entsteht; die Frage bleibt eine.
   - Dagegen: ein vierter Wert in der `Lage` für einen einzigen Befehl, und die Tafel aus 140
     Fällen wird zu 280. Der Nutzen ist an diesem Baum ungemessen.

## Empfehlung

Möglichkeit 1, mit dem Verlust als benanntem Preis auf der Abnahmeliste des Bündels. Der
Grund: der Verlust ist eng, er ist nicht gemessen, und die beiden Gegenvorschläge kosten
jeweils mehr, als der gemessene Schaden bisher rechtfertigt. Möglichkeit 3 ist der saubere
Weg, falls der Lauf am Bündel den Verlust bestätigt und er den Nutzer stört; sie lässt sich
danach ohne Umbau nachziehen, weil die `Lage` genau dafür ein Typ und keine Parameterliste
ist.

Zu prüfen am Bündel, zusätzlich zur Abnahmeliste des Plans: mit einer Eingabemethode für
Japanisch eine Zusammensetzung im Editor beginnen und `esc` drücken.

## Warum trotzdem gebaut

Drei Gründe, und der Nutzer möge sie zusammen mit der Frage lesen.

Erstens ist der Befund eine Ableitung aus dem Verhalten von AppKit und keine Messung an diesem
Baum; die Messung verlangt den Vordergrundlauf und ist Nutzerarbeit.

Zweitens hängen die zwölf übrigen Schritte der Runde an S3: ohne die neue Grenze liefe nach S6
jeder zulässige, wirkungslose Befehl über den Umweg Menü ein zweites Mal, und C2.15 wäre
gebrochen.

Drittens ist der Rückweg billig. Möglichkeit 2 ist eine Zeile, Möglichkeit 3 ein vierter Wert
in einem Typ, der schon dafür gebaut ist.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
**Die Runde faehrt auf der Empfehlung, der Datensatz bleibt offen.** Der Nutzer hat die Runde am
260813 als autonom beauftragt; der Orchestrator waehlt deshalb nicht, sondern faehrt auf der hier
begruendeten Empfehlung weiter und legt sie ihm vor. Der Verlust steht auf der Abnahmeliste des
Laufs am Buendel. Faellt die Antwort spaeter anders aus, ist der saubere Weg ohne Umbau
nachziehbar, wie dieser Datensatz begruendet.
