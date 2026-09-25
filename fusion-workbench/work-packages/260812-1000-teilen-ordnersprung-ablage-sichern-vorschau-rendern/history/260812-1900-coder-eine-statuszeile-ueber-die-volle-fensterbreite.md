# Schritt 10: Die eine Statuszeile über die volle Fensterbreite

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 10
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 434, nachher 445

---

## Was gebaut wurde

Aus zwei Statuszeilen an zwei Dateifenster-Füßen ist eine über die volle
Fensterbreite geworden. Der Umzug verschiebt **den Halter und den Schreiber**
und lässt die Quellen, wo sie sind: die vier Meldungsfelder mit je einer
Löschregel stehen unverändert in `QuelleIvars`, und genau darauf beruht die
Zusage C5.7, dass Verdrängtes nicht gelöscht wird.

```
vorher                                nachher
──────                                ───────
DateifensterQuelle links              DateifensterQuelle links  ─┐
  vier Felder + Markierungsstand        vier Felder + Stand      │ meldungswechsel
  ──> eigene Statuszeile am Fuss                                 ├─> Anwendungsdelegierter
                                      DateifensterQuelle rechts ─┘   statuszeile_nachziehen
DateifensterQuelle rechts               vier Felder + Stand              │
  vier Felder + Markierungsstand                                         v
  ──> eigene Statuszeile am Fuss                            statuszeile::zeile
                                                        (erst Rang, dann aktive Seite)
                                                                         │
                                                                         v
                                                            statuszeile::zeilentext
                                                          (nennt die inaktive Seite)
                                                                         │
                                                                         v
                                                            die eine Statuszeile
```

## Die Rangfolge, neu gefasst

`statuszeile::zeile` bekommt beide Quellensätze und die aktive Seite:

```rust
pub fn zeile<'a>(links: &'a Quellen, rechts: &'a Quellen, aktiv: Fensterseite)
    -> Option<Meldung<'a>>
```

**Die fünf Ränge der Runde 1 stehen unverändert und in derselben Reihenfolge.**
Neu ist allein die zweite Stelle der Ordnung, und sie steht in der
Schleifenreihenfolge und nicht in einer Vergleichsfunktion: außen `Rang::ALLE`,
innen `[aktiv, aktiv.andere()]`. Über die zehn Paare ist sie vollständig und
überschneidungsfrei, weil zwei Bewerber desselben Ranges immer verschiedenen
Seiten gehören; ein Gleichstand kann gar nicht entstehen.

`Rang` ist eine neue vollständige Fallunterscheidung ohne Auffangzweig, mit
`Rang::ALLE` und `Rang::art`. **Die Art wird aus dem Rang gerechnet statt
gesetzt** — `Meldung.art` entsteht als `rang.art()` und hat keinen zweiten
Schreiber, der eine Markierungszahl rot färben könnte.

`Quellen` trägt eigene Zeichenketten und keine Ausleihen. Der Grund ist der
Modulkopf von `tabelle.rs`: der Nachzug fragt zwei Dateifenster nacheinander
und ruft danach AppKit, und keine Ausleihe des Tabmodells darf einen
Objective-C-Aufruf überleben.

## Der Preis, ausgeschrieben im Doc-Kommentar

Der Doc-Kommentar an `zeile` behält die Begründung der fünf Ränge Wort für Wort
und nennt daneben ausdrücklich, was die Zusammenlegung kostet: **laufen in
beiden Dateifenstern zugleich Vorgänge, ist nur der des aktiven zu sehen.** Der
zweite ist nicht verloren, sein Feld steht, und er erscheint, sobald der erste
fällt. Der Nutzer hat diesen Preis am 260812-1105 vorgelegt bekommen und
angenommen.

## Der Namenszusatz

`zeilentext` stellt den Namen des Dateifensters voran, genau dann, wenn die
Meldung nicht von der aktiven Seite kommt:

```
aktiv links,  Meldung links   ->  "Ordner nicht lesbar"
aktiv links,  Meldung rechts  ->  "rechtes Dateifenster: Ordner nicht lesbar"
```

**Eine Regel und kein Zweig für den Fall „es steht nur ein Dateifenster".** Ist
eines ausgeblendet, ist das andere das aktive, und die eine Bedingung deckt den
Fall mit ab. Die beiden Namen stehen als vollständige Fallunterscheidung über
`Fensterseite` in `statuszeile.rs`, weil Anzeigetexte in die Oberfläche gehören
und `Fensterseite` ein Wert der Ablage ist, der von Anzeige nichts weiß.

## Was in den fünf Dateien steht

**`statuszeile.rs`** — neu: `Quellen`, `Rang` samt `ALLE` und `art`,
`Meldung<'a>`, `zeilentext`, `seitenname`. `zeile` neu gefasst. `Statuszeile`
setzt ihre Autogröße nicht mehr selbst; sie hing am Fuß eines Dateifensters, und
die Wahl gehört jetzt dorthin, wo eingehängt wird. Der Modulkopf trägt zwei neue
Abschnitte: warum es eine Zeile über die volle Breite ist und kein sechster
Bereich, und was zum Ersthelferrang gilt.

**`tabelle.rs`** — `QuelleIvars` verliert `statuszeile` und bekommt
`meldungswechsel: RefCell<Option<Box<dyn Fn()>>>`, wahlfrei wie die vier
vorhandenen Rückrufe und aus demselben Grund. `meldung_anzeigen` heißt jetzt
`meldung_gewechselt` (15 Stellen) und schreibt nichts mehr; neu sind
`meldungsquellen()` und `meldungswechsel_setzen()`. `Dateifenster::bauen` baut
keine Statuszeile mehr, `statuszeile_sicht` ist weg. Der Modulkopf sagt, wohin
die Zeile gegangen ist.

**`aufteilung.rs`** — `dateifensterinhalt` legt nur noch Tableiste und Liste
übereinander, die Liste beginnt bei `0.0`. Der Doc-Kommentar rechnet vor, dass
die Liste dabei keine Höhe verliert. Der Import von `statuszeile` ist weg.

**`fenster.rs`** — `fensterinhalt` nimmt die Zeile als dritte Ansicht, zwischen
Bereichsleiste und Fensterzeile, mit Rahmen und Maske in denselben zwei Zeilen
wie die Leiste. Die Leiste behält den unteren Rand.

**`anwendung.rs`** — die Ivars halten die eine `Statuszeile`;
`statuszeile_nachziehen` ist der eine Schreiber. Beide Dateifenster bekommen
beim Aufbau ihren Rückruf.

## Die Mindesthöhe: 318 → 336

```
300  die Fensterzeile aus der Runde 1   (FENSTERZEILE_MINDESTHOEHE)
+18  Bereichsleiste                     (Runde 5)
+18  Statuszeile                        (Runde 6)
───
336
```

Eine Summe und keine gewählte Zahl, wie die 318 es schon war. Daneben steht die
Rechnung als Zusicherung beim Übersetzen:

```rust
const _: () = assert!(MINDESTGROESSE.height == 336.0, "…");
```

Sie hält nichts fest, was der Ausdruck darüber nicht enthielte, sondern die
Zahl, die herauskommen soll: ändert jemand eine der beiden Leistenhöhen, fällt
der Bau hier aus, und die Begründung wird gelesen, statt still zu veralten.

**Die Dateiliste verliert dabei keine Höhe** (C5.4). Vorher maß sie
`H − 18 (Bereichsleiste) − Tableiste − 18 (eigene Statuszeile)`, danach
`H − 18 (Bereichsleiste) − 18 (Statuszeile) − Tableiste`. Derselbe Ausdruck. Die
drei Bereiche ohne eigene Zeile — Lesezeichenleiste, Vorschau, Editor —
verlieren 18 Punkte, und genau die holt der dritte Summand zurück (C5.3).

