Je Sicherungsmoment wird höchstens ein Zettel geschrieben, und beim Beenden gibt es kein nächstes Mal

---

`Zettelmodell::zu_sichern` (`crates/krk-ui/src/zettelmodell.rs:192`) liefert
`Option<(Zettel, &str)>` und damit den **ersten** abweichenden Zettel in der Reihenfolge
von `Zettel::ALLE`. `Anwendungsdelegierter::zettel_sichern`
(`crates/krk-ui/src/appkit/anwendung.rs:3440`) fragt einmal und schreibt einmal. Weichen
beide Zettel ab, schreibt ein Sicherungsmoment also nur einen von ihnen.

Der Doc-Kommentar an `zu_sichern` nennt das und verweist auf die Fortsetzung: „der nächste
Sicherungsmoment nimmt den zweiten." Für den vierten Moment stimmt das nicht:
`applicationWillTerminate:` (`:842`) ist der letzte, der läuft. Der Text des zweiten
Zettels ist danach fort, ohne Meldung — der Rückgabewert wird dort mit `let _ =`
fallengelassen, weil es keine Statuszeile mehr gibt.

---

**Schwere:** mittel. Datenverlust, aber nur nach einer vorher schon gescheiterten
Sicherung.

**Der Weg dorthin.** Beide Zettel weichen gleichzeitig nur ab, wenn ein Schreibvorgang
gescheitert ist: Zettel 1 ändern, auf Tab 2 klicken, die Sicherung von Zettel 1 scheitert
(kein Ablageordner, kein Schreibrecht, Sperre nicht zu nehmen), in Zettel 2 tippen, `cmd+q`
bei sauberem Editor. `applicationWillTerminate:` schreibt Zettel 1 — falls es jetzt geht —
und Zettel 2 nie.

**Beim Beenden fehlt daneben jede Meldung.** Der Bau begründet das damit, dass es keine
Statuszeile mehr gibt, und das trägt für den Weg, den die Meldung nehmen würde. Es trägt
nicht als Aussage über den Verlust: der Nutzer erfährt an keiner Stelle, dass sein Zettel
nicht auf der Platte steht.

**Ein Lösungsweg, nicht der einzige.** `zettel_sichern` läuft über beide Zettel statt über
den ersten abweichenden — die Schleife kostet nichts, weil ein unveränderter Zettel
ohnehin nichts schreibt, und `zu_sichern` bleibt als Frage nach „gibt es überhaupt etwas"
bestehen. Damit verschwindet zugleich die Sonderstellung des vierten Moments.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Hängt am Datensatz
  `260814-0908_o_ein-neuoeffnen-nach-gescheiterter-sicherung-wirft-den-ungesicherten-zettelstand-weg.md`:
  beide werden von derselben gescheiterten Sicherung ausgelöst, und wer nur einen von
  beiden behebt, lässt den anderen Weg offen.

---
Resolved: Jeder Sicherungsmoment schreibt jetzt jeden abweichenden Zettel.
`Zettelmodell::zu_sichern` (`crates/krk-ui/src/zettelmodell.rs`) liefert nicht mehr den
ersten abweichenden, sondern jeden, in der Reihenfolge von `Zettel::ALLE`; die Frage „gibt
es überhaupt etwas" beantwortet `etwas_zu_sichern` daneben, und zwar aus derselben
Aufzählung abgeleitet und nicht neben ihr gebaut. `Anwendungsdelegierter::zettel_sichern`
(`crates/krk-ui/src/appkit/anwendung.rs`) läuft über die Liste und schreibt jeden einzeln.
Es bleibt bei **einer** Erklärung und bei **vier** Aufrufern; die zwei Zählproben
`das_sichern_des_zettels_ist_genau_einmal_erklaert` und
`genau_vier_stellen_sichern_den_zettel` sind unangetastet grün.

Ein Fehlschlag bricht die Schleife nicht ab, denn er sagt über den anderen Zettel nichts.
In die Statuszeile geht der erste Grund: die Zeile trägt einen Satz, und scheitern beide
Zettel, so scheitern sie am selben Hindernis — kein Ablageordner, kein Schreibrecht, die
Sperre nicht zu nehmen.

Die Sonderstellung des vierten Moments ist damit weg: `applicationWillTerminate:` schreibt
beide Zettel, und es bleibt bei **einem** `durchgang` durch die Ablage. Der Kommentar dort
nennt jetzt beides — dass dieser Moment der letzte ist und deshalb jeden abweichenden
Zettel schreiben muss, und warum das `let _ =` davor steht.

**Die fehlende Meldung beim Beenden ist nicht behoben, und zwar mit Absicht.** Der Spec
bindet die Meldezusage ausdrücklich an die drei Momente, nach denen KRK weiterläuft, führt
den Verlust unter C4 als benannten und angenommenen Preis und die Alternative unter
„Ausdrücklich außerhalb dieser Runde": eine Meldung an dieser Stelle wäre eine Rückfrage
beim Beenden, die diese Runde nicht führt. Wer sie will, öffnet eine eigene Runde. Der
Kommentar im Code schreibt diesen Grund jetzt aus, statt ihn bei der fehlenden Statuszeile
enden zu lassen.

Der Datensatz hängt an `260814-0908_c_ein-neuoeffnen-nach-gescheiterter-sicherung-…`, und
beide sind in einem Zug behoben: die Zusage „jeder abweichende Zettel" folgt daraus, dass
ein abweichender Stand seit jener Behebung das Öffnen und das Schließen des Blattes
überdauert. Die Probe `jeder_abweichende_zettel_steht_zur_sicherung_an` hält den Fall am
Modell fest, ohne Fenster. Der Plan ist an sechs Stellen nachgezogen
(`planning/260814-0656_o_plan-…`, Kopfnotiz vom 260814-0941).

`make check` am 260814-0947 gefahren, Rückgabewert 0, „alle vier gruen".
