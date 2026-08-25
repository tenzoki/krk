# Codedurchsicht: Runde 17, Zip, Unzip und die Regel des Kontextmenues

**Reviewed-range:** `428fbc4..423d5f2`
**Not-opened:** none

**Geoeffnet:** die vier codetragenden Commits `9af13ba`, `b0841ba`, `ab74c9e` und `423d5f2`, jeder als Unterschied und jede neue Datei zusaetzlich am Baumstand. Die zwei uebrigen Commits des Bereichs, `d63d07b` und `fe1aff5`, sind als Unterschied **nicht** geoeffnet; nachgesehen ist, dass sie ausserhalb von `fusion-workbench/` keine Datei anfassen (`git diff --name-only 428fbc4..fe1aff5` liefert dort nichts). Gelesen sind stattdessen ihre Erzeugnisse: der Plan, die fuenf Entscheidungsdatensaetze und der Abschnitt „Entscheidungen des Nutzers zu Runde 17" der Sitzungsgeschichte. Von den vierzehn geaenderten Codedateien ist jede geoeffnet; in `crates/krk-core/tests/operation.rs` (1043 neue Zeilen, 26 neue Proben) sind die Namen aller Proben gelesen und die Rueempfe der zwoelf, auf denen eine Zusage dieser Durchsicht haengt.

---

## Zusammenfassung

Der Bau ist sorgfaeltig, faehrt gruen (`cargo clippy --workspace --all-targets -- -D warnings` und `cargo test --workspace` am 260825 hier gefahren, beide Exit 0) und haelt die fuenf Nutzerentscheidungen dort, wo er sie ueberhaupt schon beruehrt. Fuenf Befunde stehen dagegen, und die zwei schwersten haben dieselbe Wurzel: eine Angabe wird von einer Stelle als vertrauenswuerdig behandelt, die sie nicht geprueft hat. Beim Entpacken ist es der Zielordnername, den die Oberflaeche aus einem fremden Dateinamen rechnet; beim Packen ist es der Typ einer Quelle, den `Typ::Datei` nur scheinbar zusagt.

## Summen

| Schwere | Zahl |
|---|---|
| Kritisch | 1 |
| Hoch | 2 |
| Mittel | 1 |
| Gering | 1 |

## Was gepruefte Zusagen haelt

Vorweg, weil es den Rest einordnet:

- **Die vier ausdruecklich genannten Bindungen halten.** Der Archivname haengt die Endung an und ist mit `ordnername_zum_archiv` umkehrbar (belegt von `archivname_und_ordnername_kehren_einander_um` ueber vier Namensgestalten). Ein Archiv wird ohne Dateizugriff an der Endung erkannt, ohne Ruecksicht auf die Schreibung. Beim Entpacken geht der vorhandene Zielordner ueber die `Papierkorb`-Schnittstelle und nicht ueber ein `remove_dir`; die Probe `ueberschreiben_raeumt_den_vorhandenen_zielordner_in_den_papierkorb` beweist es mit einer Attrappe, die nichts loescht, sondern mitschreibt, und prueft danach, dass der alte Inhalt **noch dasteht** — das ist die staerkere Aussage und nicht die bequeme. Ein Vorgang traegt mehrere Archive mit je einem Ziel, und der Konflikt wird je Archiv gefragt.
- **Die Zusage L9 haelt.** `operation::starten` legt fuer jede Art einen Arbeitsfaden an (`crates/krk-core/src/operation/mod.rs:145-162`), und beide neuen Laeufe haengen darunter. Das neue Modul `kommandos/kontextmenue` fasst kein Dateisystem an: `ist_zipname`, `archivname`, `ordnername_zum_archiv` und `entpackziel` rechnen auf Zeichenfolgen und auf `Ordnermodell::zeilen`, also auf dem Bestand im Speicher.
- **Die `expect`-Marken verfallen wirklich.** Nachgestellt in einem Wegwerf-Workspace: ein `#![cfg_attr(not(test), expect(dead_code, …))]` am Modul bleibt erfuellt, solange **ein** Stueck darin unbenutzt ist, und meldet `unfulfilled lint expectation`, sobald keines mehr uebrig ist. `make lint` faehrt `-D warnings`, also haelt der Bau an. Die vier `expect` an den einzelnen Funktionen in `operationen.rs` verfallen je einzeln und damit frueher. Der Modulkopf von `kontextmenue.rs` beschreibt genau dieses Verhalten („erlischt in dem Augenblick, in dem das letzte Stueck einen Aufrufer bekommt") und ist damit richtig, nicht bloss ungefaehr richtig. **Der Preis steht nicht dabei:** die Fassung am Modul kann ein einzelnes tot gebliebenes Stueck nicht anzeigen, solange ein zweites noch tot ist.
- **Die Kiste kommt sauber herein.** `Cargo.lock` waechst um genau zwei Pakete, `zip 8.6.0` und `typed-path 0.12.3`; `crc32fast` bleibt auf 1.5.0, und der Nachtrag zu Schritt 1 hat die eine Aussage der Planerhebung, die im Baum nicht hielt, im Manifest berichtigt statt sie abzuschreiben. `cargo tree --workspace -e normal,build` nennt weder `cc` noch einen `-sys`-Namen. Beide Eintraege tragen ihre Begruendung in der Wurzel-`Cargo.toml`, und `flate2` traegt den Satz, der es vor dem naechsten Aufraeumen schuetzt.
- **Kein `unsafe` in den drei neuen Dateien**, und die Grenze aus `#![deny(unsafe_code)]` bleibt, wo sie war.

## Befunde nach Themen

### Thema 1: eine Angabe aus fremder Hand wird ungeprueft zum Pfad

**B1 — Ein Archivname aus Punkten macht den angezeigten Ordner oder seinen Elternordner zum Entpackziel. Kritisch.**
`crates/krk-ui/src/kommandos/kontextmenue.rs:343-353` und `:420-423`.

`ordnername_zum_archiv` gibt den Stamm aus `namen_teilen` unveraendert heraus, und `paar` macht daraus `ordner.join(stamm)`. `namen_teilen` trennt am letzten Punkt, sofern er nicht an Stelle 0 steht, also:

```
a.zip     -> Stamm "a"    -> <ordner>/a
..zip     -> Stamm "."    -> <ordner>/.
...zip    -> Stamm ".."   -> <ordner>/..
```

`PathBuf::join` normalisiert nichts, `fs::symlink_metadata("<ordner>/..")` trifft den Elternordner, und `zielordner_klaeren` behandelt ihn als vorhandenen Zielordner. Wer im Blatt „Ueberschreiben" waehlt, gibt damit den angezeigten Ordner oder dessen Elternordner an den Papierkorb; wer es nicht waehlt, bekommt den Archivinhalt eine Ebene zu hoch geschrieben. Alle drei Namen sind auf macOS anlegbar; nachgerechnet am 260825 mit einer eigenstaendigen Fassung der drei Funktionen.

**Warum die zwei gebauten Sperren das nicht sehen.** Die Frage der Durchsicht war, ob `enclosed_name` und `kette_anlegen` jeden Weg aus dem Zielordner heraus schliessen. Sie tun es, und sie tun es gruendlich (Thema 2). Beide arbeiten aber relativ zu dem `ziel`, das ihnen gereicht wird, und dieses `ziel` ist die eine Angabe des Weges, die aus einem fremden Dateinamen stammt und keine Pruefung durchlaufen hat. Der Ausbruch geschieht vor den Sperren, nicht an ihnen vorbei.

`krk_core::operation::umbenennen::name_pruefen` weist `.` und `..` schon heute mit `Namensfehler::Punktname` ab und wuerde auch den Namen `␣␣` als `Leer` abweisen, den `ist_zipname` fuer `␣␣.zip` durchlaesst. Gerufen wird es auf diesem Weg nicht. Der Vorschlag steht im Defektdatensatz: eine Pruefung in `paar`, an genau einer Stelle.

Ausloesen kann es heute niemand, weil die Schritte 6 und 7 fehlen. Die Regel ist trotzdem gebaut, und sie ist die Rechnung, die Schritt 7 verwenden wird.

Datensatz: `issues/260825-0942_o_ein-archivname-aus-punkten-macht-den-angezeigten-ordner-oder-seinen-elternordner-zum-entpackziel.md`

### Thema 2: die zwei Ausbruchswege des Archivs sind versperrt

**Kein Befund, und der Prueflauf gehoert trotzdem ins Protokoll.** Der Nachtrag zu Schritt 3 meldet die Abweichung selbst, und sie haelt:

- `enclosed_name` faengt den Namen, der fuer sich genommen hinausfuehrt. Nicht gefangen, sondern **gekuerzt** wird der fuehrende Schraegstrich; der Eintrag landet im Zielordner. Das ist sicher und von der Probe belegt — die Prosa daneben sagt etwas anderes, siehe B5.
- `kette_anlegen` (`crates/krk-core/src/operation/entpacken.rs:315-344`) prueft jeden Ordner auf dem Weg mit `lstat` und weist eine Verknuepfung ab. Der Zweig fuer eine Komponente, die kein blosser Name ist, steht daneben, obwohl `enclosed_name` sie schon ausgeschlossen hat, und begruendet sich selbst.
- Der letzte Bestandteil ist eigens gedeckt: eine Datei, an deren Ziel schon eine Verknuepfung steht, wird uebersprungen (`entpacken.rs:261-271`); ein Ordnereintrag geht durch `kette_anlegen`, weil die Kette bei ihm den Eintrag selbst einschliesst.
- Die zwei Proben pruefen, was sie behaupten. Die erste zaehlt die ausgelassenen Eintraege und prueft den Ort ausserhalb wie den Ort innerhalb; die zweite baut die zweistufige Falle (`hinaus -> ..`, dann `hinaus/draussen.txt`) wirklich nach und prueft danach, dass die Verknuepfung selbst im Ergebnis steht und nur der Weg durch sie versperrt ist.
- Die Rechte kommen mit `& 0o777` an, die oberen Modusbits fallen also ab. Ein setuid-Bit aus fremdem Archiv erreicht das Dateisystem nicht.

**Verknuepfungen, beide Richtungen.** Beim Packen ist `add_symlink` genommen, und die Probe prueft das Typfeld (`& 0o170_000 == 0o120_000`) und den Inhalt, nicht bloss die Existenz. Der Modulkopf schreibt aus, warum `unix_permissions(0o120777)` es nicht tut. Beim Entpacken kommt eine Verknuepfung als Verknuepfung heraus, mit einer Laengengrenze von 1024 Bytes gegen eine erfundene Groessenangabe im Archiv. Eine Verknuepfung auf den eigenen Ordner laesst den Packlauf enden, und die Probe fuehrt die drei erwarteten Eintraege namentlich auf.

### Thema 3: der Typ einer Quelle wird zugesagt, aber nicht geprueft

**B2 — Das Packen haengt an einer benannten Roehre mit Schreiber, und die Probe kann es nicht sehen. Hoch.**
`crates/krk-core/src/operation/zippen.rs:36-42` und `:259-310`.

`ohne_warten_oeffnen` oeffnet mit `O_NONBLOCK` und nimmt es vor der Rueckgabe wieder ab (`crates/krk-core/src/verzeichnis/sys.rs:842-849`). Der Schutz, den die Huelle liefert, ist damit allein das nicht blockierende **Oeffnen**; das anschliessende Lesen blockiert wieder, und die Typfrage bleibt beim Aufrufer, wie der Doc-Kommentar derselben Datei ausschreibt. Die zwei aelteren Aufrufer in `text/datei.rs` stellen sie und weisen alles ab, was `is_file()` nicht bejaht. `datei_packen` stellt sie nicht.

`typ_und_groesse` (`crates/krk-core/src/operation/mod.rs:449-460`) legt jeden Eintrag, der weder Ordner noch Verknuepfung ist, in `Typ::Datei`; benannte Roehren, Geraetedateien und Sockel liegen im selben Fach. Gemessen am 260825, mit einer eigenstaendigen Fassung der drei Schritte:

| Lage | `read(2)` |
|---|---|
| Roehre ohne Schreiber | liefert 0, kehrt sofort zurueck |
| Roehre mit einem Schreiber, der nichts schreibt | kehrt nach zwei Sekunden nicht zurueck |

Die Probe `eine_benannte_roehre_im_ordner_haelt_das_packen_nicht_an` legt die Roehre mit `mkfifo` an und haengt keinen Schreiber daran. Sie faehrt damit ausschliesslich die obere Zeile und bleibt gruen, gleich wie `datei_packen` aussieht. Da der Abbruch erst nach einem erfolgreichen `read` geprueft wird, erreicht `Esc` einen so haengenden Lauf nicht.

Der Entpacklauf ist nicht betroffen: `ZipArchive::new` verlangt `Seek`, und eine Roehre scheitert dort mit einem Fehler, der in die Abschlussliste geht. Genau das sagt der neue Abschnitt in `verzeichnis/sys.rs` auch — er sagt daneben, beim Packen erreiche „nur eine Datei ueberhaupt das Oeffnen", und meint dabei `Typ::Datei`, das Auffangfach.

Datensatz: `issues/260825-0942_o_das-packen-haengt-an-einer-benannten-roehre-mit-schreiber-und-die-probe-kann-es-nicht-sehen.md`

### Thema 4: derselbe Knopf, zwei Bedeutungen

**B3 — „Ueberschreiben" loescht beim Packen endgueltig und beim Entpacken in den Papierkorb. Hoch.**
`crates/krk-core/src/operation/zippen.rs:155-165` gegen `crates/krk-core/src/operation/entpacken.rs:170-182`.

Der Packlauf ruft `loeschen::baum_entfernen`, also ein rekursives `remove_file`/`remove_dir` ohne Papierkorb (`crates/krk-core/src/operation/loeschen.rs:101-110`). Sein Ziel ist ein Pfad, den die Oberflaeche aus einem Namen bildet; steht dort ein **Ordner** namens `Projekte.zip`, geht der ganze Baum darunter unwiederbringlich weg. `zielarchiv_klaeren` unterscheidet Datei und Ordner nicht, es fragt allein, ob etwas dasteht.

Der Mechanismus ist alt — `ziel_klaeren` nimmt fuer das Kopieren denselben Weg —, aber dort verschmelzen zwei gleichnamige Ordner vorher, sodass der Baumloescher selten drankommt. Neu ist, dass der Packlauf sein Ziel immer als Datei anlegt, und neu ist vor allem, dass dieselbe Runde nebenan die entgegengesetzte Antwort gewaehlt und mit der Runde-12-Bindung begruendet hat. `zippen::lauf` bekommt den `Papierkorb` nicht einmal gereicht (`crates/krk-core/src/operation/mod.rs:177`), die Leitung fehlt also.

Die vierte Nutzerentscheidung legt „Ueberschreiben" im gekuerzten Blatt auf `cmd+Eingabe`, also einen Anschlag. Ob die Bindung der Runde 12 diesen Zweig mitentscheidet, ist eine Nutzerfrage; sie ist mit dieser Runde zum ersten Mal an einer Stelle sichtbar, an der beide Antworten nebeneinander im Baum stehen.

Datensatz: `issues/260825-0942_o_ueberschreiben-loescht-beim-packen-endgueltig-und-beim-entpacken-in-den-papierkorb.md`

### Thema 5: Marken und Prosa

**B4 — `Packschritt` traegt kein `#[must_use]`, obwohl dieselbe Runde `Ablauf` genau dafuer markiert hat. Mittel.**
`crates/krk-core/src/operation/zippen.rs:60-74` gegen `crates/krk-core/src/operation/mod.rs:113-128`. Alle fuenf Rueckgaben werden heute ausgewertet; der Befund gilt der Zusage und nicht dem Stand. `Zielentscheid` traegt die Marke ebenfalls nicht und ist aelter.

Datensatz: `issues/260825-0942_o_packschritt-traegt-kein-must-use-obwohl-dieselbe-runde-ablauf-genau-dafuer-markiert-hat.md`

**B5 — Zwei Prosastellen in `entpacken.rs` sagen, `enclosed_name` weise einen absoluten Pfad ab; die Probe belegt das Gegenteil. Gering.**
`crates/krk-core/src/operation/entpacken.rs:49-53` und `:218-220`. Der fuehrende Schraegstrich wird gekuerzt, nicht abgewiesen; die Probe schreibt es aus und zaehlt zwei ausgelassene Eintraege von vieren. Das Verhalten ist sicher, die Beschreibung des Mechanismus ist falsch, und sie steht an der Stelle, an der der naechste Leser nachschlaegt, welche Sperre welchen Weg schliesst.

Datensatz: `issues/260825-0942_o_zwei-prosastellen-in-entpacken-rs-sagen-enclosed-name-weise-einen-absoluten-pfad-ab-die-probe-belegt-das-gegenteil.md`

## Der Abbruch, beide Richtungen

Geprueft und in Ordnung, mit einer Einschraenkung.

- **Packen.** `lauf` gibt nach `Packschritt::Abgebrochen` und nach `ArchivHin` an `halbes_archiv_wegraeumen` ab und kehrt nur bei einem erfolgreichen `finish()` vorher zurueck (`zippen.rs:104-123`); ein `NotFound` beim Wegraeumen ist ausgenommen. Die Probe bricht auf eine Wartezeit hin ab und prueft danach drei Dinge: den Abschluss, dass weniger Bytes gemeldet sind als die Datei traegt, und dass das Archiv nicht mehr dasteht. Die Zusage haelt.
- **Entpacken.** `datei_schreiben` raeumt allein die halbe Datei weg und laesst den Rest stehen. Die Probe bricht auf die **erste Fortschrittsmeldung ueber die grosse Datei** hin ab und haengt damit nicht an der Geschwindigkeit des Geraets; sie prueft danach die fertige Datei und die Abwesenheit der halben. Das ist die praezisere der zwei Proben.
- **Die Einschraenkung** ist B2: solange der Lauf in einem blockierenden `read` steht, erreicht ihn kein Abbruch, weil er zwischen zwei Lesungen geprueft wird.

## Was quer liegt

**Die zwei schweren Befunde teilen eine Form.** In beiden Faellen sagt ein Typ oder ein Name etwas zu, was er nicht geprueft hat, und die naechste Stelle verlaesst sich darauf: `Typ::Datei` sagt „Datei" und heisst „weder Ordner noch Verknuepfung"; ein Stamm aus `namen_teilen` sieht aus wie ein Dateiname und ist keiner. Beide Male steht die Pruefung, die fehlt, schon im Baum — `metadata()` am Deskriptor in `text/datei.rs`, `name_pruefen` in `operation/umbenennen.rs`. Es ist keine neue Maschine noetig, nur ein Ruf.

**Zwei Proben lassen sich von der bequemen Lage taeuschen, und die eine faellt auf.** Die Roehrenprobe faehrt den Fall, der ohnehin gutgeht, und traegt trotzdem den Namen der Zusage. Die uebrigen fuenfundzwanzig neuen Proben pruefen, was sie behaupten, und mehrere pruefen ausdruecklich auch das, was **nicht** entstehen darf. Der Unterschied ist nicht Sorgfalt, sondern Erreichbarkeit: eine Roehre mit Schreiber braucht einen zweiten Faden in der Probe.

**Die Prosa dieser Runde ist ungewoehnlich dicht und an zwei Stellen zu weit vorgelaufen.** B5 und der Satz zum Typfilter in `zippen.rs` und `verzeichnis/sys.rs` beschreiben je einen Schutz, der so nicht gebaut ist. Beide sind waehrend des Baus entstanden, als die Kiste sich anders verhielt als der Plan annahm; die Nachtraege am Plan halten die Abweichungen fest, die Modulkoepfe sind an diesen zwei Stellen nicht mitgezogen.

**Eine Beobachtung ohne Datensatz.** `Auftrag::entpackziel` (`crates/krk-core/src/operation/auftrag.rs:186-191`) verzweigt mit einem Auffangzweig `_ => None`. Das steht der Bindung „vollstaendige Fallunterscheidungen ohne Auffangzweig" entgegen, aber der unmittelbar darueber stehende `neuer_name` tut seit jeher dasselbe, und `None` ist fuer jede andere Art die richtige Antwort. Kein Defekt, sondern eine Stelle, an der eine kuenftige Art still durchfaellt — genannt, damit sie beim naechsten Zuwachs von `Art` mitgelesen wird.

## Reihenfolge

1. **B1** vor Schritt 7. Sobald der Ausfuehrungszweig steht, ist der Weg begehbar, und sein Ausgang ist ein Ordner im Papierkorb, den niemand dorthin geschickt hat. Die Aenderung ist ein Ruf in `paar`.
2. **B3** vor der Abnahme der Runde, weil sie eine Nutzerantwort braucht und nicht bloss eine Zeile. Solange sie offen ist, bedeuten zwei Eintraege desselben Menues mit demselben Blatt zweierlei.
3. **B2** vor dem Rundenabschluss. Der Fall ist selten, seine Wirkung ist ein Vorgang, den der Nutzer nicht mehr anhalten kann.
4. **B4** und **B5** sind Aufraeumen und koennen mit dem naechsten Schritt derselben Dateien mitlaufen.
