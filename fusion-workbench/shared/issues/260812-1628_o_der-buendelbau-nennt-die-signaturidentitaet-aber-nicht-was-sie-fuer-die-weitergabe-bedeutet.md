Der Bündelbau nennt die Signaturidentität, aber nicht, was sie für die Weitergabe bedeutet

---

`cargo xtask bundle` meldet nach dem Signieren genau eine Zeile
(`xtask/src/sign.rs:172-175`):

```
Signiert mit "Apple Development: Kai Stalmann (FJ8U4B3QAC)", gefunden ueber den
Schluesselbund als einzige gueltige Identitaet.
```

Die Zeile ist wahr und vollständig über das, was geschehen ist. Sie sagt nichts über die
Folge: ein so signiertes Bündel wird von Gatekeeper auf **jeder anderen Maschine**
abgewiesen. Wer die Zeile liest, hat keinen Anlass, das zu vermuten — sie klingt nach
gelungener Signatur, und sie ist eine.

---

**Am 260812 eingetreten.** Der Nutzer hat `target/KRK.app` auf einen zweiten Mac kopiert;
dort ist es als mögliche Schadsoftware abgewiesen worden. Nachgemessen an diesem Baum:

```
codesign -dvv target/KRK.app   → Authority=Apple Development: Kai Stalmann (FJ8U4B3QAC)
spctl -a -vvv -t exec …        → rejected
xcrun stapler validate …       → does not have a ticket stapled to it
lipo -info …                   → x86_64 (nicht universell)
```

Der richtige Weg ist gebaut und dokumentiert: `cargo xtask release` fährt sechs Stationen bis
zur Beglaubigung und zum angehefteten Ticket, und `README.md` beschreibt sie. Der Nutzer ist
dort nicht hingekommen, weil ihn nichts darauf gestoßen hat. Die Kosten waren ein
fehlgeschlagener Versuch auf einem zweiten Gerät und die Vermutung, KRK sei beschädigt.

**Ein zweiter Teil derselben Lücke:** `bundle` baut nicht universell, sondern für die
Architektur der Baumaschine. Auch das steht in keiner Ausgabe. Auf einem Apple-Silicon-Mac
braucht das Ergebnis dieses Baumes deshalb zusätzlich Rosetta, selbst wenn die Signatur
stimmte.

Zwei Wege stehen offen. Die Meldung um einen Satz erweitern, der die Folge nennt, etwa „nur
auf dieser Maschine lauffähig; für die Weitergabe `cargo xtask release`" — das kostet drei
Zeilen und trifft jeden Bau. Oder die Ausgabe von `bundle` am Ende um eine Zusammenfassung
ergänzen, die Identitätsart und Architektur nennt und die Weitergabe adressiert. Die zweite
trägt mehr und ist teurer.

Der Fehler wäre, die Zeile zu verschärfen, ohne den Fall der Auslieferungsidentität zu
bedenken: signiert jemand `bundle` über `KRK_SIGN_IDENTITY` mit einer Developer-ID, ist der
Warnsatz falsch. Die Meldung muss die Art der Identität lesen, nicht den Unterbefehl.

Herkunft: gemeinsamer Speicher. Betrifft `xtask` und den Bauweg des ganzen Projekts, nicht
die Directive der Runde 6.
