# Die Abkürzung für die gehaltene Datei bricht das laufende Lesen nicht ab

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Behebung von `260810-0418`
**Betroffen:** `crates/krk-ui/src/editormodell.rs` (`Editormodell::oeffnen`)
**Cross-references:** `issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`, `issues/260810-0418_*_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`, C2

---

## Der Befund

`Editormodell::oeffnen` nimmt für die schon gehaltene Datei eine Abkürzung, und
sie kehrt zurück, **bevor** sie den Ladevorgang ansieht:

```rust
pub fn oeffnen(&mut self, pfad: &Path) -> Option<Ladeausgang> {
    if self.haelt_bereits(pfad) {
        return Some(Ladeausgang::SchonOffen);
    }
    self.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf()));
    None
}
```

Ein laufendes Lesen bleibt damit stehen. Der gewöhnliche Weg ersetzt es und
lässt den Empfänger des überholten Fadens fallen; die Abkürzung tut beides
nicht, und deshalb ist sie der eine Fall, in dem **zwei** Ladeausgänge zu einer
Folge von Öffnungen gehören.

## Der Ablauf, in dem es sich zeigt

Der Editor hält B. Der Nutzer öffnet A, das Lesen läuft. In dieser Spanne holt
er mit F4 B zurück — ein Weg, den `260809-2029` als den namentlich gegangenen
festhält, weil die Vorschau den Editor nach C1 verdrängt.

1. `SchonOffen` kommt unverzüglich: der Editor wird hervorgeholt, der Fokus
   gesetzt, der Titel und die Sitzung tragen B. Richtig.
2. Danach liefert der Faden für A: `Geoeffnet`, der Editor hält A, Titel und
   Sitzung tragen A.

Das zweite F4 des Nutzers ist damit still überschrieben. Verloren geht dabei
kein Text — ein ungesicherter Stand in B ließe A über
`Ladeausgang::Zurueckgehalten` durch die Nachfrage aus C4 laufen —, wohl aber
die Wirkung des letzten Befehls. Erreichbar ist die Spanne nur, solange das
Lesen von A dauert; deshalb Low.

## Der Weg dahin

Eine Zeile in der Abkürzung, die den laufenden Ladevorgang aufgibt, bevor sie
`SchonOffen` liefert: der Nutzer hat B verlangt, also gehört das Lesen von A
niemandem mehr. Damit gilt für das ganze Modell derselbe Satz, der es heute nur
fast tut — **höchstens ein Öffnen ist offen, und es ist das zuletzt begonnene** —
und `Editormodell::oeffnen` hätte an beiden Ausgängen dieselbe Regel statt zwei.

Ungeprüft ist, ob eine Zusage des Specs an dem stehengelassenen Lesen hängt;
C2 nennt keine. Der Schritt gehört deshalb nicht in eine Behebung nebenbei,
sondern braucht die Durchsicht der Abnahmekriterien von C2 und C4.

## Warum es hier nicht behoben wurde

Der Fund fiel bei der Behebung von `260810-0418` an, und der dortige Fix hängt
**nicht** daran: die Marke der Herkunft ist im Ablauf oben in beiden Ausgängen
`Befehl`, und beim Start — der einzigen Spanne, in der eine Herkunft `Sitzung`
lautet — ist die Abkürzung unerreichbar, weil der Editor dann noch keine Datei
hält.

---
Resolved: Die Abkürzung gibt den laufenden Ladevorgang auf, bevor sie
`SchonOffen` liefert — eine Zeile `self.ladevorgang = None;` in
`Editormodell::oeffnen` (`crates/krk-ui/src/editormodell.rs`). Aufgegeben wird
über den bestehenden Mechanismus und nicht über einen zweiten daneben: der
Vorgang fällt, sein Empfänger mit ihm, und das `send` des überholten Fadens
scheitert still. Damit gilt an beiden Ausgängen von `oeffnen` derselbe Satz,
höchstens ein Lesen ist offen und es ist das zuletzt begonnene; der Satz steht
jetzt im Modulkopf und am Doc-Kommentar der Funktion, mit dem Fall, der ihn bis
zum 260810 brach.

Die Durchsicht, die der Abschnitt "Der Weg dahin" verlangt, ist gefahren: keines
der elf Abnahmekriterien von C2 und keines der elf von C4 hängt an dem
stehengelassenen Lesen. C2 sagt über den Wechsel auf eine andere Datei allein
zu, dass die Prüfung vor der Nachfrage steht (elftes Kriterium), und das
entscheidet `uebernehmen_oder_zurueckhalten` unberührt weiter. C4 sagt über den
Ladevorgang nichts.

Belegt durch die Probe
`editormodell::tests::die_abkuerzung_fuer_die_gehaltene_datei_bricht_das_laufende_lesen_ab`.
Sie fährt den Ablauf des Befundes und prüft, dass genau ein Ladeausgang kommt.
Gegengeprüft: mit zurückgenommener Fixzeile schlägt sie an zwei Stellen
unabhängig fehl, an `laedt_noch` und an dem zweiten Ausgang `Geoeffnet`, der 300
Millisekunden später aus dem Faden nachkommt.

Ein zweiter Fall derselben Art ist dabei aufgefallen und liegt außerhalb der
Dateigrenze dieser Behebung: die zurückgehaltene Datei überlebt einen zweiten
Befehl ebenso, und die Nachfrage aus C4 hält Tastenbefehle nicht an. Der
Datensatz ist
`issues/260810-1102_o_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`.

Abnahme: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt -p krk-ui -- --check`
je Rückgabewert 0, clippy ohne Warnung.
