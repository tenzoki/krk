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

`inference:` Die offene Entscheidung `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` dürfte diesen Circle binden. Sie fragt, wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt, und bindet die Runde 1 nicht. Ein Betrachter spricht eine Systemschnittstelle an, deren Umfang sich zwischen macOS 15 und macOS 26 unterscheidet; geprüft ist das nicht.

## Dependencies

Dieser Circle hängt an `260802-0842-krk-mac-dateimanager-editor-git`, dem aktiven Circle. Er **erweitert dessen Grenze** und ist keine spätere Runde davon. Der Unterschied ist nicht formal: eine spätere Runde setzt eine Zusage um, die der Circle-Datensatz schon trägt, während dieser Circle einen Punkt hereinholt, den derselbe Datensatz ausdrücklich ausschließt. Ein Ausschluss lässt sich nicht durch eine Runde aufheben, die sich auf ihn beruft.

Drei Stellen im aktiven Circle binden dieses Vorhaben:

- `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`, Fähigkeit **C10, "Die Zwischenablage als Quelle"**. Sie sagt zu: "Enthält die Zwischenablage eine Web-Adresse, übergibt KRK sie dem Systembrowser. KRK zeigt selbst keinen Web-Inhalt an." Dieser Circle ersetzt das Ziel dieser Übergabe. Das Abnahmekriterium aus C10 wird damit überholt, sobald der Betrachter steht, und der Aktivierungs-Spec muss sagen, wie es fortgeschrieben wird.
- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-0830_*_was-die-zwischenablage-auswertung-liest.md`. Der Nutzerentscheid vom 260804 legt fest, dass die Auswertung Text und Dateiverweis liest. Die Web-Adresse, die der Betrachter anzeigt, kommt aus genau dieser Auswertung. Dieser Circle erbt sie und baut keine zweite.
- `circles/260802-0842-krk-mac-dateimanager-editor-git/_*_circle.md`, Abschnitt `## Ausdrücklich außerhalb dieses Circles`, erster Punkt: "Integrierter Browser zum Navigieren von Websites." Derselbe Ausschluss steht in `CLAUDE.md` und in `## Nicht in dieser Runde` des Specs. Dieser Circle hebt ihn für den Betrachter auf, ohne den Datensatz des aktiven Circles zu ändern; die Aufhebung wirkt erst mit seiner eigenen Aktivierung.

Zwei weitere Bindungen sind zeitlich und nicht inhaltlich: die Schritte S13 und S19 der Runde 1 bauen die Zwischenablage-Auswertung und das Vorschaufenster, auf denen dieses Vorhaben aufsetzt. Es kann erst geplant werden, wenn beide stehen.

Der Plan der Runde 1 hält den Anlass dieses Circles selbst fest, unter seinen offenen Punkten: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_*_plan-navigator-geruest-runde-1.md`, "Der eigene Browser, den der Nutzer für später angekündigt hat, ist nirgends festgehalten."

## Turn log

(noch keiner)

## Parent grounding stale

**Festgestellt am:** 260807-1042
**Playmaker-Lauf:** 260807-1042-playmaker-orchestrator-phase4
**Beschränkt abgeschlossenes Kind:** `260802-0842-krk-mac-dateimanager-editor-git`, geschlossen am 260807-1035

Der Circle, auf dem dieser hier aufsetzt, trägt seit dem 260807-1035 den beschränkten Abschluss (`_b_`). Seine Beschränkung berührt die Grundlage dieses Circles an zwei Stellen. Beide gehören in die Klärungsrunde bei der Aktivierung, keine hält die Aktivierung auf.

### 1. Die dritte offene Frage steht auf einem gealterten Messstand

Der Abschnitt `## Grounding snapshot` schreibt unter `### Offene Fragen`:

> **3. Bekommt der Betrachter eine eigene Zeitzusage?** Die Maxime "superschnell" trägt in C8 zehn gemessene Zusagen, und keine davon misst Web-Inhalt. [...] Zusagen ließen sich über das setzen, was KRK selbst verantwortet: wie schnell der Tab erscheint, und wie schnell die Sprungmarken nach dem Laden stehen.

Die Frage leitet eine mögliche elfte Zusage aus den zehn bestehenden ab. Genau deren Belegstand ist der Grund der Beschränkung. Die `## Closure note` des Kindes hält fest, dass sieben der zehn Zusagen unverändert auf der Abnahmereihe `messungen/260805-2207-MacBookPro15-1-abnahme.txt` vom 260805-2207 stehen und dass drei spätere Commits Wege berührt haben, die eben diese Zusagen messen.

Zwei der sieben sind für einen Betrachter im Vorschaufenster die naheliegenden Bezugsgrößen, und beide gehören zum ungemessenen Teil:

- **L5**, Wechsel des Tabs oder des aktiven Dateifensters bis zur bedienbaren ersten Bildschirmseite, zugesagt mit 50 ms. Der Satz "wie schnell der Tab erscheint" aus Frage 3 misst denselben Vorgang in einem Vorschau-Tab.
- **L7**, Vorschau einer Textdatei bis 1 MB sichtbar, zugesagt mit 100 ms. Der Betrachter ist nach dem Grounding dieses Circles die dritte Quelle für den aktiven Vorschau-Tab und teilt sich diesen Weg mit den beiden vorhandenen.

Beide Zahlen stehen in C8 des Specs `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`.

Die drei alternden Commits treffen diese Wege nachweislich. `9a47c4a` ändert `crates/krk-ui/src/kommandos/fokus.rs` und `crates/krk-ui/src/fenstermodell.rs` und baut den dritten Fokusbefehl in das Vorschaufenster; `5d7e299` ändert `crates/krk-ui/src/tabs.rs`, wo die Tabliste und ihre Auswahl liegen. Der Betrachter setzt auf beiden Bauteilen auf. Eine elfte Zusage, die aus L5 oder L7 abgeleitet wird, erbt damit deren offenen Beleg.

**Was daraus folgt.** Der Aktivierungs-Spec sollte Frage 3 erst nach dem Abnahmelauf beantworten, den die `## Closure note` der Runde 1 als Nachholarbeit benennt (`make fixture`, danach `make alle RUNDEN=5` aus einem Terminalfenster im Vordergrund, mit KRK vorn). Ohne diesen Lauf ist der Ausgangswert unbekannt, gegen den eine neue Zusage gesetzt würde.

### 2. Der Artefakt der Beschränkung ist an diesen Circle adressiert

Die `## Closure note` benennt als Gelerntes: "Eine Messreihe altert an jedem Commit, der einen gemessenen Pfad berührt, und sie sagt es nicht selbst." Sie schließt mit dem Satz: "Eine spätere Runde, die Zeitzusagen führt, braucht dafür eine Regel statt einer Nachfrage."

Ob dieser Circle eine Zeitzusage führt, entscheidet seine eigene Frage 3. Fällt die Antwort auf ja, ist der Artefakt der Beschränkung eine bindende Eingabe: die Runde braucht dann eine Regel, die das Altern der Messung anzeigt, und nicht erst einen Abgleich am Ende, der danach fragt.

### 3. Drei Pfadzitate sind mit der Umbenennung ins Leere gelaufen

Der Abschnitt `## Dependencies` zitiert drei Dateien unter ihrem damaligen Zustandsmarker. Alle drei haben ihn am 260807-1035 verloren, und die zitierten Pfade existieren nicht mehr:

| Zeile | zitiert | ist |
|---|---|---|
| 100 | `planning/260802-1036_*_spec-navigator-geruest.md` | `_c_` |
| 102 | `260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` | `_b_circle.md` |
| 106 | `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` | `_c_` |

Der Abgleich `history/260807-1022-reconciliation.md` und der daraus entstandene Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` decken diese drei Stellen nicht ab: der Abgleich lief um 260807-1022 und damit vor der Umbenennung, und sein Suchmuster `YYMMDD-HHMM_x_name.md` erfasst die Form `_t_circle.md` ohnehin nicht. Zeile 101 desselben Abschnitts steht dort und ist die einzige Stelle dieses Circles, die der Defekt führt.

Der Playmaker berichtigt keine Zitate. Die Sternform `_*_`, die Plan und Spec seit dem 260805-0000 führen, ist der im Defekt beschriebene Weg.

**Erledigt am 260810-1740.** Die drei Zeilen 100, 102 und 106 tragen die Sternform, ebenso die übrigen Zitate dieses Datensatzes. Die Tabelle darüber bleibt im damaligen Wortlaut stehen: sie führt die falschen Zeichenketten als Befund, und eine Sternform darin würde den Befund unlesbar machen.

## Activation proposal

**Vorgeschlagen am:** 260807-1042
**Playmaker-Lauf:** 260807-1042-playmaker-orchestrator-phase4
**Domain-Gewichtung:** code

Dieser Circle ist der empfohlene nächste Kandidat, und zwar ohne Vergleichswert: er ist nach dem Abschluss der Runde 1 der einzige nicht abgeschlossene Circle im Portfolio. Eine Rangfolge mit einem Element trägt keine Information über relative Reife, deshalb stützt sich die Empfehlung auf die absoluten Signale.

**Die Voraussetzungen sind erfüllt, am Code geprüft.** Der Abschnitt `## Dependencies` nennt zwei zeitliche Bindungen, die Schritte S13 und S19 der Runde 1, und sagt: "Es kann erst geplant werden, wenn beide stehen." Beide stehen. Der Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_*_plan-navigator-geruest-runde-1.md` trägt `**Status:** Complete` bei 38 von 38 Schritten, und der Abgleich vom 260807-1022 hat die Schritte am Code belegt statt am Marker. Die vier Bauteile, die dieser Circle laut seinem Grounding erbt, liegen auf der Platte: die Auswertung der Zwischenablage in `crates/krk-core/src/zwischenablage.rs`, das Vorschaufenster in `crates/krk-ui/src/appkit/vorschau.rs` mit der Tableiste in `crates/krk-ui/src/appkit/tableiste.rs`, die Statuszeile in `crates/krk-ui/src/appkit/statuszeile.rs` und der Befehl `zwischenablage_springen` auf `opt+cmd+g` in `resources/default-keymap.toml:436`.

**Ein offener Entscheidungsdatensatz bindet diesen Circle**, und das ist für die Gewichtung `code` ein guter Wert, weil sie Circles mit wenigen unbeantworteten Fragen bevorzugt. Der Grounding-Abschnitt nennt `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` und ordnet die Bindung selbst als `inference:` ein: ein Betrachter spricht eine Systemschnittstelle an, deren Umfang sich zwischen macOS 15 und macOS 26 unterscheidet, und geprüft ist das nicht. Die drei projektweit offenen Datensätze zu Git, Editor-Formatansicht und Code-SDK binden diesen Circle nicht.

**Was gegen eine sofortige Aktivierung spricht.** Die einzige Abhängigkeit ist beschränkt und nicht kohärent geschlossen. Nach der Rangheuristik zählt allein `_c_` als erfüllte Vorbedingung, und `_b_` löst ein Kennzeichen aus. Inhaltlich trägt dieses Kennzeichen hier: die Beschränkung ist der offene Beleg der Zeitzusagen, und der Abschnitt `## Parent grounding stale` oben zeigt, dass er über die dritte offene Frage in diesen Circle hineinreicht. Der Abnahmelauf, den die Runde 1 als Nachholarbeit hinterlässt, verlangt KRK im Vordergrund und damit den Nutzer. Ihn vor der Aktivierung zu fahren kostet wenig und nimmt der Klärungsrunde eine Unbekannte ab.

Die drei offenen Fragen des Grounding-Abschnitts bleiben die erste Arbeit nach dem Übergang auf aktiv. Der Shaper im portfolio-activation-Modus klärt sie mit dem Nutzer, bevor ein Plan entsteht.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von `_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über `/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260812-0816
**Playmaker-Lauf:** 260812-0816-playmaker-direct-dispatch
**Beschränkt abgeschlossenes Kind:** `260811-1304-statusleiste-mit-bereichsschaltern`,
geschlossen am 260812-0820

Die Runde 5 hat die eine Breitenregel des Programms neu gefasst, und die Mindestbreite der
Vorschau ist damit von einer Zahl zu einem Hebel geworden. Dieser Circle lebt in einem Tab des
Vorschaufensters, und sein Bedarf an Breite ist die eine Berührung, die der Datensatz der Runde 5
zwischen beiden Vorhaben benennt. Die Berührung ist mit dem Abschluss größer geworden. Keiner der
vier Punkte unten hält die Aktivierung auf; alle vier gehören in die Klärungsrunde.

**Zur Auslösebedingung, offen benannt.** Die Regel verlangt, dass der Abschnitt
`## Grounding snapshot` dieses Datensatzes den Verzeichnisnamen des abgeschlossenen Kindes oder
den in seiner `## Closure note` genannten Artefakt zitiert. Er zitiert weder das eine noch das
andere; die Wörter Statusleiste, Bereichsleiste und Mindestbreite kommen darin nicht vor. Die
Kante läuft in die andere Richtung: der Abschnitt `## Dependencies` des Datensatzes der Runde 5
nennt diesen Circle beim Namen, stellt fest, er binde jene Runde nicht, und benennt die
Berührung ausdrücklich mit der Zahl 160. Der Vermerk steht deshalb hier, obwohl die wörtliche
Bedingung nicht greift. Wer anders entscheidet, sieht an dieser Stelle, worauf.

### 1. Die 160 Punkte der Vorschau tragen seit der Runde 5 zwei Entscheidungen statt keiner

Der Datensatz der Runde 5 sagte über die Berührung: „ein gerenderter Web-Inhalt braucht plausibel
mehr als die 160 Punkte Mindestbreite, die die Vorschau heute trägt. Wer jenen Circle aktiviert,
prüft die Zahl." Die Zahl steht unverändert bei 160 (`crates/krk-ui/src/fenstermodell.rs:213`).
Was sich geändert hat, ist, woran sie hängt. Am Baum gelesen am 260812-0816:

**Sie entscheidet, ob die Vorschau überhaupt aufgeht.** `Fenstermodell::umschalten`
(`crates/krk-ui/src/fenstermodell.rs:639`) weist seit der Runde 5 jeden Einschaltbefehl ab, dessen
Bereichssatz nicht mehr in die Fensterzeile passt; die Rechnung steht in `mindestbreiten_passen`
(`:685`) und summiert die Mindestbreiten der Bereiche, die nach dem Befehl stünden. Die Abweisung
ist stumm, das ist die Form aus C7 der Runde 1, und der Rückgabewert trägt `#[must_use]`. Ein
Schalter, dessen Bereich nicht hineinpasst, springt zurück, ohne dem Nutzer zu sagen, warum.

**Sie entscheidet, wer beim Schrumpfen weicht.** `bereichsbreiten` (`:1044`) verteilt Anteile
statt Punktzahlen. Wessen Anteil unter sein Mindestmaß fiele, bekommt genau das Mindestmaß und
scheidet aus der Verteilung aus (`:1096`); die übrigen teilen den kleineren Rest weiter. Je höher
die Zahl der Vorschau, desto häufiger nimmt die Vorschau ihr Mindestmaß und desto häufiger geben
die anderen nach. Vor der Runde 5 entschied darüber die Reihenfolge in `Bereich::ALLE` und nicht
die Mindestbreite.

**Damit gibt es eine Obergrenze, und sie ist nah.** `MINDESTGROESSE`
(`crates/krk-ui/src/appkit/fenster.rs:134`) hält die Fensterbreite bei 780 Punkten. Stehen
Lesezeichenleiste, beide Dateifenster und die Vorschau zugleich, verlangen die drei ersten
zusammen 600 Punkte, es bleiben 180 abzüglich dreier Trennlinien. Die Breite einer Trennlinie
liest KRK zur Laufzeit von AppKit (`dividerThickness()`, `crates/krk-ui/src/appkit/aufteilung.rs:616`)
und steht nirgends im Baum; bei einer dünnen Linie von einem Punkt liegt die Obergrenze bei rund
177. `inference:` gerechnet und nicht am laufenden Bündel gemessen. Über dieser Grenze geht die
Vorschau am schmalsten zulässigen Fenster gar nicht mehr auf, gleich welchen Inhalt ihr Tab trägt.
Zwischen der heutigen 160 und dieser Grenze liegen rund 17 Punkte.

**Und die Zahl gehört dem Bereich, nicht dem Tab.** `Bereich::mindestbreite` ist eine
`const fn` über die fünf Bereiche und kennt keinen Tabinhalt (`:209`). Der Betrachter lebt nach
der Directive dieses Circles in einem gewöhnlichen Tab des Vorschaufensters. Wer die Zahl für ihn
anhebt, hebt sie für jeden Vorschau-Tab an, auch für die Metadatenansicht und die Bildvorschau,
und verschiebt damit für alle Tabs, ab welcher Fensterbreite die Vorschau aufgeht. Eine
Mindestbreite je Tabinhalt wäre ein neuer Schnitt an einer Stelle, an der die Runde 5 gerade eine
zweite Rechenvorschrift ausgeschlossen hat.

**Die Ausweichbewegung ist bereits einmal abgelehnt worden.** `MINDESTGROESSE` in der Breite von
780 auf 940 zu heben stand am 260812-0430 als Möglichkeit 3 zur Wahl und ist verworfen worden,
mit zwei benannten Gründen: sie nähme dem Nutzer die Fensterbreiten zwischen 780 und 940, und sie
machte die Abweisung am Schalter wieder zu einer Vorsichtsmaßnahme
(`decisions/260812-0415_*_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`
im Circle der Runde 5). Wer für den Betrachter mehr Breite will, argumentiert also gegen eine
Antwort, die schon steht, und nicht in eine offene Frage hinein.

### 2. Eine Nutzerfestlegung vom 260808 ist von einem Agenten überstimmt worden

Das ist der schwerste Punkt des Bounded-Closure-Artefakts der Runde 5, und er trifft diesen
Circle nicht dort, wo man ihn zuerst sucht.

Die Festlegung lautete: die Lesezeichenleiste weicht dem Editor nicht. Sie stand nirgends als
Datensatz, sondern allein im Dokumentationskommentar an `bereichsbreiten`, und sie ist unter der
Anteilsregel ersatzlos gefallen
(`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_*_was-heisst-proportional-zur-letzten-aufteilung.md`).
Der Orchestrator hat sie in der Klärungsrunde autonom fallen lassen, gedeckt durch die Weisung
„mache autonom", und mit zwei Gründen begründet. Den zweiten hat der Abgleich am 260812-0815
widerlegt und der Datensatz ihn zurückgenommen: die Frage „wer weicht, wenn es eng wird" löst
sich nicht auf, sie wird nur anders beantwortet. Tragfähig bleibt der erste Grund allein, und die
Entscheidung ist als Überstimmung zu lesen und nicht als Folgerung. Der Nutzer kann sie umstoßen;
es kostet `bereichsbreiten` samt Proben ein zweites Mal.

**Dieser Circle baut nicht auf der Festlegung vom 260808 auf, sondern auf dem Mechanismus, der
sie ersetzt hat.** Sein Datensatz nennt weder die Vorrangordnung noch die Breitenregel; geprüft
durch Suche im ganzen Datensatz. Genau darin liegt die Bindung: die Mindestbreite ist erst
dadurch zum Hebel geworden, dass die Vorrangordnung gefallen ist, und Punkt 1 oben steht
vollständig auf diesem Mechanismus. Stößt der Nutzer die Überstimmung um, wechselt der Hebel
zurück auf die Reihenfolge in `Bereich::ALLE`, und die Rechnung aus Punkt 1 gilt nicht mehr in
dieser Form. Wer diesen Circle aktiviert, klärt deshalb zuerst, ob die Anteilsregel steht.

### 3. Was sich nicht bewegt hat, damit niemand danach sucht

Zwei Annahmen des Abschnitts `## Grounding snapshot` sind geprüft und halten.

C1 der Runde 1 ist unberührt. Die neue Fläche am Fensterfuß heißt `Bereichsleiste`, trägt
ausschließlich Schalter und keine Meldung; die beiden Statuszeilen an den Füßen der Dateifenster
stehen mit allen fünf Rängen
(`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_*_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md`).
Die Zusage dieses Circles, eine nicht erreichbare Adresse gehöre in die Statuszeile und nicht in
ein eigenes Meldewesen, gilt unverändert.

Der gegenseitige Ausschluss von Vorschau und Editor aus C1 der Editor-Runde steht ebenfalls
unverändert; die Runde 5 hat ihn geerbt statt ihn anzufassen. Das Halteverhalten der
Vorschau-Tabs aus C6, auf dem die Directive dieses Circles aufsetzt, hat die Runde 5 nicht
berührt.

### 4. Die Messreihe altert weiter, und L9 kommt hinzu

Der Abschnitt `## Parent grounding stale` vom 260807-1042 hält fest, dass die dritte offene Frage
dieses Circles auf einem gealterten Messstand steht: L5 und L7 sind die naheliegenden
Bezugsgrößen für eine eigene Zeitzusage des Betrachters, und beide gehören zum ungemessenen Teil.
Der Befund gilt weiter und ist um eine Zusage reicher geworden. Die `Bereichsleiste` nimmt der
Fensterzeile 18 Punkte Höhe (`statuszeile::HOEHE`, `crates/krk-ui/src/appkit/statuszeile.rs:68`),
und die `## Closure note` der Runde 5 benennt L9 aus C8 ausdrücklich als nachzumessen, ohne eine
neue Zahl zu setzen. Fällt die Antwort auf die dritte Frage dieses Circles auf ja, ist der
Artefakt der Runde 1 eine bindende Eingabe: die Runde braucht dann eine Regel, die das Altern der
Messung anzeigt, und nicht erst einen Abgleich am Ende, der danach fragt.

## Activation proposal

**Vorgeschlagen am:** 260812-0816
**Playmaker-Lauf:** 260812-0816-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Klärungsrunde und einer Untersuchung des
Darstellungsmittels, nicht davor

Dieser Circle ist der empfohlene nächste Kandidat, und zwar ohne Vergleichswert: mit dem
Abschluss der Runde 5 am 260812-0820 ist er der einzige nicht abgeschlossene Circle im
Portfolio. Eine Rangfolge mit einem Element trägt keine Auskunft über relative Reife. Der
Vorschlag stützt sich deshalb auf die absoluten Signale, und drei davon sind seit dem Vorschlag
vom 260807-1042 neu.

**Die geerbten Bauteile stehen unverändert und sind um eines reicher geworden.** Die vier
Bauteile, die der Abschnitt `## Grounding snapshot` nennt, liegen auf der Platte, geprüft im
Vorschlag vom 260807-1042 und seither nicht angefasst: die Auswertung der Zwischenablage, das
Vorschaufenster mit seiner Tableiste, die Statuszeile und der Befehl `zwischenablage_springen`
auf `opt+cmd+g`. Dazu kommt die `Bereichsleiste` aus der Runde 5, die der Vorschau einen
Schalter am Fensterfuß gibt. Für den Betrachter ist das eine Bequemlichkeit und keine Bindung.

**Der Zuschnitt ist unverändert und bleibt das stärkste Gegenargument.** Der Datensatz hält
selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und „in eine eigene
Untersuchung vor dem Plan" gehört. Daneben steht die ungemessene Verfügbarkeitsfrage für
macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
die dieser Datensatz selbst als `inference:` einordnet, und die projektweit offene Frage, ob die
Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`). Eine
Untersuchung vor dem Plan ist teurer als eine Klärungsrunde, und dieser Circle braucht beides.

**Die Klärungsrunde trägt jetzt eine vierte Frage.** Zu den drei offenen Fragen des Abschnitts
`## Grounding snapshot` kommt die Mindestbreite der Vorschau. Der Abschnitt
`## Parent grounding stale` oben schlüsselt auf, warum die Frage nicht mehr lautet „reichen 160
Punkte für gerenderten Web-Inhalt", sondern: welche Zahl trägt die Vorschau, wenn dieselbe Zahl
darüber entscheidet, ab welcher Fensterbreite die Vorschau überhaupt aufgeht und wer beim
Schrumpfen weicht, und wenn sie für jeden Vorschau-Tab gilt und nicht nur für den des
Betrachters. Die Frage gehört vor die Untersuchung des Darstellungsmittels, weil ihre Antwort
mitbestimmt, wie viel Fläche das Mittel voraussetzen darf.

**Zur Abhängigkeitslage, die in diesem Projekt nichts mehr unterscheidet.** Die einzige
Circle-Abhängigkeit dieses Datensatzes, die Runde 1, ist beschränkt abgeschlossen (`_b_`) und
nicht kohärent (`_c_`), also trägt er nach der Rangheuristik das Kennzeichen der unerfüllten
Vorbedingung. Alle fünf gefahrenen Runden tragen `_b_`, und alle fünf aus demselben Grund: der
Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Das Kennzeichen steht damit an jedem denkbaren Kandidaten und ist keine Auskunft über diesen. Für
diesen Circle bindet es inhaltlich dennoch, und darin unterscheidet er sich von der Runde 5: die
Beschränkung der Runde 1 ist der offene Beleg der Zeitzusagen, und der reicht über die dritte
offene Frage in diesen Circle hinein.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260812-2307
**Playmaker-Lauf:** 260812-2307-playmaker-direct-dispatch
**Beschränkt abgeschlossenes Kind:** `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`
(Runde 6), geschlossen am 260812

Die Runde 6 hat die Vorschau angefasst und dabei zwei Fragen entschieden, die diesem Circle
gehören. Beide Antworten fallen zu seinen Gunsten aus, und das ist die Auskunft dieses Vermerks:
sein Zuschnitt ist erhalten geblieben, statt beschnitten zu werden. Ein dritter Punkt geht in die
andere Richtung. Keiner der vier Punkte hält die Aktivierung auf; alle vier gehören in die
Klärungsrunde.

**Zur Auslösebedingung, offen benannt.** Die Regel verlangt, dass der Abschnitt
`## Grounding snapshot` dieses Datensatzes den Verzeichnisnamen des abgeschlossenen Kindes oder
den in seiner `## Closure note` genannten Artefakt zitiert. Er zitiert weder das eine noch das
andere; die Runde 6 gab es beim Anlegen dieses Circles noch nicht. Die Kante läuft in die andere
Richtung: der Abschnitt `## Dependencies` der Runde 6 nennt diesen Circle beim Namen, führt zwei
gerichtete Kanten hierher und sagt ausdrücklich, jener Circle habe heute keine Gegenkante. Der
Vermerk steht deshalb hier, obwohl die wörtliche Bedingung nicht greift. Wer anders entscheidet,
sieht an dieser Stelle, worauf.

### 1. Die zweite offene Frage ist geprüft und diesem Circle gelassen worden

Frage 2 des Abschnitts `## Grounding snapshot` lautet: „Zeigt der Betrachter auch lokale
HTML-Dateien?" Die Runde 6 hat sie aufgenommen, weil sie die Dreiteilung der Anzeige aus C6 der
Runde 1 für Markdown ohnehin anfasste, und sie hat sie mit Möglichkeit 1 beantwortet: lokale
HTML-Dateien bleiben Quelltext, und die Frage bleibt bei diesem Circle. Der Datensatz sagt das
ausdrücklich, statt es durch Schweigen offenzulassen
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_zeigt-die-vorschau-lokale-html-dateien-gerendert.md`,
umgesetzt in `crates/krk-ui/src/hervorhebung.rs:422-431`, Commit `b4d9de2`).

**Was sich für diese Frage dennoch geändert hat, ist ihre Ausgangslage.** Die Dreiteilung aus C6
ist mit der Runde 6 einmal geändert worden, für Markdown, und die Vorschau zeigt seither
gerenderten Fließtext. Ein gerendertes HTML wäre danach nicht mehr der erste Bruch mit C6,
sondern der zweite. Der Grund, aus dem die Runde 6 HTML nicht mitgenommen hat, ist die
Ungleichheit der beiden Formate: Markdown lässt sich ohne Web-Mittel vollständig zerlegen, HTML
im Allgemeinen nicht. Wer diese Frage hier beantwortet, argumentiert gegen diese Begründung und
nicht in eine offene Lage hinein.

Als Nebengewinn färbt `hervorhebung.rs` samt `syntect` eine `.html`-Datei in der Vorschau
inzwischen ein. Sie sieht besser aus als vor der Runde 6, ohne gerendert zu sein.

### 2. Die 17 Punkte Luft in der Vorschaubreite sind nicht verbraucht worden

Der Vermerk vom 260812-0816 hielt fest, dass die Mindestbreite der Vorschau von 160 Punkten seit
der Runde 5 zwei Entscheidungen trägt und dass oberhalb davon rund 17 Punkte bis zur gerechneten
Obergrenze bleiben. Die Runde 6 hatte den ersten Vorschauinhalt, für den 160 Punkte knapp werden
könnten, und stand damit vor der Wahl, diese Luft zu verbrauchen.

**Sie hat sie stehen lassen.** Der Datensatz
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`
ist mit Möglichkeit 1 beantwortet: die Zahl bleibt bei 160
(`crates/krk-ui/src/fenstermodell.rs:213`, unverändert über die ganze Runde). Die Begründung ist
nicht, dass 160 Punkte genügen, sondern dass drei ungemessene Zahlen gegeneinander verrechnet
worden wären: die Obergrenze von 177 ist gerechnet, die Breite der acht Schalter von 540 ist
gerechnet, und die Enge des gerenderten Markdown ist an keinem laufenden Bündel gemessen.

**Der Datensatz steht bewusst auf beantwortet und nicht auf umgesetzt**, weil kein Commit die
Antwort einlöst: sie besteht darin, nichts zu ändern. Der Abgleich vom 260812-2253 begründet das
ausdrücklich damit, dass ein Marker auf umgesetzt den Datensatz aus der Suche nach aktiver
Grundlage nähme und der Auslöser mit ihm herausfiele. Der Auslöser ist ein Blick auf die Vorschau
am laufenden Bündel, und er steht als C4.14 im Plan der Runde 6.

Für diesen Circle heißt das: die Luft ist da, und die Frage nach ihr ist offen geblieben statt
still entschieden zu werden. Die vierte Frage der Klärungsrunde, die der Vermerk vom 260812-0816
aufgemacht hat, gilt unverändert in ihrer dortigen Fassung.

### 3. Eine fünfte Frage kommt hinzu, und sie hängt an derselben Zahl

Offen geblieben ist in der Runde 6, ob die Vorschau bei der kleinen Systemschriftgröße bleibt
oder auf die des Editors wächst
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1707_*_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-oder-waechst-sie-auf-die-des-editors.md`).
Die Frage entscheidet, wie viele Zeichen in eine Vorschau von 160 Punkten passen, und hängt damit
an derselben Zahl wie Punkt 2. Sie gehört vor die Untersuchung des Darstellungsmittels, weil ihre
Antwort mitbestimmt, wie viel Fläche das Mittel voraussetzen darf.

### 4. Die Messreihe hinter der dritten offenen Frage ist schlechter geworden

Die dritte Frage dieses Circles leitet eine mögliche elfte Zeitzusage aus L5 und L7 ab. Der
Vermerk vom 260807-1042 hielt fest, dass beide zum ungemessenen Teil der Abnahmereihe gehören.
Der Befund gilt weiter, und die Runde 6 hat ihn verschärft: L7 wird bei tief verschachtelten
Listen jetzt ab rund 12 kB verfehlt statt ab 19 kB
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-2133_*_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-mehr-als-der-rest-der-zerlegung.md`,
offen). Die Zahl der Zusage ist unverändert, der Abstand zu ihr nicht.

Der Betrachter ist nach der Directive dieses Circles eine weitere Quelle für den aktiven
Vorschau-Tab und teilt sich den Weg von L7 mit den bestehenden. Fällt die Antwort auf die dritte
Frage auf ja, erbt eine elfte Zusage nicht mehr nur den offenen Beleg von L7, sondern auch einen
gemessenen Verlust an Abstand. Dazu steht L9 aus zwei Runden zum Nachmessen an: die Bereichsleiste
der Runde 5 nimmt der Fensterzeile 18 Punkte Höhe, die zusammengelegte Statuszeile der Runde 6
gibt jedem Dateifenster 18 zurück, und gemessen ist keine der beiden Rechnungen.

## Activation proposal

**Vorgeschlagen am:** 260812-2307
**Playmaker-Lauf:** 260812-2307-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Klärungsrunde über fünf Fragen und einer
Untersuchung des Darstellungsmittels, nicht davor

Dieser Circle ist der empfohlene nächste Kandidat, und zwar ohne Vergleichswert: mit dem
Abschluss der Runde 6 ist er der einzige nicht abgeschlossene Circle im Portfolio. Eine Rangfolge
mit einem Element trägt keine Auskunft über relative Reife. Der Vorschlag stützt sich deshalb auf
die absoluten Signale, und zwei davon sind seit dem Vorschlag vom 260812-0816 neu.

