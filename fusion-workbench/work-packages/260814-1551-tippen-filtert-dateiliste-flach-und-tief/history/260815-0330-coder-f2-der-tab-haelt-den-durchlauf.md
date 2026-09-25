# F2 — Der Tab hält den Durchlauf, zieht die Befunde ein und bricht ihn ab

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang F, Schritt F2
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C2.5 bis C2.9, C2.11, C2.13, C3.2, C3.3, C3.6, C3.7, C3.11, C3.12, C3.14
**Verification:** `make check` — exit 0

## Was entstanden ist

`Tabinhalt` hält neben seinem `lesevorgang` jetzt einen `durchlauf: Option<Durchlauf>`, und
der Einzugstakt bedient zwei Kanäle statt einem.

```text
Anlass ──> Tabliste::durchlauf_nachziehen ──> altes Feld auf None (Drop bricht ab)
             │                                        │
             │   vier Bedingungen: gelesen, nicht lesend, Filtertext, "Deep"
             │                                        │
             └────────────────────────> auftraege(&Ordnermodell) ──> leer? kein Faden
                                                      │
                                        Durchlauf::starten(...) ──> tab.durchlauf

Einzugstakt (1/60 s)
   ├── lesemeldungen_einziehen ──> Stapel, Abschluss ──> Einzug{angehaengt,ersetzt,fertig}
   │        └── fertig? ──> durchlauf_nachziehen_an(stelle)
   └── befunde_einziehen ──> try_recv-Schleife ──> befunde_setzen(Reihe) ──> befunde_neu
```

**Die Anlässe stehen an einer Stelle und nicht an sechs.** `Tabliste::durchlauf_nachziehen`
ist die eine Stelle, an der ein Durchlauf entsteht und vergeht; sie nimmt den alten in
jedem Fall weg und startet einen neuen, wenn seine vier Bedingungen stehen. Gerufen wird
sie von drei Anlässen: `nach_filteraenderung` in `appkit/tabelle.rs` (der eine Weg jeder
Änderung des Filtertexts, also Tippen, Rückschritt und `Esc` zugleich),
`tiefe_suche_umschalten` daneben, und dem Einzugstakt bei `Einzug::fertig`. Die übrigen
Anlässe brauchen keinen Ruf, weil der `Tabinhalt` mit dem Durchlauf dort ohnehin fällt: der
Ordnerwechsel tauscht ihn aus, das Schließen nimmt ihn weg, `Tabliste::abbrechen` setzt das
Feld zurück, und `lesen_starten` tut dasselbe, weil der Bestand gerade abgelöst wird und
ein Eintragsindex des alten Bestands danach auf einen beliebigen anderen Eintrag zeigte.

**Der Rückgabewert trägt `#[must_use]`**, und die Begründung ist die des Projekts: fiele er
still, bliebe der Einzugstakt aus, die Befunde stünden im Kanal, und die Zeilen erschienen
nie. Der eine Aufrufer, der ihn nicht braucht, ist der Takt selbst — dort steht `let _ =`
mit dem Grund daneben.

## Die vier Bedingungen des Starts, und die eine, die leicht fehlt

`gelesen && !liest()` ist die tragende. Ein Lesevorgang leert sein Ordnermodell nicht
vorab, sondern ersetzt es mit dem ersten Stapel; wer in dieser Spanne die Auftragsliste
bildete, benennte Ordner des **alten** Ordners unter Indizes, die gleich einem anderen
Eintrag gehören werden. Deshalb hängt der Anlass an `Einzug::fertig` und nicht am Tippen
allein.

`filter_steht() && tief()` sind die beiden aus der Directive. Die vierte ist die leere
Auftragsliste: kein Auftrag, kein Faden, und damit zählt C3.14 buchstäblich.

## Die Auftragsliste als reine Funktion (C3.14)

`fn auftraege(&Ordnermodell) -> Vec<Auftrag>` in `tabs.rs`, ohne AppKit und ohne
Tabliste, weil `krk-ui` kein Bibliotheksziel hat und eine Probe von außen nicht ansetzen
könnte. Zwei Bedingungen:

1. **Ist es ein Ordner?** Mit demselben Schnitt, den `Ordnermodell::sichtbar` zieht: eine
   symbolische Verknüpfung zählt mit. Ein zweiter Schnitt hier hieße, dass eine
   Verknüpfung nie einen Befund bekäme und damit von „noch nicht entschieden" nicht zu
   unterscheiden wäre (C2.13, C3.13).
2. **Trägt der Name die Folge — nein?** Gefragt über `Ordnermodell::name_traegt_den_filter`
   und nicht über einen eigenen Vergleich. Das ist die Wurzelbehebung, die die zweite
   Diagrammprüfung erzwungen hat: die Zuständigkeitsgrenze steht am Eingang der
   Auftragsliste und nicht als Sonderfall an einem Ausgang.

