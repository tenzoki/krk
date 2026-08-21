Der Nachtrag im Plan nennt C3.4 und C3.5 als haltend, obwohl beide eine Probe nennen, die es nicht mehr gibt

---

Der mit `94855a7` angefügte Nachtrag im Plan „Artefakt und Release" sagt: „Der Zuordnung ändert
das nichts: **C3.4 hält weiter** …, **C3.5 hält weiter und deckt statt drei Kommandos alle
sieben**". Beide Kriterien nennen die Probe `die_schreibenden_kommandos_tragen_keine_gewalt`
in `xtask/src/version.rs`, und die ist mit demselben Commit gelöscht. Der zweite Satz von C3.4
verlangt außerdem etwas, das keine Probe mehr misst.

---

**Gemessen am Baumstand `94855a7`.**

## C3.4

Der Wortlaut im Spec (`shared/planning/260821-1115_o_spec-artefakt-und-release.md:177-180`):

> Die Argumentliste des schiebenden Kommandos wird von einer Probe Wort für Wort nachgesehen,
> wie es `die_schreibenden_kommandos_tragen_keine_gewalt` in `xtask/src/version.rs` für Tag und
> Eintrag tut. **Die Probe verwirft jede der sechs oben genannten Marken.**

Die sechs stehen in der Beschreibung von C3 (`:166`): `--force`, `-f`, `--tags`, `--all`,
`--mirror`, `--delete`.

Der erste Satz hält, und der Nachtrag benennt die neue Trägerin richtig:
`git::tests::die_auftraege_stehen_wort_fuer_wort` sieht alle sieben Listen Wort für Wort nach.

**Der zweite Satz hält nicht.** Von den sechs Marken ist genau eine als angehalten
nachgesehen, `-f` in `die_kurze_marke_des_eintrags_bleibt_zulaessig` (`git.rs:657`). `--force`
kommt im Prüfmodul nur in `die_aufsichtsmeldung_nennt_kommando_und_befund` (`:663`) vor, und
dort wird der Wortlaut der Meldung geprüft, nicht das Anhalten. `--tags`, `--all`, `--mirror`
und `--delete` kommen im ganzen Prüfmodul nicht vor. Die Zählung steht im verwandten
Datensatz.

Die alte Probe erfüllte den Satz, weil sie in die andere Richtung las: sie sah die **gebauten**
Listen gegen alle neun Marken an. Die neue prüft die Aufsicht, und deren Liste misst niemand
Eintrag für Eintrag.

## C3.5

Der Wortlaut (`:181-183`):

> Die vorhandene Probe in `xtask/src/version.rs` wird bewusst erweitert und nicht umgangen:
> nach der Änderung deckt die Aufsicht **drei** schreibende Kommandos ab statt zwei, und die
> Erweiterung steht als solche im Prüfkommentar.

Das Kriterium ist kein Ergebniskriterium, sondern ein Wegkriterium: es benennt eine Probe an
einem Ort und verlangt, dass sie erweitert und nicht umgangen wird. Mit `94855a7` ist sie
gelöscht und durch etwas anderes ersetzt.

**Das ist der Sache nach richtig so.** Die Ersetzung ist stärker als die Erweiterung, sie
wirkt zur Laufzeit statt in einer Probe, und sie ist die Antwort auf einen Befund, den das
Kriterium selbst nicht vorhersehen konnte. Falsch ist allein die Aufzeichnung: ein Wegkriterium,
dessen Weg verlassen wurde, „hält" nicht — es ist **überholt**, und die Aufzeichnung soll das
sagen können.

## Was daran nicht Befund ist

C3.6 und C3.7 hält der Nachtrag zu Recht. Ich habe C3.7 gegen die alte Fassung gelesen: die
wiederhergestellte `keine_der_drei_fragen_schreibt` (`git.rs:479-503`) ist **stärker** als die
gelöschte. Die alte prüfte gegen eine Verbotsliste von vierzehn schreibenden Unterbefehlen; die
neue prüft gegen die Erlaubnisliste `LESENDE` (`rev-parse`, `tag`, `status`), die alle vierzehn
ausschließt, und behauptet zusätzlich `frage.wirkung() == Wirkung::Liest` und
`aufsichtsbefund(…) == None`, was die alte Fassung gar nicht ausdrücken konnte. Die
Fallunterscheidung um `tag` steht unverändert. Das Kriterium ist erfüllt, und der Name ist
gehalten.

## Abhilfe

Drei Zeilen im Nachtrag berichtigen, keine Änderung am Code:

- **C3.4** teilen: erster Satz hält über `die_auftraege_stehen_wort_fuer_wort`, zweiter Satz
  hält **nicht** und wartet auf die Anhalteproben je Markeneintrag (verwandter Datensatz).
- **C3.5** als **überholt** führen statt als haltend, mit dem Grund: das Kriterium beschreibt
  einen Weg, der bewusst und begründet verlassen wurde.
- Der Satz „Der Zuordnung ändert das nichts" fällt damit weg; die Zuordnungstabelle in Zeile
  505-506 zieht nach.

**Schwere:** niedrig. Kein Fehlverhalten und keine Änderung an dem, was gebaut ist. Es ist die
Aufzeichnung eines abgenommenen Kriteriums, und dieses Projekt führt „Kriterium verspricht eine
Probe und hat keine" als eigene Defektklasse — der Nachtrag selbst nennt sie zwei Absätze über
der Stelle.

**Gefunden:** coderev, Durchsicht des Commits `94855a7` am 260821-1432, Bereich
`465330b..94855a7`

**Betroffen:** `shared/planning/260821-1221_c_plan-artefakt-und-release.md:622-644` (Nachtrag),
`:505-506` (Zuordnung), `shared/planning/260821-1115_o_spec-artefakt-und-release.md:177-183`

**Domain:** code

**Verwandt:**
`shared/issues/260821-1432_o_fuenf-der-sieben-marken-ein-gewaltbuchstabe-und-die-uebergehende-marke-messen-keine-probe.md`
— die Messseite desselben Sachverhalts. Wer die Anhalteproben nachzieht, macht den zweiten
Satz von C3.4 wieder wahr und braucht dann nur noch C3.5 umzumarkieren.

---

Resolved: Behoben am 260821 im Plan `shared/planning/260821-1221_c_plan-artefakt-und-release.md`,
ohne Änderung am Code.

Der Satz „Der Zuordnung ändert das nichts" ist gefallen. An seiner Stelle steht ein Abschnitt,
der die vier Kriterien einzeln führt: **C3.4** hält in beiden Sätzen, seit die Anhalteproben je
Markeneintrag stehen (verwandter Datensatz) — der erste Satz über
`die_auftraege_stehen_wort_fuer_wort`, der zweite über `jede_einzelne_marke_wird_angehalten` und
`jeder_eintrag_der_listen_steht_in_der_anhalteprobe`. **C3.5** ist als **überholt** geführt, mit
dem Grund, den dieser Datensatz nennt: ein Wegkriterium, dessen Weg bewusst und begründet
verlassen wurde, hält nicht. **C3.6 und C3.7** stehen unverändert. Die Zuordnungstabelle in Zeile
505-506 ist nachgezogen; C3.5 trägt dort statt „Probe" den Vermerk „überholt".

Daneben ist ein zweiter Nachtrag angefügt, der ausschreibt, was die Durchsicht vom 260821-1432
am gebauten Stand geändert hat — nach demselben Muster wie der erste.
