# Nutzertest gegen ssh.strato.de

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md

Der Nutzer hat am 261017 am Bündel (Stand `1c6b29c0`) gegen die Verbindung `sftp://stalmann.org@ssh.strato.de:22` getestet und berichtet, wörtlich: „funktioniert.“ und auf Nachfrage „lesenzeichen, klick, copy, move, delete“.

| Prüfung aus „Where this work stops“ | Ergebnis |
|---|---|
| Lesezeichen anlegen (`cmd+d` in der Geheimnistabelle) | bestanden |
| Anmeldung mit Passwort aus `secrets.txt`, Anzeige des Startordners (Klick) | bestanden |
| Kopieren | bestanden (Richtung nicht einzeln genannt) |
| Verschieben | bestanden (Richtung nicht einzeln genannt) |
| Endgültiges Löschen samt Rückfrage | bestanden |
| Blatt zum Rechnerschlüssel | nicht berichtet |
| Abbruch mitten in einer großen Datei | nicht berichtet |
| Konfliktfrage | nicht berichtet |
| Umbenennen, neuer Ordner | nicht berichtet |
| Endgültiges Löschen an einer eingehängten Freigabe ohne Papierkorb | nicht berichtet |

Kein Defekt gemeldet. Damit ist belegt, dass `ssh.strato.de` das Teilsystem `sftp` anbietet und die Grundwege tragen; die nicht berichteten Prüfungen bleiben offen.
