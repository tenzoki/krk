# Schlussprüfung: Die AppKit-Proben laufen auf dem Hauptfaden

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `4b25d3c5..0067f466`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/work-packages/261101-1320-appkit-proben-laufen-auf-dem-hauptfaden/package.json`, `fusion-workbench/work-packages/261031-1645-markdown-vorschau-richtet-tabellenspalten-aus/package.json`, `261101-1436-plan-appkit-proben-laufen-auf-dem-hauptfaden.record.json`, `261101-1436-wie-kommen-die-appkit-proben-auf-den-hauptfaden.record.json`, `260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`, `261031-1645-markdown-vorschau-richtet-tabellenspalten-aus.md`
**Domain:** code
**Verdict:** revise (allein wegen Befund 1; der Code trifft Plan und Entscheid)

## Zusammenfassung

Der Läufer, das Prüfziel und die Umzüge tragen. `cargo test -p krk-ui --test hauptfaden` lief am 261101-1953 in diesem Baum grün: 47 Proben bestanden, 2 ignoriert. `defaults read -g NSAutomaticQuoteSubstitutionEnabled` zeigte vor und nach dem Lauf `0`. Ein Befund mittlerer Schwere liegt in der Regeldatei: Der neue Punkt zur Rekursionsgrenze nennt Ort, Meldung und Spielraum falsch, gemessen in einer Kopie des Baums. Drei kleine Befunde betreffen Beschreibungsreste und die Benennung einer abbrechenden Probe.

## Zahlen

| Schwere | Zahl | Befunde |
|---|---|---|
| Critical | 0 | |
| High | 0 | |
| Medium | 1 | Befund 1 |
| Low | 2 | Befunde 2 und 3 |

Ohne Defekt bleiben eine unbelegte Beobachtung (Abschnitt „Nicht belegt“) und eine kosmetische Doppelung.

## Befunde

### 1. Die Rekursionsgrenze steht in der Regeldatei an der falschen Stelle (Medium)

Laut `.claude/rules/coding-was-man-nicht-sieht.md`, letzter Punkt der Kopplungen am Prüfziel, bricht eine Zeile mehr am Doc-Kommentar von `umbenennung_beendet` (`DateifensterDelegierter`) den Bau mit „…`stringify!`“. Gemessen in einer Kopie von `0067f466` mit `cargo check -p krk-ui`:

- 30 Zeilen mehr an `umbenennung_beendet` (`appkit/tabelle.rs`, nach Zeile 5334) bleiben grün. Mit 10 Zeilen mehr sind auch `clippy -D warnings` und `cargo doc -D warnings` grün.
- Über `pub struct Namensfeld;` (`define_class!` ab `tabelle.rs:5814`, Einfügung nach Zeile 5843) ist 1 Zeile mehr grün. Ab 2 Zeilen sind beide Ziele rot, mit `recursion limit reached while expanding $crate::__macro_helpers::concat!`.

Die genannte Abhilfe, den Text an eine Methode zu verlegen, passt nicht zu einem Struct-Kommentar. Wer nach der Regeldatei handelt, kürzt einen Kommentar ohne Not und steht am wirklichen Ort ohne Hinweis. Defekt: `261101-1953-die-regeldatei-verortet-die-rekursionsgrenze-in-define-class-an-der-falschen-stelle.md`.

### 2. Fünf Stellen beschreiben weiter die Lage unter `libtest` (Low)

Nachgezogen ist allein der Kommentar am `Namensfeld`. Offen sind:

- `appkit/editor.rs:9366-9368` nennt das Prüfziel als Ausweg, der an der zurückgestellten Frage hänge.
- `appkit/editor.rs:13244` und `:13366` stehen in einer Messprobe, die jetzt auf dem Hauptfaden läuft. Dazu kommt der Rest `let _ = mtm;` (Zeile 13365).
- `appkit/editor.rs:9566` und `appkit/quicknote.rs:621`.
- `appkit/quicknote.rs:34-36` und der Quicknote-Absatz der Regeldatei sagen „das ist die Lage unter `libtest`“. Diese Stelle hat der Auftrag benannt. Sie ist nicht falsch, nennt aber die Umgebung, in der keine AppKit-Probe mehr läuft.
- Der geänderte Satz in `CLAUDE.md` zu `#![deny(unsafe_code)]` endet nach dem Einschub mit einem losen „und gegen `members` … zu halten“.

