Die einzige Sicherung gegen den Polaritätsfehler ist Prosa, und `ist_warnwuerdig` hat heute keinen Aufrufer

---

`Befund` trägt zwei Polaritäten: bei der Papierkorbfrage ist `Ja` die Erlaubnis, bei den
Zieltests des Bündels C ist `Ja` der Warngrund. Drei Modulköpfe warnen davor,
`ist_warnwuerdig` an die erste zu halten. Keine Probe und kein Typ hält es davon ab, und
`ist_warnwuerdig` hat im ganzen Baum noch keinen Aufrufer.

---

**Schwere:** Mittel. Am heutigen Baum liegt kein Verwendungsfehler vor — es gibt nur einen
Verwender, und der liegt richtig. Der Befund ist die Sicherung: Bündel C bringt die erste
Aufrufstelle von `ist_warnwuerdig`, und wer sie schreibt, hat drei Prosaabsätze und keine rote
Probe als Widerstand. Der Fehler, den sie verhindern sollen, macht aus „wir wissen nichts über
das Ziel" die Erlaubnis zu löschen und nimmt C4 seine Zusage.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `crates/krk-core/src/verzeichnis/befund.rs:50-68`, `:125`,
`crates/krk-ui/src/kommandos/loeschwarnung.rs:61-69`,
`crates/krk-ui/src/appkit/papierkorb.rs:49-59`
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

`ist_warnwuerdig` hat keine Aufrufstelle. Nachgezählt mit
`grep -rn "ist_warnwuerdig" crates/`: sechs Treffer, drei davon in `befund.rs` selbst (die
Erklärung `:125` und zwei Proben), drei außerhalb — und alle drei stehen in einem
Doc-Kommentar:

| Stelle | Was dort steht |
|---|---|
| `loeschwarnung.rs:66` | „`Befund::ist_warnwuerdig` kommt in dieser Datei nicht vor, und das ist Absicht" |
| `loeschwarnung.rs:458` | „wer aus Gewohnheit `Befund::ist_warnwuerdig` nimmt, macht aus ‚wir wissen nichts' die Erlaubnis zu loeschen" |
| `papierkorb.rs:53` | „`Befund::ist_warnwuerdig` ist hier folglich das falsche Werkzeug" |

Die erste dieser drei ist eine **Aussage über den Baum** und keine Begründung: „kommt in
dieser Datei nicht vor". Genau diese Sorte Aussage hält dieses Projekt mit einer Zählprobe,
und `loeschwarnung.rs` trägt zweihundert Zeilen darunter eine
(`die_stufenregel_hat_genau_einen_aufrufer`, `:362`), die `crate::quellbaum::aufrufstellen`
schon benutzt. Der Modulkopf von `quellbaum` schreibt die Bauform aus.

Die fünf Proben in `befund.rs` prüfen die Tafel und die Lautheit. Keine prüft, **wo**
`ist_warnwuerdig` gerufen werden darf, und das ist die Frage, an der der Fehler hängt.

## Was daran heute schon zutrifft und was nicht

Geprüft und in Ordnung: die eine Verwendung liegt auf der richtigen Polarität.
`vor_der_rueckfrage` (`loeschwarnung.rs:242-246`) prüft auf `Befund::Ja` selbst und führt
`Nein` und `Unentschieden` ausgeschrieben in denselben Zweig; die Probe
`ohne_papierkorb_erscheint_kein_blatt` (`:462`) hält beide Zeilen einzeln fest und benennt in
ihrem Doc-Kommentar, dass ihre zweite Zusicherung die eigentliche ist.

Offen ist die Sicherung nach vorn. Die zweite Polarität existiert im Baum noch nicht, also
gibt es heute keine Stelle, an der eine Probe einen Fehler fangen könnte — und genau deshalb
ist jetzt der Zeitpunkt, an dem die Sicherung geschrieben wird und nicht nach dem ersten
Verwender.

## Richtung

Zwei Wege, und der zweite ist der stärkere.

1. **Eine Zählprobe.** In `loeschwarnung.rs` und `papierkorb.rs` je eine Zusicherung, dass
   `aufrufstellen(inhalt, "ist_warnwuerdig") == 0` gilt, mit zusammengesetzter Nadel wie bei
   der Vorlage. Billig, hält gegen den Gewohnheitsfehler und macht die drei Prosaaussagen
   prüfbar.
2. **Zwei Typen für zwei Fragen.** Die Polarität gehört zur Frage und nicht zum Wert, und
   `befund.rs:67-68` sagt das selbst: „welche das ist, haengt an der Frage und nicht am Typ".
   Ein `Warnbefund` mit `ist_warnwuerdig` und ein `Erlaubnisbefund` mit `erlaubt` machen den
   Fehler unübersetzbar; der gemeinsame Rumpf bleibt eine Aufzählung mit drei Werten. Kosten:
   ein Typ mehr und eine Umrechnung an vier Prüfstellen des Bündels C.

Bündel C berührt beide Dateien ohnehin (Schritte 9 und 10), also fällt der Schnitt dort ohne
zweite Änderung an derselben Stelle.

Hinweis (260817, Aufgabe T5b): Der Typ heißt seit `260817-1419_*_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`
nicht mehr `Befund`, sondern `Loeschzielbefund`, und die Datei heißt
`crates/krk-core/src/verzeichnis/loeschzielbefund.rs`. Der Modulkopf ist dabei um den Abschnitt
`# Warum der Typ nicht Befund heisst` gewachsen, die Zeilenangaben unter **Betroffen** sind
damit verschoben. Der Befund selbst besteht unverändert: die Sicherung gegen den
Polaritätsfehler ist weiterhin Prosa in drei Modulköpfen.
