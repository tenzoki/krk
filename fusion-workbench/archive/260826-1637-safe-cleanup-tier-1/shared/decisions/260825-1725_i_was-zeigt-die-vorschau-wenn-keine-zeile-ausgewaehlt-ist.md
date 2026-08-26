# Was zeigt die Vorschau, wenn keine Zeile ausgewählt ist?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1: die Vorschau beschreibt den ausgewaehlten Eintrag und ohne Auswahl den angezeigten Ordner, fuer jeden Ordner und nicht nur die Projektwurzel. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `crates/krk-ui/src/appkit/anwendung.rs:1690` (`vorschau_fuellen`); `crates/krk-ui/src/appkit/tabelle.rs:2063` (`auswahl_merken`); `crates/krk-ui/src/appkit/tabelle.rs:1448` (`nach_lesebeginn`)

---

## Question

Der Nutzer verlangt am 260825: steht er im Projektwurzelverzeichnis und hat noch keine Zeile
angewählt — etwa unmittelbar nach dem Eintritt in den Ordner —, soll die Vorschau die
Projektübersicht zeigen, also dasselbe wie auf `./fusion-workbench/`.

Heute geschieht nichts. `Anwendungsdelegierter::vorschau_fuellen` nimmt ein `Option<PathBuf>`
entgegen und kehrt bei `None` zurück, mit einer ausgeschriebenen Begründung: „Eine aufgehobene
Auswahl lässt den Tab stehen; das Zustandsdiagramm des Specs kennt allein die **neue** Auswahl
als Auslöser." Der Vorschau-Tab zeigt also weiter, was vor dem Ordnerwechsel darin stand.

Die Frage ist, für **welche Ordner** die neue Regel gilt. Für das Projektwurzelverzeichnis
allein, oder für jeden Ordner ohne ausgewählte Zeile?

## Options

1. **Für jeden Ordner: ohne ausgewählte Zeile beschreibt die Vorschau den angezeigten Ordner
   selbst.**
   - Pros: Eine Regel ohne Ausnahme, überschneidungsfrei und vollständig: die Vorschau
     beschreibt, was ausgewählt ist, und ist nichts ausgewählt, ist es der Ordner. Sie ist von
     der Anwendung entscheidbar, weil „ist eine Zeile ausgewählt" eine Frage ist, die sie
     beantworten kann. Der heutige Zustand — der Tab zeigt eine Datei aus einem Ordner, in dem
     der Nutzer nicht mehr steht — ist ohnehin die schwächere Auskunft. Trifft kein Profil,
     zeigt die Vorschau die Metadaten des Ordners, also genau das, was sie bei angewählter
     Ordnerzeile auch zeigte.
   - Cons: Verhaltensänderung in jedem Ordner und nicht nur dort, wo der Nutzer sie verlangt
     hat. Jeder Ordnerwechsel kostet eine Zusammenfassung, also bis zu zwölf Verzeichnisleseläufe
     — auf dem Arbeitsfaden der Vorschau und nicht auf dem Hauptfaden, aber sie fällt an.
2. **Nur für das Projektwurzelverzeichnis.**
   - Pros: Die kleinste sichtbare Änderung.
   - Cons: „Projektwurzelverzeichnis" ist für die Anwendung kein Begriff. Sie müsste ihn aus
     etwas erschließen — dem Vorhandensein eines `fusion-workbench`-Ordners, dem Startordner,
     einer Liste —, und jede dieser Antworten ist eine Näherung an eine Frage, die die Vorschau
     nicht zu stellen hat. Damit wanderte Projektwissen in die Anzeige, wo heute allein die
     Leseprofile es tragen, und der Nutzer könnte es in `readers.toml` nicht mehr ändern.
3. **Ein eigener Schalter oder Befehl** („den angezeigten Ordner zusammenfassen").
   - Pros: Nichts geschieht ungefragt.
   - Cons: Ein weiterer Befehl für eine Auskunft, die der Nutzer sehen will, ohne sie
     anzufordern. Und er träfe auf die Falle, die `CLAUDE.md` unter „Was man nicht sieht"
     beschreibt: ein Kommando ohne eigenen Zweig im Ausführungspfad übersetzt, besteht jede
     Probe und tut nichts.

## Constraints

- Die Zusammenfassung darf nicht auf dem Hauptfaden entstehen; C4.7 und die Zusage L7 hängen
  daran, dass `zusammenfassen` allein auf dem Arbeitsfaden von `Vorschaumodell::datei_anzeigen`
  läuft.
- Ein Ordner, den der Nutzer nie anwählt, darf keinen Leselauf kosten. Das gilt weiter: der
  angezeigte Ordner ist ein angewählter.
- Es darf kein zweiter Weg in die Vorschau entstehen. `datei_anzeigen` bleibt der eine Eingang,
  und `auswahl_merken` bleibt die eine Stelle, die eine Zeile in einen Eintrag übersetzt.

## Recommendation

**Möglichkeit 1.** Die Regel lautet dann: **die Vorschau beschreibt den ausgewählten Eintrag,
und ohne Auswahl den angezeigten Ordner.** Sie ist mit einem Satz zu sagen, hat keine Ausnahme
und braucht kein Projektwissen in der Anwendung.

Was der Nutzer für das Projektwurzelverzeichnis verlangt, folgt daraus zusammen mit einem
**Profil** und nicht mit einer zweiten Regel: `resources/default-readers.toml` bekommt einen
Block, der einen Ordner an der Kennzeichendatei `^fusion-workbench$` erkennt und dieselben
Zeilen führt wie das Wurzelprofil, jede mit `fusion-workbench/` vor der Ortsangabe. Damit
beantwortet der Nutzer die Frage „welcher Ordner ist eine Projektwurzel" dort, wo er sie ändern
kann.

Der Preis ist zu nennen und wird nicht wegerklärt: die sieben Zeilen stehen zweimal in der
Datei, einmal für die Werkbank und einmal für das Verzeichnis darüber, und sie können
auseinanderlaufen. Ein Mechanismus dagegen — Vererbung, Verweis, Vorlage — wäre neu, und das
Profilformat kennt heute nichts dergleichen. Für zwei Blöcke in einer Datei, die der Nutzer von
Hand pflegt, ist ein Kommentar über beiden die angemessene Antwort und ein Mechanismus die
unangemessene.

Zur Umsetzung gehört, dass der Ordnerwechsel den Weg auch wirklich auslöst. Heute meldet
`auswahl_merken` nur, wenn AppKit eine Änderung der Auswahl meldet; war vorher schon nichts
ausgewählt, meldet es nichts. `nach_lesebeginn` ist die eine Stelle, die Navigation und
Auffrischung gemeinsam nachzieht, und dort gehört der Anstoß hin.

---
Implemented: 9322d5d — `DateifensterQuelle::auswahl_merken` meldet über `zu_beschreiben` (`crates/krk-ui/src/appkit/tabelle.rs:488`) ohne Auswahl den angezeigten Ordner statt `None` (`:2189`), und `nach_lesebeginn` stößt den Weg nach `auswahl_anzeigen` an (`:1546`, Begründung `:1549`), damit auch ein Ordnerwechsel ohne Auswahländerung ihn erreicht. Die Regel gilt für jeden Ordner und nicht nur für die Projektwurzel. Zwei Lücken derselben Regel sind gesondert abgelegt und bleiben offen: `shared/issues/260825-1922_*_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md` und `shared/issues/260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md`. Nachgemessen am 260826-0149 gegen den Baum, `make check` grün.
