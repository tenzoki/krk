# Welche Kombinationen bekommen die beiden neuen Umschalter, und was wird aus `opt+cmd+e`?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_o_bereichsleiste-und-proportionale-breitenregel.md` (Schritt 4), `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260812-0306_a_bekommen-die-spaltenschalter-tastenbefehle.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_*_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`, `resources/default-keymap.toml`

---

## Question

Die Klärungsrunde hat entschieden, dass die Bereichsschalter ausgelieferte Kombinationen bekommen und dass zwei fehlen: ein Umschalter für den Editor und einer für das linke Dateifenster. **Welche** Kombinationen es sind, hat sie offengelassen, und für den Editor ist die naheliegende schon vergeben.

Die Umschaltfamilie dieser Belegung steht auf `opt+cmd+<Buchstabe>` und nimmt den Anfangsbuchstaben des Bereichs: `opt+cmd+l` für die Lesezeichenleiste, `opt+cmd+d` für das zweite Dateifenster. Der Buchstabe des Editors ist vergeben, und zwar an eine Funktion, die keine Umschaltfunktion ist: `opt+cmd+e` heißt "Editor schließen", gibt die Datei frei und löst die Nachfrage aus C4 der Editor-Runde aus.

Für das linke Dateifenster gibt es keinen freien Buchstaben, der es benennt. Beide Dateifenster heißen "Dateifenster", und das `d` trägt das rechte.

## Options

1. **`opt+cmd+left` für das linke Dateifenster, `opt+cmd+b` für den Editor; `editor_schliessen` bleibt auf `opt+cmd+e`.** Zusätzlich bekommt `zweites_fenster_umschalten` mit `opt+cmd+right` eine zweite Kombination, damit die beiden Dateifenster dasselbe Muster tragen.
   - Pro: Keine ausgelieferte Kombination wechselt den Besitzer. Die Pfeile sind für die waagerechte Geometrie schon belegt, denn `ctrl+left` und `ctrl+right` verschmälern und verbreitern den aktiven Bereich. Die Zuordnung "links ist links" braucht keine Erklärung.
   - Contra: `opt+cmd+b` benennt den Editor nicht. Der Buchstabe steht für "Bearbeiten", die Norton-Bedeutung von F4, und diese Verbindung muss der Nutzer kennen.
   - Kosten im Plan: keine über Schritt 4 hinaus.
2. **`opt+cmd+e` wechselt zum neuen Umschalter; `editor_schliessen` zieht auf `opt+shift+cmd+e`.**
   - Pro: Die Familie bleibt geschlossen: `opt+cmd+<Buchstabe>` heißt durchgehend "Bereich ein- und ausblenden". Der häufigere Handgriff bekommt die kürzere Kombination.
   - Contra: Eine ausgelieferte Kombination einer abgenommenen Runde wechselt ihre Bedeutung. Wer `opt+cmd+e` gewohnt ist, gibt danach die Datei nicht mehr frei, sondern parkt den Editor, und die Nachfrage aus C4 bleibt aus, wo er sie erwartet. Daneben wäre `opt+shift+cmd+e` die erste Kombination dieser Belegung mit drei Zusatztasten.
   - Kosten im Plan: eine Zeile mehr in Schritt 4, dazu die Kommentare an beiden Einträgen.
3. **Der Editor bekommt gar keinen eigenen Umschalter; der Schalter in der Leiste ist der einzige Weg.**
   - Pro: Keine neue Kombination, keine Verwechslungsgefahr zwischen Ausblenden und Schließen.
   - Contra: Es widerspricht der Antwort vom 260812-0306, die für die Bereichsschalter ausgelieferte Kombinationen verlangt, und es bräche C2 der Runde 1, das jede Funktion über mindestens einen Tastenbefehl verlangt.

## Constraints

- `resources/default-keymap.toml` ist nach C3 die eine Quelle jeder Belegung; jede Antwort steht dort und nirgends sonst.
- Zwei Funktionen auf einer Kombination schließt C3 aus; die Konflikterkennung sieht auch die Menükürzel.
- Eine Kennung wird nicht umbenannt: eine `keymap.toml` des Nutzers, die eine unbekannte Kennung nennt, wird als Ganzes abgewiesen.

## Recommendation

**Möglichkeit 1.** Sie ändert nichts, was schon ausgeliefert ist, und die beiden Befehle bleiben unterscheidbar: `opt+cmd+e` gibt die Datei frei und fragt nach, `opt+cmd+b` blendet aus und behält den Stand. Möglichkeit 2 ist die systematisch sauberere und kostet dafür die Verlässlichkeit einer eingeübten Taste; sie ist eine Entscheidung des Nutzers und keine des Planers.


## Antwort 260812-0430

**Die Empfehlung des Plans wird uebernommen:** `opt+cmd+left` fuer das linke Dateifenster,
`opt+cmd+b` fuer den Editor-Umschalter, `editor_schliessen` bleibt auf `opt+cmd+e`, und
`zweites_fenster_umschalten` bekommt `opt+cmd+right` als zweite Kombination neben `opt+cmd+d`.

**Am Baum geprueft, nicht angenommen.** `resources/default-keymap.toml` fuehrt heute unter
`opt+cmd` die Kombinationen `delete`, `l`, `d`, `g`, `c` und `e`. Weder `b` noch `left` noch
`right` sind belegt; die Pfeiltasten kommen in der Datei nur nackt, mit `cmd` und mit `ctrl` vor
(Zeilen 226, 231, 429, 434).

**Zur Familie, gegen die das verstoesst und warum es trotzdem richtig ist.** Der Kommentar bei
Zeile 585 haelt fest, dass die Umschaltfamilie auf `opt+cmd+<Buchstabe>` steht. Fuer die beiden
Dateifenster ist die Richtungstaste der bessere Merkweg als ein Buchstabe: es gibt kein Wort,
das "linkes Dateifenster" so kurz und eindeutig traegt wie der Pfeil nach links, und `d` fuer
"zweites Dateifenster" ist schon heute eher Zaehlung als Bezeichnung. Der Buchstabe bleibt
deshalb fuer das rechte erhalten, und die beiden Pfeile treten daneben; der Kommentar in der
Belegungsdatei ist mit derselben Aenderung um diesen Satz zu erweitern.

`opt+cmd+b` fuer den Editor ist die Umkehrung von `bearbeiten`: derselbe Anfangsbuchstabe, und
`e` ist an das Schliessen vergeben, das die Datei aufgibt.

Umsetzung: Schritt 4 des Plans `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0430` — beantwortet vom Orchestrator, autonom auf Weisung des Nutzers; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented: 90b02d4 — `opt+cmd+left`, `opt+cmd+b`, `opt+cmd+right` als zweite Kombination am rechten Dateifenster; `editor_schliessen` bleibt auf `opt+cmd+e`.
Deferred:
Superseded by:
