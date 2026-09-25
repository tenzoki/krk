# Turn 3: Kurzhinweis statt Blättern, und kein Wort mehr über ein ausgeblendetes Dateifenster

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Decision:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1809_i_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`
**Issue:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1805_c_die-eine-statuszeile-zeigt-meldungen-eines-ausgeblendeten-dateifensters.md`
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 454, nachher 457, davon in `statuszeile.rs` vorher 19, nachher 22

---

## Zwei Aufgaben an derselben Zeile

Die eine nimmt Schritt 11 zurück und ersetzt ihn durch ein leichteres Mittel,
die andere behebt einen Defekt aus der Durchsicht von Turn 2. Sie treffen sich
in `crates/krk-ui/src/appkit/statuszeile.rs` und sonst nirgends.

## 1. Schritt 11 zurückgenommen, Kurzhinweis gebaut

Die `NSScrollView` ist weg. `Statuszeile` hält wieder allein ihr `NSTextField`,
`sicht` gibt es heraus, `breite_nachziehen` und `an_den_anfang` sind gelöscht.

```
vor der Rücknahme                nach der Rücknahme
─────────────────                ──────────────────
Statuszeile                      Statuszeile
  rolle: NSScrollView ─> sicht()   feld: NSTextField ─> sicht()
    └─ documentView                     + toolTip, wenn gekürzt wird
         feld: NSTextField
```

**Die Messung ist dieselbe geblieben, ihr Zweck nicht.** In Schritt 11 brachte
`sizeToFit` die Dokumentansicht auf die Breite ihres Textes; jetzt fragt
derselbe Ruf nur noch, wie breit der Text *wäre*.
`Statuszeile::abgeschnitten` merkt sich dafür den Rahmen, ruft `sizeToFit`,
liest die Breite und setzt den Rahmen unmittelbar wieder zurück — ganz, denn
`sizeToFit` schreibt auch die Höhe, und die Schrifthöhe der kleinen
Systemschrift liegt unter den 18 Punkten der Zeile. Verglichen wird gegen die
Breite, die das Feld im Fenster hat; beide Zahlen kommen aus derselben Zelle,
also entscheidet der Vergleich dieselbe Frage, die AppKit beim Kürzen
entscheidet.

**Der Hinweis kommt aus dem Feld und nicht aus dem Aufrufer.**
`Statuszeile::kurzhinweis_nachziehen` liest `stringValue()` und setzt
`setToolTip:` genau dann, wenn gekürzt wird; was der Hinweis zeigt, ist damit
der Bauart nach das, was in der Zeile steht, statt ein zweites Argument, das
damit übereinstimmen müsste. Gerufen wird es einmal in `zeigen`, **hinter**
beiden Zweigen: eine geleerte Zeile hat nichts abzuschneiden, also räumt
derselbe Ruf einen stehenden Hinweis dort ab, ohne einen eigenen Zweig zu
brauchen.

Der Zuschnitt ist von `crates/krk-ui/src/appkit/bereichsleiste.rs:474`
übernommen, dem einen vorhandenen Kurzhinweis des Baums. Eine zweite Art, einen
Hinweis zu setzen, entsteht nicht.

## Was mit der Bildlaufansicht zurückkommt

**Der Gestenklau ist weg.** Ein Zweifingerstrich über den achtzehn Punkten am
Fensterfuß bewegt wieder die Liste darüber und nicht die Zeile.

**C5.11 ist zurück auf seiner Grundlage vor Schritt 11.** Ohne Bildlaufansicht
gibt es keine `NSScroller`, und die Frage betrifft wieder allein das Textfeld,
das über `labelWithString:` entsteht und nicht auswählbar ist. Der Modulkopf
sagt das aus und sagt daneben, dass die Frage damit **nicht beantwortet** ist:
sie bleibt offen und ist am Bündel abzunehmen, mit eingeschalteter
vollständiger Tastaturbedienung.

