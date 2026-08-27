`erkennung.rs` sagt, `None` heiße „die heutige Metadatenanzeige", und das ist seit der Runde 19 der Rückfallzweig

---

Der Modulkopf und der Doc-Kommentar von `erkennen` in `crates/krk-core/src/leseprofil/erkennung.rs` sagen, ein leeres Ergebnis führe zur heutigen Metadatenanzeige (C2.5 der Runde 16). Seit `bf3a91d` tritt auf `None` der Rückfallzweig in `zusammenfassen_gezaehlt` ein: das Default-Profil für ein Verzeichnis, die sechs Angaben allein nur noch für eine Verknüpfung. Die zwei Nachbarmodule sind nachgezogen, dieses nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Low
**Domain:** code
**Tree state:** `d444879`
**Affected:** `crates/krk-core/src/leseprofil/erkennung.rs:24`, `:94-96`
**Cross-references:** `crates/krk-core/src/leseprofil/bausteine.rs:11-17` und `:300-305` (der nachgezogene Ablauf und der Rückfallzweig); `crates/krk-core/src/leseprofil/mod.rs:22-35` (das nachgezogene Ablaufbild); `planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` (Schritt 3, „Der Modulkopf von `leseprofil` zieht sein Ablaufbild nach"); `reviews/260827-1911-coderev-durchsicht-runde-19-default-profil-zaehlzeilen.md` (F2)

## Der Befund

Zwei Stellen in `erkennung.rs`:

1. `:24`, Ablaufbild im Modulkopf:
   ```text
   sonst:        die heutige Metadatenanzeige
   ```
2. `:94-96`, Doc-Kommentar von `erkennen`: „`None` als Rückgabe heißt: kein Profil greift, und die Vorschau zeigt die heutige Metadatenanzeige (C2.5). Das ist derselbe Zweig, den sie ohne diese Runde ohnehin genommen hätte."

Was `erkennen` selbst tut, ist unverändert und richtig beschrieben: zwei Durchgänge, `None`, wenn keines trifft. Falsch ist, was der Kommentar über den **Rufer** sagt. Seit `bf3a91d` verzweigt `zusammenfassen_gezaehlt` auf `None` weiter (`bausteine.rs:300-305`): ist der ausgewählte Eintrag selbst ein Verzeichnis, liefert es `Auskunft::Default` mit den drei Zählzeilen; ist er eine Verknüpfung, `None`. Die Metadatenanzeige „wie sie war" gibt es für einen Ordner ohne Treffer nicht mehr.

Derselbe Satz stand vor der Runde an vier Stellen (`bausteine.rs`, `mod.rs`, `erkennung.rs`, `resources/default-readers.toml`). Schritt 3 des Plans hat die ersten zwei nachgezogen; `erkennung.rs` stand in keiner `Files:`-Zeile des Plans und ist deshalb übergangen worden. Die vierte Stelle ist `issues/260827-1911_*_drei-saetze-im-kommentarteil-der-auslieferungsfassung-beschreiben-den-stand-vor-der-runde-19.md`.

## Was zu tun ist

Executor `coder`, allein Kommentarzeilen. Die Zeile im Ablaufbild auf den Rückfallzweig zeigen lassen (etwa „sonst: der Rückfallzweig in `bausteine::zusammenfassen_gezaehlt`, siehe dort") und den Absatz am Doc-Kommentar von `erkennen` so fassen, dass er allein sagt, was diese Funktion liefert, und für die Folge auf `bausteine.rs` verweist statt sie zu wiederholen — sonst wird derselbe Satz beim nächsten Rundfallzweig ein drittes Mal falsch. Der `#[must_use]`-Text an `erkennen` (`:97-98`) bleibt richtig und wird nicht angefasst.

## Schließbedingung

`grep -n 'heutige Metadatenanzeige' crates/krk-core/src/leseprofil/erkennung.rs` liefert nichts, `cargo doc`-Verweise lösen auf, und `make check` ist grün.
