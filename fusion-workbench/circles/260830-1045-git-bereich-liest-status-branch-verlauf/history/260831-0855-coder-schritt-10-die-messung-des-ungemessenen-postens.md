# Coder-Sitzung: Schritt 10 der Runde 23, die Messung des ungemessenen Postens

**Date:** 2026-08-31
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 10
**HEAD:** `5a1cbe8` (nicht committet; der Orchestrator committet)

## Das Ergebnis zuerst: die zweite Bedingung aus `## Stops when` ist eingetreten

Der Posten übersteigt die Vergleichsgröße in allen drei gemessenen Bäumen und in beiden
Läufen. Die Runde ist damit vor ihrem Abschluss angehalten, und die drei Möglichkeiten
des Datensatzes
`shared/decisions/260830-1006_*_darf-stufe-a-den-aufgefrischten-index-zurueckschreiben-…`
liegen dem Nutzer erneut vor.

| Baum | Posten (D minus C) | Statusabfrage (C) | Verhältnis |
|---|---|---|---|
| KRK-Klon, 2 239 verfolgte Dateien | 56,3 und 59,0 ms | 12,7 und 11,2 ms | 4,4 und 5,3 |
| angelegt, 10 000 Einträge | 35,8 und 41,3 ms | 21,0 und 15,2 ms | 1,7 und 2,7 |
| angelegt, 100 000 Einträge | 1 369 und 1 367 ms | 146,6 und 144,4 ms | 9,3 und 9,5 |

Je Zelle stehen die zwei unabhängigen Läufe. Der Bericht ist
`messungen/260831-0855-needsupdate.txt`.

**Dieser Schritt entscheidet nichts.** Ob Stufe A zurückschreiben darf, bleibt die
Nutzerfrage, die sie war; ein Schreibweg ist nicht gebaut und war auch für den Fall des
Eintritts ausdrücklich ausgeschlossen.

## Was gemessen wurde, und warum es diese vier Reihen braucht

Der Posten ist keine Größe, die ein einzelner Lauf hergibt: er ist die **Differenz**
zwischen einer Statusabfrage auf veraltetem und einer auf frischem Index. Vier Reihen je
Baum, je drei Durchgänge:

```
A  gix-Status ohne Rueckschreiben, Index veraltet  -> jeder Durchgang zahlt erneut
B  gix-Status mit write_changes                    -> Durchgang 1 zahlt, 2 und 3 nicht
C  Gitleser::marken bei frischem Index             -> die Vergleichsgroesse
D  Gitleser::marken bei veraltetem Index           -> was KRK heute je Ordnerwechsel zahlt
```

A und B fahren den rohen `gix`-Weg mit derselben Plattform-Einstellung, die
`Gitleser::marken` setzt. C und D fahren `krk_core::git::leser::Gitleser` unmittelbar,
also den ausgelieferten Code; das Repository wird über alle Durchgänge festgehalten, wie
KRK es hält.

**Die Reihe A ist die Reihe, die die Behauptung des Datensatzes prüft** — „wer nicht
zurückschreibt, zahlt die Auffrischung bei jeder Abfrage erneut". Drei Durchgänge
hintereinander auf demselben Baum kosten 75,5 / 68,5 / 74,1 ms. Der zweite Blick auf
denselben Ordner wird ohne Rückschreiben nicht billiger als der erste.

**„Frisch angefasst" heißt: jede verfolgte Datei wird mit demselben Inhalt neu
geschrieben.** Der Zeitstempel bewegt sich, der Inhalt und die Größe nicht — genau der
Fall, den `git` und `gix` als `NeedsUpdate` führen.

## Der Aufbau ist gegen die Analyse geeicht

Die Reihe C misst dieselbe Größe, die die Machbarkeitsanalyse vom 260830 unter Frage 4
als „warm" führt, und trifft sie: 11,2 und 12,7 ms gegen deren 11,4–12,1 ms für KRKs
Repository, 144,4 und 146,6 ms gegen deren 155–164 ms für 100 000 Einträge. Die
Übereinstimmung ist der Beleg, dass der neue Aufbau dieselbe Größe misst wie der alte;
ohne sie stünde die Differenz D minus C ohne Anschluss da.

## Wo das Programm steht, und warum nicht unter `crates/`

