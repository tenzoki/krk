# Wie wird das beglaubigte Bündel vor dem nächsten Entwicklungsbau geschützt?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md` (der Befund, seit dem 260813 offen und unverändert), `260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md` (was eine Meldung wert ist, die eine Folge nicht nennt)

---

## Frage

`cargo xtask bundle` und `cargo xtask release` legen beide `target/KRK.app` an, über
dieselbe `bundle::Vorlage`. Das ist Absicht: ein zweiter Bündelbauer wäre die zweite
Wahrheit über die Struktur von `KRK.app`. Die Folge ist, dass **jeder** gewöhnliche
Entwicklungsbau ein dort liegendes beglaubigtes Bündel überschreibt, und über `bundle`
hängen `run`, `run-terminal`, `tasten`, `menue`, `durchstich` und `frisch`. Wer nach einem
Auslieferungslauf `make run` tippt, hat die Beglaubigung weg.

Der Unterschied im Preis ist der Punkt: ein Entwicklungsbündel ist in Sekunden wieder da,
ein beglaubigtes verlangt zwei Übersetzungsläufe im Profil `release`, `lipo`, eine Signatur
mit gehärteter Laufzeitumgebung und einen Netzlauf zu Apple.

Der Befund `260813-0026` legt seit dem 260813 drei Zuschnitte nebeneinander und wählt
keinen; ein Abgleich am 260821 und ein zweiter am 260823 haben ihn unverändert offen
gelassen. Die Frage ist eine Nutzerentscheidung, weil jeder der drei Wege ändert, was ein
gewöhnlicher Tastendruck tut. **Sie wird jetzt gestellt, damit der Befund nicht ein viertes
Mal als „nicht entschieden" durch einen Abgleich läuft.**

## Optionen

1. **Getrennte Orte.** `release` legt sein Ergebnis woanders ab, etwa
   `target/release-bundle/KRK.app`.
   - Pro: billig und wirksam; die zwei Wege können einander nicht mehr treffen.
   - Contra: ändert einen Pfad, den `README.md`, das `Makefile`, die Messstrecke und die
     zwei späten Wege (`beglaubigen`, `veroeffentlichen`) nennen. Wer das baut, zählt die
     Stellen nach, statt sie zu schätzen.
2. **`bundle` weigert sich**, ein beglaubigtes Bündel zu überschreiben, solange nicht
   ausdrücklich etwas anderes gesagt wird.
   - Pro: der Ort bleibt einer, und die Frage „trägt das, was dort liegt, ein Ticket"
     beantwortet dieser Baum schon ohne Netz: `traegt_angeheftetes_ticket`
     (`xtask/src/veroeffentlichung.rs`) liest die ersten vier Bytes von
     `Contents/CodeResources`. Der Datensatz von 260813 nennt dafür noch
     `xcrun stapler validate`, das Netz braucht; diese Stelle gab es damals nicht.
   - Contra: ein neuer Ausweg ist nötig (Umgebungsvariable oder Schalter), sonst steht der
     Nutzer nach einer Auslieferung vor einem `make run`, das nicht mehr läuft. Damit
     bekommt der häufigste Weg des Projekts eine Bedingung, die er heute nicht hat.
3. **Nur eine Warnung.** `release` sagt am Ende, dass der nächste Entwicklungsbau das
   Ergebnis nimmt.
   - Pro: billigste Möglichkeit, ändert an keinem Weg etwas.
   - Contra: verhindert nichts, und dieses Projekt hat am 260812 gerade erlebt, was eine
     Meldung wert ist, die eine Folge nicht nennt.

## Constraints

- Ein zweiter Bündelbauer kommt nicht in Frage: `release.rs` begründet die Wiederverwendung
  von `bundle::Vorlage` ausdrücklich, und zwei Wahrheiten über die Struktur von `KRK.app`
  wären teurer als der Befund.
- Was auch gewählt wird, `cargo xtask beglaubigen <zahl>` und
  `cargo xtask veroeffentlichen <zahl>` müssen das beglaubigte Bündel danach noch finden;
  beide fragen heute `bundle::wurzel()`.
- Die Zwischenlösung bleibt bis dahin, was der Befund nennt:
  `ditto target/KRK.app ~/Desktop/KRK.app` vor dem nächsten Entwicklungsbau. Die
  Beglaubigung hängt am Bündel und nicht an seinem Ort, das Ticket reist also mit.

## Recommendation

Keine. Alle drei ändern, was ein gewöhnlicher Tastendruck dieses Projekts tut, und welcher
Preis der kleinere ist, hängt daran, wie oft der Nutzer nach einer Auslieferung noch am
selben Baum baut. Das weiß er und nicht dieser Datensatz.