Defekt: `261101-1953-nach-dem-umzug-beschreiben-fuenf-stellen-noch-die-lage-unter-libtest.md`.

### 3. Ein Abbruch des Prozesses bleibt ohne Probennamen (Low)

`appkit/hauptfaden.rs:330-357` schreibt `test <name> ...` erst nach `catch_unwind`. Ein Hängen benennt der Wachfaden (`:477-485`). Ein Abbruch durch eine Ausnahme von Objective-C oder durch `SIGSEGV` hinterlässt dagegen als letzte Zeile die vorige Probe mit `ok`. Defekt: `261101-1953-der-laeufer-nennt-eine-probe-erst-nach-ihrem-ende-und-ein-abbruch-des-prozesses-bleibt-ohne-namen.md`.

## Geprüft und ohne Befund

**Befehlszeile** (`auswerten`, `Auswahl::trifft`, `:201-252`). Teiltext, `--exact` auch für `--skip`, `--skip x` und `--skip=x`, `--ignored`, `--include-ignored`, `--list` und `MARKEN_MIT_WERT` samt Folgewort verhalten sich wie gelesen. Nachgefahren: `-- --ignored --list` listet die 2 Messproben. `-- --skip appkit::editor --skip=appkit::tabelle --list` listet 1 Probe. `-- --exact die_sammeltuer` fährt 0 Proben, filtert 49 aus und endet grün, wie `libtest`.

**Ausgang** (`:256-281`, `src/hauptfaden.rs:68-76`). Grün sind nur `Gruen` und `Aufgelistet`, alles andere endet mit 101. Das hält die Tafel `gruen_ist_allein_ein_lauf_ohne_fehlschlag_und_das_auflisten`. Außerhalb des Hauptfadens kehrt `laufen` zurück, bevor eine Vorgabe gesetzt oder eine Probe gefahren wird (`:286-292`). Das prüft `der_laeufer_verweigert_ausserhalb_des_hauptfadens` mit vorangestellter Voraussetzung.

**Wachfaden und Frist** (`:462-491`). `Beginn` und `Ende` umschließen jede Probe mit Frist, auch eine, die in Panik gerät. Bei Zeitüberschreitung beendet der Wachfaden den Prozess mit 101 und nennt die Probe. Ignorierte Proben laufen ohne Frist.

**Werksvorgaben** (`:396-446`). Der vorhandene Argumentbereich wird gelesen, kopiert, entfernt und neu gesetzt. Das Setzen trifft damit keinen stehenden Namen, und Paare aus der Befehlszeile bleiben erhalten. Rückgelesen wird über `boolForKey`, und geschrieben wird nichts. Die Zeilenangaben im Untergrenzen-Abschnitt (`NSUserDefaults.h:15`, `:144`, `:179`, `:252`, `:329`, `:335`, `:339`, `NSDictionary.h:97`, `:606`, `NSValue.h:181`, `NSObject.h:43`) stimmen gegen das SDK von Xcode auf diesem Gerät.

**Die vier `unsafe`-Blöcke** (`:400`, `:409`, `:426`, `:452`) stehen unter `appkit/` und tragen je eine SAFETY-Begründung. Der Rückgabetyp `Option<Retained<…>>` an `volatileDomainForName:` deckt das gemessene `nil`, obwohl der Header die Antwort als nicht leer führt.