Der Vergleichslauf ruft `Outcome::write_changes`. C3.8 und C10.3 verlangen, dass der
Projektbaum keinen Aufruf trägt. Das Prüfprogramm steht deshalb in einem
Wegwerf-Workspace außerhalb des Projektbaums (Sitzungs-Scratchpad,
`needsupdateprobe`), mit `krk-core` als Pfadabhängigkeit und `gix 0.87.1` in derselben
Merkmalswahl wie die Wurzel-`Cargo.toml`. Der Plan hätte ein Prüfprogramm unter
`crates/krk-bench/` erlaubt; es ist keines entstanden, und damit wächst der
Auslieferungsbaum um diesen Schritt um keine Zeile.

Ein zweiter Grund kommt dazu: die Probe
`git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen`
(`xtask/src/release.rs`) liest jede `.rs`-Datei unter der Projektwurzel und zählt die
`git`-Aufrufe. Die drei Aufrufe, mit denen das Prüfprogramm seine Wegwerf-Repositorys
anlegt, wären ihr in den Weg gelaufen.

**Der Projektbaum ist nicht gemessen und seine Zeitstempel sind nicht angefasst.** Der
eine Zugriff auf ihn ist ein `git clone --no-hardlinks` in den Messplatz, der liest. Die
Wegwerf-Repositorys lagen unter `~/Library/Caches/krk-messplatz/needsupdate-<pid>` und
sind nach dem Lauf entfernt.

## Was der Lauf nebenbei gefunden hat

`gix` 0.87.1 liefert keinen einzigen `NeedsUpdate`-Posten an den Rufer aus. Sein
Statusiterator fängt ihn selbst ab (`Iter::maybe_keep_index_change`,
`gix-0.87.1/src/status/iter/mod.rs:296`), legt ihn in `Outcome::changes` und gibt `None`
zurück; nach außen steht allein `Outcome::has_changes()`, ein Ja oder Nein ohne Zahl.

Damit ist der Zweig `EntryStatus::NeedsUpdate(_) => return None` in
`posten_deuten` (`crates/krk-core/src/git/leser.rs:398`) unerreichbar, und die drei
Prosastellen, die ihn als die Stelle beschreiben, an der KRK den Posten „liest und
verwirft", beschreiben nicht den Mechanismus, der die Zusage trägt. Die Zusage hält der
fehlende Aufruf von `write_changes` und sonst nichts. Der Befund ist gefilt:
`issues/260831-0855_o_der-zweig-fuer-needsupdate-in-posten-deuten-ist-unerreichbar-…`.

Die Folge für die Zahl im Bericht: sie ist **nicht gezählt**, sondern aus dem Aufbau
bekannt — alle verfolgten Dateien sind angefasst. Der Bericht schreibt das als
Einschränkung 2 aus.

## Prüfmittel von C10.3, mit dem heutigen Stand

- `grep -rn 'write_changes(' crates/` — keine Fundstelle. Das ist die Prüfung, die trägt.
- `grep -rn 'write_changes' crates/` — zwei Treffer, beide Prosa in einem Modulkopf
  (`git/mod.rs:17`, `git/leser.rs:50`), beide sagen, dass der Weg **nicht** gerufen wird.
  Sie stammen aus Schritt 3 und nicht aus diesem Schritt. Der Wortlaut von C3.8 („keine
  Fundstelle") ist damit weiterhin nicht erfüllt, und das ist der schon offene Widerspruch
  aus `issues/260830-1614_o_c3-8-verlangt-null-treffer-fuer-write-changes-…`, den der Plan
  in Schritt 3 zugunsten von C10.3 entschieden hat.

## Abnahme

`make check` — exit 0.

## Dateien

- `messungen/260831-0855-needsupdate.txt` (neu)
- `fusion-workbench/circles/260830-1045-git-bereich-liest-status-branch-verlauf/issues/260831-0855_o_der-zweig-fuer-needsupdate-in-posten-deuten-ist-unerreichbar-gix-faengt-den-posten-vorher-ab.md` (neu)
- `fusion-workbench/circles/260830-1045-git-bereich-liest-status-branch-verlauf/planning/260830-1317_p_plan-git-bereich-liest-status-branch-verlauf.md` (Schritt 10 auf `[DONE]`, die Klausel unter `## Where this Circle stops` mit dem Messergebnis)

Kein Quelltext unter `crates/`, `xtask/` oder `resources/` ist angefasst.
