# Umsetzung: die Schritte 8 bis 11 des Veröffentlichungswegs

**Datum:** 2026-08-21
**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `72f7a5d`
**Plan:** `shared/planning/260821-1221_*_plan-artefakt-und-release.md`, Schritte 8 bis 11

## Was entstanden ist

Die vier letzten Schritte der Runde: der Hilfetext, die Umstellung von sieben auf acht
Stationen samt ihrer Zählprobe, die `README.md`, und die Abnahme am Gerät. Der Plan steht
damit auf elf von elf und ist auf `_c_` umbenannt.

### Schritt 8 — der Hilfetext, und der Defekt am Hilfetext zu `bundle`

`HILFE` in `xtask/src/main.rs` bekommt zwei Änderungen.

Der Abschnitt zu `cargo xtask veroeffentlichen <zahl>` ist neu. Er sagt in einem Satz, was der
Befehl tut, sagt danach ausdrücklich, dass er **nichts baut und nichts beglaubigt**, nennt `gh`
als dritte äußere Voraussetzung samt der Stelle, an der sie geprüft wird, und schließt mit dem
einen Unterschied zur achten Station von `release`: dieser Weg fragt selbst nach dem Tag.

Der Abschnitt zu `bundle` bekommt den Satz, den der offene Defekt
`shared/issues/260815-1436_*_der-hilfetext-zu-bundle-schweigt-zur-weitergabe-obwohl-die-ausgabe-des-befehls-sie-jetzt-nennt.md`
verlangt hat: lokal signiert heißt ohne gehärtete Laufzeitumgebung, ohne Beglaubigung, ohne
angeheftetes Ticket und allein für die Architektur der Baumaschine; Gatekeeper weist ein solches
Bündel auf einem zweiten Mac ab, und der Weg zur Weitergabe ist `./release.sh <zahl>`. Der
Datensatz trägt eine `Resolved:`-Zeile und den Marker `_c_`.

**Die drei neuen Proben lesen den Abschnitt und nicht den ganzen Hilfetext.** Dafür steht die
Hilfsfunktion `hilfeabschnitt` im Prüfmodul: sie schneidet an der Kopfzeile mit genau zwei
Leerzeichen Einzug. Der Grund ist unmittelbar geworden — seit dieser Runde sagen **zwei**
Abschnitte „Es baut nichts", und eine Probe über den ganzen Text bliebe grün, auch wenn der Satz
beim falschen von beiden stünde. Die vorhandene Probe
`die_hilfe_nennt_die_grenze_des_nur_beglaubigungswegs` ist deshalb mitgezogen: ihre dritte
Behauptung liest jetzt den Abschnitt zu `beglaubigen`.

### Schritt 9 — aus sieben Stationen werden acht

Die sieben Stellen im Quellbaum sind nachgezogen: `README.md` (drei), `xtask/src/version.rs`
(zwei), `xtask/src/main.rs` (eine), `xtask/src/release.rs` (eine). An der Stelle in `main.rs`
war es nicht mit dem Zahlwort getan — der Satz zählt die Stationen einzeln auf, also ist die
achte in der Aufzählung mit hinzugekommen.

Gehalten wird die Zusage von der neuen Zählprobe
`release::tests::der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr`. Sie liest `README.md`,
das `Makefile` und jede `.rs`-Datei unter `xtask/`, mit derselben `rust_dateien`-Sammlung, die
die drei älteren Aufsichtsproben jener Datei benutzen.

**Zwei Vorsichtsmaßnahmen stecken in ihr, und beide sind der Rede wert.** Die Nadel steht als
`concat!`, weil die Probe in einer der Dateien liegt, die sie liest; und **keine ihrer Meldungen
und kein Wort ihres Prüfkommentars schreibt die Zeichenfolge aus**, denn ausgeschrieben zählte
die Probe sich selbst mit und wäre nie grün zu bekommen.

**Der Umfang ist auf den Quellbaum begrenzt, und das ist eine Festlegung des Plans.** Unter
`fusion-workbench/` bleibt die alte Wendung stehen: dort liegen Aufzeichnungen eines vergangenen
Standes, die nach der Ortsregel aus `CLAUDE.md` ihren damaligen Wortlaut behalten. Das
Abnahmekriterium C6.3 enthält überdies selbst die Zeichenfolge, die es verbietet; der Befund ist
gefilt (`shared/issues/260821-1221_o_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md`)
und bleibt offen — er betrifft den Spec und nicht den Baum.

