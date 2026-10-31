# In welche Richtung irrt Strip, wenn aus dem Text nicht entscheidbar ist, ob ein Umbruch vom Terminal stammt?

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md` (E9, E11), `261031-1156-die-laengste-zeile-gilt-immer-als-umgebrochen-und-strip-verschluckt-befehle-format-tabellen.md`, `261031-1226-strip-verbindet-getrennte-befehle-aehnlicher-laenge-und-howto-verspricht-das-gegenteil.md`, `261031-1226-ein-einmal-umbrochener-text-bleibt-nicht-wie-er-ist-format-loest-die-fortsetzung-ab.md`

---

## Question

Bei Text aus dem Chat sind zwei Eingabeformen aus dem Text allein nicht zu unterscheiden:

- ein Befehl, den das Terminal umbrochen hat, und zwei getrennte Befehle, deren erster lang ist;
- ein Befehl, den das Terminal zweimal umbrochen hat, und zwei oder mehr getrennte Befehle ähnlicher Länge.

Der Chat rückt jede Zeile nach der ersten ein, und die Breite des Terminals reist nicht mit (Modulkopf `crates/krk-core/src/text/glaetten.rs`). Für Format ist ein Irrtum Kosmetik. Für Strip ändert **jede** Fehlentscheidung die Bedeutung. Verbindet Strip zu viel, laufen die Argumente des zweiten Befehls im ersten mit (`rm -rf A` + `ls -la B` wird `rm -rf A ls -la B`). Verbindet Strip zu wenig, läuft beim Einfügen ein abgeschnittener erster Befehl (`git push --force-with-lease` ohne Ziel).

Seit `b21ca032` gilt: verbunden wird erst, wenn zwei Zeilen die längste bis auf ein Zehntel erreichen. Das hat der Implementierer im autonomen Modus gewählt. Das Issue A1 hatte die Frage für den Fall, dass nur Sonderfälle helfen, an den Planer zurückgegeben. Die Regel trifft beide Richtungen: einmal umbrochene Befehle bleiben getrennt, getrennte Befehle ähnlicher Länge werden verbunden. Welche Richtung Strip bevorzugen soll, hat niemand entschieden.

## Options

1. **So lassen** (Stand `c1c2b7f8`). Zweimal umbrochene Befehle werden verbunden, einmal umbrochene nicht. Getrennte Befehle ähnlicher Länge werden verbunden. `HowTo.md` beschreibt beide Fälle richtig (Issues vom 261031-1226).
2. **Strip verbindet nie über die Breite**, nur über `\`-Fortsetzungen und nur innerhalb von Anführungen und Heredocs, wo der Umbruch ohnehin bleibt. Strip nimmt dann allein Einrückung und nachlaufenden Leerraum. Ziel 2 des Briefs („sodass er im Terminal läuft“) hält dann für keinen umbrochenen Befehl. Dafür ändert Strip nie die Bedeutung, und die Ausgabe zeigt jeden Umbruch, der bleibt.
3. **Strip verbindet wie heute, meldet aber in der Statuszeile, wie viele Umbrüche es geschätzt hat** (verbunden und stehen gelassen). Der Nutzer prüft dann vor dem Einfügen. Das ist eine neue Meldung mit drei Einträgen in der Sprachtabelle, sonst unverändert.
4. **Strip fragt die Lage der Shell stärker ab**: verbunden wird nur, wenn das erste Wort der Folgezeile kein Befehl sein kann, etwa wenn es mit `-` beginnt, mit `|`, `&&`, `||` oder `;` beginnt oder die Zeile davor mit `|`, `&&` oder `||` endet. speculation: Damit ließe sich ein Teil der Fälle entscheiden. Wie groß dieser Teil ist, hat niemand gemessen. Nach `rules/critical-stance.md` §4 wäre das eine andere, teilweise entscheidbare Frage und keine Schwellenverschiebung. Der Rest bliebe in einer der Richtungen 1 bis 3.

## Constraints

- Das Beispiel des Nutzers betrifft Format. Es bleibt unter jeder Möglichkeit grün, denn Format teilt die Regel `umgebrochen`, und der Unterschied läge in `befehl`.
- `cmd+z` nimmt jede Umformung als eine Handlung zurück. Eingefügt und ausgeführt ist ein Befehl aber nicht mehr zurückzunehmen.
- Das Paket läuft autonom. Eine Antwort des Nutzers ist bis zur Abnahme am Bündel offen, die der Plan ohnehin vor einer Auslieferung verlangt.

## Recommendation

inference: Möglichkeit 4 mit 3 als Rückfall. Sie ist die einzige, die einen Teil der Frage entscheidbar macht, statt eine Schätzung zu verschieben. Ob der entscheidbare Teil die gewöhnlichen Fälle trägt, muss eine Messung an echten kopierten Befehlen zeigen, bevor gebaut wird. Bis dahin ist Möglichkeit 1 mit richtiger Dokumentation der Stand, den die zwei Issues vom 261031-1226 verlangen.

**Answered:** 261031 by user: „ok, wir nennen strip in singlestrip um, und gehen davon aus, dass die eingabe ein einzelner befehl ist, nicht eine liste. dann kann das strippen auch die umbrüche entfernen“. Keine der vier Möglichkeiten: Strip heisst Singlestrip, die Eingabe gilt als ein einzelner Befehl, und jeder Umbruch ausserhalb von Anfuehrungen und Heredocs wird entfernt; die Breitenschaetzung entfaellt fuer diesen Befehl.

**Implemented:** 261031 in `f39e14fb` (Singlestrip-Regel), `46abb49a` (Umbenennung) und `780d6870` (Kennung in der Belegung).
