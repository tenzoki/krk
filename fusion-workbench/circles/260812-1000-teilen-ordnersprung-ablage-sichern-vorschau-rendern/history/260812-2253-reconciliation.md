# Abgleich der Runde 6 gegen den Baum — 260812-2253

**Agent:** reconciler
**Domäne:** code
**Circle:** `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`
**Bereich:** `4d4402d..dc5e137`, 25 Commits
**Status:** Complete

## Umfang

| Gegenstand | gelesen | geändert |
|---|---|---|
| Pläne | 1 | 1 (Abgleichsprotokoll, zwei Belegstellen, Marke an Schritt 11) |
| Defekte | 48 (41 offen, 15 geschlossen, 1 zurückgestellt über beide Speicher) | 1 (Beleg angefügt) |
| Entscheidungen | 32 über beide Speicher | 5 (4 umbenannt, 1 Beleg angefügt) |
| Durchsichten | 6 | 0 |
| Neu abgelegt | — | 4 Defekte |

Die vier Abnahmekommandos sind neu gefahren: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` — je Exit 0,
478 Proben im Binärziel `krk`. Die Zahl im Sitzungsprotokoll stimmt.

## Was hält

**Alle elf Planschritte tragen zu Recht `[DONE]`.** Stichprobenartig gegen den Baum gelesen:
`crates/krk-core/src/ablage/atomar.rs:25,61` und `ablage/mod.rs:154,399` (Schritt 1),
`resources/default-keymap.toml:245,610` samt Zählzeile `:34` (Schritte 2 und 4),
`crates/krk-ui/src/appkit/anwendung.rs:2409` und `appkit/teilen.rs:217,253` (Schritte 3 und 5),
vier Aufrufer eines Menübauers (Schritt 6), `markdown.rs`, `hervorhebung.rs:422-431` und
`textmerkmale.rs` (Schritte 7 bis 9), `appkit/statuszeile.rs` (Schritt 10).

**Alle fünfzehn `Resolved:`-Zeilen der geschlossenen Defekte halten.** Keine behauptet eine
Änderung, die der Baum nicht trägt.

**Die Zählproben-Eigenschaften aus C6 halten**, von Hand nachgezählt: `Kommando` 75 Varianten
(C6.1 sagt 73 auf 75), `Wirkungsbereich` 7, `Bereich` 5, `Fokus` 5 — die drei letzten
unverändert; 81 Funktionen mit 87 Kombinationen in der Belegungsdatei (C6.2); zwei
`#![allow(unsafe_code)]` an den vorgesehenen zwei Stellen (C6.5); drei Prüfordner-Fassungen
(C6.6); kein `WKWebView` im Baum (C4.5); eine Hülle um `NSPasteboard` (C1.8).

**Die Rücknahme von Schritt 11 ist im Baum sauber.** `crates/krk-ui/src/appkit/statuszeile.rs`
hält wieder allein ein `NSTextField`; `NSScrollView` kommt in der Datei nur noch in Prosa vor,
die die Rücknahme erklärt und den Entscheid in Sternform zitiert. `breite_nachziehen` und
`an_den_anfang` sind weg.

## Was nicht hält

**1. Vier Entscheidungen standen auf beantwortet, obwohl der Baum sie einlöste.** Sie sind auf
umgesetzt gezogen, mit Beleg in der `Implemented:`-Zeile:
`260812-1000_i_was-tut-die-nummernspalte-bei-gerendertem-markdown` (`vorschaumodell.rs:478-486`),
`…_i_was-tut-ein-link-im-gerenderten-markdown-…` (`vorschau.rs:1120-1121`, `markdown.rs:204`),
`…_i_wie-erfaehrt-der-nutzer-dass-eine-ablagedatei-zur-seite-gelegt-wurde` (`ablage/mod.rs:191,248,273`,
`anwendung.rs:1007-1021,931-940`), `…_i_zeigt-die-vorschau-lokale-html-dateien-gerendert`
(`hervorhebung.rs:422-431`).

**Die fünfte bleibt auf beantwortet, und das ist Absicht.**
`260812-1000_a_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite`: der Baum trägt
die Antwort (`fenstermodell.rs:213`, 160 Punkte), aber kein Commit löst sie ein — die Antwort
besteht darin, nichts zu ändern. Auf umgesetzt gezogen fiele sie aus der Suche nach aktiver
Grundlage heraus und mit ihr der Auslöser, der die Frage wieder aufmacht: ein Lauf am laufenden
Bündel. Das ist die Lage, die `CLAUDE.md` für die zurückgestellte L9-Frage beschreibt, und sie
ist hier vermieden statt wiederholt. Ein Beleg dazu steht im Datensatz.

**2. Der Plan führte an zwei Stellen unkommentiert in die Irre.** Schritt 11 trägt `[DONE]` und
ist am 260812 zurückgenommen worden; das ist richtig, weil ein Plan die Aufzeichnung eines
Standes ist. Nicht richtig war, dass **C5.10** weiter „Die Zeile lässt sich nach rechts blättern"
zusagte und die Zeile `C5.10, C5.11` in `## Abnahme am laufenden Bündel` den Nutzer losschickte,
ein Blättern zu prüfen, das es nicht gibt. Von beiden ist die zweite die teurere: sie ist eine
Handlungsanweisung an genau dem Lauf, an dem die Runde abgenommen wird. Beide sind beschriftet,
Schritt 11 trägt eine Marke, und der Plan hat ein Abgleichsprotokoll bekommen.

