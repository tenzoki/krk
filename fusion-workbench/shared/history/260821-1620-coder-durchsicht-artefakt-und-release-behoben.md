# Umsetzung: die Befunde der Durchsicht „Artefakt und Release"

**Datum:** 2026-08-21
**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `bc688c8`
**Durchsicht:** `shared/reviews/260821-1346-coderev-artefakt-und-release.md`, neun Befunde
**Plan:** `shared/planning/260821-1221_*_plan-artefakt-und-release.md`

## Was entstanden ist

Alle Befunde der Durchsicht sind behoben, einschließlich der zwei, die sie als Frage und nicht
als Auflage vorgelegt hat. Der schwerste war nicht der neue Code, sondern die Aufsicht darüber;
sie hat eine andere Bauform bekommen und keinen vierten Namen.

## A — Die Aufsicht über die schreibenden Git-Kommandos (Befund A1, „Hoch")

**Die Aufsicht zählte drei Bauer namentlich auf, und ein vierter stand schon daneben.**
`version::tagliste_argumente` reichte eine gebaute Liste an `git::rufen`, ohne dass die Aufsicht
sie las; gedeckt war er allein davon, dass jemand ihm eine eigene Probe gegeben hatte. Ein
fünfter hätte übersetzt und alle Proben bestanden.

**Die gewählte Bauform bindet statt aufzuzählen.** `git::rufen` nimmt seit dieser Änderung keine
nackte Wortliste mehr entgegen, sondern einen `git::Auftrag` — die vollständige Aufzählung jedes
Kommandos, das dieses Werkzeug an `git` reicht, mit sieben Varianten. `Auftrag::worte` und
`Auftrag::wirkung` sind beide vollständige Fallunterscheidungen ohne Auffangzweig. Damit hält
der Übersetzer zweierlei: dass keine Liste an der Aufzählung vorbei bei `git` ankommt, und dass
eine neue Variante den Bau anhält, bis sie ihre Wörter genannt und sich als lesend oder
schreibend eingeordnet hat. Das ist dieselbe Bauart, die das Projekt für `Wirkungsbereich`,
`Bereich` und `Fokus` führt.

**Dazu steht die Aufsicht jetzt auf dem Weg statt daneben.** `git::aufsichtsbefund` läuft in
`git::rufen` vor jedem Prozessaufruf und liest die Liste, die wirklich hinausgeht — auch die
einer neuen Variante, die niemand nachgesehen hat. Vier Fragen: es steht ein Unterbefehl da; er
steht in der Erlaubnisliste, die zur `Wirkung` gehört; ein lesendes `tag` trägt `--points-at`
oder `--list`; kein Wort dahinter trägt Gewalt.

**Die drei Bauer sind zu Varianten geworden** und stehen nicht mehr bei ihren Abnehmern.
`version` und `veroeffentlichung` behalten die Entscheidung, einen Auftrag zu erteilen, und ihre
Begründung; die Wörter stehen dort, wo die Aufsicht steht. Beide Modulköpfe schreiben aus, warum.

**Wie stark die Zusage danach wirklich ist**, steht im Modulkopf von `git.rs` in drei Sätzen
und wird hier nicht schöner gemacht: der Übersetzer hält die Aufzählung, die Aufsicht auf dem
Weg hält jede Liste, und **nichts** hält einen zweiten Prozessaufruf an `git::rufen` vorbei —
das hält weiterhin allein die Probe `xtask_ruft_git_an_genau_einer_stelle`.

### Befund A2 — die drei Lücken der Markenliste

`-d` als kurze Form von `--delete` bei `push` und bei `tag`, `--force-with-lease` und
`--force-if-includes`, `--prune`, dazu der Refspec mit führendem `+`. Alle geschlossen, und die
Prüfung ist an drei Stellen von der Gleichheit weggegangen, weil Gleichheit hier zu wenig fängt:

- **Die Unterbefehle sind Erlaubnislisten** (`LESENDE`, `SCHREIBENDE`) und keine Verbotslisten.
  `reset`, `clean`, `checkout`, `restore`, `stash` kommen nicht durch, ohne dass sie jemand
  einzeln verboten hätte. Eine Verbotsliste fällt zur bequemen Seite, eine Erlaubnisliste zur
  sicheren.
- **`--force` wird am Wortanfang geprüft**, damit `--force-with-lease=<verweis>` mitfällt.
- **Kurze Marken werden Buchstabe für Buchstabe geprüft**, damit `-fd` nicht durchkommt. `-m`
  bleibt zulässig, weil `m` kein Gewaltbuchstabe ist.

Zusätzlich in die Liste: `--follow-tags`, das die Reichweite genauso erweitert wie `--tags`.

Die Proben stellen der Aufsicht ausdrücklich Listen, die heute niemand baut — genau der Fall,
für den sie gebaut ist.

### Befund F3/A1 — der Modulkopf von `git.rs`

Der Satz „sie liest die Listen, die hier ankommen, und keine anderen" traf auf drei von vier zu.
Der Kopf ist neu geschrieben und sagt jetzt getrennt, was der Übersetzer hält, was die Aufsicht
auf dem Weg hält und was nichts hält.

## B — Vor der ersten Auslieferung

**Befund F1 — `make release` beschrieb sich falsch.** Die `##`-Zeile in `Makefile:130` nannte
nur „bauen, signieren, beglaubigen" und verschwieg das Schieben zu `origin`, also die einzige
Wirkung der Runde, die sich nicht zurücknehmen lässt. Sie nennt es jetzt, der `release`-Absatz
der `HILFE` ebenso. **Beide Stellen sind an eine Probe gebunden**
(`die_hilfezeile_des_makefiles_nennt_das_schieben`, `der_abschnitt_zu_release_nennt_das_schieben`),
denn die Zählprobe der Runde konnte die Stelle nicht fangen: eine Zählprobe fängt, was falsch
dasteht, nie, was fehlt.

