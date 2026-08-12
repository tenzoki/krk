# Wie viele Obermenüs trägt die Menüleiste, wenn alle 81 Funktionen darin stehen?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` (C2), `crates/krk-ui/src/appkit/menue.rs:277-371`, `crates/krk-ui/src/belegungsmodell.rs:73-131`

---

## Frage

Das Hauptmenü trägt heute drei Obermenüs mit zusammen zehn Befehlen: „KRK" mit zweien, „Bearbeiten" mit sechs, „Fenster" mit zweien (`menue.rs:277-371`, am 260813 gezählt). Der Nutzerwunsch verlangt jeden Tastenbefehl auch über das Menü, also 81 Funktionen. Die Gliederung dafür steht schon: `Funktionsbereich` führt neun Bereiche, und `belegungsmodell::nach_bereichen` ordnet jede Funktion genau einem zu. Offen ist allein, wie diese neun Bereiche auf die Menüleiste fallen.

## Möglichkeiten

1. **Neun Obermenüs, eines je Funktionsbereich.** Die Reihenfolge folgt der Mac-Gewohnheit: „Anwendung" steht vorn und heißt dort KRK, „Fenster" steht hinten, die sieben übrigen dazwischen in der Reihenfolge von `Funktionsbereich::ALLE`.
   - Dafür: Eine Gliederung, drei Abnehmer. Die Belegungsansicht, die Markdown-Ausgabe der Runde 3 und das Menü zeigen dieselben neun Abschnitte in derselben Ordnung; wer eine Funktion in der Ansicht unter „Vorschau" gefunden hat, findet sie im Menü unter „Vorschau". Eine zweite Gruppierung entsteht nicht, und `nach_bereichen` sagt das in seinem eigenen Doc-Kommentar bereits über zwei Abnehmer zu.
   - Dagegen: Neun Obermenüs sind für ein Programm mit der Maxime „supersimpel" viel. Der Finder trägt sechs, Xcode zwölf.
2. **Fünf Obermenüs, mit Untermenüs für die kleineren Bereiche.** Etwa KRK, Bearbeiten, Ansicht, Fenster und ein Sammelmenü „Befehle", das Tabs, Vorschau, Editor und Leiste als Untermenüs führt.
   - Dafür: Eine kürzere Menüleiste.
   - Dagegen: Die Zuordnung Bereich → Obermenü wäre eine zweite Gliederung neben `Funktionsbereich`, von Hand gepflegt und ohne Zwang, vollständig zu bleiben. Ein Befehl liegt danach zwei Ebenen tief, und die Zusage „über das Menü erreichbar" wird für ihn mühsamer als der Tastenbefehl, den sie ergänzen soll.
3. **Ein Obermenü „Befehle" mit neun Untermenüs, daneben die drei heutigen.** Die Menüleiste wächst um genau einen Eintrag.
   - Dafür: Die kürzeste Leiste, und die Gliederung bleibt die eine.
   - Dagegen: Jeder Befehl liegt zwei Ebenen tief, und die vier Befehle, die heute in „Bearbeiten" und „Fenster" stehen, stünden zweimal oder müssten aus ihrer Gliederung herausgenommen werden.

## Randbedingungen

- Der Titel des ersten Obermenüs ersetzt macOS durch den Namen aus der `Info.plist`. Das erste Obermenü ist damit zwingend das Anwendungsmenü, und `Funktionsbereich::Anwendung` (Belegungsansicht und Beenden) gehört dorthin.
- Der Eintrag „Tastenbelegung als Markdown sichern" der Runde 3 trägt bewusst keine Kennung und steht in keiner Belegung. Er bleibt, wo er ist, im Anwendungsmenü über dem Beenden.
- Ein Obermenü „Bearbeiten" muss es weiterhin geben und es muss so heißen: macOS hängt Textbefehle und Systemzusätze an ein Menü dieses Namens, und `systemzusaetze_unterdruecken` setzt genau dort an. `Funktionsbereich::Textbefehle` ist dieses Menü.

## Empfehlung

Möglichkeit 1. Die neun Bereiche sind keine Erfindung dieser Runde, sondern eine vom Nutzer am 260806 bestellte und seither in zwei Oberflächen gezeigte Ordnung. Sie ein drittes Mal unverändert zu verwenden kostet nichts und hält die Zusage, dass es über die Funktionen dieses Programms genau eine Gliederung gibt. Möglichkeit 2 kauft eine kürzere Leiste mit einer zweiten Gliederung, und das ist in diesem Baum der teurere Handel.

Die Runde fährt bis zu einer Antwort auf Möglichkeit 1.

---
Answered:
Implemented:
Deferred:
Superseded by:
