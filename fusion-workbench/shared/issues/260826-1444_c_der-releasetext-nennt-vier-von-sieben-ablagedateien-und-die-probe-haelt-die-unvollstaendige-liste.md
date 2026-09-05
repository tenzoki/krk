Der Releasetext nennt vier von sieben Ablagedateien, und die Probe hält die unvollständige Liste
---
`RELEASETEXT` zählt auf, was ein Löschwerkzeug mitnimmt: Lesezeichen, Sitzung, Tastenbelegung, Notizzettel. `Datei::ALLE` führt daneben die Einstellungen und die Leseprofile.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/veroeffentlichung.rs`, `crates/krk-core/src/ablage/pfade.rs`

## Befund

`veroeffentlichung.rs:569-571`:

> Dort hält KRK alles, was es sich merkt: die Lesezeichen, die gesicherte Sitzung, die abweichende Tastenbelegung und die zwei Notizzettel.

`Datei::ALLE` (`crates/krk-core/src/ablage/pfade.rs:226-234`) führt sieben: `Belegung`, `Lesezeichen`, `Sitzung`, `Einstellungen`, `Leser`, zwei `Zettel`. Es fehlen die Einstellungen und die von Hand gepflegten Leseprofile (`readers.toml`, siebte Ablagedatei seit Runde 16). `CLAUDE.md` stellt an seiner eigenen Aufzählung dieselbe Lücke fest („die Aufzählung hier hat die Einstellungen und die Leseprofile übergangen, seit es sie gibt") und verweist auf `Datei::ALLE` als die Auskunft.

Die Probe `der_releasetext_traegt_jede_seiner_aussagen` (`:955-990`) hält mit `:973-978` genau die vier Genannten und kann die zwei Fehlenden nicht vermissen.

## Warum es zählt

Der Text ist die eine Stelle, die ein Fremder im Augenblick des Installierens liest (`:538-540`). Wer weder Lesezeichen noch Zettel pflegt, aber Leseprofile von Hand geschrieben hat, liest „alles, was es sich merkt" gefolgt von vier Dingen, die ihn nicht betreffen, und hält die Regel für unerheblich. Der Verlust vom 17.08. (`shared/analyses/260820-2242-…`) ist der Fall, gegen den der Text steht.

Der Text ist seit 1.0.0 dreimal veröffentlicht (`v1.0.0`, `v1.1.0`, `v1.2.0`); die vorhandenen Seiten tragen die Lücke.

## Abhilfe

Entweder die Liste durch die Regel ersetzen („alles, was KRK sich merkt — welche Dateien, sagt die Anwendung selbst") oder die zwei nachtragen und die Probe um zwei Nadeln erweitern. Die erste Form altert nicht mit der achten Ablagedatei.

**Schwere:** Medium — veröffentlichte Prosa, die die Betriebsregel für einen Teil der Nutzer entwertet.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, M3

---
Resolved: Gewählt ist die erste der zwei angebotenen Formen, weil sie nicht altert. Die
Aufzählung ist aus `RELEASETEXT` (`xtask/src/veroeffentlichung.rs`) heraus; der Absatz lautet
jetzt:

> … nehmen dabei den Ordner `~/Library/Application Support/KRK/` mit. Dort hält KRK alles, was es
> sich merkt. Nach so einem Löschen ist es fort.

Kein Zeiger auf eine Stelle, an der der Leser nachsehen könnte, ist an die Stelle der Liste
getreten, und das ist Absicht: die Releaseseite liest ein Fremder im Augenblick des
Installierens, und „welche Dateien, sagt die Anwendung selbst" wäre für ihn ein Verweis ins
Leere — KRK gibt diese Liste nicht aus. Was er braucht, ist die Betriebsregel, und die trägt
„alles" vollständig. Die achte Ablagedatei ändert daran nichts.

Die Probe `der_releasetext_traegt_jede_seiner_aussagen` hält die neue Form von der anderen Seite:
die drei Nadeln der alten Liste sind heraus, und dazugekommen ist eine Zusicherung über den
Text mit zusammengezogenen Leerzeichen, dass „Dort hält KRK alles, was es sich merkt." unmittelbar
von „Nach so einem Löschen" gefolgt wird. Eine wieder eingeschobene Aufzählung lässt sie rot
werden, gleich welche Dateien sie nennt. Der Kommentar an der Zusicherung nennt den Grund und
diesen Datensatz.

Die drei veröffentlichten Seiten (`v1.0.0`, `v1.1.0`, `v1.2.0`) tragen die Lücke weiter; ihr Text
liegt auf der Gegenseite und nicht in diesem Baum. Die nächste Veröffentlichung trägt die neue
Fassung.

Geprüft: `cargo test -p xtask` (164 Proben, Exit 0), `cargo clippy -p xtask -p krk-bench
--all-targets -- -D warnings` (Exit 0). Baumstand `ba0c6bd`.
