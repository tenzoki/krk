# S16: `appkit/editor` — die Textfläche als fünfter Bereich

**Status:** Complete
**Agent:** coder
**Datum:** 260809-1603
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 16 (Phase C)

---

## Was entstanden ist

| Datei | Was daran neu ist |
|---|---|
| `crates/krk-ui/src/appkit/editor.rs` | neu, 200 Zeilen: `Editorbereich`, `EditorIvars`, `textflaeche_bauen` |
| `crates/krk-ui/src/appkit/mod.rs` | `mod editor;`, der Modulkopf zählt zwanzig Module und acht Pfeile nach außen |
| `crates/krk-ui/src/appkit/aufteilung.rs` | zwei Codezeilen: der fünfte Parameter und `teiler.addSubview(editor)` |
| `crates/krk-ui/src/appkit/anwendung.rs` | `AnwendungsIvars::editor`, der Bau in `oberflaeche_aufbauen`, `ist_editorflaeche` |
| `crates/krk-ui/src/editormodell.rs` | nur der Kommentar über `#![allow(dead_code)]` |

Die Fläche ist eine editierbare `NSTextView` in einer `NSScrollView`. Der
`NSTextStorage`, der `NSLayoutManager` und der `NSTextContainer` kommen mit der
Textansicht und werden nicht von Hand gebaut; ein zweiter Textbestand entsteht
nicht. `Editorbereich::stand_einsetzen` ist die eine Stelle, die den Text der
Fläche ersetzt, und die Ausleihe des Modells endet vor dem Aufruf in das
Textsystem.

## Wie die Nämlichkeit nach außen gemeldet wird

Über eine Zugriffsfunktion auf das Objekt, nicht über eine Kennung und nicht
über eine Klasse:

```text
ereignisse::ersthelfer_gehoert_appkit
   │ ruft den dritten Abschluss
   v
Anwendungsdelegierter::ist_editorflaeche(ersthelfer)
   │ ivars().editor.get()
   v
Editorbereich::textflaeche()  ──>  ersthelfer.isEqual(Some(…))
```

`ist_editorflaeche` antwortete seit S4 mit `false` und einem Vermerk, dass S16
sie ablöst. Sie vergleicht jetzt den Ersthelfer des Schlüsselfensters mit der
Textfläche des Editors. `get()` und nicht `expect()`, wie
`Anwendungsdelegierter::fokus` es für Leiste und Vorschau seit der Runde 1 tut:
ohne gebauten Editor gibt es keine Fläche, mit der zu vergleichen wäre, und die
Antwort ist `false`.

Die Textfläche ist **keine** eigene Klasse. Ein `define_class!` für sie brächte
nichts: sie nimmt den Ersthelferrang als editierbare Textansicht von selbst, und
die Nämlichkeitsfrage vergleicht Objekte. Das `define_class!` aus der
Schrittbeschreibung ist `Editorbereich`; es hält Bildlaufansicht, Textfläche und
Modell in einem `RefCell`.

## `auslegen` konnte unverändert bleiben

Der Kommentar in `aufteilung.rs` hat es vorausgesagt, und es ist eingetreten.
Der Codeanteil des Diffs in dieser Datei sind genau zwei Zeilen:

```
+        editor: &NSView,
+        teiler.addSubview(editor);
```

`steht_im`, `gemessene_breiten`, `gemessene_sichtbarkeit`, `auslegen`,
`grenze_links` und `grenze_rechts` tragen keine geänderte Zeile. Sobald die
fünfte Unteransicht hängt, liefert `steht_im` für den Editor `true`, solange er
nicht ausgeblendet ist, und Zähler wie Zuteilung nehmen ihn von selbst auf. Die
drei Kommentare, die den fehlenden fünften Bereich beschrieben (`steht_im`,
`gemessene_breiten`, `auslegen`), sind auf den neuen Stand gezogen; der
Modulkopf zeigt jetzt fünf Bereiche.

Zwei Stellen daneben tragen den Editor schon richtig und brauchten nichts:
`Aufteilung::anwenden` blendet über `Bereich::ALLE` aus, bevor die erste
Zeichnung fällt (`oberflaeche_aufbauen` ruft `aufteilung_nachziehen` vor
`fenster_zeigen`), und `Fenstermodell::breiten_uebernehmen` übernimmt eine
gemessene Breite nur für einen sichtbaren Bereich über 0. Ein ausgeblendeter
Editor überschreibt seine gespeicherte Breite deshalb nicht mit 0.

