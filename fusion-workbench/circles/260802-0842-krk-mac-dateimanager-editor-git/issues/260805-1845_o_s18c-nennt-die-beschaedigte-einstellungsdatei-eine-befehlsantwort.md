S18c nennt die beschädigte Einstellungsdatei eine Befehlsantwort und zugleich eine Startmeldung

---

Der Absatz "Drei Fehler, ein Rang, keine eigene Ansicht" in Schritt 18c des
Plans sagt über denselben Fehlerfall zwei Dinge, die einander ausschließen. Erst
heißt es, eine beschädigte `settings.toml` melde sich "über denselben Weg, den
`ablage::melden` seit S10 für die drei vorhandenen Dateien geht"; drei Sätze
später heißt es, "alle drei sind **Befehlsantworten** und damit Rang 1 der fünf
Ränge aus S16b und S16c: der Nutzer hat sie mit einem Tastendruck unmittelbar
angefordert".

---

Der Widerspruch ist nicht sprachlich, sondern zeitlich. Die beschädigte
Einstellungsdatei fällt beim **Start** an, in `sitzung_laden`, lange bevor ein
Tastendruck geschehen ist. Es gibt in diesem Augenblick keinen Befehl, dessen
Antwort sie sein könnte. Der Weg der drei vorhandenen Dateien ist die Liste der
Startmeldungen am Ende von `oberflaeche_aufbauen`, und die geht über
`meldung_zeigen` in die **Fenstermeldung**, den dritten der fünf Ränge.

Umgesetzt ist am 260805-1845 die erste Hälfte des Absatzes: derselbe Weg wie bei
`keymap.toml` und `session.toml`, also Rang 3. Die zweite Hälfte ließe sich nur
einlösen, indem KRK die Meldung bis zum ersten `ctrl+o` aufhebt und sie dann als
Befehlsantwort setzt — eine Sonderregel mit eigenem Zustand, genau das, was
derselbe Absatz für die Meldung des Wirkungsbereichs ausschließt. Die beiden
übrigen Fehler aus C11, die unbekannte Bündelkennung und der nicht mehr
erreichbare Ordner, sind unbestritten Befehlsantworten und stehen auf Rang 1.

Im laufenden Bündel geprüft: eine syntaktisch kaputte `settings.toml` bringt den
Satz "… ist beschaedigt und wird durch den Auslieferungszustand ersetzt: TOML
parse error …" in die Statuszeile beider Dateifenster, unmittelbar nach dem
Aufbau der Oberfläche und ohne Zutun des Nutzers. Er verschwindet wie jede
Fenstermeldung mit dem nächsten Ordnerwechsel.

Zu berichtigen ist der Plansatz, nicht der Code: die Zuordnung "Rang 1" gilt für
zwei der drei Fehler, der dritte ist eine Startmeldung auf Rang 3.
