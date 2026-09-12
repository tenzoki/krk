Die Spalte „Wer schreibt" der HowTo-Tabelle nennt für drei Dateien das Gegenteil dessen, was KRK tut

---

`HowTo.md:25-33` führt je Ablagedatei eine Spalte „Wer schreibt". Für die drei von Hand
gepflegten Dateien steht dort genau die Umkehrung der Wirklichkeit: die eine, die KRK **nicht**
anlegt, ist KRK zugeschrieben, und die zwei, die KRK anlegt, sind allein dem Nutzer
zugeschrieben.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** vom Coder beim Beheben von
`260911-1838_*_die-dateitabelle-der-howto-fuehrt-sieben-ablagedateien-seit-der-runde-24-sind-es-acht.md`
gesehen und als Auslegungsfrage liegen gelassen; vom Nutzer am 260912 als Defekt eingeordnet.
**Betroffen:** `HowTo.md:28`, `HowTo.md:31`, `HowTo.md:32`

## Der Befund

| Zeile | Spalte sagt | Der Baum sagt |
|---|---|---|
| `keymap.toml` (28) | KRK und der Nutzer | KRK legt sie **nie** an; geschrieben wird sie an einer Stelle, beim Verlassen der F1-Ansicht mit einer Änderung |
| `settings.toml` (31) | nur der Nutzer | KRK legt sie beim ersten Start an |
| `readers.toml` (32) | nur der Nutzer | KRK legt sie beim ersten Start an |

Belegt mit `grep -rn "anlegen_falls_fehlt" crates/krk-core/src/ablage/` — vier Zeilen in
`einstellungen.rs` und `leseprofile.rs`, keine für die Belegung — und
`grep -rn "sichern(Datei::Belegung" crates/`, eine Zeile, `tasten/belegung.rs:1663`.

## Warum das nicht bloß eine Auslegungsfrage ist

Läse man die Spalte als „wer den Inhalt pflegt", wären die zwei unteren Zeilen richtig und die
obere falsch: den Inhalt von `keymap.toml` pflegt der Nutzer, KRK schreibt nur, was die
F1-Ansicht ihm vorlegt. Läse man sie als „wer die Datei schreibt", wäre die obere Zeile
vertretbar und die zwei unteren falsch. **Keine der beiden Lesarten macht alle drei Zeilen
wahr**, und deshalb ist die Spalte nicht doppeldeutig, sondern an mindestens einer Zeile
falsch, welche Lesart man auch wählt.

Zwei Absätze unter der Tabelle steht seit `ff48cde` ausgeschrieben, dass KRK `settings.toml`
und `readers.toml` beim ersten Start anlegt und `keymap.toml` nicht. Die Tabelle widerspricht
damit dem Fließtext derselben Datei, und die `HowTo.md` reist im Releasepaket mit: der Nutzer
liest beides.

## Abnahme

Die Spaltenüberschrift und ihre acht Einträge sagen dasselbe, gleich welche der zwei Lesarten
gewählt wird. Wer den Defekt nimmt, entscheidet die Lesart zuerst — „wer die Datei schreibt"
oder „wer den Inhalt pflegt" — und zieht dann alle acht Zeilen darauf nach, nicht nur die
drei genannten; die fünf übrigen sind heute unter beiden Lesarten gleich und bleiben es nur,
solange niemand sie prüft.

Geprüft mit den zwei Suchkommandos oben und gegen den Absatz „Wie die drei entstehen" in
`HowTo.md`.
