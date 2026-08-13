Das Menü „Bearbeiten" verliert seine Mac-übliche Reihenfolge und seinen Trenner

---

Seit S6 kommt die Reihenfolge innerhalb eines Obermenüs aus `resources/default-keymap.toml`,
und dort stehen die sechs zugestellten Textbefehle in dieser Folge: Ausschneiden, Kopieren,
Einfügen, Alles auswählen, Rückgängig, Wiederholen. Am 260813 über `--menue-protokoll`
nachgesehen — genau so steht das Menü jetzt da.

Auf dem Mac stehen „Rückgängig" und „Wiederholen" **oben**, durch einen Trenner von den vier
Zwischenablage-Befehlen geschieden. Bis zur Runde 7 tat KRK das auch: der Aufbau in
`menue.rs` trug die Reihenfolge und den Trenner als Programmtext.

Beides ist weg. Das Menümodell führt genau **einen** Trenner, den über dem Beenden (C2.9), und
keine zweite Ordnung neben der Belegungsdatei.

---

**Schwere:** gering (kein Befehl fällt aus, keine Kombination ändert sich; das Menü sieht an
einer Stelle unvertraut aus)
**Gefunden:** coder, beim Bauen von S6 der Runde 7 am 260813-0420
**Betroffen:** `resources/default-keymap.toml` (die sechs `text_*`-Einträge),
`crates/krk-ui/src/menuemodell.rs`
**Domain:** data — die Behebung gehört dem `ontocoder`

## Warum es nicht in `menuemodell.rs` gehört

Eine Ausnahmeliste im Modell, die zwei Einträge nach oben zieht und einen Trenner dazwischen
setzt, wäre eine zweite Ordnung neben der Gliederung — genau das, was C2.2 und der
Doc-Kommentar von `nach_bereichen` ausschließen. Die Reihenfolge steht an einer Stelle, und
das ist die Belegungsdatei.

**Die Behebung ist deshalb billig und liegt woanders:** die zwei Blöcke `text_rueckgaengig` und
`text_wiederholen` wandern in `resources/default-keymap.toml` vor `text_ausschneiden`. Das
ändert daneben die Reihenfolge im Abschnitt „Bearbeiten" der Belegungsansicht und der
Markdown-Ausgabe, und zwar in dieselbe Richtung.

## Der Trenner ist die zweite Hälfte und die teurere

Ein Trenner mitten in einem Bereich braucht eine Angabe, wo er steht, und die Belegungsdatei
kennt keine. Drei Wege, keiner davon geschenkt:

1. **Ohne Trenner leben.** Sechs Einträge in einem Menü sind ohne Gliederungslinie
   überschaubar, und die Mac-Gewohnheit ist eine Gewohnheit und keine Zusage des Spec.
2. **Ein Feld in der Belegungsdatei**, etwa `trenner_davor = true`. Es beträfe alle neun
   Obermenüs gleich und wäre nicht auf „Bearbeiten" beschränkt — die Belegungsansicht und die
   Markdown-Ausgabe müssten dann sagen, was sie damit tun.
3. **Eine benannte Ausnahme im Menümodell**, wie sie der Markdown-Eintrag schon ist. Sie wäre
   klein, aber sie führte den ersten Fall ein, in dem das Modell eine Reihenfolgeentscheidung
   trifft, die nicht aus der Belegung kommt.

Empfohlen ist Weg 1, bis der Nutzer den Trenner vermisst: er kostet nichts, und die zwei
anderen sind jederzeit nachziehbar.

---

## Zwischenstand 260813, Turn 2: die Vorbedingung ist erfuellt, der Befund bleibt offen

Der `ontorev` hat empfohlen, diesen Befund zusammen mit `260813-0534` zu fahren, weil dessen
fehlender Absatz im Dateikopf die erste Verschiebung erklaeren muss. **Der Absatz steht seit
dem 260813 im Kopf von `resources/default-keymap.toml`:** er sagt, dass
`belegungsmodell::nach_bereichen` die Funktionen einer Gruppe in Dateireihenfolge liefert, dass
drei Abnehmer sie so anzeigen, und dass ein verschobener Block einen Menueeintrag mitverschiebt.

**Die Verschiebung selbst ist nicht gefahren.** Sie bewegt zwei `[[funktion]]`-Bloecke und ist
damit eine Aenderung an Daten und nicht an Kommentaren; der Auftrag des Turns hat den
Rust-Anteil und die Kommentare der Belegungsdatei umfasst und die Datenzeilen ausdruecklich
nicht. Offen bleibt daneben die Trennerfrage, deren drei Wege oben stehen und deren Weg 1
empfohlen ist.
