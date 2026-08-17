# Jeder Löschweg fragt nach, und es gibt nur noch den Papierkorb

---
**Domain:** code
**Status:** active
**Filed by:** orchestrator
**Active spec/plan:** circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md (Spec: shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md)
**Active session history:** circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/history/260817-1208-orchestrator-session.md

---

## Directive

KRK kennt nach dieser Runde genau einen Löschweg, und er führt in den Papierkorb des Systems. Jeder Datei- und Ordner-Löschvorgang fragt vorher genau einmal nach, mit „Abbrechen" vorbelegt; wo das Ziel für den Nutzer, für seine Daten oder für den Umfang des Vorgangs ungewöhnlich ist, trägt dieselbe Rückfrage ein Warnzeichen und nennt den Grund in ihrer ersten Zeile. Ein Ziel ohne Papierkorb wird nicht gelöscht, sondern gemeldet. Der Befehl zum endgültigen Löschen fällt aus der Anwendung, aus der Belegung und aus dem Menü.

## Grounding snapshot

Erhoben am 260817 gegen den Baumstand `b8e198e`, Version 0.5.0. Die zwölfte gefahrene Runde des Projekts.

### Woher das Vorhaben kommt: ein Schadensfall am eigenen Baum

In der Nacht zum 260817 hat KRK, von Hand bedient, den Ordner `fusion-workbench/shared` des eigenen Projektverzeichnisses in den Papierkorb geräumt: 189 verfolgte Dateien in zehn Unterordnern, ein Tastendruck, keine Rückfrage. Der Verlust blieb vier Stunden unbemerkt und wurde durch einen `git status` gefunden, den der Orchestrator aus einem anderen Grund fuhr. Wiederhergestellt wurde aus `HEAD`; eine unverfolgte Datei war nur zu retten, weil der Nutzer den Ordner im Papierkorb noch fand.

Die Forensik steht in `shared/analyses/260817-0419-verlust-des-speichers-shared.md` und belegt den Hergang mit vier unabhängigen Messungen. Der Nutzer hat danach einen gleichartigen Vorfall auf einem zweiten Gerät berichtet; die Fehlbedienung ist damit belegt und nicht mehr erschlossen.

Der Befund selbst war schon sechs Stunden vor dem Schaden abgelegt, als Risiko: `shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md`, gefunden vom Shaper beim Lesen des Baums für eine andere Runde. Er trägt den Stand am Baum, die vier nachzuziehenden Stellen und fünf Fragen, die der Umsetzer nicht raten darf, und hat der Klärung dieser Runde den größten Teil der Vorarbeit abgenommen.

### Was diese Runde aufhebt

Der heutige Zustand ist keine Nachlässigkeit, sondern eine umgesetzte Nutzerfestlegung. `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md` hält die Antwort vom 260802-1105: „Delete löscht in Papierkorb, FN+F8 endgültig". Der Datensatz trägt `_i_`, nennt die Commits `daecb45` und `343a7f3` und führt die fehlende Rückfrage ausdrücklich als gewollten Preis. Nach dieser Runde stimmt kein Teil davon mehr.

Die Festlegung steht an sechs weiteren Stellen, darunter die Directive der Runde 1 und neun bindende Zeilen ihres Specs. Das Nachziehen ist deshalb ein eigener Gegenstand der Runde (C6) und kein Nebeneffekt: eine überholte Zusage, die stehen bleibt, wird von der nächsten Runde als bindend gelesen.

### Die elf Antworten des Nutzers

Sie stehen ausformuliert im Spec und im neuen Entscheidungsdatensatz `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md` und werden hier nicht wiederholt. In Stichworten: Rückfrage vor jedem Vorgang mit Abbrechen vorbelegt; laute Warnung als dasselbe Blatt mit Warnzeichen, Pfad, Zahl und Grund in der ersten Zeile; Umfangsschwelle bei 25 Einträgen im Unterbaum, gedeckelt gezählt; vier entscheidbare Zielarten plus Umfang plus Git-Arbeitsbaum; kein endgültiger Löschweg mehr; kein Löschen auf Zielen ohne Papierkorb; `ctrl+delete` für Lesezeichen bleibt ohne Rückfrage; kein Protokoll in dieser Runde; `f8` zeigt künftig auf den Papierkorb; eine gespeicherte `keymap.toml` mit der entfallenen Kennung wird wie heute verworfen; die Git-Prüfung sieht aufwärts.

Die letzte dieser Antworten hat der Nutzer am Spec-Gate umgedreht, nachdem die Kalibrierung des Shapers gezeigt hatte, dass die enge Form seinen eigenen Schadensfall nicht trifft. Der Einwand des Shapers dagegen bleibt gültig und steht im Spec: in diesem Projekt warnt danach fast jede Löschung im Quellbaum laut.

### Was nicht entscheidbar ist

Zwei der vier Zielklassen, die der Nutzer genannt hat, sind so nicht entscheidbar, und der Spec sagt es statt es zu überspielen. „Clouddrive" geht als benannte Orte (`~/Library/CloudStorage/`, `~/Library/Mobile Documents`), nicht als Klasse. „Gesharte Verzeichnisse" zerfällt in drei Dinge, von denen nur das eingehängte Netzlaufwerk sauber entscheidbar ist. KRK prüft, was es prüfen kann, und behauptet über den Rest nichts.

