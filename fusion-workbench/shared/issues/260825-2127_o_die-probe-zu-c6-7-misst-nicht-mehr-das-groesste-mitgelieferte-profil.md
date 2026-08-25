# Die Probe zu C6.7 misst nicht mehr das größte mitgelieferte Profil

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/tests/leseprofil.rs:2870-2895` (Überschrift und Kopf), `:2896-2940` (`die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`), `:2234-2242` (`ausgelieferte`); `shared/issues/260825-2107_*_der-l7-entscheid-nennt-fuer-das-groesste-mitgelieferte-profil-fuenf-leselaeufe-gemessen-sind-vier.md`; `shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md`; `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Risikotabelle, letzte Zeile); `resources/default-readers.toml` (Profil „fusion-Werkbank: der gemeinsame Speicher")

---

## Was ist

`die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` ist die **einzige**
Probe, die überhaupt an der Auslieferungsfassung misst: `ausgelieferte()` hat genau einen
Rufer (`leseprofil.rs:2898`). Sie misst das Profil einer einzelnen Runde (4 Leseläufe, 11
Öffnungen) und das der Werkbankwurzel (3 und 5), beide auf die Zahl genau.

**Nach Leseläufen ist keines von beiden mehr das größte.** Der Bericht der Runde 18 nennt für
das neue Profil `fusion-Werkbank: der gemeinsame Speicher` **zehn** Leseläufe, und derselbe
Befund steht schon in `shared/issues/260825-2107_*_der-l7-entscheid-…`: „das größte
mitgelieferte Profil ist nach Leseläufen nicht mehr das der einzelnen Runde". Jener Datensatz
zieht daraus die Berichtigung eines Satzes in einem Entscheidungsdatensatz; **die Probe, die
denselben Satz in ihrer Überschrift führt, ist dabei nicht angefasst worden.**

Damit steht der Baum so: das Profil, das mit 10 von 12 Leseläufen am nächsten an seiner
Schranke liegt, ist das einzige, dessen Kosten keine Probe hält.

## Warum das zählt

Die Risikotabelle des Plans nennt genau diese Lage und beschreibt sie als bekannt:

> `shared/` kostet zehn von zwölf Leseläufen; ein elfter Speicher sprengt den Deckel. — Der
> Abstand ist gemessen und in Schritt 10 ausgeschrieben. Zwei Läufe Luft sind wenig.

Was die Minderung nicht sagt: gemessen ist er **einmal**, von Hand, in einem Bericht. Der
Bericht wird nicht wieder gefahren. Wer die Werkbank um Speicher erweitert oder dem Profil
Zeilen auf neue Unterordner hinzufügt, überschreitet die Zwölf ohne jede Rückmeldung, und die
Folge ist nicht ein Fehler, sondern ein `--` an den Zeilen, die nicht mehr drankamen — also
dieselbe Anzeige wie „dort steht nichts".

Der Kopf der Probe schreibt selbst aus, warum die Zahl genau und nicht als Schranke dasteht:
„eine Probe, die allein `<= 7` prüft, bliebe grün, wenn ein Profil von vier auf sieben
Leseläufe steigt, und genau der Schritt wäre die Nachricht." Für das Profil, bei dem dieser
Schritt heute bevorsteht, gibt es die Nachricht nicht.

## Was zu tun wäre

Die Probe um einen dritten Fall erweitern, nach derselben Bauform: ein Prüfordner in der
Gestalt eines gemeinsamen Speichers, `zusammenfassen_gezaehlt` gegen die eingebettete
Fassung, die Beschriftungsliste als Ausweis dafür, dass das richtige Profil gegriffen hat,
und die Zahl der Leseläufe auf die Zahl genau. Dazu die Überschrift und den Kopf nachziehen:
es sind dann drei Profile, und welches „das größte" ist, hängt daran, ob nach Leseläufen oder
nach Öffnungen gefragt wird.

Ein Prüfordner mit zehn leeren Unterordnern kostet nichts und misst genau die Größe, um die
es geht.

**Schwere:** mittel. Kein Fehler heute, aber die eine Zusage der Runde, deren Abstand zur
Schranke am kleinsten ist, und die einzige, die durch nichts gehalten wird.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`.
