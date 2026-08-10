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
