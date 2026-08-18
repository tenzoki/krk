Ein vierter Träger der verkürzten Blattsperre liegt außerhalb von `crates` und fehlt in der Erhebung

---

`resources/default-keymap.toml:708` sagt, bei stehender Nachfrage weise der
Anwendungsdelegierte „jeden Befehl ausser dem Abbruch" ab. Es sind vier. Der Datensatz
`260817-1302`, der genau diese Formulierung erhebt, sagt von sich „eine Suche ueber den ganzen
Baum findet zwei" und nennt diese Stelle nicht: gelesen wurde `crates/`.

---

**Schwere:** Niedrig. Der Schluss an der Stelle hält, und zwar nachgerechnet: die drei Befehle
der Ausnahmeliste liegen ab Werk auf `cmd+q`, `shift+cmd+w` und `cmd+n` und nicht auf `return`,
über das der Absatz spricht. Falsch ist die Begründung, nicht das Ergebnis. Der Befund steht,
weil eine Erhebung, die „der ganze Baum" sagt und eine Kiste liest, beim nächsten Durchgang
dieselbe Stelle wieder nicht sieht.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `resources/default-keymap.toml:706-709`
**Verwandt:** `issues/260817-1302_o_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-und-der-datensatz-nennt-sie-nicht.md`,
`issues/260817-1111_c_die-begruendung-an-loeschauftrag-stellen-nennt-eine-ausnahme-es-sind-vier.md`
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

```toml
# resources/default-keymap.toml:706-709, im Abschnitt von mit_standardprogramm_oeffnen
# Ein Blatt faengt die Taste weiterhin ab, bevor sie hier ankommt: bei stehender
# Nachfrage weist der Anwendungsdelegierte jeden Befehl ausser dem Abbruch ab,
# und der Tastendruck laeuft unveraendert an AppKit weiter, wo ihn die
# Vorgabeschaltflaeche beantwortet.
```

Durchgelassen werden vier Kommandos: `Kommando::Abbrechen` über
`kommandos::operationen::waehrend_blatt_erlaubt` (`operationen.rs:266-268`) und
`Kommando::Beenden`, `Kommando::FensterSchliessen` und `Kommando::FensterEinblenden` über
`kommandos::zulaessigkeit::immer_erreichbar` (`zulaessigkeit.rs:197-202`), das die Blattsperre
ausdrücklich mit aufhebt. Beide Stellen selbst gelesen.

Die vollständige Erhebung über den Baum **und** die Werkbankgrenze hinweg:

```
$ grep -rn "ausser dem Abbruch\|außer dem Abbruch" crates/ CLAUDE.md resources/
crates/krk-ui/src/appkit/anwendung.rs:406    Aussage ueber den Stand bis S16, kein Befund (so in 1302)
crates/krk-ui/src/appkit/anwendung.rs:6312   nennt die Ausnahmeliste, richtig
crates/krk-ui/src/appkit/editor.rs:1298      Befund, in 1302 genannt
CLAUDE.md:123                                Befund, in 1111 genannt, offen
resources/default-keymap.toml:708            Befund, in keinem Datensatz genannt
```