**Das tragende neue Signal ist, was die Runde 6 diesem Circle *nicht* genommen hat.** Sie hat die
Vorschau angefasst, gerendertes Markdown gebaut und die Dreiteilung aus C6 geändert, also genau
die Bauteile berührt, auf denen dieser Circle sitzt. Sie hätte dabei zwei seiner Fragen
stillschweigend entscheiden können. Beide sind stattdessen als eigene Datensätze abgelegt und
zugunsten dieses Circles beantwortet worden: die Mindestbreite bleibt bei 160 Punkten, die 17
Punkte Luft sind unverbraucht, und die Frage nach lokalem HTML bleibt hier. Der Abschnitt
`## Parent grounding stale` oben schlüsselt beides auf. Der Zuschnitt dieses Circles ist damit
vollständig erhalten, und das ist bei einer Runde, die dieselbe Fläche umgebaut hat, kein
Selbstverständnis.

**Die geerbten Bauteile stehen und sind um zwei reicher geworden.** Die vier Bauteile aus dem
Abschnitt `## Grounding snapshot` liegen unverändert auf der Platte: die Auswertung der
Zwischenablage, das Vorschaufenster mit seiner Tableiste, die Statuszeile und der Befehl
`zwischenablage_springen` auf `opt+cmd+g`. Dazu kommt aus der Runde 5 die Bereichsleiste mit
einem Schalter für die Vorschau, und aus der Runde 6 zweierlei: eine Statuszeile über die volle
Fensterbreite statt zweier schmaler, und ein Kontextmenü an fünf Ansichten, das an genau einer
Stelle gebaut wird. Für einen Betrachter, dessen Meldungen über eine nicht erreichbare Adresse in
die Statuszeile gehören, ist die breitere Zeile eine unmittelbare Entlastung.

**Der Zuschnitt bleibt das stärkste Gegenargument, und er ist unverändert.** Der Datensatz hält
selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene
Untersuchung vor dem Plan gehört. Daneben steht die ungemessene Verfügbarkeitsfrage für
macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
die dieser Datensatz selbst als erschlossen und nicht als geprüft einordnet, und die projektweit
offene Frage, ob die Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`). Eine
Untersuchung vor dem Plan ist teurer als eine Klärungsrunde, und dieser Circle braucht beides.

**Die Klärungsrunde trägt jetzt fünf Fragen.** Die drei des Abschnitts `## Grounding snapshot`,
die Mindestbreite der Vorschau aus dem Vermerk vom 260812-0816, und seit der Runde 6 die
Schriftgröße der Vorschau. Die letzten beiden hängen an derselben Zahl und gehören vor die
Untersuchung des Darstellungsmittels, weil ihre Antworten mitbestimmen, wie viel Fläche das
Mittel voraussetzen darf.

**Zur Abhängigkeitslage, die in diesem Projekt nichts unterscheidet.** Die einzige
Circle-Abhängigkeit dieses Datensatzes, die Runde 1, ist beschränkt abgeschlossen (`_b_`) und
nicht kohärent (`_c_`), also trägt er nach der Rangheuristik das Kennzeichen der unerfüllten
Vorbedingung. Alle sechs gefahrenen Runden tragen `_b_`, und alle sechs aus demselben Grund: der
Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Das Kennzeichen steht an jedem denkbaren Kandidaten dieses Projekts und ist keine Auskunft über
diesen. Inhaltlich bindet die Beschränkung hier dennoch, und der Punkt 4 des Vermerks oben zeigt,
dass die Bindung seit der Runde 6 fester geworden ist statt loser.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260813-0714
**Playmaker-Lauf:** 260813-0714-playmaker-direct-dispatch
**Beschränkt abgeschlossenes Kind:**
`260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` (Runde 7), geschlossen
am 260813

Die Runde 7 hat die Belegung, das Menü und die Ablage angefasst, und alle drei stehen im
Abschnitt `## Grounding snapshot` dieses Datensatzes als Tatsachenaussagen. Keine der vier
Feststellungen unten hält die Aktivierung auf; alle vier gehören in die Klärungsrunde.

**Zur Auslösebedingung, offen benannt.** Die Regel verlangt, dass der Abschnitt
`## Grounding snapshot` dieses Datensatzes den Verzeichnisnamen des abgeschlossenen Kindes oder
den in seiner `## Closure note` genannten Artefakt zitiert. Er zitiert weder das eine noch das
andere. Anders als bei der Runde 6 läuft diesmal auch keine Kante in die Gegenrichtung: der
Abschnitt `## Dependencies` der Runde 7 lautet „Keine auf einen anderen Circle." Zwischen beiden
Circles besteht also überhaupt keine notierte Kante. Der Vermerk steht hier, weil die Runde 7 am
Baum drei Sätze dieses Grounding eingeholt hat, nicht weil eine Zitatbeziehung ihn auslöst. Wer
anders entscheidet, sieht an dieser Stelle, worauf.

### 1. Ein neuer Befehl kostet seit der Runde 7 mehr als einen Eintrag in der Belegungstabelle

Der Abschnitt `## Grounding snapshot` schreibt unter `### Was der aktive Circle schon gebaut hat
und dieser erbt`:

> Neue Befehle des Betrachters, etwa das Ein- und Ausschalten der Sprungmarken, bekommen ihre
> Einträge in derselben Tabelle.

Der Satz bleibt wahr und ist seit der Runde 7 unvollständig. Die Menüleiste wird nicht mehr als
Programmtext gepflegt, sondern aus der Belegung gerechnet: `menuemodell::aufbau` liefert neun
Obermenüs und einen Eintrag je Funktion, und `crates/krk-ui/src/appkit/menue.rs` setzt den Wert
nur noch in `NSMenu` und `NSMenuItem` um. Ein Eintrag in `resources/default-keymap.toml` erzeugt
damit selbsttätig einen Menüeintrag, und der bringt zwei Pflichten mit.

Die erste ist die Ausgrauung, und sie ist nach dem Grounding der Runde 7 eine
Korrektheitsbedingung und keine Politur: ein Menüeintrag mit Kürzel führte bis dahin einen
Befehl aus, den die Fokusprüfung gerade abgewiesen hatte. `validateMenuItem:` fragt jetzt
dieselbe Regel wie der Ereignisabgriff, nämlich `zulaessig` in
`crates/krk-ui/src/kommandos/zulaessigkeit.rs:113`. Die zweite Pflicht sind die beiden
Aufzählungsstellen, die `CLAUDE.md` unter „Was man nicht sieht" führt: `Kommando::wirkungsbereich`
und `bereich_des_kommandos`. Für einen Betrachter, dessen Befehle nur wirken, solange er den
Fokus hält, ist die Ausgrauung kein Nebenzweig, sondern der Normalfall.

Die Größenordnung: `Kommando::KENNUNGEN` trägt am 260813 sechsundsiebzig Einträge
(`crates/krk-core/src/tasten/belegung.rs:566`). Jeder Befehl des Betrachters kommt oben drauf und
durchläuft dieselben Stellen.

### 2. Die dritte Möglichkeit der ersten offenen Frage führt jetzt an eine Sperre

Die erste offene Frage dieses Circles bietet als dritte Möglichkeit an, gespeicherte
Web-Adressen in die Lesezeichenleiste aus C5 zu legen. Die Lesezeichen liegen in der Ablage unter
`~/Library/Application Support/KRK/`, und genau dort hat die Runde 7 gearbeitet: `opt+cmd+n`
startet eine weitere Instanz, und zwei Sperren über `flock` sollen verhindern, dass zwei
Instanzen einander die Ablage zerlegen.

Die Sperre ist nach der eigenen Durchsicht der Runde nicht dicht. Der Datensatz
`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0716_*_die-bewachte-luecke-ist-nicht-die-luecke-elf-schreibwege-an-der-sperre-vorbei-bleiben.md`
ist offen und benennt elf Schreibwege an der Sperre vorbei. Wer die dritte Möglichkeit wählt,
erbt damit eine bewachte Schnittstelle mit benannten Löchern statt einer ungeschützten Datei.
Die zugehörige Nutzerfrage steht ebenfalls offen:
`shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`.

Für die ersten beiden Möglichkeiten der Frage ändert sich nichts. Der Punkt betrifft allein die
dritte und verteuert sie.

### 3. Der Fokusvorbehalt fragt nach drei Textklassen, und eine Web-Ansicht ist keine davon

`ersthelfer_gehoert_appkit` (`crates/krk-ui/src/appkit/ereignisse.rs:581`) liefert `true` allein
für `NSTextView`, `NSTextField` und `NSText`, abzüglich der Editorfläche, die seit der Runde 7
über eine hereingereichte Prüffunktion erkannt wird statt über Modulwissen. Eine Ansicht, die
Web-Inhalt darstellt, gehört zu keiner der drei Klassen.

`inference:` Solange eine solche Ansicht den Ersthelferrang hält, nimmt die Belegung von KRK
folglich jede Taste. Für die Tastatursteuerung des Betrachters, die dieser Circle in seiner
Directive verlangt, ist das der gewünschte Ausgang. Für ein Eingabefeld innerhalb einer
angezeigten Seite ist es der unerwünschte, und der Baum entscheidet die Frage heute nicht: er
trägt keine Web-Ansicht, und die Aussage ist am Prädikat abgelesen und nicht gemessen.

