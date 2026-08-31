# Die acht Prosabefunde der Durchsicht der Runde 23, und der neunte aus der Behebung

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Durchsicht:** `260831-1444-coderev-git-bereich-runde-23.md`
**Geschlossen:** acht von neun
**Offen geblieben:** `260831-1444_*_drei-prosastellen-sagen-die-auswahl-der-verlaufsliste-uebersteht-den-tabwechsel-sie-faellt-mit-ihm.md`
**Gefilt:** `260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`. Dazu einzeln gefahren:
`cargo test -p krk-ui zwoelf_schalter_der_leiste_tragen_ein_kommando` (der Befehl, den der
Modulkopf der Bereichsleiste jetzt nennt) und
`cargo doc --workspace --no-deps --document-private-items`, um zu sehen, dass keiner der neu
gesetzten Doc-Verweise unaufgelöst bleibt. Der erste Versuch setzte `[`Fokus::ALLE`]` als
Verweis; `ALLE` steht unter `#[cfg(test)]` und ist damit nicht verlinkbar, also steht der Name
jetzt in einfachen Backticks, wie ihn `fokus.rs:203` schon führt. Die zwei unaufgelösten
Verweise, die `tabs.rs` weiter meldet (`:6` und `:406`), standen vorher da und gehören nicht zu
diesem Zug.

---

## Was der Auftrag war und was daraus geworden ist

Neun Datensätze, keiner mit Verhaltensänderung. Acht sind geschlossen, einer steht offen und
hat stattdessen eine Nutzerfrage bekommen.

### Befund 1 — die Auswahl der Verlaufsliste: **nicht geschlossen**

Am Baum nachgelesen und bestätigt: `Tabliste::waehlen` ruft `gitlauf_nachziehen_an(verlassen)`,
dessen dritte Zeile `self.tabs[stelle].gitmodell.zuruecksetzen()` unbedingt und vor jeder
Bedingungsprüfung steht; `Gitmodell::zuruecksetzen` setzt `*self = Self::neu()` und nimmt Kopf,
Verlauf, Zusammenfassung und Auswahl mit. Beim Zurückwechseln entsteht der Verlauf neu und die
Auswahl steht auf `None`. Der Wechsel des **aktiven Dateifensters** übersteht sie dagegen, weil
jede `Tabliste` ihr eigenes Gitmodell hält — die halbe Aussage der drei Prosastellen trifft zu.

Der Nutzerentscheid vom 260831-0120 hat Möglichkeit 2 gewählt und seine Antwortzeile nennt „sie
übersteht damit den Tabwechsel" als den sichtbaren Unterschied. Der Baum hält das nicht. Die
Prosa an den Baum anzugleichen nähme diese Entscheidung stillschweigend zurück; den Baum
anzugleichen ist eine Verhaltensänderung. Beides ist Nutzerarbeit, also steht der Datensatz
offen, trägt eine Notiz mit dem Befund und verweist auf die neue Frage in `decisions/`. Die drei
Prosastellen sind unverändert.

### Befund 2 — der Untergrenzen-Abschnitt von `appkit/git.rs`: neu erhoben

Nicht ergänzt und gestrichen, sondern Methode für Methode gegen den Rumpf der Datei und gegen
das SDK unter `xcrun --show-sdk-path` neu erhoben. Ergebnis:

- **Gestrichen**, weil im Rumpf nirgends: `window`, `makeFirstResponder:`.
- **Aufgenommen**, wie der Datensatz sie nennt: `deselectAll:` (`NSTableView.h:338`),
  `documentView` (`NSScrollView.h:48`), `initWithIdentifier:` (`NSTableColumn.h:31`).
- **Aufgenommen, vom Datensatz nicht genannt**: die zwei hier gebauten Protokollmethoden
  `numberOfRowsInTableView:` (`NSTableView.h:743`) und `tableViewSelectionDidChange:` (`:717`) —
  die Datei baut fünf `#[unsafe(method(…))]`, der Abschnitt nannte drei davon; das vierte
  bediente Protokoll `NSObjectProtocol`, dreimal adoptiert und nicht geführt; `alloc` und
  `init`, gerufen über vier `alloc` und ein `msg_send![super(this), init]`.
- **Berichtigt**: `isFlipped` steht in diesem SDK an `NSView.h:141`, nicht an `:236`. Jede
  übrige Zeilenangabe des Abschnitts ist einzeln am SDK nachgelesen und stimmt.
- **Keine Berührung liegt über macOS 15.** Die höchste bleibt `NSTableViewStyle` samt
  `setStyle:` seit 11.0, also vier Hauptfassungen unter dem Zielsystem. Das Absturzrisiko, das
  die Vorkehrung abwehrt, bestand nicht; beschädigt war die Prüfbarkeit der Liste.

Die Zählangabe „Zehn Berührungen sind jünger als ihre Klasse" ist gefallen: die Liste darunter
führt seit ihrer Niederschrift dreizehn Namen in neun Punkten, und keine Lesart ergibt zehn. An
ihrer Stelle steht jetzt, dass die Liste die Vorkehrung und keine Zusammenfassung ist, wodurch
sie am 260831 falsch war, und welches `grep` die Kandidaten der nächsten Erhebung liefert.

### Befund 3 — `rundweg.rs`

