# Planner: Umsetzungsplan zur Profil-Zusammenfassung im Vorschaufenster

**Date:** 2026-08-24
**Agent:** planner (als Unteragent des Orchestrators)
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Baumstand:** `278a008`

---

## Was der Lauf hervorgebracht hat

**Der Plan** liegt unter
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`.
Er traegt dreizehn Schritte in fuenf Buendeln, drei Mermaid-Diagramme (Modulschichtung,
Ablauf einer Zusammenfassung, Abhaengigkeitsgraph der Schritte), die Pflichtzeile
`**Decidability:**` und den Abschnitt `## Where this Circle stops` mit neun Bedingungen.

Elf Schritte gehoeren dem `coder`, einer dem `ontocoder` (die Auslieferungsfassung
`resources/default-readers.toml`), einer dem `analyst` (die Berichtigung eines
Entscheidungsdatensatzes und der Schluss eines Defekts im Werkbankspeicher).

## Die sieben Fragen aus `## Open for Planner`, beantwortet

1. Ein **siebter Wert** `Inhalt::Zusammenfassung` statt einer Nutzlast an
   `Inhalt::Metadaten`. Nach A6 ueberlebt von den sechs Metadatenzeilen genau die
   Kopfzeile; eine Nutzlastform truege in jedem erkannten Fall fuenf Angaben mit, die
   niemand anzeigt.
2. Die Kiste ist **`regex` 1.x**, in `krk-core`. Der Grund ist die Zeile
   `**Decidability:**`: C2.8 verlangt eine Laufzeitzusage, die vom Muster unabhaengig ist,
   und endliche Automaten geben sie, waehrend eine rueckverfolgende Auswertung sie nur
   ueber eine Schrittgrenze annaehert.
3. Die Datei traegt `[[profil]]` mit `name`, `pfad`, `kennzeichen` und `[[profil.zeile]]`
   mit `beschriftung` und genau einem Bausteintisch, als unmarkierte Auswahl ueber
   `#[serde(untagged)]` und `#[serde(flatten)]`. Die Vorlage steht im Baum und ist
   abgenommen: `ablage::lesezeichen::Ziel` in `bookmarks.toml`.
4. Auswertung und Profile liegen in **`krk-core`**, Modul `leseprofil/`; die Ablagehaelfte
   in `ablage/leseprofile.rs` nach der Vorlage `ablage/einstellungen.rs`. Der Grund ist
   C6.8: Proben ohne Fenster, und `krk-ui` hat kein Bibliotheksziel.
5. Der Weg an die Flaeche ist `Zusammenfassung::als_text` und `text_zeigen`. C4.6 faellt
   daraus heraus, weil `text_zeigen` den Quellbezug zuruecknimmt und `auswahl_ablegen`
   damit an die Oberklasse durchreicht.
6. Eine Datei, die zwei Bausteine lesen, wird **zweimal** geoeffnet. Der erkannte Ordner
   dagegen wird hoechstens einmal gelesen, weil die Erkennung ihn ohnehin braucht.
7. Die drei Zustandszeilen stehen als drei Vorhandensein-Bausteine auf `^_a_circle\.md$`,
   `^_t_circle\.md$` und `^_[cb]_circle\.md$`.

## Zwei Datensaetze, die der Lauf gefiltert hat

**Ein Defekt:** `issues/260824-0634_o_c6-1-sagt-der-feldbaustein-lese-kein-verzeichnis-seine-form-aus-c3-verlangt-es.md`.
C3 beschreibt den Feldbaustein als „nennt eine Datei ueber einen Ausdruck auf dem
Dateinamen", C6.1 sagt „Der Feldbaustein loest keinen [Verzeichnisleselauf] aus". Ein
Namensmuster laesst sich nur gegen aufgezaehlte Namen halten. Der Plan haelt den Satz fuer
alle fuenf mitgelieferten Profile ein, weil ihre Feldbausteine im erkannten Ordner liegen,
der ohnehin gelesen wird; allgemein gilt er nicht.

**Eine Entscheidung:** `decisions/260824-0634_o_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md`.
A7 nennt drei Zustandszeilen, das Markervokabular kennt sechs Zustaende. Eine ueberholte
(`_s_`) oder zurueckgestellte (`_d_`) Runde antwortet auf alle drei mit „nein"; in dieser
Werkbank betrifft das heute `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.
Empfohlen ist eine vierte Zeile, weil sie zwei Zeilen TOML kostet und keine Zeile Rust und
den Bausteinsatz bei vier laesst.

## Was am Baum und am Bestand nachgesehen wurde

- `crates/krk-ui/src/vorschaumodell.rs`: `Inhalt` traegt sechs Werte; der Doc-Kommentar von
  `zeigt_dateitext` nennt einen siebten bereits als den Fall, der den Bau anhalten soll.
- `crates/krk-core/src/text/datei.rs:605-637`: `bis_zur_grenze_lesen` **weist** eine Datei
  ueber der Grenze ab, statt sie anzulesen. C6.6 verlangt das Anlesen, also tritt eine
  dritte Fassung `anlesen` an dieselbe Tuer `sys::ohne_warten_oeffnen`.
- Der groesste Circle-Datensatz dieser Werkbank ist **119.614 Bytes** gross
  (`circles/260804-0933-…/_d_circle.md`), seine Ueberschrift `## Directive` steht bei Byte
  222. Ohne `anlesen` zeigte gerade dieser Circle keine Directive.
- `crates/krk-core/src/verzeichnis/leser.rs:162`: `lesen` kennt keinen Deckel; A5 braucht
  einen, und `lesen` wird zu `lesen_hoechstens(pfad, usize::MAX)`.
- `crates/krk-core/tests/baum.rs::nur_benannte_dateien_erreichen_das_atomare_schreiben`
  fuehrt fuenf Dateien auf und wird von `ablage/leseprofile.rs` rot; `tests/ablage.rs:250`
  fuehrt sechs Ablagenamen aus und wird von der siebten rot. Beide sind im Plan als
  Schrittarbeit benannt.
- `crates/krk-core/src/ablage/lesezeichen.rs:86-122`: `#[serde(flatten)]` ueber einer
  unmarkierten Auswahl steht im Baum und ist ueber eine Rundreise abgenommen. Der
  Vorbehalt und sein Ausweg stehen dort ausgeschrieben; der Plan nennt beide.
- Bestand am 260824-0634: 18 Circle-Verzeichnisse, 82 Datensaetze in `shared/issues`
  davon 54 offen, 118 in `shared/history`. Diese Zahlen gehen **nicht** in eine Probe, weil
  sie sich mit jeder Sitzung aendern; sie werden bei der Abnahme einmal nachgezaehlt.

## Zahlen des Haushalts, gerechnet und nicht geschaetzt

| Mitgeliefertes Profil | Verzeichnisleselaeufe | Dateioeffnungen | Zusage |
|---|---|---|---|
| Ein einzelner Circle | 5 | 11 | C6.7: hoechstens 7 und hoechstens 11 |
| Wurzel der Werkbank | 3 | 5 | C6.4: hoechstens 12 und hoechstens 24 |

## Was der Plan nicht getan hat

Kein Executor wurde losgeschickt. Der Nutzer sieht den Plan am Tor, bevor gebaut wird.
Kein Marker eines der acht `_a_`-Datensaetze wurde bewegt; die Wanderung auf `_i_` gehoert
dem Rundenabschluss.