## Wo der Nachzug hängt

`statuszeile_nachziehen` hat zwei Anlässe, nach dem Vorbild von
`bereichsleiste_nachziehen`:

1. **Der Meldungswechsel** eines der beiden Dateifenster, über den Rückruf aus
   dem Aufbau.
2. **`aufteilung_nachziehen`**, weil die Zeile nicht nur an den zehn Quellen
   hängt, sondern auch daran, welches Dateifenster das aktive ist: der Rang der
   aktiven Seite entscheidet jeden Gleichstand, und der Namenszusatz hängt an
   derselben Frage. Ein Wechsel des aktiven Dateifensters geht auf **beiden**
   Wegen durch jenen Nachzug — der Mausklick über `aktives_setzen`, der
   Tastenbefehl über `Kommando::FensterWechseln` und den Kopf von
   `kommando_ausfuehren`. Ein dritter Anlass daneben wäre eine zweite Zeile für
   dieselbe Frage.

**`bereichsleiste_nachziehen` bleibt daneben stehen und ist nicht damit
verschmolzen.** Die Leiste zeigt Schalterzustände, die Zeile zeigt Meldungen; ein
gemeinsamer Nachzug hätte zwei Anlässe in einer Funktion, und der
Meldungswechsel eines Dateifensters ginge die Leiste nichts an.

Derselbe Aufruf deckt den Start ab: `oberflaeche_aufbauen` ruft
`aufteilung_nachziehen`, bevor die Startmeldungen gesetzt werden, und die gehen
danach über `melden` → `meldung_zeigen` → `meldung_gewechselt` → Rückruf.

## Der Fokus: keine zweite Tür, kein sechster Wert

Die Zeile hängt **nicht** an `makeFirstResponder:` und braucht dort nichts:
`fokusanzeige_nachziehen` schreibt weiter ausschließlich die fünf Rahmenfarben
und den Fenstertitel, und diese Änderung fasst es nicht an. Ein zweiter
Beobachter entsteht nicht.

`Fokus` bekommt keinen sechsten Wert und die Fensterzeile keinen sechsten
Bereich. Die Zeile ist deren Schwester unter der Inhaltsfläche, genau wie die
Bereichsleiste — dasselbe Argument, das die Runde 5 geführt hat, und derselbe
Grund: `ersthelferbereich` geht die fünf Bereiche der `NSSplitView` durch, und
eine Ansicht darin wäre ein sechster Bereich oder ein blinder Fleck.

## Zum offenen Punkt aus der Runde 5 (C5.11)

Nicht beantwortet, wie beauftragt. Was am Baum und im SDK zu sehen ist:

- **`setRefusesFirstResponder` steht im ganzen Baum an genau einer Stelle**,
  `bereichsleiste.rs:478`, für die Schalter der Leiste. Die Statuszeile trägt es
  nicht, und sie ist auch kein `NSControl`, das den Rang von sich aus annähme.
- Das Feld entsteht über `labelWithString:`. Der Kopf des Systems beschreibt es
  als „a non-wrapping, non-editable, non-selectable text field"
  (`NSTextField.h:87-93`). Das ist die ganze Grundlage dafür, dass es außen
  bleibt.
- **Zur vollständigen Tastaturbedienung sagt der SDK-Kopf nichts**, und gemessen
  ist in diesem Baum nichts. Der Modulkopf von `statuszeile.rs` hält beides
  fest: die Grundlage und die Lücke. Schritt 11 setzt eine `NSScrollView`
  darum, und damit tritt die zweite Hälfte der Frage hinzu.

## Proben

19 statt 8 in `statuszeile.rs`. Die acht der Runde 1 treffen in der neuen Form
dieselben Aussagen — sie gehen über eine Hilfsfunktion `allein`, die einen
Quellensatz auf der aktiven Seite und einen leeren daneben stellt.

Elf sind neu:

| Probe | Was sie misst |
|---|---|
| `bei_gleichem_rang_gewinnt_die_aktive_seite` | erste Stelle, an der sich die Ordnung entscheidet |
| `der_hoehere_rang_der_inaktiven_seite_schlaegt_den_niedrigeren_der_aktiven` | zweite, das Paar aus C5.6 |
| `meldet_nur_die_inaktive_seite_steht_ihre_meldung_in_der_zeile` | dritte |
| `schweigen_beide_seiten_bleibt_die_zeile_leer` | vierte |
| `ueber_alle_zehn_bewerber_gewinnt_genau_eine_aussage` | zehn Bewerber, ein Gewinner, beide aktiven Seiten |
| `die_verdraengte_meldung_der_inaktiven_seite_erscheint_danach` | C5.7 über beide Dateifenster |
| `die_art_haengt_am_rang_und_nicht_an_der_seite` | alle fünf Ränge × beide Seiten × beide aktiven Seiten |
| `den_namenszusatz_traegt_genau_die_inaktive_seite` | C5.8 |
| `der_namenszusatz_gilt_auf_jedem_rang` | derselbe Zusatz auf allen fünf |
| `steht_nur_ein_dateifenster_traegt_kein_satz_einen_zusatz` | der Fall ohne eigenen Zweig |
| `der_zusatz_steht_vor_dem_unveraenderten_text` | `zeilentext` ersetzt nichts |

Dazu die Zusicherung an `MINDESTGROESSE` beim Übersetzen.

**Keine neue Probe behauptet den Hauptfaden über `MainThreadMarker::
new_unchecked`.** Die vier vorhandenen bleiben, wo sie sind.

## Was nicht angefasst ist

- `Dateifenster::vorgang_sichtbar` behält seine Bedeutung, das Feld steht, und
  **L8 ist nicht neu geschnitten**. Dass eine Vorgangsanzeige von einer
  Befehlsantwort verdeckt sein könnte, ist in der Messstrecke nicht erreichbar:
  `kommando_ausfuehren` räumt die Befehlsantworten beider Seiten vor jedem
  Befehl weg, und die Vorgangsanzeige entsteht danach.
- Keine der vier vollständigen Aufzählungen des Projekts wächst.
  `Wirkungsbereich` bleibt bei sieben, `Kommando` bei 75, `Bereich` bei fünf,
  `Fokus` bei fünf.
- Keine neue AppKit-Klasse. Die Autogröße ist von `statuszeile.rs` nach
  `fenster.rs` gewandert; der Kopf dort nennt jetzt `autoresizingMask`
  (`NSView.h:125`) und die drei gesetzten Werte (`NSView.h:33-41`), sämtlich
  ohne eigene Angabe und damit seit 10.0. Am SDK unter
  `/Applications/Xcode.app/…/MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers`
  gelesen.
- `#![deny(unsafe_code)]` steht unverändert an allen drei Kistenwurzeln; dieser
  Schritt fügt keinen `unsafe`-Block hinzu.

## Ein Befund am Rand

`crates/krk-ui/src/appkit/mod.rs:76` sagt weiterhin, `statuszeile` sei „die
Zeile an seinem Fuss" — gemeint war der Fuß eines Dateifensters. Die Datei
gehört nicht zu den fünf dieses Schrittes und ist deshalb nicht angefasst. Sie
ist beim nächsten Schritt, der sie ohnehin öffnet, nachzuziehen.

## Dateien

- `crates/krk-ui/src/appkit/statuszeile.rs` (349 → 819 Zeilen)
- `crates/krk-ui/src/appkit/tabelle.rs` (2683 → 2722 Zeilen)
- `crates/krk-ui/src/appkit/aufteilung.rs` (640 → 641 Zeilen)
- `crates/krk-ui/src/appkit/fenster.rs` (365 → 433 Zeilen)
- `crates/krk-ui/src/appkit/anwendung.rs` (5227 → 5321 Zeilen)
