# Coder: Strang A der Runde 8 — die Zulässigkeitsregel schließt die Schlüsselfensterlücke

**Datum:** 260813-1210
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte A1, A2 und A3 aus
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
nicht mehr und nicht weniger. Strang B, C und D bleiben ausdrücklich liegen; ein
zweiter `coder` arbeitet gleichzeitig an `xtask/` und `README.md`.
**Abnahme:** `make check` Exit 0 (build, test, clippy unter `-D warnings`, fmt).
Proben in `kommandos::zulaessigkeit` vorher 10, nachher 11.

## Was gebaut wurde

**A1 — `crates/krk-ui/src/kommandos/zulaessigkeit.rs`.** `Lage` trägt ein viertes
Feld, `schluesselfenster_gehoert_krk`, und `zulaessig` fragt es innerhalb des
`durchgelassen`-Ausdrucks, nicht darüber:

```rust
let durchgelassen = immer_erreichbar(kommando)
    || (lage.schluesselfenster_gehoert_krk
        && kein_blatt_oder_erlaubt
        && !lage.ersthelfer_gehoert_appkit);
```

Damit gilt Möglichkeit 1 aus
`decisions/260813-1110_a_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`:
`beenden` und `fenster_schliessen` kommen weiter durch, auch vor einem fremden
Schlüsselfenster. Der Modulkopf führt jetzt vier Bestandteile, die Grafik eine
vierte Eingabe, und der Abschnitt über die Ausnahmeliste sagt in einem Satz,
was sie aufhebt: jede Sperre, die nach der Lage fragt, und keine, die nach dem
Wirkungsbereich fragt.

Die Tafel heißt `die_tafel_aus_zweihundertachtzig_faellen_geht_auf` und deckt
280 Fälle: sieben Wirkungsbereiche mal fünf Fokuswerte mal acht
Wahrheitskombinationen. Sieben der acht Achtel sind vollständig abgewiesen.
Neu ist `vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl`
mit `Kommando::LeisteUmschalten` als Stellvertreter; ohne sie zeigte keine
Probe den Unterschied zur Regel der Runde 7, weil er allein in der Zeile
`Ueberall` anfällt.

**A2 — `crates/krk-ui/src/appkit/anwendung.rs`.** Neu ist das private
`enum Schluesselfenster { Hauptfenster, BlattAmHauptfenster, Fremd }` mit
`gehoert_krk()`, beides ohne Auffangzweig. `schluesselfenster()` liest
`NSApplication::keyWindow` einmal und vergleicht über `isEqual:` gegen das
Hauptfenster und gegen dessen `attachedSheet`. `fokus` ist in
`fokus_bei(Schluesselfenster)` aufgeteilt und bleibt als Hülle für seine fünf
übrigen Aufrufer stehen. `lage` erhebt das Schlüsselfenster **einmal** und gibt
denselben Wert an beide Abnehmer, das Feld und `fokus_bei`.

**A3 — der Defektdatensatz der Runde 6.**
`circles/260812-1000-…/issues/260812-1529_o_die-blattregel-sieht-den-freigabedialog-nicht.md`
trägt einen angehängten Abschnitt: die neue Regel erledigt jedes fremde
**Fenster**, erreicht den Freigabewähler aber nicht, weil er über
`showRelativeToRect:` erscheint und im Schlüsselfenster keine Spur hinterlässt.
Der Datensatz ist **nicht** geschlossen und **nicht** umbenannt; die eine
Beobachtung, an der er hängt, steht als E2 im Plan und ist Nutzerarbeit.

## Kein Verhaltenswechsel außer dem beabsichtigten

`fokus` antwortete schon vorher `Anderswo`, sobald das Schlüsselfenster nicht
das Hauptfenster war. `fokus_bei` bildet das eins zu eins ab: `Hauptfenster`
geht in den Ansichtsbaum, `BlattAmHauptfenster` und `Fremd` antworten
`Anderswo`. Was sich ändert, ist genau die Zeile `Wirkungsbereich::Ueberall` —
sie kam vor einem fremden Fenster bisher durch und kommt jetzt nur noch über
die Ausnahmeliste durch.

## Ein Befund, gemeldet statt behoben

Vier Textstellen in drei Dateien beschreiben die Regel weiter mit drei
Bestandteilen und der Tafel aus 140 Fällen: `appkit/menue.rs:1110`,
`kommandos/mod.rs:25` und zweimal `appkit/ereignisse.rs` (`:90`, `:103`). Keine
davon steht in der Dateiliste von A1 oder A2, und der Auftrag zählt die Dateien
abschließend auf. A2 verlangt ausdrücklich, dass der Absatz über die zwei
Stellen mit zwei verschiedenen Fragen die dritte Frage nennt — dieser Absatz
steht in `ereignisse.rs`, die Dateiliste des Schritts führt allein
`anwendung.rs`. Gemeldet als
`issues/260813-1420_o_vier-modulkoepfe-ausserhalb-der-dateiliste-von-a1-und-a2-nennen-noch-drei-bestandteile.md`.

Der neue Bestandteil ist bewusst als **(4)** angehängt und nicht als (1)
eingeschoben: eine Einschiebung hätte jede bestehende Nummernnennung im Baum
verschoben, auch die in `ereignisse.rs`, die niemand nachziehen durfte.

## Abnahme

```
make check
```

Exit 0. `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`
laufen darin in einem Zug. Kein `make bundle` und kein `cargo xtask bundle`:
unter `target/KRK.app` liegt ein beglaubigtes Bündel.

Ein erster Lauf brach mit Exit 2 an einem Formatierungsbefund in
`xtask/src/release.rs:969` ab, also an der Datei des zweiten `coder`. Der
Wiederholungslauf nach dessen Änderung ist grün.