Die Frage, an der das hängt, ist eine der vier, auf deren Empfehlung die Runde 7 gefahren ist,
ohne dass sie beantwortet wäre:
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`.
Wer den Betrachter aktiviert, sollte sie vorher stellen, weil ihre Antwort bestimmt, welchen
Weg ein Tastendruck in der Web-Ansicht nimmt.

### 4. Die Messreihe hinter der dritten offenen Frage steht jetzt zwei Runden zurück

Die dritte offene Frage dieses Circles leitet eine mögliche elfte Zeitzusage aus L5 und L7 ab.
Der Vermerk vom 260812-2307 hielt fest, dass die Runde 6 den Abstand zu L7 gemessen verkleinert
hat. Die Runde 7 setzt keine elfte Zusage und fasst keine der zehn an, sagt das ausdrücklich in
ihrer Directive und ist trotzdem eine weitere unabgenommene Runde: der Abnahmelauf steht seit der
Runde 6 aus und jetzt für zwei Runden.

Der Befund von 260807-1042 gilt damit unverändert und wird nur älter. L5 und L7 stehen weiter auf
der Abnahmereihe `messungen/260805-2207-MacBookPro15-1-abnahme.txt` vom 260805-2207, und
inzwischen liegen zwei gebaute Runden dazwischen. Eine elfte Zusage, die aus ihnen abgeleitet
würde, erbte einen Belegstand von acht Tagen.

## Activation proposal

**Vorgeschlagen am:** 260813-0714
**Playmaker-Lauf:** 260813-0714-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Klärungsrunde über fünf Fragen und einer
Untersuchung des Darstellungsmittels, nicht davor

Dieser Circle ist der empfohlene nächste Kandidat, wie schon beim Lauf vom 260812-2307, und
wieder ohne Vergleichswert: nach dem Abschluss der Runde 7 ist er der einzige nicht
abgeschlossene Circle im Portfolio. Eine Rangfolge mit einem Element sagt nichts über relative
Reife. Der Vorschlag stützt sich auf die absoluten Signale.

**Die Runde 7 hat den Zuschnitt dieses Circles nicht berührt, und zum ersten Mal ist das kein
Verdienst, sondern eine Randlage.** Bei der Runde 6 war die Unberührtheit ein Signal: jene Runde
baute an derselben Fläche und hat zwei Fragen dieses Circles ausdrücklich zu seinen Gunsten
entschieden, statt sie stillschweigend zu verbrauchen. Die Runde 7 hat die Vorschau gar nicht
angefasst. Sie hat an der Belegungsansicht, am Hauptmenü und an der Ablage gearbeitet, und keine
der drei Flächen ist die, auf der der Betrachter sitzt. Das ist eine schwächere Auskunft als die
vom 260812-2307, und sie sollte nicht als dieselbe gelesen werden.

**Was die Runde 7 stattdessen geliefert hat, ist ein größerer Eintrittspreis pro Befehl.** Der
Abschnitt `## Parent grounding stale` oben schlüsselt es auf: ein neuer Befehl erzeugt seit der
Runde 7 selbsttätig einen Menüeintrag und braucht dafür eine Ausgrauungsregel, einen
Wirkungsbereich und eine Zeile in `bereich_des_kommandos`. Für einen Betrachter mit eigenen
Befehlen für Blättern, Zurück, Vor und Sprungmarken ist das nicht wenig. Der Gewinn steht
daneben und ist echt: jeder dieser Befehle ist danach auf drei Wegen erreichbar statt auf einem,
ohne dass der Betrachter dafür etwas eigenes bauen müsste.

**Die geerbten Bauteile stehen unverändert.** Die vier Bauteile aus dem Abschnitt
`## Grounding snapshot` liegen auf der Platte: die Auswertung der Zwischenablage, das
Vorschaufenster mit seiner Tableiste, die Statuszeile über die volle Fensterbreite seit der
Runde 6, und der Befehl auf `opt+cmd+g`. Die 160 Punkte Mindestbreite der Vorschau sind
weiterhin nicht angetastet (`crates/krk-ui/src/fenstermodell.rs`), also bleiben die rund 17
Punkte Luft bis zur gerechneten Obergrenze unverbraucht.

**Der Zuschnitt bleibt das stärkste Gegenargument, und er ist unverändert.** Der Datensatz hält
selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene
Untersuchung vor dem Plan gehört. Daneben steht die ungemessene Verfügbarkeitsfrage für
macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`)
und die projektweit offene Frage, ob die Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`). Eine
Untersuchung vor dem Plan ist teurer als eine Klärungsrunde, und dieser Circle braucht beides.

**Die Klärungsrunde trägt weiterhin fünf Fragen**, und eine sechste steht jetzt daneben. Die drei
des Abschnitts `## Grounding snapshot`, die Mindestbreite der Vorschau aus dem Vermerk vom
260812-0816 und die Schriftgröße der Vorschau aus dem Vermerk vom 260812-2307. Dazu kommt aus der
Runde 7 die Frage, welchen Weg ein Tastendruck in einer Web-Ansicht nimmt; sie ist keine eigene
Frage dieses Circles, sondern der offene Nutzerentscheid
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`,
und sie sollte vor der Aktivierung eine Antwort haben statt danach.

**Zur Abhängigkeitslage, die in diesem Projekt nichts unterscheidet.** Die einzige
Circle-Abhängigkeit dieses Datensatzes, die Runde 1, ist beschränkt abgeschlossen (`_b_`) und
nicht kohärent (`_c_`). Nach der Rangheuristik trägt er damit das Kennzeichen der unerfüllten
Vorbedingung. Alle sieben gefahrenen Runden tragen `_b_`, und alle sieben aus demselben Grund:
der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`,
offen seit dem 260806). Das Kennzeichen steht an jedem denkbaren Kandidaten dieses Projekts und
ist keine Auskunft über diesen. Die Heuristik ist deshalb nicht angewandt, sondern ausgesetzt und
hier benannt.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260813-1510
**Playmaker-Lauf:** 260813-1510-playmaker-direct-dispatch
**Abgeschlossenes Kind:** `260813-0939-titelleiste-fuehrt-version-und-semantische-tags`
(Runde 8), geschlossen am 260813-1415 als **kohärenter Abschluss** (`_c_`)

**Zur Auslösebedingung, und sie ist diesmal nicht erfüllt.** Die Regel verlangt ein Kind auf
beschränktem Abschluss (`_b_`) und ein Zitat des Kindes im Abschnitt `## Grounding snapshot`
dieses Datensatzes. Beides fehlt: die Runde 8 trägt `_c_`, und dieser Datensatz kann sie nicht
zitieren, weil sein Grounding vom 260804 stammt. Der Vermerk steht trotzdem, weil die Runde 8 an
der einen Regel gearbeitet hat, durch die jeder Befehl dieses Betrachters laufen wird. Der
Auslöser der Regel ist an dieser Stelle zu eng gefasst: ein kohärenter Abschluss bewegt den Baum
mindestens so weit wie ein beschränkter, und der Markerbuchstabe sagt, ob die Directive erreicht
wurde, nicht ob sich der Boden verschoben hat. Wer die Regel anders liest, sieht hier, worauf.

Zwei Feststellungen, eine gegen diesen Circle und eine für ihn. Keine hält die Aktivierung auf.

### 1. Die Zulässigkeitsregel fragt seit der Runde 8 vier Dinge statt drei

Der Vermerk vom 260813-0714 hielt fest, dass ein neuer Befehl seit der gerechneten Menüleiste
mehr kostet als einen Eintrag in `resources/default-keymap.toml`. Die Runde 8 legt eine vierte
Frage an dieselbe eine Stelle: ob das Schlüsselfenster KRKs Hauptfenster ist. Die vier Eingaben
werden einmal je Befehl in der Struktur `Lage` erhoben
(`crates/krk-ui/src/kommandos/zulaessigkeit.rs`), und eine Tafel aus 280 Fällen prüft die Regel
ohne Fenster. Für die Befehle, die die Directive dieses Circles verlangt, also Blättern, Zurück,
Vor und das Schalten der Sprungmarken, ändert sich die Art des Preises nicht und seine Höhe um
eine Frage.

`Kommando::KENNUNGEN` steht dabei unverändert bei 76 Einträgen
(`crates/krk-core/src/tasten/belegung.rs:566`, am 260813-1510 nachgezählt). Die Runde 8 hat
keinen Befehl hinzugefügt; sie hat die Frage erweitert, die jeder bestehende beantworten muss.

Daneben steht seit der Runde 8 eine Ausnahmeliste als benanntes Mittel: `immer_erreichbar`
führt `Beenden`, `FensterSchliessen` und `FensterEinblenden` und hebt den Blattvorbehalt und die
Schlüsselfensterfrage auf, den Fokusvorbehalt dagegen nicht. Braucht der Betrachter einen Befehl,
der ohne Rücksicht auf die Lage erreichbar bleibt, gibt es dafür jetzt eine Stelle und eine Probe,
die die Grenze der Ausnahme festhält.

### 2. Der offene Punkt aus dem Vermerk vom 260813-0714 hat einen gebauten Präzedenzfall bekommen

Die dritte Feststellung jenes Vermerks lautete, `ersthelfer_gehoert_appkit` liefere `true` allein
für `NSTextView`, `NSTextField` und `NSText`, und eine Ansicht mit Web-Inhalt gehöre zu keiner der
drei. Das Prädikat steht unverändert (`crates/krk-ui/src/appkit/ereignisse.rs:581`), aber die Form
seiner Ausnahme ist inzwischen die gesuchte: die Editorfläche wird über eine hereingereichte
Prüffunktion `ist_editorflaeche` erkannt, also über die Nämlichkeit des Objekts und nicht über
seine Klasse, und `ereignisse.rs` kennt den Editor dafür nicht.

`inference:` Eine Web-Ansicht braucht dieselbe Behandlung aus demselben Grund: ihre Klasse trennt
sie nicht von einem Eingabefeld, das die angezeigte Seite selbst mitbringt. Der Baum trägt heute
keine Web-Ansicht, die Aussage ist am Prädikat abgelesen und nicht gemessen. Sie steht hier, weil
der Vermerk vom 260813-0714 den Punkt als offen benannt hat und die Runde 8 ihn nicht gelöst, wohl
aber vorgezeichnet hat. Die zugehörige Nutzerfrage bleibt offen:
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`.

## Activation proposal

**Vorgeschlagen am:** 260813-1510
**Playmaker-Lauf:** 260813-1510-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Untersuchung des Darstellungsmittels und
einer Klärungsrunde über sechs Fragen, und nach den beiden Nutzerschritten, die heute den
Auslieferungsweg anhalten

Dieser Circle ist der empfohlene nächste Kandidat und der einzige. Mit der Runde 8 ist der
Vergleichswert weggefallen, den der Lauf vom 260813-0958 hatte: damals standen zwei vorgesehene
Circles nebeneinander, und dieser stand auf Rang 2. Der Rangwechsel zurück auf 1 ist kein Befund
zu seinen Gunsten. Das Feld ist leer geräumt worden, nicht er besser geworden. Eine Rangfolge mit
einem Element sagt nichts über relative Reife, und der Vorschlag stützt sich wie in den vier
Läufen davor auf absolute Signale.

**Was sich zu seinen Gunsten geändert hat, liegt nicht an ihm, sondern am Projekt.** Sieben Runden
lang endete jede aus demselben Grund beschränkt, und die Playmaker-Läufe davor haben daraus
geschlossen, jede weitere Runde werde ebenso enden, solange die Frage nach dem Vordergrund offen
ist. Die Runde 8 hat das widerlegt: der Nutzer hat die elf Beobachtungen mit Bündelanteil von Hand
am laufenden `target/KRK.app` abgenommen
(`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1405-abnahmeliste-e2.md`).
Für diesen Circle heißt das: eine Runde, die ihn ausführt, kann kohärent enden, sofern der Nutzer
dieselbe Handabnahme fährt. Die Aussicht ist neu und sie hängt an der Bereitschaft des Nutzers,
nicht an einer gelösten technischen Frage.

**Der Zuschnitt bleibt das stärkste Gegenargument, und er ist unverändert.** Der Abschnitt
`## Was dieser Circle nicht festlegt` hält fest, dass das Mittel der Darstellung von Web-Inhalt
offen ist und in eine eigene Untersuchung vor dem Plan gehört. Eine Untersuchung ist teurer als
eine Klärungsrunde, und dieser Circle braucht beides. Daneben stehen die ungemessene
Verfügbarkeitsfrage für macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`)
und die projektweit offene Frage, ob die Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`).

