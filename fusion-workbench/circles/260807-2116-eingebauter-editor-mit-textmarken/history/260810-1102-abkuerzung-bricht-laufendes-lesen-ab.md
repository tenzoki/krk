# Die Abkürzung für die gehaltene Datei bricht das laufende Lesen ab

**Agent:** coder
**Status:** Complete
**Auftrag:** Behebung des Defekts
`issues/260810-1029_*_die-abkuerzung-fuer-die-gehaltene-datei-bricht-das-laufende-lesen-nicht-ab.md`
**Dateigrenze:** ausschließlich `crates/krk-ui/src/editormodell.rs`

---

## Was geändert wurde

Eine Zeile Code und drei Stellen Prosa in
`crates/krk-ui/src/editormodell.rs`.

`Editormodell::oeffnen` gibt in der Abkürzung für die schon gehaltene Datei den
laufenden Ladevorgang auf, bevor sie `Ladeausgang::SchonOffen` liefert:

```rust
if self.haelt_bereits(pfad) {
    self.ladevorgang = None;
    return Some(Ladeausgang::SchonOffen);
}
```

Aufgegeben wird über den Mechanismus, den der gewöhnliche Weg eine Zeile weiter
unten schon in Anspruch nimmt, und nicht über einen zweiten daneben: der Vorgang
fällt, sein Empfänger mit ihm, und das `send` des überholten Fadens scheitert
still. Es ist keine Generationsprüfung dazugekommen und kein Feld.

Die Prosa zieht den Satz nach, der damit gilt: **höchstens ein Lesen ist offen,
und es ist das zuletzt begonnene.** Er steht im Modulkopf unter "Der
Arbeitsfaden" und am Doc-Kommentar von `oeffnen`, jeweils mit dem Fall, der ihn
bis zum 260810 brach. "Lesen" und nicht "Öffnen", weil die zurückgehaltene Datei
fertig gelesen ist und ihr Ausgang beim Aufrufer steht statt bei einem Faden;
der Unterschied trägt den zweiten Datensatz unten.

## Die Probe

`editormodell::tests::die_abkuerzung_fuer_die_gehaltene_datei_bricht_das_laufende_lesen_ab`
fährt den Ablauf des Datensatzes: der Editor hält die eine Datei, der Nutzer
öffnet die andere, und während die gelesen wird, holt er die gehaltene zurück.
Geprüft wird, dass genau ein Ladeausgang kommt, und zwar der der gehaltenen
Datei.

Die Probe hängt nicht an einer Wettlage, aus demselben Grund wie
`ein_zweiter_ladevorgang_laesst_den_ersten_verfallen`: der Empfänger fällt in dem
Augenblick, in dem `oeffnen` den Vorgang aufgibt. Die Schleife am Ende wartet
trotzdem 300 Millisekunden ab, in denen der Faden bei diesen Dateigrößen längst
geliefert hätte — die Zusage lautet "genau ein Ausgang", und ein zweiter fällt
nur auf, wenn jemand auf ihn wartet.

**Gegengeprüft.** Mit zurückgenommener Fixzeile schlägt die Probe an zwei
Stellen unabhängig fehl: an `laedt_noch` unmittelbar, und mit der Zeile
abgeschaltet an dem zweiten Ausgang `Some(Geoeffnet)`, der aus der Schleife
nachkommt. Eine Probe, die auf dem defekten Code durchgelaufen wäre, hätte
nichts belegt.

## Die verlangte Durchsicht

Der Datensatz verlangte die Durchsicht der Abnahmekriterien von C2 und C4, weil
ungeprüft war, ob eine Zusage an dem stehengelassenen Lesen hängt. Sie ist
gefahren: **keine hängt daran.** C2 sagt über den Wechsel auf eine andere Datei
allein zu, dass die Prüfung vor der Nachfrage steht (elftes Kriterium), und das
entscheidet `uebernehmen_oder_zurueckhalten` unberührt weiter. C4 sagt über den
Ladevorgang nichts.

## Was der Fix nicht anfasst, und warum

Der Einzugstakt in `crates/krk-ui/src/appkit/editor.rs` läuft nach dem
aufgegebenen Vorgang noch, und das ist richtig ohne Zutun: `einziehen` beendet
ihn, sobald `laedt_noch` falsch ist und kein Einfärbungsfaden läuft
(`editor.rs:1168-1175`). Der Fix brauchte deshalb keine Zeile in `appkit/` und
hat die Dateigrenze nicht berührt.

## Ein zweiter Fall derselben Art, außerhalb der Grenze

Dabei aufgefallen und als eigener Datensatz abgelegt:
`issues/260810-1102_o_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`.

Die zurückgehaltene Datei überlebt einen zweiten Öffnungsbefehl ebenso, und die
Antwort auf das Blatt aus C4 überschreibt den letzten Befehl des Nutzers.
Erreichbar ist die Spanne, weil der Fokusvorbehalt Tastenbefehle nicht anhält,
solange ein Blatt steht, sondern nur dann, wenn der Ersthelfer eine Textklasse
ist; das Blatt aus C4 trägt drei Schaltflächen und kein Textfeld. Der Fund reicht
über den Editor hinaus und betrifft alle fünf Blätter.

Behoben ist er nicht: beide Wege dahin liegen in `crates/krk-ui/src/appkit/`, an
dem zur selben Zeit ein anderer Agent arbeitete. `inference:` am Code gelesen,
nicht am Bündel gemessen.

## Abnahme

| Kommando | Rückgabewert |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |
| `cargo fmt -p krk-ui -- --check` | 0 |

Die neue Probe läuft mit: 316 Proben im Binärziel `krk`, keine fehlgeschlagen.