**Umzüge.** Gelesen ist der Diff ohne Leerraum von `editor.rs`, `tabelle.rs` und `blaetter/mod.rs`. Die zwei aufgelösten Hüllen mit Rückgabewert (`editor.rs:9165-9184` und `:13364-13383`) enthalten kein `return`, also ändert sich ihr Ablauf nicht. `verwalter_ohne_fenster(mtm)` und `gelesene_quelle(mtm, …)` reichen nur den Marker durch. Die Blatt-Probe setzt den Panik-Haken auf jedem Rust-Pfad zurück, denn `set_hook(haken)` steht nach `catch_unwind` und vor jeder eigenen Panik. Ein fremder Abbruch beendet ohnehin den Prozess. Die Gründe für `ignoriert` sind die früheren `#[ignore]`-Texte, wörtlich übernommen.

**Haltebedingung, Binärziel unberührt.** Außerhalb von `cfg(test)`, `[[test]]` und `src/hauptfaden.rs` ändert der Bereich allein Doc-Kommentare (`tabelle.rs:5828-5840` am `Namensfeld`, `eintragsansicht.rs:1950`). Hinzu kommen die zwei Weiterreichungszeilen unter `#[cfg(test)]`. `inference:` Doc-Attribute erreichen den Maschinencode nicht, also schuldet das Paket keinen Abnahmelauf. Einen Binärvergleich habe ich nicht gefahren.

**Zählproben und Wurzeln.** `mod_zeilen` vergleicht beide Wurzeln Zeile für Zeile. Ein `diff` der `mod`- und Attributzeilen zeigt allein `#![allow(dead_code, unused_imports)]` als Unterschied, und den liest die Probe nicht, weil sie nur Attributzeilen vor `mod` vergleicht. `grep -rn '^#!\[deny(unsafe_code)\]' crates/*/src/*.rs xtask/src/*.rs` liefert 5 Zeilen, davon 2 für `krk-ui`, wie `CLAUDE.md` es jetzt sagt. Die Zählproben in `crates/krk-core/tests/baum.rs` und `appkit/betrachter.rs` liefen im `make check` des Ausführers und des Orchestrators grün. Ich habe sie nicht erneut gefahren.

## Nicht belegt

`WERKSVORGABEN` legt drei Schlüssel fest, die auf diesem Gerät global auf 0 stehen. `die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` vergleicht dagegen alle sieben `Abgeschaltet`-Einträge mit der frischen Fläche, darunter `setAutomaticTextReplacementEnabled:`. Der Satz „gleich was der Nutzer global eingestellt hat“ in der Regeldatei und am Kommentar von `WERKSVORGABEN` reicht damit weiter als die drei Schlüssel, falls eine frische `NSTextView` weitere Automatiken aus den Vorgaben liest. Ein Lauf des Prüfziels mit `-NSAutomaticTextReplacementEnabled NO` auf der Befehlszeile blieb grün. Ohne Gegenprobe lässt das offen, ob die Fläche den Schlüssel nicht liest oder das Paar nicht ankam. Deshalb ist kein Defekt angelegt.

## Querschnitt

Befund 2 wiederholt das Muster, das der Plan in Schritt 2 benennt: Eine Begründung zieht in vielen Kommentaren mit, und nachgezogen werden nur die Stellen, die der Plan aufzählt. `grep -rn libtest crates/krk-ui/src` liefert außerhalb des Läufers rund 50 Zeilen. Die meisten bleiben wahr („den `libtest` nicht hergibt“). Falsch werden die, die einen Ausweg als offen nennen oder eine Probe begründen, die jetzt im Prüfziel läuft.

Kosmetisch und ohne Defekt: Die Zahl 101 steht als `ROT` in `appkit/hauptfaden.rs:131` und daneben als Literal in `src/hauptfaden.rs:74`.

## Reihenfolge

1. Befund 1 vor dem Schließen des Pakets, denn er steht in einer Regeldatei, nach der jede Sitzung handelt.
2. Die Befunde 2 und 3 können folgen. Keiner von beiden ändert das Binärziel.
