# Ontocoder: die Auslieferungsfassung der Leseprofile

**Status:** Complete
**Datum:** 260824-1313
**Executor:** ontocoder
**Planschritt:** Schritt 7 des Plans `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel C
**Freigabe:** Der Nutzer hat den Schritt am 260824-1255 ausdrücklich freigegeben.

---

## Was entstanden ist

`resources/default-readers.toml`, 292 Zeilen, davon 182 Kommentar. Die fünf Profile
stehen in der vom Plan vorgeschriebenen Reihenfolge: Wurzel der Werkbank, ein Speicher, ein
Defektspeicher, das Verzeichnis aller Runden, eine einzelne Runde.

Der Kommentarkopf trägt sechs Abschnitte vor dem ersten `[[profil]]`:

1. Wofür die Datei da ist, dass KRK sie beim ersten Start wörtlich anlegt und danach nie
   wieder anfasst, und dass eine Änderung erst beim nächsten Start wirkt.
2. **Was ein Schreibfehler kostet** — der Abschnitt aus dem Befund
   `issues/260824-1242_*`, siehe unten.
3. Der Aufbau: Profil, Zeile, genau ein Baustein; die einfachen Anführungszeichen als
   wörtliche TOML-Zeichenkette; die Regel über `ordner`.
4. **Die vier Bausteine** `zaehlung`, `juengste`, `feld` und `vorhandensein`, jeder mit
   einem kurzen Beispiel (C5.10), dazu der Satz zur Verankerung: ein `feldmuster` läuft über
   den ganzen Dateiinhalt, und wer eine Zeile darin verankern will, schreibt `(?m)`.
5. Die Vorrangregel aus C2 in ihren drei Schritten.
6. Die fünf Zahlen des Haushalts, mit dem Satz, dass eine Zeile über der Grenze ihren
   Platzhalter `--` zeigt und nichts abbricht.

Die deutschen Umlaute stehen als Umlaute; die Datei ist Prosa für den Nutzer und kein
Bezeichner. Formvorbild ist `resources/default-settings.toml`, bis in die Kommentarführung.

## Die berichtigten Ausdrücke sind übernommen

Schritt 7 schreibt zwei Ausdrücke in ihrer am 260824-1224 berichtigten Fassung vor, und die
Datei trägt diese und nicht die alten:

| Baustein | übernommen |
|---|---|
| `.active-circle` | `^([^\n]+)` |
| Directive im Circle-Datensatz | `(?sm)^## Directive\s*\n+(.+?)\n\n` |

Die berichtigten Abnahmekriterien sind C3.8 und C3.9 des Specs.

## Gegen den echten Bestand gehalten, nicht bloß hingeschrieben

Zwei Läufe, beide am 260824-1313, beide gegen die achtzehn Circle-Verzeichnisse, die Speicher
unter `shared/` und die Wurzeldateien dieser Werkbank.

**Erster Lauf: die Ausdrücke einzeln**, in einem Wegwerfprogramm gegen `regex` 1.x außerhalb
des Baumes.

- Das Kennzeichen `^\.fusion-setup$` trifft in der Wurzel genau einen Eintrag.
- Die drei Feldmuster auf `.fusion-setup` liefern `krk`, `2026-08-23T21:17:17+0200` und
  `10.6.0`.
- `^([^\n]+)` auf `.active-circle` liefert den Namen der aktiven Runde.
- `(?s)## Current\n\s*(.+?)\n` auf `orchestrator-live.md` liefert die Sitzungszeile.
- Das Speichermuster trifft 78 Ordner, das Defektspeichermuster 19, `fusion-workbench/circles$`
  den einen. Kein Pfadmuster trifft die Wurzel, und keines trifft ein Rundenverzeichnis: das
  Profil der einzelnen Runde kann im ersten Durchgang nicht überholt werden (C2.3, C5.7).
- Das Kennzeichen `^_._circle\.md$` trifft jedes der achtzehn Rundenverzeichnisse.
- **Das Directive-Muster trifft alle achtzehn Circle-Datensätze.** Die alte Fassung traf
  keinen.

**Zweiter Lauf: die ganze Datei durch `krk-core`**, also `toml::from_str` in
`leseprofil::datei::Profildatei`, dann `leseprofil::datei::pruefen`, dann
`leseprofil::zusammenfassen` gegen zehn echte Ordner. Ergebnis: **fünf Profile, keine
Meldung.** KRK liefert damit keine Auslieferungsfassung mit, die ihre eigene Prüfung nicht
besteht; Schritt 8 hält das später als Probe.

Die sechs Zusammenfassungen aus C5, am Bestand vom 260824-1313:

| Ort | Ergebnis |
|---|---|
| Wurzel | Projekt `krk`, Fassung `10.6.0`, aktive Runde `260823-2208-…`, Sitzungszeile, **18** Runden, **54** offene Defekte |
| `shared/history` | **118** Datensätze, zehn Titel |
| `shared/issues` | **54** offen, 27 geschlossen, zehn Titel |
| `circles` | **18** Runden |
| eine Runde | Zustand dreizeilig (`nein`/`ja`/`nein`), Directive, Spec `ja`, Plan `ja`, 9 Entscheidungen, zehn Verlaufstitel |
| ein Speicher in einer Runde | Zahl und zehn Titel, aus demselben Profil wie `shared/…` |

Die Zahlen 18, 54, 118 sind die, die C5.1, C5.4 und C5.2 nennen. Sie stehen hier und in keiner
Probe: sie ändern sich mit jeder Sitzung.

Der Haushalt, an den zwei größten Profilen gerechnet und aus der Zeilenform abgelesen: die
Runde kostet 5 Leseläufe (erkannter Ordner, `planning` zweimal, `decisions`, `history`) und 11
Öffnungen (Circle-Datensatz, zehn Verläufe), gegen die in C6.7 zugesagten höchstens 7 und
höchstens 11. Die Wurzel kostet 3 und 5.

## Der Befund aus dem Schritt ist eingelöst

`issues/260824-1242_*_die-kommentarzeilen-der-auslieferungsfassung-sagen-nicht-dass-ein-
schreibfehler-die-ganze-datei-kostet.md` ist auf `_c_` umbenannt und trägt seine
`Resolved:`-Notiz. Der Abschnitt „Was ein Schreibfehler kostet" sagt beides, was er gefordert
hat: ein verschriebener Schlüssel in einem Baustein kostet die ganze Datei und nicht die eine
Zeile, und eine Zeile trägt genau einen der vier Bausteine. Beide Sätze stehen **vor** dem
ersten `[[profil]]`, also dort, wo der Nutzer die Datei zu bearbeiten anfängt.

## Zwei Befunde sind neu gefiled

- `decisions/260824-1313_o_deckt-das-speicherprofil-auch-decisions-memos-und-investigations-ab.md`
  — das Speichermuster zählt sechs Namen auf und lässt drei Speicher ohne Profil, darunter
  **jeden `decisions`-Ordner**. Gemessen: 21 Ordner treffen weder das Speicher- noch das
  Defektspeichermuster. Das ist keine Abweichung von der Vorgabe (C5.2 zählt die sechs Namen
  einzeln auf), sondern die Frage, ob die Auslassung gewollt war. Empfohlen ist die
  Aufnahme der drei Namen in dieselbe Aufzählung, zwei Zeilen TOML.
- `issues/260824-1313_o_der-datensatz-zur-vierten-zustandszeile-nennt-ein-verzeichnis-
  ausserhalb-der-drei-zeilen-es-sind-zwei.md` — der offene Datensatz zur vierten Zustandszeile
  und der Plan sagen beide, genau ein Rundenverzeichnis falle aus den drei Zeilen aus A7. Es
  sind zwei, beide auf `_d_`. Die Zahl beziffert den Nutzen der offenen Frage um die Hälfte zu
  klein.

## Prüfung

    make check — exit 0

Alle vier Kommandos grün. Der Baum ist von diesem Schritt nicht berührt: die Datei wird erst
mit Schritt 8 über `include_str!` eingebunden, und bis dahin liest sie niemand.

## Was noch aussteht

Schritt 8 bindet die Datei ein und nimmt sie ab, darunter die Probe zu C5.10 über die vier
Bausteinnamen im `AUSLIEFERUNGSTEXT` und die Probe, dass die eingebettete Fassung sich ohne
Meldung prüfen lässt. Der zweite Lauf oben nimmt ihr das Ergebnis vorweg, ersetzt sie aber
nicht: er lief außerhalb des Baumes und hinterlässt dort nichts.
