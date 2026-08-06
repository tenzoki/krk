# Wie kommt KRK für den Abnahmelauf in den Vordergrund?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `issues/260806-1235_o_der-sitzungslauf-der-abnahmestrecke-bricht-bei-l5-tab-ab-und-gibt-keine-zahl-mehr-aus.md`, `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S21, S22), Spec C8

---

## Question

Die Sitzungsstrecke misst nur dann, wenn KRK während des Laufs die **vorderste
Anwendung** ist. Ist sie es nicht, hat ihr Fenster keinen Tastaturfokus,
`Anwendungsdelegierter::kommando_ausfuehren` weist jeden Befehl mit einem
Wirkungsbereich ab, und die synthetischen Tastendrücke lösen nichts mehr aus.
Gemessen am 260806-1250: der Lauf kam bis L5-Tab, weil `auswahl_runter` als
einziger Befehl der Strecke `Wirkungsbereich::Ueberall` trägt und L1 und L7
damit trotzdem messen; bei `tab_naechster` (`Tabbereich`) blieb er zehn
Sekunden stehen und brach ohne Zahl ab.

Ob KRK nach vorn kommt, entscheidet nicht KRK. macOS 15 lässt eine Anwendung,
die aus einem Prozess im Hintergrund gestartet wurde, nicht nach vorn: weder
`NSApplication::activate()` noch `activateIgnoringOtherApps(true)` änderten in
der Messung etwas, `isActive()` blieb `false`. Aus einem Terminalfenster im
Vordergrund gestartet, lief derselbe Bau vollständig durch und lieferte alle
sieben Reihen mit je zwanzig Werten.

Damit hängt die Abnahme an einer Bedingung, die weder der Plan noch die
Messvorschrift aus C8 nennt, und die der Bediener bisher nur zufällig erfüllt
hat. Die Frage ist, wie die Strecke sie künftig sicherstellt.

Als Sofortmaßnahme bricht die Strecke seit dem 260806 mit
`messmodus::NICHT_IM_VORDERGRUND` ab, sobald die erste Messung im Hintergrund
beginnen soll: eine ehrliche Meldung statt eines Wartefehlers vierzig
Messungen später. Das behebt die falsche Diagnose, nicht die Bedingung.

## Options

1. **Nur der Abbruch, und die Messvorschrift nennt die Bedingung.** Der Lauf
   verweigert die Messung im Hintergrund; der Plan (S21/S22) und die Anleitung
   schreiben dazu, dass der Abnahmelauf aus einer Anwendung im Vordergrund zu
   starten ist und dass während des Laufs nicht in einer anderen Anwendung
   gearbeitet werden darf.
   - Pro: keine Zeile mehr Code als heute, keine zweite Betriebsart, und die
     Bedingung steht dort, wo die übrigen Messbedingungen stehen.
   - Contra: eine Zusage an den Bediener, die keine Maschine hält. Wer den Lauf
     über einen Auftragsplaner oder aus einer Sitzung ohne Vordergrund startet,
     bekommt zuverlässig keine Zahl.
2. **Der Lauf startet das Bündel über `open`.** LaunchServices aktiviert die
   Anwendung, KRK kommt damit auch aus einem Hintergrundprozess nach vorn.
   - Pro: die Bedingung hält die Maschine, nicht der Bediener.
   - Contra: ein über `open` gestartetes Bündel hat **keine Standardausgabe**
     (gemessen am 260803-1309, Defekt
     `issues/260803-1309_*_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`).
     Die Messzeilen bräuchten einen zweiten Weg, etwa eine Datei, deren Pfad
     der Lauf vorgibt. Das ist ein zweiter Ausgabeweg neben dem, den beide
     Strecken heute teilen.
3. **Der Lauf wartet auf den Bediener.** Statt abzubrechen wartet die Strecke,
   bis KRK vorn steht, und schreibt dazwischen eine Aufforderung.
   - Pro: kein zweiter Ausgabeweg, und der Lauf gelingt auch beim ersten
     Anlauf, wenn der Bediener anwesend ist.
   - Contra: ein Lauf, der auf einen Menschen wartet, ist nicht mehr
     unbeaufsichtigt fahrbar, und die Wartezeit steht mitten in einer Reihe,
     die C8 als eine Messung beschreibt.

## Constraints

- Die zehn Zeitzusagen aus C8 und ihre Messvorschriften bleiben unangetastet.
- Was gemessen wird, muss die Anwendung sein, die der Nutzer bedient: KRK für
  den Lauf so zu ändern, dass es auch im Hintergrund auf Befehle mit
  Wirkungsbereich hört, scheidet aus. Der Fokusvorbehalt aus C5 ist eine
  zugesagte Eigenschaft und keine Messhürde.
- Die Strecke darf im Zweifel keine Zahl ausgeben. Dieselbe Haltung wie bei
  `OHNE_BILDSCHIRM` und bei `--kalt` ohne Rechte.

## Recommendation

Möglichkeit 1 als Stand, Möglichkeit 2 als nächster Schritt, falls die Abnahme
je unbeaufsichtigt laufen soll. Der Abbruch ist bereits gebaut und macht die
Bedingung sichtbar; ob sich der zweite Ausgabeweg lohnt, hängt daran, ob die
Messreihe künftig von Hand oder von einem Auftragsplaner gefahren wird, und
das entscheidet der Nutzer.

---
Answered:
Implemented:
Deferred:
Superseded by:
