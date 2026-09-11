# Durchsicht der Runde 24: KRK meldet Neuerungen an readers, settings, keymap

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `feecd6c..d9535c4`
**Not-opened:** `fusion-workbench/.asset-provenance`, `fusion-workbench/.fusion-setup`, `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/portfolio.md`, `6c11b1f2.md`, `260905-2008-orchestrator-session.md`, `260910-0735-playmaker-direct-dispatch.md`, `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap.md` (der Circle-Datensatz selbst), `260910-0818_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l4.md`, `260910-0818_*_wo-merkt-sich-krk-fuer-welche-fassung-es-die-neuerungen-schon-gemeldet-hat.md`, `260910-1600_*_was-zeigt-das-blatt-auf-abruf-wenn-der-start-nichts-erhoben-hat.md`, `260910-0739_*_claude-md-sagt-es-gebe-keinen-vorgesehenen-circle-seit-heute-gibt-es-einen.md`, dazu alle zwölf Verlaufsprotokolle dieser Runde (`260910-0707-shaper-…` bis `260910-1830-ontocoder-…`)

**Bereich:** code. Die ganze Runde war ungedeckt: der Durchsichtsspeicher des Circles war
leer, und keine Datei im Baum trug ein `**Reviewed-range:**`, das diese Commits deckt.

## Summary

Die Runde hält, was sie gebaut hat, und sie hält es an mehr Stellen mit einer Probe als
üblich: die Pflichtstelle, die der Übersetzer nicht hält — der Ausführungszweig des neuen
Kommandos — ist erstmals von einer eigenen Quelltextprobe gehalten, und die Zählprobe der
Öffnungen ist an ihrem Gegenfall geeicht statt behauptet. Alle zehn Planschritte sind gegen
den Baum gelesen und tragen ihre Arbeit; keine behauptete Erledigung ist leer.

Ein Befund wiegt schwer: der Vergleich entscheidet „beschädigt" am rohen TOML und nicht am
Leser der Datei, und in mehreren erreichbaren Fällen erzählt das Blatt dem Nutzer damit
etwas über eine Datei, mit der KRK gar nicht arbeitet. Zwei Befunde betreffen die
Nutzerdokumentation des Schrittes 10, zwei sind Aufräumarbeit.

## Totals

| Gewicht | Zahl |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 2 |
| Low | 2 |

Dazu ein Nachtrag an einem vorhandenen offenen Datensatz.

## Befunde nach Thema

### Der Vergleich fragt den falschen Leser (High)

`crates/krk-core/src/ablage/neuerungen.rs:348-374`. `eine_datei` lädt jede verglichene Datei
als `toml::Table` und wertet die `Ersetzung` dieses Ladens als „beschädigt". `toml::Table`
nimmt jedes syntaktisch gültige TOML an; ob die Datei ihrem **eigentlichen** Leser genügt,
entscheidet eine Ebene darüber, und die fragt niemand. Gefangen ist genau ein Fall, der
unbekannte Eintrag (`:364-366` über `eigene_eintraege_moeglich`). Durch gehen:

- `keymap.toml` mit `Belegungsfehler::Schreibweise`, `::FunktionDoppelt` oder `::Konflikt`
  (`crates/krk-core/src/tasten/belegung.rs:1840-1856`) — drei von vier Werten der
  Aufzählung. `belegung::laden` (`:1766-1788`) verwirft die Datei, KRK läuft auf der
  Auslieferungsbelegung, und `neuerungen` meldet `Befund::Verglichen`.
- `settings.toml` mit falschem Typ an einem bekannten Schlüssel.
- `readers.toml` mit einem verschriebenen Bausteintisch
  (`crates/krk-core/src/leseprofil/datei.rs:27-31`, `:64-75`): die ganze Datei fällt weg, KRK
  arbeitet ohne jedes Profil, und `neuerungen` vergleicht.

Der Nutzer liest dann im selben Start zwei Sätze, die einander widersprechen, und auf dem
Blatt den Preissatz aus `preis` (`:647-650`), der das Gegenteil dessen sagt, was wirklich
gilt. Die Probe, die das halten soll,
`ein_unbekannter_eintrag_macht_settings_und_keymap_beschaedigt`
(`crates/krk-core/tests/ablage.rs:4561-4629`), legt allein den einen gefangenen Fall hin; der
Modulkopf (`neuerungen.rs:65-89`) verallgemeinert ihn zu einer Eigenschaft. Das ist eine
Probe, die weniger prüft, als ihr Doc-Kommentar behauptet.

**Vorschlag:** den Befund vom eigentlichen Leser nehmen. Der Start hat seine Antwort im
selben Durchgang bereits (`sitzung_laden`, `crates/krk-ui/src/appkit/anwendung.rs:2031-2037`);
sie ist zu reichen und nicht neu zu erheben.

Datensatz: `260911-1838_o_der-neuerungsvergleich-entscheidet-beschaedigt-am-rohen-toml-und-nicht-am-leser-der-datei.md`.

### Die Nutzerdokumentation des Schrittes 10 sagt zweimal etwas, das der Baum nicht trägt (Medium, zwei Befunde)

**`README.md:57-60`** eröffnet den neuen Abschnitt mit „KRK legt jede von ihnen beim ersten
Start an und schreibt sie danach nie wieder" und zählt darunter auch `keymap.toml`. Beide
Hälften sind für diese Datei falsch: `anlegen_falls_fehlt` gibt es nur in
`ablage/einstellungen.rs` und `ablage/leseprofile.rs:128-134`, und geschrieben wird
`keymap.toml` sehr wohl wieder, nämlich in `tasten/belegung.rs:1663`. Die Runde selbst rechnet
mit dem Gegenteil: `Befund::Fehlt` (`neuerungen.rs:197-201`) nennt „KRK legt sie nie an" als
den gewöhnlichen Fall. `HowTo.md:41` sagt es richtig, die zwei Stellen widersprechen
einander.

**`HowTo.md:24-30`** führt sieben Ablagedateien. `reported.toml` fehlt, obwohl die Runde sie
gebaut hat und der Satz unter der Tabelle den Leser auf `Datei::ALLE` als Quelle verweist —
dort stehen acht. Die vorhandene Probe
`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien` liest allein
`crates/krk-core/src/ablage/` und kann die Stelle nicht sehen; `HowTo.md` stand in der
Dateiliste des Schrittes 10, lag also im Zuschnitt.

Datensätze:
`260911-1838_o_die-readme-sagt-krk-lege-alle-drei-nutzerdateien-beim-ersten-start-an-keymap-toml-legt-es-nie-an.md`,
`260911-1838_o_die-dateitabelle-der-howto-fuehrt-sieben-ablagedateien-seit-der-runde-24-sind-es-acht.md`.

### Eine Aussage aus Schritt 7 ist mit Schritt 9 falsch geworden (Low)

`crates/krk-core/src/tasten/belegung.rs:847-849`: „er zeigt den Bestand **vom Start** und
liest die drei Dateien nicht neu". Der Nutzerentscheid vom 260910-1600 hat das umgekehrt.
Schritt 9 hat die drei anderen Stellen mit demselben Satz nachgezogen — `blatttext`
(`neuerungen.rs:521-539`), den Kopf von `blaetter/neuerungen.rs:6-7` und das Feld
`AnwendungsIvars::neuerungen` (`anwendung.rs:744`) — und diese eine nicht, weil sie im
anderen Crate liegt.

Datensatz: `260911-1838_o_der-doc-kommentar-des-neuen-kommandos-sagt-es-lese-die-drei-dateien-nicht-neu.md`.

### Fünf öffentliche Lesezugänge ohne Rufer im Betriebscode (Low)

