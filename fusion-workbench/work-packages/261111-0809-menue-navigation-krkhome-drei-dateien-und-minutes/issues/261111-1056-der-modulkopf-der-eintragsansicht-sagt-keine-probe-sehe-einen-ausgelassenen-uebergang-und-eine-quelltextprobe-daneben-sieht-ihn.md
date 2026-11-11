Der Modulkopf der Eintragsansicht sagt, keine Probe sehe einen ausgelassenen Übergang, und eine Quelltextprobe in derselben Datei sieht ihn
---
`crates/krk-ui/src/appkit/eintragsansicht.rs` sagt im Modulkopf unter „Stelle und Zeile“ seit dem 261111: „einen Uebergang, der die Umrechnung auslaesst, sieht seitdem keine Probe“. Im Prüfmodul derselben Datei steht `jede_zeilennummer_von_appkit_geht_ueber_die_umrechnung`, die genau das für die Aufrufe hält, die sie mit Namen führt: `rowForView(`, `selectedRow()` und `clickedRow()` nur in einer Zeile mit `stelle_an(`, `selectRowIndexes…` und `editColumn…` je an einer Stelle und über `zeile_von(`. Der Satz ist damit zu weit: gefallen sind allein die Proben an einer umgeordneten Tabelle, die prüften, ob die Umrechnung richtig rechnet.
---
**Filed by:** code-implementer, Kai Stalmann <ks@qantr.com>

Gefunden beim Berichtigen von `.claude/rules/coding-was-man-nicht-sieht.md` (Schritt 9 des Plans `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`). Die Regeldatei nennt seitdem beides: die gefallenen Proben und die Quelltextprobe, die bleibt, samt dem, was diese nicht sieht (eine Zeilennummer unter einem anderen Namen). Der Modulkopf widerspricht ihr in dem einen Satz.

Beleg: `crates/krk-ui/src/appkit/eintragsansicht.rs`, Modulkopf, Abschnitt `# Stelle und Zeile`, letzter Satz des ersten Absatzes; dieselbe Datei, Prüfmodul, `jede_zeilennummer_von_appkit_geht_ueber_die_umrechnung` mit ihrem Doc-Kommentar („Was sie nicht sieht“).

Abnahme: der Satz im Modulkopf nennt die Quelltextprobe und grenzt ein, was seit dem 261111 keine Probe mehr hält (die Richtigkeit einer Umrechnung, die keine Gleichheit ist, und eine Zeilennummer, die unter einem anderen als den geführten Namen hereinkommt). `make check` bleibt grün.
