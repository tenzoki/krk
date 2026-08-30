Fünf Prosastellen behaupten, eine Feldbreite halte den Bau an, wenn eine Aufzählung wächst; sie tut es nicht

---
Fünf Doc-Kommentare schreiben einer Feldbreite der Form `[T; N]` die Sicherung zu, dass ein zusätzlicher Wert der zugehörigen Aufzählung den Bau anhält. Die Sprache leistet das nicht: die Länge zwingt zu N Einträgen und sagt nichts darüber, welche N. Eine sechste `Bereich`-Variante, eine sechste `Fokus`-Variante und eine neunte `Wirkungsbereich`-Variante übersetzen sämtlich grün, während die Liste daneben unverändert fünf beziehungsweise acht Werte führt.

Die Stellen, Stand `d1fbaac`:

1. `crates/krk-ui/src/kommandos/fokus.rs:142-145` — „**Die Feldbreite steht in der Typangabe.** Ein sechster Wert haelt damit den Bau der Proben an". Der Halbsatz danach („die Aufzaehlung selbst erzwingt der Uebersetzer nicht") sagt bereits das Richtige und widerspricht dem Satz davor.
2. `crates/krk-ui/src/kommandos/fokus.rs:396-398` — „ein sechster Fokuswert oder ein neunter Wirkungsbereich \[faellt hier auf\]: beide Feldbreiten stehen in der Typangabe, und eine vergessene Zeile haelt den Bau an". Betrifft `TAFEL: [(Wirkungsbereich, [bool; 5]); 8]` (`:404`).
3. `crates/krk-ui/src/kommandos/fokus.rs:702-703` — „Die Feldbreite `[Fokus; 5]` haelt den Bau an, wenn ein sechster Wert dazukommt". Die Probe darunter zählt die fünf Werte zusätzlich von Hand auf und sähe einen sechsten ebenfalls nicht.
4. `crates/krk-ui/src/appkit/aufteilung.rs:242-243` — „ein sechster Bereich haelt hier den Bau an, wie bei `Bereich::ALLE` selbst". Der zweite Halbsatz ist der Kern des Irrtums: `Bereich::ALLE` (`fenstermodell.rs:122`) hält gar nichts.
5. `crates/krk-ui/src/appkit/bereichsleiste.rs:418-419` — „ein sechster Bereich haelt hier den Bau an, dasselbe Muster wie `Aufteilung::rahmen`".

Daneben, mit der schwächeren Formulierung und demselben Loch: `crates/krk-ui/src/kommandos/zulaessigkeit.rs:660-661` sagt zu `OHNE_SPERRE: [[bool; 5]; 8]` (`:670`), „die Tafel zeigt, dass keine Zeile und keine Spalte fehlt". Sie zeigt es für die heutigen acht mal fünf und für keine andere Zahl.

**Was wirklich hält:** die vier Feldbreiten in `Aufteilung::rahmen`, `Bereichsleiste::bereichsschalter`, `Aufteilung::gemessene_breiten` und `Fenstermodell::breiten_uebernehmen` halten den Bau an, **sobald `Bereich::ALLE` wächst**, denn sie hängen über `Bereich::ALLE.map(…)` daran. Sie sichern also den zweiten Schritt und nicht den ersten. Der erste, der Eintrag in `ALLE`, ist ungesichert.

**Beweis, eigenständig übersetzt** (`rustc --edition 2024`, grün, Ausgabe „ALLE fuehrt 5 von 6 Werten, und der Bau ist gruen."):

```rust
enum Bereich { Lesezeichen, Links, Rechts, Vorschau, Editor, Git }
impl Bereich {
    const ALLE: [Bereich; 5] = [
        Bereich::Lesezeichen, Bereich::Links, Bereich::Rechts,
        Bereich::Vorschau, Bereich::Editor,
    ];
}
```

**Abnahmetest:** keine der sechs Stellen behauptet mehr eine Sicherung durch die Feldbreite; jede sagt stattdessen, was tatsächlich hält (der Übersetzer erst nach dem Eintrag in die Liste) und was nicht. Eine Probe ist damit nicht gefordert: welche Bauform die Vollständigkeit der elf `ALLE`-Listen künftig hält, ist die offene Nutzerfrage `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`, und dieser Datensatz greift ihr nicht vor. Er verlangt allein, dass die Prosa bis dahin nicht das Gegenteil sagt.

---
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
Gefunden bei der Machbarkeitsanalyse zu `gix`, Frage 7: `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md`. Der Befund bindet den Plan der Git-Runde, weil ein sechster `Bereich` ohne Eintrag in `Bereich::ALLE` übersetzt, jede Probe besteht und keinen `NSBox`, keinen Schalter, keinen Breitenanteil und keinen Ersthelferbereich bekommt.
Verwandt: `260826-1420_*_zwei-probenkoepfe-in-statuszeile-rs-zaehlen-fuenf-raenge-und-rang-alle-traegt-sechs.md` (derselbe Befundtyp an anderer Stelle).
