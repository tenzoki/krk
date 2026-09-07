# K14: Erhebung des Einfügewegs, und der Halt an der Belegungsdatei

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Den Nutzerentscheid vom 260907-2009 umsetzen
(`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`):
`cmd+f` übernimmt im Dateifenster das Einfügen in den Filtertext, `cmd+v` wird
freigezogen und tut dort nichts, bis die Dateizwischenablage steht.

## Die Erhebung des heutigen Wegs

Das Einfügen in den Filtertext ist **kein Kommando** und trägt **keine Zeile** in
`resources/default-keymap.toml`. Es reitet auf `text_einfuegen` (`cmd+v`,
`gehalten_von = "menue"`, Zeilen 1160-1164), dessen Menüeintrag den Selektor
`paste:` mit Ziel `nil` die Antwortkette hinunterschickt
(`crates/krk-ui/src/menuemodell.rs:141`). Steht keine Textfläche davor, endet die
Kette beim Anwendungsdelegierten, der `paste:` beantwortet
(`crates/krk-ui/src/appkit/anwendung.rs:945-948`) und über den einen Vorspann
`bearbeiten_am_dateifenster` (ebenda 3346-3356) an die Datenquelle des aktiven
Dateifensters weiterreicht. Das Tor ist `zulaessigkeit::dateiablage_zulaessig`,
derselbe zweite Eingang, den `copy:` und `cut:` seit der Runde 22 nehmen; die
Ausgrauung fragt dieselbe Regel (`anwendung.rs:1011-1015`). Die Reinigung des
eingefügten Textes ist `krk_core::zwischenablage::filtertext_aus` und bleibt
unberührt.

