Eine bookmarks.toml, die serde toleriert aber nicht versteht, wird still als leer gelesen und beim nächsten Befehl überschrieben

---

Die Zusage der Runde 6 — eine Ablagedatei, die KRK nicht mehr versteht, wird unter
`atomar::beiseitepfad` gesichert, bevor der Auslieferungszustand einspringt — deckt für
`bookmarks.toml` nur den syntaktischen Fehlschlag ab. Zwei Gestalten desselben Verlusts
kommen an ihr vorbei, und in beiden steht danach eine leere Leiste, ohne dass eine
Sicherung entsteht.

---

**Gemessen am Baumstand `01d2365`**, mit einem Programm, das `krk_core::ablage::Lesezeichenliste`
über `toml::from_str` einliest, also über denselben Weg wie `Zugang::laden`
(`crates/krk-core/src/ablage/mod.rs:487-535`):

| Eingabe | Ergebnis | Meldung | beiseitegelegt |
|---|---|---|---|
| `[[eintraege]]` mit `name` und `ordner` (der heutige Bestand, 5 Einträge) | 5 Einträge | – | – |
| oberster Schlüssel heißt anders, etwa `[[lesezeichen]]` | **0 Einträge, `Ok`** | **keine** | **nein** |
| Datei ist leer (0 Bytes) | **0 Einträge, `Ok`** | **keine** | **nein** |
| Eintrag mit unbekanntem Zusatzfeld `farbe` | 1 Eintrag, unverändert | – | – |
| Eintrag ohne `ordner` / mit einer dritten Zielsorte | `Err` — „data did not match any variant of untagged enum Ziel" | ja | ja |

**Gestalt 1 — serde toleriert die Form.** `Lesezeichenliste` trägt `#[serde(default)]`
(`crates/krk-core/src/ablage/lesezeichen.rs:329-335`), und serde übergeht unbekannte Felder.
Eine Datei, deren oberster Schlüssel nicht `eintraege` heißt, ist damit **gültiges TOML mit
null Einträgen** und kein Fehler. `Zugang::laden` sieht kein `Err`, erzeugt keine `Ersetzung`
und ruft `beiseite_legen` nicht — das steht ausdrücklich allein im Zweig `Grund::Beschaedigt`
(`mod.rs:519-531`).

**Gestalt 2 — `Grund::NichtLesbar` legt nichts beiseite.** Liegt die Datei da und lässt sich
nicht lesen, liefert `laden` den Auslieferungszustand mit `Beiseite::Nicht` (`mod.rs:504-514`).
Der Nutzer bekommt zwar einen Satz, aber keine Sicherung.

**In beiden Gestalten ist der nächste gewöhnliche Schreibvorgang der Schaden**, und das ist
genau der Verlauf, den der Modulkopf von `ablage/mod.rs` unter „Eine beschädigte Datei wird
zur Seite gelegt" beschreibt: `lesezeichen_aendern` liest unter der Schreibsperre frisch,
bekommt die leere Liste, wendet **eine** Änderung an und schreibt das Ergebnis
(`crates/krk-ui/src/appkit/anwendung.rs:1732-1742`). Danach steht in `bookmarks.toml` genau
ein Eintrag, und der alte Bestand ist ohne Sicherung fort.

**Was daran heute nicht eintritt, und was das wert ist.** Der oberste Schlüssel heißt seit dem
260803 `eintraege` und ist nie umbenannt worden — `git log -S 'eintraege' --follow` auf
`lesezeichen.rs` nennt vier Commits, den letzten am 260813. Die Lücke ist deshalb heute keine
Fehlfunktion, sondern eine unbewachte Zusage: die vierte Zeile der Tabelle zeigt, dass ein
**neues Feld** getragen wird, die zweite, dass ein **umbenannter oder umgehängter oberster
Schlüssel** still zu null Einträgen führt. Wer die Ablage künftig umformt, hat für die eine
Richtung eine Sicherung und für die andere keine, und nichts im Baum sagt ihm, welche er
gerade nimmt.

**Schwere:** mittel. Kein Fehlverhalten am heutigen Bestand; die Zusage der Runde 6 ist für
diese Datei aber schmaler als ihr Wortlaut, und die Lücke fällt erst auf, wenn der Bestand
schon fort ist.

**Gefunden:** analyst, forensische Untersuchung „Lesezeichen nach Installation weg" am 260820-2235

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:487-535`,
`crates/krk-core/src/ablage/lesezeichen.rs:76-90`, `:329-335`,
`crates/krk-ui/src/appkit/anwendung.rs:1732-1742`

**Domain:** code

**Verwandt:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1204_*_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`
— dieselbe Lücke an `keymap.toml`, eine Ebene höher entdeckt.

## Vorschlag

Kein zweiter Mechanismus daneben. Die Frage, die der Ladeweg heute stellt, ist „ist das
gültiges TOML"; die Frage, die er stellen müsste, ist „hat die gelesene Datei den Bestand
hergegeben, den sie trägt". Das ist an einer Stelle entscheidbar: **eine Datei, die dasteht
und nicht leer ist, aber null Einträge ergibt, ist kein erster Start** — sie geht denselben
Weg wie eine beschädigte, also `Grund::Beschaedigt` mit `beiseite_legen`. Ob dieselbe Regel
für `session.toml` und `keymap.toml` taugt, ist zu entscheiden und nicht abzuleiten; für
`settings.toml` gilt sie nicht, weil die Datei ohnehin nie über `sichern` läuft.

Für Gestalt 2 ist die Antwort schmaler und liegt in derselben Fallunterscheidung: aus einer
Datei, die sich nicht lesen ließ, gibt es nichts zu sichern — dann darf sie aber auch nicht
überschrieben werden. Ein `sichern`, das auf einer `Ersetzung` aufsetzt, schreibt heute
trotzdem.
