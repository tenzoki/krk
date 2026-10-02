# Sitzung: Machbarkeitsanalyse `gix` (gitoxide) als Git-Anbindung, Stufe A

**Datum:** 2026-08-30 10:06
**Agent:** analyst
**Status:** Complete
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Baumstand:** HEAD `d1fbaac`, Branch `main`, `## main...origin/main` ohne Vorsprung oder Rückstand

## Auftrag

Acht Fragen zur Machbarkeit der Stufe A einer Git-Anbindung mit `gix`: Funktionsumfang, C-Freiheit, Reife, Statuskosten gegen L1/L3/L10, Kistenzuordnung, Verträglichkeit mit dem Verzeichnisleser, die Fallunterscheidungen eines sechsten `Bereich`, und der Fall ohne Repository. Kein Circle aktiv, also alles im gemeinsamen Speicher.

## Was gemacht wurde

- `gix` 0.87.1 aus der Registrierung entpackt gelesen: Merkmalsbaum, `gix-zlib`, `gix-status`, die vier APIs der Stufe A, das Vertrauensmodell.
- Ein Wegwerf-Workspace außerhalb des Projektbaums (`…/scratchpad/gixprobe`) mit vier Prüfprogrammen gebaut und gegen sechs Repositorys gefahren: KRK selbst, 10 500 Dateien, 100 500 Dateien, 10 000 unverfolgte Dateien, abgelöster HEAD, ungeborener HEAD. `Cargo.toml`, `Cargo.lock` und der Projektbaum sind nicht angefasst.
- Der Abhängigkeitsbaum für beide Mac-Ziele erhoben, mit und ohne Baustufe, und gegen KRKs heutigen Baum verrechnet.
- Der Deskriptorbedarf des Statuslaufs unter absteigendem `ulimit -n` gemessen.
- Der KRK-Baum auf die Stellen durchgesehen, die ein sechster `Bereich`, ein sechster `Fokus` und ein neunter `Wirkungsbereich` anfassen, und je Stelle bestimmt, ob der Übersetzer, eine Probe oder nichts sie hält.
- Ein eigenständig übersetztes Gegenbeispiel geschrieben, das die Prosabehauptung „die Feldbreite hält den Bau an" widerlegt.

## Befunde in einem Satz

`gix` 0.87.1 übersetzt auf beiden Mac-Zielen ohne `cc` und ohne ankommendes `-sys`-Paket, trägt alle vier Auskünfte der Stufe A und ist in jedem gemessenen Fall so schnell wie `git` oder schneller; der Preis sind 98 zusätzliche Pakete und eine kleine Fassung im Monat unter 0.x, und das eigentliche Risiko liegt nicht bei der Kiste, sondern bei `Bereich::ALLE`, das ein sechster Bereich stumm verfehlen kann.

## Geschriebene Datensätze

- `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md`
- `shared/issues/260830-1006_o_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md`
- `shared/decisions/260830-1006_o_wohnt-die-git-anbindung-in-krk-core-oder-in-einer-fuenften-kiste-krk-git.md`
- `shared/decisions/260830-1006_o_bekommt-der-git-bereich-einen-sechsten-fokuswert-oder-ist-er-nicht-fokussierbar.md`
- `shared/decisions/260830-1006_o_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md`
- `shared/decisions/260830-1006_o_darf-stufe-a-den-aufgefrischten-index-zurueckschreiben-oder-zahlt-sie-die-wiederholung.md`
- `shared/decisions/260830-1006_o_was-zeigen-git-bereich-ankreuzfeld-und-dateiliste-in-einem-ordner-ohne-repository.md`

## Nicht gefilt, weil schon vorhanden

`shared/decisions/260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` ist offen und nennt die elf `ALLE`-Listen als eine Frage; der Befund zu `Bereich::ALLE` und `Fokus::ALLE` fällt darunter und ist deshalb zitiert und nicht neu gestellt.

## Am Projektbaum geändert

Nichts. Die Analyse ist lesend gefahren; geschrieben wurde allein unter `fusion-workbench/shared/`.
