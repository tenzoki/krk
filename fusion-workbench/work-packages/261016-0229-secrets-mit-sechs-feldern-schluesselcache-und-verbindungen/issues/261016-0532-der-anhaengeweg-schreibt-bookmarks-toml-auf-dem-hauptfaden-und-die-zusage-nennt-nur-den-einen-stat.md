Der Anhängeweg schreibt `bookmarks.toml` auf dem Hauptfaden, und Plan, Probe und Doku-Kommentar nennen als Systemaufruf nur den einen `stat(2)`
---
`anhaengen_einziehen` legt das Lesezeichen über `lesezeichen_aendern` an, also auf dem Hauptfaden unter der Schreibsperre der Ablage: Sperre, Lesen und atomares Schreiben von `bookmarks.toml`. Entscheidung 20 und die Bedingung unter „Where this work stops“ sagen „kein Systemaufruf außer dem einen `stat(2)`“; die Probe nennt als einzige Ausnahme `notizordner_bereitstellen`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Entscheidungen 13 und 20, `## Where this work stops`, dritter Spiegelstrich von unten)

## Defekt

- `crates/krk-ui/src/appkit/anwendung.rs:9229`: `selbst.lesezeichen_aendern(seite, &aenderung)` im Zweig `Anhaengausgang::Angehaengt`.
- `anwendung.rs:2759-2770`: `lesezeichen_aendern` ruft `unter_der_sperre`, `zugang.laden::<Lesezeichenliste>` und `zugang.sichern(Datei::Lesezeichen, …)`, alles auf dem rufenden, hier dem Haupt-Faden.
- Die Probe `der_verbindungsweg_stellt_auf_dem_hauptfaden_nur_den_einen_stat` (`anwendung.rs`, Prüfmodul) führt `anhaengen_einziehen` unter den Rümpfen „ohne Systemaufruf außer dem `stat(2)`“; sie sieht den Aufruf nicht, weil er eine Ebene tiefer liegt, und ihr Doc-Kommentar nennt als **benannte** Ausnahme allein `notizordner_bereitstellen`.
- Der Plan sagt in Entscheidung 20 „Der Hauptfaden stellt auf diesen Wegen keinen Systemaufruf außer dem einen `stat(2)` aus Entscheidung 14“, während Entscheidung 13 dasselbe `lesezeichen_aendern` verlangt. Der Plan widerspricht sich selbst.

Der Schreibweg ist derselbe wie bei jedem anderen Lesezeichen (`cmd+d`), läuft nach einer Nutzerhandlung und liegt auf keinem der zehn gemessenen Wege aus C8. **inference:** an den Zusagen L1 bis L10 ändert er nichts. **Aber** der Plan macht den Verzicht auf den Abnahmelauf von dieser Bedingung abhängig („Hält eine davon nicht, ist die Arbeit nicht fertig, und die Frage nach dem Abnahmelauf geht an den Nutzer“). Ob sie als gehalten gilt, entscheidet deshalb nicht der Reviewer.

## Richtung der Behebung

Die Bedingung richtig fassen statt den Code umzubauen: Entscheidung 20, der Doc-Kommentar von `verbindung_herstellen`/der Probe und `CLAUDE.md` nennen das Anlegen des Lesezeichens über `lesezeichen_aendern` als zweite benannte Ausnahme, mit dem Grund (derselbe Weg wie jedes Lesezeichen, nach einer Nutzerhandlung). Die Frage nach dem Abnahmelauf legt der Orchestrator dem Nutzer vor, wie die Klausel des Plans es verlangt.

## Akzeptanz

- Der Doc-Kommentar der Probe nennt beide Ausnahmen mit Namen; die Probe prüft, dass `lesezeichen_aendern` in `anhaengen_einziehen` genau einmal steht und in keinem anderen ihrer Rümpfe.
- Die Antwort des Nutzers zur Klausel steht im Paket.

---
Resolved: Kein zweiter Schreibweg. Die Probe `der_verbindungsweg_stellt_auf_dem_hauptfaden_nur_den_einen_stat` (`crates/krk-ui/src/appkit/anwendung.rs`) nennt in ihrem Doc-Kommentar beide Ausnahmen mit Namen und prueft, dass `.notizordner_bereitstellen(` allein in `verbindungsschluessel_holen` und `.lesezeichen_aendern(` allein in `anhaengen_einziehen` je genau einmal steht. Der Doc-Kommentar von `anhaengen_einziehen` und `CLAUDE.md` (Absatz zu den drei Lesezeichensorten) nennen den Hauptfaden-Schreibvorgang als zweite benannte Ausnahme. Der Plan ist nicht angefasst; die Frage nach dem Abnahmelauf aus „Where this work stops“ steht weiter beim Orchestrator fuer den Nutzer.
