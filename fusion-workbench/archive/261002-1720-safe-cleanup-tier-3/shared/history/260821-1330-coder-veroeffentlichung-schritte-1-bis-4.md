# Umsetzung: die Schritte 1 bis 4 des Veröffentlichungswegs

**Datum:** 2026-08-21
**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `7db749e`
**Plan:** `shared/planning/260821-1221_o_plan-artefakt-und-release.md`, Schritte 1 bis 4

## Was entstanden ist

`xtask/src/veroeffentlichung.rs` (neu, 463 Zeilen) mit dem Gerüst des neuen Wegs und den ersten
drei Prüfungen. Die Gestalt ist von `beglaubigung.rs` übernommen und nicht neu erfunden: jede
Prüfung zerfällt in eine reine Hälfte und einen Prozessaufruf, und die Proben liegen an der
reinen Hälfte.

- **Schritt 1** — Modulkopf mit den vier Aussagen (wozu der Weg da ist, dass er nichts baut,
  dass er nichts einreicht, dass er den Arbeitsbaum nicht prüft), dazu die Begründung für den
  Aufruf von `gh` über den Suchpfad samt Verweis auf den Entscheidungsdatensatz.
  `pub(crate) fn ausfuehren` zerlegt mit `let [zahl] = argumente else`, meldet „genau ein
  Argument", ruft `version::versionszahl_pruefen` und fragt nach `bundle::buendelpfad`; fehlt
  das Bündel, nennt der Abbruch `./release.sh <zahl>`.
- **Schritt 2** — `gh_pruefen` mit zwei Fragen: ein Startversuch von `gh --version`, und der
  Rückgabewert von `gh auth status`. Beide Meldungen sind reine Funktionen
  (`gh_fehlt_meldung`, `nicht_angemeldet_meldung`) und tragen `#[must_use]`.
- **Schritt 3** — `TICKETDATEI` (`Contents/CodeResources`), `TICKETKENNUNG` (`b"s8ch"`) und
  `traegt_angeheftetes_ticket(&[u8]) -> bool` mit `#[must_use]`. Der Doc-Kommentar hält die
  Messung, die Verwechslungsgefahr mit der gleichnamigen Datei unter `_CodeSignature/` und den
  Grund fest, warum `xcrun stapler validate` nicht genommen ist.
- **Schritt 4** — `zipname(zahl) -> String` und `zip_packen` mit demselben
  `ditto -c -k --keepParent`, das die Einreichung führt. Der Doc-Kommentar sagt, warum ein
  zweites Mal gepackt wird und warum die zwei Zipnamen sich nicht ins Gehege kommen.

Dazu eine Zeile `mod veroeffentlichung;` in `xtask/src/main.rs` — und, über den Plan hinaus,
die Verteilungszeile; die Begründung steht unten.

## Der Befund des Planers, am Baum nachgeprüft

Vor dem Bauen nachgemessen am 260821, alles bestätigt:

- `target/KRK.app/Contents/CodeResources` beginnt mit `73 38 63 68`, also `s8ch`, gefolgt von
  `01 00 00 00`.
- Die Datei trägt 19:44 des 260820; `Info.plist`, `PkgInfo`, `MacOS/`, `Resources/` und
  `_CodeSignature/` tragen 11:35.
- `Contents/_CodeSignature/CodeResources` beginnt mit `<?xml version="1.0"`.
- `grep -rn CodeResources xtask/ Makefile README.md` findet nichts.
- `command -v gh` findet nichts.

## Zwei Abweichungen vom Plan, beide benannt

**Die Verteilungszeile in `main.rs` ist vorgezogen.** Der Plan gibt sie Schritt 7. Ohne einen
Rufer ist das ganze Modul aber toter Code, und `make check` fährt `clippy -- -D warnings`; der
Lauf wäre rot geblieben. Vorgezogen ist genau eine Zeile,
`"veroeffentlichen" => veroeffentlichung::ausfuehren(&argumente[1..])`. Schritt 7 behält seinen
Gegenstand: die achte Station in `release::ausfuehren`, der Modulkopf von `release.rs` und die
zwei Quelltextproben.