Daneben gilt: **unentschieden gilt als laut.** Eine Prüfung, die im Zweifel schweigt, wäre in genau den Lagen still, in denen KRK am wenigsten über das Ziel weiß.

## Dependencies

- `circles/260802-0842-krk-mac-dateimanager-editor-git` — die Runde 1. Sie hat beide Löschwege gebaut, das Bestätigungsblatt, die Norton-Reihe mit F8 als Löschtaste und die zehn Zeitzusagen. Ihre Directive und neun Zeilen ihres Specs tragen die Festlegung, die diese Runde aufhebt; C6 zieht sie nach.
- `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief` — die Runde 10. Sie hat die Rückschritt-Regel gebaut, deren Modulkopf seine Begründung darauf stützt, dass das Räumen ohne Rückfrage laufe. Die Regel bleibt nötig, ihr Modulkopf ist nachzuziehen. Sie hat daneben „unentschieden" vom negativen Befund getrennt (`verzeichnis::sys::ist_deskriptormangel`), worauf die Zählung der Umfangsschwelle aufsetzt.
- `circles/260816-2255-befehle-absetzen-und-makros-speichern` — zurückgestellt am 260817 zugunsten dieser Runde. Spec und Plan liegen vollständig; wer sie aufnimmt, legt einen neuen Circle an.

**Bindende Datensätze:**

- `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md` — wird von dieser Runde überholt und wandert in C6 auf `_s_`.
- `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md` — die Antwort, auf der diese Runde steht.
- `shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md` — offen, und diese Runde berührt seine Kehrseite: eine **entfallene** Funktion in einer gespeicherten Belegung. Der Nutzer hat entschieden, es beim heutigen Verhalten zu belassen.

## Turn log

- Turn 1 (Sitzung 260816-2113): Commits `664a0fd`..`472eb81`, dazu `a8b4bf8` für die
  Durchsicht. Bündel A vollständig, drei Schritte. **Die Schutzschwelle ist erreicht:** KRK
  fragt vor jedem Räumen nach, und die Durchsicht hat jeden Löschweg einzeln nachgezählt.
  Kohärenz-Befund `ok`. Sieben Befunde gefiled, davon einer hoch — eine unbekannte
  Blattantwort fällt im Löschblatt auf die zerstörende Schaltfläche. Ein Defekt, den der
  Spec nicht kannte, ist mitbehoben: der bestätigte Auftrag trägt jetzt die gezeigte
  Auswahl statt einer zweiten Lesung nach dem Blatt. Sitzungsprotokoll:
  `shared/history/260816-2113-orchestrator-session.md`. Der Nutzer hat die Sitzung nach
  diesem Turn beendet; Bündel B bis E stehen aus.
- Turn 1 (Sitzung 260817-1208): Commits `873b9f4`..`ee85950`, dazu `1a57418` für die
  Durchsicht. Die sieben Befunde des Bündels A und das ganze Bündel B, fünf Aufgaben.
  Zuerst der hohe Befund: die Vorbelegung einer unbekannten Blattantwort geht im Löschblatt
  von der zerstörenden auf die abbrechende Stelle, und die zwei widersprechenden
  Vorbelegungen in `blaetter/mod.rs` sind zu einer geworden. **Die zweite Stufe des
  Schutzes steht:** ein Ziel ohne Papierkorb wird gemeldet statt gelöscht. Die Stufenfolge
  vor der Rückfrage ist als reine Funktion `vor_der_rueckfrage` nach `kommandos/` gezogen
  und erstmals ohne Fenster prüfbar. Kohärenz-Befund `ok`. Sieben Datensätze aus der
  Durchsicht, zwei mittel, keiner hoch.
- Turn 2 (Sitzung 260817-1208): Commits `17d3550`..`792995a`, dazu `e313841` für die
  Durchsicht. Das ganze Bündel C, sechs Aufgaben, dazu die vorgezogene Umbenennung auf
  `Loeschzielbefund`. **Die dritte Stufe steht:** die Rückfrage trägt Warnzeichen und Grund,
  wenn ein Ziel ungewöhnlich oder der Umfang groß ist. Ein Nutzerentscheid am 260817-1640
  hat `ist_lokal` in `liegt_auf_netzlaufwerk` umbenannt, weil die Funktion die Umkehrung
  ihres Feldes lieferte und der Übersetzer die Verdrehung nicht sieht. Kohärenz-Befund `ok`,
  Abgleichsverdikt `review-needed` mit sechs Driftpunkten, alle in der Buchführung und
  keiner am Code. Neun Datensätze aus der Durchsicht, zwei mittel, keiner hoch. Die
  Abdeckung der Durchsichten schließt lückenlos über beide Turns. Der Nutzer hat den
  Zuschnitt auf Befunde plus Bündel B und C gewählt; **Bündel D und E stehen aus**, das
  endgültige Löschen ist damit noch im Programm.

## Closure note

(offen)
