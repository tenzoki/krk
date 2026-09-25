Ein Deskriptormangel beendet den ganzen Durchlauf still, und die Statuszeile nimmt dabei genau den Hinweis zurück, der die Liste als unfertig auswies

---

C1.11 sagt: „Ein geschlossener Befundkanal ohne weitere Meldung heißt nicht, dass die
übrigen Dateien keinen Treffer tragen. Er heißt, dass sie nicht entschieden sind." Das
Modell hält diese Bedeutung ein — ein fehlender Befund bleibt `Befund::Unentschieden`, und
`inhalt_entscheidet` lässt die Zeile weg. **Dem Nutzer wird die Bedeutung aber nicht
gezeigt, und seit dieser Runde wird sie ihm sogar entzogen.**

**Der Weg, Zeile für Zeile.** Ein `EMFILE` oder `ENFILE` beim Öffnen einer Datei liefert
`Inhaltsbefund::Unentschieden` (`crates/krk-core/src/verzeichnis/inhalt.rs:151`),
`datei_entscheiden` macht daraus `None` (`durchlauf.rs:401`), und `None` beendet nicht nur
diesen Auftrag, sondern den **ganzen Faden**: im flachen Zweig über
`let Some(treffer) = entschieden else { return; }` (`durchlauf.rs:356-358`), im tiefen über
`datei_entscheiden(...)?` (`durchlauf.rs:527`). Der Faden endet, der Kanal schließt.

Am Hauptfaden sieht `befunde_einziehen` den geschlossenen Kanal und räumt den Lauf weg
(`crates/krk-ui/src/tabs.rs:1145-1159`):

```rust
Err(TryRecvError::Disconnected) => { kanal_zu = true; break; }
...
if kanal_zu { tab.durchlauf = None; }
```

Damit fällt `Tabinhalt::liest_inhalt` auf `false` (`tabs.rs:169-171`, `durchlauf.is_some()`
ist die erste seiner zwei Bedingungen), und `filterstand_text` lässt den Satzteil
„, Inhalt wird gelesen" weg (`crates/krk-ui/src/appkit/statuszeile.rs:430-434`). Der
Nutzer sieht danach eine Statuszeile, die von einem fertigen Filterstand nicht zu
unterscheiden ist: „Filter „notiz": 12 von 4.812 angezeigt". Die 12 sind aber kein
Ergebnis, sondern der Stand beim Abbruch.

**Warum das jetzt wiegt und vorher nicht.** Vor dieser Runde konnte der Durchlauf nur beim
**Öffnen eines Ordners** an einem Deskriptormangel enden, und ein Namensdurchlauf über
Verzeichnismetadaten ist nach dem eigenen Text der Runde 10 „in Millisekunden durch"
(`tabs.rs:836-838`). Jetzt läuft er über denselben knappen, prozessweit geteilten Vorrat
minutenlang und fragt ihn zusätzlich je gelesener Datei. Das Zeitfenster, in dem ein
Deskriptormangel von Editor, Vorschau, Kopiervorgang oder dem zweiten Dateifenster den
Lauf trifft, wächst um Größenordnungen — und der Ausgang ist eine still verkürzte Liste.

**Was der Baum schon leistet und was nicht.** Die Kindprobe
`ein_deskriptormangel_beim_lesen_laesst_die_datei_unentschieden`
(`crates/krk-core/tests/verzeichnis.rs:2178-2210`) belegt unter `ulimit -n 64`, dass der
Kern richtig antwortet, nämlich mit gar keiner Meldung. Sie sagt nichts über die
Oberfläche, und die Oberfläche hat für diesen Ausgang keinen Zweig.

---

**Zusammenhang mit dem bekannten Befund.** `issues/260816-1710_o_ein-rueckwechsel-auf-einen-tab-…`
beschreibt dieselbe Sorte Anzeige mit anderem Auslöser: dort endet der Lauf, weil der
Nutzer den Tab verlassen hat, hier, weil der Prozess keine Deskriptoren mehr hatte. Eine
Behebung, die die Anzeige „dieser Filterstand ist unvollständig" einführt, deckte beide
Auslöser mit einer Regel ab; eine, die nur den Rückwechsel wieder anstößt, deckt diesen
Fall nicht — nach einem Deskriptormangel gibt es keinen Tabwechsel, der ihn neu anstößt.

Gefunden bei der Durchsicht der elften Runde, Bereich `9f5ced5..b9ab8ae`.
