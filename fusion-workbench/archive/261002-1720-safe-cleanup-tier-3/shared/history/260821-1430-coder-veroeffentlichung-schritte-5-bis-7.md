# Umsetzung: die Schritte 5 bis 7 des Veröffentlichungswegs

**Datum:** 2026-08-21
**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `3175d76`
**Plan:** `shared/planning/260821-1221_o_plan-artefakt-und-release.md`, Schritte 5 bis 7

## Was entstanden ist

Die drei wirkenden Hälften der achten Station — schieben, anlegen, verdrahten — und die eine
Umformulierung, die der Plan ausdrücklich verlangt hat: die Aufsicht über die schreibenden
Git-Kommandos deckt jetzt drei statt zwei.

### Schritt 5 — Schieben, und die Aufsicht auf drei Kommandos

`xtask/src/veroeffentlichung.rs` bekommt vier neue reine Funktionen und zwei Rufer daneben:

- `tagname(zahl) -> String` liefert `v<zahl>`; das Modul nennt den Tag an drei Stellen, deshalb
  steht die Fügung einmal da.
- `tagverweis(tag) -> String` liefert `refs/tags/<tag>`.
- `schiebe_argumente(tagverweis) -> Vec<&str>` liefert genau vier Wörter: `push`, `origin`,
  `HEAD`, `refs/tags/<tag>`. **Der Rückgabetyp ist `Vec<&str>` wie bei den zwei älteren Bauern**,
  damit die eine Aufsicht alle drei gleich liest; ein hier zusammengesetztes Wort könnte dieser
  Vektor nicht besitzen, deshalb kommt der Verweis fertig herein und wird nebenan in
  `tagverweis` gefügt. Das ist die einzige Abweichung von der im Plan notierten Signatur
  `schiebe_argumente(tagname: &str) -> Vec<&str>`, und sie ist eine, weil jene Signatur in Rust
  nicht baubar ist.
- `tagstand_pruefen(tags_auf_head, tag) -> Result<(), String>` ist die reine Hälfte der Tagfrage,
  `ohne_tag_meldung` ihre Meldung; sie nennt den erwarteten Namen und daneben die, die
  stattdessen auf HEAD stehen.
- `tagstand_fragen` und `schieben` sind die zwei Prozesshälften. Beide gehen über `git::rufen`
  und legen keine zweite Git-Aufrufstelle an.

**`auslieferungsstand_pruefen` wird nicht gerufen**, und sein Name steht in diesem Modul auch
nicht ausgeschrieben: die Probe `allein_release_fragt_nach_tag_und_arbeitsbaum` hält ihn an
genau einer Datei fest, und ein Verweis von hier trüge ihn an eine zweite. Der Doc-Kommentar von
`tagstand_fragen` sagt das ausdrücklich.

**Die Aufsicht ist geteilt und nicht umgangen.**
`version::tests::die_schreibenden_kommandos_tragen_keine_gewalt` liest jetzt drei Listen, und die
Zusage steht in zwei Hälften:

1. **Das erste Wort ist der Unterbefehl, auf Gleichheit geprüft**, je Kommando einzeln: `tag`,
   `commit`, `push`. Damit ist `push` an der einen Stelle erlaubt und an den zwei anderen
   ausgeschlossen, ohne Ausnahmeliste. `add` fällt aus der Markenliste heraus, weil diese Hälfte
   es abdeckt.
2. **Die Wörter danach tragen keine Marke, die Reichweite oder Gewalt hinzufügt** — die sechs aus
   C3 (`--force`, `-f`, `--tags`, `--all`, `--mirror`, `--delete`) und die drei, die schon
   dastanden (`--amend`, `--no-verify`, `-a`).

Der Prüfkommentar schreibt beides aus und sagt, dass der dritte Bauer in `veroeffentlichung`
steht, während die Aufsicht bleibt, weil es eine ist. Der Modulkopf von `xtask/src/git.rs` ist
nachgezogen: drei schreibende Kommandos an zwei Orten, eine Aufsicht.

### Schritt 6 — Die Releaseseite mit festem Text

`RELEASETEXT` ist eine Konstante mit `ZAHLPLATZHALTER` (`{zahl}`) als einziger Art von Fügestelle;
`releasetext(zahl)` setzt sie mit `str::replace`, `releasetitel(zahl)` liefert `KRK <zahl>`.

Der Text folgt dem Abschnitt „Betriebsregel für den Austausch der App" aus
`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md` und ist so formuliert, dass
ein Nutzer ihn ohne Kenntnis der Untersuchung versteht: Voraussetzung macOS 15, das Bündel ist
beglaubigt und startet ohne Rückfrage, drei Installationszeilen, die benannte Folge des Löschens
samt Ordner und Inhalt, und die Absicherung für den Fall, dass doch gelöscht werden muss.

