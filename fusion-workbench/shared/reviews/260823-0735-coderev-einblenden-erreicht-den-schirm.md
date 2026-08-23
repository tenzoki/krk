# Durchsicht: `df8163d` — das Einblenden erreicht den Schirm

**Reviewed-range:** `ab11eb8..df8163d`
**Not-opened:** none
**Getragen aus der vorigen Durchsicht:** nicht erhoben — die vorige Durchsicht hat kein
`**Not-opened:**`-Feld geführt. Das heißt nicht „keine", sondern „nicht aufgeschrieben"; welche
Dateien sie ungeöffnet ließ, ist aus ihrem Text nicht wiederzugewinnen.

**Durchgesehen von:** coderev, 260823-0735
**Bereich als beauftragt:** ein Commit, eine Produktionsdatei
(`crates/krk-ui/src/appkit/anwendung.rs`), dazu drei Workbench-Dateien.

## Summary

Die Korrektur selbst hält. `sichtbarkeit_aendern` ist nachweislich die eine Stelle, an der die
Sichtbarkeit im Fenstermodell wechselt, der Nachzug steht damit an der Quelle, und die drei
neuen Proben halten die Verdrahtung wirklich. Die drei Fragen der Beauftragung sind beantwortet:
der Doppelruf ist idempotent, die Stelle der Messung ist richtig gewählt und deckt eine zweite,
bisher offene Lücke mit, und die Proben laufen nicht bloß mit. Was bleibt, ist Prosaschuld an
drei Stellen, eine Reihenfolgefrage, die nur der Abnahmelauf entscheiden kann, und ein
Verhaltensbefund am Nachbarn, den die Durchsicht mitgefunden hat und der älter ist als dieser
Commit.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 2 |

## Die drei Fragen der Beauftragung

### 1. Der Doppelruf von `aufteilung_nachziehen` — folgenlos, aber ungemessen

`anwenden` (`aufteilung.rs:322-330`) schreibt beide Male dieselben Werte aus demselben Modell:
`setHidden` mit unverändertem Wert, `auslegen` (`:586-605`) mit identischen Rahmen,
`wuensche_merken` als `Cell`-Zuweisung. `statuszeile_nachziehen` (`anwendung.rs:4700-4733`) baut
Zeichenketten und nimmt nichts weg — kein `take()`, kein Verbrauch. `bereichsleiste_nachziehen`
schreibt zehn Schalterzustände aus demselben Modell. Gezeichnet wird einmal, am Ende des
Durchgangs.

**Flackern schließe ich aus, Kosten nicht.** Der zweite Durchgang kostet eine Handvoll
Obj-C-Nachrichten und zwei Zeichenkettenbauten je Umschalttastendruck. L1 ist zuletzt am 260810
gemessen. Das ist keine Vermutung eines Rückschritts, sondern eine Zeile für die Liste des
nächsten Abnahmelaufs; sie steht in `260823-0732`.

**Einen dritten Ruf gibt es**, und er ist der interessantere Teil der Frage — siehe die
Reihenfolgefrage unten.

### 2. `bildschirmbreiten_uebernehmen()` am Kopf von `editorausgang_behandeln` — richtig gewählt

Die Stelle ist die erste Anweisung des Rumpfs, also vor **beiden** Sichtbarkeitsänderern des
Rumpfs (`fokus_holen` bei `:6416`, `editor_ausblenden` bei `:6487`). Sie ist unbedenklich, auch
wenn die Aufteilung noch nicht ausgelegt ist: `Fenstermodell::breiten_uebernehmen`
(`fenstermodell.rs:920-946`) hält an `traegt_eine_ziehbewegung`, lässt sichtbare Bereiche ohne
gemessene Breite aussen vor und kehrt bei einer Summe von null um.

**Sie deckt eine zweite, bisher offene Lücke mit.** Der Zweig `Zurueckgehalten` (`:6470`) zieht
das Blatt `Anlass::AndereDatei` auf, und dessen Fortsetzung `anlass_ausfuehren` ruft
`aufteilung_nachziehen` (`:6836`) **ohne** eigene Messung. Der Doc-Kommentar von
`bildschirmbreiten_uebernehmen` (`:4500-4506`) begründet das damit, der Befehl, der das Blatt
aufgezogen habe, sei über den Kopf von `kommando_ausfuehren` gekommen. Für `AndereDatei` galt
das nicht: zwischen jener Messung und dem Blatt liegt das Lesen auf dem Arbeitsfaden, und dort
kann der Nutzer ziehen. Seit `df8163d` misst `editorausgang_behandeln` vor
`nachfrage_zeigen`, und die Begründung trägt wieder.

