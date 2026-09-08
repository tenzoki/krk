Fuenf oeffentliche Namen der zwei Module haben keinen Rufer ausser hoechstens ihrer eigenen Probe

---

`MELDEABSTAND`, `HOECHSTE_STELLENZAHL`, `Regel::ist_wirkungslos`, `Lauf::warten` und
`Abschluss::ist_abgebrochen` sind `pub` und teils ueber die Modulwurzel weitergereicht. Keiner von
ihnen hat einen Rufer in der Anwendung; drei haben gar keinen ausserhalb ihrer eigenen Datei, und
einer hat im ganzen Arbeitsbereich ueberhaupt keinen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die fuenf, mit Zaehlung

- **`MELDEABSTAND`** (`operation/fortschritt.rs:50`), weitergereicht in
  `operation/mod.rs:81`. `grep -rn --include='*.rs' MELDEABSTAND crates/` liefert fuenf Treffer,
  alle in `fortschritt.rs` und `mod.rs` selbst — zwei davon in Doc-Kommentaren. Kein Rufer in
  `krk-ui`, keiner in einer Probe.
- **`HOECHSTE_STELLENZAHL`** (`stapelumbenennen/regel.rs:38`), weitergereicht in
  `stapelumbenennen/mod.rs:59`. Fuenf Treffer, alle in `regel.rs` und `mod.rs`. Bemerkenswert
  dabei: das Blatt in `krk-ui/src/appkit/blaetter/stapelumbenennen.rs` begrenzt sein Eingabefeld
  **nicht** ueber diese Konstante, sondern nimmt jeden Text entgegen und laesst
  `Regel::aus_eingabe` einen `Regelfehler::Stellenzahl` liefern. Die Konstante ist also
  weitergereicht fuer einen Rufer, der sie nicht nimmt.
- **`Regel::ist_wirkungslos`** (`stapelumbenennen/regel.rs:97-99`). Zwei Treffer im ganzen
  Baum: die Zeile selbst und `regel.rs:193`, ihre eigene Probe. Sie beantwortet fuer die ganze
  Regel dieselbe Frage, die `Vorschauzeile::wird_umbenannt` (`vorschau.rs:38-40`) je Zeile
  beantwortet und die `Vorschau::auszufuehren` zum Filtern benutzt — eine Regel ist genau dann
  wirkungslos, wenn keine Zeile umbenannt wird. Zwei Antworten auf dieselbe Frage, von denen
  eine niemand ruft.
