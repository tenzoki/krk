# Durchsicht des Archivpakets, `f52b94d0..8c8579af`

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `f52b94d0..8c8579af`
**Not-opened:** `Cargo.lock`, `crates/krk-core/src/archiv/mod.rs`, `crates/krk-core/src/operation/entpacken.rs`, `crates/krk-core/tests/archiv.rs`, `crates/krk-core/tests/archiv_lesevorgang.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/fern_operation.rs`, `fusion-workbench/orchestrator-events.jsonl`, `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.record.json`
**Domain:** code
**Work-item:** 261017-1029-zip-und-gz-wie-ordner-im-dateifenster
**Maßstab:** Plan `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md` (E1 bis E19), die drei Entscheide und das Narrativ des Pakets; Dokumentation (`HowTo.md`, `README.md`, `CLAUDE.md`) ausdrücklich nicht
**Urteil:** revise

## Zusammenfassung

Das Ersetzen über Zwischennamen, Kennungsprüfung und ein `rename` hält, was es verspricht. Ich habe keinen Weg gefunden, auf dem ein Abbruch, ein Fehler oder ein voller Datenträger das Original beschädigt. Zip-Slip ist an zwei Stellen versperrt, der Hauptfaden bleibt frei. Lokale und ferne Wege sind unverändert.

Datenverlust entsteht an anderer Stelle: **nach** einem gelungenen Umbau. Wer die Archivdatei oder ihren Ordner in das Archiv verschiebt, verliert alles. Das ist nachgestellt und gehört vor jede Auslieferung behoben. Dazu kommen gebrochene harte Verweise im tar, nachgestellt, und zwei kleinere Lücken beim Entfernen der Quellen.

Geprüft habe ich durch Lesen und durch eigene Proben an einer Kopie des Stands `8c8579af` im Scratchpad, mit `cargo test -p krk-core --test repro_review`. Im Projekt habe ich nichts geändert. `make check` habe ich nicht gefahren.

## Summen

| Schwere | Zahl |
|---|---|
| Critical | 1 |
| High | 1 |
| Medium | 4 |
| Low | 7 |

Dazu ein Entscheid für den Nutzer (verdeckte Einträge).

## Befunde nach Thema

### Datenverlust beim Entfernen der Quellen nach dem Umbau

**Critical: Verschieben der Archivdatei oder eines Ordners, der sie enthält, in dieses Archiv löscht alles.** Nachgestellt:

- `Ort::Lokal(a.zip)` nach `a.zip:/`: Bericht `Fertig`, `uebersprungen: []`, danach `a.zip` fort.
- Ordner `projekt/` mit `projekt/a.zip` nach `a.zip:/`: Bericht `Fertig`, danach Ordner und Archiv fort.

Der Ablauf: `operation/archiv.rs`, `hinein` nimmt die lokale Quelle ohne Prüfung an. `ersetzen` benennt das neue Archiv an seinen Platz, und `Herkunft::Stelle(stelle) => stelle.baum_entfernen(…)` entfernt danach genau diesen Pfad. Die Oberfläche hält nichts auf (`anwendung.rs`, `uebertragen` und `fernauftrag_stellen` mit allein `quelle == ziel`; Abwurfregel mit `Schreibrecht::Unbekannt`). Erreichbar ist das mit F6 und mit einem Abwurf aus dem Finder. Issue `261017-1923-verschieben-einer-lokalen-quelle-die-das-zielarchiv-ist-oder-enthaelt-loescht-das-archiv.md`.

**Medium: Das Entfernen nach dem Verschieben aus einem Archiv prüft die Kennung nicht.** `Entfernposten` trägt keine Kennung. Der zweite Umbau entfernt die Pfade aus dem Archiv, das **dann** unter dem Namen steht. Issue `261017-1923-das-entfernen-nach-dem-verschieben-aus-einem-archiv-prueft-nicht-ob-das-archiv-seit-dem-kopieren-dasselbe-ist.md`.

**Medium: `baum_entfernen` nimmt mit, was während Konfliktfragen und Umbau in den Quellordner kam.** Die Grenze steht im Modulkopf, damit ist die erste der zwei bekannten Grenzen bewertet. Das Fenster schließt hier die Wartezeit an jeder Konfliktfrage und das Neuschreiben des ganzen Archivs ein. Der Verweis auf `ueber_die_grenze` verharmlost es deshalb. Issue `261017-1923-verschieben-ins-archiv-entfernt-den-ganzen-quellbaum-auch-was-waehrend-konfliktfragen-und-umbau-hinzukam.md`.