Vier Stellen laut Datensatz, sechs am Ende. Die Überschrift `:24` und `:59` tragen keine Zahl
mehr, `:26` trennt die zwei Zahlen (die Drei bleibt, weil sie an `Rundweg` gebunden ist; die
Zahl der ausgangslosen Werte heißt „die uebrigen", weil sie mit jedem neuen `Fokus` wächst),
`:65` nennt `Fokus::ALLE` statt einer Zahl und schreibt aus, was `fokus::wirkt` bei
`Wirkungsbereich::Dateibereiche` durchlässt, `:160` sagt „derselben Werte". Zwei mitgezogene
Stellen: die Überschrift `:59` sprach von „den beiden ausgangslosen Werten" (es sind drei), und
der Bullet nannte für `Fokus::Git` keinen Grund, obwohl der Rumpf bei `:141` einen hat.
`grep -nE 'fuenf (Werte|Fokuswert)' crates/krk-ui/src/kommandos/rundweg.rs` liefert nichts.

### Befund 4 — die Kurzhashlänge: die einzige Codeänderung

`KURZHASHLAENGE` ist von `git/leser.rs` nach `git/mod.rs` gewandert, als `pub(crate) const`,
neben `Commit`, `Kopf`, `Marke` und die `ObjectId`-Wiederausfuhr, aus denen beide
Schwestermodule ohnehin lesen. `leser::kurzhash` und `texte::verlaufszeile` lesen dieselbe Zahl
über dieselbe `use super::{…}`-Zeile; die nackte Sieben in `texte.rs:113` ist weg, und die Probe
`die_verlaufszeile_traegt_vier_angaben_in_dieser_reihenfolge` prüft gegen die Konstante statt
gegen ihre eigene hingeschriebene Sieben.

**Warum der Elter und nicht `pub(super)` in `leser`:** eine Abhängigkeit `texte` → `leser`
entstünde sonst für eine Zahl, und `texte` hängt heute an keinem der beiden Schwestermodule.
Die ausgeschriebene Begründung ist mitgewandert und um den Absatz erweitert, warum die Zahl
nicht bei einem der beiden Schreiber wohnt.

### Befund 5 — der Modulkopf der Bereichsleiste

`Bereichsleiste::alle_schalter` gibt es nicht und gab es nie. An seiner Stelle steht die Probe,
die wirklich zählt, samt dem Befehl, der sie fährt, und der Auskunft, dass `alle_schalter` die
freie Funktion des Prüfmoduls dieser Datei ist — die Auskunft, deren Fehlen den falschen
Verweis erst möglich gemacht hat.

### Befund 6 — der Doc-Kommentar des Giteinzugs

Der letzte Absatz nennt sein Bezugswort und sagt das Gegenteil des alten Satzes: der
Kanalschluss räumt allein `tab.gitlauf` weg und nimmt `wartende_marken` nicht mit. Die zwei
Stellen, an denen das Feld fällt, stehen namentlich da (`gitlauf_nachziehen_an`, `abbrechen`).
Die richtige Hälfte — der Kanal ist drei tief, der Faden blockiert an keiner Meldung — steht
jetzt als eigener Satz und teilt sich kein Semikolon mehr mit der falschen.

### Befund 7 — der Einzugstakt

Drei Kanäle statt zwei, ausgeschrieben und auf `Tabliste::arbeitet_noch` verwiesen. Die Zahl
bleibt eine Zahl: sie ist an die drei Zweige jener einen Methode gebunden.

### Befund 8 — `krk-core/Cargo.toml`

Der Kommentar nennt die Stellen und die Erhebung statt einer Zahl. **Warum keine Zahl:** der
Datensatz nennt 37 Vorkommen in `leser.rs`, dieselbe Erhebung liefert heute achtundvierzig, und
die Zahl der Dateien ist von drei auf vier gestiegen, weil `git::lauf` die Kiste inzwischen in
einem Kommentar nennt. Mitgezogen: der Doc-Kommentar der Wiederausfuhr in `git/mod.rs`
begründete sie damit, dass die Oberfläche sich den letzten angezeigten Commit merke, um beim
Nachladen dort weiterzumachen — das gilt seit der Umstellung auf `WeitererVerlauf { bereits }`
nicht mehr.

### Befund 9 — die drei Signaturen im Plan

`## Data Structures` nennt die Formen, die der Baum trägt, und ein Absatz unter dem Codeblock
nennt beide Abweichungen samt den zwei Durchsichtsbefunden, die sie verursacht haben. Er steht
neben dem Absatz, den die erste Runde desselben Befunds hinterlassen hat.

---

## Wo Code statt Prosa angefasst ist

Genau eine Stelle, Befund 4: `KURZHASHLAENGE` wandert von `crates/krk-core/src/git/leser.rs`
nach `crates/krk-core/src/git/mod.rs`, beide Schreiber lesen sie, und die Probe in `texte.rs`
prüft gegen sie statt gegen eine hingeschriebene Sieben. Kein Verhalten ändert sich: die Zahl
ist dieselbe, an jeder der drei Stellen. Alles übrige ist Modulkopf, Doc-Kommentar,
Zeilenkommentar, ein Kommentar in `Cargo.toml` und ein Abschnitt des Plans.

---

## Geänderte Dateien

- `crates/krk-core/Cargo.toml`
- `crates/krk-core/src/git/mod.rs`
- `crates/krk-core/src/git/leser.rs`
- `crates/krk-core/src/git/texte.rs`
- `crates/krk-ui/src/appkit/git.rs`
- `crates/krk-ui/src/appkit/bereichsleiste.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/tabs.rs`
- `crates/krk-ui/src/kommandos/rundweg.rs`
- `fusion-workbench/circles/260830-1045-git-bereich-liest-status-branch-verlauf/planning/260830-1317_p_plan-git-bereich-liest-status-branch-verlauf.md`
- acht Defektdatensätze `_o_` → `_c_`, einer mit Notiz und weiter `_o_`, eine neue Nutzerfrage
