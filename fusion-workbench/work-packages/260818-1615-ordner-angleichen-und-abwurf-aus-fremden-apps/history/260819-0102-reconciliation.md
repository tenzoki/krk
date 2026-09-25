# Reconciliation — Runde 13, Sitzung 260818-1117

**Datum:** 260819-0102
**Status:** Complete
**Domain:** code
**Bereich:** `8d5baf6..cac9218`, zwölf Commits, zwei Turns
**Baumstand:** `cac9218`
**Verification:** `make check` — Exit 0, 1357 Proben grün, null rot, `clippy` unter `-D warnings` und `cargo fmt --check` sauber
**Verdikt:** `coherent`

Die Runde hat zwei Fähigkeiten gebaut: `opt+cmd+s` stellt das andere Dateifenster auf den
Ordner des aktiven, und eine KRK-Dateiliste nimmt Dateien und Ordner entgegen, die eine fremde
Anwendung hineinwirft. Dieser Abgleich prüft jede behauptete Erledigung gegen den Baum und
nicht gegen die Sitzungsprotokolle.

## Was geprüft wurde

| Speicher | Gelesen | Geändert |
|---|---|---|
| Pläne und Specs (Circle + gemeinsam) | 2 | 2 (je eine Korrektur, Statusfeld, Abgleichsabschnitt, Marker `_o_` → `_c_`) |
| Defektdatensätze im Circle | 14 (11 geschlossen, 3 offen) | 3 (Belege an die offenen angehängt) |
| Defektdatensätze im gemeinsamen Speicher | 92, davon 4 einschlägig | 4 angehängt |
| Entscheidungsdatensätze | 1 im Circle, 31 im gemeinsamen Speicher; 11 aktiv (`_o_`/`_a_`) | 0 |
| Durchsichten im Circle | 2 | 2 (Abgleichsabschnitt je Datei) |
| Circle-Datensatz | 1 | 0 — der Zustandswechsel gehört dem Orchestrator |

**Null neue Datensätze gefilt.** Jeder Fund dieses Durchgangs hatte schon einen offenen
Datensatz; die Konvention verlangt dann eine angehängte Zeile und keine zweite Datei.

## Die zehn Planschritte

**Alle zehn stehen zu Recht auf `[DONE]`.** Die Belegtabelle steht im `## Reconciliation Log`
des Plans und wird hier nicht wiederholt. Der Plan war ausdrücklich als unvermessen zu
behandeln — er hat in dieser Runde vier Zahlen behauptet, die kein Übersetzer hält, und drei
davon waren falsch. Gelesen wurde deshalb je Schritt die Stelle im Baum und nicht die Zusage
im Schritt.

## Die drei Textkorrekturen, jede vorher am Baum gemessen

1. **Der Spec zählte zwei Posten auf dem Hauptfaden und ließ den dritten aus.** Das Auslesen
   der Ablage des Ziehvorgangs war der teuerste der drei: 155 ms bei tausend Einträgen gegen
   ein Bild von 16,7 ms. Seit `4d27c1c` liegt es unter dem Schlüssel
   `NSDraggingInfo::draggingSequenceNumber` in einem Ivar und ist damit ebenfalls konstant.
   Nachgelesen: `tabelle.rs:921` (der Ivar), `:933` (der Schlüssel), `:3218` (die eine Stelle,
   die ihn liest und schreibt).
2. **Das dritte Abnahmekriterium von C6 wäre für zwei Schreibweisen eines Ordners als
   fehlgeschlagen berichtet worden.** Es liest sich jetzt „unter derselben Schreibweise" und
   sagt daneben, dass ein durchgerutschter Fall seit `cac9218` als Zeile in der Abschlussliste
   endet und nicht mehr in einer Löschung. Nachgelesen: `operation::zielpfad`
   (`krk-core/src/operation/mod.rs:252`) vergleicht `(st_dev, st_ino)` statt Text.
