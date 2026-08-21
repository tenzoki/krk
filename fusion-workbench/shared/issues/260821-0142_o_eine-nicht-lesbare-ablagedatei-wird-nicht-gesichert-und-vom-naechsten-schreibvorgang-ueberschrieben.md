Eine nicht lesbare Ablagedatei wird nicht gesichert und vom nächsten Schreibvorgang überschrieben

---

Die zweite Gestalt des Lesezeichenverlusts vom 260820-2235. Die erste ist am 260821 behoben;
diese steht weiter, weil ihre Behebung eine Verhaltensänderung an `Zugang::sichern` oder an
seinen Aufrufern ist und eine Nutzerentscheidung braucht, die der Defektdatensatz nicht trägt.

---

**Gemessen am Baumstand `bb072a0`**, gelesen und nicht ausgeführt; die Stellen sind einzeln
gegen den Baum geprüft.

Liegt eine der vier TOML-Dateien da und lässt sich nicht lesen, liefert `Zugang::laden`
(`crates/krk-core/src/ablage/mod.rs`) den Auslieferungszustand mit `Grund::NichtLesbar` und
`Beiseite::Nicht`. Der Nutzer bekommt einen Satz in die Statuszeile, aber keine Sicherung. Das
ist für sich richtig: von einer Datei, die sich nicht öffnen ließ, gibt es keinen Inhalt zu
kopieren, und `Beiseite::Nicht` sagt genau das.

**Der Schaden entsteht danach.** `lesezeichen_aendern`
(`crates/krk-ui/src/appkit/anwendung.rs:1731-1742`) liest unter der Schreibsperre frisch,
bekommt die leere Liste, wendet **eine** Änderung darauf an und ruft `zugang.sichern`, sobald
der Ausgang `Ausgang::Geaendert` ist. Die Meldung aus `mit_meldung()` wird dabei mitgeführt und
angezeigt, aber sie steuert nichts: der Zweig unterscheidet allein nach dem Ausgang der
Änderung, nicht danach, ob die gelesene Liste aus der Datei stammt oder aus dem
Auslieferungszustand. Danach steht in `bookmarks.toml` genau ein Eintrag, und der alte Bestand
ist ohne Sicherung fort — derselbe Verlauf, den der Modulkopf von `ablage/mod.rs` unter „Eine
beschädigte Datei wird zur Seite gelegt" beschreibt, nur über den Zweig, den die Runde 6 nicht
abgedeckt hat.

Der Vorgang setzt voraus, dass die Datei erst unlesbar ist und der spätere Schreibvorgang dann
gelingt. Ein entzogenes Leserecht bei erhaltenem Schreibrecht auf dem Ordner leistet das:
`atomar::schreiben` legt eine Nachbardatei an und benennt sie um, fasst die unlesbare Datei
also nie lesend an.

**Warum die Behebung nicht mit dem Defekt vom 260820-2235 zusammen lief.** Die naheliegende
Regel lautet „eine Datei, die sich nicht lesen ließ, darf nicht überschrieben werden". Sie ist
schmal formuliert und in der Wirkung nicht schmal: der Lesezeichenbefehl des Nutzers täte dann
nichts, und was er stattdessen sieht, ist zu entscheiden. Drei Fragen hängen daran, und keine
ist aus dem Baum zu beantworten:

- Was sieht der Nutzer, der `cmd+d` drückt, während `bookmarks.toml` unlesbar ist? Die heutige
  Startmeldung steht in der Statuszeile und ist bis dahin längst überschrieben.
- Gilt die Regel für die Dauer der Sitzung oder je Durchgang? `Zugang` hält kein Gedächtnis;
  `sichern` bekommt Datei und Wert und weiß von einem früheren `laden` nichts.
- Gilt sie auch für `session.toml`, die im Takt geschrieben wird? Dort hieße „nicht
  überschreiben", dass der Sitzungsschreiber bis zum Beenden jeden Takt verwirft.

**Schwere:** mittel. Der Verlauf verlangt eine unlesbare Datei bei schreibbarem Ordner und ist
damit seltener als die erste Gestalt; dafür gibt es keine Sicherung, aus der sich der Bestand
zurückholen ließe.

**Gefunden:** analyst, forensische Untersuchung „Lesezeichen nach Installation weg" am
260820-2235, als zweite Gestalt desselben Datensatzes; hier als eigener Datensatz abgetrennt,
weil der Nutzer am 260820-2250 die Behebung der ersten Gestalt beauftragt hat.

**Betroffen:** `crates/krk-core/src/ablage/mod.rs` (`Zugang::laden`, Zweig `Grund::NichtLesbar`),
`crates/krk-ui/src/appkit/anwendung.rs:1731-1742`

**Domain:** code

**Verwandt:** `shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`
— die erste Gestalt, am 260821 behoben.