**3. Drei mit (Probe) ausgezeichnete Kriterien sind wahr und nicht abgenommen.** C4.5, C1.8 und
C6.6 — der Datensatz `issues/260812-1805_*_drei-der-fuenf-zaehlproben-der-pruefstrategie-sind-nicht-gebaut.md`
ist bestätigt. C4.5 wiegt darunter am schwersten, weil sein Wortlaut die Prüfform selbst
vorschreibt: „Die Prüfung zählt den Klassennamen im Baum."

**4. L7 ist berührt, entgegen der ausdrücklichen Zusage des Plans.**
`issues/260812-2133_*_merkzeichen-einloesen-…` misst, dass die Zusage bei tief verschachtelten
Listen ab rund 12 kB verfehlt wird statt ab rund 19 kB. Der Plan sagt unter `## Was dieser Plan
ausdrücklich nicht tut` zu, keine der zehn Zeitzusagen anzufassen. Die Zahl ist nicht geändert,
der Abstand zu ihr schon. Auf dem Referenzgerät ist nichts davon gemessen.

## Zitat-Abgleich

Maschinell über **alle** Verweise der Form `YYMMDD-HHMM_X_slug` im ganzen Baum, **mit und ohne**
Endung `.md`. Die Erweiterung um die Kurzform ist die Konsequenz aus
`shared/issues/260810-1851_*_acht-verweise-…`, wo fünf Erhebungen dieselben acht Stellen nicht
gesehen hatten.

Ausgenommen sind die Speicher, für die `CLAUDE.md` die Ortsregel zieht — `history/`, `reviews/`,
`analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/`. Was bleibt, sind die lebenden
Dokumente: Circle-Datensätze, `planning/`, `CLAUDE.md`, `README.md`, Quelltext, `portfolio.md`.

**Zwei gestorbene Zeiger gehören der Runde 6:** `_t_circle.md:7` nennt den Plan mit `_p_`
(er steht auf `_c_`), und der Plan nennt in Zeile 366 den Rechtsklick-Entscheid mit `_o_`
(er steht auf `_i_`). Abgelegt als `issues/260812-2253_*_zwei-verweise-in-lebenden-dokumenten-…`.

**Sieben weitere sind vorgefunden**, im Circle-Datensatz der Runde 5; genau eine davon hat die
Runde 6 weitergeschoben, den Statuszeilen-Datensatz von umgesetzt auf überholt. Abgelegt als
`shared/issues/260812-2253_*_sieben-verweise-im-circle-datensatz-der-runde-5-…`.

**Zwei Zahlen in `CLAUDE.md` stimmen nicht mehr:** `Kommando` steht dort mit 68 Varianten gegen
75 im Baum (neu abgelegt), und die Deckung der Untergrenzen-Angabe steht mit „31 von 33" gegen
34 von 36 — der bereits offene Datensatz `shared/issues/260812-1438_*_…` nennt seinerseits
„33 von 35" und ist damit selbst veraltet; ein Beleg dazu ist ihm angefügt. Beide gehören in
einen Lauf von `/fusion:revise-claude-md` und nicht in diesen Abgleich.

## Die Directive

Zwei Befunde in derselben Zeile `_t_circle.md:14`. Der erste ist abgelegt und bekannt: die
Directive sagt weiterhin eine blätterbare Statuszeile zu
(`issues/260812-1920_*_die-directive-des-aktiven-circles-…`). Der zweite ist neu: der Zählsatz
kündigt vier Dinge an und die Aufzählung darunter führt fünf
(`issues/260812-2253_*_die-directive-kuendigt-vier-dinge-an-…`). Beide stammen aus demselben
Vorgang, dem Nachtrag der fünften Fähigkeit am 260812-1105, und beide sind in einem Zug zu
beheben. Der Plan hat die Zahl richtig: „KRK bekommt fünf Dinge".

Der Circle-Datensatz steht nicht in meinem Schreibbereich; hier wird nur befundet.

## Beobachtungen ohne eigenen Datensatz

**`agentstate.yaml` zählt `directive_revisions_this_session: 0`**, obwohl die Directive in dieser
Sitzung einmal geändert worden ist: das Sitzungsprotokoll hält unter `## Klärungsrunde 260812-1105`
ausdrücklich fest, dass sie eine fünfte Fähigkeit bekommen hat. Kein eigener Datensatz, weil die
Datei beim sauberen Abschluss gelöscht wird.

**Acht `Implemented:`-Zeilen enden auf „noch nicht committet, der Orchestrator committet nach der
Aufgabe."** Die Commits sind seither gelandet. Kein Datensatz: `decisions/` fällt unter die
Ortsregel, die Sätze waren beim Schreiben wahr.

**Zeilennummern in den Belegen sind gewandert**, etwa `anwendung.rs:2403` auf `:2409`,
`teilen.rs:192` auf `:217`, `belegung.rs:865` auf `:457`. Die benannten Symbole stehen alle; das
ist die übliche Drift eines mitten in der Runde geschriebenen Belegs und kein Defekt.

## Neu abgelegt

| Datensatz | Speicher |
|---|---|
| `260812-2253_o_die-directive-kuendigt-vier-dinge-an-und-zaehlt-danach-fuenf.md` | Circle |
| `260812-2253_o_zwei-verweise-in-lebenden-dokumenten-der-runde-6-tragen-einen-gestorbenen-marker.md` | Circle |
| `260812-2253_o_sieben-verweise-im-circle-datensatz-der-runde-5-tragen-einen-gestorbenen-marker.md` | gemeinsam |
| `260812-2253_o_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md` | gemeinsam |

## Urteil

Drei Kanten, drei Beanstandungen, eine Ursache: `_t_circle.md:14`. Das Urteil steht als
`## Coherence` im Sitzungsprotokoll `260812-1055-orchestrator-session.md`, Empfehlung
`revise Directive`.