**Die Klärungsrunde trägt weiterhin sechs Fragen**, unverändert gegenüber dem Vermerk vom
260813-0714: die drei des Abschnitts `## Grounding snapshot`, die Mindestbreite der Vorschau, ihre
Schriftgröße, und der offene Nutzerentscheid darüber, welchen Weg ein Tastendruck in einer
Web-Ansicht nimmt. Die Runde 8 hat keine davon beantwortet und keine hinzugefügt.

**Zur Abhängigkeitsprüfung, die für diesen Circle nie eine Auskunft werden kann.** Die einzige
Circle-Kante dieses Datensatzes führt auf die Runde 1, und die trägt `_b_`. Bisher war die
Heuristik ausgesetzt, weil kein einziger Circle des Projekts `_c_` trug und der Marker deshalb
nichts unterschied. Diese Begründung ist mit der Runde 8 hinfällig. An ihre Stelle tritt eine
stärkere: `_b_` ist ein Endzustand, und ein Endzustand wird nicht zurückgenommen
(`rules/circle-records.md`, Abschnitt `### Worked transitions`). Die Prüfung „alle Abhängigkeiten
kohärent abgeschlossen" fällt für diesen Circle also für immer negativ aus, gleich welche Arbeit
noch geschieht. Ein Kriterium, dessen Wert keine künftige Arbeit ändern kann, ist für ihn kein
Rangsignal, sondern eine Konstante. Es ist deshalb nicht eingerechnet.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Activation proposal

**Vorgeschlagen am:** 260813-2203
**Playmaker-Lauf:** 260813-2203-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Untersuchung des Darstellungsmittels und
einer Klärungsrunde über sechs Fragen

Dieser Vermerk trägt eine einzige Änderung nach und wiederholt den Vorschlag vom 260813-1510
nicht. Der Circle selbst ist unverändert: derselbe Zuschnitt, dieselben sechs offenen Fragen,
dieselbe Untersuchung vor dem Plan. Er bleibt Rang 1 und einziger Kandidat.

**Die beiden Nutzerschritte, die der Vermerk vom 260813-1510 dem Aktivierungszeitpunkt
voranstellte, sind erledigt.** Jener Vermerk nannte als Bedingung den Auslieferungsweg, den
`cargo xtask release` aus zwei Gründen anhielt: kein Tag auf HEAD und geänderte verfolgte
Dateien. Beide Gründe bestehen nicht mehr, am Baum geprüft am 260813-2203:

- `git tag --points-at HEAD` liefert `v0.2.1`, und `Cargo.toml:13` führt `version = "0.2.1"`.
  Station 1 des Auslieferungswegs vergleicht genau diese beiden Werte
  (`xtask/src/release.rs`, `stand_pruefen` ab Zeile 208).
- `git status --porcelain --untracked-files=no` ist leer. Der flüchtige Sitzungszustand der
  Werkbank steht seit `7537ee5` und `5ae3800` in `.gitignore`, das Wächterprotokoll
  eingeschlossen.

Damit fällt die letzte Bedingung des vorigen Vermerks weg, die nicht am Circle selbst hängt. Der
Aktivierungszeitpunkt steht jetzt allein hinter Untersuchung und Klärungsrunde.

**Ein zweiter Nachtrag, der diesen Circle mittelbar betrifft.** Der Tag `v0.1.0` sitzt auf
`3a0a4bf`, dem Abschlusscommit der Runde 8. Damit ist deren letztes offenes Abnahmekriterium
C3.15 erfüllt, das ihre `## Closure note` als den einen verbleibenden Nutzerschritt benennt. Für
die Aussicht dieses Circles zählt das: die Runde 8 ist nicht nur kohärent geschlossen, sondern
inzwischen vollständig abgenommen, und sie ist der Beleg dafür, dass eine Runde dieses Projekts
über eine Handabnahme des Nutzers kohärent enden kann.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260814-1301
**Playmaker-Lauf:** 260814-1301-playmaker-direct-dispatch
**Abgeschlossenes Kind:** `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` (Runde 9),
geschlossen am 260814-1300 als **beschränkter Abschluss** (`_b_`)

**Zur Auslösebedingung.** Die eine Hälfte ist diesmal erfüllt, die andere nicht: die Runde 9 trägt
`_b_`, aber der Abschnitt `## Grounding snapshot` dieses Datensatzes stammt vom 260804 und kann
sie nicht zitieren. Der Vermerk steht aus demselben Grund wie die vier davor — die Runde hat an
zwei Stellen gearbeitet, durch die jeder Befehl dieses Betrachters laufen wird. Zwei
Feststellungen, beide für die Klärungsrunde vor der Aktivierung, keine hält sie auf.

### 1. Die Ausnahme im Ersthelfervorbehalt hat jetzt einen Präzedenzfall in beide Richtungen

Der Vermerk vom 260813-1510 hielt fest, die Form der Editor-Ausnahme über die hereingereichte
Prüffunktion `ist_editorflaeche` sei „die gesuchte" für eine Web-Ansicht. Die Runde 9 zeigt, dass
die Frage zwei Antworten hat und die Wahl zwischen ihnen an der Form der Ansicht hängt, nicht an
ihrem Inhalt.

Die Textfläche des Notizzettels ist die erste zweite bedienbare Textfläche dieses Baums, die in
`ersthelfer_gehoert_appkit` **absichtlich nicht** angemeldet ist. Der Modulkopf von
`crates/krk-ui/src/appkit/blaetter/zettel.rs` schreibt die Kette aus: bleibt die Fläche
unangemeldet, gehört ihr Ersthelfer AppKit, `Kommando::Abbrechen` ist damit unzulässig, der
Tastendruck läuft unverändert weiter, und `cancelOperation:` kommt beim Wächter des Blattes an.
Eine Anmeldung kehrte das um und nähme dem Zettel den Weg zurück. Die Probe
`im_textfeld_eines_blattes_ist_auch_der_abbruch_abgewiesen`
(`crates/krk-ui/src/kommandos/zulaessigkeit.rs:642`) hält die Vorbedingung fest.

Für diesen Circle heißt das: der Betrachter lebt nach seiner Directive in einem gewöhnlichen Tab
des Vorschaufensters, ist also ein Bereich der Fensterzeile und kein Blatt. Er steht damit auf der
Seite des Editors und will KRKs Befehle mit dem Fokus in sich selbst. `inference:` Ob er dafür
eine Anmeldung braucht, ist offen und am Baum nicht zu entscheiden: das Prädikat prüft auf
`NSTextView`, `NSTextField` und `NSText` (`crates/krk-ui/src/appkit/ereignisse.rs:594`), und eine
Ansicht mit Web-Inhalt gehört zu keiner der drei, so dass es für sie ohnehin `false` liefert. Die
Frage stellt sich erst, wenn ein Eingabefeld der angezeigten Seite den Ersthelferrang an ein
Objekt einer dieser drei Arten gibt. Der Baum trägt keine Web-Ansicht; gemessen ist nichts davon.

### 2. Ein neuer Befehl kommt beim Nutzer mit eigener Belegungsdatei unbelegt an

`Kommando::KENNUNGEN` steht nach der Runde 9 bei 77 Einträgen
(`crates/krk-core/src/tasten/belegung.rs:579`, am 260814-1301 nachgezählt); der Zettel ist der
eine neue Befehl. Beim Planen derselben Runde ist ein Defekt gefunden worden, der jede weitere
Runde mit neuen Befehlen trifft:
`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`.
`Belegung::bauen` nimmt die Datei des Nutzers als Quelle der Tasten und fügt eine Funktion, die sie
nicht nennt, mit leerer Tastenliste hinzu. Wer seit der Runde 7 einmal eine Taste zugewiesen hat,
bekommt jede später ausgelieferte Funktion tot.

Die Directive dieses Circles verlangt vier neue Befehle, Blättern, Zurück, Vor und das Schalten
der Sprungmarken. Jeder davon ist betroffen. Für die Abnahme der Runde 9 hat ein Handgriff
gereicht, die eigene `keymap.toml` vor dem Lauf beiseitezulegen; der Defekt nennt drei Wege zu
einer Lösung und keiner ist gewählt. Die Klärungsrunde dieses Circles wächst damit nicht um eine
Frage, wohl aber seine Abnahmeplanung um eine Vorbedingung.

## Activation proposal

**Vorgeschlagen am:** 260814-1301
**Playmaker-Lauf:** 260814-1301-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Untersuchung des Darstellungsmittels und
einer Klärungsrunde über sechs Fragen

Dieser Vermerk trägt zwei Änderungen nach und wiederholt die Vorschläge vom 260813-1510 und
260813-2203 nicht. Der Circle bleibt Rang 1 und einziger Kandidat, bei unverändertem Zuschnitt.

**Die eine Bedingung, die der Vermerk vom 260813-2203 als erledigt gemeldet hat, steht wieder
offen.** Am 260814-1301 am Baum geprüft: `git tag --points-at HEAD` liefert nichts, zwölf Commits
liegen zwischen `v0.2.1` und `HEAD` (`4907cd4`), und `Cargo.toml` führt weiter `0.2.1`. Station 1
von `cargo xtask release` vergleicht Tag und Version (`xtask/src/release.rs`, `stand_pruefen` ab
Zeile 208) und hält den Auslieferungsweg damit erneut an. Der Arbeitsbaum ist sauber. Das ist
keine Bedingung des Circles, sondern der wiederkehrende Zustand nach jeder Runde: die Runde 9 hat
zehn Commits hinzugefügt und keinen Tag gesetzt.

**Die Runde 9 nimmt der Aussicht auf einen kohärenten Abschluss etwas von ihrer Sicherheit, und
sie sagt zugleich, woran es lag.** Der Vermerk vom 260813-1510 hatte aus der Runde 8 geschlossen,
eine Runde dieses Projekts könne kohärent enden, sofern der Nutzer die Handabnahme fährt. Die
Runde 9 hat die Handabnahme gefahren und ist dennoch beschränkt geschlossen. Der Unterschied liegt
nicht am Nutzer und nicht am Baum, sondern am Zuschnitt der Abnahmeliste: die Runde 8
kennzeichnete jedes Abnahmekriterium einzeln als `(Probe)` oder `(Bündel)` und stellte je
Bündelkriterium genau eine Beobachtung; die Runde 9 führte zwei Listen je Fähigkeit, verlor die
Bindung zwischen Beobachtung und Kriterium, und 16 der 29 Kriterien mit Bündelanteil hat keine
Beobachtung angefasst. Nachzulesen in der `## Closure note` von
`circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/_*_circle.md`.

Für diesen Circle ist das die billigste verfügbare Maßnahme und sie gehört an das Plan-Gate: die
Abnahmeliste seiner Runde trägt je Kriterium eine Kennzeichnung und je Bündelkriterium eine
Beobachtung. Der Betrachter wird viele Kriterien am laufenden Bündel haben, weil sich Blättern,
Zurück, Vor und Sprungmarken an einer echten Seite zeigen und nicht an einer Probe.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Activation proposal

**Vorgeschlagen am:** 260814-1513
**Playmaker-Lauf:** 260814-1513-playmaker-user-fusion-next
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Untersuchung des Darstellungsmittels und
einer Klärungsrunde über die offenen Fragen

Der Circle bleibt Rang 1 und einziger Kandidat, bei unverändertem Zuschnitt. Dieser Vermerk trägt
zwei Änderungen seit dem 260814-1301 nach und wiederholt die Vorschläge davor nicht.

**Die Auslieferungssperre, die der Vermerk vom 260814-1301 als wieder offen gemeldet hat, ist
geschlossen.** Am 260814-1513 am Baum geprüft: `git tag --points-at HEAD` liefert `v0.3.0`, und
`Cargo.toml` führt `version = "0.3.0"`. Station 1 von `cargo xtask release` vergleicht beide und
hält den Weg damit nicht mehr an. Der Zustand ist nicht dauerhaft: er kehrt nach jeder Runde
zurück, die Commits hinzufügt und keinen Tag setzt.

**`CLAUDE.md` ist am 260814-1405 kuratiert worden und beschreibt jetzt neun Runden statt vier.**
Für diesen Circle zählt daran ein Absatz: die Datei sagt seither ausdrücklich, dass der Marker
`_c_` in diesem Projekt die Verfügbarkeit des Nutzers misst und nicht die Reife einer Runde. Wer
den Abhängigkeitsstand dieses Circles liest, trifft auf `260802-0842-krk-mac-dateimanager-editor-git`
mit beschränktem Abschluss (`_b_`) und darf daraus keine unfertige Vorbedingung ableiten. Die
Runde 1 hat alles gebaut, worauf dieser Circle aufsetzt, die Zwischenablage-Auswertung aus S13 und
das Vorschaufenster aus S19 eingeschlossen.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260815-0350
**Playmaker-Lauf:** 260815-0350-playmaker-user-fusion-next
**Abgeschlossenes Kind:** `260814-1551-tippen-filtert-dateiliste-flach-und-tief` (Runde 10),
geschlossen am 260815 als **beschränkter Abschluss** (`_b_`)

**Zur Auslösebedingung, und sie ist wie beim Vermerk vom 260814-1301 zur Hälfte erfüllt.** Die
Runde 10 trägt `_b_`. Der Abschnitt `## Grounding snapshot` dieses Datensatzes stammt vom
260804 und kann eine Runde vom 260814 nicht zitieren; er nennt weder das Verzeichnis der Runde 10
noch ihr Artefakt. Was ihn dennoch berührt, ist kein Zitat, sondern ein Bauteil: die eine
Statuszeile und die Bedeutung eines nackten Tastendrucks stehen beide in seiner Grundlage, und die
Runde 10 hat beide angefasst. Der Vermerk steht deshalb hier, mit derselben Einordnung wie seine
Vorgänger: Eingabe für die Klärungsrunde, kein Grund, die Aktivierung aufzuhalten.

### 1. Die Statuszeile trägt jetzt sechs Ränge, und der siebte hält den Bau an

Die Grundlage dieses Circles sagt, was der Betrachter zu melden hat, etwa eine nicht erreichbare
Adresse, gehöre in die eine Statuszeile und nicht in ein eigenes Meldewesen. Diese Zusage gilt
unverändert; der Preis dafür ist gestiegen. `Rang` in `crates/krk-ui/src/appkit/statuszeile.rs:207`
führt am 260815 sechs Werte, und der neue heißt `Filterstand`: er steht zwischen `Tabmeldung` und
`Markierungsstand` und nennt den stehenden Filtertext samt den gezeigten gegen die vorhandenen
Einträge. Die Aufzählung ist weiterhin eine vollständige Fallunterscheidung ohne Auffangzweig, und
ihre Reihenfolge **ist** die Rangfolge.

Für diesen Circle folgt daraus eine Frage an den Aktivierungs-Spec, die vorher keine war: an
welcher Stelle der sechs steht die Meldung des Betrachters. Eine nicht erreichbare Adresse ist der
Sache nach eine Befehlsantwort und stünde damit obenauf; eine Ladeanzeige während des Abrufs käme
der `Vorgangsanzeige` nahe, die heute allein Dateioperationen trägt. Beides ist entscheidbar, und
keines entscheidet sich von selbst: ein siebter Rang hält den Bau an und erzwingt die Einordnung
an Ort und Stelle.

### 2. Ein nackter Tastendruck im Dateifenster bedeutet seit der Runde 10 etwas anderes

Die Runde 1 hatte festgelegt, dass eine freie Taste ohne Zusatztaste im Dateifenster auf die
Sprungmarke durchfällt. Die Runde 10 hat dieses Abnahmekriterium **ersetzt und nicht ergänzt**,
nach ausdrücklicher Wahl des Nutzers: Tippen füllt jetzt einen Filter, der Sprung auf den ersten
passenden Eintrag ist auf keinem Weg mehr zu haben.

Die Directive dieses Circles verspricht „Sprungmarken auf jedem sichtbaren Link". Das ist ein
anderer Mechanismus als die gefallene Sprungmarke des Dateifensters, und die Zeichenprüfung
`krk_core::verzeichnis::sprungmarke::traegt_ein_dateiname` steht weiter, weil die Tippsuche der
Belegungsansicht aus der Runde 7 sie ebenfalls benutzt. Zwei Dinge gehören trotzdem in die
Klärungsrunde. Erstens die Benennung: wer nach der Runde 10 „Sprungmarke" liest, denkt an eine
Sache, die es im Dateifenster nicht mehr gibt, und der Aktivierungs-Spec sollte den Verweisanker
des Betrachters anders nennen oder den Unterschied ausdrücklich hinschreiben. Zweitens die Frage,
die der Betrachter neu aufwirft: das Dateifenster beansprucht nackte Zeichen inzwischen für sich,
das Vorschaufenster tut es nicht. Ein Betrachter, der über die Tastatur bedient wird und dessen
Verweisanker aus getippten Zeichen bestehen, ist der erste Bereich neben dem Dateifenster mit
diesem Anspruch.

### 3. `Esc` trägt drei Bedeutungen, und der Betrachter brächte eine vierte

Nach der Runde 10 räumt `Esc` den Filtertext weg, als dritter und letzter Rang hinter einem
stehenden Blatt und einer laufenden Operation. Der Betrachter will `Esc` für den Ausstieg aus der
Verweisanker-Anzeige, und das wäre der vierte Anspruch auf dieselbe Taste. Die Reihenfolge ist
kein Detail: sie entscheidet, was der Nutzer verliert, wenn er die Taste einmal zu oft drückt.

Der Entscheidungsdatensatz der Runde 10 zu dieser Reihenfolge steht im Speicher weiter auf offen
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_*_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`),
während der Baum die Frage beantwortet hat. Wer die Rangfolge für den Betrachter fortschreibt,
liest den Baum und nicht allein den Datensatz.

### 4. Die Belegung ist gewachsen, und die Bindung an einen offenen Defekt bleibt

Am 260815 führt `resources/default-keymap.toml` 84 Einträge und `Kommando` 78 Varianten, gezählt
mit `grep -c '^\[\[' resources/default-keymap.toml` und
`awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs`. Die Runde 10 hat den
Befehl für die tiefe Suche beigesteuert. Die vier Befehle dieses Circles, Blättern, Zurück, Vor
und das Schalten der Verweisanker, kommen darauf und erreichen einen Nutzer mit eigener
Belegungsdatei weiterhin unbelegt
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).
Der Befund ist gegenüber dem Vermerk vom 260814-1301 unverändert und wird hier nur nachgezählt.

### 5. Was die Runde 10 über das Prüfen gelernt hat, gilt diesem Circle besonders

Die Abschlussnotiz der Runde 10 nennt einen Fehlertyp, der siebenmal aufgetreten und jedes Mal von
einer Prüfung gefangen worden ist, nie von einem Menschen beim Lesen: ein Text sagt mehr zu, als
seine Probe hält. Zwei Diagrammprüfungen des Spec fanden je einen echten Fehler, bevor eine Zeile
Code geschrieben war, darunter einen fehlenden Ausgang, den drei Abnahmekriterien voraussetzten.

Der Spec dieses Circles wird einen Navigationsstapel je Tab beschreiben, dazu Zurück, Vor und die
Verweisanker. Ein Stapel mit zwei Richtungen und einem Zustand, der beim Ersetzen des Tabinhalts
stirbt, ist genau die Art Gegenstand, an der eine Diagrammprüfung vor dem Plan-Gate billig einen
fehlenden Ausgang findet. Zusammen mit der Empfehlung des Vermerks vom 260814-1301, je Kriterium
eine Kennzeichnung und je Bündelkriterium eine Beobachtung zu führen, sind das die zwei
Maßnahmen, die aus zwei aufeinanderfolgenden Runden dieses Projekts stammen und nichts kosten
außer Reihenfolge.

## Activation proposal

**Vorgeschlagen am:** 260815-0350
**Playmaker-Lauf:** 260815-0350-playmaker-user-fusion-next
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach einer Untersuchung des Darstellungsmittels und
einer Klärungsrunde über die offenen Fragen

Der Circle bleibt Rang 1 und einziger Kandidat, bei unverändertem Zuschnitt. Dieser Vermerk trägt
die Änderungen seit dem 260814-1513 nach und wiederholt die Vorschläge davor nicht.

**Die Runde 10 ist gefahren und beschränkt geschlossen.** Was sie an der Grundlage dieses Circles
bewegt, steht im Vermerk `## Parent grounding stale` vom 260815-0350 darüber und wird hier nicht
wiederholt. Für die Rangfolge selbst ändert sie nichts: kein neuer Circle trägt vorgesehen (`_a_`),
und der Abhängigkeitsgraph über die nicht terminalen Circles hat weiterhin einen Knoten und keine
Kante.

**Die Auslieferungssperre steht wieder offen, wie nach jeder Runde.** Am 260815-0350 am Baum
geprüft: `git tag --points-at HEAD` liefert nichts, 24 Commits liegen zwischen `v0.3.0` und `HEAD`
(`2d2ce87`), und `Cargo.toml` führt weiter `0.3.0`. Station 1 von `cargo xtask release` vergleicht
Tag und Version und hält den Auslieferungsweg damit an. Das ist keine Bedingung dieses Circles,
sondern der wiederkehrende Zustand: der Tag wird vom Nutzer gesetzt, nicht vom Werkzeug.

**Zwei Abnahmelisten stehen jetzt aus statt einer.** Die Runde 9 hat 21 Kriterien ohne vollen
Beleg gelassen, die Runde 10 zehn von 77 mit Bündelanteil, davon vier sicherheitsrelevant; ihre
Liste ist
`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`,
geschätzt 25 Minuten am laufenden Bündel. Beide Läufe sind Nutzerarbeit und keiner bewegt einen
Marker. Für diesen Circle zählt daran, dass er selbst viele Kriterien am laufenden Bündel haben
wird, weil sich Blättern, Zurück, Vor und die Verweisanker an einer echten Seite zeigen und nicht
an einer Probe. Wer den Betrachter aktiviert, ohne die zwei stehenden Listen zu fahren, sammelt
eine dritte an.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260818-1018
**Playmaker-Lauf:** 260818-1018-playmaker-orchestrator-phase4
**Beschränkt abgeschlossenes Kind:** `260816-1321-inhaltsfilter-mit-ankreuzfeld-content`
(Runde 11), beschränkt geschlossen am 260816-2030
**Kohärent abgeschlossenes Kind:** `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`
(Runde 12), kohärent geschlossen am 260818

