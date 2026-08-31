# Bekommt der Git-Bereich einen eigenen `Funktionsbereich` und damit ein zehntes Obermenü?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md` (E10, C1.2, C2.2, C5.6); `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (Entscheidung 8, Schritt 8); `crates/krk-ui/src/belegungsmodell.rs:101-139` (die Aufzählung), `:226-410` (die Zuordnung); `crates/krk-ui/src/belegungsausgabe.rs:641` (die Probe über die neun Bereiche)

---

## Question

Die Runde 23 legt zwei Befehle an, die den Git-Bereich angehen: `git_bereich_umschalten` auf
`opt+cmd+r` und `fokus_git` auf `shift+cmd+b`. `belegungsmodell::bereich` muss beide einem
`Funktionsbereich` zuordnen, und dieser Wert entscheidet zweierlei, was der Nutzer sieht: unter
welcher Überschrift die Zeilen in `docs/tastenbelegung.md` und in der Belegungsansicht stehen, und
in welchem Obermenü der Menüleiste ihre Einträge liegen.

Die Aufzählung trägt heute neun Werte. Ein zehnter heißt ein zehntes Obermenü mit zunächst zwei
Einträgen. Die Frage ist vor dem Bau zu entscheiden, weil sie eine sichtbare Fläche der Anwendung
verändert und weil die Runde 24 (Hinzufügen, Committen, Verwerfen, Versions-Schieberegler) ihre
Befehle in denselben Wert einordnen wird; sie ist damit keine Frage dieser Runde allein.

Der dritte neue Befehl, `spalte_marke_umschalten`, ist nicht betroffen: C5.6 ordnet ihn
ausdrücklich dem `Dateilisting` zu, wie die drei vorhandenen Spaltenschalter.

## Options

1. **`Funktionsbereich::Git` als zehnter Wert, unmittelbar hinter `Editor`.**
   - Pros: Es ist die Regel, die `belegungsmodell.rs` dreimal ausgeschrieben trägt. Zu
     `Funktionsbereich::Vorschau` steht dort: „Wer wissen will, wie er in die Vorschau kommt,
     findet unter ‚Vorschau‘ alle Befehle, die sie angehen"; zu `Editor` derselbe Satz, und
     ausdrücklich mit dem Zusatz, dass `EditorUmschalten` deshalb **nicht** unter „Fenster" steht,
     wo sein Gegenstück für die Dateifenster steht. Jeder Bereich mit eigenem Fokuswert trägt
     seinen Umschalter und seinen Fokusbefehl in seinem eigenen `Funktionsbereich`, ohne Ausnahme.
     Die Runde 24 findet ihren Platz vor. Die Reihenfolge der Obermenüs bleibt die der Fensterzeile
     von links nach rechts, wenn `Git` hinter `Editor` steht.
   - Cons: Ein zehntes Obermenü mit zwei Einträgen ist für einen Menüleisten-Nutzer viel Weg für
     wenig Ziel. Die Probe `ab Werk ist jeder Bereich besetzt, also stehen alle neun in ihrer
     Reihenfolge` (`belegungsausgabe.rs:641`) zieht auf zehn nach, und `Funktionsbereich::name`
     bekommt eine Zeile. Der Wert bleibt bis zur Runde 24 dünn besetzt.
