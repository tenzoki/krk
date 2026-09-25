# T2 — vier Befunde niedriger Schwere: drei Prosakorrekturen und ein Attribut

**Datum:** 260817-1302
**Agent:** coder
**Status:** Complete
**Datensätze:** `issues/260817-1109_*`, `issues/260817-1110_*`, `issues/260817-1111_*`,
`issues/260817-1112_*` — alle vier auf `_c_` gesetzt, jeder mit seinem `Resolved:`-Vermerk
**Durchsicht:** `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`,
Befunde 4, 5, 6 und 7
**Baumstand vorher:** `3fcd375` mit den Änderungen von T1

---

## Was umgesetzt ist

Kein Verhalten geändert. Drei Korrekturen an Prosa und ein `#[must_use]`.

`crates/krk-ui/src/kommandos/loeschwarnung.rs`

- Modulkopf, Abschnitt „Der eine Aufrufer": die Aufzählung nennt jetzt „die beiden Tasten
  `delete` und `cmd+delete` und der Menueeintrag ‚In den Papierkorb raeumen'". Der Melder der
  Bereichsleiste ist heraus (Befund 4).
- Zwei neue Absätze darunter, damit der nächste Leser die falsche Quelle nicht wieder
  aufnimmt: die Bereichsleiste ist keiner dieser Wege — sie schickt zehn Kommandos, alle
  Umschalter, und `Kommando::InPapierkorb` ist keines davon —, und `f8` kommt erst mit
  Bündel D dazu, weil es heute `Kommando::EndgueltigLoeschen` trägt.
- `frage_und_erlaeuterung` trägt `#[must_use]` mit ausgeschriebener Begründung nach dem
  Vorbild von `rueckschritt.rs:142-145`: die Funktion ist rein, ein Aufruf ohne Verwendung
  ihrer beiden Zeichenketten ist ein Aufruf ohne jede Wirkung, der Übersetzer sagt dazu von
  sich aus nichts, auch nicht unter `-D warnings`, und verlorenginge dabei die Rückfrage
  selbst (Befund 7).

`crates/krk-ui/src/appkit/anwendung.rs`

- `papierkorb_oder_zeichen_zurueck`, Punkt 1 des Doc-Kommentars: der Menüeintrag ist der eine
  Weg, der ohne Tastendruck ankommt; die Bereichsleiste steht ausdrücklich als kein zweiter
  solcher Weg, mit dem Grund (Befund 4).
- `loeschauftrag_stellen`, die Begründung für das Nachlesen der Fensterseite: ausgeschrieben,
  mit einem vorangestellten Satz, warum sie hier ausgeschrieben steht und nicht in der sonst
  üblichen Kurzform — die Zusage einer zerstörenden Handlung hängt daran. Genannt sind die
  vier durchgelassenen Kommandos mit ihren beiden Quellen und der Schluss einzeln je Befehl
  (Befund 6).
- `im_editor_oeffnen`, Kommentar im Zweig der leeren Auswahl: der Satz „es ist nichts
  ausgewählt" ist der, den KRK seit der Runde 1 führt; für den Löschweg trägt ihn seit dem
  260817 `loeschen_nach_rueckfrage` und nicht mehr `endgueltig_loeschen`, daneben stehen
  `auftrag_stellen` und `stapel_umbenennen` (Befund 5).
- `editormeldung_zeigen`, Doc-Kommentar: statt `endgueltig_loeschen` jetzt
  `loeschen_nach_rueckfrage` als der eine Rumpf jedes Löschbefehls, die beiden
  Operationsbefehle über `auftrag_stellen`, und ein Satz dazu, dass die Meldung bis zum
  260817 in `endgueltig_loeschen` stand — der Bezug auf `aktiv` hat sich mit ihr nicht
  geändert (Befund 5).

`crates/krk-ui/src/appkit/blaetter/mod.rs` — die eine Zeile, die `260817-1111` freigibt

