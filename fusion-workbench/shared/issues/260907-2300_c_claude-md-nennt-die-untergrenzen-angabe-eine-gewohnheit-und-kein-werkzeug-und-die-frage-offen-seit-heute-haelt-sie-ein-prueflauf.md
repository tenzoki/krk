# CLAUDE.md nennt die Untergrenzen-Angabe „eine Gewohnheit und kein Werkzeug" und die Frage offen — seit heute hält sie ein Prüflauf

---
Der Abschnitt `## Technologiewahl` in `CLAUDE.md` trägt zwei Aussagen, die die Arbeit vom
260907 überholt hat, und daneben eine Ausnahmeliste, die es so nicht mehr gibt.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`,
`260907-2256_*_die-untergrenzen-abschnitte-nannten-196-hereingeholte-namen-nicht-und-27-von-42-appkit-dateien-waren-betroffen.md`

## Die drei Stellen

1. „**Die Gegenmaßnahme ist eine Gewohnheit und kein Werkzeug**, und sie hält sich nicht
   von selbst." — Seit dem 260907 halten zwei Proben in `crates/krk-core/tests/baum.rs`
   sie: `jede_appkit_datei_mit_frameworkimport_traegt_den_untergrenzen_abschnitt` und
   `jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt`. Die Gewohnheit ist
   damit zur Hälfte ein Werkzeug: das Dastehen des Abschnitts und die Nennung jedes
   hereingeholten Namens hält der Bau, die Richtigkeit der Zahl weiterhin der Mensch.

2. „Ob und wie weit die Angabe prüfbar gemacht wird, ist offen" — mit einem Verweis auf
   `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`, den CLAUDE.md dort
   mit Speicherpfad schreibt. Der Nutzer hat am
   260907 die Möglichkeiten 1 und 2 gewählt und 3 verworfen; der Datensatz steht auf
   umgesetzt.

3. „der Abschnitt … steht in **jeder** Datei unter `crates/krk-ui/src/appkit/` außer
   zweien: `koordinaten.rs` und `mod.rs`, beide begründet." — Die zwei stehen nicht mehr
   als Liste da. Die Probe fordert den Abschnitt von jeder Datei ein, die einen Namen aus
   einer `objc2_`-Kiste hereinholt; die beiden holen keinen herein und fallen von selbst
   heraus. Eine dritte solche Datei bräuchte keinen Eintrag, und `koordinaten.rs` schuldet
   den Abschnitt in dem Augenblick, in dem sie den ersten Namen hereinholt.

## Abnahmetest

Die drei Stellen sagen, was der Baum trägt: dass zwei Proben die Angabe halten, welche
Hälfte sie halten und welche nicht, und dass die Ausnahme eine Eigenschaft ist und keine
Aufzählung. Die Zahl der Dateien steht dabei nicht in `CLAUDE.md`: sie ist zwischen dem
260811 und dem 260814 viermal falsch geworden, und die Datei sagt das an derselben Stelle
schon selbst.

---
Resolved: Die drei Stellen sagen, was der Baum trägt.

1. Aus „eine Gewohnheit und kein Werkzeug" ist geworden: bis zum 260907 eine
   Gewohnheit, seitdem zur einen Hälfte ein Werkzeug. Die zwei Proben stehen
   namentlich da, und welche Hälfte sie halten, steht dabei — das Dastehen des
   Abschnitts und die Nennung jedes hereingeholten Namens. **Die Richtigkeit der
   macOS-Zahl bleibt ausdrücklich eine Zusage des Menschen**, mit dem Grund
   daneben: sie am SDK zu prüfen bräuchte Xcode, und der Nutzer hat das
   verworfen.
2. „ist offen" ist gefallen; der Entscheid `260811-2050_*_…` trägt `_i_`, und
   der Absatz nennt die gewählten Möglichkeiten 1 und 2 und die verworfene 3.
3. Die zwei Ausnahmen stehen nicht mehr als Liste, sondern als Eigenschaft: den
   Abschnitt schuldet jede Datei, die einen Namen aus einer `objc2_`-Kiste
   hereinholt. `koordinaten.rs` und `mod.rs` fallen von selbst heraus, eine
   dritte solche Datei bräuchte keinen Eintrag, und `koordinaten.rs` schuldet den
   Abschnitt in dem Augenblick, in dem sie den ersten Namen hereinholt.

Die Zahl der Dateien steht weiterhin nicht in `CLAUDE.md`.

Belegt am Baum vor der Änderung: `crates/krk-core/tests/baum.rs:1238` und
`:1276`, dazu der Doc-Kommentar der ersten Probe, der die Zusage des Menschen
selbst ausschreibt.
