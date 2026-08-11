# Abnahmeanleitung: Die Tastenbelegung als Markdown

**Status:** offen, wartet auf den Abnahmelauf des Nutzers
**Grundlage:** `planning/260811-0753_o_spec-tastenbelegung-als-markdown-in-downloads.md`, C1 bis C4
**Zweck:** Die 41 Abnahmekriterien des Specs sind als Behauptungen über den Endzustand
formuliert. Diese Datei macht Schritte daraus: Handlung, Beobachtungsort, Bestehensbedingung.

Die Reihenfolge ist nach **Werkzeug** geordnet und nicht nach Fähigkeit, damit du `make menue`
einmal fährst und nicht siebenmal.

---

## Block 1 — Ohne KRK zu starten: `make menue`

```sh
make menue | grep -i tastenbelegung
```

| Nr | Bestehensbedingung |
|---|---|
| 1.1 | Es erscheint genau eine Zeile mit `eintrag="Tastenbelegung als Markdown sichern"` |
| 1.2 | Auf derselben Zeile steht `kombination=(keines)` |
| 1.3 | Auf derselben Zeile steht `menue="KRK"` |

Zur Stellung im Menü:

```sh
make menue | grep 'menue="KRK"'
```

Das gibt alle Einträge des Menüs KRK aus, in der Reihenfolge, in der sie im Menü stehen.

| Nr | Bestehensbedingung |
|---|---|
| 1.4 | Die Zeile mit `eintrag="Tastenbelegung als Markdown sichern"` steht **oberhalb** der Zeile mit `eintrag="KRK beenden"` |

Deckt ab: C1 „Eintrag im Hauptmenü", C1 „kein Tastenkürzel".

---

## Block 2 — Ohne KRK zu starten: die Verbotsseite

```sh
grep -c '^\[\[funktion\]\]' resources/default-keymap.toml
grep -c '=>' crates/krk-core/src/tasten/belegung.rs   # nur zur Sicht
git log --oneline 5e369bb..HEAD -- resources/default-keymap.toml crates/krk-ui/src/fenstertitel.rs
```

| Nr | Bestehensbedingung |
|---|---|
| 2.1 | `default-keymap.toml` führt **71** Funktionsblöcke |
| 2.2 | Der `git log` ist **leer** — weder Belegung noch Fenstertitel sind in dieser Runde angefasst |

Deckt ab: C1 „keine neue Belegungszeile", C1 „`Kommando` wächst nicht", Zeitzusagen
„keine der zehn Zahlen geändert".

---

## Block 3 — Die Datei erzeugen

```sh
make bundle && open target/KRK.app
```

Dann im Menü **KRK** den Eintrag „Tastenbelegung als Markdown sichern" wählen.

| Nr | Beobachtungsort | Bestehensbedingung |
|---|---|---|
| 3.1 | Statuszeile | Es erscheint „Tastenbelegung geschrieben: ~/Downloads/KRK-Tastenbelegung.md" |
| 3.2 | Statuszeile | Der Pfad steht **mit Tilde** da, nicht ausgeschrieben |
| 3.3 | Dateisystem | `~/Downloads/KRK-Tastenbelegung.md` existiert |
| 3.4 | Oberfläche | Direkt danach bewegt sich die Auswahl mit den Pfeiltasten, ein Tabwechsel geschieht. Die Anwendung hat **nicht sichtbar angehalten** |
| 3.5 | System | Erscheint eine Rückfrage nach Zugriff auf „Downloads"? **Notiere die Antwort** — beide Ausgänge sind zulässig. Erscheint sie nicht, hat KRK die Zustimmung schon vom Anzeigen des Ordners |

Deckt ab: C1 „Klick löst aus", C4 „Erfolgsmeldung mit vollem Pfad", C2 „TCC-Rückfrage geprüft",
Zeitzusagen „hält die Oberfläche nicht an".

---

## Block 4 — Die erzeugte Datei lesen

```sh
open -e ~/Downloads/KRK-Tastenbelegung.md      # oder in KRKs eigenem Editor
```