Wirkungsbereich hat die Funktion damit keinen im Sinne von `Kommando::wirkungsbereich`
— sie ist keines. Was ihr entspricht, ist `Anspruch::Dateiablage`:
`Wirkungsbereich::Dateifenster`, nicht während eines Blattes, nicht immer
erreichbar (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:48-57`).

## Warum `cmd+f` eine Belegungszeile verlangt

`cmd+f` liegt auf `editor_suchen` (`default-keymap.toml:1015-1019`), einer
gewöhnlichen Funktion mit Kommando und ohne `gehalten_von`, also vom
Ereignisabgriff zugestellt. Eine zweite vom Abgriff zugestellte Funktion auf
derselben Kombination ist nach der Regel vom 260805 ein Konflikt, und
`Belegung::nachschlag` (`crates/krk-core/src/tasten/belegung.rs:1486-1497`) liefert
ohnehin nur den ersten Treffer. Der einzige Weg, der zur Regel passt, ist der des
`cmd+a`-Paares: eine zweite Funktion mit dem **anderen** Zusteller, also
`gehalten_von = "menue"`.

Der Ablauf ist gelesen: mit dem Fokus im Dateifenster schlägt der Abgriff
`editor_suchen` nach, `zulaessig` weist es ab (`Kommando::EditorSuchen` trägt
`Wirkungsbereich::Editor`, `belegung.rs:1155-1159`), `behandeln` liefert `false`
(`crates/krk-ui/src/appkit/ereignisse.rs:658-661`), der Tastendruck fällt an AppKit
und erreicht dort den Menüeintrag. Mit dem Fokus im Editor schluckt der Abgriff ihn
vorher.

Der Dispatch nennt als Grund „zwei Funktionen in verschiedenen **Wirkungsbereichen**".
Das ist nicht die Regel: der Datensatz vom 260805 sagt ausdrücklich
„`Wirkungsbereich` ist kein zweiter Zusteller" und „zwei vom Abgriff zugestellte
Funktionen mit verschiedenem Wirkungsbereich bleiben ein Konflikt". Getragen wird
das Paar vom **Zusteller** und nicht vom Wirkungsbereich. Am Ergebnis ändert das
nichts; an der Bauform alles.

## Wie die Menükürzel-Regel auf das Paar wirkt

`menuemodell::zugestellte_kuerzel` (`crates/krk-ui/src/menuemodell.rs:339-346`)
sammelt die erste Kombination jeder Funktion, deren Kennung in `ZUSTELLER` steht;
`eintrag` (ebenda 280) filtert genau diese Kombinationen aus dem Zweig
`Eintrag::Befehl`. Eine neue zugestellte Funktion auf `cmd+f` nimmt „Im Text suchen"
damit die **Anzeige** des Kürzels im Hauptmenü, und der neue Eintrag zeigt es.
Wirkung geht keine verloren: der Abgriff sieht `cmd+f` vor dem Menü und führt
`editor_suchen` aus, wo es zulässig ist. Das ist genau der Preis, den der
Nutzerentscheid vom 260906-2147 für `cmd+a` angenommen hat
(`260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`).
Belegungsansicht und Markdown-Ausgabe zeigen die Kombination unverändert; sie
fragen die Belegung.

## Greift nach dem Freiziehen etwas anderes `cmd+v` auf?

Nein. In `resources/default-keymap.toml` trägt `cmd+v` genau eine Tastenliste,
die von `text_einfuegen` (Zeile 1163); `shift+cmd+v` bei `verschieben` (Zeile 156)
ist eine andere Kombination. Fällt die `paste:`-Antwort des Delegierten weg,
liefert `nachschlag` für `cmd+v` `Nachschlag::Unbelegt`, der Abgriff schluckt
nichts, AppKit stellt `paste:` mit Ziel `nil` zu, und in der Antwortkette
beantwortet es niemand mehr: die Probe in
`crates/krk-ui/src/appkit/betrachter.rs:781` hält fest, dass `paste:` im ganzen
Baum genau einmal steht, beim Anwendungsdelegierten. `inference:` der Menüeintrag
„Einfügen" ist dann grau und `cmd+v` tut im Dateifenster nichts — gelesen, aber
ohne laufendes Bündel nicht gemessen.

## Warum nichts gebaut ist

Die Umsetzung verlangt eine neue Funktion in `resources/default-keymap.toml`. Diese
Datei gehört dem `ontocoder`; dieses Projekt hat sie mehrfach so geführt (Plan der
Runde 20, Schritt 3 „ontocoder: drei Einträge in der Belegung"; Plan der Runde 22,
offene Frage zu den zwei Kommentaren). Der `coder` fasst sie nach seinem
Auftragsrahmen nicht an, sondern hält an und legt einen Befund ab:
`260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md`.

Die zwei Hälften sind dabei **nicht** getrennt commitbar. Die Belegungszeile allein
lässt `belegungsmodell::bereich` (`crates/krk-ui/src/belegungsmodell.rs:223-236`)
`None` liefern, was die Probe `jede_kennung_hat_einen_funktionsbereich` (ebenda
1030) fängt; der Code allein lässt den siebten `ZUSTELLER`-Eintrag ins Leere
zeigen. Belegungszeile und Codeanteil gehören in einen Durchgang.

## Was der Codeanteil umfasst, erhoben und nicht geschätzt

Zwei Selektoren statt einem. `paste:` fällt beim Anwendungsdelegierten weg, damit
`cmd+v` im Dateifenster schweigt; an seine Stelle tritt ein eigener Selektor
(`filterEinfuegen:`), den KRK selbst führt und der durch denselben Vorspann
`bearbeiten_am_dateifenster` und dasselbe Tor `dateiablage_zulaessig` geht. Der
`Anspruch` bleibt derselbe, die Zahl der Selektoren bleibt drei, und die Zählprobe
`die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` bleibt bei zwei Fragern.

Die Stellen:

| Datei | Was |
|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs:935-948` | `paste:` weicht `filterEinfuegen:`; Doc nachziehen |
| `crates/krk-ui/src/appkit/anwendung.rs:1011-1015` | der Zweig in `validateMenuItem:` |
| `crates/krk-ui/src/appkit/anwendung.rs:78-96, 219` | Modulkopf, „Drei Antworten ohne Kommando" |
| `crates/krk-ui/src/appkit/anwendung.rs:3319-3375` | Doc des Vorspanns und von `einfuegen_ausfuehren` |
| `crates/krk-ui/src/appkit/anwendung.rs:10411-10439` | die Probe `der_delegierte_beantwortet_copy_cut_und_paste` |
| `crates/krk-ui/src/menuemodell.rs:127-145` | `ZUSTELLER`, Länge `6` → `7`, siebter Eintrag |
| `crates/krk-ui/src/menuemodell.rs:53, 133, 191, 317, 470` | „die sechs"; „genau einen solchen Fall" |
| `crates/krk-ui/src/menuemodell.rs:473` | `die_sechs_zugestellten_tragen_ihren_selektor_und_kein_kommando` |
| `crates/krk-ui/src/belegungsmodell.rs:227-234` | die Kennung, Ziel `Funktionsbereich::Dateilisting` |
| `crates/krk-ui/src/belegungsmodell.rs:43, 140-145, 178-180, 214-215` | „die sechs" |
| `crates/krk-ui/src/belegungsausgabe.rs:274-293` | eigener Zweig in `wirkung`, sonst `NICHT_EINGEORDNET` |
| `crates/krk-ui/src/belegungsausgabe.rs:846-854` | `jede_kennung_ohne_kommando_wird_vom_menue_zugestellt` |
| `crates/krk-ui/src/belegungsausgabe.rs:52, 255-266, 301, 361, 729, 817, 822` | „die sechs" |
| `crates/krk-ui/src/appkit/menue.rs:61, 137, 172-181, 223, 886-894` | „die sechs"; die Zählvorschrift `grep -c 'gehalten_von'` gäbe 7 |
| `crates/krk-ui/src/appkit/betrachter.rs:717-727, 778-782` | die Stellenprobe zu `paste:` |
| `crates/krk-ui/src/kommandos/zulaessigkeit.rs:10, 38, 48, 234, 280, 447, 486` | Prosa zum dritten Selektor |
| `crates/krk-ui/src/kommandos/mod.rs:83` | dieselbe Prosa |
| `crates/krk-ui/src/kommandos/operationen.rs:18, 1400-1414` | die vier Abweisungssätze nennen `cmd+v` |
| `crates/krk-ui/src/appkit/zwischenablage.rs:79-97, 361` | die `paste:`-Hälfte der Hülle |
| `crates/krk-ui/src/appkit/tabelle.rs:25, 2052, 2085-2088` | die Datenquelle nennt `cmd+v` |
| `crates/krk-core/src/zwischenablage.rs:42-44` | Modulkopf der zweiten Deutung |
| `crates/krk-core/tests/belegung.rs:428-429, 472-474` | Kommentare zur Doppelung |
| `HowTo.md:82-88` | der Satz, den der Nutzer im Releasepaket liest |

`menue.rs:894` (`die_sechs_zugestellten`) und `menue.rs:929` (`GEMESSEN`) bleiben
bei sechs: sie messen, welche **AppKit**-Klasse einen Selektor beantwortet, und
`filterEinfuegen:` beantwortet keine. Falsch wird dort allein die Begründung, die
„dieselben sechs" mit den sechs `gehalten_von`-Funktionen gleichsetzt.

Neu zu schreiben ist eine Probe zum Paar, nach dem Muster von
`cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt`
(`crates/krk-core/tests/belegung.rs:471`) und
`bei_einer_doppelten_kombination_traegt_der_zusteller_das_kuerzel`
(`crates/krk-ui/src/menuemodell.rs:682`). Die zwei Muster gibt es also.

## Was nicht abgelegt ist, und warum

Ein Befund gegen `CLAUDE.md:35` (die Runde-21-Zeile nennt `cmd+v` als den Weg in
den Filtertext) ist **nicht** geschrieben. Solange die Änderung nicht steht, ist die
Zeile richtig; ein Defektdatensatz gegen eine zutreffende Aussage wäre falsch. Er
gehört in denselben Durchgang, der die Änderung baut. `activity-log-k1.md:24` und
`:873` nennen `cmd+v` ebenfalls; das ist die Aufzeichnung eines Standes und bleibt
nach der Ortsregel stehen.

## Was am Baum unverändert ist

`git status` meldet keine Datei außerhalb von `fusion-workbench/`. HEAD steht
unverändert auf `0bc7d86`. Gegen diesen Stand sind `cargo build --workspace`,
`cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`
und `cargo fmt --all --check` in einer Kette gelaufen und mit 0 zurückgekommen; das
ist der Ausgangsstand für den Durchgang, der die Änderung baut, und keine Prüfung
einer Arbeit dieser Sitzung.
