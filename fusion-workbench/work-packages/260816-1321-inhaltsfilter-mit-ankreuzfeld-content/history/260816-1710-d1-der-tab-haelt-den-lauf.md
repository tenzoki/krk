# D1: Der Tab hält den Lauf, seine Aufträge und die Zahl der ungelesenen Dateien

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt D1
**Baumstand vor der Arbeit:** `32fd038`
**Vorbedingungen:** A1 (`5c7f5b9`), C1 (`4a54212`), A2 (`7283d55`), B1 (`32fd038`)
**Erfüllt:** C2.3 (Modellhälfte), C2.4, C2.5, C3.2, C3.8, C4.4, C4.5, C4.6. C1.12 nur zur Hälfte — die andere ist am Bündel abzunehmen.

## Was entstanden ist

Angefasst ist genau eine Datei, `crates/krk-ui/src/tabs.rs`.

**Die Tafel der vier Auftragslagen steht in `auftraege`.** Der Kurzschluss
`!name_traegt_den_filter` steht weiter am Eingang und gilt jetzt für beide
Arten; was ihn übersteht, entscheidet ein Zweig über den Typ des Eintrags mit
dem zugehörigen Schalter. Ordner und Verknüpfung bekommen bei stehendem „Deep"
einen `Unterbaum`, eine gewöhnliche Datei bei wirkendem „Content" einen
`Inhalt`, sonst keiner von beiden einen. Beide Schalter werden **einmal je
Liste** gefragt und nicht einmal je Eintrag; ob der Inhaltsfilter wirkt, sagt
`Ordnermodell::inhalt_wirkt` und nicht diese Stelle (C2.10). Der Doc-Kommentar
trägt die Tafel als Textblock, und der Kommentar, der auf D1 zeigte, ist
gefallen.

**Die 1 MB reisen an genau einer Stelle in den Kern** (C1.7):
`durchlauf_nachziehen_an` übergibt
`tab.modell.inhalt_wirkt().then_some(crate::vorschaumodell::TEXTGRENZE)`. Die
Zahl bleibt, wo sie wohnt; `krk-core` bekommt keinen Bezug auf `krk-ui`. Die
Sperre davor lautet jetzt `!tief() && !inhalt_wirkt()` — allein „Content"
stößt einen Lauf an, „Deep" ist dafür nicht nötig.

**`Tabinhalt` trägt `zu_gross: u64`.** `lesen_starten` und
`durchlauf_nachziehen_an` setzen es auf null, wo ein Lauf fällt oder beginnt;
`befunde_einziehen` schreibt bei **jedem** Takt den Zählerstand des Laufs
hinein, auch bei dem, der den geschlossenen Kanal sieht. Damit steht die Zahl
nach dem Ende des Laufs noch da — sonst sähe der Nutzer sie bei einem kleinen
Ordner nie, und der Größenhinweis der Statuszeile wäre eine Anzeige, die nur
bei langen Läufen aufblitzt.

**`ordner_setzen` trägt „Content" als fünfte Übertragung hinüber**, unbedingt
und ohne Zweig, neben Sortierung, Verstecken, „Deep" und Filtertext (C1.12,
C2.4). Nichts davon geht in die Sitzung (C2.5): ein wiederhergestelltes
„Content" ohne Filtertext wäre ein Zustand, den nichts anzeigt und der nichts
tut — dieselbe Begründung, die für „Deep" schon gilt.

## Der Nutzerentscheid vom 260816-1410, umgesetzt

**Ein Tabwechsel beendet den Durchlauf des verlassenen Tabs, gleich welcher
Art** (C4.5, Möglichkeit 1 von
`decisions/260816-1359_a_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md`).
`Tabliste::waehlen` setzt `aktiv` um und ruft danach
`durchlauf_nachziehen_an` auf der verlassenen Stelle.

**Getragen wird die Regel von einer vierten Bedingung im Rumpf jener Methode:
ein verdeckter Tab bekommt keinen Durchlauf.** Ohne sie wäre der Ruf aus
`waehlen` wirkungslos gewesen — er hätte den Lauf abgebrochen und im selben
Zug einen neuen gestartet, weil Filtertext und Schalter am verlassenen Tab ja
stehenbleiben. Die Bedingung steht vor den drei anderen, weil sie die einzige
ist, die nicht am Modell hängt, und sie macht den Zuschnitt des Einzugstakts,
der über alle Tabs fragt, gegenstandslos statt falsch — genau so, wie der
Datensatz es unter `## Constraints` beschreibt.

**Der Kommentar der Runde 10 ist ersetzt und nicht gelöscht.** Er nennt jetzt
den Datensatz, die Abwägung (ein Namensdurchlauf ist in Millisekunden durch,
ein Inhaltsdurchlauf liest minutenlang für einen Tab, den niemand ansieht) und
den angenommenen Preis: wer mit stehendem Filtertext zwischen zwei Tabs hin und
her wechselt, lässt den Unterbaum jedes Mal von vorn abschreiten.

