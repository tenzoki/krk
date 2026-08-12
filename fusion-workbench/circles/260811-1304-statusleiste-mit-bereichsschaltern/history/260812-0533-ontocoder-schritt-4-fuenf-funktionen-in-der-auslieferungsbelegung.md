# Ontocoder, Schritt 4: Fünf neue Funktionen in der Auslieferungsbelegung

**Datum:** 260812-0533
**Agent:** ontocoder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 4
**Abnahme:** `make check` — **Exit 2**. Grün sind die beiden Prüfungen, die der Schritt nennt; rot sind einunddreißig andere, davon achtundzwanzig am benannten Zwischenstand und drei ohne Planschritt (`issues/260812-0533_o_drei-proben-stehen-gegen-die-neuen-belegungseintraege-und-keine-gehoert-zu-einem-planschritt.md`).

## Auftrag

Schritt 4 des Plans und nur dieser, genau eine Datei: `resources/default-keymap.toml`. Bindend
sind `decisions/260812-0415_a_welche-kombinationen-bekommen-die-beiden-neuen-umschalter.md`
(Empfehlung übernommen, Nutzerantwort vom 260812-0430) und
`decisions/260812-0306_a_bekommen-die-spaltenschalter-tastenbefehle.md` (Möglichkeit 2: geführt,
ohne ausgelieferte Kombination). Nicht committen.

## Was entstanden ist

Alles in `resources/default-keymap.toml`, die nach C3 die eine Quelle jeder Tastenbelegung
dieses Projekts ist.

**Zwei Umschalter mit Kombination.**

- `erstes_fenster_umschalten`, „Linkes Dateifenster ein- und ausblenden", `opt+cmd+left`, im
  C7-Block **vor** `zweites_fenster_umschalten`, damit links vor rechts steht.
- `editor_umschalten`, „Editor ein- und ausblenden", `opt+cmd+b`, im Editor-Block direkt hinter
  `editor_schliessen`.
- `zweites_fenster_umschalten` trägt jetzt `["opt+cmd+d", "opt+cmd+right"]`. `opt+cmd+d` steht
  zuerst und bleibt damit die Kombination, die die Belegungsansicht als erste zeigt.

**Drei Spaltenschalter ohne Kombination**, mit leerer Tastenliste und **ohne** `reserviert_fuer`:
`spalte_groesse_umschalten`, `spalte_datum_umschalten`, `spalte_typ_umschalten`, im C2-Block
hinter `versteckte_umschalten`. Der Block trägt einen gemeinsamen Kommentar davor: warum leer
statt reserviert, warum die Knappheit der 39 frei gewählten Kombinationen der Grund ist, und dass
die Spalte Name keinen Schalter und deshalb keinen Eintrag bekommt.

**Je Eintrag eine Begründung**, wie die Datei es durchgehend hält. Der Pfeil beim linken
Dateifenster ist als einziger Bruch mit der Umschaltfamilie benannt; das `b` ist als „bearbeiten"
begründet, mit dem Satz, warum `opt+cmd+e` nicht den Besitzer wechselt.

**Zwei Kommentare, die sonst das Gegenteil des Inhalts gesagt hätten.**

- Der Familienkommentar bei `editor_schliessen` legte die Umschaltfamilie auf
  `opt+cmd+<Buchstabe>` fest und nannte das Schließen die Ausblendhälfte. Er nennt jetzt
  `opt+cmd+b`, die beiden Pfeiltasten, und den Unterschied zwischen Schließen (gibt die Datei
  frei, fragt nach C4 nach) und Umschalten (blendet aus, behält den Stand).
- Der Dateikopf behauptete, einen Eintrag ohne `gehalten_von` und ohne Kommando „liefert die
  Datei derzeit nicht". Seit diesem Schritt liefert sie fünf. Der Satz ist auf eine Form
  umgeschrieben, die nicht wieder veraltet: solche Einträge führt die Datei, sooft eine Runde
  ihre Funktionen vor ihren Kommandos einträgt.

