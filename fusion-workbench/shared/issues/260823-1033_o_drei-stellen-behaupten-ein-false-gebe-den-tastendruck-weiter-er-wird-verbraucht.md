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
