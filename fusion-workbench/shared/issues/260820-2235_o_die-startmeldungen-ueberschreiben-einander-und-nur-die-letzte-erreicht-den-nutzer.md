Die Startmeldungen überschreiben einander, und nur die letzte erreicht den Nutzer

---

`oberflaeche_aufbauen` sammelt jede Startmeldung in einem `Vec` und stellt sie am Ende in
einer Schleife in die Statuszeile des aktiven Dateifensters. Die Zeile hält aber genau **eine**
Fenstermeldung; jeder Durchlauf überschreibt den vorigen im selben Zug. Von n Startmeldungen
sieht der Nutzer die n-te.

---

Die Schleife steht in `crates/krk-ui/src/appkit/anwendung.rs:1345-1347`:

```rust
let aktiv = self.ivars().modell.borrow().aktiv();
for meldung in meldungen {
    self.dateifenster(aktiv).quelle().meldung_zeigen(&meldung);
}
```

`meldung_zeigen` setzt das eine Feld `fenstermeldung` und zeichnet neu
(`crates/krk-ui/src/appkit/tabelle.rs:2955-2958`). Es gibt keine Warteschlange, keine
Verweildauer und keine Zusammenfassung. Die Schleife läuft in einem Zug auf dem Hauptfaden;
zwischen den Durchläufen zeichnet niemand.

**Wie viele Meldungen zusammenkommen können, sagt der Sammelweg**, und es sind mehr als eine:
`sitzung_laden` legt bis zu drei ab (beschädigte `session.toml`, beschädigte `settings.toml`,
„ohne Sitzungsrecht" — `anwendung.rs:1463`, `:1490-1492`), `leiste_einrichten` eine
(`:1602-1607`), `tastenabgriff_einrichten` eine, und die Belegungsmeldung aus
`tasten::belegung::fuer_den_betrieb` kommt dazu. Genau die Lage, in der KRK am meisten zu
sagen hätte — eine Ablage, an der mehreres zugleich nicht stimmt —, ist die Lage, in der er
am wenigsten davon zeigt.

**Der Preis ist die Diagnose.** Die Meldung „die Lesezeichen ließen sich nicht laden" ist der
einzige Hinweis, den eine leere Lesezeichenleiste beim Start überhaupt hinterlässt; die
Statuszeile hält sie ohnehin nur bis zum nächsten Ordner- oder Tabwechsel
(`tabelle.rs:2939-2941`). Steht danach noch eine weitere Startmeldung an, ist sie nie zu
sehen gewesen.

**Schwere:** mittel. Kein Datenverlust, aber ein stiller Verlust der einzigen Auskunft, die
KRK über eine gescheiterte Ablage gibt.

**Gefunden:** analyst, forensische Untersuchung „Lesezeichen nach Installation weg" am 260820-2235

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:1341-1347`,
`crates/krk-ui/src/appkit/tabelle.rs:2943-2958`

**Domain:** code

## Vorschlag

Nicht n Zeilen nacheinander in eine Zeile schreiben. Zwei Wege stehen zur Wahl, und beide
sind Entscheidungen und keine Ableitungen:

- **Eine Meldung aus n bauen**, also die Sätze verbinden und einmal setzen. Billig, hält den
  einen Kanal, und die Zeile wird lang.
- **Mehrere Startmeldungen sind ein Blatt und keine Statuszeile.** Das wäre eine Abkehr von
  der Antwort vom 260804-0830 (`decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`,
  Möglichkeit 1) für diesen einen Fall und gehört deshalb entschieden, nicht gebaut.

Der Fall „genau eine Meldung" muss in beiden Wegen Wort für Wort bleiben, was er heute ist.