3. **Die Prüfstrategie des Plans sagte „vier neue `#[must_use]` und ein `let _ ='".** Am Baum
   gezählt sind es **elf** und **zwei**. Die Stelle nennt jetzt das zählende Kommando statt
   einer dritten Zahl, weil genau diese Zahl in vierundzwanzig Stunden dreimal falsch war.

## Was über Spec, Plan und Auftrag hinaus gefunden wurde

**Die `#[must_use]`-Zahl war schon wieder falsch, als sie mir übergeben wurde.** Der Auftrag
nannte acht und mit `vorgang_laeuft` neun; das war der Stand der Durchsicht des zweiten Turns.
`4d27c1c` und `cac9218` haben danach drei weitere gesetzt — `abwurfmeldung`,
`gemeinsamer_quellordner` und `abwurfziel`, alle in `tabelle.rs`. Am Baumstand sind es elf, und
zwar vier in `appkit/abwurf.rs`, drei in `appkit/tabelle.rs`, zwei in `kommandos/abwurfregel.rs`
und zwei in `appkit/anwendung.rs`. Ein drittes `let _ =` steht in einem `#[cfg(test)]`-Modul und
zählt in der ausgelieferten Menge nicht mit. **Damit ist dieselbe Zahl innerhalb dieser einen
Runde dreimal falsch geworden**, an drei verschiedenen Stellen: im Plan, im Defektdatensatz und
in der Durchsicht. Das ist der Grund, aus dem die berichtigte Stelle jetzt ein `grep` nennt.

**Schritt 5 nennt acht Prosazahlen, berichtigt sind neun.** `71413c3` hat `menue.rs:1132`
mitgenommen: die Stelle verwies auf „die Tafel aus 140 Fällen", während `zulaessigkeit.rs:436`
seit `c3ada4d` 280 hält. Die Abweichung ist gedeckt, weil der Schritt selbst vorschreibt, gegen
den Baum zu zählen und nicht gegen den Plan. Sie steht hier, weil sie die **vierte** falsche
Zahl derselben Runde ist und das Muster ohne sie kleiner aussähe, als es ist.

**Fünf geschlossene Defektdatensätze tragen im Kopf weiter `**Status:** open`.** Es sind die
fünf des zweiten Durchsichtsdurchgangs (`260818-2332` bis `260818-2336`) — die einzigen des
Circles mit einer `**Status:**`-Kopfzeile überhaupt. Dieses Projekt führt denselben Riss für
Entscheidungsdatensätze in zwei Datensätzen; neu ist die Datensatzart. Angehängt an
`shared/issues/260814-1955_*_…`, mit dem Kommando, das ihn zählt.

**`shared/issues/260818-2145_*_…` trägt eine `Resolved:`-Zeile bei Marker `_o_`.** Die
Konvention kennt diese Verbindung nicht. Der Grund ist erkennbar richtig — der behobene Teil
und der offene Teil stehen in einem Datensatz —, aber der offene Teil ist eine Frage und hat
seinen eigenen Entscheidungsdatensatz schon (`shared/decisions/260811-2050_*_…`). Nicht
geändert: die Umbenennung wäre eine Wertung über die offene Frage. Als Anmerkung an den
Datensatz gehängt.

**`CLAUDE.md` nennt zehn gefahrene Runden; es sind dreizehn.** Fünfzehn Circle-Datensätze, davon
zehn beschränkt geschlossen, zwei kohärent geschlossen, einer laufend, einer vorgesehen und
einer zurückgestellt. Die Zahl ist seit dem Filing ihres Datensatzes zum dritten Mal veraltet.
Ebenso steht `**Artifact language:** en` weiter gegen den eigenen Abschnitt `## Sprache` und
gegen den Bestand: Spec, Plan, beide Durchsichten, vierzehn Sitzungsprotokolle und die
Entscheidungsdatensätze dieser Runde sind deutsch, englisch sind allein die elf Defektdatensätze
der zwei `coderev`-Durchgänge. **Beides ist an die bestehenden offenen Datensätze angehängt und
nicht an `CLAUDE.md` geändert** — die Datei gehört einem Kuratorendurchgang und nicht einem
Abgleich.

## Die Datenverlustkette, und warum sie kein Abdriften ist

`cac9218` behebt eine Kette in `krk-core`, die **vor** dieser Runde bestand: `ziel_klaeren`
beantwortete die Konfliktfrage „Überschreiben" mit `loeschen::baum_entfernen(ziel)`, einem
echten `remove_file` und nicht dem Papierkorb. War das Ziel unter zweiter Schreibweise die
Quelle, löschte das die Datei des Nutzers; gegen den unreparierten Baum gefahren endete der Fall
mit „die Quelle ist weg: NotFound". Dieselben drei Zeilen ließen einen Ordner über den
textuellen `starts_with`-Schutz in seinen eigenen Baum absteigen, 139 Einträge weit.

