# E2b — die vierte Funktion ohne Kombination steht in den Prüflisten

**Datum:** 260814-2335
**Agent:** coder
**Status:** Complete
**Auftrag:** Nachzug zwischen den Planschritten E1 und E2, kein eigener Planschritt.
Quelle: `issues/260814-2320_c_drei-proben-fuehren-die-funktionen-ohne-kombination-als-liste-und-e2-macht-sie-vierstellig.md`

## Was war

E1 (Kommando `TiefeSucheUmschalten`) und E2 (Eintrag in `resources/default-keymap.toml`)
liegen ungesichert im Baum und liefern zusammen die vierte Funktion aus, die ab Werk keine
Tastenkombination trägt: `tiefe_suche_umschalten`. Drei Proben führen diese Funktionen als
ausgeschriebene Liste fester Länge und fielen deshalb. Das ist die vom Baum vorgesehene
Stelle: der Doc-Kommentar über `OHNE_KOMBINATION_AB_WERK` sagt wörtlich, wer eine vierte
ausliefert, trage sie mit ihrem Datensatz dort nach.

## Was getan wurde

1. `crates/krk-core/tests/belegung.rs` — `OHNE_KOMBINATION_AB_WERK` von `[&str; 3]` auf
   `[&str; 4]`, `"tiefe_suche_umschalten"` hinzu. Der Grund steht als eigener Absatz und
   nicht in der Klammer der drei Spaltenschalter: dort ist es die Knappheit der
   Kombinationen, hier eine vom Nutzer offen gelassene Wahl (260814-1610,
   `decisions/260814-1552_a_welche-tastenkombination-schaltet-die-tiefe-suche.md`). Der
   Schlusssatz sagt jetzt „fünfte" statt „vierte".
2. `crates/krk-ui/src/belegungsausgabe.rs` — dasselbe im Literal von
   `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`, samt Kommentar und
   Fehlermeldung. Die Reihenfolge stimmt ohne Zutun: der Eintrag steht in der
   Belegungsdatei hinter den drei Spaltenschaltern, und die Probe vergleicht in der
   Reihenfolge der Datei.
3. Beide Kommentare zeigen jetzt aufeinander und auf den neuen Datensatz.

## Die zwei Listen: geprüft, nicht zusammengelegt

Der Auftrag verlangte zu prüfen, ob die zwei Aufzählungen eine werden können. Sie können,
aber nur gegen einen Preis, und der ist eine Nutzerfrage:
`decisions/260814-2326_o_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`.

Der Befund darin, der die Möglichkeiten erst bewertbar macht: **die zwei Listen behaupten
heute nicht dasselbe.** `krk-core` prüft eine Richtung — keine Funktion außerhalb der
Liste steht ohne Kombination da; eine Funktion **in** der Liste, die später eine Taste
bekommt, fällt still durch den `contains`-Zweig. `krk-ui` prüft mit seinem `assert_eq!`
über den ganzen Vektor beide Richtungen. Wer das Literal in `krk-ui` ersatzlos streicht,
verliert die Gegenrichtung, ohne dass eine Probe rot würde.

Ein `#[cfg(test)]` in der Bibliothek von `krk-core` löst es nicht: eine Integrationsprobe
unter `crates/krk-core/tests/` bindet die Bibliothek ohne `cfg(test)`. Die Konstante müsste
ausgelieferter Bibliothekscode werden — Prüfwissen im Bündel, das zur Laufzeit niemand
liest. Empfohlen ist stattdessen die dritte Möglichkeit: die Liste bleibt allein in
`krk-core`, und die fehlende Gegenrichtung zieht dorthin mit. Sie fasst drei Proben an und
gehört damit nicht in einen Nachzug dieser Größe.

## Verification

`make check` — Exit 0, „alle vier gruen". Die drei zuvor roten Proben laufen namentlich
grün: `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste`,
`jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`,
`belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`.

## Nicht angefasst

Die Änderungen aus E1 und E2 im Arbeitsbaum. Nicht committet — der Nutzer sichert E1, E2
und E2b zusammen.