Zwei Runden sind seit dem Vermerk vom 260815-0350 geschlossen worden, und eine dritte ist
zurückgestellt. Die Auslösebedingung ist wie bei den Vermerken davor zur Hälfte erfüllt: der
Grounding-Abschnitt dieses Circles stammt vom 260804 und zitiert keine der beiden Runden, wohl
aber Bauteile, die beide angefasst haben. Vier Punkte gehören in die Klärungsrunde bei der
Aktivierung, und der fünfte ist eine Berichtigung an diesem Datensatz selbst. Keiner hält die
Aktivierung auf.

### 1. Das Halteverhalten des Tabs gilt seit der Runde 11 nicht mehr für laufende Arbeit

Die Directive dieses Circles setzt auf C6 auf und sagt zu: „die nächste Auswahl im Dateifenster
ersetzt seinen Inhalt im aktiven Tab, ein anderer Tab behält ihn stehen." Für einen fertig
angezeigten Inhalt stimmt das weiter. Für **laufende** Arbeit stimmt es seit der Runde 11 nicht
mehr: ein Tabwechsel beendet den Durchlauf des verlassenen Tabs, so entschieden vom Nutzer am
260816-1410
(`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/decisions/260816-1359_*_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md`).
Der Rückwechsel stößt ihn nicht wieder an; das ist als Befund abgelegt und offen
(`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/issues/260816-1710_*_ein-rueckwechsel-auf-einen-tab-setzt-seinen-beendeten-durchlauf-nicht-fort.md`).

Das Laden einer Web-Seite ist laufende Arbeit, und der Betrachter erbt damit eine Frage, die es
am 260804 nicht gab: Wechselt der Nutzer während des Ladens den Tab, bricht das Laden ab oder
läuft es weiter? Beide Antworten sind vertretbar — die Runde 11 hat für das Lesen von Dateien
abgebrochen, weil ein Durchlauf über den Unterbaum Deskriptoren hält —, und der
Aktivierungs-Spec muss eine wählen, statt sich auf das Halteverhalten aus C6 zu berufen. Der
Berufungssatz ist nach der Runde 11 nicht mehr eindeutig.

### 2. Die Statuszeile trägt weiter sechs Ränge, und das ist nachgezählt

Der Vermerk vom 260815-0350 nannte den siebten Rang für die Meldungen des Betrachters. Die
Runde 11 hat trotz ihres Lesefortschritts **keinen** achten hinzugefügt: sie schreibt in den
vorhandenen `Rang::Filterstand`. Am 260818 nachgezählt trägt `Rang`
(`crates/krk-ui/src/appkit/statuszeile.rs:207`) unverändert sechs Werte. Der Punkt aus dem
Vermerk vom 260815-0350 steht damit unverändert und ist nicht teurer geworden: die Meldungen des
Betrachters wären der siebte Rang, und die Aufzählung ist ohne Auffangzweig, der Bau erzwingt
also die Einordnung.

### 3. Ein Blatt als Adresseingabe kostet seit der Runde 12 mehr als eine Fläche

Die zweite Möglichkeit der ersten offenen Frage oben — „ein Tastenbefehl öffnet ein Blatt am
Fenster, wie die Pfadeingabe aus C2" — trifft seit der Runde 12 auf einen Blattbauer, an dem
zwei Fragen offen stehen und ein Befund liegt.

Der Bauer beantwortet die Frage „welche Schaltfläche ist die ungefährliche" über
`abbruchstelle` und muss dabei eine mitbeantworten, die keine Antwort hat: welche es ist, wenn
keine es ist. Ob er die liegenlassende Schaltfläche künftig am Typ verlangt, ist eine offene
Nutzerfrage
(`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/decisions/260818-0250_*_verlangt-der-blattbauer-die-liegenlassende-schaltflaeche-am-typ.md`).
Daneben steht ein offener Befund: `Blatt::zeigen` setzt „bestätigt" mit der ersten Schaltfläche
gleich, und die im Kommentar genannte Vorbedingung ist schwächer als die wirkliche
(`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260818-0410_*_blatt-zeigen-answers-which-button-confirms-a-second-time-and-names-a-weaker-precondition.md`).

Ein Adressblatt hätte eine ausführende erste Schaltfläche und wäre genau der Fall, den der
Befund als noch nicht eingetreten beschreibt. Wer die zweite Möglichkeit wählt, wählt damit
mit, dass diese beiden Punkte vorher geklärt sind. Die erste Möglichkeit — Zwischenablage und
Verweisanker der Seite — ist davon nicht berührt.

### 4. Was die Runde 12 aufgehoben hat, berührt die Grundlage dieses Circles nicht

Das steht hier, damit niemand danach sucht. Die Runde 12 hat eine umgesetzte Nutzerfestlegung
der Runde 1 überholt: `shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`
trägt seither überholt (`_s_`), und C6 jener Runde hat Directive und Spec der Runde 1 an sieben
Stellen nachgezogen. Am 260818 gegen den Baum gelesen: **keine der drei Stellen, die dieser
Circle unter `## Dependencies` aus der Runde 1 zitiert, ist davon berührt.** Die Fähigkeit C10
des Specs steht unverändert, der Nutzerentscheid vom 260804 zur Zwischenablage-Auswertung steht
unverändert, und der Ausschluss „Integrierter Browser zum Navigieren von Websites" steht
unverändert als erster Punkt in `### Ausdrücklich außerhalb dieses Circles` des Circle-Datensatzes
der Runde 1. Alle drei Zitate lösen auf.

Zwei Zahlen dazu, ebenfalls am 260818 nachgezählt: `Kommando` trägt 78 Varianten und
`resources/default-keymap.toml` 84 Einträge, in der Summe unverändert seit dem 260815. Die
Zusammensetzung hat sich bewegt — das endgültige Löschen ist weggefallen, das Ankreuzfeld
„Content" hinzugekommen, `f8` liegt jetzt auf dem Papierkorb und `opt+cmd+delete` ist frei.

### 5. Dieser Datensatz trägt einen toten Zeiger, und er steht in der Grundlage

Zeile 438 zitiert eine Datei, deren Namensteil es nie gegeben hat. Das Zitat steht bereits in der
Sternform; falsch ist der Slug. Abgelegt als
`shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`,
mit dem richtigen Namen und dem Beleg, dass er nie gewechselt hat. Der Ort macht ihn teuer: die
Zeile begründet, dass L7 seit der Runde 6 schon ab rund 12 kB verfehlt wird statt ab 19 kB, und
der Abschnitt `## Grounding snapshot` wird bei der Aktivierung als bindende Grundlage gelesen. Wer
den Beleg nachschlägt, findet nichts und kann nicht unterscheiden, ob der Datensatz gelöscht wurde
oder ob der Zeiger falsch ist. Der Playmaker schreibt keine Circle-Datensätze außer den drei
Abschnitten, die sein Auftrag nennt; die Berichtigung der einen Zeile ist Nutzer- oder
Coder-Arbeit und gehört vor die Aktivierung.

## Activation proposal

**Vorgeschlagen am:** 260818-1018
**Playmaker-Lauf:** 260818-1018-playmaker-orchestrator-phase4
**Vorgeschlagener Aktivierungszeitpunkt:** der nächste `/fusion:next` des Nutzers; dieser Lauf
ist eine Phase-4-Zuteilung des Orchestrators und hält keine Bestätigung.

**Rang 1 von 1.** Dieser Circle ist weiterhin der einzige vorgesehene (`_a_`) des Projekts, und
die Rangfolge ist damit keine Leistung der Heuristik. Was die Heuristik beiträgt, ist die Prüfung
der Vorbedingungen, und sie fällt weiter sauber aus: ein einziger offener Entscheidungsdatensatz
bindet ihn
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
seine Abhängigkeiten führen auf die Runden 1, 5, 6 und 7, alle terminal und alle am Baum gebaut,
und die Bauteile, auf denen er aufsetzt — die Zwischenablage-Auswertung aus S13 und das
Vorschaufenster aus S19 — stehen. Der Abzug für beschränkten statt kohärenten Abschluss der
Abhängigkeiten ist wie in allen Läufen davor nicht angesetzt: `CLAUDE.md` weist den Marker `_b_`
in diesem Projekt ausdrücklich als Auskunft über die Verfügbarkeit des Nutzers für den
Abnahmelauf aus und nicht über die Reife einer Runde.

**Was sich seit dem 260815-0350 geändert hat.** Zwei Runden sind geschlossen, die elfte
beschränkt und die zwölfte kohärent, und eine dritte ist zurückgestellt, ohne gefahren worden zu
sein. Der Vermerk darüber steht unmittelbar über diesem Abschnitt; seine Punkte 1 und 3 sind
neue Arbeit für die Klärungsrunde, Punkt 2 bestätigt einen alten Punkt und Punkt 4 nimmt eine
Sorge weg. Punkt 5 ist eine Berichtigung an diesem Datensatz, die vor die Aktivierung gehört.

**Was vor der Aktivierung steht, unverändert seit dem 260804.** Erstens eine Untersuchung des
Darstellungsmittels: der Circle legt bewusst nicht fest, womit KRK Web-Inhalt darstellt, weder
eine Systemschnittstelle noch eine fremde Kiste, und die offene Frage zur
Verfügbarkeitsprüfung für Schnittstellen ab macOS 26 hängt genau daran. Zweitens die
Klärungsrunde über die drei offenen Fragen oben, von denen die erste den Zuschnitt entscheidet:
Bleibt es bei der Zwischenablage und den Verweisankern der Seite, bekommt KRK einen Betrachter;
kommen Adresseingabe und gespeicherte Web-Adressen hinzu, bekommt es einen Browser.

**Ein zweiter Kandidat steht daneben, und er ist nicht vorgesehen.** Die Runde
`260816-2255-befehle-absetzen-und-makros-speichern` ist am 260817-0445 zugunsten der zwölften
Runde zurückgestellt worden, mit vollständigem Spec (54 Abnahmekriterien) und vollständigem Plan
(22 Schritte). Zurückgestellt (`_d_`) ist ein Endzustand; eine Aufnahme ist ein neuer Circle, der
den zurückgestellten über `## Dependencies` zitiert. Der Playmaker rankt ihn nicht mit, weil er
nicht vorgesehen ist, nennt ihn aber, damit die Wahl zwischen den beiden dem Nutzer offensteht
und nicht durch das Schweigen des Portfolios vorentschieden wird.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer über
`/fusion:next` oder beim Orchestrator.
