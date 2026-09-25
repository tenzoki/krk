# Wie erfährt der Nutzer, dass eine Ablagedatei zur Seite gelegt wurde?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-core/src/ablage/mod.rs:126-160` (`Ersetzung`, `melden`); `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` (Möglichkeit 1 gewählt: Statuszeile); `crates/krk-ui/src/appkit/anwendung.rs:917` (`sitzung_laden`, sammelt die Meldungen)

---

## Question

Der Meldeweg besteht schon. `Ablage::laden` liefert bei einer beschädigten Datei eine `Ersetzung` mit dem Dateipfad und dem Grund; `melden` macht daraus einen Satz, und die Oberfläche stellt ihn in die Statuszeile am Fuß des Dateifensters. Der Nutzer hat diesen Weg am 260804-0830 gewählt.

Der Satz lautet heute sinngemäß: die Datei ist beschädigt und wird durch den Auslieferungszustand ersetzt. Mit Festlegung D stimmt er nicht mehr, denn die Datei wird nicht mehr nur ersetzt, sondern zur Seite gelegt, und der Nutzer kann sie zurückholen. Diese Auskunft ist die eigentliche Nachricht, und sie geht in einer Statuszeile leicht unter: sie steht am Fuß eines Dateifensters, verschwindet bei der nächsten Meldung und wird beim Start gezeigt, wenn der Nutzer noch gar nicht hinsieht.

Die Frage ist zu stellen, weil der Verlust der Lesezeichen genau der Schaden ist, den diese Runde verhindern soll. Eine Warnung, die niemand liest, verhindert ihn nicht.

Sie hält keinen Planschritt auf und bindet einen.

## Options

1. **Derselbe Weg, umgeschriebener Satz.** Die Meldung geht weiter in die Statuszeile und nennt jetzt den Pfad der zur Seite gelegten Datei.
   - Folge: kein neuer Mechanismus. Der Satz nennt beide Pfade, und wer ihn liest, weiß, wo seine Lesezeichen liegen.
   - Preis: die Meldung erscheint beim Start, wenn der Nutzer auf das Fenster schaut, das gerade aufgeht, und sie ist nach der ersten Ordneränderung weg. Wer sie verpasst, erfährt nichts; die Datei liegt dann still in einem Ordner, den niemand ansieht.

2. **Ein Blatt am Fenster.** Beim Start meldet ein Blatt, dass eine Ablagedatei beschädigt war, wo sie liegt und dass KRK mit dem Auslieferungszustand weiterarbeitet. Der Nutzer bestätigt.
   - Folge: die Nachricht kommt an. KRK führt Blätter bereits, `crates/krk-ui/src/appkit/blaetter/` hält sie, und die Nachfrage aus C4 der Runde 2 ist die Vorlage.
   - Preis: ein Blatt beim Start ist die aufdringlichste Form, die dieses Programm kennt, und es steht vor der Arbeit. Bei einer Fassung, die die Datei bei jedem Start als beschädigt liest, kommt es bei jedem Start. Daneben hält ein stehendes Blatt jeden Tastenbefehl außer dem Abbruch an (`Anwendungsdelegierter::kommando_ausfuehren`), was hier zwar gewollt wäre, aber die Startzusage L4 berührt.
   - `inference:` Ob ein Blatt beim Start L4 aus C8 verfehlt, also die 1000 ms bis zum bedienbaren Fenster, ist ungemessen. Ein Blatt geht auf, nachdem das Fenster steht; die Messstrecke misst bis zur Bedienbarkeit, und ein Blatt macht das Fenster gerade nicht bedienbar.

3. **Statuszeile beim Start, Blatt nur bei den Lesezeichen.** Die drei übrigen Dateien melden wie heute, `bookmarks.toml` bekommt das Blatt.
   - Folge: die aufdringliche Form trifft die eine Datei, deren Verlust der Nutzer ausdrücklich benannt hat, und lässt die drei anderen in Ruhe. `keymap.toml` ist von Hand änderbar und trägt Tippfehler von selbst; `session.toml` und `settings.toml` sind ersetzbar.
   - Preis: eine Sonderregel für eine von vier Dateien, und die Festlegung D sagt ausdrücklich, dass alle vier gleich behandelt werden. Sie sagt es allerdings über das Zur-Seite-Legen und nicht über die Meldung.

