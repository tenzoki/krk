# Welche Leser dürfen eine Platzhalterdatei eines Cloudlaufwerks herunterladen?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1435-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `261025-1444-plan-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `261025-1444-was-heisst-abbrechen-fuer-einen-download-den-die-vorschau-schon-angestossen-hat.md`, `261025-1109-warum-die-navigation-auf-google-drive-zuweilen-langsam-ist.md`, `261025-1109-bis-zur-grenze-lesen-oeffnet-die-datei-vor-der-groessenpruefung-und-holt-auf-einem-cloudlaufwerk-jede-zu-grosse-datei-herunter.md`

---

## Question

Auf einem File-Provider-Laufwerk lädt jedes Öffnen einer Datei mit `SF_DATALESS` sie herunter (`inference:` dokumentiertes Verhalten, für Google Drive nicht nachgemessen). Der Nutzer hat für die Vorschau der Auswahl entschieden: sie lädt, verzögert, einzeln, bis zu einer Grenze. Offen ist, was für die anderen Leser gilt, die heute Dateien öffnen, ohne dass der Nutzer diese Datei verlangt hat: das Aufnahmedatum der Bildfolge (`bild::aufnahmedatum` aus `Bildverzeichnis::gruppe_ordnen`), der Inhaltsfilter mit „Content“ (`verzeichnis::inhalt::traegt_der_inhalt`, auch über „Deep“), und die Profil-Zusammenfassung eines ausgewählten Ordners (`anlesen` in `leseprofil/bausteine.rs`). Die Antwort bindet jeden künftigen Leser, nicht nur diesen Plan, und sie entscheidet, welche Fallunterscheidung die zwei Leseschleusen in `krk-core/src/text/datei.rs` tragen.

## Options

1. **Allein die Vorschau der ausgewählten Datei lädt herunter; kein Hintergrundleser lädt je eine Platzhalterdatei.** Drei Klassen, nach dem, der liest: (a) die ausgewählte Datei in der Vorschau, einschließlich des Fotos an der Stelle einer Bildfolge, lädt nach der Regel des Plans; (b) jede Datei, die ein Leser liest, ohne dass der Nutzer sie gewählt hat (Aufnahmedatum, Inhaltsfilter, Profilwerte), wird nicht geöffnet und gilt als ungelesen; (c) jeder Weg, den der Nutzer ausdrücklich auf eine Datei richtet (Editor, Kopieren, Zippen, Entpacken, Teilen, Öffnen mit), bleibt unverändert und lädt wie jedes Programm.
   - Pros: die drei Klassen sind getrennt und vollständig; die Kosten eines Cursorschritts sind auf höchstens einen Download begrenzt, gleich welches Profil greift; die zwei Leseschleusen bekommen eine Platzhalterregel als Argument, und der Übersetzer zwingt jeden Rufer, sie zu wählen.
   - Cons: die Bildfolge ordnet Fotos, die nur in der Cloud liegen, nach dem Änderungsdatum statt nach dem Aufnahmedatum; der Inhaltsfilter durchsucht sie nicht und nennt ihre Zahl in der Statuszeile; ein Profilwert aus einer Platzhalterdatei fehlt.
2. **Wie 1, aber die Profil-Zusammenfassung lädt ihre Dateien weiter herunter.** Begründung wäre, dass sie Teil der Vorschau eines ausgewählten Ordners ist.
   - Pros: Profilwerte bleiben vollständig.
   - Cons: ein Cursorschritt auf einen Ordner kann bis zu `HOECHSTENS_OEFFNUNGEN` Downloads auslösen, gleichzeitig mit dem einen der Vorschau, und die Regel „höchstens ein Download zur Zeit“ gälte nicht mehr.
3. **Jeder Leser darf laden, die Vorschau nur verzögert.** Allein die Vorschau bekommt Wartezeit, Ladeplatz und Grenze.
   - Pros: kleinster Umbau.
   - Cons: ein Cursorschritt auf `Fotos/2024` holt bis zu 7 500 Fotos (Befund B3 der Analyse, `Fotos/2024` allein 11,3 GB), und ein Filterlauf mit „Content“ und „Deep“ über `Meine Ablage` bis zu 184 GB.

## Constraints

- Die Wünsche des Nutzers vom 261025 im Narrativ des Arbeitspakets binden wörtlich: die Vorschau lädt herunter.
- Die Zusage L7 (100 ms im Perzentil für die Vorschau einer lokalen Datei) darf durch die Regel nicht schlechter werden.
- Kein Leser darf einen Platzhalter als Datei ohne Treffer oder als leer ausgeben; ungelesen heißt ungelesen.

## Recommendation

Möglichkeit 1. Sie ist die einzige der drei, unter der die Zusage „höchstens ein Download zur Zeit“ für einen Cursorschritt überhaupt gilt, und die Folge für die Bildfolge ist die Regel, die `gruppe_ordnen` schon für ein Foto ohne lesbares Datum hat (C5.5 des Spec der Bildfolge). Der Plan baut nach ihr; wählt der Nutzer Möglichkeit 2, ändert sich allein die Platzhalterregel an der einen Stelle in `leseprofil/bausteine.rs`.

---
Answered: 261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md `## Recommendation` — Möglichkeit 1; ruled by user, Kai Stalmann <kai@stalmann.org>, am 261025 mit der Freigabe des Plans.
