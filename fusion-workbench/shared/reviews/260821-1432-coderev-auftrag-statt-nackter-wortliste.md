# Durchsicht: `git` nimmt einen Auftrag statt einer nackten Wortliste

**Reviewed-range:** `465330b..94855a7`
**Not-opened:** `fusion-workbench/shared/history/260821-1620-coder-durchsicht-artefakt-und-release-behoben.md`, `fusion-workbench/shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`, `fusion-workbench/shared/issues/260821-1401_o_der-leerbefund-zweig-verschweigt-eine-dastehende-sicherung-die-den-bestand-traegt.md`, `fusion-workbench/shared/reviews/260821-1401-coderev-der-leerbefund-zweig-sichert-nichts-mehr.md`

**Nur in Teilen gelesen:** `README.md`, `xtask/src/main.rs`, `xtask/src/release.rs`,
`xtask/src/veroeffentlichung.rs`, `xtask/src/version.rs` und
`fusion-workbench/shared/reviews/260821-1346-coderev-artefakt-und-release.md` — jeweils der
Unterschied zu `465330b` samt Umgebung und den Stellen, auf die er verweist, nicht die ganze
Datei. `xtask/src/git.rs`, `Makefile` und
`fusion-workbench/shared/planning/260821-1221_c_plan-artefakt-und-release.md` sind ganz
gelesen.

---

## Zusammenfassung

Der Umbau ist richtig geschnitten. `git::rufen` nimmt eine geschlossene Aufzählung entgegen,
die Aufsicht steht auf dem Weg zum Prozessaufruf statt daneben, und die Erlaubnisliste der
Unterbefehle ist für die sieben Varianten vollständig und trennscharf. **Alle drei Zusagen des
Coders treffen in ihrer Substanz zu**, und die als C3.7 abgenommene Probe ist in ihrer neuen
Form stärker als die gelöschte, nicht schwächer.

Zwei Löcher bleiben in der Aufsicht selbst, und beide haben dieselbe Wurzel: sie liest freie
Wörter, ohne die Gestalt zu kennen, die an ihrem Platz zulässig ist. Ein Refspec mit
Doppelpunkt kommt durch und löscht eine Referenz auf der Gegenseite; die Marken aus `MARKEN`
kommen in ihrer von `git` akzeptierten Kurzform durch. **Beides ist aus den heutigen sieben
Varianten nicht erreichbar** — was heute wirklich trägt, ist `version::versionszahl_pruefen`,
und die Aufsicht sagt darüber nichts.

## Zählung

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 2 |
| Mittel | 2 |
| Niedrig | 2 |

**Kein Auslieferungshindernis.** Kein Befund beschreibt ein Fehlverhalten des gebauten Standes.
Die zwei hohen betreffen die Reichweite einer Sicherung, die ausdrücklich für die Änderung von
morgen gebaut ist — der Prüfkommentar von
`die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen` sagt das selbst.

`make check` ist am Baumstand grün; `cargo test -p xtask` liefert 146 Proben, alle grün.

## Die drei Zusagen, einzeln

### 1. „Der Übersetzer hält, dass keine Liste an der Aufzählung vorbei bei `git` ankommt." — trifft zu

`rufen` (`xtask/src/git.rs:386-390`) nimmt `&Auftrag` und ruft `auftrag.worte()` selbst. Es
gibt keinen zweiten Einstieg und keinen Parameter, über den eine fertige Wortliste
hereinkäme. `worte()` (`:174-190`) und `wirkung()` (`:196-205`) sind beide vollständige
Fallunterscheidungen ohne Auffangzweig; eine achte Variante hält den Bau an beiden Stellen an.

**Zur Sichtbarkeit:** `Auftrag` ist `pub(crate)`, aber die Frage nach dem Hinzufügen von
Varianten von außen stellt sich in Rust nicht — eine Aufzählung ist geschlossen, gleich wie
sichtbar sie ist. `aufsichtsbefund`, `gewaltbefund` und `kurze_marke` sind modulprivat.

