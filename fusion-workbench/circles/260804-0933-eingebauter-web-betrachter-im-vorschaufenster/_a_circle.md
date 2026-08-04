# KRK zeigt Web-Seiten in einem eigenen Betrachter

---
**Domain:** code
**Status:** anticipated
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** (none yet)
**Active session history:** (none yet)

---

## Directive

KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters und folgt dessen Halteverhalten aus C6, der Fähigkeit "Vorschaufenster mit eigenen Tabs" des aktiven Circles: die nächste Auswahl im Dateifenster ersetzt seinen Inhalt im aktiven Tab, ein anderer Tab behält ihn stehen. Der Befehl Opt+Cmd+G, der eine Adresse aus der Zwischenablage heute dem Systembrowser übergibt, öffnet sie nach dieser Runde in KRK. Bedient wird der Betrachter über die Tastatur: blättern, zurück, vor, und Sprungmarken auf jedem sichtbaren Link, mit denen der Nutzer jedem Verweis ohne Maus folgt. Er speichert keinen Verlauf, trägt kein Adressfeld als dauerhaftes Bedienelement und lädt nichts herunter. Angezeigt werden allein `http:` und `https:`, wie schon heute.

## Grounding snapshot

Vorläufig. Ein anticipated Circle trägt noch keine erhobene Grounding; dieser Abschnitt hält fest, was beim Lesen des aktiven Circles am 260804-0933 sichtbar war, und wird bei der Aktivierung ersetzt.

### Woher das Vorhaben kommt

Der Nutzer hat am 260804-0830 zwei Zwischenablage-Funktionen beauftragt, die als Fähigkeit C10 im Spec des aktiven Circles stehen, und dabei angekündigt, er wolle "bei URL zur Zeit Systembrowser, später eigener". Der Plan der Runde 1 hält unter seinen offenen Punkten fest, dass dieser eigene Browser nirgends festgehalten ist und dass die Entscheidung darüber beim Nutzer liegt, nicht beim Planner. Dieser Circle ist die Antwort auf diesen Punkt.

Vier Fragen der Klärungsrunde hat der Nutzer am 260804 beantwortet. Der Umfang ist ein Betrachter und kein Browser. Der Ort ist ein gewöhnlicher Tab des Vorschaufensters. Der eingebaute Betrachter ersetzt den Systembrowser als Ziel von Opt+Cmd+G. Zur Tastatursteuerung kommen Sprungmarken auf jedem sichtbaren Link, weil die Steuerung über die Tastatur die erste Maxime des Projekts ist und der Nutzer den Mehraufwand dafür ausdrücklich in Kauf nimmt.

### Was der aktive Circle schon gebaut hat und dieser erbt

Vier Bauteile aus der Runde 1 trägt dieser Circle weiter, statt daneben ein zweites zu stellen.

Die Auswertung der Zwischenablage in `crates/krk-core/src/zwischenablage.rs` deutet eine Zeichenkette als lokalen Pfad, als Web-Adresse oder als nichts Verwertbares. Sie liest seit dem Nutzerentscheid vom 260804 sowohl den Text als auch den Dateiverweis des Finders. Die Web-Adresse erreicht den Betrachter über genau diese eine Auswertung; eine zweite entsteht nicht.

Das Vorschaufenster aus C6 hält beliebig viele Tabs mit denselben Befehlen zum Öffnen, Schließen und Wechseln wie die Dateifenster. Der Betrachter ist eine dritte Quelle für den aktiven Tab, neben der Auswahl im Dateifenster und der Zwischenablage aus C10, und keine vierte Fläche.

Die Statuszeile am Fuß des Dateifensters, seit dem 260804-0830 zugesagt, trägt die Meldungen, die KRK dem Nutzer zeigt. Was der Betrachter zu melden hat, etwa eine nicht erreichbare Adresse, gehört in diese Zeile und nicht in ein eigenes Meldewesen.

Die Belegung liegt als Datentabelle in `resources/default-keymap.toml`, jede Kombination ist frei änderbar, und `opt+cmd+g` steht dort bereits. Die Umstellung des Ziels ändert nichts an der Belegung. Neue Befehle des Betrachters, etwa das Ein- und Ausschalten der Sprungmarken, bekommen ihre Einträge in derselben Tabelle.

Technisch steht KRK seit dem 260802-1150 fest: Rust mit AppKit über `objc2`, Auslieferung außerhalb der App-Sandbox, Mindest-Zielsystem macOS 15 mit Unterstützung bis macOS 26.

### Wie die beiden Spannungen in den Antworten aufgelöst sind

**Kein Adressfeld, aber Adresseingabe: zur Hälfte aufgelöst, zur Hälfte offen.**

Aufgelöst ist das Bedienelement. Ein dauerhaft sichtbares Adressfeld gibt es nicht, und beide Antworten sagen dasselbe. KRK trägt an keiner Stelle eine dauerhafte Eingabezeile: die Pfadeingabe aus C2 ist ein Blatt am Fenster, das ein Tastenbefehl öffnet und das nach der Eingabe wieder verschwindet.