- **`Lauf::warten`** (`operation/fortschritt.rs:249-253`). Zehn Treffer, alle in
  `crates/krk-core/tests/operation.rs`. Der Doc-Kommentar nennt die Vorbedingung ("nur fuer
  Aufrufer, die den Kanal schon leergeraeumt haben"), die in der Anwendung niemand herstellt;
  dort haelt `Drop for Lauf` (`fortschritt.rs:256-265`) ausdruecklich **nicht** an. Der Name ist
  damit Pruefwerkzeug in einer oeffentlichen Schnittstelle.

- **`Abschluss::ist_abgebrochen`** (`operation/fortschritt.rs:64-66`). `grep -rn ist_abgebrochen
  crates/ --include='*.rs'` liefert drei Treffer im ganzen Arbeitsbereich: diese Zeile, die
  gleichnamige Methode eines **anderen** Typs in `verzeichnis/leser.rs:69`, und deren einzigen
  Rufer in `tests/verzeichnis.rs:155`. Die Methode dieses Umfangs hat also nicht einmal eine
  Probe. Die Oberflaeche kommt ohne sie aus und verzweigt ueber die Variante selbst.

  **Die zwei gleichnamigen Methoden sind ein zweiter Befund fuer sich.** `krk-core` fuehrt damit
  denselben Namen an zwei verschiedenen Typen, und `stapelumbenennen/mod.rs:41-52` haelt genau
  diese Sorte Doppelname als eigenen Grund fuer eine Umbenennung fest ("das kostete an jeder
  Fundstelle einen Blick auf den Modulpfad"). Die andere Haelfte liegt in
  `shared/issues/260826-1221_*_abschluss-ist-abgebrochen-hat-ausserhalb-der-proben-keinen-rufer-im-baum.md`,
  gefunden in der parallelen Durchsicht von `verzeichnis/`.

## Warum das jetzt auffaellt

Beide Module haben seit ihrer Entstehung keine Durchsicht gesehen; die vier Namen sind
Rueckstaende aus Runden, in denen ein Rufer geplant war oder wieder wegfiel. Ein weitergereichter
Name behauptet, dass draussen jemand ihn braucht, und wer ihn liest, sucht diesen Rufer.

## Was zu tun waere

Je Name eine der zwei Antworten: den Rufer bauen, oder die Sichtbarkeit auf das zuruecknehmen,
was wirklich gebraucht wird. Fuer `MELDEABSTAND` und `HOECHSTE_STELLENZAHL` heisst das, die
Zeile aus der jeweiligen Modulwurzel zu streichen; fuer `Regel::ist_wirkungslos` heisst es zu
entscheiden, ob die Frage je Regel neben der Frage je Zeile bestehen bleibt; fuer `Lauf::warten`,
ob ein Pruefwerkzeug `pub` sein soll. Keiner der vier Faelle ist ein Fehlverhalten am laufenden
Buendel.

## Umfang

`krk-core`, `operation/fortschritt.rs`, `operation/mod.rs`, `stapelumbenennen/regel.rs` und
`stapelumbenennen/mod.rs`.

---

## Abgleich 260908: zwei der fuenf sind erledigt, drei brauchen eine Entscheidung

**Alle fuenf am heutigen Baum nachgezaehlt; der Befund besteht fuer alle fuenf unveraendert.**

**Erledigt sind die zwei, fuer die der Datensatz die Antwort selbst gibt** („heisst das, die
Zeile aus der jeweiligen Modulwurzel zu streichen"):

- **`MELDEABSTAND`** ist aus `pub use fortschritt::{…}` in
  `crates/krk-core/src/operation/mod.rs` gestrichen. Er bleibt `pub` in seinem Modul, das
  selbst `pub` ist; wer ihn braucht, nennt den Modulpfad. An seiner Stelle steht der Grund.
- **`HOECHSTE_STELLENZAHL`** ist aus `pub use regel::{…}` in
  `crates/krk-core/src/stapelumbenennen/mod.rs` gestrichen, mit demselben Vermerk und dem
  Zusatz, den der Datensatz bemerkenswert nennt: das Blatt in `krk-ui` begrenzt sein
  Eingabefeld nicht ueber die Zahl, sondern laesst `Regel::aus_eingabe` antworten.

Der Bau bleibt gruen; kein Rufer im Arbeitsbereich nahm einen der beiden ueber die Modulwurzel.

**Offen bleiben die drei, fuer die der Datensatz „heisst es zu entscheiden" schreibt.** Der
Marker bleibt deshalb `_o_`:

- **`Regel::ist_wirkungslos`** — bleibt die Frage je Regel neben der Frage je Zeile
  (`Vorschauzeile::wird_umbenannt`) bestehen, oder faellt sie?
- **`Lauf::warten`** — soll ein Pruefwerkzeug `pub` sein? Zehn Rufer, alle in
  `crates/krk-core/tests/operation.rs`.
- **`operation::Abschluss::ist_abgebrochen`** — ohne jeden Rufer, auch ohne Probe; dazu die
  Doppelung des Namens mit `verzeichnis::Abschluss::ist_abgebrochen`, dessen Haelfte am
  260908 mit einem Vermerk am Doc-Kommentar geschlossen ist
  (`shared/issues/260826-1221_c_abschluss-ist-abgebrochen-hat-ausserhalb-der-proben-keinen-rufer-im-baum.md`).
  Ob hier derselbe Weg gilt oder die Methode faellt, ist nicht dasselbe: die des Umfangs hat
  nicht einmal eine Probe.