**Was `pub(crate)` sehr wohl bedeutet:** jedes Modul der Kiste kann die vier belegbaren
Wortplätze frei füllen — `Tagliste`, `TagSetzen`, `Eintrag.meldung`, `Eintrag.dateien`,
`Schub.verweis`. Genau dort liegen die zwei hohen Befunde.

### 2. „Die Aufsicht auf dem Weg hält, dass keine Liste einen fremden Unterbefehl oder eine Gewaltmarke trägt." — trifft für die Unterbefehle zu, für die Marken nicht ganz

**Sie greift auf jedem Weg.** `aufsichtsbefund` steht in `rufen` vor `Command::new`, und es
gibt keinen zweiten Weg zu diesem `Command::new`.

**Sie bricht ab und meldet nicht nur.** `Err(Abbruch::Lauf(aufsichtsmeldung(…)))` vor dem
Prozessaufruf; die Meldung endet auf „Es wird nichts ausgeführt", und das stimmt.

**Die Erlaubnisliste der Unterbefehle ist vollständig für die sieben Varianten.** Die vier
lesenden tragen `rev-parse`, `tag`, `status` — genau `LESENDE`. Die drei schreibenden tragen
`tag`, `commit`, `push` — genau `SCHREIBENDE`. Kein Eintrag zu viel, keiner zu wenig. Die
Ausnahme, dass `tag` in beiden steht, ist über die `--points-at`/`--list`-Frage aufgelöst und
in `eine_lesende_frage_legt_keinen_tag_an` (`:640-646`) beidseitig nachgesehen.

**Die Markenhälfte hat zwei Löcher.** Siehe die zwei hohen Befunde.

### 3. „Nichts hält einen zweiten Prozessaufruf an `rufen` vorbei." — trifft zu, die Probe misst weiter

`xtask_ruft_git_an_genau_einer_stelle` (`xtask/src/release.rs:1076-1092`) zählt zwei
Zeichenfolgen in jeder `.rs`-Datei des Baums und verlangt genau ein Vorkommen in `git.rs`.
Sie ist von `94855a7` nicht angefasst, und der gesuchte Text hat sich nicht geändert — die
Probe misst nach dem Umbau dasselbe wie davor. `rust_dateien` läuft über den ganzen Baum ohne
`target/` und `.git` (`release.rs:1287-1305`), `xtask/src` eingeschlossen. Grün nachgefahren.

Der Satz im Modulkopf, der das ausschreibt, widerspricht sich allerdings selbst — Befund D2.

## Befunde nach Thema

### A. Die Aufsicht liest freie Wörter, ohne ihre Gestalt zu kennen

#### A1 — Ein Refspec mit Doppelpunkt kommt durch und löscht eine Referenz (Hoch)

`gewaltbefund` (`git.rs:314-341`) prüft ein führendes `+`, weil ein Refspec ohne jede Marke
erzwingen kann. Die zweite markenlose Form ist der Doppelpunkt: `<quelle>:<ziel>` schreibt
eine beliebige Referenz auf der Gegenseite, `:<ziel>` löscht sie. Die Aufsicht kennt sie nicht.

Nachgemessen an einem Wegwerf-Verzeichnis, mit genau der Wortform, die `Auftrag::Schub` baut:

```
$ git push origin HEAD :refs/heads/feature
  To ../remote.git
   - [deleted]         feature
```

`MARKEN` trägt `--delete`, die Liste ist also ausdrücklich gegen das Löschen auf der Gegenseite
gebaut. Der Doppelpunkt ist dieselbe Wirkung in der älteren Schreibweise.

Nicht erreichbar aus dem heutigen Baum: `verweis` entsteht als `refs/tags/v<zahl>`, und `zahl`
geht durch `version::versionszahl_pruefen` (`version.rs:433-462`), das nur ASCII-Ziffern
durchlässt.

