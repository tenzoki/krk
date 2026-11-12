Die Regel, welches Zeilenende ein Text trägt, steht an drei Stellen, und keine ruft die andere
---
Dieselbe Regel (`\r\n`, wenn das erste `\n` hinter einem `\r` steht, sonst `\n`) steht als private Funktion in `crates/krk-core/src/ablage/einstellungen.rs` (`zeilenende`), in anderer Bauform in `crates/krk-core/src/trenntext/handlung.rs` (`zeilenende_des_textes`, über den Satzindex) und seit der Behebung des CRLF-Defekts in `crates/krk-ui/src/appkit/editor.rs` (`zeilenende_des_ausschnitts`). Wer die Regel ändert, ändert sie an einer Stelle und lässt zwei stehen.
---
**Filed by:** orchestrator, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md, 261112-0826-zwei-offene-defekte-nach-den-protokollknoepfen-beheben.md

Beleg: der Doc-Kommentar von `zeilenende_des_ausschnitts` in `crates/krk-ui/src/appkit/editor.rs` nennt die zwei anderen Stellen selbst. Die dritte ist entstanden, weil die erste privat ist und außerhalb der Dateien lag, die die Behebung anfassen durfte.

Abnahme: die Regel steht als eine öffentliche reine Funktion im Kern unter `crates/krk-core/src/text/`, mit Probe, und `einstellungen.rs` und `editor.rs` rufen sie; `handlung.rs` ruft sie oder sagt in seinem Doc-Kommentar, warum der Satzindex eine eigene Fassung verlangt.

Resolved: Die Regel steht einmal, als `krk_core::text::zeilenende::des_textes` in der neuen Datei `crates/krk-core/src/text/zeilenende.rs`, öffentlich, rein und mit zwei Proben im Prüfmodul der Datei. `angehaengt` in `crates/krk-core/src/ablage/einstellungen.rs` und `protokoll_umgeformt` in `crates/krk-ui/src/appkit/editor.rs` rufen sie; die private `zeilenende` und `zeilenende_des_ausschnitts` sind gefallen. `zeilenende_des_textes` in `crates/krk-core/src/trenntext/handlung.rs` ruft sie nicht und bleibt im Verhalten unverändert: der Satzindex antwortet an drei Arten Text anders als das erste `\n` (ein Umbruch in einer angeführten Zelle des ersten Satzes, das Ende einer `sep=`-Zeile im Vorspann, ein erster Satz ohne Zeilenende am Textende). Der Doc-Kommentar nennt die drei, und die Probe `der_satzindex_antwortet_anders_als_das_erste_lf_des_textes` im neuen Prüfmodul jener Datei hält sie. Das Verhalten aller drei Rufer ist unverändert, keine vorhandene Probe geändert.