**Die Kette lag im Baum und war unerreichbar; der Abwurf hat sie erreichbar gemacht.** Die
Directive dieser Runde sagt „Was KRK nicht ausführen kann, weist es schon während des Ziehens
ab". Eine Runde, die Abwürfe aus fremden Anwendungen annimmt und dabei einen Weg öffnet, der
die Datei des Nutzers löscht, hat diese Zusage nicht eingelöst, sondern gebrochen. Die Behebung
liegt außerhalb des Wortlauts der Directive und innerhalb dessen, was ihre Einlösung verlangt.
Sie wird deshalb als **Erfüllung der Directive** gewertet und nicht als Abdriften.

Bemerkenswert ist daneben, wo sie sitzt: nicht in `abwurf_pruefen`, wo der Befund sie vorschlug,
sondern an der entscheidbaren Stelle in `krk-core`. Der textuelle Vergleich im Ziehvorgang bleibt
und heißt im Doc-Kommentar jetzt Vorhersage. Das ist derselbe Schnitt, den die `Decidability`-
Zeile des Plans für die ganze Runde gezogen hat: vorher wird vorhergesagt, nachher entschieden.

## Die zwei Entscheidungsdatensätze

Beide `_i_`, beide zitieren `d6343e0`, beide am Baum nachgelesen und zu Recht so markiert. Die
Einzelheiten stehen im `## Reconciliation Log` des Plans. Ein Vorbehalt gehört dazu: der Code,
den sie zitieren, ist in `07347b8` und `15a2978` entstanden; `d6343e0` ist der Commit, der ihn
in Betrieb genommen hat. Das ist die zutreffende Lesart von „umgesetzt" und hier nur
festgehalten, damit eine spätere Suche nach dem Code nicht am Zitat scheitert.

## Die elf Durchsichtsbefunde

Alle elf sind geschlossen und einzeln am Baum nachgelesen; die Tabellen stehen in den zwei
Durchsichtsdateien. **Offene Befunde aus `coderev` oder `ontorev`: null.** Die drei offenen
Datensätze des Circles stammen sämtlich vom `coder` und nicht von einer Durchsicht.

## Was offen bleibt

| Datensatz | Was er festhält |
|---|---|
| `issues/260818-1704_*_` | Schritt 1 des Plans sagt, die Proben blieben grün; sie fallen zu 51. Folgenlos, weil die Schritte 1 und 2 in einem Commit gefahren sind |
| `issues/260818-2221_*_` | `abwurf_ausfuehren` reicht den Zielordner als `quellordner` weiter; die Abschlussliste liest ihn zweimal. Am Baum unverändert |
| `issues/260818-2228_*_` | Plan und `## API Changes` nennen `abwurf_ausfuehren` den dritten Rufer von `auftrag_starten`; es sind vier, und der Baum sagt es an der Funktion selbst |
| `shared/issues/260818-2145_*_` | offen für die Frage dahinter: woran fällt eine **falsche** Verfügbarkeitsangabe auf. Hängt an `shared/decisions/260811-2050_*_` |
| `shared/decisions/260815-1749_*_` | der Doppelklick auf einen Ordner ohne Leserecht schweigt, der Pfadsprung meldet. Diese Runde hat den dritten Weg mit derselben stummen Antwort hinzugefügt |

## Was die Runde nicht ist

**Gebaut, nicht abgenommen.** Die Abnahmekriterien von C4 bis C7 sind sämtlich Nutzerarbeit,
dazu zwei in C1, zwei in C2 und die zwei Kriterien an der Stelle einer elften Zeitzusage. Kein
Agent kann einen Ziehvorgang aus einer zweiten Anwendung erheben oder ein Fenster an seiner
Breite ziehen. Das ist die Eigenschaft dieses Projekts, die `CLAUDE.md` beschreibt, und keine
Häufung von Fehlschlägen: der Marker misst hier die Verfügbarkeit des Nutzers und nicht die
Reife der Runde.

**Vor dem Abnahmelauf, sonst misst er nichts:** wer seit der Runde 7 eine Taste in der
Belegungsansicht zugewiesen hat, hat eine eigene `keymap.toml`, und für ihn kommt
`ordner_angleichen` ohne seine Kombination an
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`,
offen). Der Handgriff steht unter „Nutzerarbeit" im Plan.
