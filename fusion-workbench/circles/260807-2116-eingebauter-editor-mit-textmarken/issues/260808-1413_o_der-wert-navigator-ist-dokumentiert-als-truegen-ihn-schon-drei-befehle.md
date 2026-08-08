# Der Wert `Navigator` ist dokumentiert, als trügen ihn schon drei Befehle

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht von Turn 1 der Editor-Runde
**Betroffen:** `crates/krk-core/src/tasten/belegung.rs`
**Cross-references:** Plan S3 und S5, `crates/krk-ui/src/kommandos/fokus.rs:233-245`

---

## Der Befund

S3 hat `Wirkungsbereich` um `Vorschau`, `Editor` und `Navigator` erweitert. Der
Umzug der drei Befehle, für die `Navigator` gebaut wurde, steht im Plan unter S5
und ist nicht Teil von S3. Die Dokumentation an der Aufzählung sagt das nicht.

`crates/krk-core/src/tasten/belegung.rs:193-212`:

```rust
/// Wirkt in den Bereichen des Navigators aus der Runde 1, also im
/// Dateifenster, in der Leiste und im Vorschaufenster, aber nicht im
/// Editor.
///
/// Der Wert der Befehle, deren Taste im Editor der Textflaeche gehoert:
/// `fenster_wechseln` auf `tab`, `auswahl_hoch` auf `up` und
/// `auswahl_runter` auf `down`. Sie sind in der Runde 1 mit
/// [`Wirkungsbereich::Ueberall`] entstanden, weil es damals nichts gab,
/// wovon sie auszunehmen waeren.
Navigator,
```

"Der Wert der Befehle … `fenster_wechseln`, `auswahl_hoch`, `auswahl_runter`",
Gegenwart, mit dem `Ueberall` der Runde 1 in der Vergangenheitsform daneben. Wer
das liest, hält den Umzug für vollzogen.

`Kommando::wirkungsbereich` 250 Zeilen weiter unten sagt das Gegenteil
(`belegung.rs:456-474`):

```rust
Kommando::FensterWechseln
| ... 
| Kommando::Beenden => Wirkungsbereich::Ueberall,
// Die Auswahl des fokussierten Bereichs (C2 und C5).
Kommando::AuswahlHoch | Kommando::AuswahlRunter => Wirkungsbereich::Ueberall,
```

Alle drei tragen weiterhin `Ueberall`. An dieser Stelle steht **kein** Hinweis,
dass ein Schritt sie ablöst; der Kommentar über `AuswahlHoch | AuswahlRunter`
ist der unveränderte aus der Runde 1.

## Warum das zählt

`Wirkungsbereich::Navigator` ist heute ein Wert, den kein Kommando trägt. Solange
das so ist, ist die Zusage aus dem eigenen Kommentar — "Ohne diesen Wert bewegte
`up` im Editor die Auswahl im Dateifenster statt der Schreibmarke, und das erste
Abnahmekriterium von C7 wäre gebrochen" — **nicht** eingelöst, denn der Wert ist
zwar da, aber nicht angewandt. Der Text liest sich, als wäre er es.

Die Probe in `krk-ui` hält den Stand ehrlich fest
(`crates/krk-ui/src/kommandos/fokus.rs:236-240`):

> Ohne [`Wirkungsbereich::Navigator`] trägt `auswahl_hoch` weiter
> [`Wirkungsbereich::Ueberall`] … **Der Umzug der drei betroffenen Befehle steht
> in S5**; diese Prüfung sichert die Regel, auf die er sich stützt.

Der Kern, der die Zuordnung führt, trägt diesen Satz nicht. Die verlässlichere
der beiden Stellen ist damit die entferntere.

## Was zu tun ist

Zwei Zeilen, beide in `crates/krk-core/src/tasten/belegung.rs`:

1. Den Absatz an `Navigator` (`:197-203`) in die Zukunftsform setzen und S5 als
   den Schritt nennen, der die drei Befehle umzieht — in derselben Form, in der
   `anwendung.rs:1108` und `anwendung.rs:1562` ihren Schritt nennen
   ("**S17 löst diese Zeile ab**").
2. Über `Kommando::AuswahlHoch | Kommando::AuswahlRunter` und über
   `Kommando::FensterWechseln` je einen Vermerk setzen, dass S5 sie nach
   `Navigator` bringt. Ohne ihn findet die Zuordnungsstelle den Vorwärtsverweis
   nicht, und sie ist die Stelle, die jemand ändert.
