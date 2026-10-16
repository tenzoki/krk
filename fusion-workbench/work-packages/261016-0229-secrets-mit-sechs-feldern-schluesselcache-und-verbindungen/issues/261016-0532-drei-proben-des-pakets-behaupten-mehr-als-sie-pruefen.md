Drei Proben des Pakets und ihre Doc-Kommentare behaupten mehr, als sie prüfen
---
Die Probe für `bookmarks.toml` prüft den Typ statt des Weges, die Ruferzählung von `Passwort::preisgeben` übersieht einen Funktionsverweis, und Probe und Doc-Kommentar zum Schreiber des Merkers nennen einen Rufer, wo der Code zwei hat.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Akzeptanz (2) und (6) von Schritt 6, Akzeptanz (6) von Schritt 5)

## Defekt

**(a) `eine_verbindung_schreibt_aus_dem_eintrag_allein_den_namen`** (`crates/krk-core/tests/ablage.rs:4538-4575`) baut das Lesezeichen selbst aus `entwurf.name` (`Lesezeichen::verbindung(entwurf.name.clone())`, Zeile 4549) und sucht danach die übrigen Feldwerte, die es nie hineingegeben hat. Gegen die Frage, ob der Betriebsweg (`anhaengen_einziehen`, `crates/krk-ui/src/appkit/anwendung.rs:9219-9228`) Beschriftung oder `verbindung` aus einem anderen Feld baut, ist sie blind. Was sie wirklich hält, ist die Schlüsselliste `["name", "verbindung"]`. `CLAUDE.md` schreibt ihr zu, sie halte, dass „in `bookmarks.toml` … von einer Verbindung allein der Name“ steht.

**(b) `das_passwort_wird_allein_an_netfs_preisgegeben`** (`crates/krk-ui/src/appkit/verbinden.rs:437-456`) zählt über `quellbaum::aufrufstellen` die Nadel `preisgeben(` (`crates/krk-ui/src/quellbaum.rs:204-220`). Ein Funktionsverweis ohne Klammer, etwa `.map(Passwort::preisgeben)`, ist ein zweiter Weg heraus, den sie nicht zählt. Der Modulkopf von `verbinden.rs` nennt als Grenze allein den Datenfluss nach dem Ruf.

**(c) `der_merker_hat_genau_einen_schreiber`** (`anwendung.rs:15213-15311`): Doc-Kommentar „hat … genau einen Rufer, `schluessel_merken`, und den ruft allein der Schluesselmelder des Editorbereichs“, der Rumpf prüft dagegen zwei Rufer (`oberflaeche_aufbauen` und `gemerkt_falls`, Zeilen 15241-15261). Ebenso der Doc-Kommentar von `schluessel_merken` (`anwendung.rs:10261-10263`): „Gerufen allein ueber den Schluesselmelder des Editorbereichs … ein Oeffnen mit dem gemerkten Schluessel kommt hier nie an“. Der Code ruft ihn auch aus `gemerkt_falls` (`anwendung.rs:9305-9309`) nach Klick und „Verbinden…“.

## Richtung der Behebung

(a) Entweder am Betriebsweg prüfen (eine Quelltextprobe, dass `anhaengen_einziehen` `name` und `verbindung` aus derselben getrimmten `entwurf.name` baut und aus keinem anderen Feld), oder Probe und `CLAUDE.md` sagen ehrlich, dass sie die Form der Datei halten. (b) Den Funktionsverweis als Fundstelle zählen (Nadel `preisgeben` ohne Klammer, Erklärung ausgenommen) oder die Grenze im Modulkopf nennen. (c) Beide Doc-Kommentare auf zwei Rufer nachziehen.

## Akzeptanz

- Jeder der drei Doc-Kommentare beschreibt, was sein Rumpf prüft, und keine Zusage in `CLAUDE.md` stützt sich auf mehr.
- `make check` grün.

---
Resolved: (a) Die Kernprobe heisst jetzt `ein_verbindungslesezeichen_schreibt_allein_name_und_verbindung` und sagt im Doc-Kommentar, dass sie die Form der Datei haelt (`crates/krk-core/tests/ablage.rs`); den Betriebsweg haelt die neue Quelltextprobe `anhaengen_einziehen_baut_das_lesezeichen_allein_aus_dem_namen` (`appkit/anwendung.rs`), `CLAUDE.md` nennt beide. (b) `das_passwort_wird_allein_an_netfs_preisgegeben` (`appkit/verbinden.rs`) zaehlt jede Nennung von `preisgeben` als ganzes Wort in einer Codezeile, also auch einen Funktionsverweis ohne Klammer, und prueft ihre Zaehlung an drei Beispielzeilen; Modulkopf und `CLAUDE.md` nachgezogen. (c) Doc-Kommentare von `der_merker_hat_genau_einen_schreiber` und `schluessel_merken` nennen beide Rufer (Schluesselmelder und `gemerkt_falls`); `CLAUDE.md`, `README.md` und `HowTo.md` nennen das Merken nach der PIN der Verbindungswege.