## Das `allow(dead_code)` ist **nicht** gefallen

Die Ankündigung aus S15 war falsch, und zwar messbar. Mit entfernter Zeile
meldet `cargo clippy --workspace --all-targets` **vierzehn** Fundstellen toten
Werts in `editormodell.rs`.

Der Grund: die Fläche dieses Schrittes leiht sich zwei Stücke des Modells,
`Editormodell::neu` und `Editormodell::stand`. Jedes andere hängt an einem
Befehl, und der Befehl kommt mit seinem eigenen Schritt.

| Schritt | Was dort seinen ersten Aufrufer bekommt |
|---|---|
| S24 | `Ladevorgang`, `Ladeausgang`, `einziehen`, `oeffnen`, `laedt_noch` |
| S25 | `Sicherungsausgang`, `sichern`, `Stempel::von_pfad`, `fremd_geaendert` |
| S26 | `bearbeiten`, `hat_ungesicherten_stand` |
| S33 | `Ansicht`, `ansicht_umschalten`, `Dateityp`, `typ` |
| S36 | `Suchlauf` mit seinen sechs Methoden, die vier Suchbefehle |
| S37 | `treffer_ersetzen`, `alle_treffer_ersetzen` |

Der letzte davon ist S37. Der Kommentar an der Zeile nennt jetzt ihn als
ablösenden Schritt, dazu die Messung und das Datum; die Aufstellung nach
Schritten steht dort in einer Zeile und hier in der Tabelle, damit sie nicht an
zwei Stellen auseinanderläuft.

## Reiner Text, und warum das kein Beiwerk ist

`setRichText(false)` und vier abgeschaltete Ersetzungen: automatische
Anführungszeichen, Bindestriche, Textersetzung und Rechtschreibkorrektur. Eine
typografische Ersetzung ändert Programmtext still, und C4 sagt zu, dass der
gesicherte Stand der getippte ist. Die Formatansicht aus C3 widerspricht dem
nicht: sie färbt nach S33 über vorübergehende Merkmale des Layoutverwalters ein,
die den Textspeicher nicht anfassen.

## Die vier Abnahmekommandos

Alle vier mit `export PATH="$HOME/.cargo/bin:$PATH"` gefahren, alle vier grün:

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0, keine Warnung |
| `cargo test --workspace` | 0, fünfzehn Testziele, kein Fehlschlag |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |
| `cargo fmt --all --check` | 0 |

Dazu die vier Kriterien des Schrittes:

- `cargo xtask bundle` baut und signiert `target/KRK.app` mit der
  Entwicklungsidentität.
- Der Diff zeigt fünf `addSubview`-Aufrufe in `Aufteilung::bauen`, in der
  Reihenfolge von `Bereich::ALLE`: Leiste, links, rechts, Vorschau, Editor.
- `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src`
  nennt weiterhin genau `crates/krk-ui/src/appkit/mod.rs`.
- `grep -c 'objc2' crates/krk-ui/src/editormodell.rs` liefert weiterhin 0.

## Was der nächste Schritt vorfindet

S17 baut den Fokuszweig. Er findet `Editorbereich::textflaeche` vor, dieselbe
Funktion, die schon die Nämlichkeitsfrage beantwortet; ein zweiter Weg zur
Fläche entsteht nicht. Die beiden Zweige `Fokus::Editor => false` in
`anwendung.rs` stehen unberührt, wie die Aufgabenstellung es verlangt.

S24 findet das Modell in `Editorbereich`s `RefCell` und `stand_einsetzen` als
die eine Stelle, die den Text der Fläche ersetzt.

## Angelegte Datensätze

Keine. Der einzige Befund dieses Schrittes, die falsche Ankündigung aus S15
über das `allow(dead_code)`, ist am Code berichtigt und im Plan als
Umsetzungsvermerk festgehalten; er hält keinen Schritt auf.

## Nicht angefasst

`crates/krk-core/`, `resources/default-keymap.toml` und
`crates/krk-ui/src/appkit/menue.rs` sind unberührt.
