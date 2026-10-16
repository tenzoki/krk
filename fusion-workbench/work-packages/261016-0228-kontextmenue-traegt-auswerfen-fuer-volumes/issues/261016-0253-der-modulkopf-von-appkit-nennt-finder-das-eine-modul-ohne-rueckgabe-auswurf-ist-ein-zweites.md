Der Modulkopf von appkit nennt finder das eine Modul ohne Rückgabe, auswurf ist ein zweites
---
`crates/krk-ui/src/appkit/mod.rs` sagt über `finder`: „Es ist daneben das eine Modul dieser Reihe, dessen Aufruf **nichts** zurueckgibt“. Seit `bb2f861` gibt auch `auswurf::auswerfen` nichts zurück (`-> ()`, die Antwort kommt über `fertig`).
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0253-reviewer-auswerfen-im-kontextmenue.md

**Beleg.** `crates/krk-ui/src/appkit/finder.rs` `pub fn aufdecken(pfade: &[PathBuf])` und `crates/krk-ui/src/appkit/auswurf.rs` `pub fn auswerfen(einhaengepunkt: &Path, fertig: impl FnOnce(...))`, beide ohne Rückgabewert. Der Absatz über `auswurf`, den derselbe Commit unmittelbar darunter einfügt, grenzt sich über „Antwort spaeter und auf einem fremden Faden“ ab und lässt den Satz über `finder` stehen.

**Richtung.** Den Satz über `finder` so fassen, dass er die Eigenschaft nennt, die ihn trennt (der Aufrufer erfährt nie, ob das Aufdecken gelang), statt einer Einzigkeit, die mit jedem weiteren Modul kippt (`critical-stance.md` §5).

**Abnahme.** Kein Satz im Modulkopf von `appkit/mod.rs` behauptet für `finder` oder `auswurf` eine Einzigkeit, die das jeweils andere Modul ebenfalls erfüllt.