`anwendung.rs:2840`, den `1302` als seinen ersten Träger nennt, trägt die Worte nicht wörtlich
(„ein Blatt laesst allein den Abbruch durch"); die Aussage dort ist dieselbe und der Befund
gilt. Die Zeile ist heute `:2841`.

## Warum die Stelle besonders zählt

Zwei Ebenen daneben steht seit T1 dieselbe Begründung ausgeschrieben und richtig:
`blaetter/mod.rs:296-307` nennt die vier Befehle, nennt beide Quellen und rechnet nach, dass
keiner der drei zusätzlich zugelassenen ab Werk auf einer Eingabetasten-Kombination des Blattes
liegt. Genau diesen Schluss zieht der Kommentar in der Belegungsdatei, und er zieht ihn aus der
verkürzten Prämisse.

Es ist die dritte Ausprägung eines Musters, das dieses Projekt schon führt: CLAUDE.md hält
fest, dass jedes Suchmuster mit `\.md` einen blinden Fleck hat, und `shared/issues/260810-1851`
trägt den Fall, in dem fünf Erhebungen dieselben acht Stellen nicht sahen. Hier ist der blinde
Fleck die Ordnergrenze `crates/`.

## Richtung

Die Zeile in `resources/default-keymap.toml` nachziehen, mit `1302` und `1111` in einem Zug —
sie tragen denselben Nachzug. Und die Nadel der nächsten Erhebung um `resources/` und
`CLAUDE.md` erweitern, bevor gezählt wird.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, alle fünf Stellen der Erhebung
nachgelesen, zwei Zeilennummern verschoben.** Die Nadel
`grep -rn "ausser dem Abbruch\|außer dem Abbruch" crates/ CLAUDE.md resources/` liefert weiter
fünf Zeilen. Neu sind zwei Stände: `anwendung.rs:6312` liegt jetzt an `:6463` (die Commits des
Bündels C haben rund 450 Zeilen vor der Stelle eingefügt), und `CLAUDE.md:123` liegt an `:124`,
weil `CLAUDE.md` am 260817-1600 die Zeile `**Artifact language:** en` bekommen hat.
`resources/default-keymap.toml:708`, `editor.rs:1298` und `anwendung.rs:406` stehen unverschoben.
Die Stelle mit der gleichbedeutenden, nicht wortgleichen Formulierung liegt jetzt an
`anwendung.rs:2866` und nicht an `:2841`, wie dieser Datensatz sagt.

---
Abgleich 260818 (coder, Bündel C/D-Nachzug): **bleibt offen, und zwar allein an seiner
eigenen Zeile.** Die zweite Hälfte des Befundes — dass eine Erhebung, die „der ganze Baum"
sagt und eine Kiste liest, dieselbe Stelle wieder nicht sieht — ist erledigt; die
beanstandete Zeile selbst steht unverändert.

**Warum sie nicht mitkommt.** `resources/default-keymap.toml` ist vom Auftrag dieser Runde
ausdrücklich ausgenommen: daran arbeitet als nächstes ein anderer Executor, und eine Änderung
hier fiele in dessen Commit. Die Datei ist daneben die eine Quelle jeder Tastenbelegung, also
Datenbestand und nicht Bauwerkzeug, und gehört damit ohnehin nicht dem `coder`. Die Richtung
des Datensatzes gilt unverändert: die Zeile nennt vier Kommandos statt einem, mit `1302` und
`1111` in einem Zug.

**Die Erhebung, dieses Mal über den ganzen Baum und nicht über eine Kiste.** Gezählt am
260818 nach den Korrekturen dieser Runde. Gesucht wurde nicht nach dem Wortlaut allein,
sondern nach der **Aussage**: eine Stelle, die sagt, was die Sperre *als Ganzes* durchlässt,
und dabei weniger als vier nennt. Zwei Nadeln, die zweite absichtlich weiter als die
wörtliche, weil `anwendung.rs` die Aussage in anderen Worten trug:

```
$ grep -rn "ausser dem Abbruch\|außer dem Abbruch" \
      crates/ xtask/ resources/ CLAUDE.md README.md Makefile idea.txt Cargo.toml .claude/
$ grep -rniE "(ausser|außer|bis auf|nur|allein|einzig)[^.]{0,45}(abbruch|abbrechen)" \
      <denselben Pfaden>
```

Ausgenommen sind `fusion-workbench/`, `messungen/` und `spikes/`: nach der Ortsregel in
`CLAUDE.md` behalten Aufzeichnungen eines Standes ihre damalige Formulierung. `target/` und
`.git/` tragen keinen Quelltext.

**Sechs Träger, und zwei davon nannte kein Datensatz und keine vorige Erhebung:**

| Stelle | genannt in | Stand |
|---|---|---|
| `crates/krk-ui/src/kommandos/operationen.rs`, Abschnittskopf und Doc von `waehrend_blatt_erlaubt` | keinem — **die Wurzel**, aus der die übrigen ihre Formulierung haben | behoben |
| `crates/krk-ui/src/kommandos/operationen.rs`, zwei Probennamen und eine Fehlschlagsmeldung | keinem | behoben |
| `crates/krk-ui/src/appkit/anwendung.rs`, Kopf von `kommando_ausfuehren` | `260817-1302` | behoben |
| `crates/krk-ui/src/appkit/editor.rs` | `260817-1302` | behoben |
| `resources/default-keymap.toml:710` | **dieser Datensatz** | steht, außerhalb des Umfangs |
| `CLAUDE.md:124` | `260817-1111`, `260817-1302` | steht, außerhalb des Umfangs |

Fünf Stellen sind gelesen und **kein** Träger: `krk-core/src/tasten/belegung.rs:638` und
`:952` (Aussagen über je ein einzelnes Kommando, korrekt aus `waehrend_blatt_erlaubt`
hergeleitet, und weder `Notizzettel` noch `TabSchliessen` steht auf `immer_erreichbar`),
`anwendung.rs:406` (Stand bis S16, sagt es), `anwendung.rs:6440` und `zulaessigkeit.rs:613`
(nennen die Ausnahmeliste, vollständig).

**Der blinde Fleck ist damit zweimal derselbe gewesen, und der zweite war nicht die
Ordnergrenze.** Der erste war `crates/`, wie dieser Datensatz sagt. Der zweite war die Nadel:
`anwendung.rs` trug die Aussage in anderen Worten, `operationen.rs` unter der Überschrift
statt im Satz, und beide entgingen der wörtlichen Suche. Wer die nächste Erhebung fährt, sucht
nach der Aussage und nicht nach ihrer Schreibweise — so, wie es der Modulkopf von
`crate::quellbaum` für jede Zählprobe dieses Baums schon vorschreibt.

**Die Zahl vier ist seit dem 260818 gemessen** und steht in
`zulaessigkeit::tests::waehrend_eines_blattes_kommen_genau_diese_vier_durch`. Jede Prosastelle,
die von vier spricht, hat dort ihren Beleg, und eine fünfte Zulassung lässt die Probe rot
werden.

---
Abgleich 260818 (ontocoder, Baumstand `48bb57f`): **die eigene Zeile des Datensatzes ist
nachgezogen, `CLAUDE.md:124` bleibt offen, und der Datensatz bleibt es damit auch.**

Von den sechs Trägern der Erhebung im Abschnitt darüber sind jetzt fünf behoben. Der Träger
`resources/default-keymap.toml`, den allein dieser Datensatz nennt, sagt nicht mehr „jeden
Befehl ausser dem Abbruch", sondern nennt die vier: den Abbruch über
`kommandos::operationen::waehrend_blatt_erlaubt` und die drei der Ausnahmeliste
`kommandos::zulaessigkeit::immer_erreichbar`, also „beenden", „fenster_schliessen" und
„fenster_einblenden". Belegt ist der Wortlaut an der Probe
`zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch`, die seit `441da86` die
tatsächlich durchgelassenen Kommandos zählt und ihre Namen ausschreibt; der Absatz nennt sie
jetzt selbst, damit die nächste Lesung nicht wieder eine Zahl aus einem Kommentar übernimmt.

Der Schluss, den der Absatz zieht, steht weiter und steht jetzt auf der vollständigen Prämisse:
keiner der drei zusätzlich zugelassenen Befehle liegt ab Werk auf `return`. Nachgezählt in
derselben Datei, nicht aus einem Kommentar übernommen: `beenden` trägt `cmd+q` (`:995`),
`fenster_schliessen` trägt `shift+cmd+w` (`:561`) und `fenster_einblenden` trägt `cmd+n`
(`:552`).

**Offen bleibt `CLAUDE.md:124`.** Die Datei gehört nicht zum Auftrag dieses Executors, und ein
halb erledigter Datensatz wird nicht geschlossen. Wer sie nachzieht, schließt diesen Datensatz.

`make check` — Exit 0.

---
Resolved 260818-0350 (coder): **`CLAUDE.md:124` ist nachgezogen, und damit sind alle sechs
Träger der Tabelle darüber erledigt.**

Der Satz sagte, `Anwendungsdelegierter::kommando_ausfuehren` weise „jedes Kommando außer dem
Abbruch" ab. Er sagt jetzt, es weise jedes Kommando ab bis auf vier, und schreibt sie aus: den
Abbruch über `kommandos::operationen::waehrend_blatt_erlaubt`, dessen Rumpf diese eine Zeile
ist, und die drei der Ausnahmeliste `kommandos::zulaessigkeit::immer_erreichbar`, nämlich
`Beenden`, `FensterSchliessen` und `FensterEinblenden`. Die Halbaussage über die eine Zeile in
`waehrend_blatt_erlaubt`, die der alte Satz danebenstellte, ist in denselben Nebensatz
gewandert und steht nicht mehr für sich.

**Belegt ist der Wortlaut an der Probe, nicht an einem Kommentar.**
`zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` (`zulaessigkeit.rs:661`)
filtert `Kommando::KENNUNGEN` durch `zulaessig` bei stehendem Blatt, behauptet die Länge vier
und schreibt die vier Namen einzeln aus; eine fünfte Zulassung lässt sie rot werden. Der neue
Absatz nennt die Probe selbst, damit die nächste Lesung die Zahl dort holt und nicht wieder aus
einer Prosastelle. Die Formulierung folgt `resources/default-keymap.toml` (Commit `b0eee2c`),
statt eine dritte Fassung daneben zu setzen.

**Die Erhebung noch einmal gefahren**, mit beiden Nadeln des Abgleichs 260818 über
`crates/ xtask/ resources/ CLAUDE.md README.md Makefile idea.txt Cargo.toml .claude/`. Keine
Stelle trägt mehr eine verkürzte Fassung. Was die Nadeln noch finden, ist gelesen und **kein**
Träger: `editor.rs:1306` erzählt die eigene Berichtigung („Bis zum 260818 stand hier …"),
`anwendung.rs:406` spricht vom Stand bis S16 und sagt es, `anwendung.rs:6440` und
`zulaessigkeit.rs:614` nennen die Ausnahmeliste vollständig, `belegung.rs:638` und `:952` sowie
`operationen.rs:1277` sprechen über je ein einzelnes Kommando und leiten richtig aus
`waehrend_blatt_erlaubt` ab.

Tabelle der sechs Träger: alle sechs behoben. Der letzte war `CLAUDE.md:124`.

`make check` — Exit 0.
