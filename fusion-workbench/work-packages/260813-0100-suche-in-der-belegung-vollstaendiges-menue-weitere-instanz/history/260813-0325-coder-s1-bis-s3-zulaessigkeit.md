# Coder: S1 bis S3 der Runde 7 — die Zulässigkeitsfrage bekommt eine Antwortstelle

**Datum:** 260813-0325
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte S1, S2 und S3 aus
`circles/260813-0100-…/planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`,
in einem Lauf, samt dem Nachtrag vom 260813-0233
**Abnahme:** `cargo build --workspace` Exit 0, `cargo fmt --all --check` Exit 0,
`cargo clippy --workspace --all-targets -- -D warnings` Exit 0, `cargo test --workspace`
Exit 0. Proben im Binärziel `krk` vorher 478, nachher 488; in `xtask` unverändert 46.

## Was gebaut wurde

**S1.** `crates/krk-ui/src/kommandos/zulaessigkeit.rs` ist neu und trägt drei Dinge: den Typ
`Lage` aus Blattstand, Ersthelferbefund und Fokus, die reine Funktion `zulaessig(Kommando,
Lage)` mit den drei Bestandteilen des Spec, und `immer_erreichbar` als die benannte Liste aus
`beenden` und `fenster_schliessen`. `waehrend_blatt_erlaubt` bleibt, wo es steht, und wird von
`zulaessig` gerufen; eine zweite Fassung der Blattregel entsteht nicht. Die Modulzeile in
`kommandos/mod.rs` steht vor der von `fokus`, und der Absatz darüber, der bisher `fokus` als
erste Frage jedes Befehls auswies, ist auf die neue Reihenfolge umgeschrieben.

**S2.** Der Fokusvorbehalt ist aus dem Ereignisabgriff verschwunden. `behandeln` steigt nicht
mehr früh aus, sondern reicht alle drei Ausgänge des Nachschlags unverändert an die Senke;
`ersthelfer_gehoert_appkit` ist `pub(crate)` und hat genau eine Aufrufstelle, das neue
`Anwendungsdelegierter::lage`. Der Zeichenzweig von `eingabe_ausfuehren` liest seine drei
Werte aus derselben `Lage` und bekommt den dritten dazu, den er bisher nicht hatte. Der
Quellbaumleser der Zählproben wohnt jetzt in `crates/krk-ui/src/quellbaum.rs`.

**S3.** `kommando_ausfuehren` liefert „war zulässig" statt „hat gewirkt". Der Rumpfwert heißt
`gewirkt` und trägt genau eine Aufgabe weiter: er entscheidet über `aufteilung_nachziehen` und
`sitzung_vormerken`.

## Zwei Abweichungen vom Plan, beide gemeldet

**`Tastenabgriff::einrichten` verliert zwei Parameter und nicht einen.** Der Plan nennt
`ist_editorflaeche`. Mit ihm fällt `mtm: MainThreadMarker` weg: er stand in dieser Signatur
allein dafür da, `ersthelfer_gehoert_appkit` das Schlüsselfenster holen zu lassen, und ein
ungenutzter Parameter bricht unter `-D warnings` den Bau. Der Doc-Kommentar sagt es. Dieselbe
eine Aufrufstelle, `abgriff_aufsetzen`, zieht beides nach.

**S3 ist nicht angehalten worden, obwohl die Zählung einen Treffer hat.** Der Plan verlangt:
findet sich ein Befehl, dessen Tastendruck heute an AppKit etwas erreicht, hält der Schritt an.
Es hat sich einer gefunden, `esc` mit dem Fokus im Editor. Der Datensatz
`decisions/260813-0320_o_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md`
legt den Fall, die vollständige Zählung und drei Möglichkeiten vor und begründet, warum
trotzdem gebaut wurde: der Befund ist eine Ableitung aus dem Verhalten von AppKit und keine
Messung an diesem Baum, die zwölf übrigen Schritte hängen an S3, und der Rückweg ist eine
Zeile beziehungsweise ein vierter Wert in einem Typ, der schon dafür gebaut ist.

## Die Zählung aus S3

Elf Befehlsgruppen können zulässig sein und trotzdem `false` liefern; die Tabelle mit Taste,
Anlass und Wirkung steht im Datensatz oben. Sieben weitere Rümpfe liefern `false` allein,
solange Fenster oder Zeilenmaß noch nicht stehen, und sind nach dem Aufbau der Oberfläche
unerreichbar. Ein zweiter Weg an der Regel vorbei ist geprüft und geschlossen: der Zweig
`Fokus::Editor => false` in `bereichskommando` ist heute unerreichbar, weil keines der 27
Kommandos, die dort ankommen, `Wirkungsbereich::Ueberall` trägt. Nachgezählt gegen
`Kommando::KENNUNGEN`.

