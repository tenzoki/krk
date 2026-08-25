Die Leselaufregel der Datei zählt den Erkennungslauf nicht mit und liegt bei zwei Profilen um eins daneben

---

`resources/default-readers.toml:218` gibt dem Nutzer die Regel, mit der er die Kosten eines
Profils gegen die Schranke von zwölf Leseläufen rechnet: „die Zahl der Leseläufe eines Profils
ist die Zahl der VERSCHIEDENEN Orte darin". Die Regel unterschlägt den Erkennungslauf. Ein
Profil, das über eine Kennzeichendatei erkennt und in dessen Zeilen der erkannte Ordner
selbst nicht vorkommt, kostet einen Leselauf mehr, als die Regel nennt. Zwei der zwölf
ausgelieferten Profile haben genau diese Gestalt.

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:215-218` und `:449-452`;
`crates/krk-core/src/leseprofil/mod.rs` (`HOECHSTENS_LESELAEUFE`, Doc-Kommentar: „Der
Erkennungslauf zaehlt mit: er ist ein Leselauf dieser Zusammenfassung und keiner daneben");
`crates/krk-core/src/leseprofil/erkennung.rs` (Modulkopf, „Warum die Eintraege als Abschluss
hereinkommen"); `shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md`
(Tabelle „Die vier Zahlen je Profil", Zeilen 8 und 12)

## Was der Mechanismus sagt

Der erste Erkennungsdurchgang prüft allein Pfadmuster und kostet keinen Systemaufruf. Der
zweite braucht die Namen der Einträge im ausgewählten Ordner und kostet damit einen
Verzeichnisleselauf. Dieser Lauf geht durch dieselbe Merkstelle wie jeder andere Ort: nennt
eine Zeile den erkannten Ordner ebenfalls, fällt er nur einmal an. Nennt ihn keine, steht er
trotzdem in der Rechnung — und die Regel der Datei sieht ihn nicht, weil sie allein die
Ortsangaben der Zeilen zählt.

Für ein Profil mit Pfadmuster stellt sich die Frage nicht: dort kostet die Erkennung nichts.

## Was gemessen ist

Gemessen am 260825-2126 über `leseprofil::zusammenfassen_gezaehlt`, Baum `8478753`:

| Profil | Orte in den Zeilen | Regel der Datei | gemessen |
|---|---|---|---|
| Projektwurzel mit fusion-Werkbank | `fusion-workbench`, `…/circles`, `…/shared/issues` = 3 | 3 | **4** |
| Projektwurzel mit flight-Werkbank | `flight-workbench` und dessen vier Speicher = 5 | 5 | **6** |
| fusion-Werkbank: die Wurzel | erkannter Ordner, `circles`, `shared/issues` = 3 | 3 | 3 |
| flight-Werkbank: die Wurzel | erkannter Ordner und vier Speicher = 5 | 5 | 5 |

Die zwei unteren Profile treffen, weil ihre Feldzeilen ohne Ortsangabe stehen und damit den
erkannten Ordner selbst nennen. Die zwei oberen treffen nicht: dort trägt **jede** Zeile eine
Ortsangabe, und der erkannte Ordner wird allein für die Erkennung gelesen.

Dieselben Zahlen stehen unabhängig erhoben in `shared/analyses/260825-2107-…`, Zeilen 8 und 12
der Tabelle: 4 und 6.

## Warum das zählt

Die Regel steht nicht als Beschreibung da, sondern als Rechenanleitung. Der Kommentar über dem
Profil des gemeinsamen Speichers schickt den Nutzer ausdrücklich damit los
(`:451-452`: „Von den zwölf erlaubten Läufen bleiben damit zwei übrig; wer einen elften
Unterspeicher aufnimmt, rechnet nach, was er kostet"). Wer nach der Regel rechnet, hat bei
einem über eine Kennzeichendatei erkannten Profil einen Lauf zu wenig im Budget. An den zwei
ausgelieferten Fällen ist der Abstand zur Schranke groß genug, dass nichts passiert; die
Anleitung ist trotzdem falsch.

## Was zu tun wäre

Die Regel um ihren zweiten Halbsatz ergänzen, etwa: „…die Zahl der verschiedenen Orte darin,
und dazu ein Lauf für die Erkennung, wenn das Profil über eine Kennzeichendatei erkennt und
keine seiner Zeilen den erkannten Ordner selbst nennt." Der Doc-Kommentar von
`HOECHSTENS_LESELAEUFE` sagt die Hälfte davon schon.

**Schwere:** mittel. Kein Bau und keine Probe hängt daran; die Anleitung, mit der ein Nutzer
sein eigenes Profil gegen die Schranke hält, ist um eins zu klein.