### Schritt 10 — die `README.md` zieht nach

Vier Stellen, wie geplant, und eine fünfte, die die Directive verlangt.

**Die Voraussetzungstabelle** führt `gh` als vierte Zeile, mit dem Vermerk „nur für die
Auslieferung, nicht für den Bau" und `brew install gh` als Herkunft. Der Absatz darunter ist
neu gefasst: die Auslieferung an Dritte verlangt dreierlei, das der Bau nicht verlangt — eine
Developer-ID-Identität, das vollständige Xcode samt Apple-Entwicklerkonto, und `gh`. Genannt ist
dabei, was `gh` von den anderen drei fremden Werkzeugen unterscheidet: es liefert das System
nicht mit.

**„Das Paket bauen"** bekommt Station 8 in derselben Form wie die sieben davor. Der Absatz
danach ist umgeschrieben: er sagte „Die siebte Station hat zwei äußere Voraussetzungen, **und
nur sie**", und das war mit der achten Station falsch geworden. Er sagt jetzt, welche sechs
Stationen ohne äußere Voraussetzung laufen und welche zwei nicht, und was ein Abbruch an jeder
der beiden liegen lässt.

**„Nur veröffentlichen"** ist neu, nach dem Vorbild von „Nur beglaubigen": der vollständige
Aufruf, eine Tabelle der sechs Schritte in ihrer Reihenfolge, die Zusage, die in dieser
Reihenfolge steckt, und die drei Grenzen (baut nichts, beglaubigt nichts, prüft den Arbeitsbaum
nicht). Der Abschnitt sagt ausdrücklich, dass es für diesen Weg **keine Hülle** gibt und der
Aufruf deshalb den vollen Pfad zu `cargo` trägt, mit dem Verweis auf die offene Frage
`shared/decisions/260821-1115_*_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-wie-certify-only-sh.md`.

**Der einmalige Handgriff** `git push origin --tags` steht als eigener Unterabschnitt darin.
**Eine Zahl steht dort nicht**, sondern das Kommando, das sie zählt:

```sh
comm -23 <(git tag -l | sort) \
         <(git ls-remote --tags origin | sed 's|.*refs/tags/||' | sort)
```

Am 260821 gefahren: es gibt dreizehn Namen aus, `v0.2.0` bis `v0.5.5`. Die Zahl steht in dieser
Aufzeichnung und nicht in der `README.md`, denn sie altert mit dem ersten Lauf.

**„Installieren und aktualisieren"** ist der fünfte Eingriff und der Anlass der ganzen Runde.
Der Abschnitt richtet sich an den Nutzer des Bündels und nicht an den, der es baut: herunterladen
und entpacken, KRK beenden, die neue Fassung über die alte kopieren — und die alte **nicht**
vorher löschen. Die Begründung steht dabei, mit den zwei Beweisstücken aus der Untersuchung
`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`, Abschnitt „Betriebsregel
für den Austausch der App". Der Abschnitt nennt am Ende die eine Stelle, an der derselbe Text
für die Releaseseite steht, damit nicht zwei Wahrheiten daraus werden.

### Schritt 11 — die Abnahme am Gerät

Gemessen, nicht behauptet.

| Lauf | Ergebnis |
|---|---|
| `make check` | Rückgabewert 0; Bau, 134 Proben in `xtask`, clippy unter `-D warnings`, fmt |
| `cargo xtask veroeffentlichen 0.5.6` | Rückgabewert 1, Abbruch an der ersten Stufe, Meldung nennt das GitHub-Kommandozeilenwerkzeug beim Namen |
| `cargo xtask veroeffentlichen` | Rückgabewert 2 |

Danach liegt kein `target/KRK-*.zip`, und `git ls-remote origin` führt unverändert
`refs/heads/main` auf `01d2365` und den einen Tag `v0.1.0`, während HEAD lokal auf `72f7a5d`
steht. Geschoben ist also nichts. `gh` ist auf diesem Gerät nicht installiert; das ist die
Voraussetzung dafür, dass diese zwei Läufe überhaupt etwas messen.

## Ein Befund an Schritt 1, hier behoben

**C1.6 war in der Zuordnung des Plans mit „Probe" abgenommen, und eine solche gab es nicht.**
Die Abbruchmeldung für das fehlende Bündel entstand inline im Rumpf von `veroeffentlichen` und
war damit nicht abnehmbar — genau die Defektklasse, die der Plan selbst benennt: „Kriterium
verspricht eine Probe und hat keine".

