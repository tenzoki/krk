Ein achter `Wirkungsbereich` übersetzt ohne Eintrag im Beschriftungsfeld, der Doc-Kommentar sagt das Gegenteil

---

`stelle_in_den_sieben` (`crates/krk-core/tests/belegung.rs:1892-1908`) ist eine vollständige Fallunterscheidung, und ihr Doc-Kommentar leitet daraus ab, ein achter `Wirkungsbereich` übersetze „erst, wenn er auch hier und damit im Feld steht". Der zweite Halbsatz trägt nicht: der Übersetzer verlangt einen **Zweig**, und der darf jede Zahl liefern. `SIEBEN_BESCHRIFTUNGEN` ist ein `[(Wirkungsbereich, &str); 7]` mit fester Länge, das niemand mitzieht — ein achter Wert bliebe von allen drei Beschriftungsproben unberührt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/belegung.rs:1868-1890` (`SIEBEN_BESCHRIFTUNGEN`), `:1892-1908` (`stelle_in_den_sieben`, samt Doc-Kommentar), `:1910-1933`, `:1935-1952`, `:1954-1994`; `crates/krk-core/src/tasten/belegung.rs:322-332` (`beschriftung`)

## Was der Doc-Kommentar sagt

```
/// **Der Grund fuer diese zweite Fallunterscheidung ist die erste.** Eine
/// Aufzaehlung in einer Probe waechst nicht von selbst mit der Aufzaehlung im
/// Kern: ein achter Wert bekaeme in `Wirkungsbereich::beschriftung` seine Zeile
/// vom Uebersetzer abverlangt, in einem Feld darueber aber nicht. Diese
/// Funktion stellt das her — sie ist ebenfalls ohne Auffangzweig, also
/// uebersetzt ein achter Wert erst, wenn er auch hier und damit im Feld steht.
```

Der erste Teil stimmt. Der Schluss „und damit im Feld" ist der ungedeckte Schritt.

## Was wirklich geschieht

Ein achter Wert von `Wirkungsbereich` verlangt zwei Zeilen vom Übersetzer: eine in `beschriftung` (`src/tasten/belegung.rs:323-331`) und eine in `stelle_in_den_sieben`. Die zweite darf `7` liefern — oder `0`. Das Feld daneben ist auf sieben Einträge typisiert und wächst davon nicht mit; niemand vergleicht `SIEBEN_BESCHRIFTUNGEN.len()` mit der Zahl der Varianten, und es gibt nichts, worüber ein solcher Vergleich liefe.

Alle drei Proben iterieren über **das Feld** und nicht über die Varianten:

```rust
for (bereich, erwartet) in SIEBEN_BESCHRIFTUNGEN { … }          // :1911, :1955
for (stelle, (bereich, beschriftung)) in SIEBEN_BESCHRIFTUNGEN.into_iter().enumerate()  // :1936
```

Der achte Wert bekäme damit **keine** Prüfung: seine Beschriftung dürfte leer sein, dürfte einen senkrechten Strich tragen, der die Pipe-Tabelle in `~/Downloads/KRK-Tastenbelegung.md` zerbricht, und dürfte mit der eines anderen Bereichs übereinstimmen. Genau die drei Dinge, die die drei Proben ausschließen sollen.

Der Zweig in `stelle_in_den_sieben` würde dabei nie ausgeführt: die Funktion hat genau einen Rufer, und der steht innerhalb der Schleife über das Feld (`:1912`).

## Dieselbe Lücke wie bei `KENNUNGEN`

Das ist die Form, die `shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md` für `Kommando::KENNUNGEN` beschreibt: eine Liste, deren Vollständigkeit die Frage ist, wird von einer Probe geprüft, die über sie selbst läuft. Hier kommt hinzu, dass ein Doc-Kommentar die Lücke ausdrücklich als geschlossen bezeichnet — das ist der Unterschied und der Grund für einen eigenen Datensatz: der Leser wird hier aktiv in die falsche Richtung geschickt.

## Richtung

Die Bauform, die trägt, steht ein paar Zeilen darüber im selben Baum: `stelle_in_den_sieben` liefert schon den Index, also genügt der Weg über die Varianten statt über das Feld. Ein `const ALLE: [Wirkungsbereich; N]`, dessen Vollständigkeit ein `match` ohne Auffangzweig erzwingt, und die drei Schleifen laufen darüber statt über `SIEBEN_BESCHRIFTUNGEN`; das Feld bleibt die Quelle des erwarteten Texts. Alternativ eine Zusicherung, dass jeder Wert, den `stelle_in_den_sieben` benennen kann, auch im Feld steht — dann muss die Funktion allerdings von außen aufrufbar sein und nicht nur aus der Feldschleife heraus.

Bis dahin ist der Halbsatz „und damit im Feld" zu streichen, gleich welcher Weg gewählt wird: er behauptet heute eine Deckung, die es nicht gibt.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