## Was geprüft ist

Sieben Proben sind hinzugekommen, eine ist ersetzt, alle im `#[cfg(test)]`-Modul
neben dem Code, in der Form, die dort seit der Runde 10 steht:

- `die_auftragsliste_stellt_die_tafel_der_vier_auftragslagen` — ein Bestand,
  vier Schalterstellungen, die ganze Tafel in einer Probe. Fünf Zeichen mit
  Absicht: sie liegen über beiden Schwellen, also hängt das Ergebnis allein an
  den Schaltern.
- `bei_vier_zeichen_und_deep_traegt_die_auftragsliste_keinen_inhaltsauftrag`
  (C3.2), samt dem fünften Zeichen, das die Inhaltsaufträge zurückholt.
- `eine_datei_mit_namenstreffer_bleibt_ungelesen` (C3.4 an der Auftragsliste).
- `allein_content_stoesst_einen_durchlauf_an` — die geänderte Sperre, in beide
  Richtungen: „Content" ohne „Deep" reicht, und ein zurückgenommenes Zeichen
  unter der Schwelle lässt den Lauf fallen.
- `ein_ordnerwechsel_traegt_den_stand_von_content` (C2.4) und
  `der_inhaltsfilter_geht_auch_ohne_filtertext_hinueber`.
- `die_zahl_der_zu_grossen_dateien_steht_auch_nach_dem_ende_des_laufs` — der
  ganze Weg ohne AppKit, an einem echten Prüfordner mit einer Datei über
  `TEXTGRENZE`. Die Grenze wird gehalten und nicht vorhergesagt, also muss die
  Datei echt sein.
- `ein_verdeckter_tab_bekommt_keinen_durchlauf` — die vierte Bedingung für
  sich, ohne den Umweg über `waehlen`.

**Ersetzt:** `ein_tabwechsel_laesst_den_durchlauf_stehen` heißt jetzt
`ein_tabwechsel_beendet_den_durchlauf_des_verlassenen_tabs` und prüft das
Gegenteil dessen, was sie bis zum 260816 prüfte. Der Doc-Kommentar der Probe
nennt den Datensatz.

## Zwei Feststellungen, die zum Schritt gehören

**`Tabinhalt::zu_gross()` hat bis F2 keinen Ableser** und trägt deshalb ein
`#[expect(dead_code, reason = …)]`. `expect` statt `allow`, weil die Ausnahme
sich damit selbst zurücknimmt: sobald der Größenhinweis der Statuszeile die
Methode ruft, ist die Erwartung unerfüllt, und
`unfulfilled_lint_expectations` hält unter `-D warnings` den Bau an, bis die
Zeile fällt. Das Projekt duldet keine Ausnahme ohne Ablaufdatum
(`editormodell.rs:213-225`); diese hat eins, und es ist erzwungen. Die Probe
liest das Feld unmittelbar, damit die Erwartung bis dahin erfüllt bleibt.

**Der Rückwechsel stößt den Lauf nicht wieder an.** Der Plan nennt für
`waehlen` genau einen Ruf, den auf der verlassenen Stelle, und die Datei dieses
Schritts ist allein `tabs.rs`. Wer auf einen Tab zurückwechselt, dessen Lauf
der Wegwechsel beendet hat, sieht die Befunde, die bis dahin eingetroffen
waren, und der Lauf setzt nicht fort; erst die nächste Filteränderung stößt ihn
neu an. Ein Anstoß in `waehlen` bräuchte daneben eine zweite Änderung in
`crate::appkit::tabelle` — `tab_gewechselt` wirft den Einzugstakt heute nur an,
wenn `liest_noch()` gilt, nicht wenn ein Durchlauf läuft —, und die gehört
nicht in diesen Schritt. Abgelegt als
`issues/260816-1710_o_ein-rueckwechsel-auf-einen-tab-setzt-seinen-beendeten-durchlauf-nicht-fort.md`.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, 629 Proben im Binärziel
`krk-ui`. Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` lief durch.

**C1.12 ist zur Hälfte Nutzerarbeit** und steht in keiner Probe: dass die Liste
beim Ordnerwechsel mit stehendem „Content" und ausreichend langem Filtertext
sofort zu wachsen beginnt, ist am laufenden Bündel zu beobachten. Geprüft ist
hier die Vorbedingung — der Schalter übersteht den Wechsel und wirkt danach.

## Was offen bleibt

E1 bis E3 bringen das Kommando, den Belegungseintrag und das zehnte
Ankreuzfeld; F1 die abgesetzte Zeile; F2 die zwei Satzteile am Filterstand und
damit den Ableser für `Tabinhalt::zu_gross`.
