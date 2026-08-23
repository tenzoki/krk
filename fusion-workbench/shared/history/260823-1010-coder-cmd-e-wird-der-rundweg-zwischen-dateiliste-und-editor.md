# cmd+e wird der Rundweg zwischen Dateiliste und Editor

**Date:** 2026-08-23 10:10
**Status:** Complete
**Agent:** coder
**Grundlage:** `shared/decisions/260820-1034_a_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`,
Abschnitt „Antwort des Nutzers, 260823-0942"

## Was gebaut ist

`cmd+e` trägt seit diesem Stand drei fokusabhängige Bedeutungen:

| Fokus | was `cmd+e` tut | derselbe Rumpf wie |
|---|---|---|
| Dateifenster | den ausgewählten Eintrag im Editor öffnen | `f4` (`im_editor_oeffnen`) |
| Vorschau | die angezeigte Datei im Editor öffnen | unverändert seit dem 260807-2139 |
| Editor | den Editor schließen, Vorschau zurück, Fokus in die Dateiliste | `opt+cmd+e` (`editor_schliessen`) |

Der Rückweg **schließt** und blendet nicht aus: er geht durch `editor_schliessen`, gibt die
Datei frei und löst die Nachfrage aus C4 aus. Der Nutzer hat die Wahl am 260823-0942 mit
diesem Preis vorgelegt bekommen und so getroffen.

## Die Gestalt der Änderung

**Die Fallunterscheidung steht als reine Funktion mit genau einem Rufer**, nach dem Vorbild
von `crates/krk-ui/src/kommandos/rueckschritt.rs`, und nicht als `if` im Ausführungszweig:

```
Fokus ──> rundweg() ──> Some(Rundweg) ──> einer von drei bestehenden Rümpfen
                   └──> None (Leiste, Blatt): der Befehl kommt nicht her
```

`crates/krk-ui/src/kommandos/rundweg.rs` ist neu. Die Probe
`die_regel_hat_genau_einen_aufrufer` hält die Zahl der Rufer fest, die Probe
`der_wirkungsbereich_und_die_regel_lassen_dieselben_bereiche_durch` hält die Regel gegen
`Kommando::wirkungsbereich`. Läuft eines von beiden auseinander, gäbe es entweder einen
Bereich, in dem der Befehl durchkommt und nichts findet — die Gestalt, die gerade als
Defekt gemeldet war —, oder einen Ausgang, den keine Taste je erreicht.

**Kein zweiter Weg.** Alle drei Zweige rufen bestehende Rümpfe. Für den Rückweg ist der
eine Rumpf `editor_schliessen` **herausgezogen** statt abgeschrieben worden: er nimmt jetzt
das Argument `vorschau_danach: bool` und hat zwei Rufer, `opt+cmd+e` mit `false` und den
Rückweg mit `true`.

**`Anlass::EditorSchliessen` trägt das Feld weiter bis hinter die Nachfrage.** Das musste
so sein: sagt der Nutzer in der C4-Rückfrage „Abbrechen", bleibt der Editor stehen, und
die Vorschau darf ihn dann gerade nicht verdrängen. Ein Feld und kein vierter `Anlass` —
der Anlass ist derselbe, nur die Fortsetzung unterscheidet sich um eine Zeile.

## Der Wirkungsbereich: ein Wert ersetzt, keiner hinzugefügt

`Wirkungsbereich::Vorschau` ist gefallen und `Wirkungsbereich::Dateibereiche`
(Beschriftung „Dateifenster, Vorschau und Editor") an seine Stelle getreten. Der Grund ist
nicht Sparsamkeit: `Vorschau` trug genau einen Befehl, nämlich den, der jetzt drei Bereiche
trägt. Ein Wert ohne Träger hätte den Stellvertreter der Tafel aus 280 Fällen in
`kommandos/zulaessigkeit.rs` unbesetzt gelassen, und die Tafel verlangt zu jedem Wert ein
Kommando. Die Zahl der Werte bleibt damit bei sieben, und jede Prosastelle, die von sieben
spricht, bleibt wahr.

Positiv aufgezählt und nicht als „überall außer in der Leiste": die Verneinung ließe
`Fokus::Anderswo` durch, und `cmd+e` schlösse den Editor vor einer stehenden Rückfrage.

## Umbenennungen

| alt | neu |
|---|---|
| `Kommando::EditorAusVorschau` | `Kommando::EditorRundweg` |
| Kennung `editor_aus_vorschau` | Kennung `editor_rundweg` |
| Name „Im Editor bearbeiten" | Name „In den Editor und zurück" |
| `Wirkungsbereich::Vorschau` | `Wirkungsbereich::Dateibereiche` |

Der private Rumpf `Anwendungsdelegierter::editor_aus_vorschau` behält seinen Namen: er ist
weiterhin genau der Weg aus der Vorschau und beschreibt sich richtig.

## Geänderte Dateien

- `crates/krk-ui/src/kommandos/rundweg.rs` (neu) — die reine Funktion, ihre Tafel aus fünf
  Fällen und sieben Proben
- `crates/krk-ui/src/kommandos/mod.rs` — Modul angemeldet, Modulkopf von neun auf zehn
  Module gezogen, Absatz über die zwei fokusabhängigen Regeln ergänzt
- `crates/krk-core/src/tasten/belegung.rs` — `Wirkungsbereich`, `Kommando`, `KENNUNGEN`,
  `wirkungsbereich`, `beschriftung`
- `crates/krk-ui/src/kommandos/fokus.rs` — `wirkt`, die Tafel aus 35 Paaren, der
  Editor-Durchgang, `Fokus::Editor`
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs` — Stellvertreter, die Tafel aus 280 Fällen
- `crates/krk-ui/src/belegungsmodell.rs` — `bereich_des_kommandos`, drei Kennungslisten in
  Proben
- `crates/krk-ui/src/appkit/anwendung.rs` — `Anlass`, `editor_rundweg`, `editor_schliessen`,
  `anlass_ausfuehren`, `anlass_unterbleibt`, vier Prosastellen
- `crates/krk-ui/src/appkit/editor.rs` — das Bild im Modulkopf, `Oeffnungsherkunft::Befehl`
- `crates/krk-core/tests/belegung.rs` — fünf Stellen
- `resources/default-keymap.toml` — der Eintrag selbst, dazu die Kommentare bei
  `bearbeiten`, `editor_schliessen` und `editor_umschalten`

## Prüfung

`make check` — exit 0 (Bau, 1500+ Proben, `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`).

## Was auf Codelektüre ruht und nicht auf einer Probe

Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit. Ungemessen bleiben:

1. Dass `cmd+e` im Editor bei KRK ankommt statt bei AppKit. Gelesen an
   `Anwendungsdelegierter::ist_eigene_textflaeche`: die Textfläche des Editors ist dort
   angemeldet. `opt+cmd+e` läuft über denselben Weg und tut es seit der Editor-Runde.
2. Dass der Fokus nach dem Schließen in der Dateiliste liegt. Gelesen an
   `nach_dem_sichtbarkeitswechsel`: ein ausgeblendeter Randbereich gibt den Fokus an
   `Fokus::Dateifenster` ab.
3. Dass die Vorschau danach dieselbe Datei zeigt. Gelesen an `vorschau_nachtragen`: ein
   während der ausgeblendeten Vorschau vermerkter Pfad wird beim Einblenden nachgeholt;
   ohne Auswahländerung steht ohnehin noch dieselbe Datei darin.
