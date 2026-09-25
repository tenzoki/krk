Eine dritte Stelle nennt den Rang der Fenstermeldung falsch

---

`crates/krk-ui/src/appkit/anwendung.rs:3620` sagt, eine Meldung gehe „einen Rang tiefer als eine
Befehlsantwort". Sie geht über `meldung_zeigen` und damit auf die **Fenstermeldung**, und die
steht auf **Rang 3** — zwei Ränge tiefer, nicht einen.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung von
`260811-0838_*_antwort-zeigen-nennt-vier-raenge-die-statuszeile-fuehrt-fuenf.md`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:3620`
**Domain:** code

## Zusammenhang

Es ist die dritte Stelle desselben Fehlers. Der Datensatz `260811-0838` führte eine (`:3334`,
vier Ränge statt fünf), die Nachbarprüfung fand die zweite (`Dateifenstersicht::melden`,
„einen Rang tiefer" statt Rang 3), und beide sind behoben. Diese dritte lag außerhalb der
Auftragsgrenze; der `coder` hat sie gemeldet, statt sie stillschweigend mitzunehmen.

Die fünf Ränge, nachgezählt an `crates/krk-ui/src/appkit/statuszeile.rs:75-83`: Befehlsantwort,
Vorgangsanzeige, Fenstermeldung, Tabmeldung, Markierungsstand.

## Denkbarer Weg

Dieselbe Berichtigung wie an den beiden anderen Stellen: „auf Rang 3", in der Schreibweise von
`anwendung.rs:1771`.

**Und eine Frage, die dabei mitgeht:** dreimal derselbe Fehler in einer Datei deutet darauf, dass
die Ränge in Kommentaren beschrieben statt aus einer Stelle gelesen werden. Ob sich das lohnt zu
ändern — etwa indem `statuszeile.rs` die Ränge benennt und die Kommentare auf die Namen
verweisen —, ist hier nicht entschieden.

## Dringlichkeit

Gering. Ein falscher Kommentar, kein falsches Verhalten.

---
Resolved: `anwendung.rs:3619-3621` sagt jetzt "Sie geht deshalb als Fenstermeldung auf Rang 3
der Statuszeile; auf Rang 1 loeschte der erste Tastendruck sie weg" — in der Schreibweise von
`anwendung.rs:1771`. Die Zeilennummer dieses Datensatzes stimmte.

**Es gab eine vierte Stelle, und sie stand in einer anderen Datei.** `tabelle.rs:322` nannte die
Befehlsantwort "den obersten der vier Raenge". Sie widersprach damit `tabelle.rs:1392` **in
derselben Datei**, wo "der fuenfte Rang der Statuszeile" schon richtig stand. Auch sie stammt aus
der Zeit vor S16c, als der Markierungsstand noch fehlte. Berichtigt.

**Achtzehn weitere Stellen sind bei dieser Gelegenheit gegen `statuszeile.rs:75-83` geprueft und
richtig befunden:** `anwendung.rs` 1245, 1771, 2008, 3334, 3338, 3666, 3679, 4047, 4112;
`tabelle.rs:1392`; `editor.rs:170`; `statuszeile.rs` 37, 40, 73, 102, 109, 112, 299. Der Bestand
ist damit erhoben und nicht bloss an der gemeldeten Stelle geflickt.

**Die Wurzelfrage, die dieser Datensatz aufwirft, ist als Entscheidung abgelegt** statt hier
beantwortet: `decisions/260811-1230_o_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`.
Kurz: die naheliegende Antwort gibt es nicht, weil kein Prueflauf eine Zahl in einem Kommentar
liest. Die Empfehlung lautet, die Zahl wegzulassen und die Quelle zu verlinken — aber erst, wenn
jemand die Dateien ohnehin anfasst.
Abgenommen mit `make check`, exit 0.

Geschlossen in der Sitzung `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/history/260811-0107-orchestrator-session.md`.
