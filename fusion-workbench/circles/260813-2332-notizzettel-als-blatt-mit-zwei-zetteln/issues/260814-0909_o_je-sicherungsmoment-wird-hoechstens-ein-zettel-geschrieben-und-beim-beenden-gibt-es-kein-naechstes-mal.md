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