Datensatz:
`shared/issues/260821-1432_o_ein-refspec-mit-doppelpunkt-kommt-durch-die-aufsicht-und-loescht-eine-referenz-auf-der-gegenseite.md`

#### A2 — `git` nimmt Abkürzungen langer Marken an, der Gleichheitsvergleich fängt sie nicht (Hoch)

`MARKEN` und `UEBERGEHENDE` werden mit `contains(&wort)` geprüft, also auf Gleichheit; der
Doc-Kommentar sagt es ausdrücklich. `git` nimmt lange Marken aber abgekürzt entgegen, solange
die Abkürzung eindeutig ist. Nachgemessen:

```
$ git tag t1 && git tag --del t1
Tag 't1' gelöscht (war a7cda2a)

$ git commit --ame --no-edit -a         # ändert den letzten Eintrag

$ git push --mirr /nonexistent-remote   # Optionszerleger nimmt an, erst die Gegenseite fehlt
$ git push --al   /nonexistent-remote   # dito
```

Keines dieser Wörter steht in `MARKEN`. Die Aufsicht hält keines an.

**Die Begründung, aus der `--force` danebensteht, ist zur Hälfte gedacht.** Der Doc-Kommentar
von `MARKEN` (`git.rs:222-227`) erkennt die Formen mit **Anhang** und behandelt sie am
Wortanfang. Die Formen mit **Abschlag** hat niemand betrachtet.

Zwei weitere Marken kennt keine Liste: `--exec=` und `--receive-pack=` benennen das Programm,
das auf der Gegenseite den Empfang abwickelt, und tragen ihren Wert hinter `=`.

**Damit ist der zweite der drei Sätze im Modulkopf zu stark.** „Die Aufsicht … hält, dass keine
Liste … eine Marke aus `MARKEN` … trägt" — `--del` **ist** für `git` die Marke `--delete`. Der
Vorbehalt unter „Was sie nicht kann" deckt eine Marke, an die niemand gedacht hat; er deckt
nicht, dass eine dastehende Marke in ihrer Kurzform vorbeikommt.

Datensatz:
`shared/issues/260821-1432_o_git-nimmt-abkuerzungen-langer-marken-an-und-der-gleichheitsvergleich-in-marken-faengt-sie-nicht.md`

#### A3 — Die eine Abhilfe für A1 und A2

Punktuelle Flicken gibt es für beide, und beide bleiben Verbotslisten. Der Schnitt, der beide
schließt: `Auftrag` ist eine geschlossene Aufzählung mit sieben festen Wortformen. Statt zu
prüfen, was ein freies Wort **nicht** sein darf, lässt sich je Variante angeben, welche Wörter
fest sind und welche Plätze belegt werden — und für jeden belegten Platz, welche Gestalt er
tragen darf. Aus der Verbotsliste wird eine Erlaubnisliste, wie sie das Modul für die
Unterbefehle schon führt und in `git.rs:208-212` begründet.

Nebeneffekt derselben Umstellung: die heutige Aufsicht liest auch die Eintragsmeldung hinter
`-m` als hätte sie eine Marke sein können — `git commit --only -m "-a" -- Cargo.toml` wird
angehalten, obwohl `-a` dort ein Wert ist. Aus dem heutigen Baum entsteht die Lage nicht, weil
die Meldung fest gefügt wird.

#### A4 — Was ich geprüft und **nicht** gefunden habe

Die drei Vergleichsformen an ihren eigenen Rändern, nachgebaut und laufen gelassen:

- **Erlaubnisliste der Unterbefehle:** `reset`, `clean`, `checkout`, `stash`, `update-ref`
  kommen weder lesend noch schreibend durch; die leere Liste ebenso wenig.
- **Wortanfang `--force`:** fängt `--force`, `--force-with-lease`, `--force-with-lease=…`,
  `--force-if-includes`. Ein legitimes Argument, das damit anfinge, gibt es unter den sieben
  Varianten nicht.
- **Kurze Marken Buchstabe für Buchstabe:** fängt `-f`, `-d`, `-fd`, `-df`, `-am`. Lässt `-m`
  durch, lässt `--` durch, lässt `--only`, `--porcelain`, `--untracked-files=no`, `--git-dir`,
  `--points-at`, `--list` und jeden Pfad durch. `kurze_marke` verlangt „Strich, dann nur
  Buchstaben", und das trennt die Fälle sauber.
- **Der Refspec `refs/tags/v0.5.6` gilt nicht als Gewalt.** Nachgesehen: er kommt durch. Ebenso
  `HEAD`, `origin` und die zwei Dateinamen des Eintrags.
- **Groß geschriebene kurze Marken** (`-D`, `-F`) fallen durch das Raster, weil
  `GEWALTBUCHSTABEN` klein geschrieben ist. Unter den fünf erlaubten Unterbefehlen führt `git`
  keine großgeschriebene Gewaltmarke, also **kein Befund** — aber eine Zeile wert, falls die
  Erlaubnisliste je wächst.

### B. Was die Proben nicht messen

#### B1 — Fünf der sieben Marken, ein Gewaltbuchstabe und die übergehende Marke messen keine Probe (Mittel)

`--tags`, `--all`, `--mirror`, `--delete`, `--amend`, `--no-verify` und der Buchstabe `a`
kommen im Prüfmodul von `git.rs` nirgends als angehalten vor. Wer einen dieser Einträge
löscht, lässt alle 146 Proben grün. Gezählt mit
`awk '/^#\[cfg\(test\)\]/,0' xtask/src/git.rs | grep -c -- '"--all"'` und den entsprechenden
Mustern.

Dieselbe Lage bei `beispiele()` (`:443-458`) und `die_auftraege_stehen_wort_fuer_wort`
(`:507-556`): beide sind `vec!`-Literale und keine Fallunterscheidungen, also hält der
Übersetzer sie nicht — eine achte Variante bliebe ungesehen. `beispiele()` schreibt diese
Grenze für die eigene Zusage aus, und das trägt; für C3.4, dessen erster Satz an der zweiten
Probe hängt, trägt es nicht.

Datensatz:
`shared/issues/260821-1432_o_fuenf-der-sieben-marken-ein-gewaltbuchstabe-und-die-uebergehende-marke-messen-keine-probe.md`

#### B2 — C3.7 ist sauber wiederhergestellt (kein Befund)

Ich habe die wiederhergestellte `keine_der_drei_fragen_schreibt` (`git.rs:479-503`) gegen die
gelöschte Fassung von `465330b` gelesen. Sie misst dasselbe und mehr:

| | alt (`465330b`) | neu (`94855a7`) |
|---|---|---|
| Unterbefehl | nicht in einer **Verbotsliste** von 14 schreibenden | in der **Erlaubnisliste** `LESENDE` — schließt alle 14 aus |
| Einordnung | gar nicht ausdrückbar | `frage.wirkung() == Wirkung::Liest` |
| ganze Liste | gar nicht geprüft | `aufsichtsbefund(…) == None` |
| `tag`-Regel | `--points-at` verlangt, kein Name dahinter | unverändert |

Der Name steht, die drei Fragen sind dieselben, und die Zusage ist strikt stärker geworden.
Das Kriterium ist erfüllt.

### C. Prosa gegen den gebauten Stand

#### C1 — `make ausliefern` nennt das Schieben nicht (Mittel)

`94855a7` behebt den Befund F1 der vorigen Durchsicht an der `##`-Zeile von `release` und
bindet eine Probe daran. Der Weg, den der Nutzer tippt, ist `./release.sh <zahl>`, und der
führt nach `make ausliefern` (`release.sh:36`). Dessen `##`-Zeile (`Makefile:125`) nennt das
Schieben weiterhin nicht. `make help` gibt beide nebeneinander aus, nachgefahren:

