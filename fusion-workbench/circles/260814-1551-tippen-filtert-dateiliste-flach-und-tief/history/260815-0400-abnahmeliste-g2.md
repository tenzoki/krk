# Abnahmeliste G2 — die zehnte Runde am laufenden Bündel

**Datum:** 260815-0400
**Agent:** coder (bereitet vor und schreibt auf; die Beobachtungen selbst sind Nutzerarbeit)
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang G, Schritt G2
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, 77 Abnahmekriterien
**Bündel:** `target/KRK.app`, gebaut und signiert am 260815-0155 mit „KRK Entwicklung", Version 0.3.0
**Nachgezogen:** 260815-0246, auf die Spec-Berichtigung derselben Uhrzeit. Zwei Kriterien sind dazugekommen (C2.14, C3.15), sechs sind umformuliert (C1.11, C3.8, C3.10, C3.13, C5.5, C6.10). **Die zehn Beobachtungen des Nutzers sind unverändert** — der Durchgang unten ist derselbe wie vorher.

**Das beglaubigte Bündel ist vorher gesichert worden** und liegt unter
`~/Library/Caches/krk-beglaubigt-260814-1054/KRK.app`. Es ist dort als *accepted,
source=Notarized Developer ID* geprüft; der Entwicklungsbau unter `target/KRK.app` hat es
also nicht verdrängt, sondern nur die Kopie im Baum ersetzt. Der offene Defekt dazu ist
`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-…`.

---

## Die Zahlen, am Dokument nachgezählt

Nicht aus dem Absatz des Spec übernommen, sondern an seinen 77 nummerierten Kriterien
gezählt (Fähigkeitsüberschrift für Fähigkeitsüberschrift, Kennzeichnung für
Kennzeichnung):

| | Zahl |
|---|---|
| Kriterien insgesamt | **77** — C1 zwanzig, C2 vierzehn, C3 fünfzehn, C4 zehn, C5 sieben, C6 elf |
| allein **(Probe)** | **67** |
| allein **(Bündel)** | **2** — C3.11, C3.12 |
| **beides** | **8** — C1.7, C1.15, C1.17, C1.18, C1.20, C2.1, C4.5, C5.4 |
| mit Bündelanteil, also Nutzerarbeit | **10** |
| ohne Kennzeichnung | **0** |

**Zehn der 77 Kriterien haben einen Bündelanteil**, dieselben zehn wie am 260815-0400. Sie
sind unten die zehn Beobachtungen und stehen einzeln in der Kriterientafel. Die zwei neuen
Kriterien C2.14 und C3.15 tragen beide allein **(Probe)** und ändern am Durchgang des
Nutzers nichts.

**Der Baumanteil ist gefahren.** `make check` ist nach jedem der dreizehn Bauschritte grün
gelaufen; der letzte Lauf steht im Bericht zu F2
(`history/260815-0330-coder-f2-der-tab-haelt-den-durchlauf.md`).

---

## Der Durchgang: eine Reihenfolge, drei Orte

Der Durchgang läuft von oben nach unten und wechselt den Ort dreimal. Wer die Reihenfolge
einhält, muss nicht zwischen Fenstern springen, und die vier gefährlichen Beobachtungen
liegen alle im selben Prüfordner.

```text
Ort 1  das Fenster, wie es startet        Beobachtung 6, 10       nichts wird angefasst
Ort 2  der Pruefordner mit Wegwerfdateien Beobachtung 1 bis 5     hier wird geloescht
Ort 3  ein grosser Baum unter ~           Beobachtung 7, 8, 9     nur gelesen
```

### Vorher: den Prüfordner anlegen

Die vier gefährlichen Beobachtungen räumen Dateien weg. **Sie laufen in diesem Ordner und
in keinem anderen.** Ein Ordner des Nutzers ist dafür der falsche Ort, und das ist keine
Vorsicht ohne Anlass: die geprüfte Fallunterscheidung ist genau die, deren falsche Hälfte
beim Berichtigen eines Vertippers Dateien in den Papierkorb schiebt.

```sh
mkdir -p ~/krk-abnahme/pruefordner
cd ~/krk-abnahme/pruefordner
for n in $(seq 1 12); do printf 'wegwerf %s\n' "$n" > "aaa-wegwerf-$n.txt"; done
for n in $(seq 1 6);  do printf 'anderes %s\n' "$n" > "bbb-anderes-$n.txt"; done
mkfile 512m ~/krk-abnahme/pruefordner/grosse-datei.bin   # fuer Beobachtung 5
```

Und KRK aus einem Terminalfenster **im Vordergrund** starten, sonst weist die
Wirkungsbereichs-Prüfung jeden fokusgebundenen Befehl ab:

```sh
open /Users/k1/Projects/productive/krk/target/KRK.app   # oder per Doppelklick im Finder
```

---

## Die zehn Beobachtungen

### Ort 1 — das Fenster, wie es startet

