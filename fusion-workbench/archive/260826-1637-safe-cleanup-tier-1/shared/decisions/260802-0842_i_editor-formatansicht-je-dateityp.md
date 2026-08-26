# Was zeigt die Formatansicht des Editors bei Text, bei Code und bei Markdown?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`

---

## Question

Der Entwurf verlangt für Text, Code und Markdown jeweils eine Rohansicht und eine Formatansicht. Bei Markdown ist die Formatansicht eindeutig, nämlich das gerenderte Dokument mit Überschriften, Listen und Links. Bei Code und bei einfachem Text bleibt offen, was sie überhaupt anzeigen soll. Syntaxhervorhebung ist die naheliegende Antwort für Code, aber sie ist in modernen Editoren Teil der normalen Bearbeitungsansicht und keine zweite Ansicht daneben. Für einfachen Text gibt es gar keine offensichtliche Formatierung. Ohne Antwort baut KRK entweder eine Ansicht, die bei zwei von drei Dateitypen leer bleibt, oder drei Sonderfälle mit je eigener Regel. Die Frage bestimmt, was der Nutzer beim Umschalten sieht, und gehört deshalb vor den Aktivierungs-Spec.

## Options

1. **Eine Ansicht pro Typ, jeweils sinnvoll besetzt** — Markdown wird gerendert, Code bekommt Syntaxhervorhebung mit eingeklappten Blöcken, einfacher Text bekommt Zeilenumbruch am Fensterrand und eine lesbare Schriftgröße. Die Rohansicht zeigt in allen drei Fällen die Zeichen so, wie sie in der Datei stehen.
   - Pro: das Umschalten liefert bei jedem Dateityp einen sichtbaren Unterschied. Ein Regelwerk, kein Sonderfall pro Typ.
   - Contra: der Unterschied ist bei einfachem Text schwach und rechtfertigt kaum eine eigene Ansicht.

2. **Formatansicht nur dort, wo sie etwas bedeutet** — Markdown wird gerendert, alle anderen Typen kennen nur eine Ansicht mit Syntaxhervorhebung. Der Umschalter erscheint nur bei Markdown.
   - Pro: entspricht der Maxime "supersimpel". Keine Ansicht, die nichts zu zeigen hat.
   - Contra: der Umschalter ist mal da und mal nicht, was die Bedienung uneinheitlich macht.

3. **Formatansicht als reine Leseansicht** — die zweite Ansicht ist bei jedem Typ schreibgeschützt und auf gutes Lesen ausgelegt: Markdown gerendert, Code hervorgehoben und ohne Cursor, Text im Lesesatz. Bearbeitet wird ausschließlich in der Rohansicht.
   - Pro: die Trennung ist bei jedem Typ dieselbe und leicht zu erklären, nämlich lesen gegen bearbeiten. Nebenbei schützt sie vor versehentlichen Änderungen beim Durchsehen.
   - Contra: in einer gerenderten Markdown-Ansicht direkt schreiben zu können, ist ein Komfort, den diese Option ausschließt.

## Constraints

- Beide Ansichten arbeiten auf derselben Datei. Ein Wechsel darf keine ungespeicherten Änderungen verlieren.
- Die Editor-Funktionen aus dem Entwurf, also zu einer Zeile springen, kopieren und einfügen, suchen und ersetzen sowie Textstellen als Lesezeichen speichern, müssen in mindestens einer der beiden Ansichten erreichbar bleiben.
- Die Antwort betrifft nur den eingebauten Editor, nicht das Vorschaufenster. Was die Vorschau bei nicht darstellbaren Dateien zeigt, ist bereits geklärt: die Metadaten.

## Recommendation

Option 3 gibt allen drei Dateitypen dieselbe Regel und macht den Umschalter überall verständlich. Der Verzicht auf das Schreiben in der gerenderten Markdown-Ansicht ist der Preis, und er ist verschmerzbar, solange der Wechsel schnell geht. Empfehlung, keine geprüfte Aussage.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"1. Formatansicht" — Möglichkeit 1 gewählt: eine Ansicht pro Dateityp, jeweils eigens besetzt. Markdown gerendert, Code mit Syntaxhervorhebung und einklappbaren Blöcken, einfacher Text mit Umbruch am Fensterrand und lesbarer Schriftgröße; die Rohansicht zeigt überall die Zeichen der Datei. Der Nutzer ist damit der Empfehlung dieses Datensatzes (Möglichkeit 3, durchweg schreibgeschützte Leseansicht) nicht gefolgt: die Formatansicht bleibt bearbeitbar, und der hier benannte Preis der schwachen Unterscheidung bei einfachem Text ist angenommen. Entschieden vom Nutzer am 260807-2139.
Implemented: `41309cc` — drei Besetzungen statt einer: `crates/krk-ui/src/hervorhebung.rs:200-208` führt `Darstellungsart` mit `EinfacherText`, `Code` und `Markdown`. Die Formatansicht ist bearbeitbar geblieben, wie der Nutzer es gegen die Empfehlung dieses Datensatzes gewählt hat: `crates/krk-ui/src/editormodell.rs:239` führt `Ansicht` mit zwei Werten und keinem Schreibschutz. Einfacher Text bekommt Umbruch und Lesezuschlag (`crates/krk-ui/src/appkit/editor.rs:1646`, `:1681`, `:1709`), Markdown seine Auszeichnungen (`hervorhebung.rs:238-253`). Planschritte S33 und S34 tragen `[DONE]`. Nachgeprüft im Abgleich am 260810.
Deferred:
Superseded by:

Nachtrag 260810 (Abgleich): Der Nachtrag vom 260808-0017 ist ebenfalls eingelöst — einklappbare Blöcke sind nicht gebaut, eine Suche nach `einklapp` und `klappbar` über `crates/` und `resources/` liefert null Treffer. Der Verweis in jenem Nachtrag zeigt auf den Datensatz zur Syntaxkiste, der seit dem Abgleich `_i_` statt `_a_` trägt; sein Pfad ist als `decisions/260807-2147_*_fuer-welche-sprachen-hebt-die-formatansicht-syntax-hervor.md` zu lesen.

Nachtrag 260808-0017: Die Hälfte "mit einklappbaren Blöcken" ist zurückgenommen. Der Nutzer hat für die Syntaxhervorhebung eine fertige Rust-Kiste gewählt (`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260807-2147_*_fuer-welche-sprachen-hebt-die-formatansicht-syntax-hervor.md`), und solche Kisten bringen die Blockgrenzen nicht mit: Hervorhebung braucht Wortarten, Einklappen braucht Blockgrenzen. Für die Formatansicht bei Code gilt damit in dieser Runde: Syntaxhervorhebung ja, einklappbare Blöcke nein. Die Blöcke sind ein eigenes späteres Vorhaben.
