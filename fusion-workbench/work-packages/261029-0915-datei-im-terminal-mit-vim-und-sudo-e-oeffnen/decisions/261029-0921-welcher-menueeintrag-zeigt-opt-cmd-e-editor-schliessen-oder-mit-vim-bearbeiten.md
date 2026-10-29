# Welcher Menüeintrag zeigt `opt+cmd+e`, wenn „Editor schließen“ und „Im Terminal bearbeiten“ sich die Kombination teilen?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261029-0915-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md, 260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md

---

## Question

Der Nutzer hat am 261029 entschieden, dass `opt+cmd+e` mit dem Fokus in der Dateiliste die Datei unter dem Cursor im Terminal öffnet, je nach Schreibrecht mit `vim` oder mit `sudo -e` (ein Befehl, Absatz „Geändert am 261029“ im Narrativ) und weiter „Editor schließen“ bleibt, wo es heute wirkt. Die Teilung ist nach der bestehenden Regel zulässig: „Editor schließen“ trägt `Wirkungsbereich::Editor` (`Seite::Editor`), der neue Befehl `Wirkungsbereich::Dateifenster` (`Seite::Ausserhalb`), `Wirkungsbereich::schliesst_aus` sagt für das Paar ja, und `im_weg` (`crates/krk-core/src/tasten/belegung.rs`) lässt den zweiten Träger zu. Über die Tastatur wirken danach beide. **Im Hauptmenü zeigt aber nur einer der beiden Einträge das Kürzel**: AppKit nimmt dem später stehenden von zwei gleichen Kürzeln das Zeichen, und `menuemodell::fruehere_behalten_das_kuerzel` schreibt dieselbe Regel im Modell aus. Welcher Eintrag früher steht, folgt aus dem Obermenü, in das der neue Befehl kommt (`bereich_des_kommandos` in `crates/krk-ui/src/belegungsmodell.rs`, Folge `Funktionsbereich::ALLE`: … „Dateioperationen“ vor … „Editor“), und innerhalb eines Obermenüs aus der Reihenfolge in `resources/default-keymap.toml`. Die Frage muss vor dem Bau beantwortet sein, weil sie die Zeile in `bereich_des_kommandos`, die Stelle in der Belegungsdatei und die Menüprobe festlegt.

## Options

1. **Obermenü „Dateioperationen“, neben „Ordner im Terminal öffnen“.** „Im Terminal bearbeiten“ zeigt `opt+cmd+e`, „Editor schließen“ steht ohne Kürzel im Menü „Editor“.
   - Pros: die zwei Terminalbefehle stehen beisammen, wo der Nutzer „Terminal“ sucht. Der neue Befehl zeigt sein Kürzel an der Stelle, an der man ihn kennenlernt. So ist es schon einmal gelaufen: bei `cmd+1` zeigt der neuere Eintrag unter „Home“ das Kürzel und „Nach Name sortieren“ steht ohne da.
   - Cons: „Editor schließen“ zeigt seit der Editor-Runde `opt+cmd+e` und verliert die Anzeige. Die Taste wirkt dort unverändert; wer sie im Menü nachsieht, findet sie nicht mehr, sondern nur in der F1-Ansicht, in `make tasten` und in `HowTo.md`.
2. **Obermenü „Editor“, hinter „Editor schließen“.** „Editor schließen“ zeigt weiter `opt+cmd+e`, der neue Befehl steht ohne Kürzel.
   - Pros: die Anzeige eines ausgelieferten Befehls bleibt, wie sie ist. Einen Befehl mit `Wirkungsbereich::Dateifenster` im Menü „Editor“ gibt es schon: `bearbeiten` (F4) steht dort. „Bearbeiten“ ist auch die Handlung des neuen Befehls.
   - Cons: ausgerechnet der Befehl, dessen Taste der Nutzer neu lernt, zeigt sie im Menü nicht. Die Terminalbefehle stehen in zwei Obermenüs.

## Constraints

- Eine ausgelieferte Kombination wechselt ihre Bedeutung nicht (Kommentar bei `editor_umschalten` in `resources/default-keymap.toml`). Beide Möglichkeiten halten das: `opt+cmd+e` schließt mit dem Fokus im Editor weiter den Editor. Es geht allein um die Anzeige im Menü.
- Höchstens zwei Funktionen desselben Zustellers tragen eine Kombination (`im_weg`). Das Paar ist damit voll.
- `opt+shift+cmd+e` wird nach dem Nutzerentscheid vom 261029 nicht belegt. Die Frage betrifft allein `opt+cmd+e`.

## Recommendation

Möglichkeit 1. Sie folgt der Regel, die schon gilt (der frühere Eintrag der Leiste behält das Kürzel, wie bei `cmd+1`), und stellt die Befehle zusammen, die der Nutzer gemeinsam sucht. Der Preis ist eine Anzeige und keine Wirkung. Der Plan ist auf diese Antwort gebaut; Möglichkeit 2 ändert die Zeile in `bereich_des_kommandos` (Schritt 2), die Stelle in der Belegungsdatei (Schritt 3), die Menüprobe und `der_bereich_editor_fuehrt_genau_die_befehle_des_editors` (Schritt 4) und den Satz zum Menü in der Anleitung (Schritt 6).

---
Answered: Nutzer im Chat, 261029, Antwort „1, Aa Ba Cb“ zur Planfreigabe — Möglichkeit 1: „Im Terminal bearbeiten“ unter „Dateioperationen“ zeigt opt+cmd+e, „Editor schließen“ steht ohne Kürzel; ruled by user, Kai Stalmann <ks@qantr.com>
