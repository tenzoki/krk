Drei Stellen behaupten, ein `false` gebe den Tastendruck weiter — er wird verbraucht

---

`Anwendungsdelegierter::kommando_ausfuehren` liefert seit der Runde 7 **immer** `true`. Drei
Doc-Kommentare sagen trotzdem, ein `false` aus dem Rumpf gebe den Tastendruck weiter, statt ihn
zu verbrauchen. Eine der drei stand vorher da, zwei sind mit `28cbb7b` hinzugekommen — eine
davon im Modulkopf des Moduls, das die Regel für `cmd+e` an genau einer Stelle führen soll.

---

**Am Baum gelesen und ohne Annahme entscheidbar**: der Rückgabewert steht im Quelltext.

## Der Code

`crates/krk-ui/src/appkit/anwendung.rs:3228-3233`, das Ende von `kommando_ausfuehren`:

```rust
            andere => self.bereichskommando(fokus, andere),
        };
        if gewirkt {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
        true
    }
```

Der Doc-Kommentar derselben Funktion (`anwendung.rs:2972-2982`) sagt es ausdrücklich: „**Liefert,
ob der Befehl zulaessig war, und nicht mehr, ob sein Rumpf etwas getan hat.** … Was der Rumpf
gemeldet hat, bleibt darunter erhalten und entscheidet weiterhin ueber die beiden
Nachwirkungen." Ein `false` aus einem Rumpf entscheidet also über `aufteilung_nachziehen` und
`sitzung_vormerken` — und über sonst nichts. Der Tastendruck ist in jedem Fall verbraucht.

## Die drei Stellen

**Neu, `crates/krk-ui/src/kommandos/rundweg.rs:117-119`:**

```
/// `None` heisst "von hier aus fuehrt kein Rundweg" und nicht "hier ist nichts
/// zu tun": der Aufrufer verbraucht den Tastendruck dann nicht, sondern reicht
/// ihn weiter, wie er es fuer jeden unbelegten tut.
```

**Neu, `crates/krk-ui/src/appkit/anwendung.rs:7020-7024`:**

```
/// Leiste und das stehende Blatt schon abgewiesen hat; `false` gibt den
/// Tastendruck dann weiter, statt ihn zu verbrauchen — dieselbe Antwort, die
/// [`Self::editor_oeffnen_lassen`] fuer die noch nicht gebaute Oberflaeche
/// gibt.
```

**Älter, `crates/krk-ui/src/appkit/anwendung.rs:6256-6258`** — die Quelle, aus der die zweite
zitiert:

```
/// die Oberflaeche noch nicht steht; die beiden Befehle darunter geben den
/// Tastendruck dann weiter, statt ihn zu verbrauchen.
```

## Der Baum widerspricht sich schon vorher

Fünfundvierzig Zeilen unter der dritten Stelle steht die richtige Auskunft
(`anwendung.rs:6303-6305`): „`true` verbraucht den Tastendruck, aus demselben Grund wie dort:
F4 auf leerer Auswahl gehoert nicht in die Menueleiste." Zwei Aussagen über denselben
Rückgabewert, keine drei Bildschirmseiten auseinander.

`28cbb7b` hat die falsche der beiden ausgewählt und in ein neues Modul kopiert. Das ist der
eigentliche Befund: eine falsche Aussage, die stehen bleibt, wird zur Quelle der nächsten.

## Was daran nicht kaputt ist

Kein Verhalten. Der `None`-Zweig von `rundweg` ist heute unerreichbar — `Wirkungsbereich::
Dateibereiche` weist die Leiste und das stehende Blatt vor der Regel ab, und die Probe
`der_wirkungsbereich_und_die_regel_lassen_dieselben_bereiche_durch` hält das fest. Der Schaden
ist, dass drei Stellen dem nächsten Leser eine Zusage machen, die das Programm nicht hält.

## Empfehlung

Alle drei auf dieselbe Aussage bringen: „`false` heißt allein, dass kein Nachzug der Aufteilung
und keine vorgemerkte Sitzung anfällt; der Tastendruck ist verbraucht, weil
`kommando_ausfuehren` seit der Runde 7 immer `true` liefert." Die dritte Stelle mitzunehmen ist
der Punkt — sonst wandert sie in den nächsten neuen Rumpf.

**Schwere:** Medium.

**Filed by:** coderev

---

In Arbeit: 260823-1137 durch coder. **Der Befund ist am Baum nachgeprueft und trifft
zu**: `Anwendungsdelegierter::kommando_ausfuehren` endet auf ein nacktes `true`
(`anwendung.rs`), und der Modulkopf von `appkit/ereignisse.rs` schreibt die Regel aus:
„Geschluckt wird, was zulaessig war, und nicht mehr, was gewirkt hat. Bis zur Runde 7
lautete die Grenze ‚ausgefuehrt'."

Alle drei genannten Stellen sagen jetzt: `false` heisst allein, dass kein Nachzug der
Aufteilung und keine vorgemerkte Sitzung anfaellt; der Tastendruck ist verbraucht.

**Vier weitere Stellen derselben Klasse, die dieser Datensatz nicht fuehrt**, alle mit
demselben Satz nachgezogen:

- `anwendung.rs`, `terminal_oeffnen`: „Ein `false` gaebe den Tastendruck an AppKit
  weiter, das mit ihm nichts anfangen kann."
- `anwendung.rs`, `weitere_instanz_starten`: derselbe Satz, ausdruecklich von der
  vorigen Stelle uebernommen.
- `anwendung.rs`, `bereichskommando`, Zweig `Fokus::Vorschau`: „der Tastendruck laeuft
  wie ein unbelegter weiter."
- `anwendung.rs`, `bereichskommando`, Zweig `Fokus::Editor`: „der Tastendruck laeuft
  unveraendert an AppKit weiter und wird in der Textflaeche zu einem Zeichen oder zu
  einer Bewegung der Schreibmarke." Das ist die folgenreichste der vier: ein Befehl mit
  `Wirkungsbereich::Ueberall` ist mit dem Fokus im Editor zulaessig und wird seit der
  Runde 7 geschluckt statt getippt. **Kein Defekt, sondern die Wahl der Runde 7**, aber
  der Kommentar sagte das Gegenteil; er sagt es jetzt richtig.
- `appkit/tabelle.rs`, `umbenennung_beginnen`: „dann ist der Tastendruck nicht
  verbraucht." Liegt auf demselben Kommandoweg ueber `Tabelle::kommando_ausfuehren` und
  `bereichskommando`.

**Nicht angefasst und geprueft richtig**: die Aussagen auf dem Zeichenweg
(`Ordnermodell::zeichen_anhaengen`, `::letztes_zeichen_weg`,
`Tabelle::filterzeichen_tippen`, der Zweig `Eingabe::Zeichen` in `eingabe_ausfuehren`).
Dort wird der Rueckgabewert wirklich bis zum Abgriff durchgereicht. Bleibt zum
Schliessen mit dem Commit.
