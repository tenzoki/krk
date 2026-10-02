# Abgleich 260821-1532 — Abschluss der Sitzung 260820-2200

**Status:** Complete
**Reconciler**, Domäne `code`, Baumstand `4e810f9`, Arbeitsbaum sauber, `origin/main` steht auf
`01d2365` — geschoben ist nichts.
**Anlass:** Abschluss-Abgleich der Sitzung `260820-2200`, mit vier vom Nutzer ausdrücklich
benannten Stellen darin.
**Bereich:** `01d2365..4e810f9`, 15 Commits, davon 7 am Quellbaum.

## Zur Lage

`agentstate.yaml` steht noch (die Sitzung ist nicht sauber beendet), `.active-circle` fehlt.
**Kein Circle war in dieser Sitzung aktiv**, und das ist selbst ein Befund; er steht unten unter
`## Was die Coherence-Auskunft trägt`. Spec und Plan liegen deshalb nach der Herkunftsregel in
`shared/planning/`, und das ist richtig — sie haben nur keinen Circle-Datensatz, der einen
Abschluss tragen könnte.

## Was geprüft wurde

| Gegenstand | Menge | Ergebnis |
|---|---|---|
| Planschritte des Plans „Artefakt und Release" gegen den Baum | 11 | alle elf belegt |
| Abnahmekriterien: Spec gezählt gegen Zuordnungstabelle des Plans | 40 : 40 | deckungsgleich, keine Lücke, keine Doppelung |
| Geschlossene Defekte dieser Sitzung, jede Erledigung einzeln am Baum | 8 | alle acht belegt; eine `Resolved:`-Begründung überholt |
| Namentlich behauptete Proben, einzeln im Baum gesucht | 15 | alle 15 vorhanden |
| `cargo test --workspace` | — | grün, `xtask` liefert 155 Proben |
| `cargo clippy --workspace --all-targets` | — | Rückgabewert 0 |
| `cargo fmt --all --check` | — | Rückgabewert 0 |
| Offene Entscheidungen, die diese Sitzung angelegt hat | 3 | alle drei bleiben offen, jede mit eigenem Grund |
| Prozessaufrufe in `xtask/src`, einzeln gelesen | 20 | drei Werkzeuge über den Suchpfad, nicht eins |
| Aktive Grundlage (`_o_` + `_a_`, alle Speicher) | 42 | 40 tragen, 2 tragen eine vom Baum widerlegte Aussage |
| Durchsichtsdateien dieser Sitzung | 4 | drei verlangte Nachträge waren nicht eingetragen |

## Was berichtigt wurde

**Keine Umbenennung.** Kein Dateimarker ist in diesem Durchgang bewegt worden — jede
Bewegung, die in Frage kam, hätte eine offene Entscheidung durch vollendete Tatsache
entschieden. Berichtigt sind eine Kopfzeile und sieben Fortschreibungen.

### Die Kopfzeile des Specs stand auf „Entwurf"

`shared/planning/260821-1115_o_spec-artefakt-und-release.md`. Der Spec ist am 260821 vom Nutzer
abgenommen (`77b84bb`) und vollständig gebaut; „Entwurf" war seit jenem Commit falsch. Die Zeile
sagt jetzt, was gilt, und nennt den Grund für den stehenden Marker. Dazu ein
`## Reconciliation Log` mit der gemessenen Tafel — es ist die erste Beurteilung dieses Specs.

### Eine geschlossene Begründung ist von einem späteren Commit umgestoßen

`shared/issues/260820-2235_c_eine-bookmarks-toml-die-serde-toleriert-…` hat eine
`Revised by:`-Zeile bekommen, **ohne Umbenennung**. Seine `Resolved:`-Notiz sagt, beide Hälften
der neuen Frage mündeten „in den schon bestehenden Zweig `Grund::Beschaedigt` und damit in
`Zugang::beiseite_legen`". Für die zweite Hälfte gilt das seit `d771ec6` nicht mehr: der
Leerbefund-Zweig gibt `Beiseite::Nicht` zurück und ruft `beiseite_legen` nicht
(`crates/krk-core/src/ablage/mod.rs:607-624`). Der Defekt bleibt geschlossen; allein die
Begründung ist überholt, und die Konvention sieht dafür genau diese Zeile vor.

### Fünf Datensätze haben eine Fortschreibung bekommen

