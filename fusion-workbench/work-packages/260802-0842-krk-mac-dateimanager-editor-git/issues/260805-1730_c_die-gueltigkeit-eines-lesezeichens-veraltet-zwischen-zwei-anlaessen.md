Die Gültigkeitsmarke eines Lesezeichens veraltet zwischen zwei Anlässen

---

Ein Lesezeichen wird in der Leiste als "(fehlt)" markiert, wenn sein Ordner nicht mehr existiert (C5). Geprüft wird das an drei Anlässen: beim Neuaufbau der Lesezeichenliste, bei jedem Ein- und Aushängen eines Datenträgers, und unmittelbar bevor eine Auswahl gemeldet wird. Zwischen zwei Anlässen zeigt die Leiste den Stand des letzten.

Der Fall, der bleibt: der Nutzer löscht in KRK selbst den Ordner, auf den ein Lesezeichen zeigt, sieht die Leiste an und findet den Eintrag unverändert schwarz. Erst wenn er ihn auswählt, wechselt die Marke auf grau und die Statuszeile nennt den Grund. Die Zusage aus C5 ist damit eingehalten — "die Auswahl meldet den Grund" trifft immer zu —, die Marke selbst kann aber für eine Weile falsch stehen.

---

**Warum es so gebaut ist.** Bei jedem Zeichendurchgang zu prüfen wäre ein Systemaufruf je Zeile und Bild, und die Leiste zeichnet weit häufiger, als der Nutzer in ihr etwas tut. Die drei Anlässe decken jeden Fall ab, in dem der Nutzer die Marke tatsächlich braucht.

**Was die saubere Lösung wäre.** Die Ordner der Lesezeichen in die Dateisystembeobachtung aus S14 aufnehmen. Der `FSEventStream` beobachtet heute die Ordner der sichtbaren Tabs; jeder Lesezeichenordner wäre ein weiterer Pfad, und sein Verschwinden meldete sich von selbst. Das kostet eine erweiterte Pfadliste in `auffrischung::sichtbare_ordner` und ein Neuaufsetzen des Stroms bei jeder Lesezeichenänderung. Ob es das wert ist, entscheidet der Nutzer; die Zusage aus C5 hält auch ohne.

Berührt: `crates/krk-ui/src/leistenmodell.rs` (`gueltigkeit_pruefen`), `crates/krk-ui/src/appkit/leiste.rs` (`gueltigkeit_nachziehen`), `crates/krk-ui/src/auffrischung.rs`.

---
Resolved: Ein vierter Anlass, kein vierter Mechanismus. Der Nutzerentscheid vom 260806-2300 wählt den engeren Weg gegenüber dem oben skizzierten über die Dateisystembeobachtung: die Gültigkeit wird geprüft, sobald eine Dateioperation aus C4 abgeschlossen ist. `Anwendungsdelegierter::vorgang_beenden` in `crates/krk-ui/src/appkit/anwendung.rs:2353` ruft dazu `Leistenquelle::gueltigkeit_nachziehen`, dieselbe Funktion, die schon die Auswahlmeldung ruft; `gueltigkeit_nachziehen` meldet über den Rückgabewert von `gueltigkeit_pruefen`, ob die Leiste neu zu zeichnen ist.

Begründung: der gemeldete Fall ist das Löschen in KRK selbst, und C9 hält bereits fest, dass eine abgeschlossene Dateioperation die Auffrischung von sich aus anstößt. Der vierte Anlass hängt sich an dieselbe Stelle und kostet keinen neuen Mechanismus, keine erweiterte Pfadliste in `auffrischung::sichtbare_ordner` und kein Neuaufsetzen des FSEvents-Stroms bei jeder Lesezeichenänderung. Der Weg über die Beobachtung deckte zusätzlich das fremde Programm ab, greift auf Netzpfaden nach C9 ohnehin nicht und wäre ein zweiter Mechanismus für eine Marke. `auffrischung.rs` ist deshalb unverändert.

Der Anlass greift auch nach einem Teilabbruch: ein abgebrochener Lauf trägt seinen Bericht über `abbruch_ohne_meldung_nachtragen` nach und erreicht `vorgang_beenden` auf derselben Bahn wie ein durchgelaufener.

Ausdrücklich offen geblieben: löscht ein **fremdes** Programm den Ordner, steht die Marke weiterhin bis zur nächsten Auswahl falsch. Die Zusage aus C5 hält auch dann, weil die Auswahl den Grund immer meldet. Diese Lücke steht ebenso im Programmtext, an der neuen Stelle und an `Leistenmodell::gueltigkeit_pruefen`.

Prüfung: `crates/krk-ui/src/leistenmodell.rs`, Test `nach_einer_dateioperation_meldet_die_pruefung_den_geloeschten_ordner`. Die auslösende Stelle selbst sitzt in AppKit und ist ohne Fenster nicht erreichbar; geprüft wird deshalb, was sie dort ruft, und dabei vor allem der Rückgabewert, an dem allein das Neuzeichnen hängt. `make check` läuft grün.