Offen bleibt die Fähigkeit. Die Unterscheidung zwischen Bedienelement und Fähigkeit trägt hier nur den halben Weg, denn ein Blatt statt einer Zeile nimmt das Feld weg und nicht das Eintippen. Wer eine Adresse eintippen kann, erreicht jede Adresse und nicht nur die, die KRK ihm übergeben hat. Genau daran hängt der Unterschied zwischen einem Betrachter und einem Browser, und Antwort 1 hat den Betrachter gewählt.

Der Gegeneinwand steht daneben und wird nicht unterschlagen. Mit den Sprungmarken verlässt der Betrachter die übergebene Adresse ohnehin, sobald der Nutzer einem Verweis folgt. Die Zusage "zeigt genau die Adresse, die KRK ihm übergibt" beschreibt danach den Anfang einer Sitzung und nicht ihren Verlauf. Wer so liest, für den ist die Adresseingabe ein zweiter Anfang und keine andere Art von Werkzeug.

`speculation:` Die Aufzählung "Blättern, Zurück, Vor und Adresseingabe" in Antwort 4 stammt vermutlich aus dem Optionstext der Frage und beschrieb dort die übliche Tastenausstattung eines Browsers, bevor Antwort 1 den Umfang auf den Betrachter zog. Belegen lässt sich das nicht: der Fragetext dieser Runde steht nirgends auf der Platte.

Der Widerspruch ist damit echt und keine Formulierungsfrage. Er steht unten als erste offene Frage. Die Sprungmarken aus Antwort 4 sind davon nicht berührt und gelten.

**Kein Verlauf, aber Zurück und Vor: aufgelöst, ohne Rest.**

Das Wort Verlauf trägt zwei Sachen, die sich sauber trennen lassen.

Der gespeicherte Verlauf ist eine Liste besuchter Seiten, die eine Sitzung überdauert und die der Nutzer ansehen kann. Antwort 1 schließt ihn aus, zusammen mit dem Adressfeld und dem Herunterladen. Alle drei sind Bedienelemente eines vollen Browsers.

Der Navigationsstapel ist die Folge der Seiten, die der Nutzer in einem Tab gerade betrachtet hat. Er liegt im Arbeitsspeicher, erscheint nirgends als Liste und wird nicht geschrieben. Zurück und Vor arbeiten allein auf ihm: Zurück geht einen Schritt zurück, Vor nimmt den Schritt wieder auf, den Zurück eben verlassen hat. Keines von beiden braucht etwas, was eine Sitzung überdauert.

KRK hält also einen Navigationsstapel je Tab und keinen gespeicherten Verlauf. Damit gelten Zurück und Vor beide, und Antwort 1 bleibt eingehalten.

Daraus folgt eine Eigenschaft, die der Nutzer bemerken wird, und sie folgt aus dem Halteverhalten aus C6 und nicht aus einer neuen Regel. Ersetzt die nächste Auswahl im Dateifenster den Inhalt des aktiven Tabs, stirbt der Navigationsstapel dieses Tabs mit. Zurück kommt danach nicht mehr auf die Web-Seite. Wer sie behalten will, wechselt vorher den Tab, wo sie samt ihrem Stapel stehen bleibt. Denselben Preis trägt heute schon die Vorschau der Zwischenablage aus C10, und der Spec nennt ihn dort als den Punkt, den der Nutzer am ehesten anders sieht.

### Die Grenze aus C9 bleibt, wo sie ist

Zum Systembrowser gehen heute allein `http:` und `https:`; jedes andere Schema meldet die Statuszeile als nicht verwertbar. Der Grund ist C9, "Nur lokale Laufwerke": gäbe KRK ein `smb:` oder `ftp:` an das System weiter, baute es über einen Umweg die Serververbindung auf, die C9 ausschließt.

Diese Grenze bleibt bestehen. Ein eingebauter Betrachter, der dieselben zwei Schemata zeigt, verschiebt sie nicht: er zeigt Web-Inhalt und baut keine Verbindung über ein Dateiprotokoll auf. Festgelegt vom Nutzer am 260804, weil die Frage die Circle-Grenze berührt.

### Offene Fragen

Ein anticipated Circle darf offene Fragen tragen. Die drei unten sind Eingabe für die Klärungsrunde bei der Aktivierung und je einzeln so gestellt, dass sie den Zuschnitt bestimmen.

**1. Welche Quellen dürfen die Adresse setzen?** Die Frage entscheidet, ob KRK einen Betrachter oder einen Browser bekommt, und sie löst die erste Spannung oben auf.

