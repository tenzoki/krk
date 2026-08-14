# B2 — Ordnerwechsel, Tabwechsel und `Esc`

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang B, Schritt B2
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.7, C1.8, C1.9, C1.10, C3.5
**Verification:** `make check` — exit 0

## Wo `Esc` gelandet ist, und was vor ihm steht

`Anwendungsdelegierter::abbrechen` (`crates/krk-ui/src/appkit/anwendung.rs`) hat jetzt drei
Ränge statt zwei, und der Filtertext ist der dritte und letzte:

```text
esc ──> steht ein Blatt?            ──ja──> es schliessen
         │ nein
         └──> laeuft eine Operation? ──ja──> sie abbrechen
               │ nein
               └──> steht ein Filtertext? ──ja──> ihn loeschen
                     │ nein
                     └──> nichts, wie vor dieser Runde
```

Vor ihm stehen also genau die beiden Bedeutungen, die die Taste schon hatte: das Schließen
eines stehenden Blattes und der Abbruch einer laufenden Dateioperation. Der dritte Rang
sitzt an der Stelle, an der die Funktion bis zum 260815 `false` lieferte, weil sie nichts
mehr zu tun fand — das ist die Empfehlung aus
`decisions/260814-1830_o_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`.
Eine andere Antwort verschiebt den Rang innerhalb dieser einen Funktion.

**Der Rumpf des Vorgangsrangs ist dabei umgebaut, ohne seine Bedeutung zu ändern.** Er
sprang bisher aus der Ausleihe heraus mit `return false`, und genau dieser Rücksprung ist
der Ort des neuen Rangs. Aus dem `let ... else { return false }` ist ein `map` über die
Ausleihe geworden, das den Abbruch als Seiteneffekt trägt und Art und Seite herausgibt; der
Nachzug der Fortschrittszeile und der Rücksprung stehen danach hinter der Ausleihe. Das ist
dieselbe Folge von Wirkungen wie zuvor, nur mit einem Ausgang statt zweier.

**Ein eigener Rang für das Anhalten des Durchlaufs ist nicht entstanden** (C3.5). Ohne
Filtertext hat der Durchlauf keinen Gegenstand; ein vierter Rang beantwortete dieselbe
Frage ein zweites Mal. Der Durchlauf selbst ist Strang F und steht noch aus.

**Der Filter der Tiefe bleibt stehen.** `Esc` löscht den Text und legt keinen Schalter um.
Ein Schalter, den eine Taste unbemerkt umlegt, wäre eine zweite Quelle für seinen Stand
neben der Bereichsleiste.

**Getroffen wird der sichtbare Tab des aktiven Dateifensters.** `Kommando::Abbrechen` trägt
`Wirkungsbereich::Ueberall`, kommt also auch aus dem Editor und aus der Leiste an; das Ziel
ist dasselbe wie beim Umschalten der tiefen Suche aus E1, und der Grund steht dort schon
ausgeschrieben: ein Wirkungsbereich, der dafür den Fokus verlangte, machte die Taste davon
abhängig, wo die Schreibmarke steht.

## Der Rumpf des dritten Rangs

`DateifensterQuelle::filter_leeren` (`crates/krk-ui/src/appkit/tabelle.rs`), in der Bauart
von `letztes_filterzeichen_weg` daneben: Ausleihe, Änderung am Modell, danach
`nach_filteraenderung` für die Anzeige. **Damit nimmt jede Änderung des Filtertexts
denselben Weg in die Ansicht** — das Tippen aus B1, das Zurücknehmen eines Zeichens aus C2
und jetzt das Löschen.

Es liefert mit `#[must_use]`, ob etwas zu löschen war. Der Wert entscheidet beim Aufrufer
allein darüber, ob `esc` als gewirkt gilt; ohne ihn müsste `abbrechen` die Frage „steht ein
Filtertext" ein zweites Mal stellen.

## Was der Ordnerwechsel überträgt

`Tabliste::ordner_setzen` (`crates/krk-ui/src/tabs.rs`) rettet jetzt vier Werte aus dem
alten `Ordnermodell` in das neue statt zweier: Sortierung, Verstecke, den Filter der Tiefe
und den Filtertext. Die vierte Übertragung hat als einzige eine Bedingung:

| Filter der Tiefe | Filtertext nach dem Wechsel | Kriterium |
|---|---|---|
| aus | leer | C1.9 |
| an | unverändert | C1.10 |

**Die Bedingung steht als eine Zeile da**, `let filtertext_ueberlebt = tief;`, mit dem
Verweis auf
`decisions/260814-1830_o_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`
daneben. Fällt die Antwort später auf „stehen lassen", wird aus dieser Zeile ein `true`,
und sonst ändert sich nichts.

