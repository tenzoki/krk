# Der Auffrischungsaufschub gilt nur noch für schnelle Vorgänge

**Agent:** coder
**Zeitpunkt:** 260806-1440
**Status:** Complete
**Defekt:** `issues/260806-1331_o_der-auffrischungsaufschub-gilt-fuer-alle-fuenf-operationsarten-statt-nur-fuer-die-schnelle.md`
**Prüfbericht:** `reviews/260806-1335-coderev-turn-23-s6b-vorschau-messstrecke.md`, Abschnitt 3
**Ausgangsstand:** `27c9515`, nicht committet

## Was umgesetzt ist

Die Nutzerentscheidung vom 260806: der Aufschub der Auffrischung gilt nur noch für
das Stapel-Umbenennen. Kopieren, Verschieben, Papierkorb und endgültiges Löschen
füllen ihren angezeigten Ordner wieder während des Laufs, so wie vor `fd5e3c5`.

Weg 1 der drei im Defekt genannten. Die Ursache eine Schicht tiefer — ein
Lesevorgang, der sein Ordnermodell leert, bevor er liefert — bleibt stehen; sie ist
in `schiebt_auffrischung_auf` als Kante festgehalten.

## Wo die Zuordnung steht

`crates/krk-ui/src/auffrischung.rs`, Funktion `schiebt_auffrischung_auf(&Art) -> bool`.
Eine vollständige Fallunterscheidung ohne Auffangzweig, an genau einer Stelle. Die
Kette darunter:

```
Anwendungsdelegierter::aufgeschobene_ordner   (anwendung.rs, AppKit-Seite)
  └─> auffrischung::aufgeschobene_ordner(art, vorgang.ordner())
        └─> schiebt_auffrischung_auf(art)     ← die eine Einordnung
  ↓
FSEvents-Rückruf: auffrischung_aufgeschoben(pfad, &aufgeschoben)
```

`gehoert_zu_vorgang` heißt jetzt `auffrischung_aufgeschoben`: die Liste, die es
bekommt, sind nicht mehr die Ordner des Vorgangs, sondern die aufgeschobenen. Der
alte Name hätte für eine laufende Kopie "nein" behauptet, obwohl der Ordner sehr wohl
zum Vorgang gehört.

`Vorgang::ordner` ist unverändert. Es bleibt die eine Aufzählung für den Aufschub
**und** die Abschlussauffrischung; die Abschlussauffrischung läuft weiterhin für jede
Art. Der Nachtrag aus `260806-1330` (`abbruch_ohne_meldung_nachtragen`) ist nicht
berührt und bleibt wirksam.

## Geänderte Dateien

- `crates/krk-ui/src/auffrischung.rs` — `schiebt_auffrischung_auf`,
  `aufgeschobene_ordner`, `auffrischung_aufgeschoben` (aus `gehoert_zu_vorgang`),
  Modulkopf, vier neue bzw. umgestellte Prüfungen
- `crates/krk-ui/src/appkit/anwendung.rs` — `vorgangsordner` → `aufgeschobene_ordner`,
  Rückruf der Dateisystemwache, Kommentar an `Vorgang::ordner`

Keine Workbench-Datei außer dieser. Nicht committet.

## Nachweis

**Beide Richtungen, mutationsgeprüft.** Vier Prüfungen in `auffrischung.rs` decken
alle fünf Operationsarten ab. Dass sie beißen, ist nicht behauptet, sondern gezeigt:

| Eingriff in den Code | Fehlschlagende Prüfungen |
|---|---|
| `aufgeschobene_ordner` reicht wieder für jede Art durch (Stand `fd5e3c5`) | 2 — `eine_laufende_kopie_haelt_ihren_zielordner_nicht_zurueck`, `nur_ein_aufschiebender_vorgang_gibt_seine_ordner_in_die_aufschubliste` |
| `Art::UmbenennenImStapel => false` | 3 — dazu `allein_das_stapel_umbenennen_schiebt_die_auffrischung_auf`, `der_ordner_eines_aufschiebenden_vorgangs_wird_erkannt` |

**Die Fallunterscheidung ist erzwungen.** Eine sechste Variante `Art::Probeart`
probeweise in `krk-core/src/operation/auftrag.rs` eingesetzt: der Bau bricht an vier
Stellen ab, `auffrischung.rs:180` ist eine davon (`E0004`, non-exhaustive patterns).
Beide Probeeingriffe sind zurückgenommen; `git diff` nennt nur die zwei Dateien oben.

## Am gebauten Bündel

`make bundle`, signiert mit "Apple Development: Kai Stalmann (FJ8U4B3QAC)",
`codesign --verify --deep --strict` gültig. MacBookPro15,1, macOS 15.7.7.

