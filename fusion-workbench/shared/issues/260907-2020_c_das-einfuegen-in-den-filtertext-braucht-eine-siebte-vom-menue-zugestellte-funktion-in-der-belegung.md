# Das Einfügen in den Filtertext braucht eine siebte vom Menü zugestellte Funktion in der Belegung

---

Der Nutzerentscheid vom 260907-2009 verlegt das Einfügen in den Filtertext des
Dateifensters von `cmd+v` auf `cmd+f` und zieht `cmd+v` für eine spätere
Dateizwischenablage frei. Der Weg dorthin verlangt eine neue Funktion in
`resources/default-keymap.toml` mit `gehalten_von = "menue"`. Die Datei gehört dem
`ontocoder`; der `coder` hat sie deshalb nicht angefasst und die ganze Aufgabe K14
angehalten.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

**Cross-references:**
`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
(der beantwortete Entscheid),
`260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md` (die
Zustellerregel),
`260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`
(die Kürzelregel),
`260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`
(wen die neue Zeile nicht erreicht)

## Warum überhaupt eine Zeile in der Belegung

Heute trägt das Einfügen in den Filtertext **keine** eigene Zeile: es reitet auf
`text_einfuegen` (`cmd+v`, `gehalten_von = "menue"`, Selektor `paste:`), und der
Anwendungsdelegierte beantwortet `paste:`, wenn keine Textfläche vor ihm in der
Antwortkette steht (`crates/krk-ui/src/appkit/anwendung.rs`, Methode `paste:` →
`einfuegen_ausfuehren`).

Auf `cmd+f` geht das nicht ohne Zeile. `cmd+f` liegt auf `editor_suchen`
(`resources/default-keymap.toml:1015-1019`), einer gewöhnlichen Funktion mit
Kommando und ohne `gehalten_von`, also vom Ereignisabgriff zugestellt. Eine zweite
vom **Abgriff** zugestellte Funktion auf `cmd+f` wäre nach der Regel vom 260805 ein
Konflikt, und `Belegung::nachschlag`
(`crates/krk-core/src/tasten/belegung.rs:1486-1497`) liefert ohnehin nur den ersten
Treffer — die zweite wäre still tot. Bleibt der Weg des `cmd+a`-Paares: eine zweite
Funktion auf derselben Kombination mit dem **anderen** Zusteller, also
`gehalten_von = "menue"`.

Der Ablauf ist damit gelesen und nicht geraten: mit dem Fokus im Dateifenster
schlägt der Abgriff `editor_suchen` nach, `zulaessigkeit::zulaessig` weist es ab
(`Kommando::EditorSuchen` trägt `Wirkungsbereich::Editor`,
`crates/krk-core/src/tasten/belegung.rs:1155-1159`), `behandeln` liefert `false`
(`crates/krk-ui/src/appkit/ereignisse.rs:658-661`), und der Tastendruck fällt
unverändert an AppKit. Dort trägt der neue Menüeintrag das Kürzel und schickt
seinen Selektor die Antwortkette hinunter zum Delegierten. Mit dem Fokus im Editor
schluckt der Abgriff `cmd+f` vorher, und das Menü sieht es nie.

## Die Zeile

Einzusetzen im Block des Dateilistings, hinter dem Eintrag `inhaltssuche_umschalten`
(dem „Content"-Schalter, `resources/default-keymap.toml:506-509`). Die Stelle
ist nicht gleichgültig: die Reihenfolge der Blöcke ist die Reihenfolge im
Obermenü, und `crates/krk-ui/src/belegungsmodell.rs` `bereich` ordnet die Kennung
dem `Funktionsbereich::Dateilisting` zu und nicht den Textbefehlen — die Funktion
füllt einen Filter und ist kein Textbefehl.

```toml
[[funktion]]
id = "filter_einfuegen"
name = "In den Filter einfügen"
tasten = ["cmd+f"]
gehalten_von = "menue"
```

Darunter der erklärende Kommentar, gebaut wie der bei `text_alles_auswaehlen`
(`resources/default-keymap.toml:1166-1171`): dass `cmd+f` auch bei `editor_suchen`
steht und das kein Konflikt ist, weil dort der Ereignisabgriff zustellt und hier
das Menü; dass der Fokusvorbehalt die beiden einander nie begegnen lässt — mit dem
Fokus im Editor sucht `cmd+f` im Text, im Dateifenster hängt es die Zwischenablage
an den Filtertext; und der Verweis auf den Nutzerentscheid vom 260907-2009 samt
`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`.

Es ist die erste vom Menü zugestellte Funktion außerhalb des Blocks „Bearbeiten"
und der erste Zusteller, dessen Selektor KRK selbst führt statt AppKit.

## Was in derselben Datei mitgeht

1. Der Kopf zählt im Präsens: „Ausgeliefert sind 92 Funktionen mit zusammen 95
   Kombinationen" (Zeile 34). Nachgezählt am 260907-2020 stimmen beide Zahlen
   (`grep -c '^\[\[funktion\]\]'` liefert 92); mit der neuen Zeile werden es 93 und
   96. Die Zeile ist **nicht** nachlässig zu behandeln: die Probe
   `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
   (`crates/krk-core/src/tasten/belegung.rs:1950`) liest beide Zahlen aus dem
   Dateikopf zurück und zählt die Datei dagegen. Sie trägt kein Literal, wird also
   rot, ohne dass jemand sie anfassen müsste. Der offene Befund
   `260812-0810_*_die-zahl-39-im-kopf-der-belegungsdatei-steht-im-praesens-und-ist-ungeprueft.md`
   betrifft dieselbe Bauart und wird hiervon nicht geschlossen.
