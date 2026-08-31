# Ontocoder-Sitzung: Schritt 9 der Runde 23, die drei Einträge in der Auslieferungsbelegung

**Date:** 2026-08-31
**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 9
**HEAD:** `7079519` (nicht committet; der Orchestrator committet)

## Die Prüfung vor dem Schreiben

Der Plan verlangt sie am eigenen Stand, und sie ist gefahren: keine `tasten`-Zeile von
`resources/default-keymap.toml` nannte `opt+cmd+r` oder `shift+cmd+b`. Gemessen über
alle Zeichenketten jeder `tasten`-Zeile, nicht über eine Textsuche im ganzen Text: die
naive Suche nach `opt+cmd+r` trifft drei Kommentarstellen, die `opt+cmd+right` meinen.
Die einzige doppelt vergebene Kombination der Datei ist vor wie nach diesem Schritt
`cmd+a`, und sie ist im Dateikopf als der eine ausgelieferte Fall zweier Zusteller
begründet.

## Die drei Blöcke

| `id` | `tasten` | steht hinter | erscheint im Menü unter |
|---|---|---|---|
| `fokus_git` | `["shift+cmd+b"]` | `fokus_vorschau` | „Git" |
| `git_bereich_umschalten` | `["opt+cmd+r"]` | `zweites_fenster_umschalten` | „Git" |
| `spalte_marke_umschalten` | `[]` | `spalte_typ_umschalten` | „Dateilisting" |

Die Reihenfolge der Datei ordnet nur innerhalb der Gruppe; welche Gruppe eine Funktion
bekommt, sagt `belegungsmodell::bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs:443`).
Deshalb stehen die zwei Git-Einträge an zwei weit auseinanderliegenden Stellen der Datei
und trotzdem in einem Obermenü.

**Im Menü „Git" steht der Fokusbefehl vor dem Umschalter**, als Folge dieser zwei
Plätze. Gemessen an `make menue`: das ist kein Bruch mit einer Gewohnheit, denn die Datei
hat keine. „Leiste und Fokus" führt die Fokusbefehle vor ihrem Umschalter, „Editor"
ebenfalls, „Vorschau" umgekehrt. Kein Kriterium und keine Probe legt die Reihenfolge
innerhalb einer Gruppe fest.

## Was die Kommentare tragen

Jede der zwei Kombinationen trägt ihre Begründung, wie jede andere in dieser Datei: der
Buchstabe (`r` für „Repository", `b` für „Branch"), die Nachzählung vom 260831, der
Verweis auf E10 des Specs als Nutzerwahl, und die Familie.

Beim Fokusbefehl steht daneben der Befund, den der Auftrag ausgeschrieben verlangt hat:
das Erben des Buchstabens vom Umschalter ist bei Leiste, Dateifenster und Vorschau die
Regel, beim Editor schon gebrochen, und für Git **nicht möglich**, weil `shift+cmd+r` an
`editor_ersetzen` vergeben ist. Die bleibende Unsauberkeit steht ausdrücklich da: `b`
trägt in der Umschaltfamilie den Editor und in der Fokusfamilie Git, unterschieden allein
durch die Zusatztaste. Kein Konflikt, aber eine Stelle zum Danebengreifen, und wer sie für
ein Versehen hält und berichtigt, dreht eine bewusste Wahl um.

Die am 260831 in der Fokusfamilie freien Buchstaben stehen als Liste mit Datum daneben,
`f`, `j`, `m`, `o`, `p`, `q`, `t`, `x`, dazu das Zählkommando für ihren heutigen Bestand
— dieselbe Form, die der Kommentar bei `ordner_der_datei` seit dem 260818 für die
opt+cmd-Reihe trägt. Keiner der acht ist für „Git" oder „Repository" sprechend; `g` und
`r` sind vergeben.

## Die zwei Zählstände im Kopf, und ein dritter, der gefallen ist

Die Kopfzeile lautet jetzt „Ausgeliefert sind 91 Funktionen mit zusammen 95
Kombinationen". **Gezählt und nicht gerechnet:** ein Durchlauf über `^[[funktion]]` und
über die Einträge jeder `tasten`-Zeile derselben Datei liefert 91 und 95, und
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` hält beide gegen den
Bestand. Der Defekt dazu ist geschlossen
(`260831-0230_*_die-drei-neuen-eintraege-ziehen-die-zwei-zaehlstaende-im-kopf-der-auslieferungsbelegung-nach.md`).

Zwei weitere Zahlen derselben Datei hätte dieser Schritt falsch gemacht, und keine davon
ist nachgezogen worden; beide sind gefallen:

- Der Einleitungsabsatz der Spaltenschalter sprach von „den drei Spaltenschaltern" und
  „diesen drei Funktionen". Mit `spalte_marke_umschalten` sind es vier. Er nennt jetzt
  keine Zahl.
- Im selben Absatz stand „Schon als dieser Absatz am 260812 geschrieben wurde, führte die
  Datei 85 Kombinationen, heute sind es 90." Die 90 stammt aus `18af77f` vom 260818 und
  war schon vor dieser Runde falsch: der Bestand war 93. An ihre Stelle tritt der Zeiger
  auf die Kopfzeile, die eine Probe hält. Die 85 bleibt stehen, weil sie datiert ist und
  eine Aussage über den 260812 macht, keine über heute.

Das ist die Regel dieses Projekts an ihrer eigenen Datei angewandt: eine Zahl steht an
einer Stelle, und die trägt eine Probe.

## Abnahme

`make check` — **exit 0**, „alle vier gruen". Kein Fehlschlag in keinem Prüfziel.

Die sieben Proben, die Schritt 8 als vorgesehene Zwischenröte hinterlassen hat, sind
grün, dazu die achte an den Zählständen. Namentlich aus dem Laufprotokoll:

| Probe | Stand |
|---|---|
| `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` | ok |
| `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | ok |
| `belegungsausgabe::tests::die_abschnitte_stehen_in_der_reihenfolge_der_funktionsbereiche` | ok |
| `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` | ok |
| `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` | ok |
| `belegungsmodell::tests::die_zeilen_sind_nach_bereichen_gegliedert` | ok |
| `menuemodell::tests::die_obermenues_folgen_der_gliederung` | ok |
| `tasten::belegung::tests::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` | ok |

`make menue` führt beide neuen Zeilen mit ihren Kombinationen und den vierten
Spaltenschalter unmittelbar hinter „Spalte Typ". Was `make tasten` und `make menue` am
laufenden Bündel auslösen, ist damit nicht geprüft: C1.2, C2.2 und C5.6 haben je eine
Hälfte, die Nutzerarbeit am Bündel bleibt.

## Kein Code angefasst

Geändert ist genau `resources/default-keymap.toml` und der geschlossene Defekt. Die drei
`Kommando`-Varianten stehen seit Schritt 8 (`7079519`) im Baum; die zwei Ausnahmelisten
für Funktionen ohne Kombination hat jener Schritt bereits nachgezogen.

Der Marker-Wechsel des Defekts ist mit `git mv` gefahren und liegt deshalb im Index
(`RM` in `git status`). Committet ist nichts.