**Dieser Text trägt Umlaute, der Rest des Moduls nicht.** Das ist eine bewusste Festlegung und im
Doc-Kommentar begründet: die Abbruchmeldungen dieses Baums sind Terminaltexte und stehen in
Umschrift, dieser hier ist veröffentlichte Prosa auf einer Webseite und folgt darin der
`README.md`, dem einzigen anderen Text dieses Projekts, den Fremde zu lesen bekommen.

`releaseseite_anlegen` stellt **zuerst die Existenzfrage** über ein eigenes `gh release view` und
deutet nicht die Fehlermeldung von `gh release create`; `release_steht` misst allein den
Rückgabewert. Steht das Release, bricht der Lauf mit `release_steht_meldung` ab und überschreibt
nichts. Angelegt wird gleich öffentlich, ohne die Marke für einen Entwurf und ohne die für eine
Vorabfassung — die zwei Wörter stehen deshalb nirgends im Modul, auch nicht in einem
Doc-Kommentar, damit die Quelltextprobe sie verbieten kann.

### Schritt 7 — Verdrahten

Die Verteilungszeile in `main.rs` hat der Vorgänger vorgezogen; hier kam dazu:

- `release::ausfuehren` ruft hinter `beglaubigung::beglaubigen` die achte Station:
  `veroeffentlichung::veroeffentlichen(env!("CARGO_PKG_VERSION"), Tagfrage::Erledigt)`.
- Der Modulkopf von `release.rs` führt die achte Station in derselben Form wie die sieben davor.
- Die zwei Quelltextproben: `die_achte_station_steht_hinter_der_beglaubigung` in `release.rs` und
  `dieser_weg_baut_nichts` in `veroeffentlichung.rs`.

**`Tagfrage` statt eines Wahrheitswerts.** Der Unterschied zwischen den zwei Rufern ist genau eine
Frage — der eigenständige Weg fragt nach dem Tag auf HEAD, die Station nicht, weil Station 1
dieselbe Wahrheit schon beantwortet hat. Ausgedrückt ist er als vollständige Fallunterscheidung
ohne Auffangzweig: an der Aufrufstelle steht dann der Grund und nicht ein nacktes `false`, und ein
dritter Rufer hielte den Bau an, statt sich stillschweigend für eine Seite zu entscheiden. Der
Rumpf ist beiden gemeinsam; zwei Rümpfe nebeneinander wären zwei Antworten darauf, was
Veröffentlichen heißt.

## Was sich am Bestand geändert hat

| Datei | Art der Änderung |
|---|---|
| `xtask/src/veroeffentlichung.rs` | 463 → 981 Zeilen: Schieben, Releaseseite, `Tagfrage`, `veroeffentlichen`; 9 → 16 Proben |
| `xtask/src/version.rs` | die Aufsicht deckt drei Kommandos statt zwei, mit ausgeschriebener Zusage |
| `xtask/src/git.rs` | Modulkopf: drei Bauer an zwei Orten, eine Aufsicht |
| `xtask/src/release.rs` | achte Station im Rumpf und im Modulkopf, eine Quelltextprobe |

## Abnahme

`make check` — Rückgabewert 0. Bau, 129 Proben, `fmt --check` und `clippy -- -D warnings` laufen
grün; die Kiste `xtask` trägt jetzt 129 Proben, acht mehr als vor dieser Sitzung.

Die drei Aufsichtsproben, die unangetastet bleiben mussten, sind grün geblieben:
`xtask_ruft_git_an_genau_einer_stelle`, `keine_der_drei_fragen_schreibt` und
`allein_release_fragt_nach_tag_und_arbeitsbaum`.

Zwei Läufe am Gerät, beide ohne `gh` — es ist auf diesem Gerät nicht installiert:

- `cargo xtask veroeffentlichen 0.5.6` bricht an der ersten Stufe ab, nennt das Werkzeug und
  `brew install gh`, Rückgabewert 1. Danach liegt kein `target/KRK-*.zip`, und geschoben ist
  nichts.
- `cargo xtask veroeffentlichen` ohne Argument endet mit Rückgabewert 2.

**Was auf den Nutzer wartet.** Alles, was einen Lauf gegen GitHub verlangt. Die 13 Kriterien der
Tabelle „Abnahme durch den Nutzer" im Plan sind unverändert offen; keine Probe dieser Sitzung
behauptet einen Lauf gegen den Dienst. Vorbedingungen bleiben `gh` installieren, `gh auth login`
und der einmalige Handgriff `git push origin --tags`.

## Was offen bleibt und nicht in diesen Auftrag gehörte

- Der Modulkopf von `release.rs` **sagt weiterhin „Der Weg in sieben Stationen"** und listet
  darunter acht. Das ist Schritt 9 des Plans, der die sieben Prosastellen im Quellbaum mit einer
  eigenen Zählprobe nachzieht; ein halber Vorgriff hier hätte jener Probe ihren Gegenstand
  genommen. Bis Schritt 9 läuft, steht die Stelle bewusst uneinheitlich da.
- Schritte 8, 10 und 11: Hilfetext samt Defekt `260815-1436`, `README.md`, Abnahme am Gerät.
