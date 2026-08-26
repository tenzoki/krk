# Keine Probe im Baum hält die vier Zahlen der flight-Profile

---
**Domain:** data
**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml` (Kommentar über `flight-Werkbank: die Wurzel`, „ELF ZEILEN, FÜNF LESELÄUFE, DREI ÖFFNUNGEN"; Kommentar über `Projektwurzel mit flight-Werkbank`, „ELF ZEILEN, SECHS LESELÄUFE, DREI ÖFFNUNGEN"); `crates/krk-core/tests/leseprofil.rs` (`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`, `ausgelieferte`); `shared/issues/260825-2233_c_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md` (derselbe Fall, für fusion, mit `96e32cb` behoben)

---

## Was ist

`180fc53` schreibt vier neue Zahlen in die Profildatei: elf Zeilen und fünf Leseläufe
für `flight-Werkbank: die Wurzel`, elf Zeilen und sechs Leseläufe für
`Projektwurzel mit flight-Werkbank`, drei Öffnungen für beide. Alle vier stimmen —
unabhängig nachgemessen an einem Prüfordner in flight-Gestalt und an der wirklichen
Werkbank unter `/Users/k1/Projects/productive/example/`, beide Male `(5, 3)`
beziehungsweise `(6, 3)`.

Gehalten wird davon nichts. Die Zeichenfolge `flight` kommt unter `crates/` und in
`xtask/` an keiner Stelle vor; der Ontocoder schreibt das in seinem Verlauf selbst aus
(`shared/history/260826-0810-ontocoder-die-vier-flight-speicher-tragen-jetzt-ihre-datumszeile.md`,
Abschnitt „Abnahme").

Zwei Proben berühren die Datei und halten etwas anderes:

- `ablage::leseprofile::tests::die_eingebettete_fassung_besteht_ihre_eigene_pruefung`
  und die Hilfe `ausgelieferte` halten, dass die Datei **lädt** und zwölf Profile
  liefert. Ein verschriebener Schlüssel oder ein dritter Wert für `zeigt` würde damit
  rot. Das ist die Grenze dessen, was sie prüfen.
- `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` misst drei
  fusion-Profile und seit `96e32cb` das fusion-Projektwurzelprofil. Kein flight-Profil
  steht darin.

## Warum das zählt

Der Fall ist derselbe, den dieses Projekt vor einem Tag für fusion abgelegt und behoben
hat. `260825-2233_c_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md`
hielt fest, dass die Datei ihre Leselaufregel an zwei Beispielzahlen vorführt und nur die
erste eine Probe hatte; `96e32cb` hat die zweite nachgezogen. Am selben Tag stellt
`180fc53` vier weitere Zahlen derselben Art daneben, ohne dieselbe Vorkehrung.

Die Zahlen sind heute richtig. Was fehlt, ist der Halt gegen die nächste Änderung: wer
den vier flight-Speichern einen fünften hinzufügt oder einer Zeile eine Ortsangabe gibt,
verschiebt die Läufe, und der Kommentar bliebe stehen, wie er ist.

Der Kommentar ist bei dieser Datei die einzige Auskunft, die der Nutzer je bekommt: sie
wird beim ersten Start wörtlich kopiert und danach nie wieder überschrieben
(`ablage::leseprofile::anlegen_falls_fehlt`). Eine falsche Zahl darin wandert mit.

## Was zu tun wäre

Die Probe `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` um zwei
Fälle erweitern, oder eine Schwesterprobe daneben stellen, gegen einen Prüfordner in
flight-Gestalt: `.flight-setup` mit seinen drei Feldern, je ein Datensatz in `decisions`,
`history` und `memos`, ein Lauf als **Ordner** in `archive`, dazu ein `stilwerk`. Zu
halten sind je Profil beide Hälften, wie es `96e32cb` für das fusion-Projektwurzelprofil
vormacht: die Zahl selbst (`(5, 3)` und `(6, 3)`) und ihre Herleitung
(`leselaeufe == orte.len()` an der Wurzel, `leselaeufe == orte.len() + 1` an der
Projektwurzel).

Der Ordner in `archive` ist dabei nicht Beiwerk. Er ist der einzige Eintrag, an dem die
Zeile „Ablagen, zuletzt" ihre Form belegt: gemessen liefert sie mit `zeigt = "datum"`
ein Datum und mit `zeigt = "titel"` den Platzhalter `--`, und genau das behauptet der
Kommentar über dem Profil.

**Zuständig:** `coder`, denn der Eingriff ist eine Probe und keine Zeile der Profildatei.

**Schwere:** niedrig. Keine Zahl steht heute falsch da; es fehlt die Vorkehrung gegen
morgen.