| # | Was zu tun ist | Was zu sehen sein soll | Kriterium |
|---|---|---|---|
| 6 | KRK starten und die Bereichsleiste am Fensterfuß ansehen | Die Leiste zeigt **neun** Kästchen. `Deep` steht rechts von `Typ`, mit dem Gruppenabstand davor, und die Leiste bleibt bei ihren 18 Punkten Höhe | C2.1 |
| 10 | Das Hauptmenü öffnen und den Bereich aufsuchen, in dem die drei Spaltenschalter stehen | Dort steht **„Tiefe Suche ein- und ausschalten"**, ohne Kürzel dahinter | C5.4 |

### Ort 2 — der Prüfordner, und hier wird gelöscht

**Die Reihenfolge innerhalb dieses Blocks ist bindend.** Die ersten beiden Beobachtungen
dürfen nichts löschen; die letzten beiden löschen absichtlich. Wer sie tauscht, weiß beim
Vertipper-Fall nicht mehr, welcher Druck die Datei geholt hat.

| # | Was zu tun ist | Was zu sehen sein soll | Kriterium |
|---|---|---|---|
| **1** | In `~/krk-abnahme/pruefordner` eine der `aaa-wegwerf-*`-Dateien auswählen, `aaa` tippen und **dreimal einzeln** die Rückschritt-Taste drücken | Der Filtertext verschwindet Zeichen für Zeichen (`aaa` → `aa` → `a` → leer), die Liste wächst dabei wieder. **Alle zwölf `aaa-wegwerf-*` liegen danach noch da**, und der Papierkorb hat nichts bekommen | **C1.15** |
| **2** | Erneut eine `aaa-wegwerf-*` auswählen, `aaa` tippen und die Rückschritt-Taste **gedrückt halten**, bis der Filtertext leer ist, und noch zwei Sekunden weiter halten | Der Filtertext leert sich. **Danach geschieht nichts mehr** — die Wiederholung trägt nicht über die Grenze, und die Datei liegt noch da. Erst ein neuer, frischer Druck erreicht den Papierkorb | **C1.18** |
| 5 | Das andere Dateifenster auf einen anderen Ordner stellen. Dann `grosse` tippen, `grosse-datei.bin` auswählen und mit `F5` kopieren; **während** der Fortschritt läuft, `Esc` drücken | Der Kopiervorgang bricht ab. **Der Filtertext `grosse` steht danach noch**, denn `Esc` findet die Operation vor dem Filtertext | C1.7 |
| **3** | `aaa` tippen, einen Treffer auswählen und `cmd+delete` drücken | Die Datei geht in den Papierkorb, **und der Filtertext `aaa` steht weiter**. Der Weg zum Papierkorb ist bei stehendem Filter nie versperrt | **C1.17** |
| **4** | `Esc` drücken, damit **kein** Filtertext mehr steht. Dann mehrere `bbb-anderes-*` nacheinander auswählbar lassen und die Rückschritt-Taste **gedrückt halten** | **Mehr als eine Datei** wandert in den Papierkorb. Die Grenze aus Beobachtung 2 gilt nur für eine Wiederholung, die bei stehendem Filtertext begann | **C1.20** |

### Ort 3 — ein großer Baum, es wird nur gelesen

Ein Ordner mit mehreren tausend Einträgen unter dem Benutzerordner. `~/Library/Caches`
oder ein großer Projektbaum tun es; der Filtertext sollte etwas sein, das selten trifft,
damit der Durchlauf lange genug läuft, um ihm zuzusehen.

| # | Was zu tun ist | Was zu sehen sein soll | Kriterium |
|---|---|---|---|
| 7 | Im großen Baum einen Filtertext tippen und die Statuszeile ansehen, während die Liste sich füllt | Die Zeile sagt `Filter „…": N von M angezeigt`. **Die linke Zahl steigt mit, die rechte steht** | C4.5 |
| 8 | In einem Ordner mit mehreren tausend Einträgen `Deep` einschalten, in der Liste eine Zeile auswählen und einen Filtertext tippen | Die Zeilen kommen **nach und nach** dazu. Der Bildlauf bleibt stehen und springt nicht, und die Auswahl bleibt auf ihrem Eintrag, solange es ihn gibt | C3.11 |
| 9 | Während ein Durchlauf über einen großen Baum läuft: mit den Pfeiltasten durch die Liste gehen und mit dem Tabbefehl in einen anderen Tab wechseln | **Beides antwortet ohne Verzug.** KRK steht nicht, der Durchlauf hält den Hauptfaden nicht auf | C3.12 |

---

## Zwei Beobachtungen kann der Messmodus nicht ersetzen

**C1.18 (Beobachtung 2) und C1.20 (Beobachtung 4) sind nur am gehaltenen Finger zu sehen.**
Der Messmodus baut seine synthetischen Tastendrücke mit `isARepeat = false`
(`crates/krk-ui/src/appkit/ereignisse.rs`, `ereignis_senden`), und `behandeln` ist seit
dieser Runde die erste und einzige Lesestelle dieses Werts im Baum. Ein synthetisches
Ereignis meldet sich also nie als Wiederholung, und der Wiederholungszweig der Regel
bleibt für die Messstrecke unerreichbar.

