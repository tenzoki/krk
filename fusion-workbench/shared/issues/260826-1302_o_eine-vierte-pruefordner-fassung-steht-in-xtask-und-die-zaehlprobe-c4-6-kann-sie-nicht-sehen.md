Eine vierte Prüfordner-Fassung steht in `xtask` und die Zählprobe C4.6 kann sie nicht sehen

---

`xtask/src/release.rs:905-931` erklärt `Wegwerfwurzel`: ein Ordner unter `std::env::temp_dir()` mit Prozesskennung und Laufnummer, der sich in `Drop` mit `remove_dir_all` abräumt. Das sind alle drei Zeichen, nach denen `genau_drei_pruefordner_fassungen_stehen_im_baum` (`crates/krk-core/tests/baum.rs:113-153`) sucht — und die Probe sieht ihn nicht, weil `gemeinsam::quelldateien()` nur `crates/` liest. `CLAUDE.md` sagt „Es gibt genau drei Fassungen, eine je Kiste, und das soll so bleiben"; der Baum trägt vier.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `xtask/src/release.rs:905-931`; `crates/krk-core/tests/gemeinsam/mod.rs:272-287` (`quelldateien`); `crates/krk-core/tests/baum.rs:113-153`, `:56-84`, `:177-208`

## Die vierte Fassung, im Klartext

```rust
// xtask/src/release.rs:905-931
/// Ein Wegwerf-Wurzelordner, wie ihn die Proben des Kerns benutzen.
struct Wegwerfwurzel { pfad: PathBuf }
impl Drop for Wegwerfwurzel {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.pfad); }
}
fn wegwerfwurzel(zweck: &str) -> Wegwerfwurzel {
    let laufnummer = ZAEHLER.fetch_add(1, …);
    let pfad = std::env::temp_dir().join(format!(
        "krk-xtask-test-{zweck}-{}-{laufnummer}", std::process::id()
    ));
    let _ = fs::remove_dir_all(&pfad);
    Wegwerfwurzel { pfad }
}
```

Ihr eigener Doc-Kommentar sagt es: „wie ihn die Proben des Kerns benutzen". Sie ist keine zufällige Ähnlichkeit, sondern eine Abschrift.

## Warum die Probe sie nicht sieht

`quelldateien()` baut ihre Wurzel so (`gemeinsam/mod.rs:273-277`):

```rust
let wurzel = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent().and_then(Path::parent)
    .expect(…)
    .join("crates");
```

`xtask/` liegt neben `crates/` und nicht darin. Neun `.rs`-Dateien sind damit für jede Zählprobe des Baums unsichtbar: `xtask/src/{beglaubigung,bundle,git,main,messen,release,sign,veroeffentlichung,version}.rs`.

Der Doc-Kommentar der Funktion ist an dieser Stelle genau (`:248-249`: „Jede `.rs`-Datei unter `crates/`"); die **Probennamen und Meldetexte** sind es nicht: `genau_drei_pruefordner_fassungen_stehen_im_baum`, „eine vierte Pruefordner-Fassung steht im Baum", „eine andere Datei als die benannten kann das atomare Schreiben erreichen", „die Liste der Ausnahmen von deny(unsafe_code) hat sich geaendert". Alle vier sprechen vom Baum und meinen `crates/`.

## Genau der Fehler, den die Probe schon einmal gemacht hat

Der Doc-Kommentar der Probe schreibt ihre eigene Vorgeschichte aus (`baum.rs:93-107`): bis zur Runde 7 suchte sie `impl Drop for Pruefordner`, band damit an den **Namen** und übersah eine vierte Fassung namens `Ordner`. Die Gegenmaßnahme war, nach dem Gegenstand statt nach dem Namen zu suchen. Sie greift — nur nicht in einem Verzeichnis, das die Suche nie betritt. Derselbe Befund, eine Ebene höher: nicht die Nadel ist zu eng, sondern der Heuhaufen.

Der Kopf der Datei nennt die Grenze seiner Nadeln ausdrücklich („Was eine Nadel nicht entscheiden kann", `baum.rs:17-29`) und zählt drei Blindheiten auf. Die vierte — der halbe Baum — steht nicht dabei.

## Was daran heute harmlos ist und was nicht

Harmlos: `xtask` führt keine Abhängigkeit (`xtask/Cargo.toml`), kann `atomar::schreiben` also nicht erreichen, und trägt kein `#![deny(unsafe_code)]`, dessen Ausnahme zu zählen wäre. Die zwei anderen Zählproben sind sachlich nicht verletzt.

Nicht harmlos: die C4.6-Zusage **ist** verletzt, und die Stelle, die sie halten soll, kann es nicht melden.

## Richtung

Zwei Fragen, und die zweite hängt an der ersten.

1. **Zählt `xtask` mit?** Wenn ja, liest `quelldateien()` künftig die Wurzel statt `crates/` — dann werden die drei anderen Zählproben mit erweitert und `Wegwerfwurzel` fällt sofort auf. Wenn nein, gehört der Umfang in jeden Probennamen und in jeden Meldetext, damit „im Baum" nicht mehr dasteht, wo `crates/` gemeint ist.
2. **Was wird aus `Wegwerfwurzel`?** Zusammenlegen geht nicht — dieselbe Kistengrenze wie bei den drei anerkannten Fassungen. Sie wäre also die vierte anerkannte, und dann wandert die Zahl aus `CLAUDE.md` und aus der Probe auf vier. Das ist eine Nutzerfrage; dieser Datensatz stellt sie nicht, er meldet nur, dass sie ansteht.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
