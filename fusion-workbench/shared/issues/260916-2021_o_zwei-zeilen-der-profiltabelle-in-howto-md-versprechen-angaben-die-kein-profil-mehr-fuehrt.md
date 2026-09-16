Zwei Zeilen der Profiltabelle in HowTo.md versprechen Angaben, die kein Profil mehr führt

---

Die Tabelle unter `## Die mitgelieferten Leseprofile` in `HowTo.md` (ab Zeile 428) beschreibt je Profil, was die Vorschau zeigt. Zwei ihrer Zeilen nennen Angaben, die `resources/default-readers.toml` nicht enthält, und lassen eine aus, die es enthält. Die Anleitung reist seit dem 260905 im Releasepaket mit, ein Nutzer liest die Tabelle also als Zusage.

---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

**Zeile „die Wurzel" (HowTo.md:430).** Sie verspricht „Projekt, Einrichtungszeitpunkt, fusion-Fassung, aktive Runde, Sitzung, Zahl der Runden, offene gemeinsame Defekte". Das Profil `fusion-Werkbank: die Wurzel` führt sechs Zeilen: `Projekt`, `Eingerichtet`, `fusion-Fassung`, `Runden`, `Offene Defekte, gemeinsam`, `Nachrichten`. „Aktive Runde" und „Sitzung" gibt es nicht mehr — beide Angaben sind mit fusion 11 gefallen —, und `Nachrichten` fehlt in der Aufzählung, obwohl es die Zeile ist, für die das Wurzelprofil im Kommentar der Profildatei eigens begründet wird.

**Zeile „Projektwurzel mit fusion-Werkbank" (HowTo.md:431)** sagt „dieselben Angaben, eine Ebene höher gelesen" und erbt den Fehler dadurch unverändert.

**Zeile „alle Runden" (HowTo.md:433).** Sie verspricht „Runden gesamt und je eine Zeile für vorgesehen, aktiv, kohärent geschlossen, beschränkt geschlossen, überholt und zurückgestellt, dazu die offenen Defekte aller Runden". Das Profil `fusion-Werkbank: alle Runden` führt zwei Zeilen: `Runden` und `Offene Defekte, alle Runden`. Die sechs Zustandszeilen sind gefallen, als der Zustand eines Arbeitspakets mit fusion 11 vom Dateinamen in die Kopfzeile `**Status:**` gewandert ist.

**Wie sich der Befund erheben lässt.** Die Beschriftungen eines Profils zählt
`awk '/^\[\[profil\]\]/{p=0} /^name = "fusion-Werkbank: die Wurzel"/{p=1} p&&/beschriftung/' resources/default-readers.toml`,
mit dem jeweiligen Profilnamen in der zweiten Bedingung.

**Abnahme.** Beide Tabellenzeilen nennen genau die Beschriftungen, die das jeweilige Profil in `resources/default-readers.toml` führt, in deren Reihenfolge; die Zeile „Projektwurzel mit fusion-Werkbank" bleibt ein Verweis und wird nicht ausgeschrieben.

**Gefunden am 260916** beim Nachziehen der Zeile „eine Runde" derselben Tabelle, die im selben Zug berichtigt wurde. Die zwei übrigen Zeilen standen außerhalb jenes Auftrags.

**Cross-references:** 260912-0441_*_die-spalte-wer-schreibt-der-howto-tabelle-nennt-fuer-drei-dateien-das-gegenteil-dessen-was-krk-tut.md