**Die dritte Nadel der Probe `dieser_weg_reicht_nichts_ein` ist der Anheftungsaufruf und nicht
das Wort `stapler`.** Der Plan verlangt in Schritt 3, der Doc-Kommentar solle sagen, warum
`xcrun stapler validate` nicht genommen ist, und in Schritt 4, eine Quelltextprobe solle halten,
dass das Modul `stapler` nicht nennt. Beides zugleich geht nicht: die Probe liest die Datei, in
der die Begründung steht. Die Nadel ist deshalb `stapler", "staple"`, also der Aufruf, den
`beglaubigung.rs:380` führt. Die Zusage aus C2.5 bleibt damit dieselbe — dieser Weg reicht
nichts ein und heftet nichts an —, und die Begründung darf dastehen. Der Prüfkommentar schreibt
das aus.

## Proben

Acht neue, alle in `#[cfg(test)] mod tests` desselben Moduls:

| Probe | Was sie hält |
|---|---|
| `veroeffentlichen_nimmt_genau_ein_argument` | kein Argument, zwei, `v0.5.6` — je ein Aufruffehler |
| `ohne_gh_nennt_die_meldung_das_werkzeug_und_die_abhilfe` | C5.1, der Wortlaut ohne `gh` abnehmbar |
| `ohne_anmeldung_nennt_die_meldung_gh_auth_login` | C5.2 |
| `die_voraussetzungspruefung_steht_vor_dem_packen` | C5.3, Quelltextprobe über die Reihenfolge im Rumpf |
| `das_ticket_wird_an_der_kennung_am_anfang_erkannt` | C2.3, vier Fälle |
| `ein_zu_kurzer_puffer_traegt_die_kennung_nicht` | nichts rät zur bequemen Seite |
| `ohne_ticket_nennt_die_meldung_den_handgriff` | Bedingung, Pfad, `./certify-only.sh <zahl>` |
| `der_zipname_traegt_die_zahl` | C2.1, die reine Hälfte |
| `dieser_weg_reicht_nichts_ein` | C2.5, Quelltextprobe |

Die drei bestehenden Aufsichtsproben laufen unverändert grün:
`xtask_ruft_git_an_genau_einer_stelle`, `keine_der_drei_fragen_schreibt`,
`allein_release_fragt_nach_tag_und_arbeitsbaum`. Das neue Modul ruft `git` an keiner Stelle;
die drei Vorkommen der Zeichenfolge sind Prosa.

## Abnahme

`make check` — Rückgabewert 0. Bau, 121 Proben in `xtask`, `fmt --check` und
`clippy -- -D warnings`, alle vier grün.

Dazu die zwei Läufe am Gerät, die Schritt 11 vorsieht und die dieses Gerät ohne `gh` hergibt:

- `cargo xtask veroeffentlichen 0.5.6` bricht mit Rückgabewert 1 an der ersten Stufe ab, nennt
  das Werkzeug und `brew install gh`. Danach liegt kein `target/KRK-*.zip`, und der Arbeitsbaum
  ist unverändert.
- `cargo xtask veroeffentlichen` ohne Argument endet mit Rückgabewert 2.

## Was offen bleibt

Der Zweig „`gh` ist da, aber nicht angemeldet" ist an seiner Meldung abgenommen und **am
lebenden Gerät nicht gemessen**, weil `gh` fehlt. Ebenso ungemessen sind alle Kriterien, die
der Plan in seiner Tabelle „Abnahme durch den Nutzer" führt.

Die Schritte 5 bis 11 sind nicht Gegenstand dieses Laufs. `xtask/src/git.rs` und die Probe
`die_schreibenden_kommandos_tragen_keine_gewalt` in `xtask/src/version.rs` sind unangetastet.

**Der Modulkopf beschreibt den fertigen Weg und nicht den Zwischenstand.** Er sagt, dass dieser
Weg schiebt und eine Releaseseite anlegt; beides kommt erst mit den Schritten 5 und 6. Wer
zwischen den Zügen liest, findet dort eine Zusage, die der Rumpf noch nicht einlöst.