`Bestand::ordner`, `Bestand::dateien`, `Bestand::fuer`, `Bestand::traegt_unterschied` und
`Neuerungen::traegt_unterschied` (`neuerungen.rs:238-282`) werden allein aus
`crates/krk-core/tests/ablage.rs` gerufen; `startzeile` und `blatttext` greifen die Felder
unmittelbar. Dieselbe Klasse wie `260826-1221_o_…` und `260826-1225_o_…`.

Datensatz: `260911-1838_o_vier-lesezugaenge-von-bestand-und-neuerungen-haben-keinen-rufer-im-betriebscode.md`.

### Nachtrag an einem vorhandenen Datensatz

`260820-2235_o_der-gemessene-start-laedt-die-lesezeichen-nicht-und-die-leiste-schweigt-mit-falscher-begruendung.md`
hat eine `Also seen:`-Zeile bekommen. Der Grund: `oberflaeche_aufbauen` fährt ab der zweiten
Startmeldung ein modales Blatt herunter. Heute ist das im Messmodus unerreichbar, weil
`leiste_einrichten` dort schweigt (`anwendung.rs:2192-2211`) und nur die Belegungsmeldung
übrig bleibt. Beantwortet der Nutzer jenen Datensatz mit „Ja", können zwei Meldungen
zusammentreffen, und dann steht mitten im Messlauf ein Blatt, das jeden Tastenbefehl sperrt.
Der Datensatz
`260910-0818_*_wie-erreichen-n-startmeldungen-den-nutzer-wenn-die-eine-zeile-nur-eine-traegt.md`
hat unter `## Constraints` verlangt, dass die Antwort das sagt; seine `Answered:`-Zeile sagt
nichts darüber. Kein eigener Datensatz, weil der vorhandene dieselbe Frage trägt.

## Was geprüft und in Ordnung befunden ist

**Die drei namentlich beauftragten Eigenschaften aus CLAUDE.md halten.**

- *Die drei Pflichtstellen eines neuen Kommandos.* `Kommando::KENNUNGEN` trägt
  `neuerungen_zeigen` (`belegung.rs:988`, Länge 87), `Kommando::wirkungsbereich` und
  `bereich_des_kommandos` (`belegungsmodell.rs:420-431`) sind beantwortet, und der
  Ausführungszweig steht (`anwendung.rs:4052`). Die dritte, die weder Übersetzer noch Probe
  hielt, ist erstmals gehalten:
  `neuerungsproben::der_befehl_hat_einen_eigenen_ausfuehrungszweig` liest den Quelltext von
  `kommando_ausfuehren` und verlangt den Zweig, mit zusammengesetzter Nadel, damit sie sich
  nicht selbst findet.
- *Die `ALLE`-Liste neben ihrer Aufzählung.* `Datei::ALLE` ist `[Datei; 8]`, und
  `UNLESBARE_ALLE_LISTEN` (`baum.rs:785`) trägt ihren Grund mit der neuen Zahl.
- *Die Naht zwischen Umlaut und Umschrift.* Jede der elf neuen nutzersichtbaren
  Zeichenketten trägt Umlaute (`startzeile`, die vier Zweige von `blatttext`, `gegenrichtung`,
  die drei Sätze in `preis`, die Kopfzeile des Blattes, die drei Statuszeilenmeldungen in
  `neuerungen_zeigen` und `neuerungen_erheben`); Bezeichner und Kommentare daneben tragen die
  Umschrift. Vier Proben halten den Wortlaut.

**Die Endbedingungen des Plans, soweit ohne Gerät prüfbar.** `make check` endet am Stand
`d9535c4` mit 0, alle fünf Kommandos in seiner Reihenfolge (nachgefahren am 260911, 26 min
auf diesem Gerät; `cargo test --workspace` grün, `clippy -D warnings` grün,
`RUSTDOCFLAGS="-D warnings" cargo doc` grün). Der Defekt `260820-2235` ist
geschlossen und trägt eine `Resolved:`-Zeile mit dem Commit. Die zwei sperrenden Datensätze
tragen `_i_`, der L4-Datensatz `_a_`. `opt+cmd+i` ist über jede `tasten`-Liste der
Belegungsdatei frei (genau ein Vorkommen). Die Kopfzahl der Belegungsdatei steht auf 94
Funktionen mit 97 Kombinationen und stimmt gegen 94 `[[funktion]]`-Blöcke.
`waehrend_eines_blattes_kommen_genau_diese_vier_durch` bleibt bei vier: `NeuerungenZeigen`
steht nicht in `immer_erreichbar` (`zulaessigkeit.rs:364-369`). Die Version ist der einzige
Inhalt der beiden Cargo-Manifeste in dieser Spanne; keine fremde Kiste ist dazugekommen, die
C-Freiheits-Zusage ist von dieser Runde unberührt.

**Nebenläufigkeit und Ressourcen.** Die Nacherhebung geht über `unter_der_sperre` und damit
über dieselbe `Ablage` wie jeder andere Rufer, also entsteht kein zweiter Durchgang neben
einem laufenden. `beginSheetModalForWindow_completionHandler` kehrt sofort zurück, der
`RefCell`-Borrow auf `ivars().neuerungen` überlebt den Aufruf nicht in einen zweiten Zugriff
hinein. Der `Zugang` der Erhebung hält keine Deskriptoren über den Durchgang hinaus.

**Eine bewusst hingenommene Verteuerung, kein Befund.** `uebersprungenliste`
(`kommandos/operationen.rs:735-754`) baut seit Schritt 5 jede Zeile und kürzt danach, statt
vorher `take(12)` zu nehmen. Der Kommentar über der Stelle schreibt den Tausch aus: der
Wortlaut „… und N weitere" steht dafür an genau einer Stelle. Bei Tausenden übersprungener
Einträge sind das Tausende formatierte Zeichenketten auf dem Hauptfaden, die verworfen
werden; die Liste selbst liegt ohnehin schon vollständig im Speicher.

## Übergreifende Beobachtung

**Die Runde baut an drei Stellen eine zweite Fassung einer Antwort, die der Baum schon
hat, und zwei davon sind bewusst und belegt.** Die Kürzung langer Namenslisten ist zu einer
Stelle zusammengezogen (Schritt 5), die Zahlenschreibweise ebenso — der Zielort ist dabei
bereits als Befund gemeldet (`260910-1146_o_…`). Die dritte ist nicht belegt und ist der
High-Befund oben: „ist diese Ablagedatei beschädigt" wird in `neuerungen.rs` ein zweites Mal
beantwortet, schwächer als an der Stelle, die die Frage eigentlich beantwortet. Das Muster
ist damit erkannt und an zwei von drei Stellen richtig behandelt.

**Die zwei Zahlen an Prosastellen sind wieder auseinandergelaufen, und zwar außerhalb der
Reichweite der Probe, die sie halten soll.** Innerhalb von `crates/krk-core/src/ablage/` hat
die Runde jede Stelle von sieben auf acht gezogen, weil
`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien` sie beim ersten roten
Lauf namentlich nennt. `HowTo.md` liegt außerhalb und ist liegengeblieben. Die Lehre ist
nicht, die Probe zu weiten, sondern dass jede Aufzählung, die der Nutzer liest, ihren eigenen
Zeiger auf die Quelle braucht — den Zeiger hat `HowTo.md` sogar, direkt unter der Tabelle,
und er ist beim Schreiben nicht befolgt worden.

## Reihenfolge

1. Der High-Befund vor der nächsten Auslieferung. Die Runde ist mit `v1.9.0` bereits
   ausgeliefert, der falsche Satz steht also beim Nutzer.
2. Die zwei Dokumentationsbefunde zusammen mit `260910-1843_o_…` in einem Durchgang; sie
   betreffen dieselben zwei Dateien und CLAUDE.md daneben.
3. Die zwei Low-Befunde beim nächsten Aufräumen der Kiste.
