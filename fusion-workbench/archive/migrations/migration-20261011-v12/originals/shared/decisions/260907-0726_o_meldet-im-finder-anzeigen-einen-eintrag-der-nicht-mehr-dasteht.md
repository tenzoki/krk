# Meldet „Im Finder anzeigen" einen Eintrag, den es nicht mehr gibt?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-ui/src/appkit/finder.rs` (Modulkopf, Abschnitt „Der Aufruf gibt nichts zurück"), `crates/krk-ui/src/appkit/anwendung.rs` (`im_finder_anzeigen`), `crates/krk-ui/src/kommandos/operationen.rs` (`ordner_fehlt`, `kein_finder`)

---

## Question

Der neue Eintrag „Im Finder anzeigen" (Runde 24) deckt die betroffenen Einträge über
`NSWorkspace::activateFileViewerSelectingURLs:` auf. **Dieser Aufruf liefert `void`.** Es gibt
weder einen Rückgabewert wie bei `NSWorkspace::openURL:` noch einen Rückruf wie bei
`openURLs:withApplicationAtURL:configuration:completionHandler:`. Ein Scheitern ist an ihm nicht
feststellbar.

Der Auftrag verlangte, ein gescheiterter Aufruf solle melden. Gebaut ist deshalb, was **vor** dem
Aufruf entscheidbar ist: die leere Menge (`nichts_anzuzeigen`) und die Frage, ob das System
überhaupt einen Finder nennt (`terminal::anwendung_vorhanden` → `kein_finder`). Die zweite Antwort
ist notwendig und nicht hinreichend.

**Was damit offenbleibt:** ein Pfad, den es nicht mehr gibt. Der Finder kommt in den Vordergrund
und wählt ihn nicht aus; kein Wort erreicht den Nutzer. Der Fall entsteht am ausgeworfenen
Datenträger und an jedem Eintrag, der zwischen dem letzten Verzeichnisdurchgang und dem Klick
verschwunden ist. **Der Nachbareintrag „Im Finder öffnen" hat genau dafür eine Vorprüfung**
(`operationen::ordner_fehlt`), und ihre Begründung steht dort ausgeschrieben: der Rückruf bleibt
leer, also ist die Prüfung davor die eine Gelegenheit, dem Nutzer etwas zu sagen, das er beheben
kann.

## Options

1. **So lassen.** Der Finder verhält sich bei einem verschwundenen Eintrag wie bei jedem anderen
   Werkzeug, das ihn aufdeckt.
   - Pros: nichts zu bauen; kein Zugriff auf die Platte bei jedem Klick.
   - Cons: ein Befehl, der still weniger tut, als der Nutzer markiert hat — genau der Ausgang, den
     dieser Baum an anderen Stellen ausdrücklich schließt (`kein_finder`, `ablage_weist_ab`,
     `abschlusstext` mit seiner Zahl der ausgelassenen Einträge).
2. **Den angezeigten Ordner vorprüfen**, über `operationen::ordner_fehlt`, dessen dritter Rufer
   dieser Zweig dann wäre.
   - Pros: eine Prüfung je Klick statt einer je Eintrag; fängt den Fall, der zählt (ausgeworfener
     Datenträger, gelöschter Ordner); die Meldung steht schon da.
   - Cons: der Doc-Kommentar jener Funktion sagt heute, gefragt werde von jedem Befehl, der einen
     Ordner an eine über ihre Bündelkennung **benannte** Anwendung übergibt — dieser tut das nicht,
     der Satz müsste weiter gefasst werden. Ein einzelner verschwundener Eintrag in einem noch
     stehenden Ordner bleibt weiter stumm.
3. **Jeden betroffenen Eintrag vorprüfen** (`symlink_metadata`, damit eine tote Verknüpfung als
   vorhanden gilt) und die Zahl der ausgelassenen in der Statuszeile nennen, nach dem Vorbild von
   `abschlusstext`.
   - Pros: vollständig; kein stiller Verlust.
   - Cons: ein Dateisystemzugriff je markiertem Eintrag bei jedem Klick; ein neuer Meldungstext;
     die Teilmenge, die aufgedeckt wird, weicht dann von der markierten ab, und der Nutzer liest
     zwei Angaben statt einer.

## Constraints

Was die Antwort auch sei, sie steht **vor** dem Aufruf: nach ihm antwortet niemand mehr. Und sie
darf keine zweite Auflösung einer Bündelkennung anlegen — `terminal::anwendungsort` ist die eine
Stelle des Programms, die eine auflöst, und `crates/krk-ui/src/appkit/terminal.rs` sagt es in
seinem Kopf zu.

## Recommendation

Möglichkeit 2. Sie fängt den Fall, der in der Praxis vorkommt, zum Preis eines einzigen `metadata`
je Klick, und sie stellt die zwei Finder-Einträge auf dieselbe Vorprüfung — heute prüft der eine
und der andere nicht, ohne dass ein Unterschied in der Sache das trüge. Möglichkeit 3 kauft
Vollständigkeit mit einem Zugriff je Eintrag und einem weiteren Satz in der Statuszeile; das lohnt
erst, wenn der Fall des einzelnen verschwundenen Eintrags jemandem begegnet.