**Die Untergrenzen sind nachgezogen.** `NSScrollView`, `NSClipView` und
`NSBorderType` stehen nicht mehr im Abschnitt, weil die Klassen nicht mehr im
Code stehen. An ihrer Stelle: `toolTip` (`NSView.h:310`), `stringValue`
(`NSControl.h:36`), `sizeToFit` (`NSControl.h:44`) und `frame`
(`NSView.h:129`), alle vier am SDK gegengelesen und ohne eigene
`API_AVAILABLE`, also seit 10.0. Die höchste Untergrenze der Datei ist wieder
`labelWithString:` mit 10.12.

`crates/krk-ui/src/appkit/fenster.rs` ist auf seinen Stand vor Schritt 11
zurück: die Skizze im Modulkopf nennt die Zeile wieder nur „18 pt, volle
Breite", und der Absatz über den Einzug spricht wieder vom Textfeld. Code stand
dort ohnehin keiner.

## Der Preis, benannt statt entdeckt

Gemessen wird beim Setzen des Textes. Zieht der Nutzer das Fenster danach
breiter oder schmaler, ohne dass eine neue Meldung kommt, steht der Hinweis
oder fehlt er nach der alten Breite. Aufgeschrieben als
`issues/260812-1854_o_der-kurzhinweis-der-statuszeile-veraltet-bei-einer-fensteraenderung.md`,
mit drei Zuschnitten und dem Grund, warum keiner hier gewählt ist: der eine
Auslösepunkt einer Breitenänderung wäre `setFrameSize:` am Feld, und ihn zu
überschreiben verlangte eine eigene Klasse über `NSTextField` — die ließe sich
nicht mehr über `labelWithString:` bauen und kostete damit genau die Grundlage,
auf der C5.11 heute ruht. Derselbe Tausch, den der Entscheid vom 260812 für die
Bildlaufansicht abgelehnt hat, an einer anderen Stelle.

## 2. Die Zeile schweigt über ein ausgeblendetes Dateifenster

Gewählt ist Zuschnitt 2 aus dem Defektdatensatz: die Sichtbarkeit reist in
`statuszeile::zeile` hinein, statt beim Aufrufer eine Bedingung zu ziehen.

```rust
pub fn zeile<'a>(
    links: &'a Quellen,
    rechts: &'a Quellen,
    aktiv: Fensterseite,
    sichtbar: &Sichtbarkeit,
) -> Option<Meldung<'a>>
```

Die Schleife über die beiden Seiten überspringt, wer nicht dasteht. Gefragt wird
über `fenstermodell::sichtbar_in` und `Bereich::von_seite`, also über die eine
Zuordnung von einem Bereich auf sein Feld; eine zweite Zuordnung von einer
`Fensterseite` auf ein `bool` entsteht nicht.

**Warum nicht Zuschnitt 1.** Zwei Zeilen in
`Anwendungsdelegierter::statuszeile_nachziehen` hätten C5.8 ebenso wörtlich
gemacht — und wären von keiner Probe zu erreichen gewesen. Genau daran ist
dieser Defekt vorbeigelaufen: die Probe der Datei setzte die Voraussetzung
(`Quellen::default()` für das ausgeblendete Dateifenster), die das Programm
nicht herstellte, und wäre grün geblieben, wenn jemand den Fall behoben oder
verschlimmert hätte. Was ohne Fenster prüfbar ist, gehört geprüft.

**C5.8 stimmt damit wieder wörtlich**, und zwar der zweite Satz. Er wird von
zwei Zusagen zusammen getragen: ein ausgeblendetes Dateifenster bewirbt sich
nicht, und das aktive ist immer sichtbar. Die zweite stellt das Modell an beiden
Wegen dorthin her — `Fenstermodell::umschalten` gibt die Aktivität ab, wenn das
aktive ausgeblendet wird, `Fenstermodell::aus_sitzung` zieht sie nach, wenn eine
von Hand geschriebene `session.toml` sie auf ein ausgeblendetes zeigen lässt.
Beide Stellen sind unverändert; hier wird auf sie verwiesen und keine dritte
gebaut.

