# secrets.txt mit sechs Feldern, zehn Minuten gemerkter Schlüssel und Verbindungen als Lesezeichen

## Directive

Der Auftrag des Nutzers, wörtlich:

> und ein weiteres paket:
> - mit einer änderung der home/secrets funktion: statt zwei spalten wie notes und tasks, braucht secrets: name, protocol, address, account, password, notes
> - einer weiteren erweiterung der secrets funktion dahin, dass der einmal eingegebene key für secrets für 10 min. in memory gecachte bleibt.
> - einer neuen rechte-maustaste funktion verbinden, die name, protocol, address, account, password, und notes abfragt in einem dialogfenster, dann ein neues lesezeichen unter einer neu einzurichtenden gruppe 'Verbindungen' anlegt mit dem eingegebenen namen.
> - einer erweiterung der lesezeichen funktion für die Verbindungen: beim klicken auf eine verbindung wird die verbindung hergestellt, die daten werden aus der secrets gelesen, entweder mit abfrage des keys oder des gecachten keys wenn noch valide.

Erreicht ist das, wenn vier Dinge gelten. Erstens zeigt die Geheimnistabelle des Heimordners (`secrets.txt`) sechs Spalten, Name, Protokoll, Adresse, Konto, Passwort und Notizen, statt der zwei, die Notizen und Aufgaben führen. Zweitens fragt KRK nach einmal eingegebenem Schlüssel zehn Minuten lang nicht erneut; der Schlüssel liegt dabei allein im Arbeitsspeicher und nie auf der Platte. Drittens bietet das Kontextmenü der Dateiliste „Verbinden…“ an: ein Dialog fragt die sechs Felder ab, legt den Eintrag in `secrets.txt` an und ein Lesezeichen mit dem eingegebenen Namen in einer neuen Gruppe „Verbindungen“. Viertens stellt ein Klick auf ein solches Lesezeichen die Verbindung her; die Zugangsdaten kommen aus `secrets.txt`, mit Abfrage des Schlüssels oder, solange er noch gilt, mit dem gemerkten.

Zur Ausführung, wörtlich: „nimm anschließend das andere paket auf, führe es im automode durch, committe alles, mache eine volles release und schließe das paket und die session.“
