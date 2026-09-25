# Ontocoder-Sitzung: Schritt 8, der Eintrag in der Auslieferungsbelegung

**Date:** 2026-09-10, 260910-1315
**Filed by:** ontocoder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 8
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

Ein Eintrag in `resources/default-keymap.toml`, dazu die zwei Zahlen im Dateikopf.

### Der Eintrag

```toml
[[funktion]]
id = "neuerungen_zeigen"
name = "Neuerungen anzeigen"
tasten = ["opt+cmd+i"]
```

Er steht zwischen `notizzettel` und `weitere_instanz`, also im Block der Anwendung als
ganze. Die Stelle folgt `bereich_des_kommandos`, das `neuerungen_zeigen` unter
`Funktionsbereich::Anwendung` führt; die Reihenfolge der Blöcke ist die Reihenfolge im
Menü.

Der Name trägt die Umlautschreibweise (hier ohne Umlaut im Wort), die Kennung die
Umschrift. Die Form „<Sache> anzeigen" ist die der Nachbarn: „Notizzettel anzeigen",
„Tastaturbelegung anzeigen".

### Der Begründungskommentar

Wie bei jedem anderen Block, im Stil der Datei und in Umschrift. Er sagt vier Dinge: was
der Befehl zeigt, warum er an dieser Stelle steht, warum `opt+cmd+i` frei war, und was
einem Nutzer mit älterer `keymap.toml` widerfährt.

### Die zwei Zahlen im Kopf

`# Ausgeliefert sind 93 Funktionen mit zusammen 96 Kombinationen.` wird zu `94` und `97`.
Nachgezählt an der Datei: 94 Blöcke `[[funktion]]`, 97 Einträge über alle `tasten`-Listen.

## Die Kombination

`opt+cmd+i`, vom Nutzer am 260910 festgelegt. Über alle Tastenlisten der Datei geprüft mit
`grep -oE '"[^"]+"' resources/default-keymap.toml | sort -u`: von den i-Kombinationen trug
allein `shift+cmd+i` etwas, nämlich `markierung_umkehren`. Kein Konflikt, also kein Grund
zum Abbruch; umgebaut wurde nichts.

## Prüfung

`make check` — Rückgabewert 0, „alle fuenf gruen". Die drei Proben, die Schritt 7 im
Nachtrag als rot benannt hat, sind grün, dazu die vierte, die die Kopfzahlen hält:

| Ziel | Probe | Stand |
|---|---|---|
| `-p krk-core --lib` | `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` | ok |
| `-p krk-core --lib` | `tasten::belegung::tests::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` | ok |
| `-p krk-core --test belegung` | `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | ok |
| `-p krk-ui --bin krk` | `belegungsausgabe::tests::die_dritte_spalte_haelt_die_begruendungslagen_auseinander` | ok |

Kein `FAILED` im ganzen Lauf. Die Zwischenlage aus Schritt 7 ist damit beendet.

## Was die Menüausgabe dieses Geräts zeigt, und warum das kein Befund ist

`make menue` baut das signierte Bündel und ist deshalb nicht gefahren; gefragt ist
stattdessen dasselbe Protokoll am gebauten Binärziel:
`./target/debug/krk --menue-protokoll`. Es nennt den Eintrag:

```
menue="Anwendung" eintrag="Neuerungen anzeigen" kombination=(keines) ... selektor=krkKommando:
```

**Zwei Abweichungen von der Auslieferungsbelegung, beide mit derselben Wurzel.** Der
Eintrag steht ohne Kürzel, und er steht am Fuß der Gruppe „Anwendung" hinter „KRK
beenden" statt zwischen Notizzettel und weiterer Instanz. Das Menü entsteht nicht aus
`resources/default-keymap.toml`, sondern aus der `keymap.toml` des Nutzers unter
`~/Library/Application Support/KRK/`; die stammt vom 260909, führt 93 Funktionen und die
Kennung `neuerungen_zeigen` nicht. `Belegung::bauen` hängt jede Funktion, die die
Nutzerdatei nicht nennt, unbelegt hinten an
(`crates/krk-core/src/tasten/belegung.rs`, der Zweig nach dem Kommentar „Funktionen, die
die Nutzerdatei nicht nennt, treten unbelegt hinzu").

Das ist genau der Fall, den Schritt 10 dem Nutzer erklären soll, und zugleich der Grund,
warum der Plan dort verlangt, den Befehl mit seinem Menüeintrag und nicht nur mit der
Kombination zu nennen. Angefasst ist die Nutzerdatei nicht: sie gehört dem Nutzer, und der
Handgriff dafür steht in `README.md`.

## Was nicht angefasst ist

Kein Rust, kein Commit. Angefasst sind `resources/default-keymap.toml`, der Planschritt 8
und dieses Protokoll.
