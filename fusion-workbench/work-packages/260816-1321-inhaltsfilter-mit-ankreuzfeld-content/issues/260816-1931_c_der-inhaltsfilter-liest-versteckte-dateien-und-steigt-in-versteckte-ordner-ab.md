Der Inhaltsfilter liest versteckte Dateien, deren Zeile nie stehen kann, und steigt in versteckte Ordner ab

---

Die teuerste Zusage der Runde lautet, dass gelesen wird, was gelesen werden soll. Drei
Grenzen dieser Zusage halten am Baum: keine Datei mit passendem Namen wird geöffnet,
unterhalb der Schwelle wird gar nicht gelesen, und eine Datei über 1 MB wird nicht gelesen.
Eine vierte Menge ist nirgends abgewogen worden: **die versteckten Einträge.**

**Erstens, die Auftragsliste nimmt sie mit.** `auftraege`
(`crates/krk-ui/src/tabs.rs:1077-1098`) filtert auf `!name_traegt_den_filter` und auf die
zwei Schalter, aber nicht auf `versteckt`. Der Doc-Kommentar darüber sagt das ausdrücklich
(`tabs.rs:1071-1076`):

```
/// **Ein ausgeblendeter Eintrag steht mit in der Liste.** Die Regel, die ihn
/// wegblendet, ist der erste Zweig von `Ordnermodell::sichtbar` [...]
/// Der Befund ist dabei nicht umsonst — blendet der Nutzer die versteckten
/// Eintraege waehrend des Durchlaufs ein, steht die Zeile sofort richtig da.
```

Die Begründung stammt aus der Runde 10 und ist dort richtig: ein Befund kostete damals
einen Gang durch `getattrlistbulk(2)` über einen Unterbaum, und nur für Ordner. Seit dieser
Runde kostet derselbe Befund für **jede** versteckte Datei ein `open(2)`, ein `fstat(2)`
und bis zu 1 MB gelesene Bytes samt zweier Kopien im Arbeitsspeicher (`String::from_utf8`
und `to_lowercase` in `verzeichnis/inhalt.rs:135-139` und `verzeichnis/filter.rs:123`). Die
Begründung ist mit der Kostenseite mitgewandert und nicht neu geprüft worden.

**Zweitens, der Abstieg geht in versteckte Ordner hinein.** `unterbaum_entscheiden`
(`crates/krk-core/src/verzeichnis/durchlauf.rs:513-532`) verzweigt allein über
`kandidat.typ` und kennt kein Versteckt-Kennzeichen:

```rust
match kandidat.typ {
    Typ::Ordner => offen.push(lesestand.pfad.join(&kandidat.name)),
    Typ::Datei => { /* ... traegt_der_inhalt ... */ }
    Typ::Verknuepfung => {}
}
```

Ein Quellbaum mit „Deep" und „Content" liest damit `.git` mit: jedes lose Objekt, jede
Indexdatei und jede Paketdatei unter 1 MB wird geöffnet und ganz eingelesen. Für den Nutzer
ist das der teuerste Teil des Laufs und der einzige, der ihm nie eine Zeile bringt — der
Inhalt eines `.git`-Objekts ist zlib-gepackt, fällt am `String::from_utf8` heraus und
liefert `TraegtNicht`.

**Wo die Kosten des Unterbaums stehen und wo nicht.** Der Spec nennt unter „Zwei
Eigenschaften, die diese Runde annimmt statt sie zu beheben" die Trefferbreite eines
häufigen Wortes und die Protokolldatei über 1 MB. Die versteckten Einträge kommen weder
dort noch im Plan noch in einem der zwölf Sitzungsprotokolle vor; `grep -rn 'versteck'`
über den Circle findet vier Treffer, und alle vier betreffen den Prüfschritt oder die
Übertragung beim Ordnerwechsel, keiner die Leseseite.

---

