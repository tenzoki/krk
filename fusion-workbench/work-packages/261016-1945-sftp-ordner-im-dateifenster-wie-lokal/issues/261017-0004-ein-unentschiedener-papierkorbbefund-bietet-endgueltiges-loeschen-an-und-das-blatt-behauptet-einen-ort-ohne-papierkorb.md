Ein unentschiedener Papierkorbbefund bietet endgültiges Löschen an, und das Blatt behauptet einen Ort ohne Papierkorb
---
Vor dem Paket endete `Nein` wie `Unentschieden` in einer Meldung, ein falscher Befund war also harmlos. Jetzt führen beide zur endgültigen Rückfrage. Das Blatt sagt dabei fest „Dieser Ort hat keinen Papierkorb“, auch wenn KRK es nur nicht feststellen konnte, etwa wenn `canonicalize` scheitert oder ein Pfad kein UTF-8 ist. Ein Nutzer auf einem Volume mit Papierkorb bekommt dann ein endgültiges Löschen angeboten, und die Begründung dafür ist unwahr.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (unumkehrbarer Weg auf einen unbelegten Befund hin; die Rückfrage mit „Abbrechen“ als Vorgabe bleibt der Schutz)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`, `261016-1948-darf-krk-an-einem-ort-ohne-papierkorb-nach-rueckfrage-endgueltig-loeschen.md`

**Belege**

- `crates/krk-ui/src/kommandos/loeschwarnung.rs`, `vor_der_rueckfrage`, Zweig `(false, false)`: `if papierkorb().erlaubt() { Vorstufe::Rueckfrage } else { // Kein Papierkorb, oder keine Auskunft darueber … Vorstufe::EndgueltigeRueckfrage }`.
- Quellen von `Unentschieden`: `canonicalize` scheitert (`appkit/anwendung.rs`, Löschweg), oder der Pfad ist kein UTF-8 (`appkit/papierkorb.rs`).
- Der Text der Rückfrage steht in `crates/krk-core/src/sprache/tabelle/de.rs` (Schlüssel der endgültigen Rückfrage) und sagt ohne Einschränkung, der Ort habe keinen Papierkorb.
- Veraltete Doc-Kommentare in `appkit/papierkorb.rs` („es wird nicht geloescht“, „der Aufrufer loescht auch dann nicht“).
- Der Plan stellt in E12 `Unentschieden` neben `Nein`. Der Entscheid `261016-1948-…` spricht nur von einem „Ort ohne Papierkorb“. Die Ausweitung auf „unbekannt“ ist vom Nutzer nicht beantwortet.

**Behebung (Richtung)**

Eine von zwei Lösungen, und die Wahl gehört dem Nutzer:
1. `Unentschieden` hält wie vor dem Paket mit einer Meldung an. Nur `Nein` führt zur endgültigen Rückfrage.
2. `Unentschieden` bekommt eine eigene Überschrift in allen drei Sprachen: „KRK konnte nicht feststellen, ob dieser Ort einen Papierkorb hat …“.

Die Doc-Kommentare in `papierkorb.rs` werden in beiden Fällen nachgezogen.

**Abnahme**

Die Tafel von `vor_der_rueckfrage` führt `Unentschieden` als eigene Zeile mit der gewählten Antwort. Bei Lösung 2 prüft eine Probe den eigenen Text in `de`, `fr` und `en`.

---

Resolved: Moeglichkeit 1 (vorsichtige Wahl, selbstaendig ausgefuehrt). `crates/krk-ui/src/kommandos/loeschwarnung.rs`: neue `Vorstufe::PapierkorbUnbekannt`, `vor_der_rueckfrage` schreibt die drei Befunde aus (Ja: Rueckfrage, Nein: endgueltige Rueckfrage, Unentschieden: Meldung), neue `papierkorb_unbekannt` mit Schluessel `LoeschenPapierkorbUnbekannt` in allen drei Tabellen; Tafel `die_tafel_aus_zwoelf_faellen_geht_auf` nachgezogen, Probe umbenannt in `allein_ein_sicheres_nein_fuehrt_zur_endgueltigen_rueckfrage`, dazu `die_meldung_zum_unbekannten_papierkorb_behauptet_keinen_ort_ohne_papierkorb`. Zweig in `crates/krk-ui/src/appkit/anwendung.rs`, `loeschen_nach_rueckfrage`, samt Skizze im Doc-Kommentar. Doc-Kommentare in `crates/krk-ui/src/appkit/papierkorb.rs` und an `Erlaubnisbefund` in `crates/krk-core/src/verzeichnis/loeschzielbefund.rs` nachgezogen; `HowTo.md`, Absatz zum Ort ohne Papierkorb.
