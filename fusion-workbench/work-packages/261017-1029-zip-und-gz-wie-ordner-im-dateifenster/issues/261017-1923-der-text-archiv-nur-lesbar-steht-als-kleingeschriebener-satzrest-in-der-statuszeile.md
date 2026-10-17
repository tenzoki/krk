Der Text „Archiv nur lesbar“ steht als kleingeschriebener Satzrest in der Statuszeile
---
`Text::ArchivNurLesbar` ist als Grund für die Abschlussliste geschrieben: klein, ohne Punkt („dieses Archiv ist allein lesbar“). Die Oberfläche zeigt denselben Schlüssel an drei Stellen allein in der Statuszeile. Dort beginnen die übrigen Sätze groß und enden mit Punkt, etwa `ImArchivNichtVerfuegbar`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Darstellung)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege**

- `crates/krk-core/src/sprache/tabelle/de.rs`: `Text::ArchivNurLesbar => "dieses Archiv ist allein lesbar"`. In `fr.rs` und `en.rs` steht er ebenso klein.
- `crates/krk-ui/src/appkit/anwendung.rs`, `uebertragen`: `self.antwort_zeigen(aktiv, text(Text::ArchivNurLesbar))`. Zwei weitere Stellen derselben Datei zeigen ihn ebenso.

**Behebung (Richtung)**

Ein eigener Schlüssel für die Statuszeile, in allen drei Tabellen, als ganzer Satz. `ArchivNurLesbar` bleibt der Grund im Bericht.

**Abnahme**

`grep -n 'antwort_zeigen(.*ArchivNurLesbar' crates/krk-ui/src` findet nichts mehr. Der neue Schlüssel steht in `de.rs`, `fr.rs` und `en.rs`.
