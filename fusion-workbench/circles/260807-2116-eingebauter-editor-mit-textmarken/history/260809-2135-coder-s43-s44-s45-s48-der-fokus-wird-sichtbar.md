# S43, S44, S45 und S48: Der Fokus wird sichtbar, und der Titel folgt ihm

**Agent:** coder
**Datum:** 2026-08-09, 21:35
**Circle:** `circles/260807-2116-eingebauter-editor-mit-textmarken`
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Phasen J und L
**Status:** Complete

## Was gebaut wurde

Vier Schritte in einem Zug, weil alle vier in `anwendung.rs` und `aufteilung.rs`
schreiben und einzeln gefahren einander überschrieben hätten.

**S43** stellt `Anwendungsdelegierter::fokus` von Nämlichkeit auf Enthaltensein
um. Der Durchgang läuft über `Bereich::ALLE`, holt zu jedem Wert die
Wurzelansicht über die neue Zugriffsfunktion `Aufteilung::bereichssicht` und
fragt `NSView::isDescendantOf:`; von `Bereich` auf `Fokus` kommt die neue
erschöpfende Zuordnung `kommandos::fokus::in_bereich`. Damit ist der Befund
`260809-1738` behoben, und sein Datensatz trägt den Marker `_c_` samt
Abschlussnotiz.

**S44** gibt allen fünf Bereichen einen `NSBox` statt nur den beiden
Dateifenstern und färbt ihn nach einer Regel, die außerhalb von `appkit`
steht: `Rahmenrolle` mit drei Werten und `rahmenrolle(bereich, fokus, aktiv)`.
`Aufteilung::aktives_markieren` ist zu `rahmen_setzen(fokus, aktiv,
im_vordergrund)` geworden. Drei Stellen, die die Zuordnung von Fokuswert auf
Bereich vorher je für sich rechneten, gehen jetzt über die eine neue
`bereich_mit_fokus`.

**S45** macht das Fenster zu einer eigenen Klasse `Hauptfenster`, einer
Unterklasse von `NSWindow` mit drei Überschreibungen: `makeFirstResponder:`,
`becomeKeyWindow` und `resignKeyWindow`. Sie meldet jeden Wechsel an
`Anwendungsdelegierter::fokusanzeige_nachziehen`, und damit folgt die Anzeige
dem Fokus auch beim Mausklick, den KRK selbst nicht auslöst. Weil S44 und S45
zusammen gefahren wurden, hat der von S44 angekündigte Zwischenstand in
`fokus_setzen` nie existiert.

**S48** baut `crates/krk-ui/src/fenstertitel.rs`: eine reine Funktion über die
fünf Fokuswerte, ohne AppKit und ohne Auffangzweig, dazu
`Anwendungsdelegierter::titel_nachziehen` und seine vier Aufrufstellen.

## Die fünf öffentlichen Formen aus der Plantabelle, und wer nachgezogen wurde

| Form | Aufrufstellen, die nachgezogen wurden |
|---|---|
| `Anwendungsdelegierter::fokus` antwortet für Unteransichten anders | keine Signaturänderung; alle Leser (`kommando_ausfuehren`, `breite_aendern`, der Zeichenzweig in `eingabe_ausfuehren`, `titel_nachziehen`) bekommen denselben Typ |
| `Aufteilung::bereichssicht` (neu) | ein Aufrufer, `Anwendungsdelegierter::ersthelferbereich` |
| `Aufteilung::aktives_markieren` → `rahmen_setzen(fokus, aktiv, im_vordergrund)` | ein Aufrufer, aus `aufteilung_nachziehen` nach `fokusanzeige_nachziehen` verlegt |
| `Aufteilung.rahmen` von zwei auf fünf Kästen | modulintern; `Aufteilung::bauen` behält seine Aufrufform |
| `hauptfenster` liefert `Retained<Hauptfenster>` | ein Aufrufer, `oberflaeche_aufbauen`; die Ivars halten weiter `Retained<NSWindow>` über `Retained::into_super` |

