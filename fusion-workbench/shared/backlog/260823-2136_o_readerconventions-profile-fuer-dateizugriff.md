# ReaderConventions: Profile vereinfachen den Zugriff auf Ordner und Dateien

**Filed by:** k1

Der Nutzer hinterlegt in einer Definitionsdatei `krk-rc.yaml` Profile, die den Zugriff auf
Ordner und Dateien über vordefinierte Muster vereinfachen: welche Orte welche
Leseoperationen erfordern und was im Vorschaufenster erscheint. Beispielfall ist die
fusion-workbench. Sie legt Ordner und Dateien nach fusion-eigenen Konventionen ab, das
Lesen und gelegentliche Bearbeiten ist häufig nötig, und Überblick wie Zugriff sind heute
umständlich; ein Profil beantwortet das, indem es je Ort eine Zusammenfassung definiert.
Skizziert hat der Nutzer: auf `./fusion-workbench` Projektname, Datum des letzten Setups,
Plugin-Version, aktiver Circle, Sitzungsinfo, Zahl der Circles und Zahl der offenen Defekte
im gemeinsamen Speicher; auf `shared/analyses/` die Zahl der Analysen und die zehn jüngsten
Titel, ebenso für `backlog/`, `consult/`, `history/` und die übrigen Speicher; auf `issues/`
die Zahl der offenen und der geschlossenen Defekte; auf `circles/` die Zahl der Circles; auf
einem einzelnen Circle Name, Zustand, die aus `*_circle.md` gezogene Directive, ob Spec und
Plan vorliegen, die Zahl der Entscheidungsdatensätze und die letzten zehn Verlaufstitel. Die
Definitionsdatei liegt unter `~/Library/Application Support/KRK/`, also im Bestandsort von
KRK neben Lesezeichen, gesicherter Sitzung, abweichender Tastenbelegung und Notizzetteln.
Der Nutzer nennt dies als nächsten Circle.
