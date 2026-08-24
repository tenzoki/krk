C3.14 nennt seinen eigenen Nachweis, und nichts im Baum führt ihn

---

Das Abnahmekriterium C3.14 sagt: „Ein zweiter Leseweg entsteht nicht; nachzuweisen daran, dass
keine neue Stelle im Baum eine Datei über ihren Pfad statt über den Deskriptor öffnet."
Der Nachweis ist im Baum nirgends gebaut. Keine Probe zählt die Rufer von
`verzeichnis::sys::ohne_warten_oeffnen`, und keine verbietet `File::open` in den neuen Modulen.

---

**Die Sache stimmt heute, gemessen am 260824-1852.** `crates/krk-core/src/leseprofil/*.rs` und
`crates/krk-core/src/ablage/leseprofile.rs` enthalten kein `File::open`, kein `fs::read`, kein
`read_to_string` und kein `OpenOptions`; gelesen wird über `crate::text::datei::anlesen`
(`bausteine.rs:497`) und `crate::verzeichnis::leser::lesen_hoechstens` (`bausteine.rs:258`).
Die Tür hat drei Rufer, alle in `crates/krk-core/src/text/datei.rs` (`:434`, `:620`, `:692`).

**Ungehalten ist die Zusage für die Zukunft.** Wer morgen einen fünften Baustein schreibt und
darin `std::fs::read` ruft, bekommt einen grünen Bau, eine grüne Probenreihe und ein gebrochenes
C3.14. Das ist dieselbe Lage, die der Plan in seiner Tabelle `## Was der Übersetzer einfordert,
und was er nicht einfordert` für die Grenzen aus C6 benannt und mit Schritt 12 abgestellt hat;
für C3.14 ist sie stehengeblieben, weil das Kriterium in jener Tabelle nicht vorkommt.

**Der Baum kennt die Bauart schon.** `krk-core/tests/baum.rs::nur_benannte_dateien_erreichen_das_atomare_schreiben`
hält für das atomare Schreiben genau diese Frage, mit einer ausgeschriebenen Dateiliste, und
diese Runde hat ihre siebte Zeile bekommen. Eine zweite Fassung daneben für den Leseweg wäre
dieselbe Bauart und keine neue.

**Abstellen:** eine Probe in `crates/krk-core/tests/baum.rs`, die die Dateien aufzählt, die
`File::open` oder `fs::read` erreichen dürfen. Arbeit für den `coder`.

Gefunden beim Abgleich zum Abschluss der Runde 16, 260824-1852.
