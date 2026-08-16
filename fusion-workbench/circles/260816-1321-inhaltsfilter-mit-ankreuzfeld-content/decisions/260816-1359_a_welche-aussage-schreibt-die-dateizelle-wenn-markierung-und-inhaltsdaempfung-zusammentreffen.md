# Welche Aussage schreibt die Dateizelle, wenn Markierung und Inhaltsdämpfung zusammentreffen?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `shared/decisions/260816-1310_a_sieht-der-nutzer-ob-eine-zeile-wegen-des-namens-oder-wegen-des-inhalts-steht.md` (der Nutzerentscheid, der diese Bauentscheidung hinterlässt); `crates/krk-ui/src/appkit/tabelle.rs:2808-2820` (die eine Stelle, die Farbe und Schrift der Zelle schreibt); `crates/krk-ui/src/appkit/leiste.rs:471-473` und `:541-542` (das vorhandene Vokabular der Dämpfung); `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` (C5)

---

## Question

Der Nutzerentscheid vom 260816-1330 wählt die abgesetzte Darstellung: eine Zeile, die allein wegen ihres Inhalts dasteht, wird gedämpft geschrieben. Er hinterlässt zwei Bauentscheidungen, und die zweite lautet: welche Aussage schreibt die Zelle, wenn ein Inhaltstreffer zugleich markiert ist?

Die Tabelle schreibt Farbe und Schrift je Zelle an genau einer Stelle (`tabelle.rs:2808-2820`). Sie kennt heute eine bedeutungstragende Einfärbung: markiert heißt `systemOrangeColor` und Fettschrift, nicht markiert heißt `labelColor` und gewöhnliche Schrift. Beide Kennzeichen zusammen sind Absicht, damit Farbfehlsichtige die Markierung an der Fettung erkennen. Die Auswahl färbt AppKit selbst blau; KRK schreibt dafür nichts.

Ein markierter Inhaltstreffer beansprucht damit dieselbe Eigenschaft zweimal. Die Markierung entscheidet, worauf die nächste Dateioperation wirkt; die Dämpfung ist eine Auskunft darüber, warum die Zeile im Filter steht. Ein Mischwert wäre eine dritte Farbe mit einer Bedeutung, die niemand festgelegt hat, und C5.4 schließt einen dritten Zustand ausdrücklich aus.

## Options

1. **Die Markierung schreibt, die Dämpfung weicht.** Ein markierter Eintrag bleibt orange und fett, gleich aus welchem Grund er in der Liste steht.
   - Pro: die Markierung entscheidet über Löschen, Verschieben und Kopieren. Eine Markierung, die man übersieht, kostet Dateien; eine Herkunftsauskunft, die man übersieht, kostet einen zweiten Blick. Der markierte Eintrag ist außerdem der, den der Nutzer selbst ausgewählt hat, also der, bei dem die Frage „warum steht der hier" schon beantwortet ist.
   - Kontra: innerhalb der markierten Einträge ist ein Inhaltstreffer nicht mehr von einem Namenstreffer zu unterscheiden. C5.1 gilt dann für die unmarkierten Zeilen und nicht für alle.
2. **Die Dämpfung schreibt, die Markierung weicht.**
   - Pro: C5.1 gälte ohne Ausnahme.
   - Kontra: die Markierung verschwände genau dort, wo sie Folgen hat. C5.2 verlangt ausdrücklich, dass eine markierte Zeile als markiert erkennbar bleibt; diese Möglichkeit verletzt sie.
3. **Ein Mischwert, etwa gedämpftes Orange.**
   - Pro: beide Aussagen blieben sichtbar.
   - Kontra: eine dritte bedeutungstragende Farbe, also der dritte Zustand, den C5.4 ausschließt. Sie müsste in beiden Farbtafeln gegen Orange und gegen die gedämpfte Grundfarbe unterscheidbar bleiben, und sie wäre keine dynamische Systemfarbe mehr, also bräuchte die Tabelle einen Beobachter der Erscheinung, den sie heute nicht hat.
4. **Die Dämpfung geht auf einen anderen Kanal, etwa Kursivschrift.**
   - Pro: kein Zusammenstoß, beide Aussagen stünden nebeneinander.
   - Kontra: der Nutzerentscheid nennt das Vokabular ausdrücklich und benennt es als Farbe, mit dem Satz, eine vierte Farbe wäre die zu begründende und nicht diese zwei. Die Frage lautet dort „welche der beiden schreibt die Zelle" und nicht „gibt es einen dritten Kanal"; ein Kanalwechsel wäre eine Neuverhandlung der Antwort und keine Umsetzung.

## Constraints

- Die Aussage der Markierung darf nicht verschwinden (C5.2).
- Es entsteht kein dritter Zustand (C5.4).
- Beide Farbtafeln gelten, und die Unterscheidung zieht bei einem Wechsel nach (C5.3). Die Tabelle hat keinen Beobachter der Erscheinung; sie kommt ohne ihn aus, weil `labelColor` und `systemOrangeColor` dynamische Systemfarben sind. Jede neue Farbe, die keine ist, brächte einen Beobachter mit.
- Die Zelle ist ein nacktes `NSTextField`, und Farbe wie Schrift werden in jedem Zeichendurchgang gesetzt, weil die Felder wiederverwendet werden.

## Recommendation

Möglichkeit 1, und die zweite Bauentscheidung dazu gleich mit: gedämpft wird mit `secondaryLabelColor` und nicht mit `tertiaryLabelColor`.

Die Rangfolge folgt aus den Folgen. Wer eine Markierung übersieht, verliert Dateien; wer eine Herkunftsauskunft übersieht, öffnet die Datei und sieht nach. Der Verlust ist benannt und wird nicht kleingeredet: innerhalb der markierten Einträge trägt C5.1 nicht.

`secondaryLabelColor` steht in der Lesezeichenleiste für eine Überschriftszeile, also für etwas Gültiges mit anderem Rang; `tertiaryLabelColor` steht dort für eine Marke, deren Ziel fehlt, also für etwas Kaputtes. Ein Inhaltstreffer ist eine gültige Zeile mit anderer Herkunft und nicht eine beschädigte Datei. `tertiaryLabelColor` ist daneben deutlich kontrastärmer, und die Dämpfung trifft alle vier Spalten einer Zeile.

---
Answered: `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt F1 — Möglichkeit 1: die Markierung schreibt die Zelle, die Dämpfung weicht, und gedämpft wird mit `secondaryLabelColor`. Der benannte Verlust steht im Plan unter `## Risks & Mitigations`.
Implemented:
Deferred:
Superseded by:
