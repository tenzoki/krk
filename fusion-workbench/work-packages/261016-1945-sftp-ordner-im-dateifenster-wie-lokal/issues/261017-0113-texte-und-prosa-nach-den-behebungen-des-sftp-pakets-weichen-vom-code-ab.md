Texte und Prosa nach den Behebungen des sftp-Pakets weichen vom Code ab
---
Die Behebungen `545c8d31` und `a97e4fbb` haben die Regel geändert, aber nicht jeden Text dazu. Die Meldung zu einem ungültigen Rechner nennt die alte Regel. Eine Meldung „nach dem Abbruch“ erscheint auch ohne Abbruch. Eine Methode hat keinen Rufer mehr, und ihr `must_use`-Text ist falsch. In `HowTo.md` ist ein Satzbezug verrutscht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Cross-references:** `261017-0113-reviewer-nachdurchsicht-sftp-befunde-1e114402-bis-a97e4fbb.md`

**Belege**

1. `crates/krk-core/src/sprache/tabelle/de.rs:77`, ebenso `fr.rs:81` und `en.rs:73`, `Text::FernRechnerUngueltig`: „kein Leerraum, kein „@“ und kein „-“ am Anfang“. `fern::adresse::rechnername_gueltig` weist seit `545c8d31` auch `,`, `*`, `?`, `!`, Klammern und jeden Buchstaben außerhalb von ASCII ab. Wer `bücher.example` oder `a.example,*` einträgt, liest einen Grund, der auf seine Eingabe nicht zutrifft.
2. `crates/krk-core/src/operation/fern.rs:841` `rest_wegraeumen` meldet mit `Text::VorgangNachAbbruchNichtWeggeraeumt` („nach dem Abbruch nicht weggeräumt“). Gerufen wird es auch bei einem unvollständigen Ersatz und bei `Tausch::ZielBlieb`, also ohne Abbruch.
3. `crates/krk-core/src/verzeichnis/loeschzielbefund.rs:377` `Erlaubnisbefund::erlaubt` hat seit `545c8d31` außer seiner eigenen Probe keinen Rufer (`grep -rn '\.erlaubt()' crates`). Sein `must_use`-Text sagt: „fallengelassen loescht der Aufrufer ohne Rueckweg“.
4. `HowTo.md:1994`: „„Teilen“ steht dort gar nicht im Kontextmenü. Dasselbe sagen `cmd+c` und `cmd+x` …“. „Dasselbe“ bezieht sich jetzt auf den Satz über Teilen statt auf die Meldung davor. Die Zeile ist außerdem nicht umbrochen.

**Behebung (Richtung)**

1. `FernRechnerUngueltig` in allen drei Tabellen auf die Regel bringen: erlaubt sind Buchstaben ohne Umlaut, Ziffern, `.`, `-`, `_` und IPv6 in Klammern.
2. `rest_wegraeumen` mit einem eigenen Text melden, etwa „der Ersatz unter „{name}“ ließ sich nicht wegräumen: {grund}“.
3. `erlaubt` entfernen, oder seinen Doc- und `must_use`-Text auf seine heutige Rolle bringen.
4. Den Satz über Teilen hinter den Satz über `cmd+c`, `cmd+x` und den Doppelklick stellen und umbrechen.

**Abnahme**

- `grep -n 'kein Leerraum' crates/krk-core/src/sprache/tabelle/de.rs` findet `FernRechnerUngueltig` nicht mehr.
- `rest_wegraeumen` nutzt nicht mehr den Schlüssel `VorgangNachAbbruchNichtWeggeraeumt`.
- `grep -rn 'fn erlaubt' crates` findet nichts, oder der Text nennt keinen Aufrufer, der löscht.
- In `HowTo.md` folgt „Dasselbe sagen“ unmittelbar auf die Meldung „Das geht an einem Ort auf einem Server nicht.“

---
Resolved: (1) `Text::FernRechnerUngueltig` in `crates/krk-core/src/sprache/tabelle/de.rs`, `fr.rs`, `en.rs` nennt die erlaubten Zeichen und die IP-Adresse; Probe `die_meldung_zu_einem_ungueltigen_rechner_nennt_die_erlaubten_zeichen` in `crates/krk-core/tests/fern.rs`. (2) `rest_wegraeumen` in `crates/krk-core/src/operation/fern.rs` meldet mit dem neuen `Text::VorgangErsatzNichtWeggeraeumt`; `halbes_ziel_wegraeumen`, das ebenso ohne Abbruch gerufen wird, mit dem neuen `Text::VorgangHalbeDateiNichtWeggeraeumt`. (3) `Erlaubnisbefund::erlaubt` samt seiner einzigen Probe `die_erlaubnis_traegt_nur_das_ja` aus `crates/krk-core/src/verzeichnis/loeschzielbefund.rs` entfernt, Modulkopf und Doc des Typs nachgezogen, der Verweis im Modulkopf von `crates/krk-ui/src/kommandos/loeschwarnung.rs` („Warum die Frage nach dem Papierkorb alle drei Befunde ausschreibt“) als Text statt Link. (4) `HowTo.md`, sftp-Abschnitt „Das Kontextmenü graut nichts aus“: „Dasselbe sagen“ folgt unmittelbar auf die Meldung, der Satz über „Teilen“ steht am Ende, umbrochen.
