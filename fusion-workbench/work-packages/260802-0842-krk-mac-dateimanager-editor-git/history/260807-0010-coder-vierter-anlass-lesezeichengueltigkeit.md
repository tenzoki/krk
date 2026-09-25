# Der vierte Anlass für die Gültigkeitsmarke eines Lesezeichens (D5, Turn 25)

**Agent:** coder
**Status:** Complete
**Quelle:** `issues/260805-1730_c_die-gueltigkeit-eines-lesezeichens-veraltet-zwischen-zwei-anlaessen.md`
**Nutzerentscheid:** 260806-2300 — der engere Weg, nicht der über die Dateisystembeobachtung

## Was umgesetzt wurde

Die Gültigkeit der Lesezeichenordner (C5) wurde bis dahin an drei Anlässen geprüft: beim Neuaufbau der Lesezeichenliste, beim Ein- und Aushängen eines Datenträgers, und unmittelbar bevor eine Auswahl gemeldet wird. Dazu kommt jetzt ein vierter: sobald eine Dateioperation aus C4 abgeschlossen ist.

Der Anlass ist ein vierter **Aufrufer derselben Funktion** und kein vierter Mechanismus. `Anwendungsdelegierter::vorgang_beenden` ruft `Leistenquelle::gueltigkeit_nachziehen`, dieselbe Funktion, über die schon die Auswahlmeldung läuft; sie fragt `Leistenmodell::gueltigkeit_pruefen` und zeichnet die Leiste nur dann neu, wenn die Prüfung eine Änderung meldet.

## Geänderte Dateien

| Datei | Zeilen | Was |
|---|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs` | 2325–2353 | der vierte Anlass samt Begründung; der Aufruf steht in `vorgang_beenden`, nach der Ordnerauffrischung und der Auswahlnachführung des Stapel-Umbenennens, vor der Übersprungenliste |
| `crates/krk-ui/src/appkit/leiste.rs` | 222–238 | `gueltigkeit_nachziehen` von privat auf `pub`, Kopfkommentar nennt beide Aufrufer |
| `crates/krk-ui/src/leistenmodell.rs` | 172–194 | Kopfkommentar von `gueltigkeit_pruefen`: vier Anlässe statt drei, dazu die offen gebliebene Lücke |
| `crates/krk-ui/src/leistenmodell.rs` | 637–673 | neuer Test `nach_einer_dateioperation_meldet_die_pruefung_den_geloeschten_ordner` |

`crates/krk-ui/src/auffrischung.rs` ist unverändert: keine neue Pfadliste, kein neuer Ordner in der Beobachtung.

## Warum hier und nicht in der Dateisystembeobachtung

Der gemeldete Fall ist das Löschen in KRK selbst, und C9 hält bereits fest, dass eine abgeschlossene Dateioperation die Auffrischung von sich aus anstößt. Der vierte Anlass hängt sich an dieselbe Stelle und kostet keinen neuen Mechanismus, keine erweiterte Pfadliste in `auffrischung::sichtbare_ordner` und kein Neuaufsetzen des FSEvents-Stroms bei jeder Lesezeichenänderung. Der Weg über die Beobachtung deckte zusätzlich das fremde Programm ab, greift auf Netzpfaden nach C9 ohnehin nicht und wäre ein zweiter Mechanismus für eine Marke.

## Der Teilabbruch

Geprüft und bestätigt: der Anlass greift auch, wenn die Operation abgebrochen wurde. Ein abgebrochener Lauf sendet entweder `Meldung::Fertig` mit `Abschluss::Abgebrochen`, oder — schließt der Meldekanal ohne diese Meldung — `vermitteln` trägt den Bericht über `abbruch_ohne_meldung_nachtragen` nach (`anwendung.rs:2709`). Beide Wege münden in `vorgang_beenden`. Ein teilweise gelöschter Ordner ist entweder fort oder noch da, und beides will die Marke wissen.

## Was ausdrücklich offen bleibt

Löscht ein **fremdes** Programm den Ordner, steht die Marke weiterhin bis zur nächsten Auswahl falsch. Die Zusage aus C5 hält auch dann, weil die Auswahl den Grund immer meldet. Die Lücke steht an drei Stellen: im geschlossenen Defekt, im Kommentar an der neuen Stelle und am Kopf von `gueltigkeit_pruefen`.

## Prüfung

Die auslösende Stelle sitzt in AppKit (`Anwendungsdelegierter`) und ist ohne Fenster nicht erreichbar; die Attrappe `Dateifenstersicht` aus `auffrischung.rs` deckt sie nicht ab, weil sie die Dateifenster nachbildet und nicht die Leiste. Der Test setzt deshalb auf der Ebene an, die ohne AppKit prüfbar ist: `Leistenmodell::gueltigkeit_pruefen` erkennt den gelöschten Ordner **und meldet ihn über den Rückgabewert**, denn allein daran hängt das Neuzeichnen. Ein Umbau, nur um den AppKit-Aufruf testbar zu machen, wurde nicht erzwungen.

`make check` läuft grün: Bau, 502 Tests im Arbeitsbereich (davon einer übersprungen), `clippy -D warnings`, `fmt --check`.

## Nicht zur Aufgabe gehörig, aber aufgefallen

- `Leistenquelle::orte_setzen` (`leiste.rs:213`) ruft `gueltigkeit_pruefen` direkt und wirft dessen Rückgabewert weg, statt `gueltigkeit_nachziehen` zu rufen. Das ist heute richtig, weil das anschließende `nachziehen()` die Tabelle ohnehin vollständig neu zeichnet; es ist aber der einzige der vier Anlässe, der die Funktion nicht über denselben Weg erreicht. Kein Defekt, nur eine Ungleichheit im Bild.
- `Leistenmodell::lesezeichen_setzen` prüft die Gültigkeit implizit beim Aufbau der `Gemerkt`-Einträge, nicht über `gueltigkeit_pruefen`. Auch das ist eine Ungleichheit derselben Art: vier Anlässe, drei Wege zur Prüfung.
