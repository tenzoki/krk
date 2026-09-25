# Durchsicht der Belegungsdatei: `ordner_der_datei` und `teilen`

**Date:** 2026-08-12
**Sender:** ontorev
**Reviewed-range:** `4d4402d..d6eff4b`
**Not-opened:** `crates/krk-core/src/ablage/atomar.rs`, `crates/krk-core/src/ablage/einstellungen.rs`, `crates/krk-core/src/ablage/mod.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/appkit/teilen.rs`, `crates/krk-ui/src/appkit/vorschau.rs`, `crates/krk-ui/src/kommandos/operationen.rs`, `crates/krk-ui/src/main.rs`, `fusion-workbench/circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_s_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_t_circle.md`, `.../decisions/260812-1000_a_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`, `.../decisions/260812-1000_a_was-tut-die-nummernspalte-bei-gerendertem-markdown.md`, `.../decisions/260812-1000_a_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`, `.../decisions/260812-1000_a_welchen-umfang-von-markdown-rendert-die-vorschau.md`, `.../decisions/260812-1000_a_wie-erfaehrt-der-nutzer-dass-eine-ablagedatei-zur-seite-gelegt-wurde.md`, `.../decisions/260812-1000_a_zeigt-die-vorschau-lokale-html-dateien-gerendert.md`, `.../decisions/260812-1000_i_an-welchen-drei-flaechen-haengt-das-neue-kontextmenue.md`, `.../decisions/260812-1000_i_oeffnet-der-ordnersprung-einen-neuen-tab-oder-wechselt-er-den-aktiven.md`, `.../decisions/260812-1000_i_wie-heisst-die-zur-seite-gelegte-ablagedatei-und-was-geschieht-beim-zweiten-mal.md`, `.../decisions/260812-1000_i_wird-die-datei-im-zielordner-ausgewaehlt.md`, `.../decisions/260812-1105_a_die-statuszeile-zieht-ueber-die-volle-fensterbreite-und-laesst-sich-blaettern.md`, `.../decisions/260812-1145_i_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`, `.../decisions/260812-1516_o_hebt-ein-rechtsklick-auf-eine-unmarkierte-zeile-die-markierung-anderswo-auf.md`, `.../history/260812-1055-orchestrator-session.md`, `.../history/260812-1145-planner-session.md`, `.../history/260812-1204-coder-ablage-beschaedigte-datei-zur-seite-legen.md`, `.../history/260812-1432-coder-ordnersprung-in-den-ordner-der-angezeigten-datei.md`, `.../history/260812-1434-coder-teilen-ueber-die-tastatur.md`, `.../history/260812-1500-coder-kontextmenue-an-den-drei-flaechen.md`, `.../history/260812-1600-coder-rechtsklick-bewegt-die-auswahl.md`, `.../issues/260812-1204_o_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`, `.../issues/260812-1500_c_der-rechtsklick-bewegt-die-auswahl-nicht-obwohl-der-nutzerentscheid-es-verlangt.md`, `.../planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, `fusion-workbench/shared/issues/260812-1438_o_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`

**Gegenstand:** `resources/default-keymap.toml`, Commit `95b2dfa`. Der Rust-Anteil des
Bereichs liegt beim `coderev`; hier ist er nur so weit gelesen, wie die Belegung ihn
zitiert. Die fünf teilweise gelesenen Quelldateien —
`crates/krk-core/src/tasten/belegung.rs`, `crates/krk-ui/src/belegungsmodell.rs`,
`crates/krk-ui/src/angezeigtedatei.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/appkit/tabelle.rs` — stehen deshalb nicht unter „Not-opened", sind
aber nur in den Abschnitten gelesen, die die zwei neuen Kennungen berühren.

---

## Zusammenfassung

Die zwei neuen Blöcke sind mechanisch in Ordnung: beide Kombinationen waren frei, die
Schreibweise stimmt, die Zählzeile im Dateikopf ist nachgezählt und richtig, `reserviert_fuer`
steht bei keinem, und beide Kennungen sind in allen drei vollständigen Fallunterscheidungen
des Codes angekommen. Was nicht hält, ist die **Prosa**: drei Begründungen im Kommentar sind
an der Datei oder am Code widerlegt. Sie tragen keinen Laufzeitfehler, aber sie sind die
einzige Aufzeichnung der Reihenordnung, auf die sich die nächste Runde stützen wird.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 3 |
| Niedrig | 0 |

Alle drei sind als eigene Datensätze im Speicher dieses Circles abgelegt (Herkunftsregel:
sie entstehen aus der Directive dieser Runde).

## Was geprüft ist und hält

**Beide Kombinationen waren frei.** Am 260812-1526 selbst nachgezählt, über alle
`tasten`-Listen der Datei und nicht aus dem Bericht des Erzeugers übernommen:
`opt+cmd+o` steht in genau einer Liste (`:245`, `ordner_der_datei`), `shift+cmd+s` in
genau einer (`:610`, `teilen`). Die einzige doppelt vergebene Kombination der Datei ist
`cmd+a` bei `alle_markieren` (`:272`) und `text_alles_auswaehlen` (`:851`); sie ist von
vorher und im Dateikopf als Fokusvorbehalt begründet.

**Die Schreibweise stimmt.** Über alle 87 Kombinationen geprüft: keine trägt einen
unbekannten Zusatztastennamen, und keine verletzt die Reihenfolge
`[ctrl+][opt+][shift+][cmd+]<taste>` aus dem Dateikopf. Die beiden neuen ebenso wenig.

**Die zwei Zahlen im Dateikopf stimmen.** Nachgezählt: 81 `[[funktion]]`-Blöcke,
81 `id`-Zeilen, 87 Einträge über alle `tasten`-Listen. `# Ausgeliefert sind 81 Funktionen
mit zusammen 87 Kombinationen.` (`:34`) trifft beides. Die Probe
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
(`crates/krk-core/src/tasten/belegung.rs:1513`) liest dieselbe Zeile und zählt selbst
nach; sie trägt kein Literal und hätte den Fehlstand angehalten.

**`reserviert_fuer` steht bei keinem der beiden.** Es steht überhaupt bei keinem Eintrag
der Datei — die drei Fundstellen (`:18`, `:144`, `:324`) sind Kommentare, die das Feld
beschreiben. Richtig so: beide Funktionen sind mit den Schritten 3 und 5 derselben Runde
gebaut worden.

**Der angenommene Konflikt ist wahrheitsgemäß ausgeschrieben.** Am Baum geprüft:
„Sichern unter" kommt außerhalb dieses Kommentars an keiner Stelle unter `crates/`,
`resources/` und `README.md` vor, in keiner Schreibung und in keiner englischen Form.
`cmd+s` liegt auf `editor_sichern` mit dem Namen „Sichern" (`:717-719`). Die
Ausweichmöglichkeit `shift+cmd+f` ist tatsächlich frei. Der Kommentar an `teilen`
(`:616-626`) stimmt in jedem Punkt.

**Die Deckung gegen den Code ist vollständig und ohne Rest.** Mengenvergleich über die
ganze Datei: 81 Kennungen, davon 6 mit `gehalten_von = "menue"`, bleiben 75 — genau die
75 Einträge von `Kommando::KENNUNGEN` (`crates/krk-core/src/tasten/belegung.rs:564`).
Keine Kennung ohne Belegungseintrag, kein Belegungseintrag ohne Kommando außer den
sechs zugestellten, keine doppelte Kennung. `OrdnerDerDatei` und `Teilen` stehen in
`KENNUNGEN` (`:573`, `:631`), in `Kommando::wirkungsbereich` (`:850`, `:865`, beide
`Ueberall`) und in `belegungsmodell::bereich_des_kommandos`
(`crates/krk-ui/src/belegungsmodell.rs:185` als `Dateilisting`, `:235` als
`Dateioperationen`).

**Der Platz beider Blöcke stimmt.** `ordner_der_datei` (`:242`) steht unmittelbar hinter
`ordner_aufwaerts` (`:230`) und vor `pfadeingabe` (`:255`), im Abschnitt
„C2: Navigation in der Liste". `teilen` (`:607`) steht unmittelbar hinter
`eintragspfad_kopieren` (`:593`) und vor `mit_standardprogramm_oeffnen` (`:629`). Beide
Sachgruppen tragen die Sorte Handlung, die die Kommentare für sie beanspruchen. Am Kopf
der zweiten Gruppe hängt trotzdem ein Befund — siehe unten.

**Der zitierte Entscheidungsdatensatz besteht.** Beide Kommentare zitieren
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`;
die Datei liegt dort, heute mit dem Marker `_i_`. Die Sternform ist die
Projektschreibweise und richtig gewählt.