2. Der C10-Block (Zeilen 81-92) schreibt `cmd+v` als den Weg in den Filtertext
   fest: „`paste:` seit der Runde 21 und hängt den Text oder den Dateinamen aus der
   Zwischenablage an den Filtertext an". Nach dem Entscheid tut `cmd+v` im
   Dateifenster nichts mehr, bis die Dateizwischenablage steht; die Reservierung
   ist damit wieder offen und nicht mehr „ganz eingelöst".
3. Der Kommentar bei `text_einfuegen` (Zeilen 1120-1146) und die Zeile 738 im
   C10-Kopf nennen dieselbe Zuordnung.
4. Der Kopfabsatz zur Zustellerregel (Zeilen 118-129) sagt, ausgeliefert gebe es
   „genau einen Fall der Doppelung", nämlich `cmd+a`. Mit `cmd+f` sind es zwei.
   Dieselbe Aussage steht zweimal im Code, in `crates/krk-ui/src/menuemodell.rs:317`
   und als Kommentar in `crates/krk-core/tests/belegung.rs:472`; die zwei zieht der
   `coder` nach.

## Warum das nicht getrennt commitbar ist

Beide Reihenfolgen lassen den Baum rot, und die erste nicht nur rot:

- Die Belegungszeile allein: `belegungsmodell::bereich`
  (`crates/krk-ui/src/belegungsmodell.rs:223-236`) kennt die Kennung nicht und
  liefert `None`; `nach_bereichen` (ebenda 899-905) **bricht darauf laut ab**. Über
  diese Stelle gehen `Belegungsmodell::neu`, `menuemodell::aufbau` und
  `belegungsausgabe::markdown`, also fällt nicht eine Probe aus, sondern jede, die
  das Modell, das Menü oder die Markdown-Ausgabe baut. Dazu
  `jede_kennung_hat_einen_funktionsbereich` (ebenda 1030),
  `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
  (`crates/krk-core/src/tasten/belegung.rs:1950`) und
  `die_sechs_zugestellten_tragen_ihren_selektor_und_kein_kommando`
  (`crates/krk-ui/src/menuemodell.rs:473`).
- Der Code allein: der siebte Eintrag in `menuemodell::ZUSTELLER` fände keine
  Funktion, der Filter bliebe unerreichbar, und die Proben zum neuen Paar fielen
  aus.

Die Belegungszeile und der Codeanteil gehören deshalb in **einen** Durchgang. Was
der `coder` dabei mitzuziehen hat, steht in seinem Verlaufseintrag
`260907-2020-k14-cmd-f-fuegt-in-den-filtertext-ein.md`.

Nicht betroffen ist die Konfliktfreiheit: `Belegung::konflikte`
(`crates/krk-core/src/tasten/belegung.rs:1577-1590`) vergleicht nur innerhalb
desselben Zustellers, und `die_auslieferungsbelegung_ist_konfliktfrei`
(`crates/krk-core/tests/belegung.rs:259`) bleibt grün. Das gilt **nur** für die
Zeile mit `gehalten_von = "menue"`; ohne dieses Feld stünden zwei vom Abgriff
zugestellte Funktionen auf `cmd+f`, und die eingebettete Belegung bräche schon
beim ersten Zugriff ab (`belegung.rs:162-167`).

## Abnahme

`make tasten` gibt für `cmd+f` zwei Zeilen aus, „Im Text suchen" und „In den Filter
einfügen", und für `cmd+v` nur noch „Einfügen"; `make menue` zeigt das Kürzel
`Cmd+F` am neuen Eintrag und nicht mehr an „Im Text suchen". `cargo test
--workspace` und `cargo clippy --workspace --all-targets -- -D warnings` sind grün.

---
Resolved: Beide Hälften stehen im Baum. Die Belegungszeile hat der `ontocoder`
am 260907-2046 geschrieben
(`260907-2046-k15a-die-belegungszeile-fuer-cmd-f.md`), den Codeanteil der
`coder` am 260907-2127 (`260907-2127-k15b-der-code-zur-belegungszeile-cmd-f.md`):
siebter Eintrag in `menuemodell::ZUSTELLER` mit dem eigenen Selektor
`filterEinfuegen:`, die Kennung in `belegungsmodell::bereich` auf
`Funktionsbereich::Dateilisting`, ein eigener Zweig in
`belegungsausgabe::wirkung` als fünfte Begründungslage, und beim
Anwendungsdelegierten tritt `filterEinfuegen:` an die Stelle von `paste:`, in
der Methode wie im Zweig von `validateMenuItem:`. Die 52 Fehlschläge des roten
Zwischenstands sind weg; `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all
--check` und `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` sind
je mit exit 0 zurückgekommen.

**Von der Abnahme dieses Befunds sind `make tasten` und `make menue` nicht
gefahren**: sie verlangen ein gebautes, signiertes Bündel und sind Nutzerarbeit.
Was ohne Bündel geprüft ist, prüfen zwei neue Proben —
`cmd_f_steht_bei_zwei_funktionen_und_ist_kein_konflikt`
(`crates/krk-core/tests/belegung.rs`) für die Belegungshälfte und
`cmd_f_traegt_im_menue_das_filtereinfuegen_und_nicht_die_editorsuche`
(`crates/krk-ui/src/menuemodell.rs`) für die Menühälfte. Dass `cmd+v` im
laufenden Bündel wirklich schweigt, ist ungemessen; gemessen ist allein, dass
der Delegierte `paste:` nicht mehr beantwortet
(`der_delegierte_beantwortet_copy_cut_und_das_filtereinfuegen`).
