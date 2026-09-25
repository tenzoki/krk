# Coder: S4 bis S6 der Runde 7 — das Hauptmenü trägt jede Funktion und graut aus

**Datum:** 260813-0445
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte S4, S5 und S6 aus
`circles/260813-0100-…/planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`,
in einem Lauf, samt dem Nachtrag vom 260813-0233
**Abnahme:** `cargo build --workspace` Exit 0, `cargo fmt --all --check` Exit 0,
`cargo clippy --workspace --all-targets -- -D warnings` Exit 0, `cargo test --workspace`
Exit 0. Proben im Binärziel `krk` vorher 488, nachher 511; in `xtask` unverändert 46.
Kein Bündelbau, kein Vordergrundlauf, keine Messung.

## Was gebaut wurde

**S4.** `Funktionsbereich::ALLE` und die Aufzählung selbst stehen in der Reihenfolge
Anwendung, Dateilisting, Dateioperationen, Tabs, Vorschau, Leiste und Fokus, Editor,
Bearbeiten, Fenster; `Funktionsbereich::Textbefehle::name()` liefert „Bearbeiten" statt
„Textbefehle". Der Variantenbezeichner bleibt `Textbefehle` — er benennt die sechs Funktionen
und nicht das Menü. Die Aufzählung ist mit umsortiert und nicht nur `ALLE`, damit die
Deklaration und die Anzeigefolge dieselbe Auskunft geben; ihr Doc-Kommentar sagt seit jeher
„in der Reihenfolge der Anzeige".

**S5.** `crates/krk-ui/src/menuemodell.rs` ist neu: `aufbau(&Belegung) -> Vec<Obermenue>` als
reine Rechnung über `belegungsmodell::nach_bereichen`, dessen dritter Abnehmer sie damit ist.
`Eintrag` ist eine vollständige Fallunterscheidung ohne Auffangzweig aus `Befehl`,
`Textbefehl`, `Sonderposten` und `Trenner`. Die Zuordnung der sechs zugestellten Kennungen zu
ihren AppKit-Selektoren steht als `ZUSTELLER` an dieser einen Stelle und hat die sechs
`sel!`-Literale in `hauptmenue` abgelöst.

**S6.** `hauptmenue` setzt das Modell um und baut nichts mehr selbst; `roher_befehl` bleibt die
eine Stelle, die ein `NSMenuItem` anlegt, `appkit_paar` die eine Übersetzung. Ein Eintrag mit
Kommando trägt den Sammelselektor `krkKommando:` und im `tag` seinen Index aus
`Kommando::KENNUNGEN`. Am Anwendungsdelegierten stehen dafür zwei neue Methoden:
`krkKommando:` und `validateMenuItem:` über `NSMenuItemValidation`. Die drei eigenen
Selektoren `beenden:`, `fensterEinblenden:` und `fensterSchliessen:` sind fort; die drei
Einträge laufen jetzt wie jeder andere über `kommando_ausfuehren`.

## Der Befund, um dessentwillen dieser Lauf mehr getan hat als der Plan sagt

**Zwei Menüeinträge mit `cmd+a`, und AppKit nimmt sie dem späteren still weg.** Am gebauten
Menü über `--menue-protokoll` gemessen: „Alle Einträge markieren" (Dateilisting, vorn) behielt
`cmd+a`, „Alles auswählen" (Bearbeiten, hinten) stand mit leerem Zeichen und gesetzter
Befehlstaste da. Derselbe Lauf gegen `HEAD` zeigt für „Alles auswählen" `kuerzel="a"`.

Die Folge wäre ein Bruch von C2.18 und C2.8 gewesen: ohne Menükürzel erreicht `cmd+a` den
Feldeditor eines Textfeldes auf keinem Weg (gemessen am 260804-1309), und `Cmd+A` wäre in
jedem Textfeld ausgefallen. Der Fall entsteht erst mit dieser Runde — bis dahin trug das Menü
zehn Einträge, und `alle_markieren` war keiner davon. Die Begründung des Entscheids vom
260805, „zwei Funktionen mit verschiedenen Zustellern begegnen einander nie", gilt für die
Belegungsdatei weiter und für die Menüleiste nicht mehr.

**Gebaut ist: das Menükürzel bekommt der Zusteller.** Ein Befehl von KRK braucht seines nicht,
weil der Ereignisabgriff jeden Tastendruck vor dem Menü sieht; eine zugestellte Funktion hat
diesen zweiten Weg nicht. Der Preis ist die Anzeige an einem Eintrag: „Alle Einträge
markieren" zeigt kein `Cmd+A`, obwohl `Cmd+A` es auslöst.

Warum trotz Plan gebaut und nicht angehalten: S6 ist ausdrücklich ungeteilt, damit kein
Zwischenstand entsteht, in dem das Menü Kürzel trägt und etwas kaputtmacht. Genau ein solcher
Zustand wäre entstanden. Die Richtung ist trotzdem eine Nutzerentscheidung und liegt als
Datensatz vor, mit vier Möglichkeiten und den Folgen jeder einzelnen.

- `issues/260813-0416_o_zwei-menueeintraege-mit-cmd-a-und-appkit-nimmt-dem-spaeteren-das-kuerzel.md`
- `decisions/260813-0430_o_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`

## Ein zweiter, kleinerer Verlust, gemeldet und nicht behoben

`issues/260813-0420_o_das-menue-bearbeiten-verliert-seine-mac-uebliche-reihenfolge-und-seinen-trenner.md`.
Die Reihenfolge innerhalb eines Obermenüs kommt jetzt aus der Belegungsdatei, und dort stehen
„Rückgängig" und „Wiederholen" **hinter** den vier Zwischenablage-Befehlen statt davor; der
Trenner zwischen beiden Gruppen fällt weg, weil das Modell genau einen Trenner führt. Kein
Befehl fällt aus, keine Kombination ändert sich. Behoben gehört es in
`resources/default-keymap.toml` und damit beim `ontocoder`, nicht hier — eine Ausnahmeliste im
Modell wäre die zweite Ordnung, die C2.2 ausschließt.

## Was am gebauten Menü gemessen ist

`cargo run -q -p krk-ui --bin krk -- --menue-protokoll`, Exit 0, 83 Zeilen. Kein
`make menue`; das Ziel hängt an `bundle` und überschriebe das beglaubigte Bündel.

- **C2.12:** jede Zeile trägt Beschriftung, Kombination, rohes AppKit-Paar und Selektor,
  unverändert im Format der Runde 3.
- **C2.13:** weder „Emoji & Symbols" noch „Start Dictation…" noch „AutoFill", keine Zeile mit
  `zweitform=ja` oder `verdeckt=ja`.
- **C2.1 und C2.3:** neun Obermenüs in der Folge Anwendung (4), Dateilisting (24),
  Dateioperationen (14), Tabs (4), Vorschau (3), Leiste und Fokus (8), Editor (13), Bearbeiten
  (6), Fenster (7). 83 Zeilen abzüglich Trenner und Sonderposten sind 81 Befehlseinträge, also
  genau die Zahl der Funktionen.
- **Die Selektoren:** 75-mal `krkKommando:`, je einmal `cut:`, `copy:`, `paste:`,
  `selectAll:`, `undo:`, `redo:`, `tastenbelegungSichern:`. Kein `beenden:`, kein
  `fensterEinblenden:`, kein `fensterSchliessen:`.
- **C2.9:** „Tastenbelegung als Markdown sichern" steht im Anwendungsmenü ohne Kürzel,
  darunter ein Trenner, darunter „KRK beenden" auf Cmd+Q.

Der Modus setzt den Anwendungsdelegierten nicht, also ist die Ausgrauung dabei nicht geprüft;
das war auch vorher so und ist kein Rückschritt.

## Die neuen Zählproben und was sie halten

- **C2.2** `die_gliederung_hat_drei_abnehmer` (`menuemodell.rs`): genau drei Dateien nennen
  `nach_bereichen`, namentlich. Eine Aufruferzählung über **Dateien**, weil ein Abnehmer, der
  die Gliederung zweimal in derselben Datei fragt, derselbe Abnehmer ist.
- **C2.10** `es_gibt_eine_stelle_je_anlage_und_uebersetzung` (`menue.rs`): `NSMenuItem::alloc(`
  einmal, `initWithTitle_action_keyEquivalent(` einmal, `fn appkit_paar(` einmal.
- **C2.11** `das_menue_wird_an_zwei_anlaessen_gebaut`: zwei Aufrufe von `hauptmenue(` außerhalb
  seiner eigenen Datei.
- **C2.14** `der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten`: drei Aufrufer
  von `kommando_ausfuehren` am Delegierten — Tastendruck, Menüeintrag, Klick in die
  Bereichsleiste. Gezählt über die zwei Empfängernamen `self.` und `selbst.`, weil
  `kommando_ausfuehren` daneben auch an Tabelle, Leiste und Vorschau steht.
- **C2.16, zweite Hälfte** `beide_frager_rufen_die_eine_regel` (`zulaessigkeit.rs`): genau zwei
  Aufrufer von `zulaessigkeit::zulaessig(`. Die erste Hälfte, die Erklärungszählung, steht seit
  S2 daneben; ihr Vorwärtsverweis ist jetzt auf die neue Probe umgeschrieben.
- **C2.17** `die_freigabe_eines_eintrags_wird_nirgends_gesetzt` (`menue.rs`): kein
  `setEnabled(`, kein `setAutoenablesItems(` im Baum, und genau eine Erklärung von
  `validateMenuItem:`. **Bewusst keine Rechnung über die Tafel:** die Umkehrung von C2.5 ist
  auf der Ebene der Funktion eine Tautologie, weil beide Frager dieselbe Funktion rufen; was
  wirklich zu halten ist, ist dass niemand die Freigabe an zweiter Stelle **setzt**. Gezählt
  wird dabei die Erklärung `unsafe(method(validateMenuItem:))` und nicht der Name — der steht
  in etlichen Doc-Kommentaren dieser Runde.

Dazu drei Proben zum `tag` (jedes Kommando überlebt den Weg hin und zurück, kein zweites teilt
sich einen, ein `tag` außerhalb der Liste benennt keines) und zwei zur Kollision (siehe oben).

Weggefallen ist `jede_kennung_des_hauptmenues_steht_in_der_auslieferungsbelegung`: `hauptmenue`
schlägt keine Kennung mehr nach. Was sie hielt, hält jetzt
`jede_zugestellte_kennung_steht_in_der_auslieferungsbelegung` in `menuemodell.rs` für die sechs
Kennungen, die überhaupt noch von Hand dastehen — und zwar schärfer, weil es auch prüft, dass
sie `gehalten_von = "menue"` tragen.

## Zwei kleinere Abweichungen vom Plan

**Die Selektornamen stehen als `&'static CStr` und nicht als `&'static str`.** Der Plan nennt
„seinen AppKit-Selektornamen". `Sel::register` nimmt einen `&CStr` entgegen; ein `&str`
verlangte eine `CString` je Eintrag beim Start und damit Arbeit auf dem Pfad von L4 ohne
Gegenwert. Die `c"…"`-Literale kosten nichts und halten die Tabelle an einer Stelle.

**`Eintrag` leiht seine Zeichenketten, statt sie zu kopieren.** `aufbau` gibt
`Vec<Obermenue<'_>>` zurück, gebunden an die Belegung. Der Aufbau liegt auf dem Startpfad;
162 Zeichenketten je Start wären dort Arbeit ohne Gegenwert.

## Was diese Runde L4 kostet, soweit ohne Vordergrundlauf zu sagen

Am Baum abgezählt und **nicht** gemessen. `inference:`