| Nr | Bestehensbedingung |
|---|---|
| 4.1 | Die Datei beginnt mit **genau einer** Überschrift. Kein Zeitstempel, keine Versionsangabe, kein erklärender Vorspann |
| 4.2 | Darunter folgen **neun** Abschnitte, einer je Funktionsbereich, in derselben Reihenfolge wie in der Belegungsansicht |
| 4.3 | Kein Abschnitt trägt eine **leere** Tabelle |
| 4.4 | Jede Zeile hat **drei** Spalten: Funktion, Kombinationen, wo der Befehl wirkt |
| 4.5 | Eine Funktion mit mehreren Kombinationen steht in **einer** Zeile, nicht in mehreren |
| 4.6 | Die Kombinationen stehen als `Shift+Cmd+K` und `F3` — Großschreibung, Pluszeichen, keine Symbole |
| 4.7 | In der dritten Spalte stehen **ausgeschriebene** Angaben. Nirgends `Tabbereich`, nirgends `Navigator`, nirgends `Ueberall`. Keine Legende am Fuß |
| 4.8 | „Dateifenster und Vorschau" kommt vor (das ist `Tabbereich`), ebenso „Dateifenster, Leiste und Vorschau" (das ist `Navigator`) und „überall" |
| 4.9 | Jede aufgeführte Funktion trägt mindestens eine Kombination. Keine Zeile mit leerer Kombinationsspalte |

### Die sechs Textbefehle — hier liegt der Kern dieser Runde

Suche die Zeilen für Ausschneiden, Kopieren, Einfügen, Alles auswählen, Rückgängig, Wiederholen.

| Nr | Funktion | Dritte Spalte muss lauten |
|---|---|---|
| 4.10 | Ausschneiden | „Textfelder und Editor" |
| 4.11 | Kopieren | „Textfelder und Editor" |
| 4.12 | Einfügen | „Textfelder und Editor" |
| 4.13 | **Alles auswählen** | **leer** — das ist Absicht, kein Versäumnis |
| 4.14 | Rückgängig | „Editor" |
| 4.15 | Wiederholen | „Editor" |

Eine einheitliche Beschriftung über alle sechs ist ein **Fehlschlag**. 4.13 ist der Punkt, an dem
die Messung aus S1 die ursprüngliche Annahme widerlegt hat.

### Form der Datei

```sh
file ~/Downloads/KRK-Tastenbelegung.md
head -c 3 ~/Downloads/KRK-Tastenbelegung.md | xxd | head -1
grep -c $'\r' ~/Downloads/KRK-Tastenbelegung.md
```

| Nr | Bestehensbedingung |
|---|---|
| 4.16 | `file` meldet UTF-8 |
| 4.17 | Die ersten drei Bytes sind **nicht** `efbb bf` (keine Bytefolgenmarke) |
| 4.18 | Das `grep` zählt **0** — keine Wagenrückläufe, Zeilenende ist `\n` |
| 4.19 | Die Datei lässt sich in einem gewöhnlichen Markdown-Betrachter lesen, und die Tabellen stehen als Tabellen |

Deckt ab: den größten Teil von C3.

---

## Block 5 — Zweiter Aufruf, Überschreiben

| Nr | Handlung | Bestehensbedingung |
|---|---|---|
| 5.1 | Den Menüeintrag ein zweites Mal wählen | Es liegt weiterhin **genau eine** Datei dieses Namens im Ordner. Kein `KRK-Tastenbelegung 2.md`, kein Zeitstempel im Namen |
| 5.2 | Vorher `echo fremd > ~/Downloads/KRK-Tastenbelegung.md`, dann auslösen | Die fremde Datei wird **ohne Rückfrage** überschrieben |
| 5.3 | Statuszeile bei 5.1 und 5.2 | **Dieselbe** Meldung wie beim ersten Mal. Kein Zusatz wie „ersetzt" |

Deckt ab: C2 „zweiter Aufruf überschreibt", C2 „auch eine fremde Datei", C4 „eine Meldung für
beide Fälle".

---

## Block 6 — Bei offener Belegungsansicht

Das ist der Fall, den der Spec ausdrücklich zu prüfen und nicht anzunehmen verlangt.

