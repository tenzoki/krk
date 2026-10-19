# Gilt der Stand mit den Zeilenenden der Datei als Antwort auf die Sicherungsform einer CSV, und was folgt daraus für Zeilenindex, Suche und Rohansicht?

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md, 261019-0037-ein-umbruch-in-einer-zelle-einer-crlf-csv-wird-bei-jedem-sichern-zu-crlf-auch-in-unberuehrten-zeilen.md, 260808-0021_*_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md, 261019-1122-reviewer-csv-und-tsv-schlussdurchsicht-vor-dem-schliessen.md, 261019-1122-der-zeilenindex-liefert-in-einer-crlf-csv-den-zeileninhalt-mit-cr-und-textmarken-verfehlen-ihre-zeile.md

---

## Question

Der Nutzer hat am 261018 für den Entscheid `261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md` Möglichkeit 2 gewählt. Ihr Wortlaut: „Der gehaltene Stand bleibt unverändert (ohne Marke, nur `\n`), sodass Zeilenindex, Suche und Textmarken weiter rechnen dürfen“, dazu der Constraint „Der gehaltene Stand bleibt UTF-8 ohne Marke mit `\n`; drei Stellen rechnen davon.“ Gebaut ist seit `ff2d7b76` etwas anderes: der Stand einer CSV oder TSV trägt die Zeilenenden der Datei (`datei::trenntext_oeffnen`, `Zeilenenden::WieGelesen`), und allein eine Datei mit `\r` als einzigem Zeilenende wird gewandelt (`Zeilenenden::NurCr`). Die zugesagte Wirkung (Marke und Zeilenenden bleiben) hält, und sie hält besser als die gewählte Bauweise, die einen Umbruch in einer Excel-Zelle verlor. Der Constraint aber ist gebrochen, und die Durchsicht vom 261019-0042 hatte für diese Änderung ausdrücklich eine Zeile des Nutzers verlangt. Der `Resolved:`-Vermerk des Defekts sagt: „auf Anweisung des Dispatch selbständig ausgeführt; der Entscheid ist nicht ergänzt.“ Der Datensatz steht damit auf `answered` mit einer Antwort, die der Code nicht umsetzt. Bevor das Paket schließt, braucht es eine Antwort, welche Bauweise gilt.

## Options

1. **Die Bauweise vom 261019 gilt; ein neuer Entscheid löst `261018-0822-…` ab.** Der Stand einer CSV trägt ihre Zeilenenden.
   - Pros: Byte-Treue auf der Platte für jede Datei, auch für Umbrüche in Zellen und gemischte Enden; der Code steht und ist geprüft (`ein_umbruch_in_einer_excel_zelle_bleibt_beim_sichern_einer_anderen_zelle_stehen`, `eine_csv_mit_marke_und_crlf_kommt_unveraendert_byte_fuer_byte_zurueck`).
   - Folgen, die der Nutzer mitwählt:
     - Der Zeilenindex liefert in einer `\r\n`-CSV den Zeileninhalt mit `\r`. Textmarken aus einer Fassung vor 3.9.0 auf CSV-Zeilen finden ihre Zeile nicht mehr (Defekt `261019-1122-der-zeilenindex-liefert-in-einer-crlf-csv-den-zeileninhalt-mit-cr-und-textmarken-verfehlen-ihre-zeile.md`).
     - Eine Zeile, die der Nutzer in der Rohansicht mit `return` beginnt, endet auf `\n`, auch in einer `\r\n`-Datei. `HowTo.md` sagt das bereits.
     - Ein einzelnes `\r` mitten in einer Datei mit `\n` zeigt die Rohansicht als Umbruch, den die Zeilennummern nicht zählen. `CLAUDE.md` sagt das bereits.
     - Die Zusage im Modulkopf von `krk-core/src/text/datei.rs` gilt für CSV nur noch zur Hälfte, und jede künftige Stelle, die auf `\n` als einzigem Zeilenende rechnet, muss die Form fragen.
2. **Zurück zur gewählten Bauweise, mit Satzgrenzen aus dem Index.** Der Stand bleibt mit `\n`. Beim Sichern werden allein die Satzgrenzen, die `trenntext::indexieren` kennt, in die Form der Datei gebracht; `crlf` wird am ersten Satzende außerhalb von Anführungszeichen erhoben.
   - Pros: der Constraint hält; Zeilenindex, Suche und Textmarken bleiben unberührt.
   - Cons: gemischte Zeilenenden kommen einheitlich zurück; ein `\r\n` innerhalb einer Zelle wird `\n`; der Satzindex muss beim Sichern jeder CSV einmal laufen, an 16 MiB nach der Messung im Defekt zur Zellübernahme rund 25 ms; der Code von `ff2d7b76` wird zurückgebaut.

## Constraints

- Die Directive des Pakets: unveränderte Teile der Datei verlieren ihre Form nicht.
- Für jede andere Datei gilt die Regel vom 260808 unverändert (`Dateiform::GEHALTEN`).
- Geschrieben wird weiter allein über `datei::sichern` und `ablage::atomar`.

## Recommendation

Möglichkeit 1. Sie hält die Directive vollständiger als die gewählte Bauweise, und ihre Folgen sind klein und benannt. Die eine Folge, die als Defekt bleibt, die Textmarke, schließt sich an einer Stelle (`Zeilenindex::inhalt_der_zeile` schneidet ein `\r` vor dem `\n` ab). Möglichkeit 2 brächte den Preis gemischter Zeilenenden zurück, den der Nutzer am 261018 als Kontra gelesen hat.

Answered: 261019, vom Nutzer im Chat: „1“ — Möglichkeit 1, die Bauweise vom 261019 gilt; der Entscheid 261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md ist damit abgelöst. Die drei Defekte der Schlussdurchsicht bleiben offen.

Implemented: ff2d7b76 (fix(trenntext): CSV haelt ihre Zeilenenden wie gelesen), ausgeliefert mit 3.9.0.
