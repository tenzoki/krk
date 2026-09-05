# Wird ein privates Element oeffentlich, oder wird der Verweis darauf zu Fliesstext?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` (dieselbe Bauart von Frage: was der Uebersetzer nicht haelt, muss jemand entscheiden)

---

## Frage

`RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps` bricht mit Exit 101 ab.
Kein Abnahmekommando dieses Projekts faehrt `cargo doc`, deshalb hat die Anzeige nie
jemand gesehen. Nach dem Aufraeumen der drei mechanischen Arten (unaufgeloester
Verweis, ueberfluessiges Verweisziel, Name zugleich Funktion und Modul) bleiben in
`krk-core` **53 Meldungen an 30 verschiedenen privaten Elementen**, gezaehlt mit

```sh
export PATH="$HOME/.cargo/bin:$PATH"
RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps 2>&1 \
  | grep -cE 'links to private item|unresolved link to'
```

Jede dieser Meldungen sagt dasselbe: ein Modulkopf oder ein Doc-Kommentar an einem
**oeffentlichen** Element verweist mit `[`…`]` auf ein Element, das **privat** ist.
Rustdoc kann den Verweis in der oeffentlichen Dokumentation nicht aufloesen, weil das
Ziel dort nicht existiert.

Die Frage ist nicht, ob ein Tippfehler vorliegt. Die Verweise sind sachlich richtig:
die Modulkoepfe dieses Projekts erklaeren die **Mechanik**, und die Mechanik wohnt
haeufig in einer privaten Funktion. Die Frage ist, was aus dem Verweis wird.

## Zwei Formen derselben Ursache

51 Meldungen tragen den Wortlaut `public documentation for … links to private item`.
Vier weitere tragen `unresolved link to`, und zwar an `crate::operation::zippen` (2)
und `crate::operation::entpacken` (2) in `crates/krk-core/src/verzeichnis/sys.rs`. Der
Unterschied ist der Standort des Rufers und nicht die Ursache: `operation/zippen.rs`
und `operation/entpacken.rs` sind **private Module** (`mod zippen;` ohne `pub` in
`crates/krk-core/src/operation/mod.rs`). Aus derselben Kiste heraus meldet rustdoc
"privat" (`operation/auftrag.rs:54` an `super::zippen`), von aussen "gibt es nicht".
Wer nur nach dem Wortlaut sortiert, teilt dieselbe Entscheidung auf zwei Haufen.

## Die 30 betroffenen Elemente

Gezaehlt sind die **Verweisstellen**, nicht die Elemente. Ein Element mit vier Rufern
steht einmal in der Tabelle und traegt die Vier.

| Privates Element | Wohnt in | Verweisstellen |
|---|---|---|
| `Zugang::beiseite_legen` | `ablage/mod.rs` | 6 |
| `Ordnermodell::schalter_setzen` | `verzeichnis/modell.rs` | 4 |
| `zippen` (Modul) | `operation/` | 3 |
| `Ordnermodell::sichtbar` | `verzeichnis/modell.rs` | 3 |
| `Lauf` | `leseprofil/bausteine.rs` | 3 |
| `entschiedene_verneinung` | `git/leser.rs` | 3 |
| `ortsangabe_ohne_platzhalter` | `leseprofil/datei.rs` | 3 |
| `Zeilendatei::zerlegen` | `leseprofil/datei.rs` | 3 |
| `entpacken` (Modul) | `operation/` | 2 |
| `VERWALTUNGSEINTRAG` | `verzeichnis/arbeitsbaum.rs` | 2 |
| `Ordnermodell::zeilengrund_von` | `verzeichnis/modell.rs` | 2 |
| `Ordnermodell::sicht_neu_aufbauen` | `verzeichnis/modell.rs` | 1 |
| `EMFILE`, `ENFILE`, `EWOULDBLOCK` | `verzeichnis/sys.rs` | je 1 |
| `endung_ab` | `verzeichnis/eintrag.rs` | 1 |
| `Ort`, `Ort::Einer`, `Lauf::in_einem_ordner`, `daten` | `leseprofil/bausteine.rs` | je 1 |
| `typ`, `anzeige` | `leseprofil/datei.rs` | je 1 |
| `Einstellungsdatei`, `Einstellungen::aus_datei` | `ablage/einstellungen.rs` | je 1 |
| `rechte_uebernehmen` | `ablage/atomar.rs` | 1 |
| `OBJEKTSPEICHER` | `git/leser.rs` | 1 |
| `Belegung::bauen` | `tasten/belegung.rs` | 1 |
| `Auftrag::entpackziel` | `operation/auftrag.rs` | 1 |
| `umlaufen` | `text/suche.rs` | 1 |
| `tragbar` | `zwischenablage.rs` | 1 |

Die Verteilung ueber die Dateien:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps 2>&1 \
  | grep -oE 'crates/krk-core/src/[a-z/]+\.rs' | sort | uniq -c | sort -rn
```

## Optionen

1. **Jedes verwiesene Element wird oeffentlich.** `pub` an 30 Stellen, dazu die
   zwei Module `zippen` und `entpacken` als `pub mod`.
   - Pro: Jeder Verweis bleibt ein Verweis. Der Leser der Dokumentation kommt von der
     Aussage zur Mechanik, die sie erklaert. Das Tor faellt in einem Zug.
   - Contra: Die oeffentliche Flaeche von `krk-core` waechst um 30 Elemente, die
     niemand ausserhalb ruft. `Ordnermodell::sichtbar`, `Ordnermodell::schalter_setzen`
     und `Ordnermodell::zeilengrund_von` sind innere Rechenschritte eines Modells,
     dessen Zustand ueber die oeffentlichen Wege gesetzt wird; oeffentlich gemacht
     laden sie genau die Umgehung ein, die die Modulkoepfe verbieten.
     `Zugang::beiseite_legen` ist der Kern der Bestandssicherung und hat mit
     `text_laden`/`text_sichern` bereits seine oeffentliche Fassade. `EMFILE`,
     `ENFILE`, `EWOULDBLOCK` sind Zahlen aus `errno.h`, die `ist_deskriptormangel`
     kapselt. `krk-core` hat genau einen Kunden, `krk-ui`, und keiner dieser 30
     Namen wird dort gebraucht.

2. **Jeder Verweis wird zu Fliesstext.** Aus ``[`Ordnermodell::sichtbar`]`` wird
   ``` `Ordnermodell::sichtbar` ``` — Backticks bleiben, die eckigen Klammern fallen.
   - Pro: Die Aussage des Modulkopfs bleibt Wort fuer Wort erhalten; nur die
     Verlinkung faellt. Die oeffentliche Flaeche bleibt, wie sie ist. Der Handgriff
     ist mechanisch und in einem Durchgang zu machen.
   - Contra: 53 Stellen verlieren ihren Sprung. Wer den Modulkopf liest und die
     Mechanik sehen will, sucht den Namen von Hand. Nichts haelt kuenftig einen
     Tippfehler in einem dieser Namen an: aus einem Verweis, den rustdoc prueft,
     wird eine Zeichenfolge, die niemand prueft.

3. **`#![allow(rustdoc::private_intra_doc_links)]` an der Kistenwurzel.** Die Anzeige
   wird abgeschaltet, die Verweise bleiben stehen.
   - Pro: Ein einziger Handgriff, kein Verlust an Aussage, kein Zuwachs an
     oeffentlicher Flaeche. Mit `--document-private-items` loesen die Verweise sogar
     auf, und genau so liest ein Entwickler dieses Projekts die Dokumentation.
   - Contra: Die vier Meldungen an den privaten Modulen `zippen` und `entpacken`
     bleiben stehen; `private_intra_doc_links` deckt sie nicht, weil rustdoc sie als
     `unresolved link` fuehrt. Fuer die vier braucht es zusaetzlich Option 1 oder 2.
     Und ein `allow` an der Wurzel deckt kuenftig auch die Verweise, die wirklich
     falsch sind.

4. **Gemischt nach Absicht.** Oeffentlich wird, was ein Kunde von `krk-core`
   sinnvoll ruefen koennte; Fliesstext wird der Rest. Die vier Modulverweise gehen
   nach Option 2.
   - Pro: Trifft in jedem Fall die richtige Antwort.
   - Contra: 30 Einzelentscheidungen statt einer. Und die Antwort ist bei fast allen
     dieselbe, naemlich "kein Kunde ruft das": `krk-ui` ruft heute keinen der 30.

## Randbedingungen

- **`krk-core` hat genau einen Kunden.** Das Binaerziel `krk` in `krk-ui`. Es gibt
  keine dritte Kiste, die gegen diese Flaeche baut, und `krk-ui` hat kein
  Bibliotheksziel. Die Frage "was gehoert in die oeffentliche Flaeche" ist damit
  keine Frage nach fremden Nutzern, sondern eine nach der inneren Grenze.
- **`#![deny(unsafe_code)]` ist die einzige Grenze, die der Bau heute erzwingt.**
  Eine Sichtbarkeitsgrenze erzwingt er nicht mehr, sobald sie `pub` heisst.
- **Kein Abnahmekommando faehrt `cargo doc`.** Ob es eines tun soll, gehoert zu
  dieser Frage: eine Antwort, die das Tor gruen macht, aber niemand faehrt, haelt
  nichts. `make check` faehrt heute `build`, `test`, `clippy` und `fmt`.
- Die drei mechanischen Arten sind bereits geraeumt (Sitzung 260905-2336). Die
  Zahlen oben messen den Baum **nach** dieser Raeumung.

## Empfehlung

Option 3 fuer die 49 privaten Elemente, Option 2 fuer die vier Modulverweise, und
`cargo doc` als fuenftes Kommando in `make check`.

Begruendung: Die Modulkoepfe dieses Projekts sind fuer den Entwickler geschrieben,
nicht fuer einen fremden Kunden — sie zitieren Defektdatensaetze, Rundennummern und
Planschritte. Wer sie liest, liest sie mit `--document-private-items` oder im
Quelltext. Ein Verweis, der dort aufloest, ist genau richtig. Option 1 gibt dafuer
die Sichtbarkeitsgrenze auf, die die Modulkoepfe an mehreren Stellen ausdruecklich
verteidigen; Option 2 gibt die Pruefung des Namens auf, die heute rustdoc leistet.

Der Preis von Option 3 ist benannt und begrenzt: ein `allow` an der Wurzel deckt
kuenftig auch einen falschen privaten Verweis. Er deckt **nicht** den unaufgeloesten
Verweis, das ueberfluessige Ziel und den mehrdeutigen Namen — die drei Arten, die
diese Sitzung geraeumt hat, bleiben scharf. Und mit `cargo doc` in `make check`
bleiben sie scharf gemessen statt behauptet.