| Nr | Handlung | Bestehensbedingung |
|---|---|---|
| 6.1 | Belegungsansicht öffnen, dann Menü **KRK** öffnen | **Ist der Eintrag auswählbar oder grau?** Notiere die Antwort |
| 6.2 | Falls grau | Dann sind 6.3 bis 6.5 **gegenstandslos**. Der Abweichungsfall ist nicht erreichbar, und das ist ein gültiges Ergebnis |
| 6.3 | Falls auswählbar: in der Ansicht eine Kombination ändern, **nicht sichern**, dann den Eintrag wählen | Die Datei trägt den **alten**, gesicherten Stand. Deine ungesicherte Änderung steht **nicht** darin |
| 6.4 | Danach die Ansicht über das Sichern verlassen | Die Datei von 6.3 hat sich **nicht** nachträglich geändert |
| 6.5 | Während das Blatt steht | **Ist die Statuszeile sichtbar, oder verdeckt das Blatt sie?** Ist sie verdeckt, bist du nach einem Aufruf aus dieser Lage ohne jede Rückmeldung — das ist zu **berichten**, nicht hinzunehmen |
| 6.6 | Statuszeile bei 6.3 | Es erscheint die **gewöhnliche** Erfolgsmeldung. **Kein** Zusatz darüber, dass der gesicherte Stand geschrieben wurde |

Deckt ab: C1 „auswählbar bei stehender Belegungsansicht", C3 „gesicherter Stand", C4
„Sichtbarkeit der Meldung", C4 „keine zusätzliche Meldung".

---

## Block 7 — Ohne Fokus im Dateifenster

| Nr | Handlung | Bestehensbedingung |
|---|---|---|
| 7.1 | Den Fokus in die Lesezeichenleiste setzen, dann den Eintrag wählen | Die Datei entsteht. Die Ausgabe hängt an keinem Bereich und an keinem Fokus |

Deckt ab: C1 „auswählbar ohne Fokus in einem Dateifenster".

---

## Block 8 — Die Fehlerfälle

| Nr | Handlung | Bestehensbedingung |
|---|---|---|
| 8.1 | `mv ~/Downloads ~/Downloads-weg`, dann auslösen, danach zurückbenennen | Keine Datei entsteht, KRK legt den Ordner **nicht** an, und die Statuszeile nennt den **fehlenden Ordner** als Grund |
| 8.2 | `chmod 500 ~/Downloads`, dann auslösen, danach `chmod 700` | Keine Datei, keine halb geschriebene Datei, und die Statuszeile nennt den **abgelehnten Zugriff** — erkennbar **anders** als bei 8.1 |
| 8.3 | Falls bei 3.5 eine Systemrückfrage erschien: sie ablehnen | Verhält sich wie 8.2. Kein Absturz, keine stumme Rückkehr |

Deckt ab: C2 „fehlender Ordner", C2 „abgelehnter Zugriff", C2 „abgelehnte Rückfrage", C4
„gescheiterter Aufruf trennt die Gründe".

---

## Was diese Anleitung nicht führt

**Vier Kriterien sind von Proben im Baum gehalten und brauchen keinen Handgriff.** Sie stehen
hier nur, damit du sie nicht suchst: die Auflösung über `pfade::benutzerverzeichnis()`, das
Fehlen verdrahteter Zahlen, die Herkunft der Schreibweise aus `anzeige()`, und der Nachweis,
woher die Zelle jedes der sechs Textbefehle kommt. `make check` hält sie; läuft es grün, sind sie
erfüllt.

**Ein Kriterium ist ohne Vorbereitung nicht prüfbar:** die fünfte Zelle „(von KRK nicht
eingeordnet)" entsteht nur mit einer von Hand geschriebenen `keymap.toml`, die einer Funktion
**mit** Kommando ein `gehalten_von = "menue"` gibt. Wenn du es prüfen willst, sag Bescheid — der
Fall ist von einer Probe gehalten, und ein Handlauf dafür wäre ein eigener Schritt.

---

## Ergebnis eintragen

Diese Datei hat keine Kästchen. Abgehakt wird im Spec, `### C1` bis `### C4` — dort stehen die
41 Kriterien mit `- [ ]`. Diese Anleitung sagt nur, wie du zu jedem von ihnen kommst.
