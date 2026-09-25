# Coder: `fenster_einblenden` steht auf der Ausnahmeliste

**Datum:** 260813-1330
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** genau ein Defekt —
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/issues/260813-1258_*_fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar.md`.
Die sieben übrigen offenen Defekte des Circles bleiben liegen, die
Prosastellen, die dem Code hinterherlaufen, ausdrücklich eingeschlossen.
**Abnahme:** `make check` Exit 0 (build, test, clippy unter `-D warnings`, fmt).
Proben in `kommandos::zulaessigkeit` vorher 11, nachher 12.

## Der Befund, am Baum nachgelesen

Er trägt. Die in A1 gebaute vierte Bedingung faltet „ein fremdes Fenster steht
vorn" und „gar kein Fenster steht vorn" zu `schluesselfenster_gehoert_krk ==
false`. Nach `fenster_schliessen` (`performClose:`, `anwendung.rs:3508`)
überlebt das Fenster, gibt aber den Schlüsselrang ab; KRK hält genau eines,
also liefert `keyWindow()` nichts. `zulaessig` wies `FensterEinblenden` damit
ab, und `validateMenuItem:` graute den Eintrag über dieselbe Regel aus. Übrig
blieb `applicationShouldHandleReopen:`, also der Klick auf das Dock-Symbol.

`Kommando::FensterEinblenden` trägt `Wirkungsbereich::Ueberall`
(`krk-core/src/tasten/belegung.rs:749`), kam vor der Runde 8 also durch. Der
Verlust ist neu und trifft die Randbedingung „Kein Verlust gegenüber heute"
sowie C7 der Runde 1.

## Der gewählte Zuschnitt

Weg 1 des Defekts, ein Eintrag mehr auf der Ausnahmeliste. Er folgt dem Satz,
den der Entscheid
`decisions/260813-1110_a_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`
der Liste gegeben hat: sie hebt jede Sperre auf, die nach der **Lage** fragt,
und keine, die nach dem **Wirkungsbereich** fragt. Bestandteil (3) bleibt
unberührt.

Weg 2, ein vierter Wert `Keines` in `Schluesselfenster`, ist nicht gegangen:
er zieht eine fünfte Eingabe in die Regel oder eine zweite Lesart des vierten
Feldes nach sich und wächst die Tafel von 280 auf 420 Fälle. Die Faltung
besteht damit fort, fällt nach dieser Behebung aber an keinem Befehl mehr an,
den der Nutzer als Verlust bemerkt.

## Was geändert wurde

Eine Datei: `crates/krk-ui/src/kommandos/zulaessigkeit.rs`.

1. `immer_erreichbar` führt drei Kommandos statt zwei:

```rust
matches!(
    kommando,
    Kommando::Beenden | Kommando::FensterSchliessen | Kommando::FensterEinblenden
)
```

2. Der Modulkopf trägt einen Absatz zur Herleitung des dritten Eintrags: ein
   geschlossenes Fenster gibt den Schlüsselrang ab, die Lage meldet danach
   denselben Wert wie vor einem fremden Fenster, und Cmd+N ist der eine
   Rückweg daraus.
3. Drei Prosastellen zählten „beide Einträge" und zählen jetzt drei
   (Modulkopf, Doc an `immer_erreichbar`, Doc an
   `die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf`). Die
   Kurzbeschreibung von `immer_erreichbar` nennt neben dem fremden auch das
   fehlende Schlüsselfenster — genau die Lage, die der Defekt aufgedeckt hat.
4. Neue Probe `ohne_schluesselfenster_kommt_fenster_einblenden_durch`, das
   Gegenstück zu `vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl`:
   über alle fünf Fokuswerte kommt der Befehl in
   `lage(false, false, false, fokus)` durch. Die erste Zusicherung nennt den
   Weg, über den er das tut; ohne den Listeneintrag bliebe die Schleife rot.

Die Tafel aus 280 Fällen bleibt unberührt: ihr Stellvertreter für `Ueberall`
ist `LeisteUmschalten`, und `jeder_stellvertreter_traegt_den_bereich_den_er_vertritt`
hält fest, dass keiner der sieben auf der Ausnahmeliste steht.

## Nebenwirkung, benannt und geprüft

Cmd+N kommt jetzt auch während eines Blattes und während einer Umbenennung in
der Liste durch. `Kommando::FensterEinblenden` ruft `fenster_zeigen`, also
`makeKeyAndOrderFront` plus `activate` (`anwendung.rs:3484`): das holt ein
bereits vorderes Fenster nach vorn und tut sonst nichts. Ein anhängendes Blatt
behält dabei den Schlüsselrang.

## Nicht behoben, weil außerhalb des Schnitts

Dieselbe Ursache trifft zwei weitere `Ueberall`-Befehle: `weitere_instanz`
(opt+cmd+n) und `belegung_ansehen` (F1). Beide sind ohne Fenster weiterhin
abgewiesen. `belegung_ansehen` stellt ein Blatt am Hauptfenster auf und könnte
ohne Fenster ohnehin nichts zeigen; `weitere_instanz` wäre ohne Fenster
sinnvoll. Ein Verlust gegenüber heute ist keiner von beiden, sobald der
Rückweg über Cmd+N steht — wer sie dennoch auf die Liste will, braucht den
genannten Grund je Eintrag, den die Liste ausdrücklich verlangt.

## Am laufenden Bündel ungeprüft

Wie der Defekt selbst ist die Behebung am Quelltext erhoben. Die Bestätigung
gehört in die Abnahmeliste aus E2: Fenster über Shift+Cmd+W schließen, dann
Cmd+N drücken und das Menü „Fenster" öffnen. Kein `make bundle` gefahren.

## Datensatz

`260813-1258_o_…` → `260813-1258_c_…`, Abschnitt `Resolved:` angehängt.
Nicht committet: das erledigt der Orchestrator.
