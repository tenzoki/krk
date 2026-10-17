# Eine sftp-Verbindung öffnet den entfernten Ordner im Dateifenster, und Kopieren und Verschieben gehen wie lokal

## Directive

Der Nutzer, wörtlich: „das terminal hilft mir nicht. ich wollte das so implementiert haben, dass wir zwischen den dateilisten über das protokoll kopieren etc können, also so als wäre das das file:// protocol.“ Anlass war ein Verbindungs-Lesezeichen `sftp://stalmann.org@ssh.strato.de:22`, das macOS mit „kein Programm zum Öffnen der URL festgelegt“ abwies. Aus der Machbarkeitsanalyse `261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md` hat der Nutzer Möglichkeit 2 gewählt („2“): KRK spricht SFTP selbst über das `ssh` des Systems (`/usr/bin/ssh -s … sftp`), ohne Drittsoftware und ohne neue Kiste, die C mitbringt. Dazu hat er zwei Fragen beantwortet: „Passwort an ssh und sshfs ? ja“ und „endgültig löschen? ja“.

Erreicht ist das, wenn ein Klick auf ein sftp-Verbindungs-Lesezeichen den entfernten Ordner im aktiven Dateifenster zeigt, der Nutzer darin navigieren kann und Kopieren und Verschieben zwischen einem lokalen und einem entfernten Dateifenster in beide Richtungen mit Fortschritt, Abbruch und Konfliktfrage funktionieren wie zwischen zwei lokalen Ordnern; Umbenennen, neuer Ordner und Löschen (nach Rückfrage endgültig, weil der Server keinen Papierkorb hat) gehen im entfernten Fenster ebenso. Was dort noch nicht geht (Git, Inhaltsfilter, Zip), ist ausgegraut und nicht still wirkungslos. Für lokale Ordner bleibt alles, wie es ist.

Zur Ausführung, wörtlich: „doch, du solltest das heute nacht im automode durchführen“.

## Stand nach der Nacht vom 261016 auf 261017

Gebaut sind alle neun Schritte des Plans `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md` (`d407cd14..1e114402`), dazu vier Behebungsrunden nach vier Durchsichten (`545c8d31`, `a97e4fbb`, `0284ace2`, `f4072616`); alle 21 Befunde sind geschlossen, `make check`, Bündelbau und `cargo tree` je Mac-Ziel liefen grün. Das Paket bleibt bewusst offen (`claimed`), weil zwei Haltebedingungen des Plans Nutzerarbeit sind und nicht vorliegen: der Test gegen `ssh.strato.de` am Bündel und der Abnahmelauf L1 bis L10 oder ein ausdrücklicher Verzicht darauf. Bis dahin enthält kein Release diese Arbeit. Zwei Festlegungen hat der Orchestrator unter dem selbständigen Modus getroffen, die der Nutzer bestätigen soll: ein unentschiedener Papierkorbbefund meldet und löscht nicht; „Zur Zwischenablage springen“ öffnet in einem fernen Tab den lokalen Ordner. Ferne Tabs werden beim Neustart nicht wiederhergestellt (Plan).
