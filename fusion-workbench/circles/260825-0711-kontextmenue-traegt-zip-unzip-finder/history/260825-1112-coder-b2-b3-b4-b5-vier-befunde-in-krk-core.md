# Coder: B2, B3, B4 und B5 — vier Befunde der ersten Durchsicht in `krk-core`

**Datum:** 2026-08-25 11:12
**Status:** Complete
**Agent:** coder
**Baumstand:** `428fbc4` plus die Änderungen dieses Schritts und die parallel laufende Arbeit eines zweiten Coders in `krk-ui`

## Auftrag

Vier Befunde der ersten Durchsicht dieser Runde,
`reviews/260825-0942-coderev-runde-17-zip-unzip-und-die-regel-des-kontextmenues.md`, alle vier in
`krk-core`. `crates/krk-ui/` war für diesen Schritt gesperrt: ein zweiter Coder arbeitete dort
gleichzeitig an `kommandos/kontextmenue.rs` und `operationen.rs`.

## B3 — „Überschreiben" bedeutete beim Packen etwas anderes als beim Entpacken

`issues/260825-0942_*_ueberschreiben-loescht-beim-packen-endgueltig-und-beim-entpacken-in-den-papierkorb.md`

Der Datensatz trägt am Fuß die **Antwort des Nutzers**: Möglichkeit 1. `zippen::lauf` bekommt den
`Papierkorb` gereicht und nimmt ihn wie `entpacken`; `loeschen::baum_entfernen` fällt aus diesem
Zweig. Umgesetzt in `crates/krk-core/src/operation/zippen.rs` (`lauf` und `zielarchiv_klaeren`
nehmen `&dyn Papierkorb`, der Zweig `Konfliktantwort::Ueberschreiben` ruft
`papierkorb.in_den_papierkorb(ziel)`) und `crates/krk-core/src/operation/mod.rs` (`ausfuehren`
reicht den `Papierkorb` in beide Bahnen; die Leitung fehlte bisher ganz). Der `use`-Zweig auf
`loeschen` ist damit aus `zippen.rs` gefallen.

