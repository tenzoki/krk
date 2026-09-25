# Fünf Defekte um die Textfläche des Editors

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1057 bis 260810-1145

---

## Auftrag

Fünf offene Datensätze im Circle `260807-2116-eingebauter-editor-mit-textmarken`,
in zwei Gegenständen gebündelt: der Preis je Tastendruck (`260809-2322` und
`260810-0054`) und die Korrektheit am Eingang der Fläche (`260810-1044` und
`260810-1028`), dazu die Abwägung `260810-1001` mit dem Entscheidungsdatensatz
`260810-1044`.

Dateigrenze: `crates/krk-ui/src/appkit/editor.rs`,
`crates/krk-ui/src/hervorhebung.rs`, `crates/krk-ui/Cargo.toml`, dazu bei Bedarf
eine neue Datei unter `crates/krk-ui/tests/`. Ausdrücklich nicht anzufassen:
`krk-core/**`, `editormodell.rs`, `appkit/anwendung.rs`, `resources/**`, das
Plandokument.

## Zuerst gemessen, dann geändert

Die Messungen liefen aus einem Wegwerf-Prüfziel mit `harness = false` unter
`crates/krk-ui/tests/`, das den Hauptfaden selbst hält; es ist nach der Messung
entfernt, `Cargo.toml` steht unverändert. Gerät: macOS 15.7.7 (Build 24G720),
Rust 1.97.1, `--release`. Korpus:
`crates/krk-ui/src/appkit/anwendung.rs` (229 kB) und Vielfache davon.

**Der Preis je Tastendruck (`260809-2322`, vorher ungemessen):**

```
       Byte   NSString->String   ist_in_gehaltener_form   in_gehaltene_form   Summe
    229 029           0,98 ms                  0,017 ms            0,015 ms   1,02 ms
  1 832 232           7,61 ms                  0,130 ms            0,131 ms   7,87 ms
 19 467 465          88,30 ms                  1,805 ms            1,858 ms  91,96 ms
```

96 Prozent des Preises liegen im Umschreiben aus UTF-16, nicht in der Wandlung.

**Die Einfärbung (`260810-0054`, Nachmessung):** 0,25 MB/s statt der geführten
0,30 MB/s, dieselbe Größenordnung. Der benannte Ausweg gemessen: Zustände
aufheben kostet sechs bis neun Prozent Aufpreis, ein Wiedereinstieg nach einem
Anschlag in der Mitte 0,2 bis 0,5 ms unabhängig von der Dateigröße, der
Zeilenvergleich 0,13 ms bei 229 kB und 12 ms bei 19 MB.

**Die Zahl, die den Zuschnitt bestimmt hat und vorher in keinem Datensatz
stand:** je Zeile einen Zustand aufzuheben kostet rund 780 Byte. An 19 MB mit
394 060 Zeilen wuchs der Prozess von 23,3 MB auf 331,5 MB, also um das
Sechzehnfache der Datei. Daraus folgt `ZUSTANDSABSTAND = 32`.

## Was gebaut ist

**1. Das Fortschreiben der Einfärbung (`260810-0054`, geschlossen).**
`hervorhebung::fortschreiben` rechnet den vorigen Durchgang fort:
Zeilenvergleich, Wiedereinstieg am letzten Haltepunkt vor der geänderten Zeile,
Rechnen bis der Zustand des Zerlegers wieder mit einem aufgehobenen
zusammenfällt, Zusammensetzen aus übernommenem Anfang, neu gerechneter Mitte und
verschobenem Schwanz. Neu in `hervorhebung.rs`: `Einfaerbungsstand` (der
aufgehobene Stand, öffentlich), `Schluessel`, `Zeilenanfang`, `Haltepunkt`,
`Vorlage`, `Einstieg`, `Rest`, `Anschluss`, `Rechnung`, das Merkmal `Strecke` mit
`teilen` und `ab_der_stelle`, und `rechnen` als der eine Durchgang.
`Einfaerbungsvorgang::starten` nimmt die Vorlage und gibt sie mit dem Ergebnis
zurück; `Abholung::Fertig` trägt jetzt den Stand statt der Formatierung.
`formatieren` steht unter `cfg(test)` und ist die eine Seite der Zusage, an der
das Fortschreiben hängt.

An der gebauten Fassung gemessen: 1 458 ms auf 9,6 ms bei 229 kB (152-fach),
10 475 ms auf 10,8 ms bei 1,8 MB (968-fach).

`editor.rs` hält den Stand in `EditorIvars::einfaerbungsstand` zwischen zwei
Anfragen; ein überholtes Ergebnis lässt seine Formatierung fallen, den Stand aber
nicht — er ist die richtige Vorlage für den nächsten Lauf. Fallen gelassen wird er
beim Wechsel in die Rohansicht und ohne gehaltene Datei.

**2. Das eingefügte CRLF ist rücknehmbar (`260810-1044`, geschlossen).** Der
Umkehrpunkt entsteht in `text_zurueckschreiben` **vor** dem Ruf an
`Editormodell::bearbeiten`, und nur dann, wenn `ist_in_gehaltener_form` eine
Wandlung ankündigt; damit kostet er nichts je Tastendruck. `Verlauf` trägt einen
dritten Wert `TraegtNurDiese`: der Stapel fällt, und die eine gültige Handlung
wird danach angemeldet. Beide Einwände des Datensatzes sind entkräftet — der
Umkehrpunkt trägt den gehaltenen Stand aus dem Modell und nicht den `\r`-Text der
Fläche, und der Stand vor dem Einfügen ist nicht fort. Der benannte Preis: das
erste `cmd+z` nimmt das Einfügen zurück, ein zweites tut nichts.

