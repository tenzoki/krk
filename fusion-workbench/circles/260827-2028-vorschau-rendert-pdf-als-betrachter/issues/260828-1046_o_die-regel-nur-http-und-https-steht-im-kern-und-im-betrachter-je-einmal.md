Die Regel „nur http und https" steht im Kern und im Betrachter je einmal

---

Die Grenze aus C9 der Runde 1, dass KRK allein `http:` und `https:` an das System gibt, steht seit der Runde 20 an zwei Stellen mit zwei Fassungen:

- `crates/krk-core/src/zwischenablage.rs:65`: `ohne_schema(text, "http").is_some() || ohne_schema(text, "https").is_some()` entscheidet für den Sprung aus der Zwischenablage (`Ziel::Web`).
- `crates/krk-ui/src/appkit/betrachter.rs:638-640`, `ist_webschema`: `schema.eq_ignore_ascii_case("http") || schema.eq_ignore_ascii_case("https")` entscheidet für den Klick auf einen Verweis im PDF (`Verweisdelegierter::verweis_geklickt`, `:293-301`).

Beide Rufer enden in `zwischenablage::im_browser_oeffnen` (`crates/krk-ui/src/appkit/zwischenablage.rs:285-290`), dessen Doc-Kommentar (`:281-284`) sagt: „Nur `http:` und `https:` erreichen diesen Aufruf. Die Grenze zieht die Deutung im Kern". Das gilt seit `5ff1ee4` nur noch für einen der zwei Rufer; der zweite zieht sie selbst, in `krk-ui`. Wer C9 einmal lockert oder ein Schema hinzunimmt, ändert eine Stelle und übersieht die andere, und der Kommentar an der Hülle sagt ihm nicht, dass es zwei sind.

Der Plan (`planning/260828-0712_*`, Schritt 6) sah `im_browser_oeffnen(url.absoluteString())` ohne Filter vor; die Coder haben die Grenze mit Verweis auf C9 hinzugefügt (`betrachter.rs`, Modulkopf „Verweise"). Die Entscheidung ist richtig, ihre Form ist die zweite Abschrift.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Betroffen:** `crates/krk-core/src/zwischenablage.rs`, `crates/krk-ui/src/appkit/betrachter.rs`, `crates/krk-ui/src/appkit/zwischenablage.rs` (Doc-Kommentar)
**Schwere:** Low (heute stimmen beide Fassungen überein; der Defekt ist die Doppelung, nicht ein Verhalten)

Fix: `pub fn ist_webschema(schema: &str) -> bool` in `krk_core::zwischenablage`, mit der Begründung aus C9 am Kommentar; `deuten` dort ruft es für den Schemavergleich, `betrachter::ist_webadresse` ruft es über `NSURL::scheme`, und die private Fassung samt ihrer Probe `allein_http_und_https_sind_webschemata` wandert in den Kern (`crates/krk-core/tests/zwischenablage.rs` oder das Prüfmodul der Datei). Der Doc-Kommentar an `im_browser_oeffnen` nennt danach beide Rufer und die eine Regel.