**Das ist keine Auslassung**, sondern eine Eigenschaft der Schnittstelle, und es steht
hier, damit später niemand nach der fehlenden Messstrecke sucht. Die Regel selbst ist
vollständig an Proben abgenommen (`kommandos/rueckschritt.rs`, sieben Proben, darunter die
Tafel über alle acht Wahrheitskombinationen); ungeprüft ist allein, dass AppKit die
Wiederholung so meldet, wie die Regel es annimmt.

---

## Die 75 Kriterien im Einzelnen

Je Kriterium seine Kennzeichnung aus dem Spec und der Nachweis. Bei **(Probe)** steht der
Name der Prüfung, damit ein Leser sie nachfahren kann; die Datei davor ist ihr Ort. Bei
**(Bündel)** steht die Nummer der Beobachtung von oben. Neun Kriterien tragen keinen
eigenen Probennamen — sie stehen unten unter „Was beim Zusammentragen aufgefallen ist"
und sind hier als solche gekennzeichnet.

Nachzufahren ist eine einzelne Probe mit
`cargo test -p krk-core <name>` beziehungsweise `cargo test -p krk-ui --bin krk <name>`
(`cargo` liegt unter `$HOME/.cargo/bin`).

### C1 — Tippen filtert die sichtbare Liste des Tabs

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C1.1 | Probe | `krk-ui/src/kommandos/navigation.rs::eine_sichtbare_auswahl_bleibt_stehen`, `…::ohne_bestehende_auswahl_entsteht_keine`; die Modellseite `krk-core/tests/verzeichnis.rs::ein_zeichen_zurueck_laesst_die_liste_wieder_wachsen` (ruft `zeichen_anhaengen`) |
| C1.2 | Probe | `krk-core/src/verzeichnis/filter.rs::die_folge_zaehlt_an_jeder_stelle_des_namens`, `…::die_schreibung_des_namens_spielt_keine_rolle`, `krk-core/tests/verzeichnis.rs::der_filter_nimmt_die_teilzeichenfolge_an_jeder_stelle_und_in_jeder_schreibung` |
| C1.3 | Probe | `krk-core/src/verzeichnis/filter.rs::der_vergleich_faltet_keine_umlaute_und_keine_akzente`, `krk-core/tests/verzeichnis.rs::der_filter_faltet_keine_umlaute` |
| C1.4 | Probe | `krk-core/tests/verzeichnis.rs::die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer` |
| C1.5 | Probe | `krk-core/tests/verzeichnis.rs::im_filter_steht_keine_zeitmessung` |
| C1.6 | Probe | `krk-core/tests/verzeichnis.rs::bei_flacher_suche_bleibt_jeder_ordner_stehen`, `…::eine_verknuepfung_zaehlt_fuer_die_sichtbarkeit_als_ordner` |
| **C1.7** | Probe + **Bündel** | **Die Probenhälfte fehlt** (Befund 2 unten, Datensatz `issues/260815-0020_o_c1-7-verlangt-eine-probe-fuer-die-reihenfolge-von-esc-und-b2-hat-keinen-ort-dafuer.md`). Bündel: **Beobachtung 5** |
| C1.8 | Probe | `krk-ui/src/tabs.rs::der_filtertext_gehoert_dem_tab_und_nicht_dem_fenster` |
| C1.9 | Probe | `krk-ui/src/tabs.rs::ein_ordnerwechsel_leert_den_filtertext_wenn_die_tiefe_suche_aus_ist` |
| C1.10 | Probe | `krk-ui/src/tabs.rs::mit_tiefer_suche_ueberlebt_der_filtertext_den_ordnerwechsel`, `…::die_tiefe_suche_geht_auch_ohne_filtertext_hinueber` |
| C1.11 | Probe | `krk-core/tests/verzeichnis.rs::eine_ausgefilterte_auswahl_kommt_beim_leeren_des_filters_zurueck`, `krk-ui/src/kommandos/navigation.rs::eine_weggefallene_auswahl_geht_auf_die_erste_zeile`, `…::eine_leere_sicht_bekommt_keine_auswahl`. **Am 260815-0246 umformuliert** (Befund 9): die Meldung „es ist nichts ausgewählt" bei leerer Auswahl ist Verhalten der Runde 1 und trägt keine Probe dieser Runde |
| C1.12 | Probe | `krk-core/tests/verzeichnis.rs::die_sprungmarke_steht_nirgends_mehr_im_baum` |
| C1.13 | Probe | **Keine eigene Probe** (Befund 4). Belegt am Bestand: `resources/default-keymap.toml` trägt keinen Einstiegsbefehl für den Filter, `grep -c '^\[\[funktion\]\]'` gibt 84, und der einzige neue Eintrag ist `tiefe_suche_umschalten` |
| C1.14 | Probe | `krk-core/tests/verzeichnis.rs::ein_zeichen_zurueck_laesst_die_liste_wieder_wachsen`, `krk-ui/src/kommandos/rueckschritt.rs::ein_stehender_filtertext_nimmt_ein_zeichen_zurueck` |
| **C1.15** | Probe + **Bündel** | `krk-ui/src/kommandos/rueckschritt.rs::ein_stehender_filtertext_nimmt_ein_zeichen_zurueck`, `…::die_tafel_aus_acht_faellen_geht_auf`. Bündel: **Beobachtung 1** — sicherheitsrelevant |
| C1.16 | Probe | `krk-ui/src/kommandos/rueckschritt.rs::ohne_filtertext_raeumt_ein_frischer_druck` |
| **C1.17** | Probe + **Bündel** | `krk-ui/src/appkit/ereignisse.rs::nur_die_nackte_ruecktaste_gilt_als_rueckschritt`. Bündel: **Beobachtung 3** — sicherheitsrelevant |
| **C1.18** | Probe + **Bündel** | `krk-ui/src/kommandos/rueckschritt.rs::eine_wiederholung_bei_stehendem_filtertext_traegt_nicht_ueber_die_grenze`, `…::der_merker_ueberlebt_das_leerwerden_des_filtertextes`. Bündel: **Beobachtung 2** — sicherheitsrelevant, und der Messmodus kann sie nicht fahren |
| C1.19 | Probe | `krk-ui/src/menuemodell.rs::jede_funktion_der_belegung_steht_genau_einmal_im_menue` (die Rückschritt-Taste hängt weiter an der einen Funktion `in_papierkorb`), `krk-ui/src/kommandos/rueckschritt.rs::die_regel_hat_genau_einen_aufrufer` |
| **C1.20** | Probe + **Bündel** | `krk-ui/src/kommandos/rueckschritt.rs::eine_wiederholung_ohne_je_stehenden_filtertext_raeumt_weiter`. Bündel: **Beobachtung 4** — sicherheitsrelevant, und der Messmodus kann sie nicht fahren |

