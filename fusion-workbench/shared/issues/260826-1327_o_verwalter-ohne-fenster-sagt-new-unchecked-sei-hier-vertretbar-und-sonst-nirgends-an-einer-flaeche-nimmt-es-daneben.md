verwalter_ohne_fenster sagt, new_unchecked sei "hier vertretbar und sonst nirgends"; an_einer_flaeche nimmt es daneben

---

Das Pruefmodul von `editor.rs` behauptet den Hauptfaden an zwei Stellen ueber
`MainThreadMarker::new_unchecked`, nicht an einer: `verwalter_ohne_fenster` (:3875) und
`an_einer_flaeche` (:4730). Der Doc-Kommentar der ersten sagt "hier vertretbar und sonst nirgends", der
der zweiten nennt sich "der eine Ort, an dem eine Probe eine AppKit-Ansicht baut" und traegt die
Sperre, die die erste nicht hat. CLAUDE.md nennt allein `an_einer_flaeche`.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/editor.rs:3866`: "**Das `new_unchecked` ist hier vertretbar und sonst
  nirgends.**" — an `verwalter_ohne_fenster` (`:3874-3876`).
- `:4725-4731`: `an_einer_flaeche` mit `static SPERRE: Mutex<()>` und dem zweiten `new_unchecked`.
- `:4683-4688`: "Mehrere Proben, die gleichzeitig auf verschiedenen Faeden AppKit-Objekte bauen, waeren
  eine zweite Behauptung … unter der Sperre baut zu jeder Zeit hoechstens eine." Die sechs Proben auf
  `verwalter_ohne_fenster` (`:3923`, `:3953`, `:3997`, `:4073`, `:4163`) laufen ohne diese Sperre auf
  eigenen Faeden, zwei davon mit `NSRunLoop::currentRunLoop().runMode_beforeDate` (`:4031`, `:4103`).
  Das ist Foundation und keine Ansicht — der Kopf `:3866-3873` begruendet es —, aber "sonst nirgends"
  ist falsch, und die Sperre deckt die Haelfte der Behauptungen.
- Ein dritter Ort im selben Teilbaum: `appkit/blaetter/mod.rs:1110`, ebenfalls unter `#[cfg(test)]`
  (`:849`).
- CLAUDE.md, "Was man nicht sieht": "die, die eine `NSTextView` bauen, behaupten den Hauptfaden ueber
  `MainThreadMarker::new_unchecked` in `an_einer_flaeche`" — richtig fuer die Flaechen, aber nicht die
  ganze Zahl der Behauptungen.

Ausserhalb von `#[cfg(test)]` kommt `new_unchecked` in `krk-ui` nicht vor (`grep -rn new_unchecked
crates/krk-ui/src`: drei Treffer, alle in Pruefmodulen). Die Zusage aus CLAUDE.md haelt insoweit.

## Was zu tun waere

Den Satz `:3866` auf "an den drei Stellen des Teilbaums" stellen und entscheiden, ob die Verwalterproben
unter dieselbe Sperre gehoeren; die Frage haengt an
`circles/260807-2116-…/decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`
(zurueckgestellt), dessen Zahl "vier" mit den sechs Verwalterproben ebenfalls nicht mehr stimmt.

## Umfang

`krk-ui`, `appkit/editor.rs` und `appkit/blaetter/mod.rs`, Pruefmodule; CLAUDE.md.