2. **Die zwei Befehle reihen sich bei den vorhandenen Werten ein**: `git_bereich_umschalten` unter
   `Fenster` („Das Anwendungsfenster und seine Bereiche: wechseln, ein- und ausblenden, Breiten"),
   `fokus_git` unter `LeisteUndFokus`.
   - Pros: Kein zehntes Obermenü, keine Änderung an der Aufzählung, kein Nachzug der Probe. Der
     Umschalter passt dem Wortlaut nach in `Fenster`.
   - Cons: Es ist genau die Ausnahme, die `belegungsmodell.rs` für den Editor ausdrücklich abgelehnt
     hat, mit ausgeschriebener Begründung; die Regel hätte danach einen Sonderfall, und der nächste
     Bereich stellte die Frage erneut. `fokus_git` unter `LeisteUndFokus` ist zudem sachlich falsch:
     jener Wert heißt „Die Leiste aus C5 samt ihrem Ein- und Ausblenden und den beiden Fokusbefehlen,
     die zwischen ihr und dem Dateifenster wechseln", und der Git-Bereich ist weder das eine noch das
     andere. Die Runde 24 stünde vor derselben Frage mit vier weiteren Befehlen, und dann wäre die
     Antwort ohnehin ein eigener Wert — nur mit zwei Befehlen, die dann umziehen müssten.
3. **`Funktionsbereich::Vorschau` mitbenutzen**, weil sich Vorschau, Editor und Git dieselbe Fläche
   teilen.
   - Pros: Kein neuer Wert, und die drei stehen räumlich beieinander.
   - Cons: Der Name des Obermenüs sagte dann etwas anderes als sein Inhalt. Die Gliederung fragt nach
     der Gegend der Anwendung, und der Git-Bereich ist eine andere Gegend als die Vorschau, auch wenn
     beide am rechten Rand sitzen; der Editor teilt sich dieselbe Fläche und hat aus genau diesem
     Grund seinen eigenen Wert bekommen.

## Constraints

- `belegungsmodell::bereich` ist ein vollständiges `match` über `Kommando` ohne Auffangzweig; jeder
  der drei neuen Befehle braucht in jedem Fall eine Zeile.
- Ein `Funktionsbereich`, dessen Funktionen sämtlich unbelegt sind, erzeugt keinen Abschnitt in der
  Belegungsausgabe (`belegungsausgabe.rs`, Probe „nur der eine besetzte Bereich bekommt einen
  Abschnitt"). Ein zehnter Wert muss deshalb zusammen mit seinen Kommandos landen und nicht davor.
- Die Reihenfolge der Aufzählung ist die Reihenfolge der Obermenüs; `Anwendung` steht vorn und
  `Fenster` hinten, weil macOS es so hält.
- Die Antwort bindet die Runde 24. Ein Umzug danach kostet den Nutzer die Gewohnheit.

## Recommendation

Wir empfehlen Möglichkeit 1. Die Regel, nach der dieses Projekt seine Befehle gliedert, steht in
`belegungsmodell.rs` dreimal ausgeschrieben und hat für den Editor schon einmal genau gegen die
Bequemlichkeit entschieden, die Möglichkeit 2 anbietet; eine Ausnahme jetzt machte aus einer Regel
eine Ermessensfrage, die bei jedem weiteren Bereich neu zu stellen wäre. Der genannte Nachteil ist
real und vorübergehend: das Obermenü ist bis zur Runde 24 dünn, und danach trägt es sechs Befehle.

Wer Möglichkeit 2 wählt, sollte die Regel dazu ausschreiben, nach der künftig entschieden wird,
welcher Bereich einen eigenen `Funktionsbereich` bekommt und welcher nicht; ohne sie ist die
Einreihung die teurere Antwort.

---
Abgleich 260831-1417: Der Marker bleibt `_o_`, und das ist Absicht. Die Vorbelegung des Plans (Entscheidung 8) ist mit Schritt 8 gebaut — `Funktionsbereich::Git` steht als zehnter Wert in `crates/krk-ui/src/belegungsmodell.rs:101`, `make menue` führt das zehnte Obermenü an achter Stelle zwischen „Editor" und „Bearbeiten" (`260831-1334-coder-schritt-16-die-abnahmekommandos-ohne-fenster.md`) —, aber gebaut ist nicht beantwortet: der Plan schreibt unter `## Where this Circle stops` aus, dass dieser Datensatz nach der Runde weiter auf `_o_` steht, sofern der Nutzer ihn nicht beantwortet. Ein `_a_` oder `_i_` nähme ihn aus der Suche nach aktiver Grundlage heraus, und die Frage wäre der Sache nach entschieden, ohne dass jemand sie entschieden hätte.