| Datensatz | Was dazugekommen ist |
|---|---|
| `decisions/260821-1221_*_ruft-xtask-…-suchpfad-…` | die Voraussetzung „jedes fremde Werkzeug mit vollem Pfad" trifft nicht zu; dazu der nachgeholte Nachtrag der Durchsicht, mit berichtigter Zahl |
| `decisions/260821-1115_*_bekommt-…-eine-eigene-huelle-…` | der nachgeholte Nachtrag: die `README.md` trägt die Begründung für Option 1 inzwischen ausgeschrieben |
| `decisions/260821-0142_*_gilt-die-strenge-bestandsregel-…` | die erste Randbedingung stimmt seit `d771ec6` nicht mehr, und was das an Option 2 verschiebt |
| `issues/260821-1401_*_zwei-mit-d771ec6-neu-geschriebene-prosastellen-…` | die Erzeugertabelle zählt vier; es sind sieben |
| `issues/260821-0142_*_eine-nicht-lesbare-ablagedatei-…` | `Beiseite::Nicht` hat mit `d771ec6` einen weiteren Erzeuger bekommen, und der Schlusssatz gilt für ihn mit |
| `issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-…` | der nachgeholte Nachtrag: zwei Abfangstellen seit dem 260821, beide hinter dem Schaden |

## Die vier vom Nutzer benannten Stellen, einzeln

### 1. Der Spec bleibt `_o_`, und die Lage ist dieselbe

Der Abgleich vom 260820-2056 hat zwei Specs aus demselben Grund stehen lassen. **Die Prüfung,
ob die Lage hier dieselbe ist, ist gefahren und nicht angenommen:** die zwei Lesarten der offenen
Frage `shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-…` fallen an dieser
Datei auseinander. Nach der Lesart der belegten Bauarbeit stünde der Spec auf `_c_` — elf von elf
Planschritten belegt, vier Durchsichten gefahren, jeder Befund behoben oder als eigener Datensatz
abgelegt. Nach der Lesart der Abnahmekriterien nicht: 15 der 40 sind ohne den Nutzer nicht
abzunehmen, und was ihnen fehlt, liegt außerhalb des Baums (`gh` fehlt, es gibt keine Anmeldung,
`git push origin --tags` ist nie gefahren).

Ein `_c_` hier entschiede die Frage durch vollendete Tatsache. Der Marker bleibt.

### 2. Die zwei Entscheidungen zum Veröffentlichungsbefehl bleiben `_o_`

**Die Hülle** (`260821-1115`): der Baum hat keine bekommen — die Wurzel führt `release.sh` und
`certify-only.sh` und kein `publish-only.sh`, das `Makefile` kein Ziel `veroeffentlichen`. Das
ist aber genau die Fassung, die Spec und Plan ausdrücklich als **vorläufig** bezeichnen, und der
Plan führt die Frage unter `## Open Questions` als ungehakt. Eine Empfehlung, die sich selbst
vorläufig nennt, plus ein Baum, der ihr folgt, ergeben kein `Answered:`.

**Der Suchpfad** (`260821-1221`): `gh` wird über den Suchpfad gerufen, an vier Stellen über die
Konstante `GH`. Gefragt ist aber, ob daraus die **Regel für jedes künftige fremde Werkzeug**
wird; die letzte Randbedingung sagt es wörtlich, und der Modulkopf von
`xtask/src/veroeffentlichung.rs:43-45` legt die Frage ausdrücklich dem Nutzer vor — mit
ausgeschriebenem `_o_`, sodass eine Umbenennung dort einen toten Zeiger im Quelltext erzeugte.

**Beim Nachmessen ist die Voraussetzung dieser Frage gefallen.** Sie geht davon aus, `xtask`
rufe jedes fremde Werkzeug mit vollem Pfad und `gh` sei die erste Ausnahme. Es sind drei:
`iconutil` (`xtask/src/bundle.rs:427`, seit `8695b77` vom 260811) und `rustup`
(`xtask/src/release.rs:604`, seit `d577295` vom 260806) stehen seit zehn und fünfzehn Tagen ohne
vollen Pfad und ohne Begründung da. Als eigener Defekt abgelegt (siehe unten).

### 3. Der Bestandsregel-Datensatz trifft weiter zu, eine Randbedingung nicht mehr

`260821-0142`. Die Frage steht unverändert: `Datei::leerbefund` gibt `Leerbefund::Beschaedigt`
weiter allein für `Datei::Lesezeichen` zurück (`pfade.rs:234-241`), `Sitzung` trägt weiter kein
`deny_unknown_fields`, und die Messung, die die dritte Randbedingung verlangt, ist nicht
gefahren. Also `_o_`.

Die **erste** Randbedingung ist durch `d771ec6` überholt, und sie verschiebt die Rechnung von
Option 2: eine streng gestellte `session.toml` ohne obersten Schlüssel bekäme eine Meldung und
den Auslieferungszustand, aber keine Sicherung. Der Grund, mit dem die Empfehlung Option 2 trägt,
ist für diese Hälfte schwächer als am 260821-0142. Für die andere Hälfte, `deny_unknown_fields`,
ist er unberührt.

### 4. Die acht geschlossenen Defekte, einzeln nachgelesen

Alle acht sind belegt. Die fünfzehn namentlich behaupteten Proben stehen alle im Baum, und der
volle Prüflauf ist grün. Eine Stelle hält nicht: die `Resolved:`-Begründung von `260820-2235`
(siehe oben, `Revised by:`).

