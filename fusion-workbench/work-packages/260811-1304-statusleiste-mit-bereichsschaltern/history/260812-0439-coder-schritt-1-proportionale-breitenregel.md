# Coder, Schritt 1: Die Breitenregel wird proportional

**Datum:** 260812-0439
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 1
**Abnahme:** `make check` — Exit 0

## Auftrag

Schritt 1 des Plans und nur dieser: die Breitenregel von „feste Bereiche zuerst, Dateifenster
teilen den Rest" auf einen Anteil je sichtbarem Bereich umstellen. Nicht committen; der
Orchestrator trägt ein.

## Was entstanden ist

**`crates/krk-ui/src/fenstermodell.rs`**

- Neuer Typ `Zeilenmass { gesamt, trennerbreite }` mit `verfuegbar(anzahl_sichtbar)`. Die Rechnung
  „n sichtbare Bereiche brauchen n minus eine Trennlinie" steht danach an dieser einen Stelle;
  vorher stand sie in `appkit::aufteilung::auslegen` und war ohne Fenster nicht prüfbar.
- `bereichsbreiten(mass, breiten, sichtbar)` neu gefasst, nach dem im Plan ausgeschriebenen
  Rechenweg: sichtbare sammeln, verfügbare Breite und Mindestsumme bilden, dann entweder der
  zweite Zweig (alle schrumpfen mit demselben Faktor) oder die Wasserstandsrechnung im Verhältnis
  der Wünsche.
- Neue freie Funktion `anteilig`: verteilt einen Betrag im Verhältnis von Gewichten und gibt dem
  **letzten** Bereich den Rest. Damit ist die Summe in beiden Zweigen genau die verfügbare Breite
  und nicht die verfügbare Breite plus n Rundungsfehler.
- `breiten_uebernehmen` hat seine Sonderregel für die beiden Dateifenster verloren und rechnet
  jetzt allgemein zurück: `faktor = gespeicherte Summe der sichtbaren / gemessene Summe der
  sichtbaren`. Ein sichtbarer Bereich ohne gemessene Breite bleibt außen vor, in beiden Summen und
  beim Setzen; das ist der Schutz, den vorher die Bedingung `gemessen > 0` trug.
- `Bereich::ist_beweglich` **entfernt.** Die Funktion hatte nach der Umstellung keinen Aufrufer
  außerhalb der Proben mehr, und `cargo clippy --all-targets -- -D warnings` hält an totem Code an.
  Die Zweiteilung in feste und bewegliche Bereiche existiert in der Breitenregel nicht mehr;
  `Bereich::seite` bleibt und trägt die Zuordnung weiter, die `breite_aendern` braucht.