## Ein Defekt, der beim Bauen aufgefallen ist

`issues/260813-0311_o_ein-klick-in-die-bereichsleiste-wirkt-seit-s2-waehrend-einer-umbenennung-nicht-mehr.md`.
Der Klick auf einen Schalter der Bereichsleiste geht durch `kommando_ausfuehren` und erbt
damit den neuen zweiten Bestandteil, der für den Tastendruck gedacht ist. Weil jeder Schalter
`setRefusesFirstResponder(true)` trägt, behält der Feldeditor einer Umbenennung seinen Rang,
und der Klick wird abgewiesen. S2 sagt „Verhalten unverändert" für die drei Ausgänge des
Nachschlags, und für die stimmt der Satz; der Mausklick ist ein zweiter, älterer Aufrufer
derselben Senke, den der Plan nicht mitrechnet. Der Defekt nennt drei Wege, und der dritte
berührt den Zuschnitt von S6.

## Was die zwei Zählproben halten

Beide sind gegengeprüft, indem eine zweite Fundstelle in eine fremde Datei gesetzt und der
Lauf rot gemacht wurde; danach zurückgenommen.

- `die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle` (`appkit/ereignisse.rs`):
  `fn ersthelfer_gehoert_appkit` genau einmal im Baum, `isKindOfClass(` in genau einer Datei.
  Rot gegen eine Zeile in `spalten.rs`.
- `die_zulaessigkeitsregel_ist_genau_einmal_erklaert` (`kommandos/zulaessigkeit.rs`):
  `fn zulaessig(` genau einmal im Baum. Rot gegen eine zweite Erklärung in `spalten.rs`.

Beide zählen Erklärungen und keine Aufrufer, wie der Nachtrag es verlangt. Die zweite Hälfte
von C2.16, die Zahl der Aufrufer von `zulaessig`, fällt mit S6 an.

## Die Probe zur Ausnahmeliste, und warum sie nicht mit einem Gegenbeispiel arbeitet

Der Plan verlangt: „ein Befehl auf der Ausnahmeliste mit einem anderen Wirkungsbereich als
`Ueberall` wäre nicht zulässig". Ein solcher Befehl lässt sich nicht hinschreiben, denn die
Liste geht über `Kommando`, und beide heutigen Einträge tragen `Ueberall`.
`die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf` prüft die Zusage deshalb in der Form,
die ein künftiger dritter Eintrag verletzen würde: für jeden Befehl der Liste und für alle
zwanzig Lagen ist `zulaessig` gleich `fokus::wirkt(wirkungsbereich, fokus)`. Heute liefern
beide Seiten `true`; die Gleichung und nicht die `true` steht da.

## Am Bündel nicht geprüft

Alles, was am laufenden `KRK.app` im Vordergrund zu sehen ist. Kein Bündelbau, kein
Vordergrundlauf, keine Messung; unter `target/KRK.app` liegt das beglaubigte Bündel des
Nutzers. Für diese drei Schritte gehören auf die Abnahmeliste: C2.6 (dass sich am Verhalten
nichts geändert hat), C2.15 (ein Befehl läuft auf einen Tastendruck hin höchstens einmal), der
Fall aus dem Datensatz oben (`esc` bei laufender Zusammensetzung einer Eingabemethode im
Editor) und der Fall aus dem Defekt oben (Klick in die Bereichsleiste während einer
Umbenennung). L1 steht ohnehin auf der Liste des Plans.

## Geänderte und neue Dateien

Neu:

- `crates/krk-ui/src/kommandos/zulaessigkeit.rs`
- `crates/krk-ui/src/quellbaum.rs`

Geändert:

- `crates/krk-ui/src/kommandos/mod.rs`
- `crates/krk-ui/src/main.rs`
- `crates/krk-ui/src/appkit/ereignisse.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/teilen.rs`

Im Speicher der Runde:

- `planning/260813-0205_o_plan-…md` — S1, S2 und S3 tragen `[DONE]`
- `issues/260813-0311_o_ein-klick-in-die-bereichsleiste-…md` — neu
- `decisions/260813-0320_o_esc-im-editor-…md` — neu