**Befund B3 — eine Meldung sagte an zwei von drei Stellen das Gegenteil.** `gh_fehlt_meldung`
endete mit „Es ist nichts gepackt und nichts veroeffentlicht", und an den zwei späten
Verwendungsstellen lag das Zip und war geschoben. Die geteilte Hälfte trägt jetzt nur, was
überall stimmt — Werkzeug, Grund, Abhilfe —, und der Stand steht in zwei eigenen reinen
Funktionen: `vorab_ohne_gh_meldung` mit dem alten Schlusssatz und `spaet_ohne_gh_meldung` mit
der wahren Auskunft, dass gepackt und geschoben ist und allein die Releaseseite fehlt.

## C — Ehrlichkeit der Aufzeichnung

**Die Abnahmezahlen des Plans stimmten nicht.** C1.4 trug in der Zuordnung „Quelltextprobe, dazu
Nutzer" und fehlte in der Tabelle der Nutzerabnahme; C1.5 nennt sein Mittel selbst
(Änderungszeiten des Bündelinhalts), war aber mit einer Nadelprobe über den Quelltext
abgenommen. Beide stehen jetzt in der Tabelle, C1.5 hat in der Zuordnung den Zusatz bekommen,
und die Zahlen lauten **25 abgenommen, 15 beim Nutzer** statt 27 zu 13 — daneben der ehrlichere
vierteilige Schnitt: 21 an Proben, 4 am Lesen, 9 allein beim Nutzer, 6 halb und halb. Die Regel,
aus der beide Fehler folgten, steht jetzt im Plan: wo eine Quelltextprobe steht, gehört „dazu
Nutzer" daneben, außer die Zusage ist selbst eine über den Text.

**Befund F2** — die zwei Zeilen `/// Alle .rs-Dateien des Baums…` hingen seit `465330b` am
Doc-Block der neuen Zählprobe statt an `rust_dateien`. Zurückgeschoben.

## Die zwei freigestellten Punkte, beide mitgenommen

**Befund B1 — die Reihenfolgeprobe hält jetzt sechs Schritte statt vier.** Die zwei, die
fehlten, lagen beide in der prüfenden Hälfte: die Tagfrage und die Ticketprüfung. Die Probe ist
zu einer Kette über alle sechs Stellen geworden und benennt beim Ausfall, welches Paar in der
falschen Reihenfolge steht.

**Befund B4 — `gh_pruefen` steht jetzt zusätzlich in Station 1.** C5.1 war wörtlich schon
erfüllt, die Begründung des Specs nicht: auf dem `release`-Weg war am Kopf der achten Station
bereits eine Einreichung bei Apple gelaufen. Die achte Station behält ihre eigene Prüfung, denn
ihr zweiter Rufer hat keine Station vor sich; `bundle` und `make check` bekommen keine
Abhängigkeit von `gh`. Nachgezogen sind der Modulkopf von `release`, der Hilfetext und vier
Stellen der `README.md`, dazu eine neue Probe
`die_aeussere_voraussetzung_steht_vor_der_ersten_uebersetzung`.

**Die zwei Anmerkungen aus G1/H1** sind mitgenommen, weil die `README.md` ohnehin angefasst
wurde: die Zweckangabe zu `gh` in der Voraussetzungstabelle sagt jetzt, wofür es gebraucht wird,
und der Verweis auf die Werkbank-Untersuchung im Abschnitt „Installieren und aktualisieren"
sagt dazu, dass der Leser des ausgelieferten Zips diese Datei nicht hat und nicht braucht.

## Was ausdrücklich nicht geschehen ist

- **Der Defekt `shared/issues/260813-0026_o_bundle-und-release-schreiben-an-denselben-ort-…`
  bleibt offen.** Keine Datei dieser Arbeit behauptet einen Abschluss.
- **Die zwei offenen Entscheidungsdatensätze** (`260821-1115` zur Hülle, `260821-1221` zum
  Suchpfad) bleiben offen und sind nicht angefasst.
- **Nichts ist gegen GitHub gemessen.** `gh` fehlt auf diesem Gerät, und keine neue Probe
  behauptet einen Lauf gegen den Dienst. Die Prüfung auf `gh` in Station 1 ist am Quelltext
  abgenommen und nicht an einem Lauf.

## Abnahme

```
make check   → exit 0
```

Alle vier Kommandos grün: `cargo build --workspace`, `cargo test --workspace` (xtask: 146
Proben, vorher 134), `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`.

## Geänderte Dateien

- `xtask/src/git.rs` — `Auftrag`, `Wirkung`, `aufsichtsbefund`, `rufen` mit der Aufsicht auf dem
  Weg
- `xtask/src/version.rs` — drei Bauer entfallen, Aufrufe über Varianten
- `xtask/src/veroeffentlichung.rs` — Schieben über die Variante, die geteilte `gh`-Meldung,
  `gh_pruefen` mit zweitem Rufer, die Reihenfolgeprobe über sechs Glieder
- `xtask/src/release.rs` — `gh_pruefen` in Station 1, Doc-Kommentar zurückgeschoben, zwei neue
  Proben
- `xtask/src/main.rs` — der `release`-Absatz der `HILFE`, Station 1, eine neue Probe
- `Makefile` — die `##`-Zeile von `release`
- `README.md` — Station 1, Station 8, die Voraussetzungstabelle, zwei Anmerkungen
- `fusion-workbench/shared/planning/260821-1221_c_plan-artefakt-und-release.md` — die Zahlen,
  die zwei Tabellenzeilen, der Nachtrag