- Die Begründung an `Taste`, warum die Schaltflächen des Blattes weiter auslösen: sie nennt
  jetzt die vier durchgelassenen Kommandos und dazu, warum ihr Schluss hält — keiner der drei
  zusätzlich zugelassenen Befehle liegt ab Werk auf einer Eingabetasten-Kombination des
  Blattes, sie liegen auf `cmd+q`, `shift+cmd+w` und `cmd+n`. Sonst nichts angefasst; T1 ist
  dort gerade gelandet.

`CLAUDE.md` ist **nicht** mitgezogen. Die Wahl war freigestellt; die Datei ist eine normative
Fläche, und die dort stehende Verengung ist im neuen Datensatz benannt.

## Nachgezählt statt übernommen

Drei Zahlen aus den Datensätzen weichen von der Zählung am Baum ab, und es gilt die Zählung:

1. **Die Bereichsleiste schickt zehn Kommandos und nicht elf.** `260817-1109` sagt elf, und
   seine eigene Aufstellung summiert sich auf zehn: fünf Bereiche (`:164-168`), drei Spalten
   (`:182-184`), die Tiefe (`:195`), der Inhalt (`:214`). `grep -c 'Kommando::'
   crates/krk-ui/src/appkit/bereichsleiste.rs` zählt zehn. Am Befund ändert es nichts: alle
   zehn sind Umschalter.
2. **Der Satz „es ist nichts ausgewählt" steht an vier Stellen und nicht an zwei.**
   `260817-1110` nennt `loeschen_nach_rueckfrage` (`:4622`) und `auftrag_stellen` (`:5093`);
   dazu tragen ihn `stapel_umbenennen` (`:4893`) und `im_editor_oeffnen` (`:5530`) selbst.
3. **Die verkürzte Blattsperre steht an mehr Stellen, als `260817-1111` nennt.** Zwei weitere
   Träger sind Befunde derselben Klasse (`anwendung.rs:2840`, `editor.rs:1298`), drei weitere
   Stellen mit denselben Worten sind **keine** (`belegung.rs:640`, `belegung.rs:955`,
   `anwendung.rs:405` — die ersten zwei sagen etwas über `waehrend_blatt_erlaubt` und nicht
   über die ganze Sperre, die dritte ist eine Aussage über den Stand bis S16).

Am Baum nachgelesen und nicht aus den Datensätzen übernommen: `endgueltig_loeschen` trägt
keine Prüfung der leeren Auswahl mehr, `immer_erreichbar` führt genau drei Kommandos,
`waehrend_blatt_erlaubt` genau eines, und die drei Bodies `beenden`, `fenster_schliessen` und
`fenster_zeigen` schreiben `aktiv` nicht — `aktiv_setzen` wird von keinem gerufen.

## Neuer Datensatz

`issues/260817-1302_o_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-und-der-datensatz-nennt-sie-nicht.md`
— die zwei Träger, die `260817-1111` nicht nennt, mit der Begründung für den Aufschub, den
drei Nicht-Befunden und der zweiten Verengung in `CLAUDE.md:123` (die Aufzählung der vier
Bestandteile von `zulaessigkeit::zulaessig` nennt `immer_erreichbar` gar nicht, obwohl es drei
der vier aufhebt).

## Abnahme

`make check` — exit 0. Vier Kommandos grün: Bau, 1250 Proben über den Workspace, `fmt
--check`, `clippy --all-targets -- -D warnings`.

Zusätzlich `cargo doc -p krk-ui --document-private-items --no-deps`: keine der neu
eingefügten Doc-Verweisungen ist unaufgelöst. Die 67 `broken_intra_doc_links`-Warnungen der
Kiste bestehen vor und nach dieser Arbeit — sie kommen daraus, dass `krk-ui` kein
Bibliotheksziel hat; keine liegt in einer angefassten Zeile.
