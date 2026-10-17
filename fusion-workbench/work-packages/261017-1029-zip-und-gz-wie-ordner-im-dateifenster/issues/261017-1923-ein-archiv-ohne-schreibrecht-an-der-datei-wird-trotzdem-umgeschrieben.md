Ein Archiv ohne Schreibrecht an der Datei wird trotzdem umgeschrieben
---
Ob ein Archiv beschreibbar ist, entscheidet allein das Format (`Archivformat::beschreibbar`). Geschrieben wird über einen Zwischennamen und ein `rename` im Ordner, und dafür reicht das Schreibrecht am Ordner. Eine Archivdatei mit `0o444`, die der Nutzer mit `chmod a-w` oder über „Informationen“ gegen Änderungen gesperrt hat, schreibt KRK ohne Rückfrage neu, wieder mit `0o444`. Lokal öffnet KRK eine solche Datei nie zum Schreiben. Der Plan sagt in E12 nur, dass das Recht an der Datei **allein** nicht genügt, nicht ob es nötig ist.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (eine Schutzabsicht des Nutzers wird übergangen; kein Verlust, der Umbau schreibt, was der Nutzer verlangt)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/archiv/format.rs`, `beschreibbar`: `Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz => true`.
- `crates/krk-core/src/archiv/ersetzen.rs`, `Zwischendatei::anlegen`: `OpenOptions::new().read(true).write(true).create_new(true).mode(rechte)`. Das gelingt auch bei `rechte = 0o444`, denn die Rechte gelten erst für das nächste Öffnen. Danach ersetzt `fs::rename` das Original.
- `inference:` Ein Archiv mit dem Finder-Kennzeichen „geschützt“ (`uchg`) ist sicher, denn `rename` auf eine solche Datei scheitert mit `EPERM`. Nicht nachgestellt.

**Behebung (Richtung)**

Eine von zwei Richtungen:

- Der Umbau fragt am Deskriptor des alten Archivs, ob der Eigentümer es schreiben darf (`mode & 0o200`, `access(W_OK)`). Fehlt das Recht, endet er vor dem ersten Byte mit eigenem Grund, „das Archiv ist schreibgeschützt“.
- Der Nutzer entscheidet ausdrücklich, dass ein Archiv wie ein Ordner gilt, also allein das Recht am Ordner zählt. Dann sagt `HowTo.md` es.

**Abnahme**

Erste Richtung: eine Probe setzt eine Archivdatei auf `0o444`, legt darin einen Ordner an, und danach ist das Archiv Byte für Byte unverändert, mit Grund im Bericht. Zweite Richtung: der Satz steht in `HowTo.md`, und der Entscheid ist verlinkt.
