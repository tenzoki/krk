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

---
Resolved: Am 260813 über den ersten der beiden Wege behoben — `Kommando::FensterEinblenden` steht jetzt in `immer_erreichbar` (`crates/krk-ui/src/kommandos/zulaessigkeit.rs`). Das folgt der Bedeutung, die der Entscheid `decisions/260813-1110_a_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md` der Liste gegeben hat: sie hebt jede Sperre auf, die nach der Lage fragt, und keine, die nach dem Wirkungsbereich fragt. Bestandteil (3) bleibt damit unberührt, und `fenster_einblenden` trägt `Wirkungsbereich::Ueberall`, kommt also in jedem Fokus durch.

Der zweite Weg, ein vierter Wert `Keines` in `Schluesselfenster`, ist **nicht** gegangen worden. Die Faltung besteht unverändert fort; sie fällt nach dieser Behebung an keinem Befehl mehr auf, den der Nutzer als Verlust bemerkt.

**Vier Stellen im Baum, alle in derselben Datei:**

1. `immer_erreichbar` führt drei statt zwei Kommandos.
2. Der Modulkopf trägt einen neuen Absatz zur Herleitung: nach `fenster_schliessen` liefert `keyWindow()` nichts, KRK hält genau ein Fenster, und die Lage meldet darum denselben Wert wie vor einem fremden Fenster.
3. Drei Prosastellen zählten „beide Eintraege"; sie zählen jetzt drei. Die Kurzbeschreibung von `immer_erreichbar` nennt neben dem fremden auch das fehlende Schlüsselfenster.
4. Neue Probe `ohne_schluesselfenster_kommt_fenster_einblenden_durch` als Gegenstück zu `vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl`: über alle fünf Fokuswerte kommt der Befehl ohne Schlüsselfenster durch, und die erste Zusicherung nennt den Weg, über den er das tut.

**Verifikation:** `make check` (build, test, clippy mit `-D warnings`, fmt), Exit 0. Die neue Probe läuft grün, ebenso die Tafel aus 280 Fällen und `die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf`, die jetzt über drei Einträge geht.

**Nicht behoben, weil außerhalb dieses Schnitts:** die beiden weiteren `Ueberall`-Befehle, die dieselbe Ursache trifft — `weitere_instanz` (opt+cmd+n) und `belegung_ansehen` (F1). Beide sind ohne Fenster weiterhin abgewiesen. `belegung_ansehen` stellt ein Blatt am Hauptfenster auf und könnte ohne Fenster ohnehin nichts zeigen; `weitere_instanz` startet ein zweites KRK und wäre ohne Fenster sinnvoll. Der Bedienweg zurück ist mit `fenster_einblenden` da, also ist keiner der beiden ein Verlust gegenüber heute; wer sie dennoch will, braucht einen genannten Grund je Eintrag, so wie die Liste ihn verlangt.

---

**Abgleich 260813-1345: die Behebung hält, alle vier Stellen nachgelesen.**

1. `immer_erreichbar` führt drei Kommandos: `Beenden`, `FensterSchliessen`, `FensterEinblenden`
   (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:198-201`).
2. Der Modulkopf trägt die Herleitung über das fehlende Schlüsselfenster.
3. Die drei Prosastellen zählen drei (`:107`, `:193`, dazu die Kurzbeschreibung an
   `immer_erreichbar`).
4. Die Probe `ohne_schluesselfenster_kommt_fenster_einblenden_durch` steht bei `:527-540` und
   geht über alle fünf Fokuswerte.

`make check` beim Abgleich wiederholt: exit 0, `cargo test --workspace` 1025 Proben grün,
`clippy --all-targets -- -D warnings` grün. Die Tafel aus 280 Fällen bleibt von der Erweiterung
unberührt, weil ihre Stellvertreter keinen Eintrag der Ausnahmeliste führen (`:361-364`).

**Die beiden ausdrücklich nicht behobenen Punkte bestehen wie beschrieben fort** und sind kein
Widerspruch zur Schliessung: die Faltung von „fremdes Fenster" und „kein Fenster" in
`Schluesselfenster::Fremd` (`crates/krk-ui/src/appkit/anwendung.rs:2623-2639`), und die beiden
weiteren `Ueberall`-Befehle `weitere_instanz` und `belegung_ansehen`, die ohne Fenster weiterhin
abgewiesen sind.

**Ein Nebensatz ausserhalb dieses Datensatzes ist durch die Behebung falsch geworden**: der
Nachtrag aus Schritt A3 am Defekt der Runde 6 zählt die Ausnahmeliste mit zwei Einträgen.
Abgelegt als
`260813-1345_o_der-nachtrag-aus-a3-zaehlt-die-ausnahmeliste-mit-zwei-eintraegen-und-turn-2-hat-einen-dritten-gebracht.md`.
