# Drei Stücke des Editormodells haben keinen Aufrufer, und der Plan nennt keinen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Umsetzung von S35, S36 und S37
**Betroffen:** `crates/krk-ui/src/editormodell.rs` (`Suchlauf::treffer`, `Editormodell::haelt_zurueck`, `Editormodell::suche_beenden`)
**Cross-references:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` Abschnitte `#### 15.`, `#### 31.`, S37

---

## Der Befund

`editormodell.rs` trug seit S16 ein `#![allow(dead_code)]` am Dateikopf, mit der
ausgeschriebenen Ankündigung, die Zeile falle mit S37:

> Der letzte davon ist S37; **dann** ist die Zeile wegzunehmen.

Mit S37 ist sie weggenommen. Von den vierzehn Fundstellen, die sie abdeckte,
haben zehn ihren Aufrufer bekommen. Vier nicht:

| Stück | Aufrufer |
|---|---|
| `Editormodell::stempel` | kommt mit S31, dem Melden einer Änderung von außen |
| `Suchlauf::treffer` | **kein Schritt des Plans nennt einen** |
| `Editormodell::haelt_zurueck` | **kein Schritt des Plans nennt einen** |
| `Editormodell::suche_beenden` | **kein Schritt des Plans nennt einen** |

Alle vier tragen jetzt ein eigenes `#[allow(dead_code)]` mit dem Grund daran,
statt sich hinter einer Zeile am Dateikopf zu verstecken. Für `stempel` ist das
in Ordnung: der Schritt steht im Plan und ist datiert. Für die drei übrigen ist
es eine Ausnahme ohne Ablaufdatum, und genau das führt dieser Eintrag.

## Warum jedes der drei heute ohne Aufrufer ist

- **`Suchlauf::treffer`** gibt die ganze Trefferliste heraus. Die Oberfläche
  kommt mit `zahl`, `nummer`, `angesteuert` und `meldung` aus; die ganze Liste
  bräuchte, wer alle Treffer zugleich zeichnete, und das sagt C5 nicht zu.
- **`Editormodell::haelt_zurueck`** fragt, ob eine gelesene Datei auf die
  Nachfrage aus C4 wartet. Die Oberfläche erfährt das als `Ladeausgang` und
  beantwortet es im Rückruf des Blattes, ohne zwischendurch nachzusehen.
- **`Editormodell::suche_beenden`** beendet den Suchlauf. Der Spec sagt keinen
  Befehl zu, der das tut; die Suche endet von selbst beim Tippen
  (`bearbeiten`), beim Dateiwechsel (`uebernehmen`) und beim Schließen
  (`schliessen`), und jede dieser drei Stellen setzt das Feld unmittelbar.

## Was zu entscheiden ist

Zwei Wege, und die Wahl gehört nicht in einen Schritt, der Suchen und Ersetzen
baut:

1. **Streichen.** Drei Zugriffsfunktionen ohne Aufrufer sind drei Zusagen, die
   niemand einlöst; der Plan verlangt „nur bauen, was gebraucht wird".
2. **Stehen lassen und die Ausnahme führen.** `haelt_zurueck` ist eine
   naheliegende Frage, die ein späterer Zustandsbericht stellen könnte, und
   `suche_beenden` ist die Gegenseite von `suche_starten`.

Bis zur Entscheidung bleiben sie mit ihrer einzelnen Ausnahme stehen. Der
Arbeitsbereich ist grün, und die Proben am Dateiende fassen jedes der drei an —
tot ist keines, unaufgerufen sind alle drei.