**C8 unverändert.** Durchstichstrecke, eine Runde, mit den Änderungen
(`messungen/260806-1224-durchstich.txt`): L1 100,0 % im Bild, L2 50,4 ms, L3 147,5 ms,
L4 316,7 ms, L10 57,0 ms — alle fünf gehalten.

**Sitzungsstrecke grün, vorher wie nachher.** Der vollständige Sitzungslauf
(L1, L5, L6, L7, L8, L9) läuft auf beiden Ständen bis `krk-messung fertig` durch,
je 20 Wiederholungen. L8 Median 162,4 ms vorher / 162,6 ms nachher, L9 11,7 ms /
10,4 ms. Die wiederhergestellte Auffrischung während einer Kopie kostet die beiden
Größen nichts.

### Was am Bündel nicht vorgeführt ist, und warum nicht

Weder das Kopieren noch das Umbenennen ist am Bündel als Verhalten gezeigt. Der
Grund ist nachgemessen und kein Versäumnis:

**Das Kopieren.** Die einzige Dateioperation, die der Messmodus über KRKs eigene
Ereignisschlange auslöst, ist die Kopie in L8/L9 — und die Strecke bricht sie sofort
nach der L9-Messung wieder ab. Zwischen F5 und `esc` liegen rund 300 ms (L8-Median
162 ms, L9-Median 10 ms, dazu zwei Auslösetakte von je 97 ms). Die
Sammelverzögerung des `FSEventStream` beträgt 0,3 s
(`appkit/fsevents.rs`, `SAMMELVERZOEGERUNG`). Der erste Meldestapel trifft also
frühestens zusammen mit dem Abbruch ein, und danach läuft kein Vorgang mehr, der
etwas aufschieben könnte. In diesem Fenster kann sich der Aufschub gar nicht
auswirken — vorher nicht und nachher nicht.

Gemessen bestätigt: die Zahl der Verzeichnisleser-Fäden über den ganzen
Sitzungslauf, per `sample` bei 1 ms erhoben, liegt bei 101 (vorher) gegen 98
(nachher) — Rauschen.

**Das Umbenennen.** Ein Stapel-Umbenennen braucht das Regelblatt aus C4 mit
Texteingabe; der Messmodus kennt weder den Tastendruck noch eine Anweisung, die ein
Blatt ausfüllt. Derselbe Befund stand schon im Verlaufseintrag `260806-1240`.

### Kontrolle: der Auffrischungsweg lebt am Bündel

Damit das Nullergebnis oben nicht als "die Auffrischung passiert nie" missdeutet
wird, ist der Weg gegengeprüft. KRK gewöhnlich aus dem Bündel gestartet, linkes
Dateifenster auf einem Prüfordner mit 5.000 Einträgen, 20 s beobachtet:

| Lage | Verzeichnisleser-Fäden in 20 s |
|---|---|
| kein Zutun | 0 |
| 30 fremde Änderungen im angezeigten Ordner, alle 0,5 s eine | 29 |

FSEvents, `ordner_neu_lesen` und die Messmethode arbeiten also. Ein Stapel-Umbenennen
über 5.000 Einträge im angezeigten Ordner würde denselben Zähler treiben — eine
Messstrecke, die es auslöst, gibt es heute nicht.

## Beobachtung für den Nutzer, nicht selbst entschieden

Ein Verschieben **innerhalb eines Datentraegers** läuft über `rename(2)` und ist damit
genauso schnell wie ein Stapel-Umbenennen; über genügend Einträge könnte es dieselbe
Meldelawine auslösen, die den Defekt vom 260805-1337 getragen hat. Beobachtet worden
ist das nicht, und die Entscheidung nennt das Verschieben ausdrücklich als nicht
aufschiebend — sie ist deshalb genau so umgesetzt. Der Vorbehalt steht als Kante im
Doc-Kommentar von `schiebt_auffrischung_auf`, samt dem Hinweis, dass die Antwort
darauf an die Lesestelle gehört (Weg 2 des Defekts) und nicht in eine zweite Ausnahme
in der Einordnung. Ob daraus ein eigener Defekt wird, entscheidet der Nutzer.

## Abnahme

`make check` grün: `cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`.

Modulgrenze gehalten: `auffrischung.rs` nennt keine `objc2`-Kiste und kein `unsafe`.
`#![allow(unsafe_code)]` steht weiterhin nur in `krk-core/src/verzeichnis/sys.rs` und
`krk-ui/src/appkit/mod.rs`.

Die `session.toml` des Nutzers ist gesichert und zurückgespielt worden — die
Sitzungsläufe überschreiben sie; Prüfsumme vor und nach dem Lauf
`8083a5d87a3820910c1c85e2440b9b168b8f1ac5`. Der Prüfordner der Kontrolle ist wieder
entfernt, das Kopierziel wieder leer, die vier Messplatz-Bestände unberührt.

Neu und unversioniert im Baum: `messungen/260806-1224-durchstich.txt`.