```
  ausliefern     Version setzen, eintragen, taggen und ausliefern: make ausliefern VERSION=0.2.0
  release        Bauen, signieren, beglaubigen, HEAD und Tag zu origin schieben, veroeffentlichen
```

Die Begründung im Kommentar über `release` — „denn sie ist es, die `make help` vor dem Tippen
ausgibt" — trägt für `ausliefern` mit einem Zusatz: es wirkt über das Gerät hinaus **und**
schreibt in den Arbeitsbaum.

Datensatz:
`shared/issues/260821-1432_o_die-hilfezeile-von-make-ausliefern-nennt-das-schieben-nicht-obwohl-release-sh-genau-dorthin-fuehrt.md`

#### C2 — `Makefile` und `HILFE`, im Übrigen zutreffend und gebunden (kein Befund)

Die `##`-Zeile von `release` trifft zu: `cargo xtask release` fährt Station 8, und die schiebt
HEAD und `refs/tags/v<zahl>` zu `origin`. Gebunden ist sie an
`release::tests::die_hilfezeile_des_makefiles_nennt_das_schieben` (`release.rs:1218-1236`),
die auf `origin` und `schieben` prüft.

Der `release`-Absatz der `HILFE` (`main.rs:80-105`) trifft ebenfalls zu und ist an
`main::tests::der_abschnitt_zu_release_nennt_das_schieben` gebunden, die auf „schiebt HEAD",
„origin" und „nicht zurücknehmen" prüft. Beide Proben grün.

Der `README`-Abschnitt zu Station 1 und Station 8 trifft zu; er ist an keine Probe gebunden,
was der Bauart des Projekts entspricht.

#### D1 — Der Nachtrag im Plan nennt C3.4 und C3.5 als haltend (Niedrig)

