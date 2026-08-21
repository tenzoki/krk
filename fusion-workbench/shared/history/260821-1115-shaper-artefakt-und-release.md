# Shaper: Artefakt und Release

**Datum:** 2026-08-21 11:15
**Status:** Complete
**Agent:** shaper (Modus: user-direct, dritte und abschließende Klärungsrunde)
**Baumstand:** `d771ec6`

## Auftrag

Dritte Runde einer Klärung zu „Artefakt und Release". Alle Fragen waren beantwortet; der Auftrag
lautete, den Spec zu schreiben. Die Vorrunden lagen nicht im Gedächtnis dieses Laufs vor, sondern
vollständig im Auftragstext.

## Was der Nutzer gesetzt hat

Umfang: nur Artefakt und Release. Ein neues xtask-Ziel baut ein weitergebbares Artefakt und hängt
es an ein GitHub-Release. Kein App-Code, keine neue Kiste, keine Netzverbindung zur Laufzeit.

Acht Antworten aus zwei Klärungsrunden: Zip als Hülle mit einem zweiten `ditto` nach dem Heften,
eng begrenztes Schieben von Zweig und einem Tag, `gh release create` als Werkzeug, ein eigener
Unterbefehl als achte Station, Dateiname `KRK-<zahl>.zip`, fester Text aus dem Werkzeug, gleich
öffentlich, und die fehlenden Tags einmalig von Hand nachgeschoben.

Sechs Vorgaben des Shapers hatte der Nutzer gesehen und nicht widersprochen; alle sechs sind
übernommen.

## Was gelesen wurde

`xtask/src/main.rs`, `beglaubigung.rs`, `git.rs`, `release.rs` (Modulkopf und `ausfuehren`),
`version.rs` (schreibende Kommandos und ihre Probe), `Makefile`, `release.sh`, `README.md`
(Abschnitt „Auslieferung"), die Untersuchung
`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md` und die zwei zitierten
Defektdatensätze `260813-0026` und `260815-1436`. Dazu die vorhandenen Specs unter
`shared/planning/`, die offenen Datensätze in `shared/decisions/` und `shared/issues/`.

## Eine Zahl aus dem Auftrag ist berichtigt

Der Auftrag sprach von fünfzehn fehlenden Tags. Gemessen am 260821 sind es **dreizehn**: lokal
stehen 14 Tags, auf der Gegenseite steht einer, `v0.1.0`. Gemessen mit `comm -23` über
`git tag -l` und `git ls-remote --tags origin`. Der Spec trägt die gemessene Zahl und verlangt für
die `README.md` das zählende Kommando statt einer festen Zahl.

## Eine Gabelung, die keine der acht Antworten deckt

Ob der neue Unterbefehl eine eigene Hülle wie `certify-only.sh` und ein Makefile-Ziel bekommt.
Der Punkt hat Gewicht, weil `cargo` auf diesem Gerät nicht auf dem Standard-PATH steht und die
Hüllen dem Nutzer genau das abnehmen. Gefiltert als
`shared/decisions/260821-1115_o_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-wie-certify-only-sh.md`
mit drei Optionen. Der Spec fährt auf der konservativen Fassung weiter, also ohne beides.

## Ergebnis

`fusion-workbench/shared/planning/260821-1115_o_spec-artefakt-und-release.md`, sechs Capabilities
mit 40 Abnahmekriterien, zwei Mermaid-Diagrammen, einem Abschnitt zu den drei verworfenen Wegen
und einem Abschnitt „Offen für den Planer".

Der Spec liegt im gemeinsamen Speicher, weil kein Circle aktiv ist. `fusion-paths` hat
`OUT_PLAN=shared/planning` aufgelöst.

## Nicht getan

Kein Plan, kein Code, keine technische Wahl. Die Ticketprüfung, die Modulaufteilung und der
Wortlaut der Meldungen sind ausdrücklich dem Planer überlassen.
