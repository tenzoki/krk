# Gilt die Duldung einer zurückgezogenen Kennung in der eigenen `keymap.toml` für jede künftig zurückgezogene, und rückwirkend für `endgueltig_loeschen`?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md, 261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md, 260817-0536_*_was-geschieht-mit-einer-gespeicherten-keymap-die-die-entfallene-funktion-fuehrt.md, 260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md

---

## Question

Mit „Termine: Sortierrichtung umkehren“ zieht KRK zum zweiten Mal eine Funktionskennung zurück (`termine_richtung_umkehren`). `Belegung::bauen` weist heute jede Kennung ab, die der Wortschatz der Auslieferung nicht führt, und `belegung::laden` setzt dann die ganze Auslieferungsbelegung ein. Beim ersten Mal, `endgueltig_loeschen` am 260817, hat der Nutzer genau das gewählt und dazugesagt, eine spätere Runde dürfe die Frage neu stellen.

Für die zweite Kennung ist die Frage beantwortet: das Kriterium C3.10 der abgenommenen Spezifikation verlangt, dass eine eigene `keymap.toml` mit der entfallenen Kennung weiter lädt und die übrigen Zuweisungen gelten. Der Plan baut dafür in Schritt 3 eine Liste geduldeter zurückgezogener Kennungen in `krk-core/src/tasten/belegung.rs`, die beim Laden einer Nutzerdatei übergangen werden, mit `termine_richtung_umkehren` als einzigem Eintrag.

Offen ist, wie weit die Liste reicht. Die Antwort bindet jede spätere Arbeit, die eine Funktion zurückzieht, und sie entscheidet, ob zwei zurückgezogene Kennungen auf Dauer verschieden behandelt werden. Schritt 3 hängt nicht an ihr: der eine Eintrag ist allen drei Möglichkeiten gemeinsam.

## Options

1. **Die Liste gilt allein für diese Kennung.** Jede spätere Arbeit, die eine Funktion zurückzieht, stellt die Frage wieder.
   - Pros: nichts ist für die Zukunft festgelegt.
   - Cons: die Frage wiederholt sich bei jedem Rückzug, und die Antwort „die eigene Belegung bleibt“ ist dann jedes Mal erst nach einer Rückfrage sicher.
2. **Die Liste ist die Regel für jede künftig zurückgezogene Kennung; `endgueltig_loeschen` bleibt abgewiesen.** Wer eine Funktion zurückzieht, trägt ihre Kennung in die Liste ein. Die Antwort vom 260817 steht.
   - Pros: ein Rückzug kostet keinen Nutzer mehr seine Belegung; ein Tippfehler in einer von Hand gepflegten Datei fällt weiter auf, weil nur benannte Kennungen übergangen werden. Die Probe `eine_keymap_mit_der_zurueckgezogenen_kennung_wird_als_ganzes_verworfen` bleibt, wie sie ist.
   - Cons: zwei zurückgezogene Kennungen verhalten sich verschieden, und der Grund ist allein das Datum ihres Rückzugs.
3. **Die Liste ist die Regel, und `endgueltig_loeschen` kommt rückwirkend hinein.** Die Antwort vom 260817 wird damit überholt.
   - Pros: eine Regel ohne Ausnahme.
   - Cons: kehrt eine umgesetzte Nutzerentscheidung um; der Nutzen ist klein, weil eine Belegung mit dieser Kennung seit dem 260817 bei jedem Start schon als Ganzes verworfen wird und ihr Besitzer seither mit der Auslieferung oder einer neu gesicherten Belegung arbeitet.

## Constraints

- Das Laden scheitert nie und liefert immer eine benutzbare Belegung; die Datei auf der Platte bleibt in jedem Fall stehen.
- Es gibt genau eine Quelle der ausgelieferten Belegung, `resources/default-keymap.toml`. Die Liste ist keine zweite Belegung: sie führt Namen, die KRK nicht mehr kennt, und keine Tasten.
- Eine unbekannte Kennung, die nicht in der Liste steht, wird weiter abgewiesen, damit ein Tippfehler auffällt.
- C3.10 gilt in jedem Fall.

## Recommendation

Möglichkeit 2. Die Spezifikation hat für den zweiten Fall anders entschieden als der Nutzer am 260817 für den ersten, und die Begründung von damals („einmalig“) trägt mit dem zweiten Rückzug nicht mehr. Eine Regel für die Zukunft spart die Rückfrage bei jedem weiteren Rückzug. Die rückwirkende Aufnahme empfehlen wir nicht: sie änderte ein Verhalten, das eine eigene Probe als Nutzerantwort festhält, ohne dass heute jemand davon etwas hätte.
