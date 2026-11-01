# Die AppKit-Proben laufen auf dem Hauptfaden, und make check wird wieder grün

## Directive

Wortlaut des Nutzers (261101), Antwort „2“ auf die Frage nach dem Abbruch von `make check`:

> 2. Erst ein neues Paket „AppKit-Proben auf den Hauptfaden“ anlegen und die Frage 260810-1044 wieder aufnehmen. Das Tabellenpaket bleibt bis dahin offen.

Erreicht ist das Ziel, wenn `make check` auf dem Mac des Nutzers grün durchläuft, mit den Proben unter `appkit::`, und keine dieser Proben den Hauptfaden mehr nur behauptet.

## Anlass

`make check` bricht auf dem Mac des Nutzers mit `fatal runtime error: Rust cannot catch foreign exceptions, aborting` (SIGABRT) ab. Erhoben am 261101 in der Sitzung des Tabellenpakets `261031-1645-markdown-vorschau-richtet-tabellenspalten-aus`:

- 25 Proben unter `appkit::editor::tests::` werfen `NSInternalInconsistencyException` („-[_NSViewGeometryInWindowConcreteObservation invalidate] must be called on main thread only“), ausgelöst über `Eintragsansicht::bauen` → `spalten_einrichten` → `NSTableView setHeaderView:` → `NSScrollView tile` (`crates/krk-ui/src/appkit/eintragsansicht.rs`, um Zeile 2241 bis 2249).
- Die Proben laufen auf einem Faden von libtest und behaupten den Hauptfaden über `MainThreadMarker::new_unchecked` in `an_einer_flaeche` (`crates/krk-ui/src/appkit/editor.rs`, um Zeile 8720).
- Die Prüfung hängt an der SDK-Fassung, gegen die gelinkt ist: dieselbe Binärdatei, mit `vtool` auf SDK 15 umgeschrieben, läuft grün; mit SDK 26.0 bricht sie ab. Gebaut wird gegen SDK 27.0 auf macOS 26.6.2.
- Erster schlechter Commit nach `git bisect`: `d356ca61` (notes.txt als Tabelle, 260926), der `spalten_einrichten` einführte.
- `appkit::editor::tests::die_flaeche_der_quicknote_ist_reiner_text_mit_eigenem_verwalter` hängt, vermutlich aus demselben Grund.
- Davon getrennt schlagen `die_sammeltuer_ist_eine_sicht_auf_dieselben_bits` und `die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` fehl, weil auf diesem Gerät global `NSAutomaticQuoteSubstitutionEnabled`, `NSAutomaticDashSubstitutionEnabled` und `NSAutomaticSpellingCorrectionEnabled` auf 0 stehen; die Proben setzen die Werkseinstellung voraus.
- Ein Versuch, `spalten_einrichten` vor `setDocumentView` zu ziehen, macht aus dem Abbruch ein Hängen und ist keine Behebung.

Im ausgelieferten Bündel laufen dieselben Aufrufe auf dem echten Hauptfaden; ein Defekt für den Nutzer ist nicht belegt.

## Grundlage

Wieder aufgenommen wird die zurückgestellte Frage `260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md` mit ihrem Defekt `260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`. Gemessen war dort am 260810: ein `[[test]]` mit `harness = false` bekommt den Hauptfaden und wird von `cargo test` mitgefahren. Die Frage betraf vier Proben; heute sind es mindestens die 26 oben.

Die Hilfsdateien der Erhebung liegen außerhalb des Baums unter `/private/tmp/claude-501/-Users-kai-Projects-productive-F08-KRK-krk/0d876e38-8be9-4836-8bba-c89e56b51c4e/scratchpad/exlog/`.
