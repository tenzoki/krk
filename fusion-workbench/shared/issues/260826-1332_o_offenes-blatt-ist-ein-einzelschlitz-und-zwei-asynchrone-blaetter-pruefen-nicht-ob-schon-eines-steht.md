`offenes_blatt` ist ein Einzelschlitz, und zwei asynchrone Blätter prüfen nicht, ob schon eines steht

---

`Anwendungsdelegierter::offenes_blatt` (`crates/krk-ui/src/appkit/anwendung.rs:707`) hält genau
einen `Blattgriff`. Neun Öffner schreiben ihn mit `= Some(griff)`, ohne den alten Wert
anzusehen. Sieben davon sind Tastenbefehle, die bei stehendem Blatt gar nicht erst zulässig
werden. Zwei kommen **vom Arbeitsfaden** und sehen keine Sperre: die Konfliktfrage
(`konflikt_fragen`, `:6505-6539`) und die Abschlussliste (`vorgang_beenden` →
`uebersprungen::zeigen`, `:6686-6692`). Steht in diesem Augenblick der Notizzettel oder die
Belegungsansicht, bekommt `beginSheetModalForWindow:` ein Fenster mit anhängendem Blatt, und der
Griff des sichtbaren Blattes ist überschrieben.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs`
**Cross-references:** `shared/issues/260826-1325_*_esc-im-stapel-umbenennen-blatt-mit-fokus-in-der-vorschautabelle-schliesst-das-blatt-nicht-sondern-leert-den-filter-dahinter.md` (derselbe Schlitz, anderer Verlust)

## Wie es erreichbar ist

Ein Kopierlauf läuft; C4 sagt die Oberfläche währenddessen bedienbar zu. Der Nutzer öffnet den
Notizzettel (`notizzettel_zeigen`, `:3895-3934`, ohne Vorgangsprüfung) oder die
Belegungsansicht (`belegung_ansehen`, `:3805-3820`). Dann trifft der Lauf einen Konflikt.
`konflikt_fragen` ruft `konflikt::zeigen` und schreibt `offenes_blatt = Some(konfliktgriff)`
(`:6539`); der Griff des Zettels ist damit fort.

`inference:` Was AppKit mit dem zweiten `beginSheetModalForWindow:` tut, ist am Bündel nicht
gemessen. Nach `NSWindow.h` (`sheets`, seit 10.9) wird ein weiteres Blatt eingereiht und
erscheint, sobald das erste zugeht. Unter dieser Annahme geht es so weiter: der Nutzer schließt
den Zettel mit `Esc` über den Zettelwächter; der Abschlussblock ruft
`zettel_blatt_geschlossen` (`:4185-4196`), und das setzt `offenes_blatt = None` (`:4188`) —
jetzt ist der **Konfliktgriff** fort. Das Konfliktblatt erscheint. Sein Ersthelfer ist eine
Schaltfläche, also wird `Kommando::Abbrechen` zulässig, und `Esc` läuft in `abbrechen`
(`:5648`): kein Griff, zweiter Rang, `vorgang.zustand.abbrechen()`. Der Arbeitsfaden wartet
aber auf die Konfliktantwort und liest das Abbruchkennzeichen erst danach; das Blatt bleibt
stehen und ist mit `Esc` nicht mehr zu schließen, weil KRK die Taste verbraucht, bevor die
Tastenentsprechung der Schaltfläche „Abbrechen" sie sieht. Übrig bleiben Maus und Return
(= Überspringen).

Umgekehrt ist es gesperrt: steht das Konfliktblatt, kommt weder `Notizzettel` noch `F1` durch,
und `beenden_erlauben` (`:7606-7608`) antwortet bei stehendem Blatt `TerminateCancel`, statt
die Nachfrage als zweites Blatt einzureihen. Diese eine Stelle kennt die Frage also; die zwei
asynchronen Öffner kennen sie nicht.

## Was `blatt_steht` dabei sieht

`blatt_steht` (`:3005-3011`) fragt `attachedSheet`, also das **vorderste** Blatt. Für die
Zulässigkeit ist das richtig; für „darf ein zweites eingereiht werden" ist es keine Antwort.

## Denkbarer Weg

Die beiden asynchronen Öffner lassen die Meldung im `Vorgangszustand` stehen, solange
`blatt_steht` wahr ist, und holen sie beim nächsten Takt der Bündelung wieder ab — der Zustand
trägt sie ohnehin als `Option` (`stand.konflikt.take()`, `stand.bericht.take()`, `:6457-6461`).
Dann gibt es nie zwei Blätter, und der Schlitz bleibt ein Schlitz. Ein `assert!` oder eine
Meldung an der Schreibstelle von `offenes_blatt`, die einen noch gesetzten Griff sichtbar macht,
wäre die Messung dazu.
