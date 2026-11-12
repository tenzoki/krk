# Darf Format oder Singlestrip in einer CRLF-CSV einen Zellumbruch im Ausschnitt zu `\r\n` machen?

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md, 261019-0037-ein-umbruch-in-einer-zelle-einer-crlf-csv-wird-bei-jedem-sichern-zu-crlf-auch-in-unberuehrten-zeilen.md, 261019-1122-gilt-der-stand-mit-den-zeilenenden-der-datei-als-antwort-auf-die-sicherungsform-einer-csv.md, 261112-0851-reviewer-schlussdurchsicht-zwei-defekte-nach-den-protokollknoepfen.md

---

## Question

Seit `f0a60eea` schreibt `protokoll_umgeformt` (`crates/krk-ui/src/appkit/editor.rs`) ein geändertes Ergebnis in einer einzigen Zeilenendeform: `nachher.replace('\n', form)`. Trägt der Ausschnitt einer CSV mit `\r\n` in `minutes/` einen Umbruch in einer Zelle (ein einzelnes `\n`, wie Excel ihn schreibt) oder ein einzelnes `\r`, und ändert die Umformung irgendwo im Ausschnitt etwas, kommt auch dieser Umbruch als `\r\n` zurück, in Zeilen, an denen die Umformung sonst nichts ändert. Bei leerer Auswahl ist der Ausschnitt die ganze Datei. Die Probe `die_umformung_am_protokoll_haelt_die_zeilenenden_des_ausschnitts` hält genau das fest (Abschnitt „Gemischt und geaendert“), und die Abnahme des behobenen Defekts verlangt es („kein einzelnes `\n`“).

Der Defekt vom 261019 hat denselben Wandel beim Sichern als Fehler behandelt. Dort geschah er ungefragt bei jedem Sichern; hier folgt er einem ausdrücklichen Klick, ist mit `cmd+z` zurückzunehmen und steht bis `cmd+s` allein im Editor. Ändert die Umformung nichts außer der Form der Zeilenenden, bleibt der Ausschnitt Byte für Byte (`None`); die Frage betrifft allein den Fall, dass sie etwas ändert. Entschieden hat das bisher niemand: die Abnahme stammt aus dem Defektdatensatz und nicht aus einem Entscheid.

## Options

1. **So lassen** — ein geändertes Ergebnis trägt eine einzige Form.
   - Pros: eine Regel, gebaut und geprüft; `glaetten` sagt nicht, welche Zeile des Ergebnisses zu welcher des Ausschnitts gehört.
   - Cons: ein Zellumbruch in einer Zeile, die die Umformung nicht anfasst, wechselt seine Bytes; nach `cmd+s` liest ein anderes Programm die Zelle anders.
2. **Zeilenweise erhalten, wo die Zuordnung feststeht** — Format fügt nie Zeilen hinzu; hat das Ergebnis so viele Zeilen wie der Ausschnitt, ist die Zuordnung eins zu eins, und jede Zeile behält ihr eigenes Ende. Sonst die Regel von heute.
   - Pros: der häufige Fall (Format kürzt allein Leerraum am Zeilenende) lässt jeden Umbruch stehen.
   - Cons: zwei Zweige statt eines; für Singlestrip greift er nicht, weil es Zeilen verbindet und leere Randzeilen nimmt.
3. **Vor der Rohansicht einer CSV oder TSV stehen Format und Singlestrip nicht** — `zulaessigkeit::protokolltext` sagt für eine Trenntextdatei nein, „Kopieren“ bleibt.
   - Pros: die Frage entfällt; beide Befehle lesen Zeilen und kennen keine Anführung, ein Format über eine CSV verbindet Sätze.
   - Cons: nimmt eine Funktion zurück, die der Plan `261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md` für jede Datei in `minutes/` vorsieht.

## Constraints

- Eine Umformung, die außer der Form der Zeilenenden nichts ändert, bleibt keine Handlung.
- Der Weg der Quicknote und einer Datei in gehaltener Form (allein `\n`) bleibt unverändert.
- Geschrieben wird weiter allein über `umbau_anwenden`.

## Recommendation

Möglichkeit 1, bis ein Nutzer den Fall trifft. Der Wandel folgt einem Klick, ist sichtbar und rücknehmbar, und Format über eine CSV mit Zellumbrüchen ändert die Datei ohnehin stärker als an ihren Zeilenenden. Möglichkeit 2 lohnt, wenn Format vor CSV-Dateien wirklich benutzt wird.