**Der Aufstieg hat keine eigene Zeile bekommen**, weil er keine braucht: `ordner_lesen` in
`appkit/tabelle.rs` ist die eine Tür für hinein, hinaus, Pfadeingabe und Sprung aus der
Zwischenablage, und sie ruft `ordner_setzen`. Der Aufstieg zählt damit wie der Einstieg,
wie C1.9 es sagt.

**In die Sitzung geht nichts davon.** Die beiden neuen Werte werden vom alten Modell in das
neue getragen und nicht über `Tabzustand`, der `session.toml` schreibt.

**Beide Setzer laufen unbedingt und nicht nur bei einer Änderung.** Das frische Modell hat
null Einträge; `tief_setzen` und `filtertext_setzen` bauen also je eine leere Sicht neu auf,
und ein Zweig davor wäre eine zweite Stelle, an der die Übertragung anders ausfallen könnte.

## Was ohne Zeile angefallen ist

**Der Tabwechsel.** `tab_gewechselt` ist unberührt. Der Filtertext gehört dem
`Ordnermodell`, das `Tabinhalt` hält; ein Wechsel setzt die sichtbare Stelle um, und die
Ansicht zeigt danach das Modell des neuen Tabs — mit dessen Filtertext (C1.8). Die
Vorhersage des Plans hat gehalten, und es war keine Zeile zu schreiben.

**Die Auffrischung.** `Tabliste::aktiven_neu_lesen` lässt den Filtertext stehen, ebenfalls
ohne Zeile: der Tab bleibt seit `5f2e45d` stehen und entsteht nicht neu, also bleibt sein
Modell und damit sein Filtertext. Angefasst ist dort allein die Prosa, die das jetzt
ausdrücklich sagt.

## Proben

Fünf neue, alle in `crates/krk-ui/src/tabs.rs` im `#[cfg(test)]`-Modul — die Datei trägt
keine Zeile AppKit und ist deshalb der Ort, an dem der prüfbare Teil dieses Schritts steht.

| Probe | Kriterium |
|---|---|
| `ein_ordnerwechsel_leert_den_filtertext_wenn_die_tiefe_suche_aus_ist` | C1.9 |
| `mit_tiefer_suche_ueberlebt_der_filtertext_den_ordnerwechsel` | C1.10 |
| `die_tiefe_suche_geht_auch_ohne_filtertext_hinueber` | C1.10, Randfall |
| `eine_auffrischung_laesst_den_filtertext_stehen` | die Zusage aus B2 zu `aktiven_neu_lesen` |
| `der_filtertext_gehoert_dem_tab_und_nicht_dem_fenster` | C1.8 |

Sie laufen gegen vorhandene Ordner (`std::env::temp_dir()` und `/`), weil `ordner_setzen`
und `waehlen` einen Lesevorgang starten; geliefert hat der in keiner Probe etwas, weil
`einziehen` nicht gerufen wird.

## Was offen geblieben ist

**Die Probenhälfte von C1.7.** Der Spec kennzeichnet das Kriterium mit „Probe für die
Reihenfolge, Bündel für den Tastendruck". Die Bündelhälfte gehört G2. Für die Probenhälfte
gibt es keinen Ort: die Rangfolge hängt an drei Ivars des Anwendungsdelegierten
(`offenes_blatt`, `vorgang`, das Tabmodell des aktiven Dateifensters), und `krk-ui` hat kein
Bibliotheksziel. Eine reine Funktion über drei Wahrheitswerte wäre der naheliegende Weg,
aber ein siebter Typ, den die `## Data Structures` des Plans nicht führt. Datensatz:
`issues/260815-0020_o_c1-7-verlangt-eine-probe-fuer-die-reihenfolge-von-esc-und-b2-hat-keinen-ort-dafuer.md`.

**Eine zweite Stelle baut denselben frischen `Tabinhalt`** und ist nicht mitgezogen:
`Tabliste::verdeckten_tab_setzen`, der Weg des Datenträgerauswurfs, trägt weiter zwei von
jetzt vier Werten hinüber. Datensatz:
`issues/260815-0020_o_verdeckten-tab-setzen-baut-denselben-frischen-tabinhalt-und-traegt-zwei-von-vier-werten-hinueber.md`.

## Geänderte Dateien

- `crates/krk-ui/src/tabs.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`

Genau die drei, die die Zeile `Files:` von B2 nennt. Der Befund aus
`issues/260814-2357_o_c2-nennt-zwei-dateien-…` ist hier nicht wieder eingetreten und
deshalb auch nicht angehängt worden.

## Verification

```
make check — exit 0
```

Alle vier Abnahmekommandos grün, `clippy` unter `-D warnings`.
