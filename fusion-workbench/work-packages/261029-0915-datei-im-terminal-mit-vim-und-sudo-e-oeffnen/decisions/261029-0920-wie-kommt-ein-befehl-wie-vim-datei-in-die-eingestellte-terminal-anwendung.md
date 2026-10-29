# Wie kommt ein Befehl wie `vim <datei>` in die eingestellte Terminal-Anwendung: über eine kurzlebige `.command`-Datei oder über AppleScript `do script`?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261029-0915-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 261029-0957-geht-eine-nicht-schreibbare-datei-in-einem-beschreibbaren-ordner-an-sudo-e-oder-an-vim.md, 260805-1623_*_taste-und-einstellbarkeit-des-terminal-befehls.md, 260826-1421_*_pfade-ohne-gueltiges-utf-8-vier-huellen-glaetten-still-mit-to-string-lossy-und-drei-weisen-ab.md

---

## Question

Der bestehende Befehl „Ordner im Terminal öffnen“ (`ctrl+o`, `Kommando::TerminalOeffnen`) übergibt der Anwendung aus `settings.toml` (Schlüssel `terminal`, eine Bündelkennung) einen Ordner über `NSWorkspace openURLs:withApplicationAtURL:configuration:completionHandler:` (`crates/krk-ui/src/appkit/terminal.rs`, `ordner_oeffnen`). Ein Befehl lässt sich auf diesem Weg nicht mitgeben. Der neue Tastenbefehl des Pakets braucht aber genau das: je nach Schreibrecht an der Datei soll `vim <datei>` oder `sudo -e <datei>` in einem Terminalfenster laufen (ein Befehl statt zwei, Nutzerentscheid vom 261029, Absatz „Geändert am 261029“ im Narrativ). Die Frage ist, über welchen Weg KRK den Befehl hinüberreicht. Sie muss vor dem Bau beantwortet sein, weil sie bestimmt, ob KRK eine neue Berechtigung braucht, welche Terminal-Anwendungen unterstützt sind und ob der Kern einen Skripttext erzeugt.

## Options

1. **Eine kurzlebige ausführbare `.command`-Datei, übergeben über denselben `NSWorkspace`-Weg** — KRK schreibt unter `std::env::temp_dir()` (auf macOS das nutzereigene `$TMPDIR` unter `/var/folders/…/T/`, Rechte `0700`) eine Datei `krk-terminal-<prozess>-<zähler>.command` mit Rechten `0700`, angelegt mit `create_new`, Inhalt `#!/bin/sh`, `rm -f -- "$0"`, `cd -- '<ordner>' || exit 1`, `exec vim -- '<datei>'` bzw. `exec sudo -e -- '<datei>'`. Jeder Pfad steht in einfachen Anführungszeichen, ein `'` darin als `'\''`. Ein Pfad ohne gültiges UTF-8 wird abgewiesen, wie es `ordner_oeffnen` tut. Die Datei wird an die eingestellte Anwendung übergeben; das Skript löscht sich als erste Handlung selbst.
   - Pros: keine neue Berechtigung, kein neuer objc2-Name (dieselbe Hülle wie `ctrl+o`), die Bündelkennung aus `settings.toml` gilt unverändert. Der Skripttext entsteht als reine Funktion im Kern und ist ohne AppKit prüfbar, bis hin zu einem Probelauf mit einem Ersatz-`vim` unter `/bin/sh`. Aufgeräumt wird ohne Buchführung in KRK.
   - Cons: ob eine Terminal-Anwendung eine `.command`-Datei **ausführt**, entscheidet sie selbst, und KRK kann es nicht feststellen. Geprüft ist es für Terminal.app: ihre `Info.plist` führt `com.apple.terminal.shell-script` und `public.unix-executable` mit der Rolle `Shell` (gelesen am 261029 unter `/System/Applications/Utilities/Terminal.app`). Für iTerm2 und Ghostty ist es ungeprüft, keine von beiden ist auf diesem Gerät installiert (`speculation:` iTerm2 übernimmt Shell-Skripte, Ghostty ist offen). Führt die Anwendung das Skript nicht aus, bleibt eine 0700-Datei im privaten `$TMPDIR` liegen, bis macOS sie dort abräumt.
2. **AppleScript `tell application "Terminal" to do script "…"`** über `NSAppleScript` oder `osascript`.
   - Pros: kein Dateischreiben, das Fenster bekommt den Befehl unmittelbar.
   - Cons: braucht die Automation-Berechtigung (TCC-Abfrage beim ersten Mal, im Bündel ein `NSAppleEventsUsageDescription` in allen drei `InfoPlist.strings`). Gebunden an Terminal.app, die eingestellte Bündelkennung verliert für diesen Befehl ihre Bedeutung. `osascript` wäre ein Unterprozess, den die Regel „allein Programme des Systems“ zwar zuließe, aber ein neuer Prozessstart im Betriebscode; `NSAppleScript` wäre ein neuer objc2-Name mit Untergrenzen-Eintrag. Der Befehlstext müsste zweimal gequotet werden, einmal für AppleScript und einmal für die Shell.

## Constraints

- KRK hält, speichert und übergibt kein Passwort. `sudo` fragt selbst im Terminal. Kein `sudo -A`, kein Weg über `crates/krk-ui/src/anmeldehilfe.rs`, keine dritte Aufrufstelle von `Passwort::preisgeben`.
- Kein neues `unsafe`. Jeder neue objc2-Name bekommt seinen Eintrag im Untergrenzen-Abschnitt.
- Ein Pfad ohne gültiges UTF-8 wird abgewiesen und nie mit `to_string_lossy` geglättet.
- Die eingestellte Terminal-Anwendung (`settings.toml`, `terminal`) bleibt der eine Ort, an dem der Nutzer sie wählt.

## Recommendation

Möglichkeit 1. Sie braucht keine Berechtigung und keinen neuen Namen aus AppKit, sie lässt die Wahl der Terminal-Anwendung beim Nutzer, und ihr einziger offener Punkt, ob eine andere Anwendung als Terminal.app das Skript ausführt, ist mit Möglichkeit 2 nicht kleiner, sondern ganz verloren. Für die ausgelieferte Einstellung `com.apple.Terminal` ist sie an der `Info.plist` belegt. Ob eine andere Anwendung mitspielt, sagt die Nutzerabnahme, und die Anleitung schreibt es hin. Der Plan des Pakets ist auf diese Antwort gebaut. Fällt die Wahl auf Möglichkeit 2, gilt er für die Schritte 1, 5 und 6 nicht mehr und wird neu geschrieben.

---
Answered: Nutzer im Chat, 261029, Antwort „1, Aa Ba Cb“ zur Planfreigabe — Möglichkeit 1: kurzlebige ausführbare .command-Datei an die eingestellte Terminal-Anwendung; ruled by user, Kai Stalmann <ks@qantr.com>
