# Der L7-Entscheid nennt für das größte mitgelieferte Profil fünf Leseläufe, gemessen sind vier

---
**Domain:** code
**Filed by:** analyst
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_o_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md` (Abschnitt `## Question`, letzter Absatz); `shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md`; `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Schritt 4); `crates/krk-core/src/leseprofil/bausteine.rs` (Modulkopf, Abschnitt „Ein Ort wird je Zusammenfassung hoechstens einmal gelesen")

---

## Was ist

Der offene Entscheidungsdatensatz zu L7 schließt seinen Frageteil mit dem Satz: „Für einen
erkannten Ordner kommen bis zu zwölf Leseläufe und vierundzwanzig Dateiöffnungen dazu; das
größte mitgelieferte Profil kostet gemessene fünf und elf, aber gemessen ist die **Zahl der
Aufrufe** und nicht die Zeit."

Die Zahl fünf stammt aus der Runde 16 und ist mit Schritt 4 der Runde 18 falsch geworden. Seit
ein Ort je Zusammenfassung höchstens einmal gelesen wird, kostet das Profil einer einzelnen
Runde **vier** Leseläufe und nicht fünf; die elf Dateiöffnungen stimmen unverändert. Nachgemessen
am 260825-2107 über `zusammenfassen_gezaehlt` an `circles/260802-0842-krk-mac-dateimanager-editor-git`:
4 Leseläufe, 11 Dateiöffnungen.

Dazu kommt eine zweite Verschiebung, die der Satz nicht mehr trägt: **das größte mitgelieferte
Profil ist nach Leseläufen nicht mehr das der einzelnen Runde.** Es ist das Profil des
gemeinsamen Speichers mit zehn Leseläufen und null Dateiöffnungen. Nach Dateiöffnungen bleibt
das Rundenprofil mit elf das größte. Wer den Satz heute liest, hält vier für fünf und hält ein
Profil für das teuerste, das es in einer der beiden Größen nicht mehr ist.

## Warum das zählt

Der Satz steht in dem Absatz, der ausschreibt, was ungemessen bleibt, und er ist damit Teil der
Grundlage, auf der der Nutzer zwischen den drei Möglichkeiten jenes Datensatzes wählt. Eine
Grundlage mit einer überholten Zahl lädt zu der Annahme ein, das Rundenprofil sei der Fall, an
dem sich die Schranke entscheidet. Der Fall, an dem das gilt, ist heute ein anderer.

**Der Datensatz ist offen und wird gelesen, um beantwortet zu werden.** Deshalb ist die Zahl
nachzuziehen und nicht als Aufzeichnung eines damaligen Standes stehen zu lassen; die Ortsregel
für Aufzeichnungen betrifft die Marker in zitierten Dateinamen und nicht eine Messzahl, die als
gegenwärtig behauptet dasteht.

## Was zu tun wäre

Den einen Satz nachziehen, ohne die Frage anzufassen: „das größte mitgelieferte Profil kostet
nach Leseläufen zehn (der gemeinsame Speicher, ohne eine einzige Dateiöffnung) und nach
Dateiöffnungen elf (eine einzelne Runde, bei vier Leseläufen)". Die Zahlen und ihre Herkunft
stehen in `shared/analyses/260825-2107-…`, Abschnitt „Die vier Zahlen je Profil".

**Schwere:** niedrig. Kein Bau und keine Probe hängt daran.

**Gefunden:** analyst, bei der Kostenmessung zu Schritt 10 des Plans der Runde 18.

**Betroffen:** `fusion-workbench/circles/260823-2208-…/decisions/260824-1900_o_…` (Abschnitt `## Question`, letzter Absatz)
