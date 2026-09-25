Ein Lesezeichen, das bei markierter Datei angelegt wird, merkt nur den Ordner, und der Sprung markiert nichts

---

Der Nutzer legt im Dateifenster ein Lesezeichen an, während eine Datei markiert ist, und erwartet, dass die Auswahl des Lesezeichens später in den Ordner springt **und die Datei markiert**. KRK springt nur in den Ordner und markiert keine Zeile.

---

**Filed by:** user, Kai Stalmann <kai@stalmann.org>

**Befund am Code (260925)**

- `Ziel` (`crates/krk-core/src/ablage/lesezeichen.rs`) kennt zwei Sorten, `Ordner { ordner }` und `Textstelle { datei, zeile, zeileninhalt }`. Eine Sorte „Datei im Dateifenster“ gibt es nicht.
- `Anwendungsdelegierter::anlegeziel` (`crates/krk-ui/src/appkit/anwendung.rs`) legt außerhalb des Editors immer `Ziel::Ordner` auf den **angezeigten Ordner** an; die markierte Zeile wird nicht gelesen.
- `Anwendungsdelegierter::leistenauswahl_ausfuehren` ruft für `Ziel::Ordner` `ordner_lesen(ordner, None)`. Der zweite Parameter von `DateifensterQuelle::ordner_lesen` (`crates/krk-ui/src/appkit/tabelle.rs`) nimmt bereits einen Namen zur Auswahl an, und `Tabliste::auswahl_auf_namen` merkt ihn während des Lesens vor; der Sprung kann also markieren, bekommt aber keinen Namen.

Das Verhalten ist damit die gebaute Regel aus C5 und kein Absturz oder Lesefehler: dem Lesezeichen fehlt die Angabe, welche Datei gemeint war.

**Was vor der Behebung zu entscheiden ist**

Wo die Datei im Lesezeichen steht: als dritte Sorte von `Ziel` (eine Datei, deren Ordner geöffnet und die darin markiert wird) oder als wahlfreies Feld an `Ziel::Ordner`. Beides ändert die Form von `bookmarks.toml` und muss bestehende Dateien weiter lesen (dreizehntes Abnahmekriterium von C6). Dazu kommt, welche Zeile gilt, wenn mehrere markiert sind, und was geschieht, wenn die Datei fehlt, der Ordner aber noch steht.

**Abnahme**

Ein Lesezeichen, angelegt bei markierter Datei, öffnet bei seiner Auswahl den Ordner der Datei im aktiven Dateifenster und setzt die Auswahl auf diese Datei; eine bestehende `bookmarks.toml` wird unverändert gelesen, und ein Lesezeichen ohne markierte Datei verhält sich wie bisher.

---
Resolved: `Ziel::Ordner` trägt das wahlfreie Feld `auswahl` (Entscheid 260925-1901_*_merkt-sich-ein-lesezeichen-die-markierte-datei-als-feld-oder-als-eigene-sorte.md); `anlegeziel` merkt den Namen der ausgewählten Zeile über `DateifensterQuelle::auswahl_name`, `leistenauswahl_ausfuehren` reicht ihn an `ordner_lesen` weiter. Gehalten von der Probe `eine_ordnermarke_mit_gemerktem_eintrag_uebersteht_die_rundreise` (`crates/krk-core/tests/ablage.rs`).