Dazu drei Stellen, die der Plan bei S44 nennt und die dieselbe Zuordnung
zweimal rechneten: `fokus_holen`, `fokus_setzen` und `breite_aendern` gehen
jetzt alle über `bereich_mit_fokus`. Das Verhalten ändert sich dabei an keiner
Stelle, weil das aktive Dateifenster nie ausgeblendet ist — die Zusage steht
seit der Runde 1 an `holt_hervor` und ist die Bedingung, unter der die alte und
die neue Rechnung dasselbe liefern.

Die vier Aufrufstellen des Titels: der Ordner- und Tabwechsel eines
Dateifensters über den Melder in `oberflaeche_aufbauen`, der Dateiwechsel im
Editor in `im_editor_oeffnen`, der Tabwechsel der Vorschau im
Vorschau-Zweig von `bereichskommando`, und der Fokuswechsel über
`fokusanzeige_nachziehen`. Dazu die letzte Zeile des Aufbaus, nachdem
`fokus::BEIM_START` gesetzt ist.

## Zwei Abweichungen vom Plan, beide benannt

**Erstens: `fokus()` ist in zwei Funktionen zerlegt.** Der Plan sieht eine vor.
Der Grund ist das achte Abnahmekriterium von C9: geht das Fenster in den
Hintergrund, soll die Anzeige zurücktreten. `fokus()` antwortet in genau diesem
Augenblick `Fokus::Anderswo`, weil es kein Schlüsselfenster mehr gibt — die
Anzeige hätte also nichts zu schreiben gehabt und wäre in voller Akzentfarbe
stehen geblieben. `ersthelferbereich()` beantwortet deshalb die Frage ohne die
Vorabfrage nach dem Schlüsselfenster, und `fokus()` ist jene Vorabfrage plus
dieser Aufruf. Zwei Fragen, zwei Antworten, eine Rechnung: `fokus` fragt "wohin
geht ein Befehl jetzt", `ersthelferbereich` fragt "wo liegt der Ersthelfer".

Damit trägt das siebte Abnahmekriterium von C9, dass ein stehendes Blatt keinem
Bereich seine Anzeige nimmt, eine eigene Zeile: `fokusanzeige_nachziehen` fragt
`blatt_steht()` und schreibt dann gar nichts. Der Plan wollte es aus
`Fokus::Anderswo` anfallen lassen; das geht nicht mehr, seit die Anzeige nicht
mehr über `fokus()` liest. Für die drei übrigen Aufrufstellen von
`titel_nachziehen` gilt die Plan-Fassung unverändert weiter: sie reichen
`self.fokus()` herein, und ein Blatt ergibt dort `Anderswo` und damit `None`.

**Zweitens: `Fokus::ALLE` trägt jetzt `#[cfg(test)]`.** Die Aufzählung entstand
mit S17, weil die Fokusabfrage sie durchlief. S43 hat die Abfrage auf
`Bereich::ALLE` umgestellt, und damit zählt das Programm die Fokuswerte
nirgends mehr auf. Vier Proben tun es weiterhin, und ohne die Aufzählung führte
jede von ihnen eine eigene Liste derselben fünf Werte. `#[cfg(test)]` sagt
genau das; ein `#[allow(dead_code)]` mit einer Ankündigung wäre die unehrlichere
Antwort gewesen. Die Plantabelle führt `Fokus` unter "nicht geändert" — die
Aufzählung ist geblieben, ihr Geltungsbereich nicht.

## Der offene Datensatz hat nicht aufgehalten

`decisions/260809-2043_o_bedeutet-der-akzentrahmen-kuenftig-den-fokus-oder-das-aktive-dateifenster.md`
ist unverändert offen. Gebaut ist die Vorbelegung des Specs, die erste
Möglichkeit. Eine andere Antwort ändert einen Funktionsrumpf und keinen Aufbau:
unter der dritten Möglichkeit entfällt `Rahmenrolle::AktivOhneFokus` in
`rahmenrolle` und wird `Ruhig`, unter der zweiten kommt eine zweite Anzeige
daneben. Die fünf Kästen, der Auslösepunkt und `bereich_mit_fokus` bleiben in
allen drei Fällen unberührt. Der Doc-Kommentar an `Rahmenrolle` sagt es.

## Geänderte und neue Dateien

Neu:

- `crates/krk-ui/src/fenstertitel.rs`

Geändert:

- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/aufteilung.rs`
- `crates/krk-ui/src/appkit/fenster.rs`
- `crates/krk-ui/src/kommandos/fokus.rs`
- `crates/krk-ui/src/main.rs`
- `crates/krk-ui/src/appkit/editor.rs` — **eine Zugriffsfunktion, additiv**

Die Zeile in `editor.rs` ist zu benennen, weil die Datei für das parallel
laufende Bündel S46/S47 reserviert war. S48 braucht den Pfad der gehaltenen
Datei, und `Editorbereich` gab ihn bisher nicht heraus; `Editormodell::pfad` ist
seit S9 öffentlich, die Hülle darum fehlte. Die neue `Editorbereich::pfad` steht
neben `haelt_datei` und weit entfernt von `textflaeche_bauen`, das S46 angefasst
hat; beide Änderungen stehen nebeneinander im Baum, ohne einander zu berühren.

## Die vier Abnahmekommandos

Alle vier grün, geprüft über `make check`:

```
cargo build --workspace      0
cargo test --workspace       0, 55+139+34+43+15+26+7+5+20+16+9+251+5+35 Proben
cargo fmt --all --check      0
cargo clippy --workspace --all-targets -- -D warnings   0
cargo xtask bundle           target/KRK.app gebaut und signiert
```

## Die Greps aus den Abnahmekriterien

| Kriterium | Ergebnis |
|---|---|
| `grep -rn 'aktives_markieren' crates/krk-ui/src` findet nichts | 0 Code-Stellen, 1 Doc-Kommentar (die Nachfolgerin nennt ihre Vorgängerin) |
| `grep -c 'holt_hervor' .../anwendung.rs` liefert 0 | 0 Code-Stellen, 5 Doc-Kommentare |
| `grep -c 'controlAccentColor' .../aufteilung.rs` liefert 2 | 2 |
| `grep -c 'fokusanzeige_nachziehen' .../anwendung.rs`: ein Rumpf, zwei Aufrufstellen | ein Rumpf (2114), zwei Aufrufstellen (598, 2090), drei Doc-Verweise |
| `grep -rEln '#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src` nennt genau eine Datei | `appkit/mod.rs` |
| `grep -c 'objc2' crates/krk-ui/src/fenstertitel.rs` liefert 0 | 0 |
| `grep -rn 'setTitle' crates/krk-ui/src` zeigt zwei Stellen | zwei am Fenster: `fenster.rs:254` (Aufbau) und `anwendung.rs:2172` (`titel_nachziehen`); die übrigen Treffer sind `setTitle` an Spaltenköpfen und Schaltflächen und `setTitlePosition` am Kasten |
| Der Diff zeigt genau eine Überschreibung von `makeFirstResponder:` | eine, ruft die Oberklasse zuerst und gibt ihr Ergebnis unverändert zurück |
| `fokus()` ruft `fokusansicht` nicht mehr; `fokusansicht` steht unverändert | beides erfüllt |

**Zu den beiden ersten Zeilen.** Der Plan verlangt dort buchstäblich null
Treffer. Null Code-Stellen ist erreicht; was übrig bleibt, sind Doc-Kommentare,
die die abgelöste Fassung namentlich nennen — drei davon stehen seit S17
unverändert dort und sind weiterhin richtig, zwei hat dieser Schritt
geschrieben, um die Ablösung zu erklären. Sie zu tilgen hieße, richtige
Erklärungen für einen Grep zu opfern; der Zweck des Kriteriums, keine vergessene
Aufrufstelle, ist erfüllt.

## Was noch zu prüfen ist

Vierzehn Abnahmekriterien dieser vier Schritte verlangen KRK im Vordergrund und
sind von keinem Agenten zu fahren. Die Prüfliste steht im Bericht an den Nutzer
und nicht hier.

## Vermerk zum Stilprofil

`fusion-rules coder` hat allein `stilwerk/chat-voice-de.yaml` geliefert und kein
`default-voice-de.yaml`. Diese Datei folgt deshalb der Projektsprache aus
`CLAUDE.md` und keinem geladenen Langform-Profil.
