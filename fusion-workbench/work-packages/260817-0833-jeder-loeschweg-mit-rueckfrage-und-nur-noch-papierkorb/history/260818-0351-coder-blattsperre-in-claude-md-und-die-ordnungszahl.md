# Die letzten zwei Befunde: die Blattsperre in CLAUDE.md und die Ordnungszahl im Modulkopf

**Status:** Complete
**Agent:** coder
**Datum:** 260818-0351
**Baumstand vor der Arbeit:** `b0eee2c`
**Abnahme:** `make check` — Exit 0

## Was der Auftrag war

Die beiden letzten offenen Befunde der Runde schließen. Beide sind Prosabefunde, beide in
zwei Dateien:

1. `CLAUDE.md:124` trug die verkürzte Blattsperre — der sechste und letzte Träger der Tabelle
   aus `issues/260817-1419_*`.
2. `crates/krk-ui/src/kommandos/zulaessigkeit.rs`, Modulkopf, nannte einen „dritten" Eintrag
   der Ausnahmeliste, die schon drei führt (`issues/260818-0319_*`).

## Befund 1 — die verkürzte Blattsperre in CLAUDE.md

### Was dastand

> `Anwendungsdelegierter::kommando_ausfuehren` (`krk-ui/src/appkit/anwendung.rs`) weist jedes
> Kommando außer dem Abbruch ab, solange `NSWindow::attachedSheet` ein Blatt meldet; die
> Abfrage dafür ist `blatt_steht` in derselben Datei, die Regel selbst eine einzige Zeile in
> `kommandos::operationen::waehrend_blatt_erlaubt`.

### Was jetzt dasteht

> `Anwendungsdelegierter::kommando_ausfuehren` (`krk-ui/src/appkit/anwendung.rs`) weist jedes
> Kommando ab bis auf vier, solange `NSWindow::attachedSheet` ein Blatt meldet: den Abbruch
> über `kommandos::operationen::waehrend_blatt_erlaubt`, dessen Rumpf diese eine Zeile ist,
> und die drei der Ausnahmeliste `kommandos::zulaessigkeit::immer_erreichbar`, die die
> Blattsperre ausdrücklich mit aufhebt, nämlich `Beenden`, `FensterSchliessen` und
> `FensterEinblenden`. Die Abfrage nach dem Blatt ist `blatt_steht` in derselben Datei.
> **Welche vier durchkommen, zählt die Probe
> `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch`**, sie schreibt ihre
> Namen aus, und eine fünfte Zulassung lässt sie rot werden; jede Prosastelle, die von vier
> spricht, hat dort ihren Beleg.

Die Nebenaussage des alten Satzes über die eine Zeile in `waehrend_blatt_erlaubt` ist nicht
gestrichen, sondern in den Nebensatz gewandert, der die Funktion ohnehin nennt. Sie stimmt:
`operationen.rs:280-282` trägt genau `kommando == Kommando::Abbrechen`.

### Womit die vier belegt sind

An der Probe und nicht an einem Kommentar.
`zulaessigkeit::tests::waehrend_eines_blattes_kommen_genau_diese_vier_durch`
(`zulaessigkeit.rs:661`) filtert `Kommando::KENNUNGEN` durch `zulaessig` auf einer Lage mit
stehendem Blatt, behauptet `durchgelassen.len() == 4` und prüft die vier Namen einzeln:
`Abbrechen`, `Beenden`, `FensterSchliessen`, `FensterEinblenden`. Ihre Fehlschlagsmeldung sagt
dazu, wer die Liste ändert, ziehe die Prosastellen mit, die ihre Länge nennen.

Die Formulierung ist nicht neu erfunden, sondern von `resources/default-keymap.toml`
übernommen, das dieselbe Stelle in `b0eee2c` schon nachgezogen hat: „weist … jeden Befehl ab
bis auf vier. Es sind der Abbruch über `waehrend_blatt_erlaubt` und die drei der Ausnahmeliste
`immer_erreichbar`". Dieselbe Aussage steht seit T1 in `blaetter/mod.rs:304-311`.

### Die Erhebung noch einmal

Beide Nadeln des Abgleichs 260818 über `crates/ xtask/ resources/ CLAUDE.md README.md Makefile
idea.txt Cargo.toml .claude/`:

```sh
grep -rn "ausser dem Abbruch\|außer dem Abbruch" <Pfade>
grep -rniE "(ausser|außer|bis auf|nur|allein|einzig)[^.]{0,45}(abbruch|abbrechen)" <Pfade>
```

Keine Stelle trägt mehr eine verkürzte Fassung. Sieben Treffer bleiben und sind einzeln
gelesen; **kein** Träger:

| Stelle | warum kein Träger |
|---|---|
| `appkit/editor.rs:1306` | erzählt die eigene Berichtigung: „Bis zum 260818 stand hier …" |
| `appkit/anwendung.rs:406` | spricht vom Stand bis S16 und sagt das |
| `appkit/anwendung.rs:6440` | nennt die Ausnahmeliste, vollständig |
| `kommandos/zulaessigkeit.rs:614` | Probenname, nennt Abbruch **und** Ausnahmeliste |
| `kommandos/operationen.rs:1277`, `:1316` | Aussage über `waehrend_blatt_erlaubt` allein, richtig |
| `krk-core/.../belegung.rs:638`, `:952` | Aussagen über je ein einzelnes Kommando, richtig hergeleitet |

Damit sind alle sechs Träger der Tabelle in `260817-1419` erledigt.

## Befund 2 — die Ordnungszahl im Modulkopf

Gezählt wurde die Liste: `immer_erreichbar` (`zulaessigkeit.rs:197-202`) führt drei Einträge,
`Beenden`, `FensterSchliessen`, `FensterEinblenden`. Gemeint war ein vierter.

**Die Ordnungszahl ist ganz gefallen**, nach dem Muster von `926377f`, und mit ihr eine zweite
Zahl im selben Absatz, die heute noch stimmte:

```text
- alle drei Befehle tragen `Wirkungsbereich::Ueberall`
+ jeder Eintrag der Liste traegt `Wirkungsbereich::Ueberall`

- Mit einem dritten Eintrag, der einen Bereich braucht, fiele der Unterschied an
+ Sobald ein Eintrag der Liste einen Bereich braucht, faellt der Unterschied an

+ Eine Ordnungszahl steht hier bewusst nicht: sie altert mit jedem
+ neuen Eintrag, die Regel darueber nicht.
```

Die Aussage über den Wirkungsbereich ist am Baum nachgeprüft und nicht aus dem alten Satz
übernommen: `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) ordnet alle drei
demselben Zweig `Wirkungsbereich::Ueberall` zu.

**Zwei Ordnungszahlen sind absichtlich stehen geblieben.** „Dass sie den **dritten**
Bestandteil nicht aufhebt" zeigt auf die vier durchnummerierten Bestandteile des Abschnitts
darüber, nicht auf eine wachsende Liste. Und der Doc-Kommentar an `immer_erreichbar` selbst
sagt weiter „Alle drei Eintraege stammen aus ‚kein Verlust gegenueber heute'" — er steht
unmittelbar über der Liste, die er zählt.

## Was in CLAUDE.md sonst geprüft wurde

Der Auftrag verlangte, weitere durch die Runde falsch gewordene Aussagen zu melden statt zu
ändern. Geprüft und **haltend**:

- die drei Aufzählungszahlen in Zeile 74: `Wirkungsbereich` sieben, `Bereich` fünf, `Fokus`
  fünf, je am Baum nachgezählt;
- die zwei `#![allow(unsafe_code)]` in Zeile 88: es sind weiter `verzeichnis/sys.rs` und
  `appkit/mod.rs`;
- die Untergrenzen-Deckung in Zeile 152: 40 Dateien unter `crates/krk-ui/src/appkit/`, und die
  zwei ohne den Abschnitt sind weiter `koordinaten.rs` und `mod.rs`, also genau die zwei
  begründeten Ausnahmen — die Dateien dieser Runde tragen ihn;
- „hat genau einen Rufer" zur Rückschritt-Regel in Zeile 140: der eine Aufruf steht in
  `anwendung.rs:4537`;
- „am 260815 trägt `Kommando` keine einzige Git-Variante" in Zeile 170;
- der Absatz zur Rückschritt-Taste selbst, der die Rückfrage seit 260817 schon führt.

**Nichts Weiteres gefiled.** Die zwei bekannten falschen Aussagen (die Zahl der gefahrenen
Runden in Zeile 12 und die ausgelieferte Version `v0.4.1` in Zeile 39) sind bereits gefiled und
nach Auftrag nicht erneut zu melden.

## Geänderte Dateien

- `/Users/k1/Projects/productive/krk/CLAUDE.md`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/kommandos/zulaessigkeit.rs`
- `.../issues/260817-1419_c_ein-vierter-traeger-der-verkuerzten-blattsperre-…md` (`_o_` → `_c_`)
- `.../issues/260818-0319_c_der-modulkopf-der-zulaessigkeit-nennt-einen-dritten-eintrag-…md` (`_o_` → `_c_`)

## Abnahme

`make check` — Exit 0.
