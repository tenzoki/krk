Nach dem Schließen des Fensters ist „Fenster einblenden" weder über Cmd+N noch über das Menü erreichbar

---

Die vierte Bedingung aus A1 weist jeden Befehl ab, sobald `NSApplication::keyWindow` nichts liefert — und genau das ist die Lage, nachdem das Hauptfenster geschlossen wurde. `Kommando::FensterEinblenden` (Cmd+N) ist der eine Rückweg aus dieser Lage, steht aber nicht auf der Ausnahmeliste. Der Weg zurück zum geschlossenen Fenster über Tastatur und Menü ist damit weg; übrig bleibt allein der Klick auf das Dock-Symbol. Das ist ein Verlust gegenüber heute und widerspricht der gleichnamigen Randbedingung des Spec.

---

**Schwere:** hoch. Kein Absturz, kein Datenverlust, aber der Wegfall eines zugesagten Bedienwegs (C7 der Runde 1).

**Wo es anfällt**

`crates/krk-ui/src/appkit/anwendung.rs:2632-2647` — `schluesselfenster` faltet „kein Schlüsselfenster" und „fremdes Fenster" zu demselben Wert:

```rust
let (Some(schluessel), Some(haupt)) = (
    NSApplication::sharedApplication(self.mtm()).keyWindow(),
    self.ivars().fenster.get(),
) else {
    return Schluesselfenster::Fremd;
};
```

`crates/krk-ui/src/kommandos/zulaessigkeit.rs:164-173` — `Fremd` heißt `schluesselfenster_gehoert_krk == false`, und der ganze `durchgelassen`-Ausdruck fällt damit auf die Ausnahmeliste zusammen:

```rust
let durchgelassen = immer_erreichbar(kommando)
    || (lage.schluesselfenster_gehoert_krk
        && kein_blatt_oder_erlaubt
        && !lage.ersthelfer_gehoert_appkit);
```

`crates/krk-ui/src/kommandos/zulaessigkeit.rs:189-191` — die Liste führt zwei Befehle:

```rust
matches!(kommando, Kommando::Beenden | Kommando::FensterSchliessen)
```

`crates/krk-core/src/tasten/belegung.rs:749` — `FensterEinblenden` trägt `Wirkungsbereich::Ueberall` und kam deshalb vorher durch.

`resources/default-keymap.toml:506-513` — `fenster_einblenden` auf `cmd+n`, mit dem Kommentar „Der Rueckweg, nachdem das Fenster geschlossen wurde".

**Die Kette, am Baum nachgelesen**

1. `fenster_schliessen` ruft `performClose:` (`anwendung.rs:3508-3514`); das Fenster überlebt, wird aber ausgeordnet und gibt den Schlüsselrang ab. KRK hält genau ein Fenster, also liefert `keyWindow()` danach `None`.
2. `schluesselfenster()` antwortet `Fremd`, `gehoert_krk()` liefert `false`.
3. `zulaessig(FensterEinblenden, …)`: `immer_erreichbar` ist `false`, der zweite Zweig ist wegen `schluesselfenster_gehoert_krk == false` ebenfalls `false` → **abgewiesen**.
4. Der Tastendruck läuft unausgeführt an AppKit weiter, dort greift die Tastenentsprechung des Menüeintrags — und `validateMenuItem:` (`anwendung.rs:748-762`) fragt dieselbe Regel und graut den Eintrag aus.

**Vorher war es zulässig.** `fokus()` antwortete ohne Schlüsselfenster `Fokus::Anderswo`, `ersthelfer_gehoert_appkit` liefert ohne Schlüsselfenster `false` (`appkit/ereignisse.rs:585-587`), `blatt_steht` ebenfalls `false`, und `fokus::wirkt(Ueberall, Anderswo)` sagt ja. Der Befehl kam also durch.

**Nicht am laufenden Bündel beobachtet.** Der Befund ist am Quelltext erhoben; die Kette ist kurz und ohne Verzweigung, aber die Bestätigung gehört in die Liste aus E2: Fenster über Shift+Cmd+W schließen, dann Cmd+N drücken und das Menü „Fenster" öffnen.

**Zwei weitere Befehle trifft dieselbe Ursache**, beide mit `Wirkungsbereich::Ueberall` und beide ohne Fenster sinnvoll: `weitere_instanz` (opt+cmd+n) und `belegung_ansehen` (F1, das allerdings ein Blatt am Hauptfenster aufstellt und ohne Fenster ohnehin nichts zeigen könnte). Die übrigen der vierundzwanzig `Ueberall`-Befehle wirken auf den Fensterinhalt; sie abzuweisen ist der Zweck der Runde.

**Warum es niemandem aufgefallen ist**

Der Entscheid `decisions/260813-1110_a_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md` hat die Ausnahmeliste ausdrücklich aus „kein Verlust gegenüber heute" begründet und dabei nur die beiden Befehle geprüft, die schon darauf standen. Der Doc-Kommentar an `Schluesselfenster` (`anwendung.rs:867-877`) benennt die Faltung sogar: „`Fremd` deckt zwei Lagen, und beide sollen dieselbe Antwort bekommen … In beiden Faellen darf kein Tastenbefehl auf das Hauptfenster wirken." `FensterEinblenden` wirkt aber nicht *auf* das Hauptfenster, es holt es zurück; für diesen Befehl ist die Zusammenfassung der beiden Lagen falsch.

Keine Probe deckt den Fall ab: `die_ausnahmeliste_kommt_durch_blatt_und_textfeld` (`zulaessigkeit.rs:517-531`) geht allein über `Beenden` und `FensterSchliessen`.

**Was zu tun ist**

Zwei Wege, und der zweite verlangt eine Nutzerentscheidung:

1. **`Kommando::FensterEinblenden` auf die Ausnahmeliste.** Das folgt der Bedeutung, die der Entscheid der Liste selbst gegeben hat („sie hebt jede Sperre auf, die nach der Lage fragt"), und kostet eine Zeile in `immer_erreichbar` plus eine Zeile im Modulkopf. Nebenwirkung: Cmd+N kommt dann auch während eines Blattes und während einer Umbenennung in der Liste durch; beides holt ein bereits vorderes Fenster nach vorn und tut sonst nichts.
2. **Einen vierten Wert `Keines` in `Schluesselfenster`.** Er trennte „KRK hat gar kein Fenster vorn" von „ein fremdes Fenster steht vorn". Das ist der genauere Schnitt, zieht aber eine fünfte Eingabe in die Regel oder eine zweite Lesart des vierten Feldes nach sich und wächst die Tafel von 280 auf 420 Fälle.

Der erste Weg ist der kleinere und der, den die bestehende Mechanik vorsieht. Eine Probe gehört dazu: vor einem fremden Schlüsselfenster kommt `FensterEinblenden` durch, so wie `vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl` (`zulaessigkeit.rs:487-502`) das Gegenstück hält.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1 der Runde 8, Bereich `59b0a6c..21dbc59`.
- Berührt C5.6 des Spec, die Randbedingung „Kein Verlust gegenüber heute" desselben Spec, und C7 der Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md:340`).
- Der Rückweg über das Dock-Symbol bleibt: `applicationShouldHandleReopen:` (`anwendung.rs:779-786`) ruft `fenster_zeigen` und geht nicht durch die Zulässigkeitsregel.
