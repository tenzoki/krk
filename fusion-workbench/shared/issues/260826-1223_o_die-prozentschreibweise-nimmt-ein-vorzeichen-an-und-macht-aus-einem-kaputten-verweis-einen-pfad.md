Die Prozentschreibweise nimmt ein Vorzeichen an und macht damit aus einem kaputten Verweis einen Pfad

---

`prozent_dekodieren` liest die zwei Zeichen hinter einem `%` mit `u8::from_str_radix(…, 16)`. Diese Funktion nimmt ein führendes `+` an. `%+A` wird damit zu einem Zeilenumbruch statt zu `None` — genau das Ergebnis, das der Doc-Kommentar zwei Zeilen darüber ausschließt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Am Baum

`crates/krk-core/src/zwischenablage.rs:113-133`. Der Doc-Kommentar:

> `None` fuer eine kaputte Folge, etwa ein `%` am Ende oder `%zz`, und fuer ein Ergebnis, das kein UTF-8 ist. Beides still zu uebergehen hiesse, aus einem beschaedigten Verweis einen Pfad zu machen, den es nicht gibt.

Der Rumpf, Zeile 125:

```rust
gelesen.push(u8::from_str_radix(ziffern, 16).ok()?);
```

`u8::from_str_radix` nimmt nach der Beschreibung der Standardbibliothek ein führendes `+` oder `-` an. Für einen vorzeichenlosen Typ scheitert `-`, `+` nicht.

## Gemessen

Am 260826 mit `rustc -O` gegen eine wörtliche Kopie der Funktion:

```
"/a%20b"  -> Some("/a b")
"/a%2Gb"  -> None
"/a%+5b"  -> Some("/a\u{5}b")
"/a%+Ab"  -> Some("/a\nb")
"/a% 5b"  -> None
"/a%\t5"  -> None
u8::from_str_radix("+5",16) = Ok(5)
```

Zwei der sechs Fälle liefern einen Pfad, wo `None` stünde. Der Weg dorthin ist der Befehl `zwischenablage_springen` (`krk-ui/src/appkit/tabelle.rs:2400-2418`): der Inhalt der Zwischenablage geht durch `deuten`, ein `file:`-Verweis durch `verweis_zu_pfad` und damit durch diese Stelle. Aus `file:///tmp/a%+Ab` wird der Pfad `/tmp/a\nb`, und der Nutzer bekommt die Meldung, dieser Pfad sei nicht erreichbar, statt der Meldung, die Zwischenablage trage nichts Verwertbares.

## Schwere

Niedrig. Kein Schalenaufruf hängt daran, der erzeugte Pfad geht durch `pfadeingabe::pruefen` und wird dort abgewiesen; der Schaden ist die falsche Meldung und ein Verweis, den KRK anders liest als jedes andere Werkzeug. Der Befund steht trotzdem, weil er die Aussage des eigenen Doc-Kommentars widerlegt und weil die Behebung eine Zeile ist: die zwei Zeichen vor dem `from_str_radix` gegen `is_ascii_hexdigit` halten.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.