Beide Kriterien nennen `die_schreibenden_kommandos_tragen_keine_gewalt` in `version.rs`, und
die ist mit `94855a7` gelöscht. Der erste Satz von C3.4 hält über
`die_auftraege_stehen_wort_fuer_wort`, der zweite („Die Probe verwirft jede der sechs oben
genannten Marken") nicht — von den sechs ist genau `-f` als angehalten nachgesehen. C3.5 ist
ein **Wegkriterium**: es benennt eine Probe an einem Ort und verlangt ihre Erweiterung. Der
Weg wurde bewusst und begründet verlassen; das heißt „überholt" und nicht „hält weiter".

Der Nachtrag benennt die Ersetzung im Absatz darüber richtig. Was fehlt, ist der Schluss
daraus für die zwei Kriterien.

Datensatz:
`shared/issues/260821-1432_o_der-nachtrag-im-plan-nennt-c3-4-und-c3-5-als-haltend-obwohl-beide-eine-probe-nennen-die-es-nicht-mehr-gibt.md`

#### D2 — Der dritte der drei Sätze im Modulkopf widerspricht sich selbst (Niedrig)

`git.rs:33-37`: „Was **nichts** hält, ist ein zweiter Prozessaufruf an `rufen` vorbei; das hält
weiterhin allein die Probe `xtask_ruft_git_an_genau_einer_stelle`". Erst „nichts hält", dann
„das hält allein die Probe". Die dritte Kategorie des Absatzes ist nicht leer, sondern trägt
eine schwächere Sicherung — und deren Grenze (ein Programmname aus einer Variablen) steht
nirgends.

Der Absatz ist die einzige Stelle im Baum, die die Stärke der Absicherung ausschreibt. An
seinem schwächsten Punkt ist er uneindeutig.

Datensatz:
`shared/issues/260821-1432_o_der-dritte-der-drei-saetze-im-modulkopf-von-git-rs-widerspricht-sich-selbst.md`

## Querliegende Beobachtungen

**Die Aufsicht kennt Wörter, aber keine Plätze.** A1, A2 und der Falschalarm auf eine
Eintragsmeldung sind drei Gestalten derselben Sache. Solange die Prüfung ein flaches Wortfeld
liest, muss sie raten, ob ein Wort ein Schalter, ein Wert, ein Refspec oder ein Pfad ist — und
jede Antwort darauf ist eine Näherung. `Auftrag` weiß es genau: die Aufzählung ist geschlossen
und jede Variante kennt ihre Plätze. Die Auskunft liegt vor, sie wird nur nicht an die
Aufsicht weitergereicht.

**Die Sicherung, die heute wirklich trägt, steht anderswo.** Was verhindert, dass ein
Doppelpunkt oder eine Marke in `Schub.verweis` landet, ist `versionszahl_pruefen` in
`version.rs`. Der Modulkopf von `git.rs` erwähnt sie nicht, und die Aufsicht weiß nichts von
ihr. Diese Runde hat die Zusage von „drei Bauer stehen namentlich da" auf „die Aufzählung ist
die Aufsicht" gehoben — der nächste Schritt derselben Bewegung wäre, dass die Aufsicht die
Gestalt der belegten Plätze selbst kennt, statt sie beim Aufrufer zu belassen.

**Der Befundtyp der vorigen Durchsicht wiederholt sich eine Ebene höher.** A1 jener Durchsicht
lautete: eine Aufzählung von Namen kann nicht zusagen, dass sie vollständig ist. Die Antwort
darauf war richtig — eine geschlossene Aufzählung der **Kommandos**. Die Aufzählung der
**Marken** darunter ist aber weiterhin eine Namensliste, und sie ist weiterhin unvollständig
und diesmal zusätzlich ungemessen (B1). Wer nur die Kommandos schließt, hat die halbe Bewegung
gemacht.

**Die Berichtigung an einer von zwei Stellen.** F1 der vorigen Durchsicht ist an `release`
behoben und an `ausliefern` nicht (C1); B4 ist an Station 1 vorgezogen und an Station 8
belassen, dort begründet und richtig. Der Unterschied: bei B4 hat jemand beide Rufer
nachgezählt, bei F1 nicht.

## Empfohlene Reihenfolge

Kein Befund hält eine Auslieferung auf. Wer sie heute fahren will, kann.

1. **D1 und C1** — zwei Zeilen Prosa und eine `##`-Zeile, beide ohne Codeänderung, beide vor
   dem nächsten Lauf sinnvoll: C1 ist die Auskunft, die der Nutzer unmittelbar vor dem
   Schieben liest.
2. **B1** — die Anhalteproben je Markeneintrag und `beispiele()` erschöpfend machen. Klein,
   und macht den zweiten Satz von C3.4 wieder wahr.
3. **A1 und A2 zusammen** — als eine Umstellung nach A3, nicht als zwei Flicken. Sie ist
   die eigentliche Arbeit dieses Befundsatzes und gehört in eine eigene Runde oder an das Ende
   dieser, nicht zwischen zwei andere Schritte.
4. **D2** — beim Anfassen des Modulkopfs für A2 mitnehmen; es ist derselbe Absatz.

## Was ich nicht geprüft habe

- **Alles gegen GitHub.** `gh` fehlt auf diesem Gerät. Der Schub selbst, die Releaseseite, die
  Ticketprüfung am Netz — ungemessen.
- **Die vier Dateien unter `Not-opened:`.** Drei betreffen die Ablage und den Lesezeichenvorfall,
  einer ist das Sitzungsprotokoll des Coders; keine berührt `xtask`.
- **Der Abnahmelauf der zehn Zusagen aus C8.** Nicht Gegenstand dieses Commits.
- **Ob `git` weitere Kurzformen annimmt, die ich nicht ausprobiert habe.** Ich habe fünf
  gemessen; die Liste ist nicht erschöpfend, und das ist genau der Grund, aus dem A3 eine
  Erlaubnisliste vorschlägt statt einer längeren Verbotsliste.