- *Nur KRK und die angezeigte Seite.* Die Adresse kommt aus der Zwischenablage über Opt+Cmd+G, danach folgt der Nutzer den Verweisen der Seite über die Sprungmarken. Ein anderer Weg ins Web besteht nicht. Das ist die engste Lesart von Antwort 1.
- *Zusätzlich eine Adresseingabe.* Ein Tastenbefehl öffnet ein Blatt am Fenster, wie die Pfadeingabe aus C2, der Nutzer tippt eine Adresse und landet dort. Kein dauerhaftes Feld, aber jede Adresse erreichbar.
- *Zusätzlich gespeicherte Web-Adressen.* Die Lesezeichenleiste aus C5 hält heute Ordner. Sie könnte auch Web-Adressen halten, womit KRK einen zweiten dauerhaften Zugang ins Web bekäme.

Die drei Möglichkeiten bauen aufeinander auf. Die dritte setzt die zweite nicht voraus, wohl aber dieselbe Grundhaltung: der Betrachter ist ein eigenständiger Zugang und keine Fortsetzung einer Übergabe aus KRK.

**2. Zeigt der Betrachter auch lokale HTML-Dateien?** C6 kennt heute drei Ausgänge: Text erscheint als Text, ein Bild als Bild, alles andere als Metadaten. Eine `.html`-Datei im Dateifenster fällt damit unter Text und erscheint als Quelltext. Ob der Betrachter sie stattdessen gerendert zeigt, ändert die Dreiteilung aus C6 und damit eine abgenommene Fähigkeit des aktiven Circles.

**3. Bekommt der Betrachter eine eigene Zeitzusage?** Die Maxime "superschnell" trägt in C8 zehn gemessene Zusagen, und keine davon misst Web-Inhalt. Eine Seite lädt über das Netz, und die Ladezeit liegt nicht bei KRK. Zusagen ließen sich über das setzen, was KRK selbst verantwortet: wie schnell der Tab erscheint, und wie schnell die Sprungmarken nach dem Laden stehen. Ob eine elfte Zahl entsteht und woran sie gemessen wird, gehört in den Aktivierungs-Spec.

### Was dieser Circle nicht festlegt

Womit KRK Web-Inhalt darstellt, ist offen und gehört in eine eigene Untersuchung vor dem Plan. Der Circle legt kein Mittel fest, weder eine Systemschnittstelle noch eine Kiste.

`inference:` Die offene Entscheidung `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` dürfte diesen Circle binden. Sie fragt, wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt, und bindet die Runde 1 nicht. Ein Betrachter spricht eine Systemschnittstelle an, deren Umfang sich zwischen macOS 15 und macOS 26 unterscheidet; geprüft ist das nicht.

## Dependencies

Dieser Circle hängt an `260802-0842-krk-mac-dateimanager-editor-git`, dem aktiven Circle. Er **erweitert dessen Grenze** und ist keine spätere Runde davon. Der Unterschied ist nicht formal: eine spätere Runde setzt eine Zusage um, die der Circle-Datensatz schon trägt, während dieser Circle einen Punkt hereinholt, den derselbe Datensatz ausdrücklich ausschließt. Ein Ausschluss lässt sich nicht durch eine Runde aufheben, die sich auf ihn beruft.

Drei Stellen im aktiven Circle binden dieses Vorhaben:

- `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Fähigkeit **C10, "Die Zwischenablage als Quelle"**. Sie sagt zu: "Enthält die Zwischenablage eine Web-Adresse, übergibt KRK sie dem Systembrowser. KRK zeigt selbst keinen Web-Inhalt an." Dieser Circle ersetzt das Ziel dieser Übergabe. Das Abnahmekriterium aus C10 wird damit überholt, sobald der Betrachter steht, und der Aktivierungs-Spec muss sagen, wie es fortgeschrieben wird.
- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md`. Der Nutzerentscheid vom 260804 legt fest, dass die Auswertung Text und Dateiverweis liest. Die Web-Adresse, die der Betrachter anzeigt, kommt aus genau dieser Auswertung. Dieser Circle erbt sie und baut keine zweite.
- `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`, Abschnitt `## Ausdrücklich außerhalb dieses Circles`, erster Punkt: "Integrierter Browser zum Navigieren von Websites." Derselbe Ausschluss steht in `CLAUDE.md` und in `## Nicht in dieser Runde` des Specs. Dieser Circle hebt ihn für den Betrachter auf, ohne den Datensatz des aktiven Circles zu ändern; die Aufhebung wirkt erst mit seiner eigenen Aktivierung.

Zwei weitere Bindungen sind zeitlich und nicht inhaltlich: die Schritte S13 und S19 der Runde 1 bauen die Zwischenablage-Auswertung und das Vorschaufenster, auf denen dieses Vorhaben aufsetzt. Es kann erst geplant werden, wenn beide stehen.

Der Plan der Runde 1 hält den Anlass dieses Circles selbst fest, unter seinen offenen Punkten: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, "Der eigene Browser, den der Nutzer für später angekündigt hat, ist nirgends festgehalten."

## Turn log

(noch keiner)
