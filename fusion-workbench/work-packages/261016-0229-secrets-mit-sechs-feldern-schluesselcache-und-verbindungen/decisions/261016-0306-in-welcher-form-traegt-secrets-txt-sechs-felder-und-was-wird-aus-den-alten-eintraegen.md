# In welcher Form trägt der Klartext von `secrets.txt` sechs Felder, und was wird aus den Einträgen im bisherigen Format?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0229-secrets-mit-sechs-feldern-schluesselcache-und-verbindungen.md, 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md, 260926-0007_*_in-welchem-format-stehen-notes-txt-und-tasks-txt.md, 260926-2253_*_spec-termine-als-weitere-datei-im-heimordner.md

---

## Question

Der Klartext von `secrets.txt` hat heute die Form von `notes.txt`: je Eintrag eine Themenzeile `## <Thema>` und Text darunter, zwei Spalten in der Tabelle. Der Auftrag verlangt sechs Spalten, Name, Protokoll, Adresse, Konto, Passwort und Notizen. Das Format bindet über diesen Plan hinaus: `README.md` beschreibt es so, dass sich die Datei ohne KRK lesen lässt, und die Rohansicht zeigt es dem Nutzer zum Bearbeiten von Hand. Zu entscheiden ist die Form und der Übergang für vorhandene Einträge, ohne dass ein Geheimnis verloren geht.

## Options

1. **Ein Geheimnis ist eine Notiz, deren Text mit bis zu vier Feldzeilen beginnt.** `## <Name>`, darunter höchstens je einmal `protocol:`, `address:`, `account:`, `password:` (Schlüssel ohne Rücksicht auf Groß- und Kleinschreibung gelesen, klein geschrieben), alles danach sind die Notizen. Ein Eintrag im alten Format ist damit ohne Umbau ein Geheimnis mit leeren Feldern: Thema wird Name, Text wird Notizen. Gelesen wird jede Form, geschrieben wird allein der Eintrag, den eine Handlung berührt, und der in der Grundform mit allen vier Feldzeilen.
   - Pros: byte-treu wie `notes.txt` und `appointments.md`; dieselbe Zerlegung im Kern (`Notizen::lesen`), dieselbe Regel „eine Handlung schreibt nur, was sie berührt“; kein Umbaulauf, also keine Stelle, an der ein Umbau scheitern und einen Stand halb schreiben könnte; die Datei bleibt in jedem Textprogramm pflegbar, und `README.md` bleibt die Anleitung zum Lesen ohne KRK.
   - Cons: eine Notizzeile eines alten Eintrags, die zufällig mit `password: ` beginnt, wird als Feld gelesen. Verloren geht dabei nichts, sie wandert nur in die Spalte, die ihr Wortlaut nennt.
2. **Ein neues Format (TOML-Tabellen je Eintrag) mit einem Umbaulauf beim ersten Öffnen.**
   - Pros: eindeutige Felder, keine Leseregel für Feldzeilen.
   - Cons: ein Umbaulauf schreibt ohne Handlung des Nutzers die ganze Datei neu; die Form weicht von den drei anderen Eintragsdateien ab, und die Rohansicht zeigt eine zweite Syntax.
3. **Eine Formatversion im Kopf des Chiffrats, die das Klartextformat ankündigt.**
   - Pros: ein Leser weiß vor dem Zerlegen, welche Form folgt.
   - Cons: verändert das gemessene Dateiformat (`FORMATVERSION`) für eine Frage, die der Klartext selbst beantworten kann; ältere KRK lehnten die Datei dann als beschädigt ab.

## Constraints

- Kein vorhandener Eintrag darf beim Übergang an Inhalt verlieren; das alte Format bleibt lesbar.
- Der Kopf des Chiffrats und die Ableitung bleiben unverändert (`README.md`, `## Das Dateiformat von secrets.txt`).
- Name, Protokoll, Adresse, Konto und Passwort sind je eine Zeile; die Notizen dürfen Absätze tragen, aber keine Zeile, die mit `## ` beginnt.

## Recommendation

Wir empfehlen Möglichkeit 1, nach dem Muster der Termine („Ein Termin ist eine Notiz, deren Thema ein Datum ist“, Modulkopf von `heimordner/eintraege.rs`). Die Leseregel lautet genau: nach der Themenzeile wird eine ununterbrochene Folge von Zeilen als Feldzeilen gelesen, solange jede mit einem der vier Schlüssel, einem Doppelpunkt und danach nichts oder einem Leerzeichen beginnt **und** dieser Schlüssel in der Folge noch nicht stand; die erste andere Zeile beendet die Folge. Die Grundform schreibt alle vier Feldzeilen, leer als `protocol:` ohne Leerzeichen dahinter. Weil eine geschriebene Grundform alle vier Schlüssel trägt, endet die Folge danach in jedem Fall, und eine Notizzeile, die wie eine Feldzeile aussieht, bleibt Notiz. Ein alter Eintrag bekommt seine Grundform beim ersten Ändern einer seiner Zellen und nicht vorher; die Schlüssel sind englisch, damit die Datei in jeder der drei Sprachen der Oberfläche dieselbe bleibt.