**Die Zählzeile im Kopf** steht auf `# Ausgeliefert sind 79 Funktionen mit zusammen 85
Kombinationen.` Nachgezählt statt übernommen: 74 + 5 = 79 Blöcke, 82 + 3 = 85 Kombinationen (zwei
neue Umschalter, dazu `opt+cmd+right`). Gegengezählt an der Datei mit `grep -c '^\[\[funktion\]\]'`
und der Zahl der Zeichenketten in allen `tasten`-Zeilen: 79 und 85.

**Die drei neuen Kombinationen waren frei**, am Baum geprüft und nicht angenommen: unter `opt+cmd`
führte die Datei `delete`, `l`, `d`, `g`, `c` und `e`; die Pfeiltasten kamen nur nackt, mit `cmd`
und mit `ctrl` vor. `die_auslieferungsbelegung_ist_konfliktfrei` bestätigt es.

## Abnahme

`make check` — **Exit 2**, und das ist der Befund des Schrittes und keine Nachlässigkeit.

**Grün, und das sind die beiden Prüfungen, die der Schritt nennt** (`cargo test -p krk-core --lib
tasten::belegung`, Exit 0): `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`,
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, dazu
`beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren` und
`eine_belegung_ueberlebt_schreiben_und_wiedereinlesen`. Damit ist die Datei gültiges TOML, geht
durch `Belegung::bauen` ohne Konflikt, und Kopfzeile und Inhalt stimmen überein.

**Rot, achtundzwanzig Proben in `krk-ui`, am vom Plan benannten Zwischenstand.**
`belegungsmodell::bereich` findet für die fünf neuen Kennungen keinen Funktionsbereich, weil sie
noch kein Kommando tragen. Die Schritte 5 und 7 geben ihnen eines; danach greift der erste Zweig
von `bereich`, und die achtundzwanzig laufen wieder.

**Rot, drei Proben, die kein Schritt dieses Plans wieder grün macht.** Sie stehen als
`issues/260812-0533_o_drei-proben-stehen-gegen-die-neuen-belegungseintraege-und-keine-gehoert-zu-einem-planschritt.md`
mit Vorschlag: eine Probe schreibt die 74 als Literal hin und trägt sie im Namen, eine verlangt
von jeder nicht reservierten Funktion mindestens eine Kombination (gegen die Antwort vom
260812-0306), und eine behauptet, ab Werk sei keine Funktion unbelegt. Nicht mitbehoben: alle drei
liegen in Prüfdateien und gehören dem `coder`.

`cargo clippy` und `cargo fmt` sind in diesem Lauf nicht gefahren, weil `make check` an `cargo
test` abbricht. Der Schritt fasst keine Rust-Datei an; die Änderung erreicht den Übersetzer allein
über `include_str!`, und `cargo build --workspace` ist im selben Lauf grün durchgelaufen.

## Was dieser Schritt nicht tut

- **Kein Kommando.** Die fünf Einträge tragen für die Dauer eines Schrittes keines; das ist der
  in der Datei dokumentierte Zustand „belegt, aber noch nicht gebaut", und `Funktion::kommando`
  liefert `None`. Am laufenden Bündel drücken `opt+cmd+left` und `opt+cmd+b` bis Schritt 5 ins
  Leere.
- **Kein `reserviert_fuer`.** Das Feld heißt „benannt, aber einer späteren Runde vorbehalten",
  und die fünf Funktionen gibt es ab Schritt 5 beziehungsweise 7 wirklich.
- **Keine Umbenennung.** `zweites_fenster_umschalten` behält seine Kennung; eine `keymap.toml`
  des Nutzers, die eine unbekannte Kennung nennt, wird als Ganzes abgewiesen.
- **Kein Marker gewechselt.** Die beiden bindenden Datensätze bleiben auf `_a_`: ihre Umsetzung
  ist mit den Kommandos aus den Schritten 5 und 7 fertig, nicht mit den Einträgen, und die Zeile
  `Implemented:` verlangt einen Commit-Hash, den dieser Schritt nicht hat.

## Am Plan nachgezogen

Schritt 4 steht auf `[DONE]`. Die Anmerkung am Schritt nennt jetzt den beantworteten Datensatz
mit seinem heutigen Marker `_a_` und trägt den Befund zur Abnahme nach: `make check` läuft nach
diesem Schritt nicht durch, und warum das so gehört.

Nicht committet: der Orchestrator trägt ein.
