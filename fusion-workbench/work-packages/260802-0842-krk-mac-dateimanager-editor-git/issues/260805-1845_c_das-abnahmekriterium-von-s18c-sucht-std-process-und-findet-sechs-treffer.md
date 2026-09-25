Das Abnahmekriterium von S18c sucht `std::process` und findet sechs Treffer, die nichts mit Unterprozessen zu tun haben

---

Das letzte Abnahmekriterium von Schritt 18c verlangt: "`grep -rn
'Command::new\|std::process' crates/krk-ui/src crates/krk-core/src` liefert
keinen Treffer." Das kann nicht aufgehen. Der zweite Teilausdruck trifft jedes
`std::process::exit` und jedes `std::process::id`, und beide stehen seit
früheren Schritten im Programm, ohne dass irgendwo ein Unterprozess entsteht.

---

Am 260805-1845 gemessen, sechs Treffer, alle älter als dieser Schritt:

- `crates/krk-ui/src/kommandos/pfadeingabe.rs:132` — `std::process::id()` im
  Namen eines Prüfordners,
- `crates/krk-ui/src/main.rs:68` — `std::process::exit(AUFRUFFEHLER)`,
- `crates/krk-ui/src/appkit/anwendung.rs:1974, 1990, 2035, 2039` — die vier
  Ausgänge des Messmodus aus S8 und S21.

Gemeint ist die Zusage, dass KRK das Terminal nicht über einen Unterprozess
ruft, sondern über `NSWorkspace`. Diese Zusage hält; der zutreffende Zähler ist
`grep -rn 'Command::new\|process::Command' crates/krk-ui/src
crates/krk-core/src`, und er liefert null Treffer.

Es ist dieselbe Falle wie bei den Kriterien, die `grep unsafe` verlangten
(`issues/260802-1810_c_abnahmekriterium-mit-grep-unsafe-kann-nicht-aufgehen.md`
und `issues/260803-1200_c_abnahmekriterium-von-schritt-6-traegt-denselben-grep-fehler.md`):
ein Suchmuster, das die Zusage nur ungefähr trifft und deshalb an Stellen
anschlägt, die sie gar nicht meint. Zu berichtigen ist das Kriterium im Plan.

---
Resolved: Das Kriterium sucht jetzt Command::new|process::Command statt Command::new|std::process und trifft damit 0 statt 11 Stellen (Plannachzug 260806-1313). Am Bestand nachgemessen.
