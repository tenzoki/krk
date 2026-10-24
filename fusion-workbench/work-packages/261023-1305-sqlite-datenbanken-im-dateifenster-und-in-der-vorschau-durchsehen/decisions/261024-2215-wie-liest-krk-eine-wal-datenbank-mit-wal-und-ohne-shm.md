# Wie liest KRK eine WAL-Datenbank mit `-wal` und ohne `-shm`?

---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** issues/261024-2112-mode-ro-legt-neben-einer-wal-datenbank-mit-wal-und-ohne-shm-eine-neue-shm-an.md; analyses/261024-2133-shm-neben-wal-ohne-shm-zusage-korrigieren-oder-anlegen-verhindern.md; decisions/261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md

---

## Frage

Steht neben einer WAL-Datenbank ein `-wal`, aber keine `-shm`, legt die nur lesende Verbindung (`mode=ro`) eine neue `-shm` an. Das widerspricht der Prämisse des Entscheids 1605 („eine neue Datei legt KRK nie an“). Der Fall ist häufig: Programme im Sperrmodus `exclusive` (Firefox, Safari, Electron-Programme wie Dropbox) legen nie eine `-shm` an.

## Möglichkeiten

1. Zusage korrigieren, `mode=ro` bleibt; SQLite legt in diesem Fall eine `-shm` an.
2. Ohne Sperre lesen (`vfs=unix-none` + `locking_mode=exclusive`, nach `F_GETLK`-Vorfrage). Empfehlung des Analysten.
3. Solche Datenbanken nicht betreten, Grund melden.
4. Ein leeres `-wal` (0 Bytes) ohne `-shm` zählt wie ein fehlendes: `immutable=1`, nichts wird angelegt, gelesen wird der volle Stand, weil ein leeres `-wal` nichts Neueres trägt. Für ein nicht leeres `-wal` ohne `-shm` gilt Möglichkeit 1, und die Zusage nennt genau diesen Rest. Vorschlag des Consultants.

## Abwägung

- Möglichkeit 2 ist nachgemessen unsicher: Schreibt ein anderes Programm während der offenen Verbindung und fährt einen Checkpoint mit TRUNCATE, liefert die nächste Anweisung `SQLITE_CORRUPT` oder `SQLITE_IOERR_SHORT_READ` (drei Läufe, normaler und exklusiver Schreiber). Die Datenbank selbst blieb heil. Der häufigste Auslöser des Falls (Firefox startet, während KRK liest) ist genau dieses Bild, und die Vorfrage sieht ein später startendes Programm nicht. Dazu kämen eine dritte Öffnungsform und `F_GETLK` im `unsafe`-Teil.
- Erhebung allein über `stat`: 29 `-wal` ohne `-shm`, davon 23 leer (Firefox, Safari, SharedStorage, CallHistory). Möglichkeit 4 deckt diese ohne neue Datei und mit richtigen Daten.
- Eine von KRK angelegte `-shm` im Rest richtet gemessen keinen Schaden an: ein exklusiver Schreiber ignoriert sie, ein normaler Schreiber danach hinterlässt eine heile Datenbank; auf diesem Mac bleiben `-wal` und `-shm` auch nach sauberem Ende eines normalen Schreibers stehen.
- Eine angelegte `-shm` danach wieder zu löschen, kann zwei Indizes für eine Datenbank erzeugen und die Datenbank beschädigen; verworfen.

## Antwort

Möglichkeit 4, für den Rest Möglichkeit 1.

Answered: 261024-2215, orchestrator im Auftrag des Nutzers („you can go on in automode from here and invlove the consultant as sparring partner“, Paketnarrativ, Abschnitt „Nutzerwort 261024“), nach Analyse 261024-2133 und Gegenprüfung durch den Consultant. Möglichkeit 4, Rest nach Möglichkeit 1.
