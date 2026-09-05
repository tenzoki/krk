Das atomare Schreiben verliert Besitzer, erweiterte Attribute und Zugriffslisten der ersetzten Datei

---
**Der Restbestand aus der Behebung von `260904-1902`.** Dort ging es um die Rechtebits, und die überleben das Schreiben seit dem 260905. Alles andere, was am ersetzten Verzeichniseintrag hing, tut es nicht, und der Grund ist derselbe: `Nachbardatei::umbenennen` setzt über `rename(2)` einen **neuen** Eintrag an die Stelle des alten. Was an der alten Datei hing, hängt danach an der neuen — und die ist frisch angelegt.

Verloren geht heute, alles am selben Ursprung:

- **Besitzer und Gruppe.** Die Nachbardatei gehört dem schreibenden Prozess. Eine Datei, die einem anderen Nutzer oder einer anderen Gruppe gehörte und in deren Ordner KRK schreiben darf, gehört danach dem Nutzer. `chown(2)` hat keine Entsprechung in der Standardbibliothek; die Behebung bräuchte `libc`, und `krk-core` führt keines (`CLAUDE.md`, „Technologiewahl").
- **Die erweiterten Attribute.** Auf macOS hängen daran unter anderem die **Finder-Marken** (`com.apple.metadata:_kMDItemUserTags`), der Finder-Kommentar, die Quarantäneangabe und die Herkunft eines geladenen Dokuments. Wer eine markierte Datei in KRK sichert, verliert die Marke, und nichts sagt es ihm. `listxattr`/`getxattr`/`setxattr` brauchen ebenfalls `libc`.
- **Die Zugriffslisten (ACLs) und die Dateiflags** (`uchg`, `hidden`). Dieselbe Lage, dieselbe Sperre.
- **Das Anlagedatum** (`st_birthtime`). Es steht danach auf jetzt.
- **Harte Verweise.** Trug die Datei einen zweiten Namen, so zeigt dieser nach dem Sichern weiter auf den **alten** Inhalt: der neue Stand steht nur unter dem Namen, den KRK geschrieben hat.
- **Eine symbolische Verknüpfung als Ziel.** `text::datei::oeffnen` und `Stempel::von_pfad` folgen ihr, das Schreiben ersetzt sie. Wer eine Verknüpfung im Editor sichert, hat danach an ihrer Stelle eine gewöhnliche Datei, und das Verknüpfungsziel steht unverändert. **Dieser eine Punkt ist ohne `libc` behebbar** — `fs::canonicalize` vor der Wahl des Ziels —, und er ist deshalb der nächstliegende Kandidat.

Nicht in dieser Liste steht das **Änderungsdatum**: es steht nach dem Sichern auf jetzt, und das ist richtig, denn die Datei ist gerade geändert worden. `Editormodell::sichern` zieht seinen Stempel unmittelbar danach nach (`crates/krk-ui/src/editormodell.rs:1010`), und `Stempel` trägt Änderungszeit und Größe und keine Inode-Nummer; der Wechsel des Eintrags stört den Vergleich in `fremd_geaendert` also nicht.

Ebenfalls nicht in der Liste stehen `setuid`, `setgid` und `sticky`. Sie werden **bewusst** nicht übertragen, und die Begründung steht am Doc-Kommentar von `atomar::RECHTEMASKE`: die Nachbardatei gehört dem schreibenden Nutzer, das ersetzte Ziel muss ihm nicht gehört haben, und ein mitgetragenes `setuid` übertrüge das Recht eines fremden Besitzers auf einen Inhalt, den dieser Nutzer geschrieben hat. Ob diese Wahl so bleibt, ist eine Nutzerfrage und keine Sache dieses Datensatzes.

**Warum das nicht mit `260904-1902` behoben ist.** Jeder Punkt außer dem letzten verlangt `libc` in `krk-core`, und das ist eine Änderung der Bauvoraussetzungen und damit eine Nutzerentscheidung. Der Auftrag jener Behebung waren die Rechte, und der Rest ist hier benannt statt verschwiegen.

**Abnahmetest**, falls behoben wird: eine Datei im Finder mit einer Marke versehen, sie in KRK öffnen, ändern und sichern; die Marke steht danach noch daran. Und: eine symbolische Verknüpfung im Editor sichern; sie ist danach noch eine Verknüpfung, und ihr Ziel trägt den neuen Stand.

---
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Domain:** code
Gefunden bei der Behebung von `260904-1902_*_das-atomare-schreiben-weitet-die-rechte-einer-600-datei-auf-644.md`, Fassung v1.7.0. Nicht am laufenden Programm gemessen, sondern aus dem Schreibweg gelesen: `crates/krk-core/src/ablage/atomar.rs`, `vorbereiten` und `Nachbardatei::umbenennen`.
