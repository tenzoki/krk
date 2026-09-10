# KRK meldet, was eine neue Fassung an readers.toml, settings.toml und keymap.toml mitbringt

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Claim:** Claimed 260910-0754: Kai Stalmann <kai@stalmann.org>, checkout 6c11b1f2.
**Active spec/plan:** 260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md
**Active session history:** 260905-2008-orchestrator-session.md

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

**Die drei Dateien und ihr Weg stehen fest und sind an einer Stelle nachzulesen.**
`Datei::ALLE` (`crates/krk-core/src/ablage/pfade.rs`) führt sieben Ablagedateien, und genau
drei davon pflegt der Nutzer von Hand: `keymap.toml`, `settings.toml` und `readers.toml`.
Alle drei tragen `Leerbefund::Vorgabe` und stehen darin zusammen, während `bookmarks.toml`
und `session.toml` als von KRK geschriebene Dateien `Leerbefund::Beschaedigt` tragen
(`260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`,
Möglichkeit 2). Die Auswahl, die der Nutzer in der ersten Runde getroffen hat, deckt sich
damit genau mit einer Fallunterscheidung, die der Baum schon führt.

**Die Auslieferungsfassung liegt im laufenden Programm und nicht im Bündel.** Alle drei
Dateien stehen über `include_str!` einkompiliert bereit: `AUSLIEFERUNGSTEXT` in
`ablage/einstellungen.rs` und in `ablage/leseprofile.rs`, `AUSLIEFERUNGSTEXT` in
`tasten/belegung.rs`. Der Vergleich, den die Directive verlangt, braucht deshalb weder das
Bündel noch das Netz: er stellt einen einkompilierten Text neben eine Datei auf der Platte.
Beim ersten Start wird die Datei wörtlich aus diesem Text geschrieben und danach von KRK nie
wieder angefasst.

**Jede der drei Dateien hat eine benannte Einheit, über die sich ein Unterschied zählen und
nennen lässt.** `readers.toml` führt `[[profil]]` mit einem `name`, `keymap.toml` führt
`[[funktion]]` mit `id` und `name`, `settings.toml` führt oberste Schlüssel und trägt heute
genau einen, `terminal`. Die Zusage „Zahl je Datei beim Start, Namen auf Abruf" ist damit an
allen drei Dateien einlösbar.

**Die Spannung zu C1.6 der Runde 16 ist gesehen und aufgelöst, nicht übergangen.** Für
`readers.toml` allein gilt `Ersatz::Nichts`: eine beschädigte Datei führt zu gar keinem
Profil, und die Auslieferungsfassung springt ausdrücklich nicht ein, weil das hiesse, dem
Nutzer Profile unterzuschieben, die er vielleicht gerade herausgenommen hat (Modulkopf von
`ablage/leseprofile.rs`, „Zweite Abweichung"; Circle
`260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`). Diese Runde nimmt davon
nichts zurück: sie meldet und schreibt nicht. Die Startzeile nennt allein, was die
Auslieferungsfassung führt und die Nutzerdatei nicht, und sie erscheint einmal je Fassung.
**Der Rest bleibt als bekannter Preis stehen:** wer ein Profil bewusst herausgenommen hat,
liest seinen Namen mit jeder neuen Fassung wieder in der Liste auf Abruf. Der Nutzer hat das
am 260910 vorgelegt bekommen und angenommen.

**Ein offener Defekt liegt auf dem Weg der Startzeile.** Die Startmeldungen überschreiben
einander, und nur die letzte erreicht den Nutzer: `Anwendungsdelegierter` sammelt sie in
einem Vektor und ruft je Meldung `meldung_zeigen`, das das eine Feld `fenstermeldung` setzt
(`crates/krk-ui/src/appkit/anwendung.rs`, die Schleife am Ende des Aufbaus;
`crates/krk-ui/src/appkit/tabelle.rs`, `meldung_zeigen`). Gemeldet ist das als
`260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`.
Die Zeile dieser Runde ist eine weitere Startmeldung und träte in genau diese Kollision. Die
Runde muss den Defekt entweder beheben oder ihre Zeile ausdrücklich gegen ihn stellen; ihn zu
übergehen hiesse, eine Zusage zu bauen, die im ersten Start neben einer beschädigten
Ablagedatei still ausfällt.

