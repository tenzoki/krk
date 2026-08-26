`Esc` im Stapel-Umbenennen-Blatt mit dem Fokus in der Vorschautabelle schließt das Blatt nicht, sondern leert den Filter dahinter oder bricht einen laufenden Vorgang ab

---

Sechs Blätter, die `anwendung.rs` öffnet, legen ihren Griff nicht in `offenes_blatt` ab. Solange der Ersthelfer eines solchen Blattes ein Textfeld ist, deckt der Fokusvorbehalt das: `Esc` geht an AppKit und die Schaltfläche „Abbrechen" beantwortet es. Sobald der Ersthelfer **kein** Textfeld ist — im Stapel-Umbenennen-Blatt ist das die Vorschautabelle, die der Modulkopf ausdrücklich als Tabulator-Ziel führt —, kommt `Kommando::Abbrechen` durch die Zulässigkeitsregel, `abbrechen()` findet kein Blatt und fällt auf Rang 2 (laufenden Vorgang abbrechen) oder Rang 3 (Filtertext des aktiven Tabs leeren). Der Tastendruck ist danach verbraucht; die Schaltfläche des Blattes sieht ihn nie.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Kette am Baum (HEAD `7ac511a`)

1. `Anwendungsdelegierter::stapel_umbenennen` (`crates/krk-ui/src/appkit/anwendung.rs:5828-5832`) ruft `stapelumbenennen::zeigen(…)`. Die Funktion liefert nichts zurück (`crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs:388-394`); ihr Rumpf endet auf `blatt.zeigen(fenster, …)` (`stapelumbenennen.rs:434`), und `Blatt::zeigen` lässt den Griff fallen: `let _griff = self.zeigen_mit_wahl(…)` (`blaetter/mod.rs:765`). `offenes_blatt` bleibt `None`.
2. Der Modulkopf des Blattes sagt, dass die Vorschautabelle den Fokus bekommen kann: „Der Tabulator laeuft durch die vier Felder und die Vorschau und wieder zurueck … In der Vorschau blaettern die Pfeiltasten, sobald sie den Fokus hat. … die Escape-Taste bricht ab" (`stapelumbenennen.rs:35-40`). Der Waechter hängt allein an den vier Feldern (`stapelumbenennen.rs:402-409`).
3. Mit dem Fokus in der Tabelle ist der Ersthelfer eine `NSTableView`. `ersthelfer_gehoert_appkit` (`crates/krk-ui/src/appkit/ereignisse.rs:702-718`) fragt allein `NSTextView`, `NSTextField`, `NSText` und antwortet `false`.
4. `zulaessigkeit::zulaessig` (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:177-185`): `blatt_steht` ist wahr, aber `waehrend_blatt_erlaubt(Abbrechen)` ebenso (`kommandos/operationen.rs:283-285`); das Schlüsselfenster ist `BlattAmHauptfenster` und gehört KRK (`anwendung.rs:3038-3041`, `1077-1082`); der Fokus ist `Anderswo` (`anwendung.rs:5942-5947`), und `fokus::wirkt(Ueberall, Anderswo)` ist wahr (`kommandos/fokus.rs:345`). `Abbrechen` ist zulässig.
5. `Anwendungsdelegierter::abbrechen` (`anwendung.rs:3648-3673`): Rang 1 findet `offenes_blatt == None`; Rang 2 bricht einen laufenden Vorgang ab, falls einer läuft — `stapel_umbenennen` prüft `vorgang_laeuft` beim Öffnen nicht, erst `stapel_beauftragen` beim Bestätigen (`anwendung.rs:5856`); Rang 3 ruft `filter_leeren()` am sichtbaren Tab des aktiven Dateifensters.
6. `kommando_ausfuehren` liefert `true` (`anwendung.rs:3407`), der Abgriff schluckt das Ereignis, und der Schaltfläche mit `Taste::Escape` (`blaetter/mod.rs:337`) kommt es nie an.

## Derselbe Zuschnitt an sechs Stellen

Blätter aus `anwendung.rs` ohne Eintrag in `offenes_blatt`: `namenseingabe::frei_zeigen` (`anwendung.rs:2011`, `2165`), `namenseingabe::zeigen` (`5698`), `stapelumbenennen::zeigen` (`5828`), `blaetter::zeilennummer::zeigen` (`7172`), `blaetter::suche::zeigen` (`7207`). Die fünf Eingabeblätter tragen Textfelder und sind geschützt, solange der Feldeditor den Rang hält; mit vollständiger Tastaturnavigation (Tab auf eine Schaltfläche) fällt derselbe Schutz. Dagegen tragen alle sechs Blätter, die einen `Blattgriff` zurückgeben (`loeschbestaetigung`, `konflikt`, `uebersprungen`, `ungesichert`, `zettel`, `belegungsansicht`), ihren Griff in `offenes_blatt`.

## Was am Baum belegt ist und was nicht

Belegt ist die Kette bis zur Zulässigkeit und dem Rumpf von `abbrechen`. Nicht am laufenden Bündel gemessen ist, welche Ansicht AppKit als Ersthelfer führt, wenn die Vorschautabelle über den Schlüsselring den Fokus bekommt; der Abnahmelauf verlangt KRK im Vordergrund. Seit der Runde 7 (Abgriff schluckt jeden zulässigen Befehl) ist `Esc` auf diesem Weg tot; seit der Runde 10 (dritter Rang von `abbrechen`, 260815) leert es zusätzlich den Filter des Tabs hinter dem Blatt.

## Vorschlag

`stapelumbenennen::zeigen` (und die fünf Eingabeblätter) liefern den `Blattgriff` wie die anderen sechs zurück, und `anwendung.rs` legt ihn in `offenes_blatt` ab; dann geht `Esc` denselben Weg wie in der Löschbestätigung. Alternativ: `abbrechen()` prüft `blatt_steht()` vor Rang 2 und 3 und tut bei stehendem, aber nicht registriertem Blatt nichts — das hält den Filter und den Vorgang, schließt das Blatt aber weiterhin nicht. Die erste Fassung schließt die Lücke, die zweite verkleinert sie.

Gefunden bei der Vollbaum-Durchsicht R7 an HEAD `7ac511a`.

Also seen: 260826-1340 by coderev (R10, die elf Blätter) — unabhängig gefunden, derselbe Befund, Schwere Hoch. Zwei Ergänzungen: erstens ist die Vorschautabelle ohne Systemeinstellung erreichbar, weil `schluesselring_legen` (`stapelumbenennen.rs:615-627`) sie ausdrücklich in den Tabring legt (`felder.stellen → tabelle → felder.suchen`), vier Tabs vom Ersthelfer; zweitens trägt `mod.rs:494-495` die Zusage „wer den Griff nicht braucht, laesst ihn fallen; das schadet nicht", die für die Lebensdauer des Blattes stimmt und für den Abbruchbefehl nicht — die Voraussetzung, dass jedes stehende Blatt seinen Griff hinterlegt, steht nirgends. Mein eigener Datensatz `260826-1331` ist zugunsten dieses hier zurückgenommen; der Typ `Blattgriff` ohne `#[must_use]` steht als eigener Niedrig-Befund in `shared/issues/260826-1335_*`.