**An den übrigen Aufrufwegen fehlt sie an einer Stelle**, und das ist der eigene Befund
`260823-0731`: `aktives_setzen` (`:4320-4325`) zieht die Aufteilung ohne vorherige Messung nach.

### 3. Die drei Proben halten — nicht bloß mitgelaufen

`rumpf` (`:7813-7827`) schneidet ab `fn <name>(`, also **ohne** den Doc-Kommentar, und zieht
Kommentarzeilen ab. Die Nadeln tragen alle eine öffnende Klammer, die Doc-Verweise der Form
``[`Self::aufteilung_nachziehen`]`` tragen keine — kein Treffer in Prosa. Die Rumpfgrenze
`"\n    }\n"` trifft in beiden gelesenen Methoden die schließende Klammer der Methode und keine
innere. Alle drei laufen und sind grün (`cargo test -p krk-ui sichtbarkeitsproben`).

**Eine der drei hält weniger, als ihr Doc-Kommentar zusagt.**
`die_editorfortsetzung_misst_vor_dem_einblenden` prüft die Messung nur gegen `fokus_holen(` und
lässt den Zweig `Abgewiesen` ungedeckt; der Befund steht in `260823-0733`.

## Befunde nach Thema

### Prosa, die die Korrektur falsch gemacht hat — Medium

`shared/issues/260823-0730_o_drei-prosastellen-um-den-neuen-nachzug-sind-mit-df8163d-falsch-geworden.md`

Der Commit hat vier Prosastellen mitgezogen und drei nicht:

- `:4500-4506` zählt „die beiden uebrigen Aufrufer" von `aufteilung_nachziehen`. Es sind vier,
  zwei davon ungenannt, und für einen von ihnen gibt es keine Begründung — er ist der
  Verhaltensbefund unten. Der Satz war schon vor `df8163d` um eins daneben und ist jetzt um zwei.
- `:3462-3468` (`ordner_angleichen`) sagt, ein hervorgeholtes Dateifenster bekomme seinen
  Nachzug „allein" über den Rückgabewert. Seit `df8163d` bekommt es ihn zuerst aus
  `sichtbarkeit_aendern`.
- `:4174-4177`, mit `df8163d` neu geschrieben, datiert den Nachzug in `anlass_ausfuehren` auf
  diesen Befund. `git blame` weist ihn als `d18913e6` vom 260810 aus.

### Ein Klick nimmt eine Ziehbewegung zurück — Medium, älter als dieser Commit

`shared/issues/260823-0731_o_ein-klick-in-das-andere-dateifenster-nimmt-eine-ziehbewegung-zurueck.md`

`aktives_setzen` ruft `aufteilung_nachziehen` ohne vorherige Messung. Eine Trennlinie mit der
Maus verschieben und dann ohne Tastendruck in das **andere** Dateifenster klicken, schiebt die
Trennlinie zurück; `sitzung_vormerken` in der Zeile danach schreibt die zurückgeschobene Lage
in die `session.toml`. Der Weg entstand mit `537fda53` vom 260804 und wurde mit `76ceb683` vom
260819 breiter. Gefunden hier, weil `df8163d` genau die Prosastelle stehen ließ, die diesen
Aufrufer als geprüft mitzählt. Nicht am laufenden Bündel bestätigt.

### Reihenfolge beim Ausblenden und die Kosten des Doppelrufs — Low bis Medium, unentschieden

`shared/issues/260823-0732_o_der-neue-nachzug-laeuft-vor-dem-fokusumzug-und-oeffnet-den-ring-den-eine-probe-offen-haelt.md`

Für das **Einblenden** ist „die Fläche steht zuerst" richtig und vom Commit gut begründet. Für
das **Ausblenden** dreht dieselbe Zeile die Reihenfolge um: `setHidden(true)` trifft jetzt eine
Ansicht, die den Ersthelfer noch hält, und erst danach zieht `nach_dem_sichtbarkeitswechsel`
den Fokus absichtlich weg. Damit wird der Ring, den `fokusnachzugproben` (`:8046-8062`)
schriftlich führt — `setHidden` → `makeFirstResponder:` → Melder →
`aktives_dem_ersthelfer_nachziehen` → `aktives_setzen` → `aufteilung_nachziehen` —, von einer
neuen Stelle aus betreten, und zwar **vor** dem gewollten Fokusumzug. `aktives_setzen` kann
dabei `modell.aktiv()` umsetzen, und `fokus_setzen(Fokus::Dateifenster)` löst sein Ziel über
genau dieses `aktiv` auf (`:2234`).

**Ob das eintritt, hängt daran, welchen Rang AppKit nach dem Ausblenden vergibt, und das ist
aus dem Baum nicht zu entscheiden.** Nimmt es das Fenster selbst, kehrt
`aktives_dem_ersthelfer_nachziehen` sofort um und nichts davon geschieht. Zu bedenken ist auch,
dass die alte Reihenfolge nicht sauber war, nur anders unsauber. `df8163d` hat keine saubere
Ordnung durch eine schmutzige ersetzt. Der Datensatz führt deshalb eine Prüfliste für den
Abnahmelauf und keine Behauptung eines Rückschritts.

### Die Probe deckt einen von zwei Zweigen — Low

`shared/issues/260823-0733_o_die-probe-zur-editorfortsetzung-laesst-den-zweig-abgewiesen-ungedeckt.md`

## Was ich geprüft und **nicht** beanstandet habe

- **`sichtbarkeit_aendern` ist wirklich die eine Stelle.** `grep` über alle
  `modell.borrow_mut()` in `crates/krk-ui/src/appkit/` liefert in `anwendung.rs` sechs
  Schreiber: `aus_sitzung` (`:1044`, gefolgt vom Nachzug bei `:1292`), `fenster_wechseln`
  (`:3014`), `spalte_umschalten` (`:4044`), `sichtbarkeit_aendern` (`:4196`), `breite_aendern`
  (`:4304`), `aktiv_setzen` (`:4321`). Nur einer ändert Sichtbarkeit.
  `Fenstermodell::sichtbar_setzen` (`fenstermodell.rs:524`) ist privat; die Zusage im
  Modulkopf der neuen Proben trifft zu.
- **Die Ausleihe des `RefCell` ist beendet, wenn `aufteilung_nachziehen` läuft.** `let nachher =
  …borrow().sichtbarkeit();` gibt den `Ref` mit dem Semikolon frei. Der reentrante Weg über
  `anwenden` → Melder → `aktives_setzen` → `borrow_mut()` findet keine offene Ausleihe vor.
- **Der Ring ist begrenzt.** `aktives_setzen` handelt nur, wenn `aktiv_setzen` `true` liefert;
  beim zweiten Durchgang tut es das nicht.
- **`ersthelfer_gehoert_appkit` und `ist_eigene_textflaeche` sind nicht berührt.** Der Commit
  meldet keine neue Textfläche an und nimmt keine ab.
- **Die Belegung ist nicht angefasst.** `EditorAusVorschau` trägt unverändert
  `Wirkungsbereich::Vorschau`; die Commit-Nachricht sagt es zu und der Baum hält es.
- **`make check` fährt durch.** Die vier Abnahmekommandos wurden vom Auftraggeber nach dem
  Commit mit Rückgabewert 0 gefahren; `cargo test -p krk-ui sichtbarkeitsproben` habe ich
  nachgefahren.

## Wo ich an die Grenze stoße

Drei Aussagen dieser Durchsicht ruhen auf Codelektüre und sind ohne KRK im Vordergrund nicht zu
entscheiden: dass die Fläche nach `df8163d` wirklich erscheint und der Fokus darin landet
(die eigentliche Zusage des Commits), die Reihenfolgefrage beim Ausblenden, und der
Verhaltensbefund `260823-0731`. Der Abnahmelauf ist Nutzerarbeit; die Prüflisten stehen in den
Datensätzen.

## Empfohlene Reihenfolge

**Kein Auslieferungshindernis.** Die Korrektur ist besser als der Stand davor, und keiner der
vier Befunde ist ein Rückschritt gegenüber `ab11eb8`.

1. `260823-0730` (Prosa) — eine Bearbeitung, drei Stellen, ohne Verhaltensrisiko. Am besten
   zusammen mit der Entscheidung aus 2, damit `:4500-4506` nur einmal angefasst wird.
2. `260823-0731` (Ziehbewegung) — braucht eine Entscheidung vorab: Messung an die Rufer oder in
   `aufteilung_nachziehen` selbst. Nicht nebenbei greifen.
3. `260823-0733` (Probe) — klein, gehört zur nächsten Berührung dieser Datei.
4. `260823-0732` (Reihenfolge, L1) — auf die Liste des nächsten Abnahmelaufs. Vorher ist nichts
   zu tun.
