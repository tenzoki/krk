Der Editor hält die 16 MB nur am Eingang: ein durch Bearbeiten darüber gewachsener Stand wird gesichert und beim nächsten Öffnen abgewiesen

---

`EDITORGRENZE` wird an genau einer Stelle geprüft, in `krk_core::text::datei::oeffnen` auf dem
Ladefaden (`crates/krk-ui/src/editormodell.rs:467`). Danach kennt das Modell keine Grenze mehr:
`bearbeiten` (`editormodell.rs:941-947`) nimmt jeden Stand an, `treffer_ersetzen` (`:1180-1203`)
und `alle_treffer_ersetzen` (`:1213-1230`) lassen ihn wachsen, und `sichern` (`:986-1007`)
schreibt ihn ungeprüft. Eine Datei knapp unter 16 MB, in die der Nutzer einen grossen Block
einfügt oder in der ein Sammelersetzen jeden Treffer verlängert, liegt danach darüber; das
Sichern meldet `Gesichert`, und das nächste F4 auf dieselbe Datei antwortet
`Abgewiesen(ZuGross)`. KRK hat dann eine Datei geschrieben, die es selbst nicht mehr öffnet, und
kein Satz hat es angekündigt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-ui/src/editormodell.rs` (`bearbeiten`, `treffer_ersetzen`,
`alle_treffer_ersetzen`, `sichern`)
**Baumstand:** `ca8072d`

## Was das Modell heute hält und was nicht

Die Zusage aus `CLAUDE.md` lautet „nimmt Textdateien bis rund 16 MB an", und die Ansicht bindet
ihr Rückgängig-Budget daran (`appkit/editor.rs:879-885`, `STAPELBUDGET == EDITORGRENZE` mit
`const _: () = assert!`). Der Stand selbst hat keine solche Bindung. Die Probe
`eine_datei_ueber_der_grenze_wird_gestellt_und_nicht_aufgenommen` (`editormodell.rs:1420`)
misst allein den Eingang.

Die Abkürzung `SchonOffen` (`:747-757`) verdeckt den Fall im laufenden Betrieb: solange der
Editor die Datei hält, wird sie nicht neu gelesen und die Grenze nicht gefragt. Sichtbar wird er
erst nach `schliessen` oder nach einem Neustart, also an einer Stelle, die mit dem Sichern nichts
mehr zu tun hat.

## Drei Wege

1. **`sichern` weist einen Stand über der Grenze ab** und liefert `Gescheitert` mit dem Grund.
   Kleinster Eingriff; der Nutzer sitzt dann auf einem Stand, den er nicht loswird, ausser er
   kürzt ihn.
2. **`bearbeiten` meldet die Überschreitung**, etwa als dritter Ausgang neben `true`/`false`, und
   die Statuszeile sagt es beim Tippen. Verlangt eine Änderung an der Signatur, die das
   `#[must_use]` schon trägt.
3. **Die Grenze gilt nur dem Einlesen, und das steht so im Modulkopf.** Dann muss
   `datei::oeffnen` eine Datei, die KRK selbst geschrieben hat, auch wieder öffnen — also gilt die
   Grenze nicht mehr, oder es gibt zwei.

Weg 1 und Weg 2 schliessen einander nicht aus. Welcher gilt, ist eine Zusagefrage und gehört
vor den Spec der Runde 2, Abschnitt zu C2.