**Low, die zweite bekannte Grenze: „abgebrochen“, obwohl das Archiv ersetzt ist.** Die Meldung nennt die übertragenen Einträge, falsch ist sie also nicht, sie widerspricht aber der Regel „Abbruch heißt unverändert“. Issue `261017-1923-der-vorgangsstatus-sagt-abgebrochen-obwohl-das-archiv-ersetzt-und-die-quellen-entfernt-sind.md`.

### Ein gelungener Umbau verändert mehr, als der Nutzer angefasst hat

**High: harte Verweise im tar brechen.** Nachgestellt: `a.txt` und der harte Verweis `b.txt` darauf. Nach Löschen oder Umbenennen von `a.txt` endet `/usr/bin/tar -xf` mit Status 1, und `b.txt` entsteht nicht. Der Bericht sagt `Fertig`. Issue `261017-1923-ein-umbau-eines-tar-bricht-harte-verweise-deren-ziel-er-entfernt-oder-umbenennt.md`.

**Entscheid: verdeckte Einträge fallen bei jedem Umbau weg.** Das ist bewusst gebaut und im Kopf von `umbau.rs` begründet. Der Plan hat es aber nicht entschieden, und die Directive deckt es nicht. Empfehlung: ein solches Archiv gilt als allein lesbar, wie eines mit verschlüsseltem Eintrag. Entscheid `261017-1923-darf-jeder-umbau-die-verdeckten-eintraege-eines-archivs-verwerfen.md`.

**Low: ein Archiv mit `0o444` wird ohne Rückfrage umgeschrieben.** Issue `261017-1923-ein-archiv-ohne-schreibrecht-an-der-datei-wird-trotzdem-umgeschrieben.md`.

### Korrektheit der Formate

**Medium: eine Datei über 4 GiB lässt sich in kein Zip legen.** Nachgestellt: „Large file option has not been set“. `set_auto_large_file` wirkt in `zip-8.6.0` allein auf den Datenbeschreiber, `large_file` setzt niemand. `inference:` `zippen.rs` hat dieselbe Grenze. Issue `261017-1923-eine-datei-ueber-4-gib-laesst-sich-in-kein-zip-kopieren.md`.

**Low: eine Sonderdatei im lokalen Quellbaum lässt das ganze Hineinkopieren scheitern**, während der ferne Weg sie auslässt. Issue `261017-1923-eine-sonderdatei-in-einem-lokalen-quellbaum-laesst-das-ganze-hineinkopieren-scheitern.md`.

Geprüft und in Ordnung:

- **Zip-Rohform** (`zipform.rs`): die Versätze der festen Köpfe (42 zentral, 26/28 lokal), die Folge im ZIP64-Feld, ZIP64-Ende und Zeiger ab 65 535 Sätzen oder 4 GiB Versatz. Die Teilnummer wird auf 0 gesetzt. Das Unicode-Namensfeld fällt nur bei neuem Namen. Das UTF-8-Kennzeichen setzt der Umbau für einen Namen außerhalb von ASCII. Der Abgleich jedes gezeigten Eintrags mit seinem zentralen Satz über `central_header_start` und `header_start` fängt ein Zip mit Daten vor dem ersten Eintrag ab. Ein Eintrag mit Datenbeschreiber, gebaut mit `printf … | /usr/bin/zip -`, überlebt ein Anlegen daneben, und `unzip -t` ist danach fehlerfrei (nachgestellt).
- **tar**: lange Namen über 100 Zeichen bleiben beim Umbenennen eines anderen Eintrags erhalten, und ein umbenannter langer Name steht danach allein unter dem neuen Namen in `tar -tf` (nachgestellt). `raw_header_position` stimmt zwischen Leser (gewöhnliche Form) und Umbau (`raw(true)`) überein, beide nennen den Kopf des eigentlichen Eintrags. PAX-`size` gegen Kopfgröße und GNU-Lücken werden vor dem ersten Byte abgewiesen.
- **gz**: allein lesbar über `Schreibform` ohne Fall für `Gz`, an drei Stellen vor dem Start abgewiesen.

### Sicherheit

