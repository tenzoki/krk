Der Überschneidungssatz in `textmerkmale::anwenden` gilt seit `markdown.rs` nicht mehr, und vier Auszeichnungen setzen jetzt dieselbe Schrift

---

Der SAFETY-Kommentar in `crates/krk-ui/src/appkit/textmerkmale.rs:210-227`
sagt zu: „`Ueberschrift` und `FesteSchrift` setzen beide die Schrift und
überlappen einander deshalb nie: die Fallunterscheidung in
`crate::hervorhebung` fragt die Überschriftsstufe zuerst und die feste Schrift
nur sonst." Der Satz ist seit Planschritt 8 falsch. `crate::markdown` ist ein
**zweiter** Erzeuger von `Formatierung`, und er liefert genau diese
Überschneidung — die eigene Probe `die_auszeichnungen_stehen_von_aussen_nach_innen`
(`markdown.rs:722`) schreibt sie sogar fest.

---

**Was sich geändert hat.** `Auszeichnung` ist von drei auf fünf Werte
gewachsen, und **vier** von ihnen setzen jetzt denselben Merkmalsnamen
`NSFontAttributeName` (`textmerkmale.rs:197-208`): `Ueberschrift`,
`FesteSchrift`, `Betonung`, `StarkeBetonung`. Nur `Listenzeile` setzt einen
anderen. Der Kommentar begründet die Unbedenklichkeit der Überschneidungen
damit, dass die beiden überlappenden Auszeichnungen „verschiedene
Merkmalsnamen — Schrift gegen Absatzstil" setzten. Bei vier von fünf Werten
trifft das nicht mehr zu, und `addAttributes:range:` legt bei gleichem Namen
nicht zusammen, sondern ersetzt.

**Die Ersatzregel steht in `markdown.rs` und trägt nicht überall.**
`Zerlegung::abschliessen` (`markdown.rs:663-681`) sortiert nach Anfang und bei
gleichem Anfang das **längere** zuerst, damit außen vor innen steht und innen
das innere gewinnt. Diese Regel bricht, sobald außen und innen **dieselbe
Länge** haben: dann entscheidet die stabile Sortierung nach Einfügereihenfolge,
und die läuft von innen nach außen.

Gemessen mit `markdown::rendern` (unverändert in ein Prüfprogramm kopiert):

```
Quelle : "# `Code` im Titel"
Reihenfolge: Ueberschrift{1}(0,13), FesteSchrift(0,4)
  -> innen gewinnt: der Quelltext steht in fester Schrift. Wie beabsichtigt.

Quelle : "**`code`**"
Reihenfolge: FesteSchrift(0,4), StarkeBetonung(0,4)
  -> die StarkeBetonung gewinnt: `code` steht fett in der Systemschrift und
     hat seine feste Schrift verloren.

Quelle : "*kursiv **fett** wieder kursiv*"
Reihenfolge: Betonung(0,25), StarkeBetonung(7,4)
  -> "fett" ist fett und nicht mehr kursiv.
```

Der zweite Fall ist die Umkehrung des ersten, und beide entstehen aus derselben
Sortierung. Der dritte ist die allgemeine Form: wo zwei schriftsetzende
Auszeichnungen einander enthalten, geht die äußere für den überlappten Bereich
vollständig verloren, statt sich mit der inneren zu verbinden.

**Warum der Kommentar wiegt und nicht nur der Effekt.** `CLAUDE.md` führt unter
„Was man nicht sieht" einen Fall, in dem genau eine solche Zusicherung im Code
eine ganze Sitzung und einen Fehlbefund gekostet hat. Eine SAFETY-Begründung,
die eine Unmöglichkeit behauptet, die eintritt, ist die gefährliche Sorte
veralteter Kommentar: der nächste Leser prüft die Bedingung nicht nach, weil
die Datei sie ihm zusagt.

**Was zu tun ist**, in dieser Reihenfolge:

1. Den SAFETY-Kommentar berichtigen: vier Auszeichnungen setzen die Schrift,
   sie überlappen einander, und was gilt, entscheidet die Reihenfolge der
   Liste.
2. Entscheiden, ob die Reihenfolge genügt oder ob die Schrift zusammengelegt
   werden soll (fett **und** kursiv, feste Schrift **und** fett — über
   `NSFontDescriptor`-Merkmale statt über ein Ersetzen). Der erste Weg ist eine
   Zeile Kommentar, der zweite ein Umbau von `anwenden` auf einen
   Schriftzustand je Stelle.
3. Die Reihenfolge bei gleicher Länge festlegen, statt sie der stabilen
   Sortierung zu überlassen; heute ist sie ein Nebenprodukt.

**Gewicht:** mittel — die falsche Zusicherung im Code wiegt schwerer als die
Anzeige, die nur Auszeichnungen betrifft, die einander enthalten.

**Herkunft:** Circle der Runde 6, Planschritte 7 und 8.

---

**Teilweise behoben 260812 — der Datensatz bleibt offen.** Von den drei
Punkten sind der erste und der dritte erledigt, der zweite nicht.

