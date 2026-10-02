# Umsetzung: die Aufsicht liest Plätze statt freier Wörter

**Datum:** 2026-08-21
**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `ca38a77`
**Durchsicht:** `shared/reviews/260821-1432-coderev-auftrag-statt-nackter-wortliste.md`, sechs Befunde
**Plan:** `shared/planning/260821-1221_*_plan-artefakt-und-release.md`
**Wahl des Nutzers:** die Umstellung, nicht die zwei Flicken

## Was entstanden ist

Die zwei schweren Befunde hatten dieselbe Wurzel: die Aufsicht las freie Wörter, ohne die
Gestalt zu kennen, die an ihrem Platz zulässig ist. Der Prüfer hat vorgeschlagen, die Auskunft,
die `Auftrag` ohnehin hat, bis zur Aufsicht durchzureichen. Der Vorschlag trägt am Baum, und er
ist übernommen.

## Der Kern: `Wort` und `Gestalt`

`Auftrag::wortplaetze` ersetzt `Auftrag::worte` als die eine Stelle, an der die Varianten ihre
Wörter nennen. Sie liefert `Vec<Wort>`, und ein `Wort` ist entweder `Fest` — ein Wort, das die
Variante selbst mitbringt — oder `Platz(Gestalt, wert)` — ein Platz, den der Aufrufer belegt.
`Auftrag::worte` leitet sich daraus ab und zählt die Varianten nicht ein zweites Mal; damit
bleibt genau eine Fallunterscheidung, die eine achte Variante anhält.

`Gestalt` hat vier Werte, und jeder gehört zu einer Art von Wert, die dieses Werkzeug
hinausreicht: `Tagname`, `Tagverweis`, `Meldung`, `Pfad`. `Gestalt::befund` ist wieder eine
vollständige Fallunterscheidung ohne Auffangzweig.

**`Gestalt::Tagname` ruft `version::versionszahl_pruefen`.** Das ist die Prüfung, die vor der
Umstellung die Sicherung war, die in Wahrheit trug — der Grund, aus dem kein Doppelpunkt und
keine Marke je in einen Refspec dieses Werkzeugs geriet —, während die Aufsicht nichts von ihr
wusste. Der Prüfer hat verlangt, dass sie in die neue Bauform kommt oder wenigstens in ihre
Prosa; sie ist in die Bauform gekommen. Eine eigene Ziffernprüfung in `git.rs` wäre eine zweite
Vorschrift darüber, wie eine Versionszahl dieses Projekts aussieht, und genau das verbietet der
Doc-Kommentar jener Funktion. Der Preis ist ein Verweis von `git` auf `version`, also ein Zyklus
auf Modulebene; er ist bewusst in Kauf genommen, weil die Alternative eine zweite Wahrheit wäre
und weil die gerufene Funktion rein ist und keine weitere Abhängigkeit mitbringt.

**Die Stellung zählt neben der Gestalt.** `stellungsbefund` verlangt, dass eine `Meldung`
unmittelbar hinter `-m` steht und ein `Pfad` hinter dem Trenner `--`. Das ist der Grund, aus dem
eine Eintragsmeldung wie eine Marke aussehen darf: `git` liest sie dort nicht als eigenes Wort.
Der Falschalarm, den der Prüfer als Nebeneffekt genannt hat — `git commit --only -m "-a" --
Cargo.toml` wurde angehalten —, ist damit weg und ist als Probe festgehalten.

## Wo die Verbotsliste bleibt, und warum

**An festen Wörtern**, also an dem, was jemand beim Hinzufügen einer Variante selbst
hinschreibt. Dort ist eine Gestaltprüfung nicht möglich: die Wörter sind Unterbefehle, Marken
und Trenner in beliebiger Zusammensetzung, und eine Erlaubnisliste über sie wäre die Aufzählung
aller `git`-Optionen, die dieses Werkzeug je brauchen könnte. Der Modulkopf sagt jetzt
ausdrücklich, dass `MARKEN` dort **eine zweite Gelegenheit hinzusehen und keine Zusage** ist. Das
ist die ehrliche schwache Aussage; eine starke wäre falsch.

