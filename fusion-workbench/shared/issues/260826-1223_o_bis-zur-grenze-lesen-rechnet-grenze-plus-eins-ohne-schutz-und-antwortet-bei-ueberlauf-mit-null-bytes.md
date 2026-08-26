`bis_zur_grenze_lesen` rechnet `grenze + 1` ohne Schutz und antwortet beim Überlauf mit null Bytes

---

Die Grenze ist ein `u64` des Aufrufers, und die Schranke gegen die wachsende Datei rechnet ungeschützt `grenze + 1`. Bei `u64::MAX` bricht das im Profil `debug` ab und läuft im Profil `release` auf `take(0)` über: die Hülle liefert `Ok(Vec::new())`, also eine leere Datei, wo der Vertrag die Bytes zusagt. Kein heutiger Aufrufer erreicht den Fall.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Am Baum

`crates/krk-core/src/text/datei.rs:619-647`. Die Signatur nimmt jede Zahl:

```rust
pub fn bis_zur_grenze_lesen(pfad: &Path, grenze: u64) -> Result<Vec<u8>, Lesehindernis>
```

und die Schranke, deren Zweck der Doc-Kommentar über neun Zeilen ausschreibt (`datei.rs:610-618`), rechnet in Zeile 640:

```rust
datei.take(grenze + 1).read_to_end(&mut bytes)
```

Beide Prüfungen davor sind auf `grenze` ausgelegt und laufen sauber; allein diese Addition kann überlaufen. Im Freigabeprofil wird daraus `take(0)`, `read_to_end` liest nichts, `bytes.len() as u64 > grenze` ist falsch, und die Funktion antwortet `Ok(vec![])`. Das ist die stille falsche Antwort, gegen die derselbe Doc-Kommentar argumentiert: „Ohne diese Schranke waere ‚es wird nie mehr als die Grenze gelesen' eine Vorhersage aus einer alten Auskunft, mit ihr ist es eine Eigenschaft der Bauart."

## Nicht erreichbar, und trotzdem gemeldet

Die drei Aufrufer bringen kleine Konstanten mit: `krk-ui/src/vorschaumodell.rs:717` (`BILDGRENZE`, 64 MB), `:728` (`TEXTGRENZE`, 1 MB) und `krk-core/src/verzeichnis/inhalt.rs:134` (die von der Vorschau geerbte Zahl). Keiner kann den Fall auslösen. Der Befund steht, weil die Hülle öffentlich ist, ihr Doc-Kommentar sie ausdrücklich als die Fassung beschreibt, bei der „die Grenze als Argument reist" und jeder Aufrufer „seine eigene Zahl mitbringt" — die Einladung, eine neue Zahl mitzubringen, steht also im Vertrag.

`lesen` ist nicht betroffen: dort ist der Summand die Konstante `EDITORGRENZE` (`datei.rs:464`). `anlesen` rechnet gar nicht (`datei.rs:712`).

## Vorschlag

`grenze.saturating_add(1)`. Eine Zeile, kein Verhaltensunterschied für jeden heutigen Aufruf.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.