- Dokumentationskommentare: der neue an `bereichsbreiten` schreibt die Anteilsregel mit ihren zwei
  Zweigen aus. Die Sätze über die Reihenfolge von `Bereich::ALLE` als Zusage und über die
  Festlegung vom 260808 sind ersatzlos gefallen. Daneben nachgezogen, weil die Änderung sie falsch
  machte: der Modulkopf (Absatz „fest und beweglich" und die Zeile unter der Skizze),
  `Bereich::teilt_flaeche_mit`, `Bereich::anfangsbreite`, `Bereich::seite`, `breite_aendern`.

**`crates/krk-ui/src/appkit/aufteilung.rs`**

- `auslegen` reicht `Zeilenmass { gesamt: teiler.frame().size.width, trennerbreite:
  teiler.dividerThickness() }` weiter und zählt die sichtbaren Bereiche nicht mehr selbst. Die
  Schleife, die die Rahmen setzt, ist unverändert.

**`crates/krk-ui/src/appkit/fenster.rs`** — eine Datei mehr als der Plan nennt, und nur ein
Kommentar darin. Der Kommentar an `MINDESTGROESSE` beschrieb, wie `bereichsbreiten` zwischen 780
und 920 Punkten Fensterbreite den Editor kürzt und die übrigen verschont. Nach der Umstellung
schrumpfen dort **alle vier** mit demselben Faktor. Ein Kommentar, der das Gegenteil des Codes
sagt, ist in diesem Baum schon einmal teuer geworden; deshalb steht er jetzt richtig da, und die
Frage nach 940 Punkten Untergrenze bleibt unangetastet offen.

## Prüfungen

Elf bestehende Proben zur Breitenregel nachgezogen, fünf neue dazu. `bereichsbreiten` hat in
`fenstermodell.rs` damit sechzehn Proben, das Prüfmodul insgesamt 32.

**Fünf ändern ihre Aussage und tragen einen neuen Namen** — der Plan nannte zwei davon vorher,
die drei anderen fallen aus derselben Umstellung an:

| vorher | nachher |
|---|---|
| `die_leiste_weicht_dem_editor_nicht` | `die_leiste_schrumpft_mit_dem_editor` |
| `am_engen_fenster_gewinnt_das_mindestmass_der_dateifenster` | `am_engen_fenster_gewinnt_das_mindestmass_gegen_den_anteil` |
| `der_frei_gewordene_platz_geht_an_die_dateifenster` | `der_frei_gewordene_platz_geht_an_die_uebrigen_bereiche` |
| `der_eingeblendete_editor_bekommt_seine_breite_und_die_dateifenster_den_rest` | `der_eingeblendete_editor_bekommt_seinen_anteil` |
| `jeder_feste_bereich_bekommt_seine_breite_ohne_zweite_aufzaehlung` | `jeder_sichtbare_bereich_bekommt_seinen_anteil_ohne_zweite_aufzaehlung` |

Dazu `beweglich_ist_genau_ein_dateifenster_und_die_zuordnung_laeuft_in_beide_richtungen` →
`die_zuordnung_von_bereich_und_fensterseite_laeuft_in_beide_richtungen`, weil die Aufzählung
`ist_beweglich` weg ist. Jeder Name, der eine Aussage trug, die nicht mehr gilt, ist mitgezogen:
ein Probenname, der das Gegenteil seiner Zusicherung sagt, hält dieselbe Falle bereit wie ein
Kommentar.

**Zwei Proben haben ihre Fensterbreite gewechselt, und beide Male steht der Grund im Kommentar:**

- `kein_bereich_faellt_unter_sein_mindestmass`: 500 → 800 Punkte. Bei 500 liegt der Fall im zweiten
  Zweig, in dem **jeder** unter sein Mindestmaß fällt; die Probe hätte dort ihre eigene Zusage
  widerlegt. Bei 800 misst sie die Wasserstandsrechnung, und den zweiten Zweig misst die neue Probe
  daneben.
- `der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt`: 1400 → 1280 Punkte. Das ist
  kein Zurechtrücken, sondern die Grenze der Zusage; siehe den Defekt unten.

**Fünf neue Proben:** `das_zeilenmass_zieht_je_trennlinie_ab`,
`das_verhaeltnis_zweier_bereiche_ueberlebt_das_einblenden_eines_dritten`,
`die_summe_ist_immer_die_verfuegbare_breite` (vier Lagen der Sichtbarkeit × fünf Fensterbreiten ×
drei Trennlinienbreiten), `unter_der_summe_der_mindestbreiten_schrumpfen_alle_mit_demselben_faktor`,
`das_vergroessern_des_fensters_laesst_die_gespeicherten_breiten_stehen`. Der Plan verlangte vier;
die fünfte gehört zum neuen Typ, dessen Rechnung vorher keine Probe erreichte.

## Ein Defekt abgelegt

`issues/260812-0439_o_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`

Kriterium C4.9 des Plans hält nach Schritt 1 nur noch bei einer Fensterbreite. Gemessen mit einer
Wegwerfprobe, die danach wieder entfernt wurde: bei 1280 Punkten springt die Trennlinie um 40, bei
1400 um 43,75, bei 1920 um 60. Die Ursache ist kein Versehen, sondern die Entscheidung des Plans
zugunsten von C4.7: die Rückrechnung hält die gespeicherte Summe fest, und `breite_aendern` rechnet
in gespeicherten Punkten, während C4.9 Punkte auf dem Schirm meint. Der Datensatz führt die
Messung, beide Wege hinaus und die Empfehlung — den Schritt umrechnen, sobald Schritt 2 das
`Zeilenmass` bis zu den Aufrufern durchgereicht hat.

## Abnahme

`make check` (`cargo build`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D
warnings`, alle über den Workspace) — **Exit 0**, „alle vier grün". Kein Vordergrund nötig, wie der
Plan für diesen Schritt zusagt.

Nicht committet: der Orchestrator trägt ein.