**Ein ausgeblendeter Ordner steht mit in der Liste.** Die Regel, die ihn wegblendet, ist
der erste Zweig von `sichtbar`, und ein zweites Mal hier gefragt wäre die zweite Fassung
derselben Regel, die diese Runde gerade abgeschafft hat. Umsonst ist der Befund nicht:
blendet der Nutzer die versteckten Einträge während des Durchlaufs ein, steht die Zeile
sofort richtig da.

## Was die Zählprobe von A2 gefunden hat

Der erste Bau rief `traegt_die_folge` unmittelbar aus `tabs.rs`, und
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer` schlug fehl:
drei Rufer statt zwei. Der Befund ist berechtigt, auch wenn keine zweite Fassung entstanden
war — ein dritter Rufer aus `krk-ui` heißt, dass die Oberfläche eine Regel des Kerns direkt
stellt, statt das Modell zu fragen, das sie ohnehin führt. Die Behebung ist
`Ordnermodell::name_traegt_den_filter`: der Zweig des Prüfschritts, herausgegeben.
`sichtbar` ruft ihn jetzt selbst, der Vergleich behält seine zwei Rufer, und `tabs.rs`
kennt weder `filter_klein` noch die Kleinschreibung.

## Der Befund zu `befund_setzen`

`issues/260814-2145_*` ist geschlossen, und die vorgeschlagene Gegenmaßnahme ist gebaut:
`befund_setzen` heißt jetzt `befunde_setzen` und nimmt eine Reihe entgegen; der Neuaufbau
läuft einmal je Reihe statt einmal je Ordner. Gerufen wird er einmal je Einzugstakt,
nachdem `befunde_einziehen` den Kanal mit einer `try_recv`-Schleife leergeräumt hat. Der
Takt hatte die Reihe ohnehin schon in der Hand; die Gegenmaßnahme kostet weniger als ihre
Messung, und die Messung bräuchte den Abnahmelauf am Bündel, also Nutzerarbeit. Ein
einzelner Setzer bleibt nicht daneben stehen — zwei Schreiber wären zwei Bauarten mit zwei
Neuaufbau-Verhalten; die sieben Rufer in `crates/krk-core/tests/verzeichnis.rs` sind auf die
Reihenform umgestellt.

## Die Anzeige springt nicht (C3.11)

`Einzug` bekommt sein fünftes Feld `befunde_neu`, und die Ansicht antwortet darauf wie auf
`ersetzt`: `reloadData`, danach `auswahl_anzeigen`. `noteNumberOfRowsChanged` genügt hier
nicht, denn ein Befund stellt seine Zeile an die Stelle, die die Sortierung ihr zuweist,
also mitten in die Liste; alle Zeilen darunter tragen danach einen anderen Eintrag.
`reloadData` rührt den Bildlauf nicht an, und die Auswahl hängt am Eintragsindex und
wandert mit.

**Der Einzugstakt fragt jetzt `arbeitet_noch` und nicht `liest_noch`.** Ein Durchlauf läuft
gerade dann, wenn kein Lesevorgang mehr läuft; mit der engeren Frage hielte der Takt an,
bevor der erste Befund da ist.

## Was ohne Zeile anfällt und von zählenden Proben gehalten wird

- **C3.2** — Dateien und namentlich passende Ordner stehen sofort. Keine Zeile: der
  Prüfschritt entscheidet sie am Namen, und die Auftragsliste nimmt sie nicht auf.
- **C2.9** — `Spalte::ALLE.len() == 4`, und `NSOutlineView` kommt in `appkit/tabelle.rs`
  null Mal vor. Der Quelltext ist über `include_str!` gebunden.
- **C2.10 und C6.1** — der Pfad, den `betroffene` für eine allein über einen tiefen
  Treffer sichtbare Zeile baut, hat unter dem angezeigten Ordner genau **einen**
  Bestandteil.
- **C2.11** — im Rumpf von `angezeigtedatei.rs` stehen genau zwei `return Some(`.

## Fünfzehn Proben

In `tabs.rs`, `#[cfg(test)]` neben dem Code: fünf über die Auftragsliste (C3.14 zweimal,
C3.2, C2.13, Eintragsindex statt Zeile), fünf über Anlass und Abbruch (C3.6 zweimal, C3.7,
Tabwechsel, `abbrechen`), eine über den ganzen Weg von F2 ohne AppKit (Lesevorgang,
`Einzug::fertig`, Auftragsliste, Arbeitsfaden, Befundkanal, `befunde_setzen`, Zeile steht),
und vier über die Zusagen, die ohne Zeile anfallen.

## Dateien

- `crates/krk-ui/src/tabs.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-core/src/verzeichnis/modell.rs` — `befunde_setzen` und
  `name_traegt_den_filter`, beide aus F2 heraus veranlasst
- `crates/krk-core/tests/verzeichnis.rs` — die sieben Rufer des Setzers
