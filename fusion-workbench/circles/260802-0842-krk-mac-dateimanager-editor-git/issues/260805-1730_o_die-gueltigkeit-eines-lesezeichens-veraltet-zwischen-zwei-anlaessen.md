Die Gültigkeitsmarke eines Lesezeichens veraltet zwischen zwei Anlässen

---

Ein Lesezeichen wird in der Leiste als "(fehlt)" markiert, wenn sein Ordner nicht mehr existiert (C5). Geprüft wird das an drei Anlässen: beim Neuaufbau der Lesezeichenliste, bei jedem Ein- und Aushängen eines Datenträgers, und unmittelbar bevor eine Auswahl gemeldet wird. Zwischen zwei Anlässen zeigt die Leiste den Stand des letzten.

Der Fall, der bleibt: der Nutzer löscht in KRK selbst den Ordner, auf den ein Lesezeichen zeigt, sieht die Leiste an und findet den Eintrag unverändert schwarz. Erst wenn er ihn auswählt, wechselt die Marke auf grau und die Statuszeile nennt den Grund. Die Zusage aus C5 ist damit eingehalten — "die Auswahl meldet den Grund" trifft immer zu —, die Marke selbst kann aber für eine Weile falsch stehen.

---

**Warum es so gebaut ist.** Bei jedem Zeichendurchgang zu prüfen wäre ein Systemaufruf je Zeile und Bild, und die Leiste zeichnet weit häufiger, als der Nutzer in ihr etwas tut. Die drei Anlässe decken jeden Fall ab, in dem der Nutzer die Marke tatsächlich braucht.

**Was die saubere Lösung wäre.** Die Ordner der Lesezeichen in die Dateisystembeobachtung aus S14 aufnehmen. Der `FSEventStream` beobachtet heute die Ordner der sichtbaren Tabs; jeder Lesezeichenordner wäre ein weiterer Pfad, und sein Verschwinden meldete sich von selbst. Das kostet eine erweiterte Pfadliste in `auffrischung::sichtbare_ordner` und ein Neuaufsetzen des Stroms bei jeder Lesezeichenänderung. Ob es das wert ist, entscheidet der Nutzer; die Zusage aus C5 hält auch ohne.

Berührt: `crates/krk-ui/src/leistenmodell.rs` (`gueltigkeit_pruefen`), `crates/krk-ui/src/appkit/leiste.rs` (`gueltigkeit_nachziehen`), `crates/krk-ui/src/auffrischung.rs`.
