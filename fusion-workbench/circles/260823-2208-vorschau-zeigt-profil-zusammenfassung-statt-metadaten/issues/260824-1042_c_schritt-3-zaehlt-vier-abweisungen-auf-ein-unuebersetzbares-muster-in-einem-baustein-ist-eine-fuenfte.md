Schritt 3 zählt vier Abweisungen auf; ein unübersetzbares Muster **in** einem Baustein ist eine fünfte

---

Der Planschritt 3 schreibt aus, was `leseprofil::datei::pruefen` abweist, und nennt vier Fälle:
das unübersetzbare Erkennungsmuster eines Profils (C2.7), das Profil ohne Pfadmuster und ohne
Kennzeichen, das Feldmuster ohne genau eine Fanggruppe (C3.10) und die Ortsangabe, die schon am
Text herausführt (C3.13). Ein fünfter Fall steht in keiner der beiden Fassungen und tritt am
Baum zwangsläufig auf: ein Muster **innerhalb** eines Bausteins — `muster` in `zaehlung`,
`juengste` und `vorhandensein`, `datei` in `feld` —, das sich nicht übersetzen lässt. Die
Umsetzung hat ihn als Zeilenabweisung eingeordnet; der Plan sagt dazu nichts.

---

**Gemessen am Baumstand nach Schritt 3**, `crates/krk-core/src/leseprofil/datei.rs`.

## Warum der Fall nicht zu umgehen war

`Regex::new` liefert für jedes der fünf Muster ein `Result`. Ohne eine Regel bleiben genau drei
Möglichkeiten, und zwei davon sind im Projekt ausgeschlossen: ein `unwrap` (ein Muster aus einer
von Hand gepflegten Datei brächte KRK beim Start zum Absturz) oder ein stilles Fallenlassen
(`CLAUDE.md`, „No silent failures"). Übrig bleibt die Abweisung mit Meldung, und die Frage ist
allein ihre Reichweite: das ganze Profil oder die eine Zeile.

## Wie die Umsetzung entschieden hat, und woraus

Als **Zeilenabweisung**, also dieselbe Reichweite wie C3.10 und die textliche Hälfte von C3.13.
Der tragende Grund steht im Modulkopf von `datei.rs` unter „Was abgewiesen wird, und wie weit"
und ist der Unterschied, den der Plan selbst zwischen seinen vier Fällen zieht: ein Profil, das
seinen Ort nicht erkennt, ist nicht halb brauchbar, sondern gar nicht — eine Zeile, deren
Baustein fehlt, steht dagegen mit ihrer Beschriftung und dem Platzhalter da, und die übrigen
Zeilen des Profils stimmen weiter (C3.12). Ein unübersetzbares `muster` in einer Zählung sagt
nichts über die Erkennung des Profils aus; es kostet eine Zeile und nicht sechs.

Die Umsetzung nennt die vier Fälle des Plans deshalb als **Beispiele einer Regel** und nicht als
Liste: Muster im Erkennungsteil kosten das Profil, Muster und Ortsangaben in einer Zeile kosten
die Zeile.

## Was zu tun ist

Die Einordnung ist nachzulesen und entweder zu bestätigen oder umzustoßen. Sie steht heute nur
im Quelltext, und der Plan zählt an ihrer Stelle vier Fälle auf, die ein späterer Leser für
vollständig halten wird — genau die Lesart, die diesen Datensatz nötig gemacht hat.

Ein Abnahmekriterium ist nicht betroffen: C2.7 spricht ausdrücklich vom Pfadmuster, C3.10 vom
Feldmuster, und keines der sechsundfünfzig sagt etwas über ein `muster` in einer Zählung.

**Schwere:** niedrig. Kein Fehlverhalten, und die gewählte Reichweite folgt der Regel, die der
Plan für seine vier Fälle selbst zieht. Der Befund ist die Lücke in der Aufzählung.

**Gefunden:** coder, beim Bau von Schritt 3 am 260824-1042.

**Betroffen:** `crates/krk-core/src/leseprofil/datei.rs`,
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Schritt 3

**Domain:** code

---
Resolved: Der Planschritt 3 nennt die fuenfte Abweisung seit dem 260824-1224 und **bestaetigt die Einordnung der Umsetzung**: ein unuebersetzbares Muster innerhalb eines Bausteins kostet die Zeile und nicht das Profil. Der Grund steht dabei, in derselben Form, die der Datensatz vorschlaegt: Muster im Erkennungsteil kosten das Profil, Muster und Ortsangaben in einer Zeile kosten die Zeile, und die fuenf Faelle sind Beispiele dieser Regel und keine abgeschlossene Liste. Die Probenzeile desselben Schrittes ist von „die vier Abweisungen" auf die Abweisungen umgestellt. Kein Abnahmekriterium ist betroffen und kein Code angefasst.