Behoben im Muster, das das Modul ohnehin durchgehend führt: die Meldung steht jetzt als reine
Funktion `ohne_buendel_meldung(buendel, zahl) -> String` mit `#[must_use]` da, neben
`gh_fehlt_meldung`, `ohne_tag_meldung` und `ohne_ticket_meldung`, und die Probe
`ohne_buendel_nennt_die_meldung_den_ganzen_weg` nimmt sie ab. **Der Wortlaut ist unverändert**,
das Verhalten also auch. Die Probe prüft dabei auch die eine Aussage, die leicht falsch würde:
dass die Abhilfe der ganze Weg ist und **nicht** `./certify-only.sh` — jener Weg setzt selbst
ein fertiges Bündel voraus.

## Eine Prosastelle, die diese Runde falsch gemacht hat

Der Doc-Kommentar von `traegt_angeheftetes_ticket` sagte, die Zeichenfolge `CodeResources` stehe
„weder im Bauwerkzeug noch im `Makefile` noch in der `README.md`". Das war die Messung vom
260821, aus der Zeit **vor** Schritt 3 — und Schritt 3 hat sie selbst widerlegt, indem er
`TICKETDATEI` anlegte; Schritt 10 hat sie ein zweites Mal widerlegt, indem er den Pfad in die
Tabelle der `README.md` schrieb.

Die Aussage ist auf die tragende Hälfte zurückgeschnitten: **kein Aufruf unter `xtask/`
schreibt die Datei**, genannt wird sie dort allein von der Prüfung, die liest, und keiner der
Aufrufe von `codesign`, `ditto` oder `xcrun` legt sie an. Das ist die Hälfte, auf der die
Ticketprüfung steht; die andere war eine Zählaussage, die mit jeder Erwähnung altert.

## Zwei tote Marker in lebendem Text

`shared/planning/260821-1115_*_spec-artefakt-und-release.md` und der Plan selbst zitierten den
Defektdatensatz `260815-1436` in seiner damaligen `_o_`-Form. Mit der Umbenennung auf `_c_` wären
das zwei tote Verweise geworden — die Defektklasse aus
`shared/issues/260812-2253_*_sieben-verweise-im-circle-datensatz-der-runde-5-tragen-einen-gestorbenen-marker.md`.
Beide stehen jetzt in der Sternform. Die Verweise unter `history/`, `reviews/`, `decisions/` und
im Ereignisprotokoll bleiben unangetastet: sie sind Aufzeichnungen eines Standes und behalten
nach der Ortsregel ihren damaligen Marker.

## Was offen bleibt

- **Dreizehn der 40 Abnahmekriterien warten auf den Nutzer.** Sie stehen in der Tabelle
  „Abnahme durch den Nutzer" des Plans und sind ohne `gh`, ohne Anmeldung und ohne einen zweiten
  Mac nicht abzunehmen. Kein Agent kommt an sie heran, und es ist dafür keine Probe erfunden
  worden.
- **Vor dem ersten echten Lauf:** `brew install gh`, `gh auth login`, einmalig
  `git push origin --tags`.
- `shared/issues/260821-1221_o_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md`
  bleibt offen; er betrifft den Spec.
- `shared/issues/260813-0026_o_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`
  bleibt offen; die Ticketprüfung mildert ihn und schließt ihn nicht.
- Die zwei Entscheidungsdatensätze der Runde (`260821-1115` zur Hülle, `260821-1221` zum
  Suchpfad) stehen weiter offen und halten nichts auf.

## Dateien

- `xtask/src/main.rs` — Hilfetext, drei neue Proben, `hilfeabschnitt`
- `xtask/src/release.rs` — Modulkopf, Zählprobe
- `xtask/src/version.rs` — Modulkopf, zwei Stellen
- `xtask/src/veroeffentlichung.rs` — `ohne_buendel_meldung` samt Probe, ein Doc-Kommentar
- `README.md` — fünf Stellen
- `fusion-workbench/shared/issues/260815-1436_c_…` — `Resolved:` und Marker
- `fusion-workbench/shared/planning/260821-1221_c_plan-artefakt-und-release.md` — elf `[DONE]`,
  Status, Marker
- `fusion-workbench/shared/planning/260821-1115_o_spec-artefakt-und-release.md` — ein Verweis
