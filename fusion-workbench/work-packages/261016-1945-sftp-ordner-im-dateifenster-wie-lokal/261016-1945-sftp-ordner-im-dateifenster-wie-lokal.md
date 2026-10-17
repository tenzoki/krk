# Eine sftp-Verbindung öffnet den entfernten Ordner im Dateifenster, und Kopieren und Verschieben gehen wie lokal

## Directive

Der Nutzer, wörtlich: „das terminal hilft mir nicht. ich wollte das so implementiert haben, dass wir zwischen den dateilisten über das protokoll kopieren etc können, also so als wäre das das file:// protocol.“ Anlass war ein Verbindungs-Lesezeichen `sftp://stalmann.org@ssh.strato.de:22`, das macOS mit „kein Programm zum Öffnen der URL festgelegt“ abwies. Aus der Machbarkeitsanalyse `261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md` hat der Nutzer Möglichkeit 2 gewählt („2“): KRK spricht SFTP selbst über das `ssh` des Systems (`/usr/bin/ssh -s … sftp`), ohne Drittsoftware und ohne neue Kiste, die C mitbringt. Dazu hat er zwei Fragen beantwortet: „Passwort an ssh und sshfs ? ja“ und „endgültig löschen? ja“.

Erreicht ist das, wenn ein Klick auf ein sftp-Verbindungs-Lesezeichen den entfernten Ordner im aktiven Dateifenster zeigt, der Nutzer darin navigieren kann und Kopieren und Verschieben zwischen einem lokalen und einem entfernten Dateifenster in beide Richtungen mit Fortschritt, Abbruch und Konfliktfrage funktionieren wie zwischen zwei lokalen Ordnern; Umbenennen, neuer Ordner und Löschen (nach Rückfrage endgültig, weil der Server keinen Papierkorb hat) gehen im entfernten Fenster ebenso. Was dort noch nicht geht (Git, Inhaltsfilter, Zip), ist ausgegraut und nicht still wirkungslos. Für lokale Ordner bleibt alles, wie es ist.

Zur Ausführung, wörtlich: „doch, du solltest das heute nacht im automode durchführen“.

## Stand nach der Nacht vom 261016 auf 261017

Gebaut sind alle neun Schritte des Plans `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md` (`d407cd14..1e114402`), dazu vier Behebungsrunden nach vier Durchsichten (`545c8d31`, `a97e4fbb`, `0284ace2`, `f4072616`); alle 21 Befunde sind geschlossen, `make check`, Bündelbau und `cargo tree` je Mac-Ziel liefen grün. Das Paket bleibt bewusst offen (`claimed`), weil zwei Haltebedingungen des Plans Nutzerarbeit sind und nicht vorliegen: der Test gegen `ssh.strato.de` am Bündel und der Abnahmelauf L1 bis L10 oder ein ausdrücklicher Verzicht darauf. Bis dahin enthält kein Release diese Arbeit. Zwei Festlegungen hat der Orchestrator unter dem selbständigen Modus getroffen, die der Nutzer bestätigen soll: ein unentschiedener Papierkorbbefund meldet und löscht nicht; „Zur Zwischenablage springen“ öffnet in einem fernen Tab den lokalen Ordner. Ferne Tabs werden beim Neustart nicht wiederhergestellt (Plan).

## Schließungsnotiz

Geschlossen am 261017 als `done`, Commits `d407cd14..6415a776`. Der Nutzertest gegen `ssh.strato.de` ist bestanden für Lesezeichen, Klick, Kopieren, Verschieben und Löschen (`261017-0952-nutzertest-gegen-ssh-strato-de.md`); nicht berichtet sind Rechnerschlüsselblatt, Abbruch, Konfliktfrage, Umbenennen und Löschen an einer Freigabe ohne Papierkorb. Auf den Abnahmelauf L1 bis L10 hat der Nutzer verzichtet (`261017-0955-schuldet-das-sftp-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md`). Vier Durchsichten, alle 21 Befunde geschlossen; die letzte (`261017-0145-reviewer-schlussdurchsicht-tausch-a97e4fbb-bis-0284ace2.md`) urteilte revise, ihre drei Befunde behebt `f4072616` ohne erneute Durchsicht. Die Abschlussmeldung bindet keinen Beleg, weil das Narrativ nach dem Beleg um den Nachtstand ergänzt wurde. Unbestätigt sind zwei Festlegungen des selbständigen Modus (unentschiedener Papierkorbbefund löscht nicht; „Zur Zwischenablage springen“ im fernen Tab lokal).

Die Haltebedingungen des Plans wurden unter dem selbständigen Modus nicht vorgelegt und stehen hier wörtlich:

- Die Schritte 1 bis 9 sind abgeschlossen, und für jeden ist seine Akzeptanz am Baum erfüllt, Grundabnahme eingeschlossen.
- `das_passwort_wird_allein_an_netfs_und_ssh_preisgegeben` ist grün und nennt als Rufer von `Passwort::preisgeben` allein `einhaengen` in `verbinden.rs` und `passwort_reichen` in `anmeldehilfe.rs`.
- **Nutzerarbeit: der Test gegen `ssh.strato.de` ist am Bündel gefahren, und sein Ergebnis steht als Datensatz im Paket.** Geprüft werden dabei die Anmeldung über die Verbindung `sftp://stalmann.org@ssh.strato.de:22` mit Passwort aus `secrets.txt` und das Blatt zum Rechnerschlüssel, falls der Schlüssel unbekannt ist. Dazu kommen Anzeige und Navigation des Startordners, Kopieren und Verschieben je in beide Richtungen mit Fortschritt, ein Abbruch mitten in einer großen Datei und die Konfliktfrage, schließlich Umbenennen, neuer Ordner und endgültiges Löschen samt Rückfrage. Ja oder nein: der Datensatz liegt vor, und jede gescheiterte Prüfung hat einen Defekt im Paket.
- Bietet `ssh.strato.de` das Teilsystem `sftp` nicht an oder weicht `RENAME` dort von E10 ab, ist ein Defekt im Paket abgelegt, und das Paket schließt nicht, bis er behoben oder vom Nutzer zurückgestellt ist.
- **Nutzerarbeit: der Abnahmelauf L1 bis L10 aus C8 ist am Bündel im Vordergrund gefahren, und alle zehn halten**, oder der Nutzer hat in einem Entscheid dieses Pakets ausdrücklich auf ihn verzichtet (E16).
- Das endgültige Löschen an einer eingehängten Freigabe ohne Papierkorb ist am Bündel geprüft, oder der Testdatensatz vermerkt ausdrücklich, dass keine Freigabe zur Hand war.
- Der Entscheid `261016-1956-welche-form-hat-ein-ort-im-dateifenster-und-wie-haelt-der-bau-fernaufrufe-vom-hauptfaden-fern.md` ist beantwortet, bevor Schritt 2 beginnt.
- Ein Auslieferungslauf, der diese Arbeit enthält, setzt voraus, dass der Test gegen `ssh.strato.de` bestanden ist.