**Dazu die Zusage, die der Nutzer der Antwort mitgegeben hat**: angetastet wird allein der Eintrag,
dessen Name dem Archivnamen genau gleicht. Sie stand nirgends und hielt nur durch den Zufall, dass
beide Löschstellen auf `ziel` liegen. Sie steht jetzt als eigener Abschnitt im Modulkopf von
`zippen.rs` („Angetastet wird allein der Eintrag, der genau so heißt wie das Archiv") und noch
einmal kurz an `zielarchiv_klaeren`, und sie wird von einer eigenen Probe gehalten:
`ueberschreiben_raeumt_allein_den_gleichnamigen_eintrag_in_den_papierkorb`.

Die Probe nimmt den Fall, der den Befund gefährlich machte: am Archivnamen steht ein **Ordner mit
Inhalt**. Sie belegt drei Dinge in einem Zug — der gleichnamige Eintrag geht in den Papierkorb und
liegt dort vollständig (`baum_entfernen` hätte ihn zerlegt), der Nachbar ohne die Endung `.zip`
bleibt unangetastet, und dieser Nachbar ist zugleich die Quelle des Laufs, die dieser Zweig nie
anfassen darf.

**Die Papierkorbattrappe der Proben hat dafür eine zweite Fassung bekommen.**
`Papierkorbattrappe::default` schreibt weiterhin nur mit und lässt den Eintrag stehen — das ist
überall dort die stärkere Aussage, wo zu belegen ist, dass der Kern nicht selbst gelöscht hat.
`Papierkorbattrappe::raeumend` hängt den Eintrag zusätzlich per `fs::rename` in eine Ablage um. Der
Packlauf braucht das: er legt seine Zieldatei an der Stelle an, an der eben noch der weggeräumte
Eintrag stand, und ein stehen gebliebener Ordner ließe `File::create` scheitern. Was in der Ablage
ankommt, ist vollständig da, und genau daran ist abzulesen, dass kein rekursives Löschen im Spiel
war.

`die_regel_ueberschreiben_ersetzt_ein_vorhandenes_archiv` ist auf denselben Weg umgestellt: sie lief
über `OhnePapierkorb` und wäre nach der Änderung rot geworden. Sie belegt jetzt den Papierkorbweg
für die Datei am Archivnamen.

Der überholte Querverweis im Modulkopf von `entpacken.rs` — „nicht über `baum_entfernen`, das der
Zip-Lauf für seine einzelne Zieldatei nimmt" — ist nachgezogen; dort steht jetzt, dass der Packlauf
seit dem 260825 denselben Weg nimmt.

**Nicht angefasst** ist der Nebenbefund des Datensatzes zu `UmbenennenIn` und zum selbst getippten
Namen: er ist älter als diese Runde, gilt für das Kopieren ebenso und trägt keine Antwort des
Nutzers.

## B2 — das Packen hing an einer benannten Röhre mit Schreiber

`issues/260825-0942_*_das-packen-haengt-an-einer-benannten-roehre-mit-schreiber-und-die-probe-kann-es-nicht-sehen.md`

Die Wurzel ist behoben wie vorgeschlagen: `datei_packen` fragt nach dem Öffnen `metadata()` **am
offenen Deskriptor** und lässt jeden Eintrag aus, den `is_file()` nicht bejaht — mit dem Grund
„keine gewoehnliche Datei" in der Abschlussliste. Die Frage steht **vor** `start_file`, damit ein
ausgelassener Eintrag auch keine leere Zeile im Archiv bekommt.

`crates/krk-core/src/verzeichnis/sys.rs` ist unberührt geblieben, die Ausnahme
`#![allow(unsafe_code)]` nicht erweitert. Der Doku-Kommentar an `ohne_warten_oeffnen` ist dort aber
nachgezogen: er sagte, beim Packen erreiche „nur eine Datei überhaupt das Öffnen", und meinte damit
`Typ::Datei` — das Auffangfach, das Röhren, Geräte und Sockel mitträgt.

**Die Probe ist aussagekräftig gemacht.** Die alte legte eine Röhre ohne Schreiber an und war grün,
gleich wie der Code aussah; sie bleibt als der leichtere Fall stehen, jetzt unter dem Namen „ohne
Schreiber" und mit der Aussage, warum sie den schwereren nicht treffen konnte. Daneben steht
`eine_benannte_roehre_mit_schreiber_haelt_das_packen_nicht_an`.

Zwei Entscheidungen daran, beide im Doku-Kopf der Probe ausgeschrieben:

- **Der Schreiber ist ein `O_RDWR` auf die Röhre und kein zweiter Prozess.** Ein Schreiber, der nur
  schreibend öffnete, bliebe seinerseits im `open` stehen, bis ein Leser kommt — die Probe hinge
  dann schon beim Aufbau. Ein Deskriptor, der beide Richtungen trägt, kehrt sofort zurück und zählt
  für die Röhre als Schreiber. Kein `unsafe`, kein Fremdaufruf.
- **Gewartet wird mit Frist.** Neuer Helfer `bericht_mit_frist` neben `bericht_abholen`: ein Lauf,
  der in einem `read(2)` steht, ließe sonst den ganzen Testlauf stehen, und ein stehender Testlauf
  benennt nichts.

**Gegenprobe gefahren**: mit entfernter Typfrage fällt die Probe nach zwei Sekunden mit „das Packen
haengt an der Roehre; die Typfrage am Deskriptor fehlt". Der Fix ist danach wiederhergestellt.

Der Abschnitt „Gelesen wird ohne zu warten" im Modulkopf von `zippen.rs` behauptete einen Typfilter,
den es an dieser Stelle nicht gab; er heißt jetzt „Gelesen wird ohne zu warten, und der Typ wird am
Deskriptor gefragt" und trennt die zwei Hälften der Sperre.

## B4 — `Packschritt` trug kein `#[must_use]`

`issues/260825-0942_*_packschritt-traegt-kein-must-use-obwohl-dieselbe-runde-ablauf-genau-dafuer-markiert-hat.md`

Die Marke steht am Typ, mit der Begründung daneben und einem Verweis auf `Ablauf`, damit die zwei
als Paar lesbar bleiben. `Zielentscheid` (`operation/mod.rs`) hat sie im selben Zug bekommen, wie
der Befund es nebenbei vorschlägt: er entscheidet, ob überhaupt geschrieben wird. Beide ohne Folge
für den Bau — alle Rückgaben wurden schon ausgewertet.

## B5 — zwei Prosastellen in `entpacken.rs` sagten das Gegenteil der Probe

`issues/260825-0942_*_zwei-prosastellen-in-entpacken-rs-sagen-enclosed-name-weise-einen-absoluten-pfad-ab-die-probe-belegt-das-gegenteil.md`

Nur Prosa, kein Code und keine Probe geändert. Beide Stellen trennen jetzt die zwei Ausgänge: ein
Name, der über `..` aus dem Zielordner herausführte, liefert `None` und wird ausgelassen; ein
führender Schrägstrich wird abgestreift, und der Eintrag entsteht im Zielordner statt in der Wurzel.
Sicher ist beides, ausgelassen ist nur das erste. Der Modulkopf nennt dafür die Probe
`ein_eintrag_der_aus_dem_zielordner_herausfuehrt_entsteht_nirgends`, die beide Ausgänge
ausschreibt; der Kommentar im Rumpf verweist auf den Modulkopf statt die Aussage zu wiederholen.

## Geänderte Dateien

- `crates/krk-core/src/operation/zippen.rs` — B2, B3, B4
- `crates/krk-core/src/operation/mod.rs` — B3, B4
- `crates/krk-core/src/operation/entpacken.rs` — B5 und der B3-Folgeverweis, nur Prosa
- `crates/krk-core/src/verzeichnis/sys.rs` — B2, nur Prosa
- `crates/krk-core/tests/operation.rs` — B2 und B3

## Abnahme

`make check` — Rückgabewert 0, „alle vier gruen"; `crates/krk-core/tests/operation.rs` läuft mit 56
Proben durch, zwei mehr als vorher. Zusätzlich `cargo test -p krk-core` (alle Ziele grün) und
`cargo clippy -p krk-core --all-targets -- -D warnings` (Rückgabewert 0). `cargo doc -p krk-core`
meldet keine neue Warnung in den vier angefassten Dateien.

## Nicht getan

Nichts unter `crates/krk-ui/`. Keine Änderung dort war nötig: `zippen::lauf` ist `pub(crate)`, und
`operation::starten` nimmt den `Papierkorb` schon seit jeher entgegen. Kein Commit.