Der Aufbau ist einmal je Menübau, und das sind zwei Anlässe, davon einer auf dem Startpfad.
Er kostet: ein Durchgang durch `nach_bereichen` (81 Funktionen, neun Bereiche, also 729
Vergleiche auf Bereichsgleichheit — der Preis stand schon vorher, weil die Belegungsansicht
denselben Aufruf macht, aber nicht auf dem Startpfad), 81-mal ein Nachschlag in `ZUSTELLER`
(sechs Zeilen, linear), 75-mal ein Nachschlag in `Kommando::KENNUNGEN` für den `tag` (75
Zeilen, linear, also rund 2.800 Vergleiche im Mittel), 81-mal ein Durchgang durch die sechs
zugestellten Kürzel, und je Eintrag eine `NSString::from_str` und ein `NSMenuItem`.

**Der Posten, der wirklich zählt, ist der letzte**, und er ist der, den der Plan nennt: 83
Objective-C-Objekte statt zehn. Die Rechnung davor ist einige zehntausend Maschinenbefehle und
gegen einen Kaltstart nicht sichtbar; der lineare Nachschlag im `tag` ließe sich zu einer
Tabelle machen, wenn eine Messung ihn je auffällig fände. **Ob L4 hält, sagt allein der Lauf
am Bündel**, und die Runde behauptet es nicht.

## Am Bündel nicht geprüft

Alles, was am laufenden `KRK.app` im Vordergrund zu sehen ist. Für diese drei Schritte gehören
auf die Abnahmeliste:

- **C2.6**, die fünf Fälle der Ausgrauung.
- **C2.7**, dass während eines Blattes alles grau ist außer Abbruch und Ausnahmeliste.
- **C2.18**, dass Cmd+Q und Shift+Cmd+W während einer Umbenennung und während eines Blattes
  weiterwirken — und daneben, dass Cmd+A im Textfeld weiterhin den Text auswählt (der Befund
  von 260813-0416).
- **C2.19**, dass ein ausgegrauter Eintrag auch mit der Maus nicht bedienbar ist.
- **Opt+Cmd+Q bekommt keine Zweitform „Quit and Keep Windows"** und Opt+Shift+Cmd+W kein
  „Close All": die drei eigenen Selektoren sind fort, und `KRK_KOMMANDO` ist so wenig
  `terminate:` und `performClose:` wie sie es waren. Am Baum ist das nur abgeleitet.
- **Die neue Menügliederung im Bild**, samt der Belegungsansicht und der Markdown-Ausgabe, die
  ihr seit S4 folgen.
- **L4**, siehe oben.

Der Prüfvorbehalt des Spec bleibt, wie er ist: ob ein Menüeintrag mit einem Kürzel **ohne**
Befehlstaste dem Ersthelfer die Taste wegnimmt, ist am eigenen Baum nicht belegt. Trifft die
Herleitung zu, verhindert die Ausgrauung den Schaden; trifft sie nicht zu, kostet sie den
Mausklick. Aufbau und Ausgrauung sind deshalb in einem Schritt gebaut, und es gibt keinen
Zwischenstand ohne sie.

## Geänderte und neue Dateien

Neu:

- `crates/krk-ui/src/menuemodell.rs`

Geändert:

- `crates/krk-ui/src/main.rs`
- `crates/krk-ui/src/belegungsmodell.rs`
- `crates/krk-ui/src/appkit/menue.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs`

Im Speicher der Runde:

- `planning/260813-0205_o_plan-…md` — S4, S5 und S6 tragen `[DONE]`
- `issues/260813-0416_o_zwei-menueeintraege-mit-cmd-a-…md` — neu
- `issues/260813-0420_o_das-menue-bearbeiten-verliert-…md` — neu
- `decisions/260813-0430_o_wer-bekommt-das-menuekuerzel-…md` — neu

Der Datensatz `decisions/260813-0159_o_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`
bleibt **offen**: S4 fährt auf seiner Empfehlung, aber der Nutzer hat sie nicht beantwortet,
und die Runde hält die fünf Datensätze aus `## Offene Fragen` bis zu einer Antwort offen.