**Die beiden Hälften sind verschieden zu behandeln, und deshalb steht hier keine
Vorschrift.**

Bei der Auftragsliste ist die Zeile nachweislich unsichtbar: der erste Zweig von
`sichtbar` (`modell.rs:588-590`) blendet sie weg, gleich was ihr Befund sagt. Sie zu lesen
kauft allein den Fall, dass der Nutzer mitten im Lauf die Verstecke einblendet.

Beim Abstieg ist der Fall anders: ein Treffer unter einem versteckten Ordner ist ein Treffer
unter dem sichtbaren Ordner darüber, und ihn zu übergehen änderte die Bedeutung von C3.1.
Ein Ausschluss hier wäre eine neue Regel und keine Sparmaßnahme.

Gefunden bei der Durchsicht der elften Runde, Bereich `9f5ced5..b9ab8ae`.

---
Resolved: 260816-2230, erste Hälfte behoben, zweite ausdrücklich nicht — so, wie der Datensatz oben es trennt.

**Die Auftragsliste.** Sie ist nicht um einen Versteckt-Zweig ergänzt worden, sondern an ihre Wurzel gezogen: sie stand in `krk-ui` und war die zweite Fassung des Prüfschritts, und genau deshalb fehlte ihr dessen erster Zweig. Der Prüfschritt liefert jetzt einen Wert `Zeilengrund` (`Steht`, `FaelltWeg`, `UnterVorbehalt(Auftragsart)`), und `Ordnermodell::auftraege` ist ein Gang über diesen Wert. Wessen Zeile an einem Befund hängt, verdient einen Auftrag — es ist dieselbe Frage, und sie steht jetzt einmal da. Ein ausgeblendeter Eintrag fällt damit ohne eigene Regel heraus; die freie Funktion `auftraege` in `crates/krk-ui/src/tabs.rs` ist gefallen.

**Der Handel ist umgedreht, nicht abgeschafft.** Bis heute bekam jeder versteckte Eintrag seinen Befund im Voraus, damit seine Zeile beim Einblenden sofort richtig dasteht; das kostete seit der Runde 11 je verstecktem Eintrag ein `open(2)` und bis zu 1 MB gelesene Bytes. Jetzt zahlt nichts, wer nie einblendet, und wer einblendet, zahlt einen neuen Lauf: `DateifensterQuelle::verstecke_umschalten` (`crates/krk-ui/src/appkit/tabelle.rs`) zieht seit dieser Änderung `durchlauf_nachziehen` und `meldung_gewechselt` nach, in derselben Bauart wie die beiden anderen Schalter. Das Ausblenden ist damit eine Eingabe der Auftragsliste geworden und steht als solche im Doc-Kommentar von `Tabliste::durchlauf_nachziehen`.

**Der Abstieg bleibt, wie er war**, und das ist eine Entscheidung und kein Übersehen: ein Treffer unter einem versteckten Ordner ist ein Treffer unter dem sichtbaren Ordner darüber, und ihn zu übergehen wäre eine neue Regel und keine Sparmaßnahme — sie änderte die Bedeutung von C3.1. Der Grund steht am Doc-Kommentar von `Ordnermodell::auftraege`, damit die nächste Durchsicht nicht dieselbe Frage noch einmal aufmacht. Der Kostenpunkt „ein Quellbaum liest sein `.git` mit" ist trotzdem weitgehend weg: das `.git` selbst ist ein versteckter Ordner des angezeigten Ordners und bekommt keinen Auftrag mehr.

Prüfbar gemacht in `crates/krk-core/tests/verzeichnis.rs`: `ein_ausgeblendeter_eintrag_bekommt_keinen_auftrag` misst beide Hälften des Handels an einem Zug, `die_auftragsliste_traegt_je_typ_die_richtige_art` hält den Schnitt fest. Die Abnahmeliste führt dafür die neue Beobachtung **27**.

Abnahme: `make check` — exit 0.