**Wo KRK sich die gemeldete Fassung merkt, ist offen und hat zwei Kandidaten.** Die eigene
Versionsnummer liegt zur Übersetzungszeit über `env!("CARGO_PKG_VERSION")` bereit
(`crates/krk-ui/src/appkit/titelzusatz.rs`). Ablegen lässt sie sich nicht in einer der drei
gemeldeten Dateien, denn KRK schreibt sie nicht; es bleiben `session.toml` und eine achte
Ablagedatei. Eine achte hält den Bau an drei vollständigen Fallunterscheidungen in
`ablage/pfade.rs` an (`format`, `leerbefund`, `ersatz`) und erzwingt dort je eine bewusste
Antwort. `session.toml` trägt bereits die offene Frage nach einer eigenen Fassungsangabe
(`260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md`);
wer hier eine Versionsnummer hineinschreibt, baut den halben Gegenstand jener Frage und muss
sie mitbeantworten, statt eine zweite Fassungsangabe daneben zu stellen.

**Das Blatt auf Abruf hat zwei Vorbilder und drei Pflichtstellen.** Gebaut wird es über
`Blatt::mit_schaltflaechen` (`crates/krk-ui/src/appkit/blaetter/mod.rs`), wie der Notizzettel
der Runde 9 und die Belegungsansicht aus C3. Ein stehendes Blatt sperrt jeden Tastenbefehl bis
auf vier; die Probe `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch`
schreibt sie namentlich aus. Der neue Befehl braucht eine Zeile in
`resources/default-keymap.toml`, eine in `Kommando::wirkungsbereich` und in
`bereich_des_kommandos`, eine in `Kommando::KENNUNGEN` und einen eigenen Ausführungszweig;
die letzten beiden hält der Übersetzer nicht, sondern eine Probe beziehungsweise gar nichts
(`CLAUDE.md`, „Was man nicht sieht").

**Der Wortlaut der Meldungen folgt der Naht vom 260907:** was der Nutzer durch KRKs
Oberfläche liest, trägt Umlaute, Kommentare und Bezeichner tragen die Umschrift.

## Dependencies

- `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` — die Runde, die
  `readers.toml` und mit C1.6 die Regel `Ersatz::Nichts` gebaut hat. Ihre Begründung bindet
  diese Runde.
- `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` — das Vorbild für das Blatt auf Abruf.
- `260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
  — offener Defekt auf dem Weg der Startzeile.
- `260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md`
  — offene Frage, die dieselbe Datei betrifft, in die sich die gemeldete Fassung schreiben liesse.
- `260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`
  — trennt die drei von Hand gepflegten Dateien von den zwei geschriebenen.

## Turn log

## Activation proposal

**Vorgeschlagene Aktivierung:** 260910-0735
**Lauf:** Playmaker-Sitzung `260910-0735-playmaker-direct-dispatch`

Diese Runde ist die empfohlene nächste, und sie ist es ohne Konkurrenz: sie ist der einzige
vorgesehene Circle im Bestand. Ihre Grundlage nennt eine offene Nutzerfrage,
`260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md`
im gemeinsamen Entscheidungsspeicher, und sie nennt sie nicht als Hindernis, sondern als
Überschneidung: wer die gemeldete Fassung nach `session.toml` schreibt, beantwortet jene
Frage mit. Dazu kommt ein offener Defekt auf dem Weg der Startzeile,
`260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
im gemeinsamen Defektspeicher; die Grundlage schreibt aus, dass die Runde ihn beheben oder
ihre Zeile ausdrücklich gegen ihn stellen muss. Beide Abhängigkeiten auf andere Runden,
`260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` und
`260813-2332-notizzettel-als-blatt-mit-zwei-zetteln`, tragen den beschränkten Abschluss.
Eine Heuristik, die allein den kohärenten Abschluss als erfüllte Vorbedingung zählt, würde
daraus eine Sperre lesen; in diesem Projekt misst der beschränkte Abschluss die
Verfügbarkeit des Nutzers für den Abnahmelauf und nicht die Reife der Runde, wie `CLAUDE.md`
unter „Projektstand" ausschreibt. Gebaut ist beides, und die zwei Runden binden diese hier
mit dem, was sie gebaut haben.

Der Aktivierungsschritt selbst gehört nicht diesem Lauf. Umbenannt wird der Datensatz vom
Nutzer über `/fusion:next` oder vom Orchestrator.
