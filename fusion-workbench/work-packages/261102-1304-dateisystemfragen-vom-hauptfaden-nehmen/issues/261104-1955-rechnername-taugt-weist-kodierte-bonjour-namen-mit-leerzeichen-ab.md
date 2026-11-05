`rechnername_taugt` weist einen prozentkodierten Rechnernamen mit Leerzeichen ab, und damit Bonjour-Namen, die bis 3.14.1 einhängten
---
Schwere: Hoch. Seit `7ab5e891`, ausgeliefert in 3.14.2. `url_aus` fragt das dekodierte `host`, und ein gültig kodierter Name mit Leerzeichen gilt als untauglich. Eine solche Verbindung lässt sich weder anlegen noch öffnen.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg im Quelltext.** `crates/krk-ui/src/appkit/verbinden.rs:764` bis `:770`: `url_aus` liest `adresse.host()` und gibt ohne `rechnername_taugt` kein `NSURL` zurück. `:776` bis `:780`: `zeichen.is_whitespace()` macht den Namen untauglich. Die Probe `:810` bis `:811` hält `smb://nas%20local/x` als abzuweisen fest. Abnehmer von `url_aus`: `verbinden.rs:564` (Einhängen), `:732` (Übergeben) und `blaetter/verbindung.rs:165` (Prüfregel des Blattes, `Entwurfsfehler::KeineUrl`). Es trifft also das Anlegen und das Öffnen.

**Gemessen am 261104 auf macOS 27.2** mit einem Probeprogramm gegen `URLWithString:encodingInvalidCharacters:`:

```
smb://Mein%20NAS._smb._tcp.local/Daten           => host=[Mein NAS._smb._tcp.local]
afp://Time%20Capsule._afpovertcp._tcp.local/Data => host=[Time Capsule._afpovertcp._tcp.local]
smb://Kai%E2%80%99s%20Mac._smb._tcp.local/       => host=[Kai’s Mac._smb._tcp.local]
smb://nas local/x                                => host=[nas local] abs=[smb://nas%20local/x]
```

Das `host` ist dekodiert. Die Form, in der der Nutzer den Namen getippt hat, ist daran nicht mehr zu erkennen, und eine rohe und eine kodierte Eingabe fallen zusammen.

**Fehlerbild.** Ein Nutzer trägt `Mein%20NAS._smb._tcp.local/Daten` ein. Das Blatt weist die Adresse ab. Eine schon gespeicherte Verbindung dieser Form öffnet nicht mehr.

**inference, nicht gemessen:** Der Bonjour-Dienstname eines Mac oder NAS darf Leerzeichen tragen, und das System nennt solche Freigaben in dieser kodierten Form. An diesem Gerät war keine Freigabe eingehängt, und ein Einhängen über NetFS mit einem solchen Namen ist nicht gefahren. Ebenso wenig gemessen: dass `smb://nas%20local/x` vor macOS 27.2 einen `NSURL` ergab; das folgt aus der Bauart von `URLWithString:`, weil die Zeichenkette gültig kodiert ist.

**Verlangt war der Fall nicht.** Die rote Probe von 27.2 betraf `smb://nas local/x`, also das rohe Leerzeichen. Der Commit `7ab5e891` nennt die Erweiterung auf kodierte Namen als eigene Entscheidung.

**Zweite Stelle desselben Mechanismus, drift:** `crates/krk-core/src/heimordner/verbindung.rs:56` bis `:58` sagt weiter, „was auch kodiert keine URL ergibt, etwa ein Leerzeichen im Rechnernamen“. Auf 27.2 ergibt es eine URL, und die Abweisung kommt von `rechnername_taugt`. `verbinden.rs:747` ist nachgezogen, diese Stelle nicht. Für `blaetter/verbindung.rs:153` gilt die Aussage weiter.

**Richtung.** Die Frage am Text der Eingabe stellen, bevor `NSURL` kodiert: ein rohes Leerzeichen oder Steuerzeichen im Rechnerteil der Zeichenkette weist ab, ein gültiges `%20` nicht. Das `host` kann die Unterscheidung nicht liefern.

**Abnahme.** `gezeigt("smb://Mein%20NAS._smb._tcp.local/Daten")` ist `Some` und `gezeigt("smb://nas local/x")` ist `None`, auf macOS 27.2 und in der Probe in `verbinden.rs`. Der Satz in `verbindung.rs` nennt den Stand. Ob NetFS einen solchen Namen einhängt, prüft der Nutzer an einer Freigabe.

---
Resolved: `url_aus` (`crates/krk-ui/src/appkit/verbinden.rs`) prueft den Rechnerteil jetzt an der rohen Zeichenkette, bevor `NSURL` kodiert, sodass ein rohes Leerzeichen, Steuerzeichen oder eines der Zeichen `<>"\^`{|}` abweist und ein gueltiges `%20` eine URL ergibt; ungemessen bleibt das Einhaengen ueber NetFS mit einem solchen Namen, das an keiner Freigabe gefahren ist.