**3. Eine falsche SAFETY-Begründung berichtigt (`260810-1139`, neu und
geschlossen).** Vor `addAttributes:range:` stand, die Stellen der Formatierung
seien aufsteigend und überschneidungsfrei. Für die Auszeichnungen ist das falsch,
und das ist gemessen: an `- Punkt mit `Code`` liefert die Formatierung
`FesteSchrift` bei 12 und danach `Listenzeile` bei 0. Zulässig ist der Aufruf
trotzdem, weil die beiden verschiedene Merkmalsnamen setzen und
`addAttributes:range:` zusammenlegt.

## Was offen bleibt, und warum

**`260809-2322` (der ganze Stand je Tastendruck).** Der Preis ist jetzt gemessen
und im Datensatz eingetragen. Der Schnitt, der ihn erledigt, liegt in
`Editormodell::bearbeiten` und in `krk-core/src/text/datei.rs`; beide lagen
außerhalb der Dateigrenze.

**Die Annahme beider Datensätze, sie „lebten von derselben Antwort", ist mit
Zahlen widerlegt.** Drei Unterschiede: verschiedene Fäden (Hauptfaden gegen
Arbeitsfaden), verschiedene Größenordnungen (92 ms bei 19 MB gegen 7 000 ms bei
1,8 MB), verschiedene Angaben. Das Fortschreiben braucht `editedRange` aus
`NSTextStorage` **nicht** — der Zeilenvergleich findet die geänderte Zeile selbst.

**`260810-1028` (Herkunft an `datei_oeffnen`).** Der genannte Weg verlangt drei
Änderungen, zwei davon in `appkit/anwendung.rs`: die Signatur von
`Ausgangsmelder`, die Sichtbarkeit von `Oeffnungsherkunft` und
`editor_oeffnen_lassen`. Eine halbe Fassung bricht den Bau oder erzwingt nichts.
Der Datensatz führt die drei Punkte jetzt als Aufstellung, plus einen vierten:
die Herkunft gehört neben den Ladevorgang und nicht neben den Bereich, sonst
wiederholt sie den Fehler von `260810-0418` eine Ebene tiefer.

**`260810-1001` und die Entscheidung `260810-1044`.** Die Messung des Vorgängers
hält: `harness = false` gibt den Hauptfaden, `cargo test` fährt es mit. **Beide
Optionen des Entscheidungsdatensatzes ruhen aber auf einer falschen Annahme:
`krk-ui` hat kein Bibliotheksziel.** Eine Prüflaufdatei unter `tests/` ist eine
eigene Kiste und erreicht nichts aus `krk-ui`, ob `pub` oder nicht; gemessen als
`E0433`. Keine der beiden Optionen ist damit baubar, und `Answered:` wäre falsch
gesetzt. Der Datensatz führt jetzt zwei neue Optionen — ein `src/lib.rs` samt
Umbau von `main.rs`, oder ein zweiter Kistenkopf unter `src/`, der die Oberfläche
ein zweites Mal übersetzt und `cfg(test)` verliert — und empfiehlt die erste,
außerhalb eines Bugfix-Durchgangs. Der Doc-Kommentar von `an_einer_flaeche` ist
entsprechend berichtigt.

## Prüfungen

Neu in `hervorhebung.rs`: `ein_fortgeschriebener_durchgang_gleicht_dem_vollen`
(14 Änderungen, jede in beiden Richtungen),
`das_fortschreiben_traegt_auch_die_markdown_auszeichnungen` (sieben),
`das_fortschreiben_traegt_ueber_viele_haltepunkte`,
`derselbe_text_kommt_ohne_rechnung_zurueck`,
`eine_andere_tafel_laesst_die_vorlage_fallen`,
`eine_andere_sprache_laesst_die_vorlage_fallen`,
`die_haltepunkte_stehen_im_gemessenen_abstand`, `einfacher_text_hebt_nichts_auf`.
Verglichen wird die **Wirkung** Zeichen für Zeichen und nicht die Buchführung:
zwei aneinanderliegende Stücke derselben Farbe sind dasselbe wie ein
zusammengezogenes, und genau dieser Unterschied fällt an den beiden Nähten an.

Neu in `editor.rs`: `eine_anmeldung_nach_dem_leeren_steht_im_stapel`, in der
Betriebsart der Laufzeit samt Umlauf der Laufschleife.

## Zeitzusagen aus C8 der Runde 1

Keine der zehn ist berührt. Sie messen Navigator-Wege — Verzeichnislesen,
Auswahl, Bildlauf, Vorschau —, und keine davon läuft durch `hervorhebung.rs` oder
durch den Rückweg der Textfläche. Die Geschwindigkeit der Syntaxhervorhebung ist
der vierte Gegenstand, den der Spec der Runde 2 unter `## Verhältnis zu den zehn
Zeitzusagen aus C8 der Runde 1` der späteren Messrunde übergibt; die Zahlen oben
gehören dorthin.

## Abnahme

```
  cargo build --workspace                exit 0
  cargo test --workspace                 exit 0   744 Prüfungen bestanden
  cargo clippy --workspace --all-targets exit 0   keine Warnung
  cargo fmt -p krk-ui --check            exit 0
```

`cargo fmt --all` ist bewusst nicht gefahren: an den Nachbarkisten arbeiteten
parallel andere Agenten.

## Geänderte Dateien

- `crates/krk-ui/src/hervorhebung.rs`
- `crates/krk-ui/src/appkit/editor.rs`

`crates/krk-ui/Cargo.toml` steht unverändert: der `[[test]]`-Abschnitt der
Messung ist mit dem Wegwerf-Prüfziel wieder entfernt.
