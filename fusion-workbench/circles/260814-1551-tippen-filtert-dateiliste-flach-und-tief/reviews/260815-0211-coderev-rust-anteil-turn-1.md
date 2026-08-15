# Durchsicht des Rust-Anteils von Turn 1

**Sender:** coderev
**Reviewed-range:** `ba4af5f..05881d1`
**Not-opened:** `fusion-workbench/circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, `fusion-workbench/circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/*` (achtzehn Datensätze der Schritte A1 bis G2, darunter die Abnahmeliste `260815-0400-abnahmeliste-g2.md`), `fusion-workbench/circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/*` (vierzehn Datensätze, nur die Dateinamen gelesen), `fusion-workbench/circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/_t_circle.md`, `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/shared/history/260814-1500-orchestrator-session.md`
**Datum:** 260815-0211
**Circle:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`
**Grundlage:** Spec `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, 75 Abnahmekriterien

Der ganze Rust-Anteil des Bereichs ist geöffnet, jede Datei unter `crates/` und `resources/`, die der Bereich anfasst. Nicht geöffnet sind die Werkstattdatensätze der Runde; der Auftrag hat die Durchsicht auf den Rust-Anteil geschnitten, und die zehn offenen Defekte und fünf Fragen des Circle sind über ihre Titel und ihre Befundabschnitte gelesen, damit hier nichts doppelt liegt.

`make check` fährt auf `05881d1` grün: Bau, Proben, `cargo fmt --all --check` und `cargo clippy --workspace --all-targets -- -D warnings`.

---

## Zusammenfassung

Ein Befund ist **hoch**, zwei sind **mittel**, vier sind **niedrig**. Kein Befund ist kritisch.

Der hohe betrifft den Durchlauf: ein Deskriptormangel des Prozesses wird dort zu einem dauerhaften „kein Treffer darunter", und die Zeile eines Ordners, unter dem ein Treffer liegt, fällt ohne Meldung aus der Liste. Er ist nachgestellt und nicht erschlossen. Die beiden mittleren sind Abweichungen zwischen C1.11 und dem Baum. Die vier niedrigen sind sämtlich vom Typ, den der Auftrag benannt hat: eine Probe sagt mehr zu, als sie hält.

**Der Bau selbst ist von hoher Güte.** Die Fallunterscheidung der Rückschritt-Taste ist als reine Funktion mit ausgeschriebener Tafel gebaut und in der gefährlichen Richtung dicht; der Durchlauf steigt nachweislich nicht in symbolische Verknüpfungen hinab und endet auf einem Verknüpfungskreis mit dem richtigen Befund; der Prüfschritt der Sichtbarkeit steht an genau einer Stelle; der Vergleich und die Zeichenregel stehen je einmal. Die Zählproben dieser Runde sind überwiegend am Baum und nicht an einer Datei gefasst und benennen ihre Blindheit im Doc-Kommentar.

---

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 2 |
| Niedrig | 4 |

Alle sieben liegen als eigene Datensätze unter `issues/` dieses Circle, abgelegt am 260815-0211.

---

## Befunde nach Themen

### Ressourcengrenzen im Durchlauf

**H1 — Ein Deskriptormangel des Prozesses wird zu einem dauerhaften „kein Treffer darunter" (hoch).**
`crates/krk-core/src/verzeichnis/durchlauf.rs:226` und `:250`. Jeder Fehlschlag von `Schwungleser::oeffnen` gilt als Aussage über den Ordner; `EMFILE` und `ENFILE` sind aber ein Zustand des Prozesses, und der Durchlauf erzeugt ihn selbst, weil er einen offenen Deskriptor je Ebene hält. Nachgestellt an einem 400 Ebenen tiefen Prüfbaum mit dem Treffer ganz unten: mit der Deskriptorgrenze der Sitzung meldet der Durchlauf `treffer: true`, unter `ulimit -n 64` meldet er `treffer: false`. Derselbe Baum, dieselbe Frage, zwei Antworten, keine Meldung. Die Probe `der_durchlauf_kennt_keine_tiefengrenze` sieht es nicht, weil `cargo test` die angehobene Grenze der Anmeldesitzung erbt, während `launchctl limit maxfiles` auf diesem Gerät 256 als Voreinstellung führt. Erreichbar ist der Fehler erst bei rund 250 Ebenen und damit selten; sichtbar ist er nie.
Datensatz: `issues/260815-0211_o_ein-deskriptormangel-des-prozesses-wird-zu-einem-dauerhaften-kein-treffer-darunter.md`

**Was daneben hält und geprüft ist.** Der Abstieg läuft über einen eigenen Stapel von Ebenen und nicht über die Rekursion des Fadens (C3.8); ein 400 Ebenen tiefer Baum läuft durch. In eine symbolische Verknüpfung wird nicht abgestiegen, und zwar nicht über eine Besuchtliste, sondern über den Typ: `getattrlistbulk` meldet `VLNK` als `Typ::Verknuepfung` (`verzeichnis/sys.rs:338-344`), der Abstiegszweig verlangt `Typ::Ordner`. Ein Prüfordner mit einer Verknüpfung auf sich selbst endet mit `treffer: false` statt in einem Kreis (C3.9). `.` und `..` liefert `getattrlistbulk` nicht, es gibt also auch von dieser Seite keinen Kreis. Das Abbruchkennzeichen wird ausschließlich an der Stapelgrenze gelesen und nie beim Absteigen (C3.4, zweite Hälfte); der Abstand zwischen zwei Prüfungen ist nach oben durch einen Stapel begrenzt, weil eine frisch aufgelegte Ebene mit leerem Stapel beginnt und damit sofort an der Grenze steht.

### Die Rückschritt-Taste und der Merker

**Kein Befund.** Die Regel steht als reine Funktion in `crates/krk-ui/src/kommandos/rueckschritt.rs` und wird an genau einer Stelle gerufen; die Tafel deckt alle acht Wahrheitskombinationen ohne Auffangzweig ab. Der gefährliche Zweig ist dicht: bei stehendem Filtertext gibt es keinen Weg zu `in_papierkorb`, und `betroffene` wird für diesen Tastendruck nicht befragt (C1.15, C6.9). `cmd+delete` fällt an `Anschlag::ist_nackter_rueckschritt` heraus und räumt in jeder Lage (C1.17).

**Der Merker ist auf die richtige Seite geneigt, und das ist der Punkt.** Er wohnt als `Cell<bool>` in den Ivars des Anwendungsdelegierten und wird von jeder anderen Eingabe am Kopf von `eingabe_ausfuehren` zurückgesetzt (`appkit/anwendung.rs:2645-2663`). Ein stehengebliebenes `true` wäre die harmlose Richtung: der erste Anschlag einer neuen Wiederholung trägt `isARepeat = false`, und die zweite Zeile der Tafel setzt den Merker dabei unbedingt auf `false` zurück. Die gefährliche Richtung wäre ein fälschlich auf `false` stehender Merker mitten in einer Wiederholung, die bei stehendem Filtertext begann — dafür müsste zwischen zwei Anschlägen derselben Wiederholung eine andere Eingabe die Senke erreichen. Ich habe dafür keinen Weg gefunden: ein Zusatztastenwechsel während der Wiederholung fällt im Nachschlag auf `Nachschlag::Unbelegt` und ruft die Senke gar nicht, und ein Klick auf die Bereichsleiste geht über `kommando_ausfuehren` und nicht über `eingabe_ausfuehren`, lässt den Merker also unangetastet. Das ist eine Prüfung durch Lesen des Weges und keine Messung am Bündel; C1.18 und C1.20 sind am laufenden Bündel abzunehmen, weil der Messmodus seine Ereignisse mit `isARepeat = false` baut.

### Der Spec und der Baum bei C1.11

**M1 — C1.11 sagt „meldet nichts", und jeder Operationsbefehl meldet „es ist nichts ausgewählt" (mittel).**
`appkit/anwendung.rs:4914-4925`. Bei einem Filtertext ohne Treffer ist `betroffene` leer, und `auftrag_stellen` setzt darauf eine Befehlsantwort. C1.11 zweiter Satz sagt „tut nichts und meldet nichts" und trägt **(Probe)**; eine Probe dafür gibt es nicht. Wahrscheinlich ist das Kriterium zu berichtigen und nicht der Baum — die Meldung ist die einzige Auskunft, die der Nutzer in dieser Lage bekommt.
Datensatz: `issues/260815-0211_o_c1-11-sagt-meldet-nichts-und-jeder-operationsbefehl-meldet-es-ist-nichts-ausgewaehlt.md`

**M2 — Die Ersatzzeile aus C1.11 greift beim Tippen und nicht beim Umschalten von „Deep" (mittel).**
`appkit/tabelle.rs:1711-1726`. Drei der vier Stellen, die ändern, was der Filter übrig lässt, gehen durch `nach_filteraenderung` und lassen damit `ersatzzeile` laufen; `tiefe_suche_umschalten` geht durch `umsortiert` und tut es nicht. Wer „Deep" bei stehendem Filtertext einschaltet, während die Auswahl auf einem Ordner ohne Treffer darunter steht, hat danach keine sichtbare Auswahl statt der ersten sichtbaren Zeile. Die Auswahl des Modells bleibt dabei erhalten und kommt mit dem Befund zurück; die Spanne dazwischen ist der Befund.
Datensatz: `issues/260815-0211_o_die-ersatzzeile-aus-c1-11-greift-beim-tippen-und-nicht-beim-umschalten-von-deep.md`

### Proben, die mehr zusagen als sie halten

Der Auftrag hat diesen Typ eigens benannt, und er kommt viermal vor. Keiner der vier ist schwer, und alle vier haben dieselbe Form: der Doc-Kommentar spricht über den Gegenstand, die Zählung fasst eine Schreibweise oder eine Datei.

**N1 — Die Abbruchprobe bricht vor dem ersten Stapel ab (niedrig).**
`crates/krk-core/tests/verzeichnis.rs`, `der_abbruch_greift_in_einem_ordner_ohne_unterordner`. `abbrechen()` steht unmittelbar hinter `starten()`; der Arbeitsfaden holt keinen einzigen Stapel. Die 5.000 Dateien und die Zusicherung `> 2 * STAPELGROESSE` spielen im Ablauf keine Rolle, lesen sich aber wie der Beleg für die Zahl „zwei" aus C3.4. Die zweite Hälfte von C3.4 — die Prüfung hängt nicht am Absteigen — hält die Probe dagegen wirklich.
Datensatz: `issues/260815-0211_o_die-abbruchprobe-bricht-vor-dem-ersten-stapel-ab-und-misst-die-zwei-stapel-grenze-nicht.md`

**N2 — `im_filter_steht_keine_zeitmessung` liest vier Dateien, und `tabs.rs` trägt den Filter mit (niedrig).**
Seit Schritt F2 hält `crates/krk-ui/src/tabs.rs` den Filtertext über den Ordnerwechsel, den `Durchlauf` je Tab und den Einzug der Befunde. Ein Zeitgeber dort sähe die Probe nicht. Die Datei ist nicht ohne Weiteres nachzutragen, weil ihr Prüfmodul `Duration` und `SystemTime` führt und `code_zeilen` nur Kommentarzeilen streicht.
Datensatz: `issues/260815-0211_o_die-probe-gegen-eine-zeitmessung-liest-vier-dateien-und-tabs-rs-traegt-den-filter-mit.md`

**N3 — `die_dateiliste_bleibt_flach_und_hat_vier_spalten` liest eine Datei, C2.9 spricht über den Baum (niedrig).**
Die Probe zählt `NSOutlineView` über `include_str!("appkit/tabelle.rs")`. „Keine zweite Tabellenklasse" ist damit gar nicht geprüft, denn eine zweite Klasse stünde gerade nicht in dieser Datei. `crate::quellbaum::quelldateien` steht für genau diesen Fall bereit, und drei andere Proben derselben Runde benutzen sie.
Datensatz: `issues/260815-0211_o_die-probe-fuer-die-flache-dateiliste-liest-eine-datei-c2-9-spricht-ueber-den-baum.md`

**N4 — `die_angezeigte_datei_bleibt_bei_zwei_quellen` zählt `return Some(` (niedrig).**
Eine dritte Quelle als Rumpfwert, als `.or_else`, als Zweigwert eines `match` — keine davon zählt die Probe mit. Es ist die Blindheit, die `quellbaum.rs` unter `# Was keine Zaehlung entscheiden kann` selbst beschreibt, und der Doc-Kommentar der Probe benennt sie nicht, sondern erklärt die heutige Schreibweise zur Regel.
Datensatz: `issues/260815-0211_o_die-probe-fuer-die-angezeigte-datei-zaehlt-return-some-und-haengt-an-der-schreibweise.md`

**Zwei Zählproben halten dagegen, was sie zusagen.** `die_regel_hat_genau_einen_aufrufer` zählt über `aufrufstellen`, das die Schreibweise des Aufrufs nicht mehr voraussetzt, und benennt seine verbleibende Blindheit (`use … as`). `die_sprungmarke_steht_nirgends_mehr_im_baum` und `die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer` laufen über `gemeinsam::quelldateien()`, also über `crates/` und nicht über eine Kiste.

### Ein abgenommenes Kriterium der Runde 1 ist ersetzt

**Kein Befund.** Die Sprungmarke ist restlos gefallen: `struct Sprungmarke`, `Sprungmarke::tippen`, `erste_zeile_mit` und die Konstante `PAUSE` haben im ganzen Baum null Fundstellen, geprüft über `quelldateien()`. Die Rücksetzrufe in `nach_lesebeginn`, `tab_gewechselt` und `umsortiert` sind mitgefallen, das Ivar `sprungmarke` ist weg, und die fünf Proben in `tests/navigation.rs` sind gestrichen statt umgeschrieben.

**Was mit ihr fiel und von dieser Runde ersetzt ist**, ist im Spec unter „Was diese Runde am Spec der Runde 1 ändert" einzeln benannt und stimmt mit dem Baum überein. Die Zeichenprüfung `traegt_ein_dateiname` ist geblieben, hat das Modul gewechselt und behält ihre zwei Rufer (C1.4). `Nachschlag::Sprungmarke` behält seinen Namen, und der Doc-Kommentar sagt, warum: der Wert benennt „eine Taste ohne Zusatztaste, die keiner Funktion gehört", und das trifft weiter zu. Verloren gegangen ist nichts, das kein Kriterium dieser Runde ersetzt.

---

## Was quer liegt

**Der Bau fährt an drei Stellen auf einer Empfehlung, die eine offene Nutzerfrage noch kippen kann, und jede dieser Stellen ist eine Zeile.** `ordner_setzen` trägt die Antwort auf „bleibt der Filtertext bei einem Ordnerwechsel stehen" in `let filtertext_ueberlebt = tief;`, mit dem Hinweis daneben, was aus der Zeile bei einer anderen Antwort würde. `Rang::ALLE` trägt die Rangfolgefrage in einer Zeile. `bereichsleiste_nachziehen` trägt die Frage „je Tab oder je Fenster" in der Quelle eines einzigen Arguments. Das ist die richtige Bauart für einen Spec, der auf vier offenen Fragen fährt, und es ist an allen drei Stellen gleich gemacht.

**Die drei Sicherheitsflächen dieser Runde liegen je an einer Stelle.** Der Prüfschritt der Sichtbarkeit (`Ordnermodell::sichtbar`), die Regel der Rückschritt-Taste (`kommandos::rueckschritt`) und die Frage, ob ein Name den Filtertext trägt (`filter::traegt_die_folge`) haben je einen Ort und je eine Probe darüber. Der Zweig `Name trägt die Folge?` ist eigens als `name_traegt_den_filter` herausgegeben, damit die Auftragsliste des Durchlaufs ihn nicht nachbaut. Das ist die Naht, an der diese Runde am ehesten hätte auseinanderlaufen können, und sie ist geschlossen.

**Zwei bekannte Lagen dieses Baums bestehen fort und sind keine Befunde dieser Runde.** `krk-ui` hat kein Bibliotheksziel, also stehen die Proben der Oberfläche in `#[cfg(test)]`-Modulen und manche Zusage hat keinen Ort; die Liste der ab Werk unbelegten Funktionen steht deswegen zweimal im Baum, und der Circle führt die Frage dazu bereits als Datensatz. Der Abnahmelauf am laufenden Bündel steht aus und ist Nutzerarbeit; zehn der 75 Kriterien haben einen Bündelanteil.

---

## Reihenfolge

**Vor der Auslieferung:** H1. Der Befund ist selten erreichbar, aber er liefert stillschweigend eine falsche Liste, und die billigste der drei genannten Stufen — `EMFILE` und `ENFILE` von den übrigen Öffnungsfehlern trennen und den Auftrag dann gar nicht erst entscheiden — ist klein und ändert an der übrigen Bauart nichts.

**Vor dem Schluss der Runde, weil beide den Spec betreffen:** M1 und M2. Beide sind eher am Spec zu berichtigen als am Baum, und beide sind Fragen an den Nutzer, keine Bauaufträge.

**Aufräumen, in beliebiger Reihenfolge:** N1 bis N4. Keiner der vier zeigt einen Fehler im Bau; alle vier zeigen eine Probe, die weniger hält als ihr Text zusagt. Am billigsten sind N3 und N4, weil sie in einem Zug mit dem Doc-Kommentar zu erledigen sind, der die Grenze benennt.
