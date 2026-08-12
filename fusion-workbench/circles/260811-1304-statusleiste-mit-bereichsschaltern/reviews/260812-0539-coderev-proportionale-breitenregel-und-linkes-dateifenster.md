# Code-Durchsicht: die proportionale Breitenregel und das ausblendbare linke Dateifenster

**Datum:** 260812-0539
**Sender:** coderev
**Reviewed-range:** `5aa22df..8ffaac2`
**Not-opened:** none
**Maßstab:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`,
Fähigkeiten C4 und C5, Implementierungsschritte 1 bis 3
**Abnahme nachgefahren:** `cargo clippy --workspace --all-targets` Exit 0,
`cargo test --workspace` Exit 0 (alle Prüfziele grün)

---

## Zusammenfassung

Die drei Schritte liefern, was der Plan verlangt: die Anteilsregel terminiert, ihre Summe ist in
beiden Zweigen exakt die verfügbare Breite, die Abweisung an den Mindestbreiten greift nur beim
Einschalten und lässt keinen Zustand entstehen, aus dem heraus nichts mehr aufginge, und die drei
Zusicherungen in `aus_sitzung` stehen in der richtigen Reihenfolge. Die Zahlen in den neuen Proben
sind gerechnet und nicht geraten; alle sieben, die ich nachgerechnet habe, gehen auf.

**Vier Befunde, zwei davon von Gewicht**, und beide sitzen an derselben Stelle: an der Grenze
zwischen dem, was auf dem Schirm steht, und dem, was gespeichert wird. Unter der Anteilsregel ist
diese Abbildung nur so lange umkehrbar, wie kein Bereich an seinem Mindestmaß hängt. Wo sie es nicht
ist, kippt einmal eine Deckelung und einmal die Aufteilung des Nutzers.

**Zählung:** kritisch 0, hoch 0, mittel 2, niedrig 2.

---

## Was ich nachgerechnet und bestätigt habe

Die vier Fragen des Auftrags an die Rechnung, jede mit dem Weg, auf dem ich sie beantwortet habe.
Nachgerechnet ist mit einem Nachbau der Funktionen aus dem Stand `8ffaac2` in einer eigenen Datei
außerhalb des Baums; am Baum ist nichts angefasst.

**Die Wasserstandsrechnung terminiert, in jedem Fall.** Jeder Durchgang nimmt mindestens einen
Bereich heraus, weil `zu_klein` sonst leer wäre und die Schleife zurückkehrte. Die Menge der offenen
Bereiche wird also echt kleiner, und mehr als fünf Durchgänge kann es nicht geben. Der Fall, den der
Kommentar an `fenstermodell.rs:900` offenlässt — alle fallen zugleich heraus —, tritt nicht ein:
fielen alle heraus, wäre `rest` kleiner als die Summe der Mindestbreiten der offenen, und diese
Ungleichung ist die Verneinung der Schleifeninvariante `rest >= Mindestsumme der offenen`, die vom
ersten Zweig hergestellt und von jedem Durchgang erhalten wird. Der Abbruch über die leere Liste ist
damit toter Code, und das ist gut so: er verlöre den Restbetrag.

**Die Summe trifft die verfügbare Breite in beiden Zweigen.** Über 200 000 zufällige Eingaben
(Sichtbarkeit, gespeicherte Breiten einschließlich sehr kleiner, Gesamtbreiten von 0 bis 4000,
Trennlinien von 0 bis 11) beträgt die schlimmste Abweichung 4,5·10⁻¹³ Punkte. Der Kunstgriff in
`anteilig`, dem letzten Bereich den Rest statt seines gerundeten Anteils zu geben, trägt.

**Kein sichtbarer Bereich bekommt 0, solange die Zeile positiv ist.** Über dieselben 200 000
Eingaben kein einziger Fall. Der Grund steht im Code: `wunsch` deckelt nach unten auf 1,0, und die
Gewichte des zweiten Zweigs sind die Mindestbreiten, alle positiv.

**Die zweite Abweisung greift nur beim Einschalten**, und ihre Bedingung
(`mindestsumme <= verfuegbar`) ist die genaue Verneinung der Zweigbedingung in `bereichsbreiten`
(`verfuegbar < mindestsumme`). Die Fallunterscheidung ist damit überschneidungsfrei und
vollständig. Eine Folge von Befehlen, nach der kein Bereich mehr einschaltbar wäre, obwohl Platz
ist, kann es nicht geben: die Prüfung liest allein die Mindestbreiten der Menge nach dem Befehl und
die Fensterbreite, hält also keinen Zustand, in dem sie sich merken könnte, dass sie schon einmal
abgewiesen hat.

**Die Reihenfolge der drei Zusicherungen in `aus_sitzung` ist richtig.** Die erste stellt her, was
die zweite braucht (ein sichtbares Dateifenster für `aktiv.andere()`); die dritte fasst weder die
Dateifenster noch das aktive an. Keine hebt eine andere auf. Die Probe
`eine_sitzung_ohne_sichtbares_dateifenster_holt_das_linke_hervor` fährt beide Werte von `aktiv` und
misst genau diese Abhängigkeit.

**Die Zahlen in den Proben sind gerechnet.** Nachgerechnet und aufgegangen:
`der_eingeblendete_editor_bekommt_seinen_anteil` (1280 auf 1480 Wunsch, Faktor 32/37),
`am_engen_fenster_gewinnt_das_mindestmass_gegen_den_anteil` (Editor auf 320, Rest 700 auf 1020),
`die_leiste_schrumpft_mit_dem_editor` (beide Lagen), `kein_bereich_faellt_unter_sein_mindestmass`
(Leiste auf 120, Vorschau bei 160,727 knapp über ihren 160),
`unter_der_summe_der_mindestbreiten_schrumpfen_alle_mit_demselben_faktor` (600/760),
`jeder_sichtbare_bereich_bekommt_seinen_anteil_ohne_zweite_aufzaehlung` (1600 auf 1280, das
1,25-Fache) und `das_verhaeltnis_zweier_bereiche_ueberlebt_das_einblenden_eines_dritten`. Keine
Probe rechnet den Code nach; jede schreibt die Zahlen aus und begründet sie im Kommentar.

**Die Kommentare sind sorgfältig nachgezogen.** Die sechs Suchbegriffe des Auftrags ("weicht",
"Vorrang", "260808", "das linke", "nie `false`", "Punktzahl") liefern über den ganzen Baum keine
Stelle mehr, die die alte Welt behauptet; wo die Begriffe stehen, stehen sie als Rückblick
("Bis zur Bereichsleisten-Runde …"). Übersehen sind zwei Kleinigkeiten, beide unten als Befund 3
und 4.

---

## Befunde

### Befund 1 (mittel): Die Breitenbefehle aus C7 wirken unter der Mindestsumme in die falsche Richtung

**Datensatz:** `issues/260812-0539_o_die-breitenbefehle-aus-c7-wirken-unter-der-mindestsumme-in-die-falsche-richtung.md`
**Ort:** `crates/krk-ui/src/fenstermodell.rs:598` (`breite_aendern`), zusammen mit `:645`
(`massstab`) und `:692` (`breiten_uebernehmen`)
**Betrifft:** `krk-ui`, Kriterium C4.9

Steht das Fenster schmaler als die Summe der Mindestbreiten — bei sichtbarem Editor auf der
Mindestgröße von 780 Punkten ist das der Fall, 920 gegen 777 —, dann tun `opt+cmd+links` und
`opt+cmd+rechts` auf einem Dateifenster dasselbe, und beide das Gegenteil ihres Namens.
Nachgerechnet:

```
gespeichert vor dem Befehl:  [193.04, 386.09, 386.09, 260.00, 514.78]
nach opt+cmd+rechts:         [193.04, 457.14, 315.03, 260.00, 514.78]
nach opt+cmd+links:          [193.04, 457.14, 315.03, 260.00, 514.78]
```

71,05 Punkte statt der 40, die C4.9 zusagt, und in beiden Richtungen dieselben. Auf dem Schirm
bewegt sich nichts, weil der zweite Zweig die Wünsche nicht liest; sichtbar wird es erst beim
Wiederaufziehen, wo die beiden Dateifenster dann 457 zu 315 stehen statt gleich breit.

Die Ursache ist die Deckelungskette

```rust
betrag.min(dort - mindestmass(anderer)).max(mindestmass(bereich) - hier)
```

Sie setzt voraus, dass die untere Schranke nicht über der oberen liegt, sonst gewinnt `.max()` und
das Vorzeichen des Betrags spielt keine Rolle mehr. Die Voraussetzung hält, weil
`breiten_uebernehmen` sonst gespeicherte Breiten über dem skalierten Mindestmaß hinterlässt — im
zweiten Zweig aber liegen sie um denselben Faktor darunter. Der Kommentar an `breite_aendern`
("Am Mindestmass hoert der Schritt auf, statt es zu unterschreiten") sagt für diesen Fall das
Gegenteil des Codes.

Die Kette hat daneben keinen Boden bei 0. Erreichbar wird eine negative gespeicherte Breite bei
`mindestsumme > 2 × verfuegbar`; bei den ausgelieferten 780 Punkten Mindestbreite und einer
größtmöglichen Mindestsumme von 920 nicht, wohl aber über eine von Hand geschriebene
`session.toml`, deren Breiten stark auseinanderliegen. `Breiten` prüft beim Einlesen keine Zahl ab.

`der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt` misst bei 1280, 1400 und 1920
Punkten, also nur dort, wo nichts gedeckelt ist. C4.9 ist damit für den zweiten Zweig unbelegt.

**Nicht im Vorbeigehen zu beheben:** der eine Weg ist eine Vorabfrage, ob die beiden skalierten
Mindestmaße überhaupt nebeneinander passen, der andere das Heben von `MINDESTGROESSE` auf 940
Punkte — und das ist bereits die offene Nutzerfrage
`decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`.

### Befund 2 (mittel): Ein zusammengezogenes Fenster ersetzt die Aufteilung des Nutzers dauerhaft

**Datensatz:** `issues/260812-0539_o_ein-zusammengezogenes-fenster-ersetzt-die-aufteilung-des-nutzers-dauerhaft.md`
**Ort:** `crates/krk-ui/src/appkit/aufteilung.rs:151` (`neu_auslegen`) und
`crates/krk-ui/src/fenstermodell.rs:692` (`breiten_uebernehmen`)
**Betrifft:** `krk-ui`, Kriterium C4.7

`splitView:resizeSubviewsWithOldSize:` speist bei jedem Bild die gemessenen Breiten wieder als
Wünsche ein. Solange nichts gedeckelt ist, ist das idempotent, weil die Abbildung ein einheitlicher
Faktor ist. Sobald gedeckelt wird, ist sie es nicht mehr. Ein Hin und Her am Fensterrand genügt:

```
bei 1280:            [155.31, 362.39, 362.39, 0.00, 396.91]
auf  780 gezogen:    [101.35, 202.70, 202.70, 0.00, 270.26]
wieder auf 1280:     [166.57, 333.13, 333.13, 0.00, 444.17]
```

Die Dateifenster verlieren 8,1 Prozent, der Editor gewinnt 11,9, und das neue Verhältnis ist genau
das der Mindestbreiten. Der nächste beliebige Befehl trägt es über
`bildschirmbreiten_uebernehmen` in das Modell und damit in `session.toml`.

Die Risikotafel des Plans nennt den Kern und nimmt ihn an, aber in einem engeren Zuschnitt: sie
spricht von *einem* gedeckelten Bereich und setzt ein "Nachlesen", also einen Befehl, voraus. Hier
sind es alle sichtbaren zugleich, und es braucht keinen Befehl. Der Kommentar an `MINDESTGROESSE`
hält den Unterschied für den Schirm ausdrücklich fest; dass er auch für die gespeicherten Zahlen
gilt, steht nirgends.

Die Probe `das_vergroessern_des_fensters_laesst_die_gespeicherten_breiten_stehen` misst von 1280
auf 2000 Punkte und damit nur den ungedeckelten Fall. C4.7 hält dort und nur dort.

### Befund 3 (niedrig): Die Zuordnung von Bereich auf Sichtbarkeit steht zweimal gleichlautend da

**Datensatz:** `issues/260812-0539_o_die-zuordnung-von-bereich-auf-sichtbarkeit-steht-seit-schritt-3-zweimal-gleich-da.md`
**Ort:** `crates/krk-ui/src/fenstermodell.rs:238` und `crates/krk-ui/src/appkit/aufteilung.rs:454`

Schritt 3 hat `aufteilung::sichtbar_im` den Zweig `Bereich::Links => true` genommen. Damit sind die
beiden Funktionen Zeile für Zeile dieselbe Fallunterscheidung, und der einzige Grund für zwei
Fassungen ist entfallen. Der Kommentar an der ersten nennt sie weiterhin "**Die eine Zuordnung** von
einem `Bereich` auf sein Feld in `Sichtbarkeit`" — das trifft seit dieser Änderung nicht mehr zu.
`sichtbar_in` ist `pub`, `aufteilung.rs` spricht das Modul bereits an, und `sichtbar_im` hat einen
einzigen Aufrufer.

### Befund 4 (niedrig): Eine Probe heißt noch nach der abgeschafften Zweiteilung

**Datensatz:** `issues/260812-0539_o_eine-probe-heisst-noch-nach-der-abgeschafften-zweiteilung-in-feste-und-bewegliche-bereiche.md`
**Ort:** `crates/krk-ui/src/fenstermodell.rs:1587`

`ein_fester_bereich_aendert_nur_seine_eigene_breite` und ihr Kommentar tragen den Begriff "fester
Bereich", den Schritt 1 mit `ist_beweglich` beseitigt hat. Die Zusicherung gilt weiter; die
Unterscheidung dahinter heißt heute `Bereich::seite`. Das Protokoll zu Schritt 1 zieht fünf andere
Probennamen aus genau diesem Grund mit und begründet es dort; diese sechste ist übersehen worden.

---

## Was quer liegt

**Beide Befunde von Gewicht sitzen an derselben Naht.** Unter der Anteilsregel ist die Abbildung
zwischen gespeicherter Zahl und Bildschirmpunkt ein einziger Faktor, und das ist die Grundlage für
drei Stellen zugleich: die Rückrechnung in `breiten_uebernehmen`, den Maßstab in `breite_aendern`
und das Wiedereinspeisen in `neu_auslegen`. Die Grundlage trägt genau so weit, wie kein Bereich an
seinem Mindestmaß hängt. Der Kommentar an `massstab` sagt das für seine Stelle ausdrücklich ("Er
gilt genau, solange kein sichtbarer Bereich an seinem Mindestmass haengt") — die beiden anderen
Stellen haben denselben Vorbehalt, nennen ihn aber nicht, und die Folgen sind dort schwerer als bei
`massstab`. Wer die Antwort auf die offene Nutzerfrage nach `MINDESTGROESSE` sucht, findet hier ihr
drittes und viertes Argument.

**Die Deckelung ist die einzige nichtlineare Stelle der ganzen Regel**, und die drei Stellen
behandeln sie verschieden: `bereichsbreiten` rechnet sie sauber aus, `massstab` benennt sie und
lässt sie stehen, `breite_aendern` und `neu_auslegen` gehen über sie hinweg. Eine einheitliche
Antwort wäre eine Antwort, kein Dickicht: entweder es gibt keine Deckelung mehr (Weg über
`MINDESTGROESSE`), oder jede Stelle, die vom Schirm zurückrechnet, prüft vorher, ob sie darf.

**Was nicht quer liegt.** Die Regel steht wirklich an einer Stelle. `bereichsbreiten` ist der
einzige Rechenweg, `zeilenmass` die einzige Stelle, an der die Geometrie aus AppKit kommt,
`Anwendungsdelegierter::zeilenmass` die einzige, an der sie in das Modell geht, und `sichtbare()`
die einzige, die die Liste der sichtbaren Bereiche bildet. Die Zweiteilung in feste und bewegliche
Bereiche ist restlos verschwunden, und mit ihr die Literalliste, die neben `Bereich::ALLE` stand.
Der Zuschnitt der drei Schritte hat gehalten, was der Plan ihm zugetraut hat: alle drei sind ohne
Fenster abzunehmen, und `make check` fährt bei jedem Zwischenstand durch.

---

## Empfohlene Reihenfolge

1. **Vor Schritt 4** nichts. Keiner der vier Befunde hält einen Planschritt auf; Schritt 4 fasst
   allein die Belegungsdatei an.
2. **Zusammen mit der Antwort auf
   `decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`:**
   Befund 1 und Befund 2. Beide hängen an derselben Wahl, und die Wahl steht beim Nutzer. Ein
   Heben von `MINDESTGROESSE` auf 940 erledigt den erreichbaren Teil beider und macht daneben den
   schon abgelegten Defekt
   `issues/260812-0512_o_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`
   gegenstandslos. Drei Datensätze an einer Frage sind das Argument dafür, sie vor Schritt 8 zu
   stellen und nicht danach.
3. **Aufräumen, wann es passt:** Befund 3 und Befund 4. Befund 3 gehört sinnvoll in denselben
   Schritt wie die nächste Änderung an `Sichtbarkeit`, also Schritt 7.

**Kein Freigabehindernis** unter den vieren. Die Runde hat noch fünf Schritte vor sich, und keiner
der Befunde ist ein Absturz, ein Datenverlust oder eine Sicherheitslücke.