## Befunde

### 1. `opt+cmd+c` kopiert nicht den Pfad desselben Ordners (mittel)

`resources/default-keymap.toml:250-251` begründet die Wahl von `opt+cmd+o` damit, der
Befehl sei „der unmittelbare Nachbar von opt+cmd+c, das den Pfad **desselben Ordners**
kopiert". Es sind zwei verschiedene Ordner. `opt+cmd+c` nimmt den Ordner, den das aktive
Dateifenster anzeigt (`crates/krk-ui/src/appkit/tabelle.rs:964-966`, `angezeigter_ordner()`);
`opt+cmd+o` nimmt den Ordner über der Datei, die Vorschau oder Editor zeigt
(`crates/krk-ui/src/appkit/anwendung.rs:2393-2404`, `datei.parent()` über
`angezeigtedatei::welche`). Beide fallen genau dann zusammen, wenn der Sprung ein
Leerlauf wäre — die Zusage ist in der Lage falsch, für die es den Befehl gibt.

Der Satz stammt wörtlich aus Möglichkeit 1 des Entscheidungsdatensatzes und aus dessen
Antwortabschnitt. Die Wahl der Kombination hängt nicht daran, die Begründung schon.

Datensatz: `issues/260812-1526_o_der-kommentar-an-ordner-der-datei-nennt-opt-cmd-c-den-pfad-desselben-ordners.md`

