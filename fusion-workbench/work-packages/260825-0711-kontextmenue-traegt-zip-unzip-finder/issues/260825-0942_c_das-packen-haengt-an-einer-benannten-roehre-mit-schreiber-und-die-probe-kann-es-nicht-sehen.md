Das Packen haengt an einer benannten Roehre mit Schreiber, und die Probe kann es nicht sehen

---

`zippen::datei_packen` oeffnet jede Quelle ueber `sys::ohne_warten_oeffnen` und liest sofort los. Die Huelle nimmt `O_NONBLOCK` jedoch wieder ab, bevor sie den Deskriptor herausgibt, und `datei_packen` fragt danach keinen Typ am Deskriptor. Eine benannte Roehre, an der ein Schreiber haengt, laesst `read(2)` deshalb unbegrenzt stehen; der Abbruch wird erst nach einem erfolgreichen `read` geprueft, also erreicht `Esc` den Lauf nicht. Der Modulkopf sagt das Gegenteil, und die vorhandene Probe legt eine Roehre **ohne** Schreiber an und kann den Fall nicht treffen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-core/src/operation/zippen.rs:36-42` — der Modulkopf: "mit `O_NONBLOCK` faellt sie an ihrem Typ heraus, statt den Vorgang ohne Meldung anzuhalten".
- `crates/krk-core/src/operation/zippen.rs:259-310` — `datei_packen` oeffnet und liest; zwischen `ohne_warten_oeffnen` und der Leseschleife steht keine Typfrage.
- `crates/krk-core/src/verzeichnis/sys.rs:842-849` — `ohne_warten_oeffnen` ruft `blockierend_stellen` selbst, nimmt `O_NONBLOCK` also ab. Der Doc-Kommentar derselben Datei (`:826-833`) schreibt aus, dass die Typpruefung Sache des Aufrufers bleibt.
- `crates/krk-core/src/operation/mod.rs:449-460` — `typ_und_groesse` legt jeden Eintrag, der weder Ordner noch Verknuepfung ist, in `Typ::Datei`. Benannte Roehren, Zeichen- und Blockgeraete und Sockel liegen damit im selben Fach wie eine gewoehnliche Datei.
- `crates/krk-core/tests/operation.rs`, `eine_benannte_roehre_im_ordner_haelt_das_packen_nicht_an` — `mkfifo` und sonst nichts.

## Gemessen

Am 260825 auf diesem Geraet, mit einer eigenstaendigen Fassung der drei Schritte der Huelle (`open` mit `O_NONBLOCK`, `F_GETFL`/`F_SETFL` ohne `O_NONBLOCK`, dann `read`):

- Roehre ohne Schreiber: `read` liefert 0, der Aufruf kehrt sofort zurueck. Das ist die Lage der Probe, und sie ist gruen, gleich wie der Code aussieht.
- Roehre mit einem Schreiber, der sie offen haelt und nichts hineinschreibt: `read` kehrt nach zwei Sekunden nicht zurueck und wird abgeschossen.

## Der Unterschied zu den zwei aelteren Aufrufern

`text::datei::lesen` und `text::datei::bis_zur_grenze_lesen` fragen nach dem Oeffnen `metadata()` am Deskriptor und weisen alles ab, was `is_file()` nicht bejaht. Genau diesen Schritt hat der Packlauf nicht. `entpacken` braucht ihn nicht: `ZipArchive::new` verlangt `Seek`, und eine Roehre scheitert dort mit einem Fehler, der in die Abschlussliste geht.

## Vorschlag

In `datei_packen` nach dem Oeffnen `metadata()` am Deskriptor fragen und den Eintrag ueberspringen, wenn `is_file()` nein sagt — mit dem Grund in der Abschlussliste, wie bei jeder anderen ausgelassenen Quelle. Die Probe bekommt einen zweiten Fall: eine Roehre mit einem Schreiber, der sie fuer die Dauer des Laufs offen haelt.

Der Modulkopf ist im selben Zug nachzuziehen: heute behauptet er einen Typfilter, den es an dieser Stelle nicht gibt. Ebenso der Doc-Kommentar in `verzeichnis/sys.rs:826-830`, der sagt, beim Packen erreiche "nur eine Datei ueberhaupt das Oeffnen" — er meint `Typ::Datei`, und das ist das Auffangfach.

## Umfang

`krk-core`, `operation/zippen.rs` und die zugehoerige Probe. Der Entpacklauf ist nicht betroffen.

---
Resolved: Wurzel behoben wie vorgeschlagen. `datei_packen`
(`crates/krk-core/src/operation/zippen.rs`) fragt nach dem Oeffnen `metadata()` **am offenen
Deskriptor** und laesst jeden Eintrag aus, den `is_file()` nicht bejaht — mit dem Grund
"keine gewoehnliche Datei" in der Abschlussliste und **vor** `start_file`, damit kein leerer
Eintrag im Archiv zurueckbleibt. `verzeichnis/sys.rs` ist unberuehrt geblieben, die Ausnahme
`#![allow(unsafe_code)]` nicht erweitert.

Die Probe ist aussagekraeftig gemacht: `eine_benannte_roehre_mit_schreiber_haelt_das_packen_nicht_an`
(`crates/krk-core/tests/operation.rs`) haengt einen Schreiber an die Roehre und haelt ihn ueber
den ganzen Lauf. Der Schreiber ist ein `O_RDWR` auf die Roehre und kein zweiter Prozess: ein
nur schreibendes `open` bliebe seinerseits stehen, bis ein Leser kommt, und die Probe haenge
schon beim Aufbau. Gewartet wird mit Frist (neuer Helfer `bericht_mit_frist`), damit ein
Rueckfall den Befund meldet statt den Testlauf stehen zu lassen. Gegenprobe gefahren: mit
entfernter Typfrage faellt sie nach 2 s mit "das Packen haengt an der Roehre; die Typfrage am
Deskriptor fehlt". Die alte Probe bleibt als der leichtere Fall stehen, jetzt mit dem Namen
"ohne Schreiber" und der Aussage, warum sie den schwereren nicht treffen konnte.

Beide Prosastellen sind nachgezogen: der Abschnitt "Gelesen wird ohne zu warten" im Modulkopf
von `zippen.rs` und der Doku-Kommentar an `ohne_warten_oeffnen`
(`crates/krk-core/src/verzeichnis/sys.rs`), der sagte, beim Packen erreiche "nur eine Datei
ueberhaupt das Oeffnen".
