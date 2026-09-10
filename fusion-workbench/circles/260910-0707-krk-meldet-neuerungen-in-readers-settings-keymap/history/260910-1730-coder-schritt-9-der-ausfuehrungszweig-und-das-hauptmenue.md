# Coder-Sitzung: Schritt 9, der Ausführungszweig und der Weg über das Hauptmenü

**Date:** 2026-09-10, 260910-1730
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 9
**Entscheid:** `260910-1600_*_was-zeigt-das-blatt-auf-abruf-wenn-der-start-nichts-erhoben-hat.md`, Möglichkeit 1
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

Der Befehl „Neuerungen anzeigen" tut jetzt etwas. Bis zu diesem Schritt stand er mit Namen
und Kombination im Hauptmenü und fiel im Ausführungs-`match` durch den Auffangzweig; das
Blatt aus Schritt 6 hatte keinen Rufer.

### `crates/krk-ui/src/appkit/anwendung.rs`

- **Der Zweig** `Kommando::NeuerungenZeigen => self.neuerungen_zeigen()` steht im `match`
  von `Anwendungsdelegierter::kommando_ausfuehren`, neben dem Notizzettel und aus demselben
  Grund: `Wirkungsbereich::Ueberall`, kein Bereich der Fensterzeile als Gegenstand.
- **`neuerungen_zeigen`** entscheidet, welchen Bestand das Blatt zeigt. Hält der `ivar`
  einen (der Start hat erhoben), zeigt es den vom Start und fasst die Platte nicht an.
  Steht dort `None`, wird auf Verlangen nachgetragen: `unter_der_sperre(neuerungen::erheben)`.
- **`neuerungen_blatt`** ist die eine Stelle, die `blaetter::neuerungen::zeigen` ruft; der
  Griff geht nach `offenes_blatt` wie bei jedem anderen Blatt, damit `esc` das Blatt
  schließt.
- Der Doc-Kommentar am `ivar` `neuerungen` sagt jetzt, dass ein `None` sich auch durch den
  Befehl **nicht** füllt.

**Die Taste und der Menüeintrag brauchen keine zwei Zweige.** Beide Wege enden in
`kommando_ausfuehren`: der Ereignisabgriff über `appkit/ereignisse.rs`, der Menüeintrag
über seinen Melder, und der Kopf von `appkit/menue.rs` schreibt hin, dass es keinen zweiten
Ausführungsweg gibt.

### Der Nutzerentscheid vom 260910-1600, im Code

`None` im `ivar` heißt „nicht erhoben, weil für diese Fassung schon gemeldet ist" und nicht
„kein Unterschied" — das ist der häufigste Fall, denn er tritt bei jedem zweiten und jedem
weiteren Start derselben Fassung ein. Der Zweig verwechselt die zwei Auskünfte nicht: ein
Blatt „nichts Neues" auf einen leeren `Option` hin wäre eine Behauptung über Dateien, die
niemand angesehen hat.

**Der nachgetragene Bestand wird nicht in den `ivar` gelegt.** Dort stünde sonst ein dritter
Zustand, den das Feld nicht erklärt: weder „vom Start" noch „nicht erhoben". So bleibt die
Regel eine einzige — ist beim Start erhoben worden, zeigt jeder Abruf denselben Stand vom
Start; sonst zeigt jeder Abruf, was gerade auf der Platte steht.

**Zwei Ausgänge, und der zweite wird gemeldet.** Die Nacherhebung braucht einen `Zugang`,
also die Schreibsperre. `Sperrhindernis::OhneOrdner` und `Sperrhindernis::Gesperrt` bekommen
je eine eigene Antwort in der Statuszeile, wie bei jedem anderen Rufer von
`unter_der_sperre`. Ein stiller dritter Ausgang entsteht nicht: ein Tastendruck, der nichts
zeigt und nichts sagt, ist von einem Defekt nicht zu unterscheiden.

Die Erhebung beim Start ist **nicht** angefasst. Möglichkeit 3 des Datensatzes — beim Start
immer erheben — hätte die Bedingung gebrochen, unter der die L4-Frage beantwortet ist; die
Probe `bei_gleichem_merker_wird_keine_der_drei_dateien_geoeffnet` hält sie und bleibt grün.

### Das `#[expect(dead_code)]` ist gefallen

`blaetter::neuerungen::zeigen` trug seit Schritt 6 ein `#[expect(dead_code, reason = …)]`
mit Ablaufdatum. Mit dem Rufer wird die Ausnahme selbst zur Warnung, und unter `-D warnings`
wäre der Dokumentationslauf rot geworden. Sie ist entfernt, und der Doc-Kommentar sagt
stattdessen, welchen Bestand der eine Rufer hereinreicht.

## Abweichung von der Dateiliste des Schrittes

**Zwei Dateien im Kern sind dazugekommen, und der Nutzerentscheid vom 260910-1600 verlangt
sie.** Der Schlusssatz des Blattes lautete wörtlich:

> Gezeigt ist der Stand vom Start; KRK liest diese Dateien im Betrieb nicht neu.

Mit der Nacherhebung auf Verlangen ist dieser Satz **im häufigsten Fall falsch**: dann ist
gezeigt, was eine Sekunde zuvor von der Platte gelesen wurde. Die Datei, die ihn schreibt,
kennt den Unterschied nicht und soll ihn nicht kennenlernen — `blatttext` bekommt einen
`Bestand` und nicht seine Herkunft. Der Satz sagt deshalb jetzt, was in **beiden** Fällen
gilt:

> Gezeigt ist der Stand, den KRK zuletzt gelesen hat. Womit KRK arbeitet, steht seit dem
> Start fest: eine geänderte Datei wirkt erst beim nächsten Start.

Die zweite Hälfte ist die Auskunft, auf die es dem Nutzer ankommt; sie war schon vorher der
Grund für den Satz. Angefasst sind dafür `crates/krk-core/src/ablage/neuerungen.rs` (der
Wortlaut und der Doc-Kommentar darüber) und `crates/krk-core/tests/ablage.rs` (die Probe,
die den Wortlaut hält). Dazu die Zeichnung im Kopf von
`crates/krk-ui/src/appkit/blaetter/neuerungen.rs`, die den Weg über `neuerungen_zeigen`
zeigt statt eines Pfeils vom Kommando unmittelbar auf `zeigen`.

## Die Pflichtstelle ist jetzt gehalten

Der Plan nennt den Zweig „die Pflichtstelle, die weder der Übersetzer noch eine Probe
hält". Sie hält jetzt eine:
`neuerungsproben::der_befehl_hat_einen_eigenen_ausfuehrungszweig`
(`crates/krk-ui/src/appkit/anwendung.rs`) liest den Quelltext von `kommando_ausfuehren` und
verlangt den Zweig. Die Nadel steht zusammengesetzt da (`concat!`), wie der Kopf von
`crate::quellbaum` es verlangt: als ein Stück geschrieben fände die Probe sich selbst und
wäre grün, ohne dass der Zweig steht.

Am laufenden Bündel ist nichts geprüft; das ist der Abnahmelauf und Nutzerarbeit.

## Abnahme

- `make check` — Rückgabewert **0**, alle fünf Kommandos in ihrer Reihenfolge, zweimal
  gefahren (der erste Lauf gab die Zeile „alle fuenf gruen", der zweite den Code selbst).
