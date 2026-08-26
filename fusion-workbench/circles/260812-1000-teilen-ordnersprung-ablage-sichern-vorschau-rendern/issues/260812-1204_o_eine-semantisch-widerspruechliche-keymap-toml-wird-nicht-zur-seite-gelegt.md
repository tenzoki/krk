Eine semantisch widersprüchliche `keymap.toml` wird nicht zur Seite gelegt

---

Schritt 1 dieser Runde legt eine beschädigte Ablagedatei zur Seite, und zwar in
`Ablage::laden`, Zweig `Grund::Beschaedigt`. Genau dort geht `keymap.toml` nur
zur Hälfte durch: Ist die Datei gültiges TOML, aber inhaltlich widersprüchlich —
eine unbekannte Funktionskennung, eine doppelte Funktion, eine falsch
geschriebene Kombination —, dann liest `Ablage::laden` sie ohne Beanstandung,
und erst `belegung::laden` (`crates/krk-core/src/tasten/belegung.rs:1225-1237`)
baut eine eigene `Ersetzung` mit `Grund::Beschaedigt`. Diese Stelle sieht das
Zur-Seite-Legen nicht und trägt seit Schritt 1 `Beiseite::Nicht`.

Die Folge ist derselbe Datenverlust, gegen den Festlegung D gerichtet ist, auf
einem zweiten Weg: Eine künftige Fassung von KRK, die eine Kommandokennung
umbenennt, liest die `keymap.toml` des Nutzers als widersprüchlich, arbeitet auf
der Auslieferungsbelegung weiter, und sobald der Nutzer die Belegungsansicht
öffnet und eine Taste ändert, schreibt `belegungsansicht_verlassen`
(`crates/krk-ui/src/appkit/anwendung.rs:2360-2390`) diese Auslieferungsbelegung
über seine Datei. Eine Sicherung gibt es dann nicht, weil beim Laden keine
angelegt wurde.

---

**Kontext**

- Der Weg ist enger als der von Festlegung D: er verlangt eine Änderung des
  Nutzers in der Belegungsansicht, während `bookmarks.toml` und `session.toml`
  beim gewöhnlichen Beenden geschrieben werden. Er ist deshalb nicht der Fall,
  den die Runde als erstes verhindern muss, aber es ist derselbe Fall.
- Der Plan schließt ihn ausdrücklich aus: Schritt 1 sagt „**nur** im Zweig
  `Grund::Beschaedigt`" in `Ablage::laden`, und Kriterium C3.3 begründet die
  Gleichbehandlung der vier Dateien damit, dass alle vier durch `Ablage::laden`
  gehen. Diese eine Prüfung geht eine Ebene höher.
- Der Datensatz
  `decisions/260812-1000_a_wie-heisst-die-zur-seite-gelegte-ablagedatei-und-was-geschieht-beim-zweiten-mal.md`
  bindet weiter: welcher Weg auch gewählt wird, es bleibt bei einem festen Namen
  und einer Sicherung, die nicht überschrieben wird.
- Zwei Wege stehen offen und sind beide nicht in diesem Schritt zu gehen: das
  Zur-Seite-Legen als eigene Funktion neben `Ablage::laden` anzubieten, die
  `belegung::laden` mit dem bereits gelesenen Text ruft, oder die semantische
  Prüfung in den Ladeweg zu ziehen. Der erste öffnet einen zweiten Aufrufer, der
  zweite ändert den Zuschnitt zwischen Ablage und Tastenmodul.
- Gefunden bei der Umsetzung von Schritt 1; nicht behoben, weil die Änderung den
  Schritt verlässt.

---
Also seen: 260826-1225 by coderev — gilt am Baumstand `004ff72` unverändert.
`tasten::belegung::laden` (`crates/krk-core/src/tasten/belegung.rs:1492-1512`) baut die
`Ersetzung` weiter eine Ebene über `Zugang::laden` und trägt `Beiseite::Nicht`; der Code
verweist inzwischen selbst auf diesen Datensatz (`belegung.rs:1507-1509`). Die beiden hier
offengelassenen Wege stehen beide noch offen: `Zugang::beiseite_legen` ist weiter privat
(`crates/krk-core/src/ablage/mod.rs:862`), und die semantische Prüfung wohnt weiter im
Tastenmodul.
