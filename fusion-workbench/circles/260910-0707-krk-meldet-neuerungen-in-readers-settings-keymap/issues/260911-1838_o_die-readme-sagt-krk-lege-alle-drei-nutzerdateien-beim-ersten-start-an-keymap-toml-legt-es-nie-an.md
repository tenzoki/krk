Die README sagt, KRK lege alle drei Nutzerdateien beim ersten Start an; `keymap.toml` legt es nie an

---

`README.md:57-60` eröffnet den neuen Abschnitt mit „KRK legt jede von ihnen beim ersten
Start an und schreibt sie danach nie wieder" und zählt darunter `keymap.toml`,
`settings.toml` und `readers.toml`. Für `keymap.toml` sind **beide** Hälften falsch: KRK
legt sie nie an, und es schreibt sie sehr wohl ein zweites Mal, nämlich beim Verlassen der
Belegungsansicht. Der ganze Fall `Befund::Fehlt` der Runde beruht auf der ersten Hälfte.

---

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** Durchsicht der Runde 24, Bereich `feecd6c..d9535c4`, Schritt 10
**Betroffen:** `README.md:57-60`, und mittelbar `README.md:94-99` (der Handgriff nennt als
Schritt 2 das Beiseitelegen einer Datei, die es bei `keymap.toml` oft gar nicht gibt)

## Die Gegenbelege im Baum

- **Nicht angelegt.** `crates/krk-core/src/ablage/mod.rs`, Abschnitt „Zwei der sechs
  TOML-Dateien entstehen einmal und werden nie wieder geschrieben", nennt `settings.toml`
  und `readers.toml` und nicht `keymap.toml`. `anlegen_falls_fehlt` steht in
  `ablage/einstellungen.rs` und in `ablage/leseprofile.rs:128-134`; für die Belegung gibt es
  keine solche Stelle.
- **Doch wieder geschrieben.** `crates/krk-core/src/tasten/belegung.rs:1663` ruft
  `zugang.sichern(Datei::Belegung, …)`; `HowTo.md:180-183` beschreibt genau diesen Weg
  („geschrieben wird erst beim Verlassen des Blattes, und dann wird `keymap.toml` mit dem
  Auslieferungsstand überschrieben").
- **Die Runde selbst rechnet mit dem Gegenteil.**
  `crates/krk-core/src/ablage/neuerungen.rs:197-201` an `Befund::Fehlt`: „Der gewöhnliche
  Fall bei `keymap.toml`: KRK legt sie nie an, und wer seine Belegung nie geändert hat, hat
  sie nicht." Der Plan dieser Runde trägt denselben Satz in seiner `Decidability`-Zeile.

## Was daran zählt

Der Satz steht als erster im Abschnitt und trägt seine Voraussetzung. Wer ihn glaubt, sucht
`~/Library/Application Support/KRK/keymap.toml` auf einer frischen Installation und findet
nichts, und er kann sich nicht erklären, warum das Blatt für dieselbe Datei „Diese Datei
liegt nicht in Ihrer Ablage" sagt. `HowTo.md:46-48` macht die Aussage nicht und ist richtig;
die zwei Stellen widersprechen einander.

## Abnahme

`README.md` sagt für jede der drei Dateien, was für sie gilt: `settings.toml` und
`readers.toml` entstehen beim ersten Start und werden danach nicht wieder geschrieben,
`keymap.toml` entsteht erst, wenn der Nutzer seine Belegung ändert, und wird dann von der
Belegungsansicht geschrieben. Der Handgriff darunter sagt, was zu tun ist, wenn die Datei
gar nicht dasteht — nämlich nichts.

Geprüft am Baum: `grep -rn "anlegen_falls_fehlt" crates/krk-core/src/ablage/` nennt die zwei
Dateien, die angelegt werden, und `grep -rn "sichern(Datei::Belegung" crates/` die eine
Stelle, die `keymap.toml` schreibt.