### 2. Beide neuen Kommentare verengen die Reihenordnung (mittel)

Der Entscheidungsdatensatz führt beide Reihen **zweigliedrig**: „`opt+cmd+X` wirkt auf
Ordner **und Bereiche**", „`shift+cmd+X` wirkt auf Eintrag, Auswahl **und Fokus**". Die
Kommentare geben je nur das erste Glied wieder, und in dieser Form widerlegt die Datei
sie:

- `:246-249` sagt, die `opt+cmd`-Reihe trage, „was einen Ordner herstellt oder liefert",
  und zählt dann fünf Beispiele auf, die Bereiche schalten. Der Satz widerlegt sich in
  seiner eigenen Aufzählung. Übergangen sind außerdem `opt+cmd+delete`
  (`endgueltig_loeschen`, `:133`) und `opt+cmd+e` (`editor_schliessen`, `:688`).
- `:611-613` sagt, die `shift+cmd`-Reihe trage, „was auf die betroffenen Eintraege
  wirkt". Von 17 `shift+cmd`-Kombinationen tun das mindestens zehn nicht.

Dazu kommt eine zweite, unvereinbare Beschreibung derselben Reihe an anderer Stelle
derselben Datei: der Kommentar an `editor_schliessen` (`:689-690`) nennt `opt+cmd` die
„Umschaltfamilie".

Das Gewicht liegt nicht im Wortlaut, sondern darin, dass die Reihenordnung der
ausdrückliche Grund für den Nutzerentscheid vom 260812-1105 war und mit diesem Commit
zum ersten Mal aufgeschrieben ist.

Datensatz: `issues/260812-1527_o_die-zwei-neuen-kommentare-verengen-die-reihenordnung-und-widersprechen-der-datei.md`

### 3. Der Abschnittskopf über `teilen` ist alt geworden (mittel)

Der Kopf „Pfade kopieren und mit dem Standardprogramm öffnen" (`:573-580`) sagt „Drei
Funktionen der Runde 4" und „Die drei Kombinationen sind der Nutzerentscheid vom
260811-1505". Der Abschnitt führt seit `95b2dfa` vier Blöcke, und der vierte gehört zur
Runde 6 und zum Entscheid vom 260812-1105. Die Begründung dazwischen trägt `teilen`
dagegen: der Platz ist richtig, der Kopf ist alt. Keine Probe hält das an — die
Zählzeile im Dateikopf ist geprüft, Abschnittsköpfe sind es nicht.

Datensatz: `issues/260812-1528_o_der-abschnittskopf-nennt-drei-funktionen-der-runde-4-und-fuehrt-jetzt-vier.md`

## Reihenfolge der Berichtigung

1. **Befund 2 zuerst.** Er legt fest, wie die Datei ihre eigene Ordnung beschreibt, und
   die beiden anderen Berichtigungen fassen dieselben Kommentarblöcke an. Wer ihn nicht
   zuerst entscheidet, fasst die Blöcke zweimal an.
2. **Befund 1 im selben Zug**, weil er in einem der beiden Blöcke steht, den Befund 2
   ohnehin öffnet.
3. **Befund 3 danach**, unabhängig von beiden. Die Frage, ob Abschnittsköpfe künftig
   Runden oder Fähigkeiten nennen, gehört mit hinein.

Keiner der drei hält Code an, und keiner ist ein Grund, den Commit zurückzunehmen.

## Ein Hinweis ohne eigenen Datensatz

Beide neuen Kommentare setzen den zitierten Pfad über drei Kommentarzeilen um
(`:247-249`, `:624-626`), also `circles/` auf der einen, der Verzeichnisname auf der
zweiten, `decisions/` und der Dateiname auf der dritten. Kein `grep` nach dem Pfad
findet ihn danach. Das ist **kein neuer Befund**: die Datei macht es an den älteren
Zitaten genauso (etwa `:577-578`), und CLAUDE.md führt die Sorte Fehlstelle bereits unter
„Jedes Suchmuster dieses Projekts, das `\.md` verlangt, hat einen blinden Fleck"
(`shared/issues/260810-1851_*_acht-verweise-…`, geschlossen). Es steht hier, weil die
zwei neuen Zitate den Pfad über eine Zeile mehr brechen als die bestehenden, und weil
jede künftige Erhebung über die Verweise dieser Datei sie übersieht.
