# Auf welchem Weg verlassen Zugangsdaten aus `secrets.txt` KRK, wenn eine Verbindung hergestellt wird, und was steht im Lesezeichen?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0229-secrets-mit-sechs-feldern-schluesselcache-und-verbindungen.md, 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md, 260926-0033_*_darf-text-aus-secrets-txt-in-die-zwischenablage.md, 260926-1004_*_eine-textmarke-in-secrets-txt-schreibt-eine-klartextzeile-in-die-lesezeichendatei.md, 260926-1010_*_das-tastenprotokoll-schreibt-jedes-in-secrets-txt-und-ins-pin-blatt-getippte-zeichen-auf-die-standardausgabe.md

---

## Question

Ein Klick auf ein Lesezeichen der Gruppe „Verbindungen“ soll die Verbindung herstellen, mit den Zugangsdaten aus `secrets.txt`. Das ist ein neuer Weg, auf dem Geheimnisinhalt KRK verlässt, und er wird vom Nutzer ausgelöst, nicht von einem Programm nebenbei. Die Regel des Projekts (CLAUDE.md, „Jeder Weg, auf dem Editorinhalt KRK verlässt, muss `Editormodell::haelt_geheimnisse` fragen“) ist für zwei Lecks gebaut worden, die genau daher kamen, dass ein neuer Weg nicht gefragt hat. Zu entscheiden ist, über welchen Kanal das Passwort an das System geht, welche Protokolle KRK überhaupt bedient, was das Lesezeichen in `bookmarks.toml` trägt und wie dieser Weg zur bestehenden Regel steht.

## Options

1. **Das Passwort verlässt KRK allein als Funktionsargument von `NetFSMountURLSync`; alle übrigen Protokolle bekommen eine URL ohne Passwort, und die aufgerufene Anwendung fragt selbst.** Einhängen über NetFS für `smb`, `afp`, `nfs`, `webdav`, `webdavs` und `ftp`; Übergabe an das System über `NSWorkspace` für `http`, `https`, `ssh`, `sftp` und `vnc`, als `<schema>://[konto@]adresse` (bei `http` und `https` ohne Konto). Das Lesezeichen trägt den Anzeigenamen und den Namen des Eintrags, sonst nichts.
   - Pros: das Passwort steht in keiner URL, keinem Prozessargument, keiner Umgebungsvariable, keiner Zwischenablage und keiner Protokollzeile; NetFS nimmt Benutzer und Passwort als eigene Argumente, die eine URL ausdrücklich übersteuern (`NetFS.h`, Kommentar über `NetFSMountURLSync`, geprüft am 261016 im SDK dieses Geräts); ein Kanal, eine Probe.
   - Cons: bei `ssh`, `sftp`, `vnc` und den Webprotokollen tippt der Nutzer das Passwort in der fremden Anwendung selbst; KRK liest es dafür aus `secrets.txt` nicht aus.
2. **Wie 1, zusätzlich legt KRK bei den Übergabeprotokollen das Passwort für kurze Zeit in die Zwischenablage.**
   - Pros: der Nutzer fügt es in der fremden Anwendung nur noch ein.
   - Cons: die Zwischenablage lesen alle Programme und Verlaufsdienste mit; der Entscheid `260926-0033_*_darf-text-aus-secrets-txt-in-die-zwischenablage.md` erlaubt sie als Ausfuhr **durch den Nutzer**, nicht als Ausfuhr durch KRK. Ein „kurz“ hält KRK nicht, sobald ein Verlaufsdienst mitliest.
3. **Passwort in der URL (`smb://konto:passwort@…`, `vnc://konto:passwort@…`).**
   - Pros: jedes Protokoll bekäme seine Zugangsdaten.
   - Cons: die URL landet in Verläufen, Protokollen des Systems und Absturzberichten; vom Auftrag ausdrücklich ausgeschlossen.

## Constraints

- `bookmarks.toml` liegt im Klartext unter `~/Library/Application Support/KRK/`; das Leck `260926-1004` war genau eine Klartextzeile dort.
- Das Tastenprotokoll (`--tasten-protokoll`) zeigt jeden Anschlag eines verdeckten Blattes als `(verdeckt)` (`260926-1010`).
- Ein Klartext, der `secrets.txt` für einen Klick liest, soll den Hauptfaden nicht betreten, und eine halbe Sekunde Ableitung gehört auf einen Arbeitsfaden.

## Recommendation

Wir empfehlen Möglichkeit 1, und die Einordnung zur Regel um `haelt_geheimnisse` lautet so. **Dieser Weg liest keinen Editorinhalt**, sondern den gesicherten Stand der Platte auf einem eigenen Faden (`krk-verbinden`), und fällt deshalb nicht unter die Frage `haelt_geheimnisse`; er ist eine **Ausfuhr durch den Nutzer** derselben Art wie Zwischenablage und Ziehen, mit einem festen Empfänger, dem Einhängedienst des Systems. Gehalten wird er nicht von der Sorgfalt künftiger Rufer, sondern von einem Typ: das Passwort reist im Kern als `Passwort` mit privatem Feld, ohne `Display`, mit verdecktem `Debug` und Tilgen in `Drop`, und es gibt genau eine Methode, die es herausgibt; eine Quelltextprobe hält, dass sie genau einen Rufer hat, die Bindung an NetFS. Klartext, Eintrag und Passwort leben allein auf dem Faden und fallen dort.

**Das Lesezeichen trägt den Namen, unter dem der Eintrag in `secrets.txt` steht, und sonst nichts aus ihm** (`verbindung = "<Name>"` neben `name`). Der Name ist der eine Wert, den der Nutzer als Beschriftung der Leiste sehen will und deshalb im Klartext ablegt; Protokoll, Adresse, Konto, Passwort und Notizen stehen nie in `bookmarks.toml`, und eine Probe hält das an einer gebauten Datei. Gefunden wird der Eintrag beim Klick über den getrimmten Namen; stehen zwei Einträge mit diesem Namen, verbindet KRK nicht und sagt es, weil ein Passwort an den falschen Server schlimmer ist als keine Verbindung. „Verbinden…“ legt deshalb keinen Namen an, der schon steht.

**Verbunden wird allein auf einen ausdrücklichen Anstoß**, einen Mausklick auf die Zeile oder den Befehl „Verbindung herstellen“; eine Auswahl, die mit den Pfeiltasten über die Zeile läuft, verbindet nicht, anders als bei Ordnern und Textmarken, deren Auswahl ihr Ziel öffnet. Ein Einhängen über das Netz ist kein Sprung, den man im Vorbeigehen auslösen sollte.

---
Implemented: crates/krk-ui/src/appkit/verbinden.rs (Modulkopf), crates/krk-core/src/ablage/lesezeichen.rs, crates/krk-core/src/heimordner/verbindung.rs — Passwort allein als NetFS-Argument, Lesezeichen traegt nur den Namen, Verbinden nur auf ausdruecklichen Anstoss; Tilgen nachgeschaerft in 9d1b111