Zwei kleinere Ungenauigkeiten sind gemessen und **nicht** bewegt worden, weil sie schon
aktenkundig sind:

- Die Verschiebungstabelle in `260821-1023_c_…` nennt fünf Zeilennummern um genau eine zu
  niedrig. Der offene Datensatz `260821-1023_o_sieben-prosastellen-…` trägt seit dem 260821-1401
  die geprüfte Tabelle; ich habe alle neun Zeilen am Baum nachgeschlagen — die geprüfte stimmt,
  die im geschlossenen Datensatz nicht.
- Dieselbe Notiz nennt „vier mitgezogene Prosastellen"; es sind fünf. Steht bereits im offenen
  Datensatz `260821-1401_o_…` unter „Verwandt".

## Was nur gekennzeichnet und nicht bewegt wurde

**Kein Marker dieser Sitzung ist bewegt worden.** Die sechs offenen Defekte und drei offenen
Entscheidungen dieser Sitzung stehen weiter offen; jeder Grund ist oben oder im Datensatz selbst
belegt. Die sechs `_a_`-Entscheidungen des Projekts sind unberührt — keiner der sieben
Codecommits dieser Sitzung löst eine von ihnen ein.

**Die Ortsregel ist eingehalten.** Nichts unter `history/`, `reviews/`, `analyses/`,
`messungen/` und `spikes/` ist angefasst worden. In `issues/` und `decisions/` ist ausschließlich
angehängt worden; keine Beschreibung ist geändert, kein Zitat nachgezogen.

## Zwei neue Datensätze

- **`shared/issues/260821-1532_o_zwei-fremde-werkzeuge-werden-seit-langem-ueber-den-suchpfad-gerufen-und-drei-stellen-nennen-gh-als-die-erste-ausnahme.md`**
  — `iconutil` und `rustup` stehen seit dem 260811 und dem 260806 ohne vollen Pfad da. Drei
  Prosastellen behaupten, `gh` sei die erste Ausnahme; eine davon, der Modulkopf von
  `veroeffentlichung.rs`, ist bei genauem Lesen nicht falsch, die anderen zwei sind es. Der
  Befund verschiebt die offene Entscheidung `260821-1221`.
- **`shared/issues/260821-1532_o_drei-von-durchsichten-verlangte-nachtraege-an-offenen-datensaetzen-sind-nie-eingetragen-worden.md`**
  — die Durchsicht `260821-1346` legt drei Befunde als Nachtrag an offenen Datensätzen ab, statt
  sie zu filen. Keiner der drei war eingetragen. Alle drei sind in diesem Durchgang nachgeholt;
  der Datensatz steht für den Mechanismus. Der Sache nach eine Frage an fusion.

## Was die Coherence-Auskunft trägt

Die Auskunft ist an `shared/history/260820-2200-orchestrator-session.md` angehängt. Die drei
Belege in Kurzform:

**Die Directive der Sitzung ist zweimal verschoben worden, und die Verschiebung steht nirgends
aufgeschrieben.** `agentstate.yaml` führt unter `session.directive` weiter „Nach dem
Überkopieren der App sind alle Lesezeichen weg. Ursache finden und beheben", das
Sitzungsprotokoll unter `**Directive:**` die Fassung „Es braucht einen persistenten
Speicherort", und `control.directive_revisions_this_session` steht auf `0`. Gearbeitet worden
ist an zwei anderen: erst „die Wurzel finden", dann „Artefakt und Release". Beide Verschiebungen
sind Nutzerentscheidungen, beide sind richtig — die ursprüngliche Directive beruhte auf einer
falschen Annahme, denn der Ablageordner ist persistent und liegt außerhalb des Bündels
(`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`, Beweisstücke B1 und B2).
**Nicht erreicht ist sie nicht, sondern gegenstandslos geworden.**

**Diese Sitzung hat einen vollständigen Rundenumfang gebaut, ohne dass ein Circle aktiv war.**
Ein Spec mit 40 Abnahmekriterien, ein Plan mit elf Schritten, vier Durchsichten, sieben
Codecommits, und dazu der Satz im Plan: „Diese Runde schließt damit voraussichtlich
beschränkt." Es gibt aber keinen Circle-Datensatz, der ein `_b_` tragen könnte:
`fusion-workbench/circles/` führt einen `_a_`, elf `_b_`, vier `_c_` und einen `_d_`, keinen
`_t_`. Die Ablage in `shared/` folgt der Herkunftsregel und ist richtig; was fehlt, ist der
Träger des Abschlusses. Wer später fragt, welche Runde die achte Station gebaut hat, findet
keine.

**Fünfzehn Abnahmekriterien warten auf den Nutzer**, und drei Vorbedingungen liegen außerhalb
des Baums: `gh` installieren, `gh auth login`, und einmalig `git push origin --tags`, weil auf
der Gegenseite 13 der 14 lokalen Tags fehlen. Das ist in diesem Projekt der Regel- und nicht der
Ausnahmefall.
