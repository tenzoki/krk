Der größte Code-Commit nach der letzten Durchsicht ist ungelesen und trägt eine Nutzerzusage

---

`dd74b0e` ändert vier Codedateien mit 303 hinzugefügten Zeilen und liegt hinter dem Bereich beider
Durchsichten der Runde 17. Er setzt die dritte Zusage um, die der Nutzer in dieser Runde gegeben
hat, und keine Durchsicht hat ihn gelesen.

---

**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1230, beim Abgleich zum Abschluss der Runde 17.**

## Was der Baum trägt

Die zwei Durchsichten der Runde decken zusammen `428fbc4..6faaa91` ab und sagen das selbst aus:

- `reviews/260825-0942-coderev-…` — `**Reviewed-range:** 428fbc4..423d5f2`
- `reviews/260825-1144-coderev-…` — `**Reviewed-range:** 6ad9198..6faaa91`, dazu der Satz
  „zusammen tilen die beiden den ganzen Sitzungsbereich"

Der Sitzungsbereich ist `428fbc4..ddd41ff`. Danach folgen drei Commits, von denen einer Code
trägt:

```
$ git diff --stat 6faaa91..HEAD -- crates/
 crates/krk-core/src/operation/zippen.rs      |  28 ++-
 crates/krk-ui/src/appkit/anwendung.rs        |  58 ++++--
 crates/krk-ui/src/appkit/tabelle.rs          |   2 +-
 crates/krk-ui/src/kommandos/kontextmenue.rs  | 215 +++++++++++++++++++--
```

Alle vier stammen aus `dd74b0e`. `e3478e6` und `ddd41ff` fassen nur die Werkbank an.

## Warum das trägt

`dd74b0e` ist nicht die Nachbesserung einer Kleinigkeit, sondern der Bau der dritten Nutzerzusage
dieser Runde: „Trifft das Ziel eines Laufs eine seiner eigenen Quellen, fällt diese Quelle aus dem
Lauf" (`shared/history/260824-2120-orchestrator-session.md`, Abschnitt „Nutzerantwort zum schweren
Befund der zweiten Durchsicht"). Er legt dafür die neue Regel `ist_ziel_des_laufs` an, die neue
öffentliche Funktion `packziel`, verschiebt in `zipauftrag_stellen` die Frage „gibt es etwas zu
packen" hinter die Zielklärung und schreibt den Modulkopf von `zippen.rs` in seiner Begründung um.
Das ist der Umfang, für den in dieser Runde sonst eine Durchsicht gelaufen ist — die zweite
Durchsicht las sechs Commits mit vierzehn geänderten Dateien.

Der Befund ist nicht, dass der Commit falsch wäre. Geprüft ist er: `make check` läuft am Stand
`ddd41ff` mit Exit 0 über alle vier Kommandos, und drei Proben in `kontextmenue.rs`
(`das_archiv_des_vorigen_laufs_faellt_aus_den_quellen`,
`ein_archiv_das_zielordner_eines_anderen_ist_faellt_aus_den_quellen`,
`ein_einzelnes_archiv_bleibt_seine_eigene_quelle`) halten die Zusage, mit gefahrener Gegenprobe.
Der Befund ist, dass die Runde ohne den Blick von außen schließt, den sie sich für jeden anderen
Commit genommen hat.

## Dieselbe Lage hat das Vorhaben schon einmal gehabt

`shared/decisions/260815-1812_o_der-eine-codecommit-der-sitzung-260815-1328-ohne-durchsicht-ist-nicht-nur-markdown.md`
steht seit dem 260815 offen und stellt dieselbe Frage für dieselbe Lage. Sie ist unbeantwortet,
und deshalb entsteht sie wieder. Verwandt, aber nicht dasselbe:
`shared/issues/260817-1122_o_der-durchsichtsbereich-schliesst-seinen-ersten-commit-aus.md` (der
Bereich lässt vorn etwas aus) und
`shared/issues/260810-1907_o_die-durchsicht-von-turn-2-hat-kein-durchsichtsdokument-hinterlassen.md`.

## Vorschlag

Zwei Wege, und die Wahl ist die des Nutzers.

1. **Eine dritte Durchsicht über `6faaa91..ddd41ff` fahren**, bevor die Runde schließt. Sie liest
   vier Dateien und einen Commit; der Aufwand ist klein, weil der Bereich klein ist.
2. **Die Runde so schließen und den Bereich der ersten Durchsicht der nächsten Runde
   mitgeben.** Das ist der billigere Weg und schiebt den Blick auf, statt ihn zu ersparen.

Die dahinterliegende Frage — ob ein Turn, der einen Befund behebt, seinerseits eine Durchsicht
schuldet — gehört in den offenen Datensatz `260815-1812` und nicht hierher.

**Schwere:** mittel. Kein gemessener Fehler, ein fehlender Prüfschritt an der Stelle, an der die
Runde ihre eigene Zusage baut.

**Betroffen:** `crates/krk-core/src/operation/zippen.rs`,
`crates/krk-ui/src/kommandos/kontextmenue.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/appkit/tabelle.rs` — jeweils der Anteil aus `dd74b0e`

---
Resolved: Die dritte Durchsicht hat `6faaa91..ddd41ff` gelesen
(`reviews/260825-1249-coderev-runde-17-dritte-durchsicht-das-ziel-eines-laufs-und-seine-quellen.md`),
die vierte danach den Behebungscommit
(`reviews/260825-1358-coderev-runde-17-vierte-durchsicht-der-behebungscommit.md`). Die vier
Durchsichten dieser Runde tilen damit den Codeanteil des ganzen Sitzungsbereichs; die Commits, die
keine Durchsichtsspanne deckt, fassen allein Werkbank-Prosa und `CLAUDE.md` an.