**Nachgemessen.** Alle drei Ausgaben des Datensatzes stimmen am Baum genau so.

**Punkt 1, der Kommentar: berichtigt.** Der SAFETY-Kommentar in
`crates/krk-ui/src/appkit/textmerkmale.rs` sagt jetzt, was gilt: vier der fuenf
Auszeichnungen setzen `NSFontAttributeName`, verschachtelte Listenzeilen setzen
einander ueberlappend denselben Absatzstil, `addAttributes:` ersetzt bei
gleichem Namen, und was gilt, entscheidet allein die Reihenfolge der Schleife.
Die behauptete Unmoeglichkeit steht nicht mehr da.

**Punkt 3, die Reihenfolge bei gleicher Laenge: festgelegt.** `Offen` traegt
einen `rang`, den Zaehler der geoeffneten Bereiche, und
`Zerlegung::abschliessen` (`crates/krk-ui/src/markdown.rs`) sortiert nach
Anfang, dann nach absteigender Laenge, dann nach dem Rang. Die Ordnung ist
damit total und haengt nicht mehr an der Stabilitaet der Sortierung. Weil der
Rang beim Oeffnen vergeben wird, steht das aeussere Element vorn — die
Richtung, die die Regel schon vorher behauptete.

Sichtbar geaendert hat das genau einen der drei gemessenen Faelle:
`` **`code`** `` liefert jetzt `StarkeBetonung(0,4), FesteSchrift(0,4)` statt
umgekehrt, `code` steht also in fester Schrift und nicht mehr fett in der
Systemschrift. Verloren geht weiterhin eine der beiden, nur jetzt nach der
Regel und nicht nach einem Nebenprodukt. Die Probe
`bei_gleichem_bereich_steht_das_zuerst_geoeffnete_vorn` haelt es fest.

**Der neue Zuschnitt hat die Ueberschneidungen vermehrt.** Seit
`Auszeichnung::Listenzeile { tiefe }` ueberlappen sich auch die Absatzstile
verschachtelter Listenpunkte, und in `> - Punkt im Zitat` decken zwei
Listenzeilen verschiedener Tiefe **denselben** Bereich. Fuer diesen Fall
liefert der dritte Sortierschluessel die richtige Antwort (die tiefere
gewinnt), und die Probe `ein_punkt_im_zitat_liegt_eine_ebene_tiefer` prueft
genau ihn.

**Was fehlt: Punkt 2, das Zusammenlegen der Schrift.** Wo zwei
schriftsetzende Auszeichnungen einander enthalten, geht die aeussere fuer den
ueberlappten Bereich weiterhin vollstaendig verloren: in
`*kursiv **fett** wieder kursiv*` ist „fett" fett und nicht mehr kursiv. Fett
**und** kursiv oder feste Schrift **und** fett brauchten einen Schriftzustand
je Stelle statt eines Ersetzens — etwa `applyFontTraits:range:` fuer die beiden
Betonungen und zwei Durchgaenge in `anwenden`, erst Schrift setzen, dann Schnitt
zulegen.

**Warum das hier nicht gebaut ist.** Es ist eine Verhaltensaenderung an AppKit
in einer Datei, die keine einzige Probe traegt (Datensatz
`260812-1805_*_textmerkmale-rs-traegt-keine-einzige-probe.md`), und die
Wirkung laesst sich ohne Vordergrundlauf nicht sehen. Ein ungeprueftes
Umschreiben von `anwenden` neben zwei gemessenen Behebungen waere die
schlechtere Wahl gewesen. Der Kommentar an Ort und Stelle nennt die fehlende
Faehigkeit und verweist auf diesen Datensatz.

---
Resolved: Punkt 1 (Kommentar berichtigt) und Punkt 3 (Reihenfolge über einen Rang statt über
die stabile Sortierung) sind mit `a9e1149` erledigt. Punkt 2, das Zusammenlegen der
Schriftschnitte, ist kein verbleibender Teil dieses Datensatzes mehr: er steht seit dem
260812-1851 als eigener, vom Nutzer zurückgestellter Datensatz
`issues/260812-1851_*_zwei-schriftschnitte-legen-sich-nicht-zusammen-fett-in-kursiv-bleibt-aufrecht.md`,
mit dem Auslöser, der die Frage wieder aufmacht. Dieser Datensatz hatte danach nichts Eigenes
mehr offen und blieb nur stehen, weil keiner der beiden den anderen nannte; ein `find` über
offene Punkte lieferte damit Arbeit, die der Nutzer vertagt hat. Gefunden von der Durchsicht
`reviews/260812-1920-coderev-turn-3-der-runde-6.md`, abgelegt als
`issues/260812-1920_*_dieselbe-verbleibende-arbeit-steht-zweimal-einmal-offen-und-einmal-zurueckgestellt.md`.
Der Fall der Überschrift, die ihre Schriftgröße verliert, ist ebenfalls ausgezogen und steht als
`issues/260812-1920_*_eine-auszeichnung-in-einer-ueberschrift-verliert-deren-schriftgroesse.md`.

