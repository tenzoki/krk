`Geladen<T>` traegt kein `#[must_use]`, und vier der fuenf Ladewege koennen ihre `Ersetzung` still fallen lassen

---

`Geladen<T>` (`crates/krk-core/src/ablage/mod.rs:474-480`) traegt neben dem Wert das Feld
`ersetzung`, und dieses Feld ist die **einzige** Auskunft darueber, dass eine Ablagedatei
beschaedigt war, zur Seite gelegt wurde und durch den Auslieferungszustand ersetzt ist. Weder
der Typ noch einer der vier Ladewege, die ihn liefern, traegt `#[must_use]`. Wer den
Rueckgabewert fallen laesst, verliert diese Meldung ohne Warnung des Uebersetzers, und der
Nutzer erfaehrt nie, dass sein Bestand ersetzt worden ist.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Affected:** `crates/krk-core/src/ablage/mod.rs:474` (`Geladen`), `:617` (`Zugang::laden`),
`:740` (`Zugang::text_laden`), `:493` (`Geladen::mit_meldung`),
`crates/krk-core/src/ablage/einstellungen.rs:149` (`laden`),
`crates/krk-core/src/tasten/belegung.rs:1492` (`laden`)
**Tree state:** `004ff72`
**Domain:** code

## Die Regel und der Nachbar, an dem die Auslassung sichtbar wird

CLAUDE.md fuehrt die Regel als bindend: „Ein Rueckgabewert, dessen stilles Fallenlassen
unbemerkt bliebe, bekommt in diesem Projekt `#[must_use]`", entschieden vom Nutzer am
260811-2140.

Der Nachbar steht in derselben Ebene und in derselben Aufgabe:

```rust
// crates/krk-core/src/ablage/leseprofile.rs:100-103   MIT
#[must_use = "die zweite Haelfte des Paares sind die Meldungen ueber abgewiesene \
              Profile und Zeilen; wer sie fallen laesst, verschweigt dem Nutzer, \
              warum ein Profil seiner readers.toml nicht greift"]
pub fn laden(zugang: &Zugang<'_>) -> (Geladen<Profile>, Vec<String>) {

// crates/krk-core/src/ablage/einstellungen.rs:149     OHNE
pub fn laden(zugang: &Zugang<'_>) -> Geladen<Einstellungen> {
```

Beide sind der Ladeweg einer von Hand gepflegten Ablagedatei, beide liefern eine `Ersetzung`,
beide werden vom selben Rufer in `krk-ui` gebraucht. Die eine schuetzt ihre Meldung, die andere
nicht. Die Auslassung ist damit nicht als Abwaegung zu lesen.

## Warum das Fallenlassen unbemerkt bliebe

`Zugang::laden` **schreibt**: es legt eine beschaedigte Datei ueber
`Zugang::beiseite_legen` unter `atomar::beiseitepfad` daneben
(`crates/krk-core/src/ablage/mod.rs:672`). Der Rueckgabewert ist danach die einzige Stelle, an
der noch steht, dass das geschehen ist — auf der Platte liegt eine `.beschaedigt`-Datei, von
der niemand mehr spricht, und der Wert in der Hand des Rufers ist der Auslieferungszustand,
der von einem echten Bestand nicht zu unterscheiden ist. Genau diese Ununterscheidbarkeit ist
der Verlust, gegen den die Runde 6 gebaut hat
(`shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`).

Der Uebersetzer haelt heute nichts davon auf: `zugang.laden::<Sitzung>(Datei::Sitzung);` als
Anweisung ohne Bindung baut grün.

## Die eine Stelle, an der die Behebung steht

`#[must_use]` gehoert an den **Typ** `Geladen<T>` und nicht an die fuenf Funktionen. Ein
Vermerk am Typ deckt `Zugang::laden`, `Zugang::text_laden`, `einstellungen::laden`,
`belegung::laden` und die erste Haelfte des Paares aus `leseprofile::laden` auf einmal ab; fuenf
Vermerke waeren fuenf Stellen, an denen der sechste Ladeweg vergessen werden kann.
`Geladen::mit_meldung` (`:493`) liefert `(T, Option<String>)` und traegt den Satz fuer die
Statuszeile in seiner zweiten Haelfte; es braucht einen eigenen Vermerk, denn das Paar ist kein
`Geladen` mehr.

**Verwandt, aber nicht dasselbe:**
`shared/issues/260826-1221_*_must-use-traegt-sieben-praedikate-des-verzeichnisbaums-und-zwanzig-gleichartige-daneben-nicht.md`
zaehlt dieselbe Regel unter `verzeichnis/` an reinen Praedikaten ab,
`shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-und-des-stapelumbenennens.md`
unter `operation/` und `stapelumbenennen/`, und
`shared/issues/260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-waehrend-die-kiste-daneben-66-traegt.md`
unter `tasten/` und `text/`. Dort geht jeweils eine gerechnete Antwort verloren, hier eine
Meldung an den Nutzer ueber einen bereits geschriebenen Datenverlust. **Die vier Datensaetze
sind vier unabhaengige Befunde derselben Sitzung an vier Modulgruppen und zusammen zu
raeumen**; nur dieser hier ist mit einem einzigen Vermerk am Typ erledigt.

**Gefunden:** coderev, Vollbaum-Durchsicht von `crates/krk-core/src/{ablage,leseprofil}/` am
260826-1225.
