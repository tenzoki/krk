# Nur-Beglaubigungsweg für ein bereits gebautes Bündel

**Status:** Complete
**Coder**, Domäne `code`.
**Anlass:** Der Auslieferungslauf des Nutzers vom 260820 ist am Upload zu Apple in einen Zeitüberlauf gelaufen (`HTTPClientError.deadlineExceeded`). Das universelle, mit Developer-ID und gehärteter Laufzeitumgebung signierte Bündel lag fertig unter `target/KRK.app`, und es fehlte allein das Ticket. Ein zweites `./release.sh 0.5.5` bricht in dieser Lage an Station 1 ab, weil der Tag `v0.5.5` nach dem Lauf nicht mehr allein auf HEAD steht.

Der Zuschnitt ist eine Nutzerentscheidung: vier Schichten wie bei `release`, die Logik in xtask und nicht in einem Skript.

```
./certify-only.sh 0.5.5
  └─ make beglaubigen VERSION=0.5.5      Pfad zu cargo, Notarprofil
       └─ cargo xtask beglaubigen 0.5.5  die ganze Logik
```

## Was gebaut wurde

**Nichts von der Beglaubigung selbst ist neu geschrieben.** `beglaubigen(&Path)` samt `werkzeug_pruefen` und `NOTAR_PROFIL_VARIABLE` ist aus `xtask/src/release.rs` nach `xtask/src/beglaubigung.rs` gewandert; Station 7 von `release` ruft jetzt `beglaubigung::beglaubigen`. Zwei Rufer, eine Funktion, keine Kopie. Der Modulkopf von `release.rs` trägt den Verweis auf den Umzug und darauf, warum der zweite Weg Station 1 übergeht.

Der neue Unterbefehl `cargo xtask beglaubigen <zahl>` prüft zweierlei am gebauten Bündel und reicht es dann ein:

1. **Die Versionszahl gegen `CFBundleShortVersionString` der `Info.plist` im Bündel.** Die eine Sache, die das Argument rechtfertigt: `target/KRK.app` überlebt jede Sitzung, und ohne diese Frage ginge ein Bündel von vorgestern still bei Apple ein. Gegen die `Cargo.toml` wird ausdrücklich **nicht** geprüft — sie sagt, was ein neuer Bau trüge, und der findet hier nicht statt.
2. **Den Signaturstand gegen die zwei Bedingungen der Beglaubigung:** eine `Authority=`-Zeile, die mit `Developer ID Application` beginnt, und `runtime` in der Merkmalsliste hinter `flags=`. Ein Bündel aus `cargo xtask bundle` fällt an beiden durch, und der Abbruch spart die sinnlose Einreichung. Der gesicherte Zeitstempel wird nicht eigens gefragt: `sign::signieren_gehaertet` setzt `--options runtime` und `--timestamp` in einem Aufruf, die zwei sind nicht einzeln zu haben.

Weder Tag noch Arbeitsbaum werden geprüft. Das ist der Zweck des Wegs und steht als solcher im Modulkopf, im Hilfetext, im Skriptkopf und in `README.md` — samt der Folge, dass ein so beglaubigtes Bündel nicht durch die Vorprüfungen der Auslieferungskette gegangen ist. Gebaut wird nichts: kein Übersetzungslauf, kein `lipo`, keine Montage, keine Signierung; ohne Bündel bricht der Befehl ab und nennt `cargo xtask release`.

## Was wiederverwendet statt neu geschrieben wurde

| Stelle | vorher | jetzt |
|---|---|---|
| `release::beglaubigen`, `werkzeug_pruefen`, `NOTAR_PROFIL_VARIABLE` | privat in `release.rs` | in `beglaubigung.rs`, von beiden Wegen gerufen |
| `bundle::plist_zeichenkette` | privat | `pub(crate)` — ein Leser für `<key>…</key><string>…</string>` |
| `version::versionszahl_pruefen` | privat | `pub(crate)` — eine Vorschrift, wie eine Versionszahl aussieht |
| `sign::DEVELOPER_ID_PRAEFIX` | Identitätssuche im Schlüsselbund | zusätzlich die Erkennung am signierten Bündel |
| `target/KRK.app` | zweimal zusammengesetzt | `bundle::buendelpfad`, mit `#[must_use]`; `Vorlage::zusammensetzen` ruft sie mit |

## Proben

Zehn neue in `beglaubigung.rs`, zwei in `main.rs`. Die drei Signaturanzeigen der Prüfmuster sind **gemessen und nicht erfunden**: wörtlich die Ausgabe von `codesign --display --verbose=2` am ausgelieferten Bündel und an zwei Kopien davon, die für diesen Zweck mit der Entwicklungsidentität beziehungsweise mit der Developer-ID ohne Härtung neu signiert wurden. Der gemischte Fall — Identität stimmt, Merkmal fehlt — ist der Grund, warum die Prüfung zwei Fragen stellt und nicht eine.

Zwei Proben lesen den Quelltext des Moduls: dass es weder die Vorprüfung von Station 1 noch `git` ruft, und dass es über keinen der drei Bauaufrufe verfügt. Die Nadeln stehen als `concat!`, weil die Probe in der Datei liegt, die sie liest — ohne das schlägt sie auf sich selbst an, und genau das ist beim ersten Lauf passiert. Zugleich hat die vorhandene Probe `release::tests::allein_release_fragt_nach_tag_und_arbeitsbaum` angeschlagen, weil ein Doc-Kommentar den Namen der Station ausschrieb; sie hält, dass jene Station an genau einer Stelle des Baums steht, und der Kommentar ist umformuliert.

## Prüfung

`make check` — Rückgabewert 0, alle vier grün (112 Proben in xtask, 0 rot).

**Scharf gefahren ist der Weg nicht**, weil eine Einreichung des Nutzers bei Apple in Bearbeitung ist. Geprüft ist er bis zu dem Punkt, an dem er einreichen würde:

- `./certify-only.sh 0.5.9` gegen das Bündel mit `0.5.5` — bricht mit beiden Zahlen und beiden Handgriffen ab, Rückgabewert 2 über die ganze Kette.
- `KRK_NOTARY_PROFILE= cargo xtask beglaubigen 0.5.5` — beide Prüfungen laufen am echten Bündel grün durch, danach hält die fehlende Profilangabe den Lauf an, vor `ditto` und vor jeder Einreichung.
- `make beglaubigen` ohne `VERSION` und `./certify-only.sh` ohne Argument — je Rückgabewert 2.

## Geänderte Dateien

- `xtask/src/beglaubigung.rs` (neu)
- `certify-only.sh` (neu, ausführbar)
- `xtask/src/release.rs`, `xtask/src/main.rs`, `xtask/src/bundle.rs`, `xtask/src/version.rs`
- `Makefile`, `README.md`

## Offen

`CLAUDE.md` nennt unter „Bauen und prüfen" die vier Schichten der Auslieferungskette und kennt den zweiten Weg noch nicht. Nachzuziehen ist dort ein Satz; der Kurator entscheidet die Form.