**C5.7 bleibt unangetastet.** Die vier Meldungsfelder der ausgeblendeten Seite
werden nicht geräumt, und ihre Meldung steht wieder in der Zeile, sobald der
Bereich zurückkommt. Eine der drei neuen Proben misst genau das.

`statuszeile_nachziehen` holt `aktiv` und `sichtbarkeit()` in **einer**
Ausleihe, wie `bereichsleiste_nachziehen` daneben es hält, und fragt weiterhin
beide Dateifenster nach ihren Quellen; verworfen wird die Antwort des
ausgeblendeten dort, wo die Regel steht. Der Kommentar an
`aufteilung_nachziehen`, der bis zum 260812 ausdrücklich sagte, die Statuszeile
werde „nicht wegen der Sichtbarkeit" nachgezogen, nennt jetzt beide Gründe —
und für die Sichtbarkeit ist jener Nachzug sogar der einzige Weg.

## Proben

22 in `statuszeile.rs` gegenüber 19, keine der neunzehn ist weggefallen.

- `steht_nur_ein_dateifenster_traegt_kein_satz_einen_zusatz` **misst** die
  Voraussetzung jetzt, statt sie zu setzen: das ausgeblendete Dateifenster
  bekommt einen Quellensatz, und zwar einen mit dem *höheren* Rang.
- `die_meldung_eines_ausgeblendeten_dateifensters_steht_nicht_in_der_zeile` geht
  den Weg des Defekts Schritt für Schritt.
- `die_meldung_kommt_mit_dem_eingeblendeten_dateifenster_zurueck` hält C5.7 fest.
- `auf_jedem_rang_bewirbt_sich_allein_das_sichtbare_dateifenster` läuft über
  alle fünf Ränge in beide Richtungen.

Der Testhelfer, der eine Sichtbarkeit mit einem ausgeblendeten Dateifenster
herstellt, prüft sich selbst gegen `sichtbar_in`, damit die Probe die Zuordnung
nicht ein zweites Mal aufschreibt und gegen eine falsche prüfen könnte.

**Keine Probe für den Kurzhinweis.** `abgeschnitten` und
`kurzhinweis_nachziehen` messen an einer Instanz und brauchen den Hauptfaden,
den `libtest` nicht hergibt; das ist der Zustand aus
`issues/260810-1001_*`, und dieser Schritt vermehrt ihn nicht. Abzunehmen ist
der Hinweis am Bündel.

## Was am Bündel zu sehen ist

- Eine Meldung, die breiter ist als das Fenster, wird gekürzt und zeigt sich
  beim Verweilen mit dem Zeiger vollständig in einem Kurzhinweis.
- Passt sie hinein, erscheint kein Hinweis.
- Ein Zweifingerstrich über der Zeile bewegt wieder die Liste darüber.
- Kein Rollbalken mehr am Fensterfuß.
- Blendet der Nutzer ein Dateifenster aus, verschwindet dessen Meldung aus der
  Zeile und kommt beim Einblenden zurück. Solange nur eines steht, trägt kein
  Satz mehr den Zusatz „linkes/rechtes Dateifenster:".

## Geänderte Dateien

- `crates/krk-ui/src/appkit/statuszeile.rs` — Rücknahme der Bildlaufansicht,
  `abgeschnitten` und `kurzhinweis_nachziehen`, die Sichtbarkeit in `zeile`,
  Modulkopf und Untergrenzen, drei neue Proben und eine umgeschriebene.
- `crates/krk-ui/src/appkit/fenster.rs` — Modulkopf und zwei Doc-Absätze an
  `fensterinhalt` auf den Stand vor Schritt 11. Kein Code.
- `crates/krk-ui/src/appkit/anwendung.rs` — `statuszeile_nachziehen` reicht die
  Sichtbarkeit durch; ein Kommentar an `aufteilung_nachziehen` nachgezogen.
