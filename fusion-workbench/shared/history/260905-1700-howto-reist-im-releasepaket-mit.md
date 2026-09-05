# HowTo.md reist im Releasepaket mit

**Agent:** coder
**Datum:** 260905-1700
**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** keiner aktiv, Artefakte nach `shared/`

## Auftrag

`HowTo.md` reist in `target/KRK-<zahl>.zip` neben `KRK.app` mit. Das Zip der
Einreichung bei Apple (`target/KRK.zip` aus `beglaubigung.rs`) bleibt
unangetastet.

## Vier Entscheidungen

**Was mit dem Bündel geschieht.** Nichts. Gepackt wird weiter mit demselben
`ditto -c -k --keepParent`, denselben drei Schaltern, demselben Werkzeug; was
sich ändert, ist allein die Quelle: statt des Bündels ein Ordner, in dem das
Bündel und die Anleitung liegen. Das Bündel wird mit `ditto` in diesen Ordner
kopiert und nicht mit `cp`, weil an den symbolischen Verweisen und den
erweiterten Attributen hängt, ob die Signatur nach dem Entpacken noch trägt.
Der Abnahmelauf hat es gemessen (unten).

**Wo die Datei im Zip liegt.** In einem Ordner mit der App, nicht lose daneben.
Der Ordner heißt wie das Zip ohne seine Endung, `KRK-<zahl>`; `zipname` setzt
sich seither aus `paketname` und der Endung zusammen, damit die zwei Namen nicht
auseinanderlaufen können. Nach dem Doppelklick liegt also ein Ordner
`KRK-<zahl>` da, darin `KRK.app` und daneben `HowTo.md`. Bis dahin lag die App
selbst da. Zwei Gründe: eine Datei namens `HowTo.md` lose im Ladeordner sagt
nicht, zu welchem Programm sie gehört, und ein Archiv mit mehreren Einträgen in
der Wurzel überließe die Gruppierung dem Entpacker.

**Woher der Inhalt kommt.** Eingebacken über `include_str!("../../HowTo.md")`,
wie `resources/default-keymap.toml` in `krk-core`. Der stille Ausfall ist damit
nicht behandelt, sondern abgeschafft: fehlt die Datei, übersetzt `xtask` nicht.
Ein Lesen zur Laufzeit hätte den Fehlschlag an die Stelle gelegt, an der das
Bündel gebaut, signiert und beglaubigt ist. Ausgeliefert wird dadurch der
eingecheckte Stand, was zu Station 1 passt, die keinen Lauf mit geänderter
verfolgter Datei durchlässt.

**Ob der Schritt anderswo fehlt.** Nein. `zip_packen` hat einen Rufer,
`veroeffentlichen`, und den teilen sich beide Veröffentlichungswege: die achte
Station von `release` und das eigenständige `cargo xtask veroeffentlichen`.
`cargo xtask bundle` baut allein das Bündel, und in das Bündel gehört die
Anleitung nicht. `certify-only` reicht bei Apple ein und packt dafür sein
eigenes, danach gelöschtes Zip; eine Anleitung hat dort nichts zu suchen.

## Der Releasetext sagt es jetzt

`RELEASETEXT` nennt den entpackten Ordner und was darin liegt, und ein
eigener Abschnitt sagt, was in der Anleitung steht und dass die vollständige
Tastenbelegung die laufende Anwendung selbst ausgibt, mit F1. Die Probe
`der_releasetext_traegt_jede_seiner_aussagen` ist um drei Behauptungen
gewachsen: den Ordner `KRK-<zahl>`, die Datei `HowTo.md`, den Hinweis auf F1.

## Geänderte Dateien

- `xtask/src/veroeffentlichung.rs` — `ANLEITUNG`, `ANLEITUNGSTEXT`,
  `paket_stellen`, `paketname`, `zip_packen` gegen das Paket, `RELEASETEXT`,
  drei neue und zwei erweiterte Proben
- `xtask/src/release.rs` — `Wegwerfwurzel` und `wegwerfwurzel` sind
  `pub(crate)`, damit die neue Packprobe sie leiht statt eine fünfte Fassung
  danebenzustellen
- `README.md` — Installationsschritt 1, die Stationstabelle, die Tabelle des
  Veröffentlichungswegs

## Wie geprüft wurde

`make check` grün, Rückgabewert 0 (157 Proben in `xtask`, dazu der ganze
Arbeitsbereich).

Der echte Packlauf ist durch den geänderten Code selbst gefahren, an
`target/KRK.app` in der ausgelieferten, signierten und beglaubigten Fassung
1.7.1, über eine Probe, die danach wieder entfernt wurde. Sie rief `zip_packen`,
entpackte das Ergebnis mit `ditto -x -k` und maß daran:

```
PAKETINHALT: ["HowTo.md", "KRK.app"]
codesign --verify --deep --strict --verbose=2 -> exit 0
    valid on disk / satisfies its Designated Requirement
spctl -a -vvv -t exec -> exit 0
    accepted, source=Notarized Developer ID
xcrun stapler validate -> exit 0
HowTo.md: 22111 Bytes
```

Signatur, Gatekeeper-Urteil und angeheftetes Ticket überstehen das Kopieren, das
Packen und das Entpacken also unbeschädigt.

Im Baum bleibt davon die Probe `das_zip_traegt_die_anleitung_neben_dem_buendel`:
sie packt wirklich, mit demselben `ditto`, an einem nachgemachten Bündel, und
liest die Auslegung des Archivs am entpackten Ergebnis nach. Die Signatur prüft
sie nicht — dafür braucht es ein beglaubigtes Bündel, und das hat `make check`
nicht.

## Nicht getan

Kein Commit, kein Release. Die Auslieferung fährt der Nutzer.

## Gefilte Datensätze

- `260905-1658_*_claude-md-nennt-zwei-stellen-an-denen-der-nutzer-die-betriebsregel-liest-mit-der-anleitung-im-paket-sind-es-drei.md`
- `260905-1659_*_wo-wohnt-die-betriebsregel-jetzt-da-sie-den-nutzer-an-drei-stellen-erreicht.md`
