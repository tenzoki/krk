# Verlangt der Blattbauer die liegenlassende Schaltfläche am Typ?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:**
`issues/260817-1419_c_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`
(der Befund, dessen Mechanismuswechsel diese Frage ist),
`issues/260817-1106_c_eine-unbekannte-blattantwort-faellt-im-loeschblatt-auf-die-zerstoerende-schaltflaeche.md`
(der eingetretene Fehler)

---

## Frage

`abbruchstelle` (`crates/krk-ui/src/appkit/blaetter/mod.rs`) beantwortet die
Frage „welche Schaltfläche ist die ungefährliche" und muss dabei eine Frage
mitbeantworten, die keine Antwort hat: welche es ist, wenn keine es ist. Der
`unwrap_or(0)` ist eine Näherung darauf. Sie ist heute unauffällig, weil die
Stelle 0 in jedem Blatt des Baums die abbrechende ist; in einem Blatt mit
ausführender erster Schaltfläche wäre sie der zerstörende Ausgang, also genau
der Fehler, den `260817-1106` behoben hat.

Seit dem 260818 hält dagegen ein `assert!` in `Blatt::mit_schaltflaechen`, das
in jedem Profil greift und dessen Wirken gemessen ist. Damit ist der Befund
geschlossen. Offen bleibt der stärkere Weg: die liegenlassende Schaltfläche am
**Typ** zu verlangen, statt sie zu prüfen. Dann ist `abbruchstelle` total, der
Rückfall entfällt, und der Übersetzer trägt die Zusage statt einer Prüfung zur
Laufzeit.

## Warum es nicht nebenbei geht

Die naheliegende Signatur des Befunds,
`mit_schaltflaechen(mtm, frage, ausfuehrende: &[Schaltflaeche], liegenlassende: Schaltflaeche)`,
verliert die **Stelle** der liegenlassenden Schaltfläche in der Reihenfolge. Die
Reihenfolge ist bindend (C4): bei der Rückfrage vor dem Räumen steht die
liegenlassende **vorn**, bei `Blatt::neu` **hinten**. Eine Form, die beides
trägt, braucht einen eigenen Typ für den Bauplan, und das ist ein
Entwurfsschnitt und keine Messung.

## Möglichkeiten

1. **Es bleibt beim `assert!`.** Kosten: keine. Preis: die Zusage steht zur
   Laufzeit und nicht beim Übersetzen, und `abbruchstelle` behält eine Zeile in
   ihrer Tafel, die einen Fall beschreibt, den es nicht geben soll.
2. **Ein Typ für den Bauplan**, der die liegenlassende Schaltfläche und ihre
   Stelle zusammen trägt und ohne sie nicht baubar ist. Kosten: ein Typ, elf
   Aufrufstellen, und die Zählprobe je Datei wird überflüssig. Ertrag: die
   Verdrehung wird unübersetzbar, und `abbruchstelle` ist total.
3. **Nur `Blatt::neu` und die Rückfrage bekommen reine Bauplanfunktionen, die
   übrigen bleiben.** Der halbe Schritt; er steht seit dem 260818 zur Hälfte am
   Baum (`standardschaltflaechen`, `loeschbestaetigung::schaltflaechen`) und
   ließe sich auf die fünf übrigen Blätter ausdehnen, ohne die Signatur zu
   ändern. Ertrag: jedes Blatt wird ohne AppKit prüfbar, aber die Zusage bleibt
   eine Probe und keine Übersetzungsbedingung.

## Randbedingungen

- Die Reihenfolge der Schaltflächen ist Abnahmekriterium (C4) und darf sich
  durch den Umbau nicht ändern.
- `Blatt::neu` und `Blatt::mit_schaltflaechen` bleiben der eine Bauer; ein
  zweiter wäre der Doppelbau, den die Zählprobe verhindert.

## Empfehlung

Möglichkeit 3, falls die nächste Runde ohnehin an den Blättern arbeitet: sie
kostet je Blatt drei Zeilen und macht jeden Bauplan messbar. Möglichkeit 2 lohnt
erst, wenn ein Blatt dazukommt, dessen erste Schaltfläche ausführt — heute hat
keines das.
