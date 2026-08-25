# Wie erreichen neue Auslieferungsprofile einen Nutzer, der KRK schon einmal gestartet hat?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1 fuer diese Runde (Handgriff des Nutzers, im README beschrieben), Moeglichkeit 2 als Gegenstand einer spaeteren Runde. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `crates/krk-core/src/ablage/leseprofile.rs` (`anlegen_falls_fehlt`); `resources/default-readers.toml`; `README.md`

---

## Question

`resources/default-readers.toml` bekommt in der Runde 18 drei neue Profile und mehrere neue
Zeilen. Die Datei erreicht damit **niemanden**, der KRK schon einmal gestartet hat.

`ablage::leseprofile::anlegen_falls_fehlt` schreibt die Auslieferungsfassung nur, wenn
`~/Library/Application Support/KRK/readers.toml` fehlt. Steht sie da, wird sie nicht angefasst,
„gleich was in ihr steht (C1.2). Auch eine leergeräumte bleibt leer; der Nutzer hat sie so
gewollt." Das ist eine ausdrückliche Zusage der Runde 16 und keine Lücke.

Auf dem Gerät, auf dem entwickelt wird, steht die Datei seit dem 260824. Die Arbeit dieser
Runde wäre nach dem Auslieferungslauf unsichtbar, und zwar ohne jede Meldung: KRK arbeitete
weiter mit den fünf Profilen von gestern.

## Options

1. **Der Nutzer räumt die Datei von Hand weg, KRK legt sie beim nächsten Start neu an.**
   Dokumentiert in `README.md` und im Bericht dieser Runde.
   - Pros: Kostet keinen Code und keine Zusage. Nutzt genau den Weg, den `anlegen_falls_fehlt`
     schon hat. Der Nutzer sieht, was er tut.
   - Cons: Eigene Änderungen an der Datei gehen verloren, wenn er sie nicht vorher sichert.
     Ein Nutzer, der die Stelle im `README.md` nicht liest, bekommt die neuen Profile nie und
     hat keinen Anhaltspunkt, warum.
2. **Ein Befehl „Leseprofile auf die Auslieferungsfassung zurücksetzen"** im Hauptmenü.
   - Pros: Im Programm auffindbar, kein Umgang mit dem Dateisystem.
   - Cons: Ein neues `Kommando` mit allem, was daran hängt: eine Zeile in
     `Kommando::wirkungsbereich`, eine in `bereich_des_kommandos`, ein Eintrag in
     `resources/default-keymap.toml` und — die Falle, die `CLAUDE.md` unter „Was man nicht
     sieht" beschreibt — ein Zweig in `Anwendungsdelegierter::kommando_ausfuehren`, den der
     Übersetzer **nicht** einfordert. Ohne ihn steht der Befehl im Menü und tut nichts. Dazu
     eine Rückfrage vor dem Überschreiben. Das ist eine eigene Runde wert und nicht ein
     Nebenschritt dieser.
3. **KRK ergänzt beim Start fehlende Auslieferungsprofile in der Nutzerdatei.**
   - Pros: Wirkt ohne Zutun.
   - Cons: Bricht die Zusage C1.2 und die Bauart dahinter. `Profildatei` trägt bewusst kein
     `Serialize`, weil ein Schreibweg die rund 180 Kommentarzeilen wegnähme, die der Zweck der
     Datei sind. Und ein Profil, das der Nutzer absichtlich herausgenommen hat, käme zurück.
4. **Die Auslieferungsfassung bekommt eine Fassungsnummer, und KRK meldet in der Statuszeile,
   wenn die Nutzerdatei älter ist.**
   - Pros: Der Nutzer erfährt davon im Programm, ohne dass etwas überschrieben wird.
   - Cons: Ein neues Feld in der Gestalt der Datei und damit ein Bruch für jede vorhandene
     Nutzerdatei, die es nicht trägt — oder ein optionales Feld, dessen Fehlen „alt" heißt und
     das damit bei jedem Start meldet, bis der Nutzer handelt. Und es behebt nichts, es sagt
     nur Bescheid. Möglichkeit 2 wäre der bessere zweite Schritt danach.

## Constraints

- C1.2 der Runde 16 bleibt: KRK schreibt die Nutzerdatei nach ihrer Anlage nicht mehr.
- Die Kommentarzeilen der Auslieferungsfassung sind der Zweck der Datei und dürfen auf keinem
  Weg verloren gehen.
- Der Nutzer dieses Vorhabens **muss** die neuen Profile bekommen, sonst ist die Arbeit dieser
  Runde für ihn nicht vorhanden.

## Recommendation

**Möglichkeit 1 für diese Runde, Möglichkeit 2 als eigener Gegenstand einer späteren.**

Der Weg lautet im Einzelnen, und er gehört in dieser Ausführlichkeit in den Bericht der Runde
und in `README.md`:

1. KRK beenden.
2. `~/Library/Application Support/KRK/readers.toml` beiseitelegen, nicht löschen — etwa nach
   `readers.toml.alt`. Wer eigene Profile darin hat, holt sie sich daraus zurück.
3. KRK starten. Die Datei entsteht neu aus der Auslieferungsfassung, samt Kommentaren.

Der Schritt 2 heißt beiseitelegen und nicht löschen, aus demselben Grund, aus dem die
Betriebsregel dieses Vorhabens beim Installieren „darüberkopieren und die alte nicht vorher
löschen" lautet: der Bestand des Nutzers liegt außerhalb des Bündels, und ein Werkzeug oder ein
Handgriff, der ihn mitnimmt, hat ihn genommen.

Dass diese Runde eine Nutzerhandlung verlangt, ist **kein** Nebenbefund, den ein Bericht in
einem Halbsatz erledigt. Ohne sie ist von der halben Runde nichts zu sehen, und der Nutzer
hätte keinen Anhaltspunkt, warum: eine unveränderte `readers.toml` verhält sich vollkommen
richtig. Der Bericht der Runde nennt den Weg deshalb ausdrücklich und nicht unter „Details".