**Medium: lange Namen und PAX-Köpfe werden beim Lesen ohne Obergrenze gelesen.** `tar-0.4.46` liest sie mit `read_all` ganz ein. Der Umbau hat `HOECHSTENS_VORKOPF`, Leser und Lesezeiger haben keine Grenze. Issue `261017-1923-lange-namen-und-pax-koepfe-im-tar-werden-beim-lesen-ohne-obergrenze-in-den-speicher-gelesen.md`.

Geprüft und in Ordnung:

- `..`, absolute Namen, NUL und kein UTF-8 kommen über `Innenpfad::aus_archivname` nicht ins Verzeichnis.
- Einträge hinter einer Verknüpfung im selben Archiv fallen im Sammler weg.
- Am Ziel fragt die Maschine `symlink_metadata` und legt mit `create_new` an. Nachgestellt: ein Zip mit `a -> <draußen>` und `A/boese.txt` auf ein APFS-Ziel ohne Groß- und Kleinschreibung. Bei „Überspringen“ und bei „Überschreiben“ entsteht nichts unter `<draußen>`.
- `HOECHSTENS_EINTRAEGE` begrenzt die Zahl der Einträge, und eine neue Quelle muss genau ihre angekündigte Größe liefern (`quelle_schreiben`).

### Hauptfaden, lokale und ferne Wege, Oberfläche

Kein Archivzugriff auf dem Hauptfaden: Einstieg am Namen, Register ohne Systemaufruf, jedes Lesen und jeder Umbau hinter `&Fernfaden`. `Lesevorgang::starten` steht wörtlich, und `ort_passt` antwortet lokal und fern wie vorher. Jeder neue Schlüssel steht in `de.rs`, `fr.rs` und `en.rs`, mit gleichen Platzhaltern.

Kleinbefunde:

- Issue `261017-1923-ein-abwurf-auf-eine-ordnerzeile-im-archivfenster-landet-im-angezeigten-ordner.md`.
- Issue `261017-1923-eine-verknuepfung-auf-ein-archiv-wird-nur-an-ihrem-eigenen-namen-erkannt.md`: Abweichung von E8, dazu eine Lücke im Doc-Kommentar der Einstiegsprobe.
- Issue `261017-1923-der-text-archiv-nur-lesbar-steht-als-kleingeschriebener-satzrest-in-der-statuszeile.md`.
- Issue `261017-1923-ein-archivtab-faellt-nicht-auf-den-naechsten-vorhandenen-ordner-zurueck-wenn-sein-ordner-wegfaellt.md`.

## Übergreifende Beobachtungen

- **Die Proben halten das Ersetzen, nicht das, was danach geschieht.** `archiv_ersetzen_ist_der_einzige_schreiber_an_einer_archivdatei` liest allein `archiv/`. Die drei Wege, die nach einem gelungenen Umbau Daten entfernen, liegen alle in `operation/archiv.rs`:
  - `baum_entfernen` der lokalen oder fernen Quelle (Critical, Medium);
  - der zweite Umbau des Quellarchivs (Medium);
  - das Verwerfen verdeckter Einträge (Entscheid).

  Keiner davon fragt, ob das Entfernte das Kopierte ist. Eine gemeinsame Regel, „entfernt wird nur, was nachweislich unverändert am Ziel steht, gemessen an Gerät, Inode und Kennung“, schlösse alle drei. Dieselbe Regel passt auf `ueber_die_grenze` aus dem sftp-Paket.
- **Gleichheit am Pfadtext.** `hinein` vergleicht `quelle.datei() == ziel.datei()`, und das Register schlüsselt nach dem Pfad, wie er im Tab steht. Für das Lesen kostet eine zweite Schreibweise nur ein zweites Verzeichnis (E2). Für das Verschieben ist es die Vorbedingung des Critical-Befunds in seiner dritten Gestalt (`inference:`).
- **Zwei Entscheide stehen im Speicher noch auf `open`**, obwohl der Bau auf ihren Empfehlungen ruht: zu `Ort::Archiv` und zu tar. Der Plan verlangt sie beantwortet vor Schritt 1 und 2. Das ist Buchführung und kein Code. Ich nenne es für den nächsten Abgleich.

## Reihenfolge

1. **Vor jeder Auslieferung:** Critical (Selbstverschieben) und High (harte Verweise).
2. **Vor dem Nutzertest am Bündel:** Entscheid zu den verdeckten Einträgen, dazu die drei Medium-Befunde zu Datenverlust und Speicher (Kennung beim Entfernen, Quellbaum, Kopfgröße).
3. **Danach:** 4 GiB im Zip und die Low-Befunde.
