# Drei Zeilenzitate im Quelltext zeigen ins Leere, und keines davon stammt aus `52fba42`

---

`shared/issues/260823-1336_o_die-zeilenzitate-der-zwei-offen-gebliebenen-befunde-*` hält fest, dass
die Zeilenangaben nach `anwendung.rs` in den Workbench-Datensätzen nach `52fba42` ins Leere zeigen.
Dieselbe Klasse gibt es **im Quelltext**: vier Doc-Kommentare zitieren einen Zeilenbereich einer
anderen Datei dieses Baums, drei davon treffen ihn nicht mehr. Keiner der drei ist durch `52fba42`
falsch geworden — sie waren es schon vorher, und nichts im Baum hält sie.

---

**Jede Angabe einzeln nachgeschlagen.**

## Die vier Zitate und ihr Stand

| Zitat steht in | Zitiert | Zugesagter Inhalt | Steht dort heute | Stand |
|---|---|---|---|---|
| `crates/krk-core/src/tasten/belegung.rs:1272` | `crates/krk-ui/src/appkit/menue.rs:322-342` | `NSMenuItem.keyEquivalent` nimmt eine Zeichenkette | `registerDefaults` für die Vorgaben, dann der Doc-Kopf des Sammelselektors | **falsch** |
| `crates/krk-core/src/tasten/parser.rs:34` | `crates/krk-ui/src/appkit/menue.rs:322-342` | dieselbe Stelle, „die Zuordnung in `zeichen_der_taste` dort" | dasselbe; `fn zeichen_der_taste` steht bei `menue.rs:580` | **falsch** |
| `crates/krk-ui/src/hervorhebung.rs:178` | `crates/krk-ui/src/appkit/leiste.rs:439-442` | warum KRK das Erscheinungsbild von Hell und Dunkel nicht nachbaut | `let mtm = self.mtm(); let stelle = …; modell.beschriftung(stelle)?` — der Zeilenaufbau der Leiste; die Begründung steht bei `leiste.rs:567` | **falsch** |
| `xtask/src/release.rs:796` | `crates/krk-ui/src/appkit/anwendung.rs:575` | wörtlich `let schwach = objc2::rc::Weak::from_retained(&self.retain());` | ein Doc-Kommentar zum Feld `ablage`; die zitierte Zeile steht dreißigmal in der Datei, zuerst bei `:1101` | **falsch** |
| `crates/krk-ui/src/appkit/tabelle.rs:381` | `krk-bench/src/messen.rs:1199` | die kopflose Strecke baut keine `NSTableView` | der Kommentar zur kopflosen Strecke und `Messreihe::fahren` | trägt noch |

Die fünfte Zeile steht in der Tafel, weil sie zeigt, dass die Form nicht an sich schadhaft ist:
ein Zitat, das auf einen selten angefassten Abschnitt zeigt, hält jahrelang. Die vier darüber
zeigen, wovon das abhängt, nämlich von nichts.

## Keines ist durch `52fba42` entstanden

Das war die Frage der Beauftragung. Die Antwort ist nein, und sie ist entschieden und nicht
geschätzt:

- **Nur ein Zitat im Quelltext zeigt überhaupt nach `anwendung.rs`**, `xtask/src/release.rs:796`.
- Es war **schon vor dem Bereich falsch**: bei `28cbb7b` steht auf `anwendung.rs:575` derselbe
  Doc-Kommentar zum Feld `ablage` wie heute, und die zitierte Zeile stand dort auf `:1195`.
  Richtig war es zuletzt bei `4db66ed`, dem Commit, der das Zitat angelegt hat; dort steht auf
  Zeile 575 wörtlich `let schwach = objc2::rc::Weak::from_retained(&self.retain());`.
- Die drei übrigen zeigen in Dateien, die `52fba42` nicht angefasst hat.

Nachgeschlagen mit `grep -rn "\.rs:[0-9]" crates/ xtask/ README.md CLAUDE.md`. Das Muster findet
außerdem sechzehn Zitate in Fremdkisten (`objc2-app-kit-0.3.2/src/generated/…`, `syntect-5.3.0/…`);
die bleiben hier außen vor, weil eine fremde Kiste an ihrer Version hängt und sich unter einem
festen `Cargo.lock` nicht verschiebt.

## Warum das hier steht und nicht in `260823-1336`

Jener Datensatz ist offen und wird nicht angefasst. Er beschreibt die Workbench-Datensätze, deren
Zitate ein einzelner Commit auf einen Schlag verschoben hat, und sein Auslöser ist dieser Commit.
Die vier hier sind über Monate einzeln veraltet, ohne dass ein Commit sie zusammen getroffen hätte,
und ihr Gegenmittel ist ein anderes: nicht Nachziehen, sondern die Zeilenzahl weglassen. Der Baum
macht das an den meisten Stellen schon so und verweist über den Funktionsnamen oder den Modulkopf.

## Vorschlag

Die drei falschen Zitate auf die Form ziehen, die dieser Baum sonst benutzt: die Datei nennen und
den Namen der Funktion oder des Abschnitts, nicht die Zeile. Für die drei ist der Zielname
bekannt und steht in der Tafel oben (`zeichen_der_taste` in `menue.rs`, die Begründung zu Hell und
Dunkel in `leiste.rs`, die zitierte `Weak`-Zeile in `anwendung.rs`). Eine Zeilenzahl, die niemand
hält, ist in diesem Baum dieselbe Sorte Angabe wie eine Aufzählerzahl, die niemand hält — und für
die hat `260823-1032` gerade entschieden, dass sie gestrichen und nicht korrigiert wird.

**Schwere:** Low. Kein Verhalten ist betroffen. Der Schaden ist Lesezeit: wer dem Zitat folgt,
landet an einer Stelle, die von etwas anderem handelt, und muss erst merken, dass er falsch steht.

**Gefunden:** coderev, Auslieferungsdurchsicht `28cbb7b..b58e9d1`, Baumstand `b58e9d1`

**Domain:** code

**Cross-references:** `shared/issues/260823-1336_o_die-zeilenzitate-der-zwei-offen-gebliebenen-befunde-*`
(nicht angefasst), `shared/issues/260823-1032_c_zwei-zahlen-im-modulkopf-der-kommandos-*`

---
Resolved:

Also seen: 260826-1440 by coderev — ein viertes Zitat derselben Gattung in `veroeffentlichung.rs:374-375` (`beglaubigung.rs:344`, `:369`, `:379`); am `c13bf1c` stimmen alle drei noch, die nächste eingefügte Zeile in `beglaubigung.rs` kippt sie.
