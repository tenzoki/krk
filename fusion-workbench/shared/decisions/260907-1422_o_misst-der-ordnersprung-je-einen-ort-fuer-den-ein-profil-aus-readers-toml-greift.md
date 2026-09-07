# Misst der Ordnersprung je einen Ort, für den ein Profil aus `readers.toml` greift?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md` (Möglichkeit 2 gewählt, Möglichkeit 3 nicht); `260827-1322_*_faellt-das-default-profil-auch-im-messmodus-an-und-was-misst-l7-danach.md`; `crates/krk-ui/src/appkit/anwendung.rs` (`Anwendungsdelegierter::sitzung_laden`); `crates/krk-core/src/leseprofil/bausteine.rs` (`zusammenfassen_gezaehlt`); `crates/krk-ui/src/messmodus.rs` (`sitzungsschritte`, der L7-Ordnerblock)

---

## Question

Der am 260907 gebaute Ordnersprung misst die Vorschau des L6-Unterordners, und ausgewertet wird dieser Ordner vom eingebauten Default-Profil. Ein Profil aus `readers.toml` kann ihn nicht auswerten, und zwar aus zwei voneinander unabhängigen Gründen: der Messmodus lädt die Ablage nicht, `AnwendungsIvars::profile` bleibt dort leer, und `erkennen` liefert deshalb für jeden Ordner `None`; und der Messplatz ist ein erzeugter Prüfordner, auf den keines der mitgelieferten Profile passt, auch wenn sie geladen wären.

Damit misst der Sprung die eine Hälfte des Preises, die jeder Ordner in jedem Projekt zahlt — drei Zählzeilen, ein Leselauf —, und nicht die andere, die ein erkannter Ort kostet: nach dem Entscheid vom 260824-1900 bis zu zwölf Leseläufe und vierundzwanzig Dateiöffnungen. Die zweite Hälfte bleibt ungemessen, wie sie es vorher war.

Die Frage steht jetzt, weil der Auftrag, aus dem der Sprung entstanden ist, ausdrücklich einen Ort verlangt hat, „für den ein Leseprofil greift“. Diese Formulierung lässt sich im Messmodus nur so einlösen, wie sie eingelöst worden ist, nämlich durch das eingebaute Profil. Wer sie auf `readers.toml` bezieht, verlangt Möglichkeit 3 des Entscheids vom 260824-1900, und die ist nicht gewählt worden.

## Options

1. **Es bleibt beim Default-Profil.** Der Ordnersprung misst den Leselauf, den jeder Ordner kostet, und die Kosten eines erkannten Ortes bleiben eine Sache der Zählproben zu C6 und keiner Zeitmessung.
   - Pro: der Messmodus bleibt unabhängig vom Bestand des Geräts, und die Zahlen hängen an keiner Datei unter `resources/`. Der gemessene Ordner trägt 1.000 Eintrage, die der Messplan zusagt und die Prüfung vor dem Lauf hält; der Leselauf ist damit reproduzierbar bemessen.
   - Contra: die teurere Hälfte der Vorschauarbeit bleibt dauerhaft ohne Zeitmessung. Ein Profil, das zwölf Leseläufe kostet, könnte L7 verfehlen, ohne dass ein Abnahmelauf es je zeigte.

2. **Der Messmodus lädt einen mitgelieferten Profilsatz mit** (Möglichkeit 3 von 260824-1900), und der Messplatz bekommt einen Ordner, den eines der Profile erkennt.
   - Pro: L7 misst die vollständige Arbeit der Vorschau, teure Profile eingeschlossen.
   - Contra: die Messung hinge an einer Datei unter `resources/`, und jede Änderung an dieser Datei änderte die Zahlen, ohne dass jemand die Anwendung angefasst hätte. Der Messmodus lädt heute mit Absicht nichts aus der Ablage.

3. **Der Messplatz bekommt eine eigene, im Prüfordner-Erzeuger festgeschriebene Profildatei**, die die Anwendung im Messmodus über den Messplan bekommt.
   - Pro: misst die teure Hälfte, ohne die Zahlen an `resources/default-readers.toml` zu hängen.
   - Contra: ein zweiter Weg, auf dem Profile in KRK hereinkommen; heute gibt es genau einen, und eine Probe hält ihn (`die_leseprofile_werden_im_baum_genau_einmal_geladen`).

## Constraints

- Keine der drei Möglichkeiten setzt eine elfte Zeitzusage oder fasst eine der zehn an.
- Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; keine der drei lässt sich vor der Antwort messen.
- Der Rückfallweg bleibt einer (Constraint 4 des Specs der Runde 16); Möglichkeit 3 darf ihn nicht verdoppeln.

## Recommendation

Möglichkeit 1, vorerst. Der Sprung misst seit dem 260907 das, was jeden Nutzer trifft, und das ist die Hälfte, die vorher gar nicht gemessen war. Die teure Hälfte ist heute über die Zählproben zu C6 gedeckt, die Aufrufe zählen statt Millisekunden; ob dieser Deckung eine Zeitmessung fehlt, ist erst nach dem nächsten Abnahmelauf zu beurteilen, und vorher wäre jede Antwort darauf geraten.