## Constraints

- Der Kern gibt nichts aus. `melden` liefert einen Text zurück und schreibt ihn nirgendwohin; die Aufrufrichtung bleibt von oben nach unten (`crates/krk-core/src/ablage/mod.rs`, Abschnitt „Der Kern gibt nichts aus"). Eine Antwort darf daran nichts ändern.
- Ein stehendes Blatt weist jedes Kommando außer dem Abbruch ab, und die Regel dafür steht an genau einer Zeile in `kommandos::operationen::waehrend_blatt_erlaubt`. Wer ein Startblatt baut, arbeitet mit dieser Sperre und nicht gegen sie.
- Die Wahl vom 260804-0830 für die Statuszeile bleibt der Regelweg für Fehler. Eine Antwort, die alles ins Blatt verschiebt, hebt sie auf und bräuchte dafür einen eigenen Grund.

## Recommendation

**Wir empfehlen Möglichkeit 1**, und zwar mit einem Satz, der zuerst sagt, was der Nutzer tun kann, und danach, was geschehen ist. Der Grund gegen das Blatt ist nicht die Aufdringlichkeit, sondern die Wiederholung: eine Fassung, die eine alte Datei nicht mehr liest, erzeugt den Fall bei jedem Start, und ein Blatt, das bei jedem Start kommt, wird nach dem dritten Mal weggeklickt, ohne gelesen zu werden.

Wir halten allerdings fest, dass Möglichkeit 1 die Nachricht nicht sicher zustellt. Wenn dem Nutzer die sichere Zustellung wichtiger ist als die Ruhe beim Start, ist Möglichkeit 3 die bessere Wahl; ihre Sonderregel für eine von vier Dateien ist der ehrlichere Preis als ein Blatt, das jedes Mal aufgeht.


## Antwort 260812-1105

**Moeglichkeit 1, Nutzerentscheid, mit Folge.**

**Nutzerentscheid vom 260812-1105: die Statuszeile beim Start**, mit einem Satz, der zuerst sagt,
was der Nutzer tun kann, und danach, was geschehen ist. Der Satz nennt beide Pfade.

Kein Blatt. Eine Fassung, die eine alte Datei nicht mehr liest, erzeugt den Fall bei jedem Start,
und ein Blatt, das bei jedem Start kommt, wird nach dem dritten Mal weggeklickt, ohne gelesen zu
werden. Dazu haelt ein stehendes Blatt jeden Tastenbefehl ausser dem Abbruch an und beruehrt damit
die Startzusage L4.

**Der Nutzer hat die Wahl an eine Bedingung geknuepft, und die vergroessert diese Runde.** Er
weist darauf hin, dass die Statuszeile heute nur unter einem Dateifenster steht und damit fuer die
meisten Meldungen zu schmal ist. Seine Vorgabe: die Statusmeldung muss die volle Fensterbreite
nutzen, und es muss moeglich sein, nach rechts zu blaettern. Das ist als fuenfte Faehigkeit dieser
Runde aufgenommen und ueberholt eine Entscheidung der Runde 5; der Datensatz dazu ist
`260812-1105_a_die-statuszeile-zieht-ueber-die-volle-fensterbreite-und-laesst-sich-blaettern.md`.

Die im Datensatz benannte Schwaeche von Moeglichkeit 1 bleibt bestehen: die Meldung stellt sich
nicht sicher zu, sie ist nach der ersten Ordneraenderung weg.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented: `crates/krk-core/src/ablage/mod.rs:191` (`impl fmt::Display for Ersetzung`, der Satz nennt beide Pfade), `:248` (`melden` als der eine Weg des Kerns, ein Rueckgabewert und keine Ausgabe) und `:273` (`mit_meldung`); verbraucht in `crates/krk-ui/src/appkit/anwendung.rs:1007-1021` und ausgegeben ueber die Startmeldungen bei `:931-940`. Kein Blatt im Weg — Schritt 1 des Plans, Commit `755571a`. Abgeglichen am 260812-2253. Am laufenden Buendel bleibt C3.10 zu sehen.
Deferred:
Superseded by:
