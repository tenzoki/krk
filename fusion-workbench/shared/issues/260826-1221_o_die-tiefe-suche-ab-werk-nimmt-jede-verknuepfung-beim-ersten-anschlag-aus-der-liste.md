Die tiefe Suche ab Werk nimmt jede Verknuepfung beim ersten Anschlag aus der Liste

---

Seit `20c9833` steht `tief` in `Ordnermodell::neu` ab Werk auf `true`
(`crates/krk-core/src/verzeichnis/modell.rs:374`). Damit greift der Zweig
`UnterVorbehalt(Auftragsart::Unterbaum)` fuer jede Verknuepfung, deren eigener Name den
Filtertext nicht traegt — und der Durchlauf beantwortet ihn ausnahmslos negativ, weil er in
eine Verknuepfung nicht hinabsteigt. Eine Verknuepfung auf einen Ordner voller Treffer
verschwindet damit beim ersten getippten Zeichen aus der Liste und kommt nicht zurueck,
solange der Filtertext steht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Affected:** `crates/krk-core/src/verzeichnis/modell.rs:374`, `:746-772`;
`crates/krk-core/src/verzeichnis/durchlauf.rs:490-501`
**Tree state:** `004ff72`
**Domain:** code

## Der Weg, Zeile fuer Zeile

```rust
// modell.rs:757-772 — der Pruefschritt
if !(eintrag.ist_ordner() || eintrag.ist_verknuepfung()) { ... }   // Verknuepfung faellt hier NICHT heraus
if !self.tief { return Zeilengrund::Steht; }                        // ab Werk uebersprungen
Zeilengrund::UnterVorbehalt(Auftragsart::Unterbaum)
```

```rust
// durchlauf.rs:499-501 — die Antwort
if ist_verknuepfung(wurzel) {
    return Some(false);
}
```

`sichtbar` (`modell.rs:811-820`) macht aus `Befund::KeinTreffer` ein `false`. Der Befund ist
endgueltig: er faellt erst mit dem Filtertext oder mit einem Schalterwechsel
(`modell.rs:1131`), nicht mit einem weiteren Durchlauf.

## Was daran neu ist und was nicht

**Das Verhalten ist alt, gewollt und geprueft.** C1.6/C2.13, Probe
`eine_verknuepfung_zaehlt_fuer_die_sichtbarkeit_als_ordner`
(`crates/krk-core/tests/verzeichnis.rs:913-937`).

**Neu ist, welche Haelfte jener Zusage der Nutzer ab Werk bekommt.** Die Probe schreibt beide
Haelften aus:

- `"flach bleibt die Verknuepfung stehen wie jeder Ordner"` (`tests/verzeichnis.rs:925`)
- `"tief steht allein die Verknuepfung, deren eigener Name traegt"` (`:935`)

Bis zum 260826 war die erste Haelfte die Vorgabe und die zweite ein Klick. Seither ist es
umgekehrt. Die Probe merkt davon nichts, weil ihr Aufbauhelfer `gefiltert`
(`tests/verzeichnis.rs:708-713`) `modell.tief_setzen(false)` ausdruecklich setzt; dasselbe tut
`handmodell` (`:1220-1229`). Beide Helfer stellen also den Zustand **vor** der Vorgabenaenderung
her, und keine Probe der Datei misst die Vorgabe gegen eine Verknuepfung.

## Warum ein eigener Datensatz und nicht ein Nachtrag am Schwellendatensatz

`shared/decisions/260826-0859_*_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md`
behandelt genau diese Sorte Folge und benennt sie als solche: „Damit hat sich eine zweite
Groesse mitverschoben, die niemand angefordert hat." Er behandelt **eine** davon, die Schwelle
des Inhaltsfilters. Dies hier ist eine dritte, sie steht in keiner seiner drei Moeglichkeiten,
und keine der drei aendert etwas an ihr: die Verknuepfung faellt weg, gleich wo die
Inhaltsschwelle liegt und ob „Content" ueberhaupt steht.

CLAUDE.md nennt sie ebenfalls nicht; der Absatz zur Vorgabe (`88f18ed`) spricht allein vom
Durchlauf ueber den Unterbaum.

## Der Nebeneffekt daneben, benannt und nicht mitgemeldet

Ab Werk gehen mit demselben Anschlag auch **alle** gewoehnlichen Ordner unter Vorbehalt und
sind unsichtbar, bis der Durchlauf sie einzeln beantwortet. Das ist voruebergehend und die
gewollte Wirkung der tiefen Suche; es steht hier nur, damit ein spaeterer Leser die zwei Faelle
nicht verwechselt. Die Verknuepfung ist der Fall, der **nicht** zurueckkommt.

## Richtung

Zu entscheiden ist, ob die Vorgabe „Deep = ein" diese Folge tragen soll; das ist eine
Nutzerfrage und keine Codeaenderung, die sich aus dem Baum ableiten liesse. Drei denkbare
Antworten, alle billig:

1. **So lassen und aufschreiben.** Die Folge steht dann in CLAUDE.md und im
   Vorgabenkommentar bei `Ordnermodell::neu`, wo `260826-0859` schon steht.
2. **Die Verknuepfung faellt aus dem Vorbehalt.** Der Zweig `!self.tief` in
   `zeilengrund_von:766` bekaeme ein `|| eintrag.ist_verknuepfung()` — eine Verknuepfung
   staende dann immer, wie ein Ordner bei flacher Suche. Kostet eine zweite Regel im
   Pruefschritt, den der Modulkopf ausdruecklich als **eine** Regel fuehrt.
3. **Der Durchlauf steigt in Verknuepfungen ab.** Faellt aus: C3.7 und C3.9 verbieten es, und
   ohne mitgefuehrte Besuchtliste entstuenden Ringe (`durchlauf.rs:141-143`).

Vorgelegt gehoert die Frage mit ihren Folgen, nicht als Pro-und-Contra-Liste. Wer sie
beantwortet, sollte wissen, dass Moeglichkeit 2 den einen Pruefschritt um einen Zweig
verbreitert, den bisher keine Zusage verlangt.