Die Liste ist im selben Zug stärker geworden. `verwandte_marke` vergleicht nicht mehr auf
Gleichheit, sondern am Wortanfang in beide Richtungen, nachdem ein Anhang hinter `=` abgetrennt
ist: `--del` fällt als `--delete`, `--force-with-lease` als `--force`, `--exec=/bin/sh` als
`--exec`. `--exec` und `--receive-pack` sind als Einträge dazugekommen; `--force` ist aus dem
eigenen Wortanfang-Vergleich in `MARKEN` gewandert, wodurch die Konstante `GEWALTANFANG`
entfällt. Kurze Marken bleiben ausdrücklich draußen — `-m` wäre sonst der Anfang von `--mirror`
—, sie liest weiter `kurze_marke` Buchstabe für Buchstabe. Der Doppelpunkt wird an einem festen
Wort ebenfalls angehalten, für den Fall, dass ihn jemand in eine neue Variante schreibt.

## Die vier übrigen Befunde

- **B1** — `jede_einzelne_marke_wird_angehalten` schreibt die elf langen Marken und die drei
  Gewaltbuchstaben **aus**, statt über die Listen zu laufen: ein gelöschter Eintrag wird rot.
  `jeder_eintrag_der_listen_steht_in_der_anhalteprobe` nimmt die Gegenrichtung. `beispiele()`
  entsteht aus `naechster`, einer Fallunterscheidung ohne Auffangzweig; was diese Kette nicht
  hält, steht in ihrem Doc-Kommentar. `die_auftraege_stehen_wort_fuer_wort` zählt gegen
  `beispiele().len()` nach, statt eine Zahl in Prosa zu führen.
- **C1** — die `##`-Zeile von `make ausliefern` nennt das Schieben. Die Probe heißt jetzt
  `die_hilfezeilen_des_makefiles_nennen_das_schieben` und läuft über `SCHIEBENDE_ZIELE`.
- **D1** — der Nachtrag im Plan führt C3.4 und C3.5 einzeln; C3.5 ist als **überholt** geführt.
  Die Zuordnungstabelle ist nachgezogen, ein zweiter Nachtrag angefügt.
- **D2** — der Modulkopf trennt drei Stärken statt zweier und nennt die dritte „allein eine
  Probe hält", samt ihrer Grenze. Die Nadel jener Probe steht dort nicht ausgeschrieben, sonst
  zählte der Modulkopf sich selbst mit.

## Was gemessen ist

`make check` — Exit 0, alle vier grün, 155 Proben in `xtask` (davor 146).

Vier Mutationen gefahren, jede wird rot und ist danach zurückgenommen: ein gelöschter Eintrag
aus `MARKEN`, eine entfernte Doppelpunkt-Regel, ein auf Gleichheit zurückgestellter
Markenvergleich, ein `Gestalt`-Platz, der zum festen Wort gemacht wird. Eine achte Variante von
`Auftrag` hält den Bau an drei Stellen an — `wortplaetze`, `wirkung`, `naechster` —, nachgefahren.

An einem eigenen Wegwerf-Verzeichnis nachgemessen und nicht aus der Durchsicht übernommen:
`git push origin HEAD :refs/heads/feature` löscht den Zweig `feature` auf der Gegenseite, `git
tag --del` löscht einen Tag, und `git push --mirr` kommt durch den Optionszerleger.

**Ungemessen: alles gegen GitHub.** `gh` fehlt auf diesem Gerät.

## C3.7

`keine_der_drei_fragen_schreibt` steht unter demselben Namen, mit denselben drei Fragen und
denselben Behauptungen; angepasst ist allein der Aufruf auf `wortplaetze`. Grün.
