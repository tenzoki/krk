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
