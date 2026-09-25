# Die Ersatzzeile aus C1.11 greift beim Tippen und nicht beim Umschalten von „Deep"

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.11 (erster Satz), C2; `crates/krk-ui/src/appkit/tabelle.rs:1711-1726` (`tiefe_suche_umschalten`), `:1170-1205` (`nach_filteraenderung`), `:1836-1852` (`umsortiert`); `crates/krk-ui/src/kommandos/navigation.rs`, `ersatzzeile`

---

## Befund

Vier Stellen ändern, was der Filter von der Liste übrig lässt. Drei nehmen denselben Weg, die vierte nicht:

| Stelle | Nachzug | `ersatzzeile` läuft |
|---|---|---|
| `filterzeichen_tippen` | `nach_filteraenderung` | ja |
| `letztes_filterzeichen_weg` | `nach_filteraenderung` | ja |
| `filter_leeren` (`Esc`) | `nach_filteraenderung` | ja |
| `tiefe_suche_umschalten` | `umsortiert` + `meldung_gewechselt` | **nein** |

`umsortiert` schreibt in seinem eigenen Doc-Kommentar aus, was daraus folgt: „**Fällt die Zeile der Auswahl weg, bleibt die Auswahl hier leer.** … Der Filter braucht eine andere Antwort; sie steht in `nach_filteraenderung`." Genau diese andere Antwort bekommt das Umschalten von „Deep" nicht.

**Wann es auffällt.** Ein Filtertext steht, „Deep" ist aus, die Auswahl steht auf einem Ordner, dessen Name den Filtertext nicht trägt (bei flacher Suche bleibt er sichtbar, C1.6). Der Nutzer klickt „Deep" an. Der Ordner fällt aus der Sicht, bis ein Befund für ihn eintrifft — und bis dahin steht in der Tabelle **keine** Zeile ausgewählt, statt der ersten sichtbaren. C1.11 erster Satz sagt: „Fällt die Zeile weg, auf der die Auswahl stand, geht die Auswahl auf die erste sichtbare Zeile."

Die Auswahl des **Modells** bleibt dabei auf dem Eintrag stehen und kommt zurück, sobald der Durchlauf den Ordner als Treffer meldet. Verloren ist also nichts; sichtbar ist in der Spanne dazwischen aber nichts, und ein Befehl, der eine Auswahl braucht, findet in der Spanne keine (`betroffene` läuft über die Sichtreihenfolge).

## Die Frage dahinter

C1.11 steht in C1, und C1 handelt vom Tippen. Ob das Kriterium auch für den Schalter aus C2 gilt, sagt der Spec nicht. Zwei Antworten sind möglich:

1. **Es gilt.** Dann ruft `tiefe_suche_umschalten` `nach_filteraenderung` statt `umsortiert` + `meldung_gewechselt`, und die vier Wege sind wieder einer.
2. **Es gilt nicht.** Dann gehört der Satz in C2, damit die Ungleichheit der vier Wege eine Entscheidung ist und kein Versehen.

Der Befund ist gering im Gewicht und deutlich in der Form: derselbe Vorgang nimmt zwei Wege, und einer der beiden trägt eine Zusage, die der andere nicht trägt.

---
Resolved: 260815-0246, shaper. Antwort 2 des Datensatzes gewählt: C1.11 gilt für das Tippen und nicht für den Schalter, und der Satz steht jetzt als eigenes Kriterium C2.14 in C2. Begründung: beim Tippen fällt eine Zeile endgültig weg, beim Umschalten von "Deep" nur so lange, bis der Befund für ihren Ordner eintrifft; die Ersatzzeile setzte die Auswahl in diesem Fall auf die erste sichtbare Zeile und verlöre den Platz des Nutzers dauerhaft, obwohl der Eintrag gleich darauf wiederkommt. Die Ungleichheit der vier Wege ist damit eine Entscheidung mit Begründung. Der Preis steht in C2.14 benannt: in der Spanne dazwischen ist keine Zeile ausgewählt, und ein Operationsbefehl meldet "es ist nichts ausgewählt". Am Baum ist nichts geändert.
