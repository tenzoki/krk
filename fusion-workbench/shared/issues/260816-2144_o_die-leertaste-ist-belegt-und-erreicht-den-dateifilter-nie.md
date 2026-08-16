Die Leertaste ist an die Markierung vergeben und erreicht den Dateifilter nie

---

Der Nutzer hat am 260816-2144 gemeldet, dass er in den Suchtext des Dateifilters kein
Leerzeichen eingeben kann. Dateinamen mit Leerzeichen sind auf dem Mac der Regelfall, und
der Filter der Runde 10 kann nach ihnen nicht suchen.

---

**Schwere:** Mittel. Kein Datenverlust, aber eine zugesagte Funktion greift bei einer
großen Klasse von Dateinamen nicht, und ein Ausweichweg wie bei den Großbuchstaben
existiert hier nicht.
**Gefunden von:** Nutzer, gemeldet am 260816-2144
**Betroffen:** `resources/default-keymap.toml:287-290`,
`crates/krk-core/src/tasten/belegung.rs:1222-1247`
**Domain:** code

## Die Zeichenregel ist es nicht

`traegt_ein_dateiname` (`crates/krk-core/src/verzeichnis/filter.rs:90-94`) nimmt das
Leerzeichen an. Die Funktion weist genau drei Klassen ab, und in keine fällt es:

```rust
!zeichen.is_control()
    && !(FUNKTIONSTASTEN_ANFANG..=FUNKTIONSTASTEN_ENDE).contains(&zeichen)
    && zeichen != '/'
```

Ein Leerzeichen ist kein Steuerzeichen, liegt nicht im privaten Bereich `U+F700` bis
`U+F8FF`, in dem AppKit die Pfeile und die Funktionstasten meldet, und ist kein
Schrägstrich. Der Tastendruck kommt bei dieser Regel nur nie an.

## Die Ursache liegt eine Stufe früher, in der Belegung

`space` trägt in `resources/default-keymap.toml:287-290` die Funktion
`markierung_umschalten`, „Eintrag markieren und zum nächsten rücken", aus C2 der ersten
Runde.

`Belegung::nachschlag` (`crates/krk-core/src/tasten/belegung.rs:1222`) durchläuft zuerst alle
Funktionen und liefert die gefundene. Erst wenn keine passt, entscheidet die Fallunterscheidung
am Ende der Funktion (`belegung.rs:1243-1247`), ob der Druck als `Nachschlag::Tippen` in den
Filtertext geht oder als `Nachschlag::Unbelegt` an AppKit zurückfällt. Eine belegte
Kombination erreicht diese Stelle nicht.

Der Modulkopf von `filter.rs` hält die Trennung ausdrücklich fest: welcher **Tastendruck**
ankommt, entscheidet der Nachschlag; welches **Zeichen** aufgenommen wird, entscheidet
`traegt_ein_dateiname`.

## Warum der behobene Fall vom 260816-1101 diesen hier nicht mit erledigt

`shared/issues/260816-1101_c_kein-zeichen-mit-umschalttaste-erreicht-den-dateifilter.md` hat
am selben Tag denselben Weg berührt und die Trennung von Schreib- und Befehlstasten
eingeführt: Umschalt und Wahl tippen, Befehl und Steuerung nicht. Damit erreichen `_`, jeder
Großbuchstabe und über die Wahltaste `@`, `|`, `~` und `\` den Filter.

Das Leerzeichen bleibt außen vor, und der geschlossene Datensatz sagt selbst, warum: die
Fallunterscheidung steht **hinter** der Belegungssuche, weshalb „eine Erweiterung keinem
belegten Kürzel etwas wegnehmen" kann. Genau diese Eigenschaft, dort eine Zusage, ist hier
die Ursache. Die beiden Fälle sind komplementär und nicht dasselbe: dort ging es um
unbelegte Kombinationen, hier um eine belegte.

## Der Zielkonflikt, den eine Behebung auflösen muss

Dieselbe Taste kann nicht zugleich markieren und ein Leerzeichen tippen. Beide Funktionen
sind zugesagt, die Markierung in C2 der ersten Runde, der Filter in der Runde 10. Die
Auflösung berührt damit eine bestehende Zusage, gleich welchen Weg sie nimmt, und ist ein
Nutzerentscheid und keine Sache des Umsetzers.

Drei Wege sind denkbar. Der Preis steht bei jedem, und keiner ist hier gewählt.

1. **Die Bedeutung hängt an der Lage, wie beim Rückschritt.** Steht ein Filtertext, tippt die
   Leertaste ein Leerzeichen; sonst markiert sie. Der Präzedenzfall steht im Baum und ist
   erprobt: `crates/krk-ui/src/kommandos/rueckschritt.rs` trennt drei Fälle derselben Taste
   als reine Funktion mit einem Rufer. **Preis:** bei stehendem Filtertext ist die Markierung
   über die Leertaste nicht mehr erreichbar, und filtern und dann markieren ist gerade der
   naheliegende Gebrauch des Filters. Der Preis wiegt hier schwerer als beim Rückschritt,
   wo der zweite Fall das Löschen war und nicht ein Alltagsgriff.
2. **Die Markierung zieht auf eine andere Kombination.** Der Filter bekommt die nackte
   Leertaste, das Markieren etwa `shift+space` oder eine Taste ohne Norton-Vorbild.
   **Preis:** die Leertaste zum Markieren ist die Gewohnheit aus Norton Commander, und die
   Belegungsdatei begründet ihre Kombinationen durchweg mit ihren Vorbildern.
3. **Beides an der Leertaste, unterschieden durch eine Zusatztaste in die andere Richtung.**
   Die nackte Taste markiert weiter, eine Kombination tippt das Leerzeichen. **Preis:** ein
   Zeichen des Filtertextes bekäme einen Eingabeweg, den kein anderes Zeichen hat, und der
   Nutzer müsste ihn wissen.

Ein vierter Gedanke ist zu verwerfen, bevor ihn jemand aufgreift: ein Ersatzzeichen im
Filtertext, das für das Leerzeichen steht. `traegt_die_folge` hat drei Rufer, seit der Runde
11 auch den Inhaltsfilter, und eine Übersetzungsregel im Vergleich träfe alle drei und machte
das Ersatzzeichen selbst unsuchbar.

## Was der Nutzer von Hand prüfen muss

Kein Agent kann am laufenden Bündel einen wirklichen Tastendruck auslösen; die Prüfung
verlangt KRK im Vordergrund. Nach einer Behebung sind zwei Fälle zu prüfen: ein Dateiname mit
Leerzeichen lässt sich im Filter finden, und der gewählte Weg zum Markieren wirkt weiterhin,
auch bei stehendem Filtertext.

## Nicht geprüft

Ob die Tippsuche der Belegungsansicht aus der Runde 7, der zweite Rufer der Zeichenregel,
dasselbe Verhalten zeigt. Sie liest `traegt_ein_dateiname` und hängt damit an derselben
Zeichenregel, aber ob die Leertaste dort über denselben Nachschlag läuft, ist am Baum nicht
nachgesehen.