### C2 — Das Ankreuzfeld „Deep" und der gefilterte Ordnerbaum

| # | Kennzeichnung | Nachweis |
|---|---|---|
| **C2.1** | Probe + **Bündel** | `krk-ui/src/appkit/bereichsleiste.rs::die_leiste_traegt_neun_schalter`, `…::der_neunte_schalter_heisst_deep_und_steht_rechts_von_typ`. Bündel: **Beobachtung 6** |
| C2.2 | Probe | `krk-ui/src/appkit/bereichsleiste.rs::der_neunte_schalter_gibt_fokus_keinen_sechsten_wert` (Befund 6: der Schritt E3 sieht daneben einen Bündelanteil, den der Spec nicht kennzeichnet) |
| C2.3 | Probe | `krk-ui/src/appkit/bereichsleiste.rs::jeder_schalter_nennt_genau_ein_eigenes_kommando` |
| C2.4 | Probe | `krk-core/tests/verzeichnis.rs::ohne_filtertext_aendert_die_tiefe_suche_nichts`, `krk-ui/src/appkit/bereichsleiste.rs::jeder_schalter_wirkt_aus_jedem_fokus` |
| C2.5 | Probe | `krk-core/tests/verzeichnis.rs::bei_tiefer_suche_entscheidet_name_oder_befund`, `…::ein_namentlich_passender_ordner_steht_auch_ohne_treffer_darunter` |
| C2.6 | Probe | `krk-core/tests/verzeichnis.rs::bei_tiefer_suche_entscheidet_name_oder_befund` |
| C2.7 | Probe | **Keine eigene Probe** (Befund 5). Belegt daran, dass `Ordnermodell::sichtbar` der eine Prüfschritt jeder Ebene ist (`filter_und_verstecke_gehen_durch_denselben_pruefschritt`) und der Filtertext den Einstieg übersteht (`mit_tiefer_suche_ueberlebt_der_filtertext_den_ordnerwechsel`) |
| C2.8 | Probe | `krk-core/tests/verzeichnis.rs::ein_namentlich_passender_ordner_steht_auch_ohne_treffer_darunter`, `krk-ui/src/tabs.rs::die_auftragsliste_laesst_namentlich_passende_ordner_aus`; die Zahl `0 von N` aus `krk-ui/src/appkit/statuszeile.rs::der_satz_nennt_filtertext_gezeigte_und_vorhandene` |
| C2.9 | Probe | `krk-ui/src/tabs.rs::die_dateiliste_bleibt_flach_und_hat_vier_spalten` |
| C2.10 | Probe | `krk-ui/src/tabs.rs::eine_zeile_aus_einem_tiefen_treffer_liegt_im_angezeigten_ordner` |
| C2.11 | Probe | `krk-ui/src/tabs.rs::die_angezeigte_datei_bleibt_bei_zwei_quellen` |
| C2.12 | Probe | `krk-core/tests/verzeichnis.rs::die_eingestellte_sortierung_bleibt_die_ordnung_der_gefilterten_liste` |
| C2.13 | Probe | `krk-core/tests/verzeichnis.rs::eine_verknuepfung_zaehlt_fuer_die_sichtbarkeit_als_ordner`, `…::eine_verknuepfung_auf_einen_ordner_meldet_kein_treffer`, `krk-ui/src/tabs.rs::eine_verknuepfung_bekommt_einen_auftrag_wie_jeder_ordner` |
| **C2.14** | Probe | **Neu am 260815-0246** (Befund 10). Modellseite: `krk-core/tests/verzeichnis.rs::eine_ausgefilterte_auswahl_kommt_beim_leeren_des_filters_zurueck` (der Eintrag bleibt gemerkt, während seine Zeile fehlt), `…::der_befund_faellt_bei_jeder_aenderung_der_frage_zurueck` (das Einschalten von „Deep" setzt die Befunde zurück). Die Oberflächenseite hat **keine eigene Probe**: `Tabellenansicht::tiefe_suche_umschalten` ruft `umsortiert` und `meldung_gewechselt` und nicht `nach_filteraenderung`, nachzulesen in `krk-ui/src/appkit/tabelle.rs` |

### C3 — Der Durchlauf über den Unterbaum

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C3.1 | Probe | `krk-core/tests/verzeichnis.rs::der_durchlauf_liest_ueber_den_schwungleser_und_setzt_keine_grenze` |
| C3.2 | Probe | `krk-ui/src/tabs.rs::dateien_und_passende_ordner_warten_nicht_auf_den_durchlauf` |
| C3.3 | Probe | `krk-core/tests/verzeichnis.rs::ein_treffer_tief_unten_entscheidet_den_ordner`, `krk-ui/src/tabs.rs::der_tab_zieht_die_befunde_ein_und_die_zeile_des_tiefen_treffers_erscheint` |
| C3.4 | Probe | `krk-core/tests/verzeichnis.rs::der_abbruch_greift_in_einem_ordner_ohne_unterordner` — **zur Hälfte gemessen** (Befund 7) |
| C3.5 | Probe | `krk-ui/src/tabs.rs::ohne_seine_drei_bedingungen_beginnt_kein_durchlauf`; der dritte Rang von `Esc` steht in `krk-ui/src/appkit/anwendung.rs::abbrechen` |
| C3.6 | Probe | `krk-ui/src/tabs.rs::je_tab_laeuft_nie_mehr_als_ein_durchlauf`, `…::ein_weiteres_zeichen_loest_den_laufenden_durchlauf_ab` |
| C3.7 | Probe | `krk-ui/src/tabs.rs::das_ausschalten_von_deep_bricht_den_durchlauf_ab`, `…::ohne_seine_drei_bedingungen_beginnt_kein_durchlauf` |
| C3.8 | Probe | `krk-core/tests/verzeichnis.rs::der_durchlauf_kennt_keine_tiefengrenze`, `…::der_durchlauf_liest_ueber_den_schwungleser_und_setzt_keine_grenze`, **und seit dem 260815** `…::die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden` für den zweiten Satz des Kriteriums |
| C3.9 | Probe | `krk-core/tests/verzeichnis.rs::eine_verknuepfung_auf_einen_ordner_meldet_kein_treffer` |
| C3.10 | Probe | `krk-core/tests/verzeichnis.rs::ein_nicht_lesbarer_ordner_gilt_als_kein_treffer` |
| **C3.11** | **Bündel** | **Beobachtung 8** — kein Probenanteil |
| **C3.12** | **Bündel** | **Beobachtung 9** — kein Probenanteil |
| C3.13 | Probe | `krk-core/tests/verzeichnis.rs::jeder_auftrag_bekommt_genau_einen_befund`, `…::ein_ordner_ohne_treffer_meldet_den_negativen_befund`; die Abbruchursache von „nicht entschieden" über `…::der_abbruch_greift_in_einem_ordner_ohne_unterordner` |
| C3.14 | Probe | `krk-ui/src/tabs.rs::die_auftragsliste_laesst_namentlich_passende_ordner_aus`, `…::ein_ordner_mit_lauter_passenden_unterordnern_stoesst_null_durchlaeufe_an` |
| **C3.15** | Probe | **Neu am 260815-0246, und zur Hälfte gemessen** (Befund 11). Gemessen ist, dass der Durchlauf keinen eigenen Deskriptormangel erzeugt: `krk-core/tests/verzeichnis.rs::die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden`. **Ungemessen** ist, dass ein von außen herbeigeführter Mangel zu keinem Befund führt; die Trennung selbst steht in `krk-core/src/verzeichnis/sys.rs::ist_deskriptormangel` und hat keine Probe |

### C4 — Die eine Statuszeile trägt den Filter

Alle acht Proben stehen in `krk-ui/src/appkit/statuszeile.rs`.

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C4.1 | Probe | `der_filterstand_steht_zwischen_tabmeldung_und_markierungsstand` |
| C4.2 | Probe | `der_filterstand_gilt_nicht_als_fehler` |
| C4.3 | Probe | `der_satz_nennt_filtertext_gezeigte_und_vorhandene` |
| C4.4 | Probe | `ausgeblendete_markierungen_stehen_daneben_und_sonst_nicht` |
| **C4.5** | Probe + **Bündel** | `die_linke_zahl_waechst_und_zaehlt_zeilen_und_keine_treffer` für die Rechnung. Bündel: **Beobachtung 7** |
| C4.6 | Probe | `die_linke_zahl_waechst_und_zaehlt_zeilen_und_keine_treffer` |
| C4.7 | Probe | `waehrend_der_ersatz_aussteht_nennt_der_rang_nichts` |
| C4.8 | Probe | `ohne_filtertext_meldet_der_rang_nichts` |
| C4.9 | Probe | **Keine Probe** (Befund 1). Belegt durch Ansehen: der Diff legt keine Ansicht an, und der Filtertext erreicht genau einen Empfänger, das Feld `Quellen::filterstand` |
| C4.10 | Probe | `jeder_der_sechs_raenge_hat_genau_ein_feld` |

### C5 — Der Befehl für „Deep" in Belegung, Hauptmenü und Belegungsansicht

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C5.1 | Probe | `krk-core/tests/belegung.rs::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, `…::jedes_kommando_traegt_genau_einen_wirkungsbereich` |
| C5.2 | Probe | `krk-core/tests/belegung.rs::jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste`, `krk-core/src/tasten/belegung.rs::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` |
| C5.3 | Probe | `krk-core/tests/belegung.rs::jedes_kommando_traegt_genau_einen_wirkungsbereich`, `krk-ui/src/belegungsmodell.rs::jede_kennung_hat_einen_funktionsbereich` |
| **C5.4** | Probe + **Bündel** | `krk-ui/src/menuemodell.rs::jede_funktion_der_belegung_steht_genau_einmal_im_menue`. Bündel: **Beobachtung 10** |
| C5.5 | Probe | **Am 260815-0246 an den Baum gezogen** (Befund 3, damit erledigt). Die Belegungsansicht führt ihn: `krk-ui/src/belegungsmodell.rs::jede_kennung_hat_einen_funktionsbereich`. Dass die Markdown-Ausgabe eine ab Werk unbelegte Funktion **nicht** führt, hält `krk-ui/src/belegungsausgabe.rs::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` fest — und genau das sagt das Kriterium jetzt auch |
| C5.6 | Probe | **Keine eigene Probe** (Befund 5). Belegt am Baum: eigener Zweig `Kommando::TiefeSucheUmschalten` in `krk-ui/src/appkit/anwendung.rs:2938`, vor dem Auffangzweig auf `bereichskommando` |
| C5.7 | Probe | `krk-core/tests/belegung.rs::jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` über `OHNE_KOMBINATION_AB_WERK` (jetzt vier Einträge), `krk-ui/src/belegungsausgabe.rs::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` |

### C6 — Markierung und Dateioperationen unter dem Filter

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C6.1 | Probe | `krk-ui/src/tabs.rs::eine_zeile_aus_einem_tiefen_treffer_liegt_im_angezeigten_ordner` |
| C6.2 | Probe | `krk-core/tests/verzeichnis.rs::die_markierung_besteht_unter_dem_filter_fort_und_wirkt_wieder` |
| C6.3 | Probe | `krk-core/tests/verzeichnis.rs::die_markierbefehle_behalten_ihren_zuschnitt_unter_dem_filter` |
| C6.4 | Probe | `krk-core/tests/verzeichnis.rs::die_markierbefehle_behalten_ihren_zuschnitt_unter_dem_filter` |
| C6.5 | Probe | `krk-core/tests/verzeichnis.rs::die_markierung_besteht_unter_dem_filter_fort_und_wirkt_wieder` |
| C6.6 | Probe | **Keine eigene Probe** (Befund 5). Gedeckt von der Probe zu C6.2 und C6.5, die beide Richtungen mitführt |
| C6.7 | Probe | **Keine eigene Probe** (Befund 5). Belegt daran, dass das Stapelumbenennen in dieser Runde keine Zeile bekommen hat: `git diff 0c3a6f4 HEAD -- crates/krk-core/src/stapelumbenennen/` ist leer |
| C6.8 | Probe | `krk-core/tests/verzeichnis.rs::filter_und_verstecke_gehen_durch_denselben_pruefschritt` |
| C6.9 | Probe | `krk-ui/src/kommandos/rueckschritt.rs::ein_stehender_filtertext_nimmt_ein_zeichen_zurueck`, `…::die_tafel_aus_acht_faellen_geht_auf` |
| C6.10 | Probe | `krk-ui/src/kommandos/rueckschritt.rs::die_regel_hat_genau_einen_aufrufer`, `…::die_tafel_aus_acht_faellen_geht_auf`. **Der Vorbehalt aus Befund 8 ist am 260815-0246 im Spec aufgelöst**: das Kriterium sagt jetzt selbst, dass die zweite Größe als zwei Wahrheitswerte dasteht |
| C6.11 | Probe | `krk-ui/src/appkit/ereignisse.rs::nur_die_nackte_ruecktaste_gilt_als_rueckschritt`; `f8` und `opt+cmd+delete` tragen `Kommando::EndgueltigLoeschen`, `ctrl+delete` geht durch `Leistenquelle::kommando_ausfuehren` und erreicht die Regel nicht |

---

## Was beim Zusammentragen aufgefallen ist

Elf Befunde. Keiner hält den Abnahmelauf auf; vier betreffen die Frage, ob ein Kriterium
so nachgewiesen ist, wie sein eigenes Kennzeichen es behauptet.

**Die Befunde 9 bis 11 sind am 260815-0246 dazugekommen**, mit der Spec-Berichtigung
derselben Uhrzeit. Die Befunde 3 und 8 sind mit ihr erledigt und unten als solche
gekennzeichnet.

**1. C4.9 trägt (Probe), und eine Probe gibt es nicht.** Der Spec sagt „Probe über die
Zahl der Anzeigen". Der Schritt D1 hat sie nicht gebaut und sagt auch warum: sie müsste
die Abwesenheit einer Ansicht prüfen. Belegt ist es durch Ansehen des Diffs. Das ist ein
tragfähiger Beleg, aber es ist kein Probenname, und die Kennzeichnung im Spec verspricht
einen. Nachzuziehen ist der Spec oder die Probe, und die Entscheidung gehört nicht in
diese Liste.

**2. Die Probenhälfte von C1.7 hat keinen Ort.** Offener Datensatz
`issues/260815-0020_o_c1-7-verlangt-eine-probe-fuer-die-reihenfolge-von-esc-und-b2-hat-keinen-ort-dafuer.md`.
Die Rangfolge von `Esc` hängt an drei Ivars des Anwendungsdelegierten, und `krk-ui` hat
kein Bibliotheksziel. **C1.7 ist damit in der Sache ein reines Bündelkriterium**, obwohl
es zwei Kennzeichnungen trägt; Beobachtung 5 ist sein einziger Nachweis.

**3. C5.5 und C5.7 widersprachen sich. Erledigt am 260815-0246.** Datensatz
`issues/260814-2320_c_c5-5-verlangt-den-eintrag-in-der-markdown-ausgabe-und-c5-7-schliesst-ihn-daraus-aus.md`,
geschlossen. C5.5 verlangte den Eintrag in der Markdown-Ausgabe, C5.7 liefert keine
Kombination aus, und die Markdown-Ausgabe nimmt seit dem Nutzerentscheid vom 260811-0110
nur Funktionen auf, die mindestens eine Kombination tragen. C5.5 sagt jetzt, was der Baum
tut: der Eintrag steht dort nicht, solange er ab Werk keine Kombination trägt, und er
steht dort, sobald der Nutzer ihm eine zuweist.

**4. C1.13 trägt (Probe über das Fehlen eines Einstiegsbefehls), und diese Probe gibt es
nicht.** Der Nachweis ist von Hand zu führen, mit einem Blick in
`resources/default-keymap.toml`. Er hält — der einzige neue Eintrag der Runde ist
`tiefe_suche_umschalten` —, aber er hält nicht von selbst weiter.

**5. Vier weitere Kriterien haben keinen eigenen Probennamen:** C2.7, C5.6, C6.6 und C6.7.
Alle vier sind Zusagen darüber, dass etwas **unverändert** bleibt oder aus einer anderen
Probe folgt, und für alle vier steht oben, woran es hängt. Sie sind damit belegt, aber ein
Leser kann sie nicht nachfahren, sondern nur nachlesen.

**6. Der Schritt E3 sieht bei C2.2 und C2.3 einen Bündelanteil, den der Spec nicht
kennzeichnet.** Sein Bericht sagt, `setRefusesFirstResponder` lasse sich ohne AppKit nicht
nachsehen und das Aufblitzen der Selbstkippung ebenso wenig. Der Spec kennzeichnet beide
allein als **(Probe)**, und diese Liste folgt dem Spec: die Zahl der Bündelkriterien
bleibt **zehn**. Wer die zwei halben Anteile mitnehmen will, sieht sie bei Beobachtung 6
nebenbei — der neunte Schalter nimmt beim Klicken keinen Fokusrahmen an, und sein Kästchen
kippt nicht sichtbar von selbst.

**7. C3.4 ist zur Hälfte gemessen.** Der Schritt F1 sagt es selbst: der Durchlauf meldet je
Auftrag genau einmal, also kann keine Probe sich mit dem Fortschritt **innerhalb** eines
Ordners verabreden. Geprüft ist die Aussage, für die das Kriterium den Prüfordner ohne
Unterordner ausdrücklich verlangt — der Abbruch hängt nicht am Absteigen. Ungeprüft
bleibt „an **jeder** Stapelgrenze".

**8. C6.10 nannte zwei Größen, die Signatur trägt drei. Erledigt am 260815-0246.**
Datensatz
`issues/260814-2254_c_c6-10-sagt-zwei-groessen-und-keine-dritte-die-signatur-traegt-drei-wahrheitswerte.md`,
geschlossen. Die dritte ist der Merker, ob die laufende Wiederholung bei stehendem
Filtertext begann; sachlich ist sie die zweite Hälfte derselben Größe, die C1.18 und C1.20
aufspannen. Die Probe `die_tafel_aus_acht_faellen_geht_auf` schreibt alle acht
Kombinationen aus, deckt also mehr ab, als das Kriterium verlangt. Es war kein Widerspruch
am Verhalten, sondern einer an der Formulierung, und C6.10 trägt den fehlenden Halbsatz
jetzt.

**9. C1.11 sagte „meldet nichts", und jeder Operationsbefehl meldet „es ist nichts
ausgewählt". Erledigt am 260815-0246.** Datensatz
`issues/260815-0211_c_c1-11-sagt-meldet-nichts-und-jeder-operationsbefehl-meldet-es-ist-nichts-ausgewaehlt.md`,
geschlossen. `Anwendungsdelegierter::auftrag_stellen` antwortet seit der Runde 1 auf eine
leere Auswahl mit dieser Meldung, und alle vier Operationsbefehle gehen dort durch. Das
Kriterium ist an den Baum gezogen worden und nicht umgekehrt: die Meldung ist in der Lage,
in der die Liste leer vor dem Nutzer steht, seine einzige Auskunft darüber, warum sein
Tastendruck nichts getan hat. **Die Meldung selbst trägt keine Probe dieser Runde**, weil
sie kein Zweig dieser Runde ist.

**10. Die Ersatzzeile aus C1.11 greift beim Tippen und nicht beim Umschalten von „Deep".
Erledigt am 260815-0246, als neues Kriterium C2.14.** Datensatz
`issues/260815-0211_c_die-ersatzzeile-aus-c1-11-greift-beim-tippen-und-nicht-beim-umschalten-von-deep.md`,
geschlossen. Von den vier Stellen, die ändern, was der Filter übrig lässt, nehmen drei den
Weg über `nach_filteraenderung` und die vierte, `tiefe_suche_umschalten`, nicht. **Das ist
so richtig**: beim Tippen fällt eine Zeile endgültig weg, beim Umschalten nur so lange,
bis der Befund für ihren Ordner eintrifft. Die Ersatzzeile würde in diesem Fall den Platz
des Nutzers verlieren, obwohl der Eintrag gleich darauf wiederkommt. Die Ungleichheit der
vier Wege ist damit eine Entscheidung und kein Versehen. **Der Preis steht in C2.14**: in
der Spanne dazwischen ist keine Zeile ausgewählt, und ein Operationsbefehl meldet „es ist
nichts ausgewählt".

**11. C3.15 trägt (Probe), und die Probe deckt nur die Rückrichtung ab.** Neu am
260815-0246 mit dem Kriterium selbst. Gemessen ist, dass der Durchlauf keinen eigenen
Deskriptormangel erzeugt: die Kindprobe unter 64 Deskriptoren entscheidet eine Kette von
zweihundert Ebenen. **Ungemessen ist die andere Hälfte** — dass ein von außen
herbeigeführter Mangel zu keinem Befund führt statt zu einem negativen.
`krk_core::verzeichnis::sys::ist_deskriptormangel` hat keine eigene Probe, und der Zweig
`Err(fehler) if ist_deskriptormangel(&fehler) => return None` in
`crates/krk-core/src/verzeichnis/durchlauf.rs` wird von keiner Prüfung erreicht. **Das ist
Coder-Arbeit und keine Sache dieser Liste**; die Kindprobe mit `ulimit -n 64` steht als
Muster bereits daneben.

**Kein Coder-Bericht widerspricht der Zahl zehn.** Die Summe der Bündelanteile über alle
dreizehn Berichte ist dieselbe Menge, die die Kennzeichnung im Spec liefert: C1.7, C1.15,
C1.17, C1.18, C1.20, C2.1, C3.11, C3.12, C4.5, C5.4.

---

## Danach

Das beglaubigte `v0.3.0` unter `target/KRK.app` ist durch den Entwicklungsbau ersetzt. Wer
es zurück will, kopiert es aus `~/Library/Caches/krk-beglaubigt-260814-1054/` oder fährt
`cargo xtask release` neu; Letzteres verlangt den Tag `v0.3.0` auf HEAD und einen
unveränderten verfolgten Baum.

Der Prüfordner ist nach dem Durchgang entbehrlich:

```sh
rm -rf ~/krk-abnahme
```

---

## Ergebnisse

*Hier trägt der Nutzer ein, was er gesehen hat. Die Runde schließt kohärent (`_c_`), wenn
alle zehn Beobachtungen bestanden sind; sonst beschränkt (`_b_`) mit dem Befund.*

| # | Kriterium | Ergebnis |
|---|---|---|
| 1 | C1.15 | |
| 2 | C1.18 | |
| 3 | C1.17 | |
| 4 | C1.20 | |
| 5 | C1.7 | |
| 6 | C2.1 | |
| 7 | C4.5 | |
| 8 | C3.11 | |
| 9 | C3.12 | |
| 10 | C5.4 | |

estimated effort (ai-based): about 25 min
