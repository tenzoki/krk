# Editor und Vorschau gehen auf die kleine Systemschrift, und der Löschzielbefund wird zwei Typen

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was verlangt war

Zwei Nutzerantworten vom 260907-1651 in Code bringen.

Erstens: Editor und Vorschau gehen gemeinsam auf die kleine Systemschriftgröße (`260812-1707_*_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-oder-waechst-sie-auf-die-des-editors.md`, Möglichkeit 3). Es bleibt bei **einer** Schriftwahl; die Größe, die der Editor seit der Runde 2 trug, fällt mit, und das ist mitentschieden.

Zweitens: die zwei gegenläufigen Löschprüfungen bekommen zwei Typen, damit die Verdrehung nicht mehr übersetzt (`260818-0249_*_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen.md`, Möglichkeit 2). Der dritte Wert `Unentschieden` bleibt.

## Auftrag 1: die eine Schriftwahl, jetzt auf der kleinen Größe

### Was an `grundschrift` hing, und was nicht

Erhoben mit `grep -rn 'grundschrift\|smallSystemFontSize\|systemFontSize\|userFixedPitchFontOfSize' crates/ --include='*.rs'`. Drei Rufer von `textmerkmale::grundschrift`:

```
appkit/editor.rs    grundschrift_setzen        setFont: an der Editorfläche
appkit/editor.rs    textflaeche_bauen          (neu; setzte vorher selbst)
appkit/vorschau.rs  textanzeige                setFont: an der Vorschaufläche
appkit/textmerkmale.rs  zuruecksetzen          Merkmal über den ganzen Textspeicher
```

`zuruecksetzen` hat keinen eigenen Bereich, sondern läuft aus `anwenden` heraus über dieselben zwei Flächen. **Es ändern sich damit genau zwei Flächen: die Textfläche des Editors und die Textanzeige der Vorschau** — beide in Roh- wie in Formatansicht, weil `LESEZUSCHLAG` ein Zuschlag auf die Grundlage ist und keine eigene Zahl.

Nicht betroffen und geprüft: die Dateiliste (`appkit/tabelle.rs:4476`, eigene Rechnung über `systemFontSize` für die Zeilenhöhe), die Statuszeile, die Bereichsleiste, die Belegungsansicht, der Titelzusatz, die Leiste und der Git-Bereich (alle schon auf `smallSystemFontSize`), sowie der Kopf über der Editorfläche (`editor::kopf_bauen`, `smallSystemFontSize`, dieselbe Form wie die Statuszeile).

### Die Zeilennummernspalte geht ohne Änderung mit

`appkit/nummernspalte.rs` zieht ihre eigene Schrift aus `userFixedPitchFontOfSize(smallSystemFontSize())` — sie stand also schon in der Größe, auf die die Textflächen jetzt gehen. Ihre Breite (`noetige_dicke`) misst die größte Nummer **in dieser eigenen Schrift** und nicht in der der Fläche; die senkrechte Lage jeder Nummer kommt aus den Zeilenkästen des Layoutverwalters und folgt der neuen Schrift von selbst. Keine ihrer Maße hängt an der Schriftgröße der Textfläche, und keine Zeile war anzufassen.

### Was geändert ist

Die Fallunterscheidung über Schriftart und Größe steht jetzt an **einer** Stelle, `textmerkmale::grundmerkmale`. `grundschrift` fragt sie für die Schrift, die neue `grundgroesse` für die Zahl allein, und `anwenden` ruft `grundgroesse(ansicht, art)` statt ein eigenes Literal zu rechnen.

Dass es vorher drei Stellen waren, ist als eigener Datensatz abgelegt:
`260907-1717_*_die-grundschrift-der-zwei-textflaechen-entstand-an-drei-stellen-und-der-modulkopf-sagt-an-einer.md`.

## Auftrag 2: zwei Typen für zwei Richtungen

### Die Erhebung der Prüfungen

Erhoben mit `grep -rn -- '-> Loeschzielbefund' crates/*/src` (und, für die Verbraucher, `grep -rn 'Loeschzielbefund' crates/`). Vier Prüfungen, wie der Datensatz sagt:

| Prüfung | Ort | Ja heißt |
|---|---|---|
| Papierkorb | `krk-ui/src/appkit/papierkorb.rs`, `fuehrt_einen_papierkorb` | Erlaubnis |
| Netzlaufwerk | `krk-ui/src/appkit/volumes.rs`, `liegt_auf_netzlaufwerk` | Warngrund |
| Git-Arbeitsbaum | `krk-core/src/verzeichnis/arbeitsbaum.rs`, `beruehrt_einen_arbeitsbaum` (über `traegt_arbeitsbaum` und `liegt_in_arbeitsbaum`) | Warngrund |
| Umfang | `krk-core/src/verzeichnis/umfang.rs`, `zaehlen` | eigener Typ `Umfang` |

Drei zu eins, nicht zwei zu zwei: die Erlaubnisrichtung trägt genau eine Frage. Die vierte bleibt bei ihrem eigenen Typ, weil ihre entschiedenen Antworten Zahlen tragen und kein Ja.

### Die zwei Typen

`Warnbefund` mit `ist_warnwuerdig()` (`Ja | Unentschieden`) und `oder()`, `Erlaubnisbefund` mit `erlaubt()` (nur `Ja`). Beide mit denselben drei Werten, beide ohne `Ord`, beide ohne Auffangzweig bei den Aufrufern.

**Zwei eigenständige Aufzählungen und kein gemeinsamer Rumpf mit zwei Hüllen.** Ein `Warnbefund(Dreiwertig)` neben einem `Erlaubnisbefund(Dreiwertig)` hielte die Richtung nur so lange, bis jemand `Warnbefund(anderes.0)` schreibt; zwei Nominaltypen ohne jede Umrechnung dazwischen halten sie ohne diese Lücke. Der Preis ist die Wiederholung von drei Varianten, und die ist der Gegenstand des Schnitts und nicht sein Versehen.

`oder` steht allein am `Warnbefund`. Es hat dort seine Rufer (der Aufwärtsgang und die Auswahlschleife in `arbeitsbaum.rs`), und seine Tafel ist aus der **Richtung** abgeleitet: `Ja` saugt auf, weil ein zutreffender Auslöser zutrifft. Am `Erlaubnisbefund` müsste `Nein` aufsaugen, und keine Stelle im Baum fasst zwei Erlaubnisse zusammen — die Verknüpfung wäre Vorrat.

### Der Modulkopf ist kürzer geworden

`loeschzielbefund.rs` hielt die zwei Richtungen in einem eigenen Abschnitt „Die zwei Polaritaeten" in Prosa auseinander. Der ist weg; was er sagte, sagen die Typnamen. An seiner Stelle steht, **warum** es zwei sind, mit dem Fall vom 260817. Dieselbe Kürzung in `arbeitsbaum.rs`, `volumes.rs`, `papierkorb.rs`, `loeschwarnung.rs` und `anwendung.rs`: `grep -rn 'Polaritaet' crates/` liefert nichts mehr.

Der **Modulname** `loeschzielbefund` bleibt, obwohl kein Typ mehr so heißt. Er benennt den Gegenstand; ein Modul `loeschziel` daneben ließe „Ziel" ein drittes Mal im Baum stehen, neben `verweisziel::Verweisziel` und dem `Kopierziel` der Operationsmaschine. Der Modulkopf schreibt das aus.

### Die Probe für die Unübersetzbarkeit

Zwei Doktests am Modulkopf von `loeschzielbefund.rs`, und sie gehören zusammen:

- ein gewöhnlicher, der beide Richtungen **richtig** herum einsetzt und übersetzt;
- ein `compile_fail`, der sich von ihm in genau einem Zeichen unterscheidet — `Erlaubnisbefund::Ja` an einer Stelle, die einen `Warnbefund` erwartet.

Das Paar ist die Antwort auf die Schwäche von `compile_fail`, das jeden Übersetzungsfehler als Erfolg zählt: ein Tippfehler im zweiten Block ließe ihn grün, und dagegen steht der erste, der dieselben Einfuhren und dieselben Signaturen trägt. **Nicht ausdrückbar bleibt** die Verwechslung zweier Fragen **derselben** Richtung — Netzlaufwerk gegen Arbeitsbaum tragen beide `Warnbefund`, und die hält kein Typ. Der Modulkopf sagt das ausdrücklich, damit niemand den Schnitt für mehr hält, als er ist.

Der Übersetzer hat die Zusage während der Umstellung selbst eingelöst: zwei Stellen im Prüfmodul von `loeschwarnung.rs` (Zeilen 1212 und 1225) fielen mit `expected Erlaubnisbefund, found Warnbefund` aus. Dieselbe Datei führte bis dahin **beide** Richtungen unter denselben drei nackten Namen `Ja`, `Nein`, `Unentschieden`.

### Eine Zählprobe ist gefallen, zwei stehen

Die drei Zählproben `hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt` waren die Antwort auf `260817-1419`, den Befund, dass gegen die Verdrehung nur Prosa stand. Sie messen je Datei und je nach Datei etwas anderes:

- **`appkit/papierkorb.rs` — gefallen.** Ihr Gegenstand war genau die Verdrehung, und die hält jetzt der Übersetzer: `ist_warnwuerdig` gibt es am `Erlaubnisbefund` nicht. Eine Probe, die nur noch eine Zeichenfolge zählt, die ohnehin nicht übersetzte, misst nichts. An ihrer Stelle steht ein Kommentar mit dem Grund.
- **`kommandos/loeschwarnung.rs` — bleibt.** Ihr zweiter Grund lebt und ist von keinem Typ zu erledigen: `Ja` und `Unentschieden` führen dort zu **verschiedenen** Warngründen, und eine zusammenfassende Frage kennt den Unterschied nicht mehr.
- **`appkit/volumes.rs` — bleibt.** Sie hält eine Modulgrenze: diese Datei beantwortet den Auslöser und beurteilt ihn nicht.

Die Doc-Kommentare der zwei bleibenden sagen jetzt, welcher ihrer Gründe an den Übersetzer gegangen ist und welcher nicht. Keine Stelle im Baum zählt, wie viele dieser Proben es gibt.

### Eine Fallunterscheidung ist gewandert

`loeschwarnung::vor_der_rueckfrage` schrieb die drei Antworten der Papierkorbfrage selbst aus, weil der Typ die Richtung nicht kannte. Sie fragt jetzt `papierkorb().erlaubt()`; die Vollständigkeit ist nicht verloren, sondern an `Erlaubnisbefund::erlaubt` gezogen, wo ein vierter Wert den Bau anhält. Die Tafel im Doc-Kommentar und die Zwölf-Felder-Probe stehen unverändert.

## Angefasste Dateien

Auftrag 1:

- `crates/krk-ui/src/appkit/textmerkmale.rs` — Modulkopf (Verfügbarkeitsabschnitt: `smallSystemFontSize`, `NSFont.h:76`), `LESEZUSCHLAG`, `anwenden`, `grundschrift`, neu `grundgroesse` und `grundmerkmale`
- `crates/krk-ui/src/appkit/editor.rs` — `textflaeche_bauen` samt Doc-Kommentar
- `crates/krk-ui/src/fenstermodell.rs` — `Bereich::mindestbreite`, Doc-Kommentar: die „rund 40 Zeichen" sind an der alten Größe gemessen und damit heute eine untere Schranke

Auftrag 2:

- `crates/krk-core/src/verzeichnis/loeschzielbefund.rs` — neu geschrieben: zwei Typen, zwei Doktests, Proben
- `crates/krk-core/src/verzeichnis/mod.rs` — Wiederausfuhren und drei Prosastellen
- `crates/krk-core/src/verzeichnis/arbeitsbaum.rs`, `crates/krk-core/tests/arbeitsbaum.rs` — `Warnbefund`
- `crates/krk-core/src/verzeichnis/umfang.rs` — zwei Verweise
- `crates/krk-ui/src/appkit/volumes.rs` — `Warnbefund`, Modulkopf
- `crates/krk-ui/src/appkit/papierkorb.rs` — `Erlaubnisbefund`, Modulkopf, Zählprobe gefallen
- `crates/krk-ui/src/kommandos/loeschwarnung.rs` — beide Typen, `erlaubt()`, Prüfmodul
- `crates/krk-ui/src/appkit/anwendung.rs` — die drei Beschaffungsstellen

## Abnahme

```
cargo build --workspace                                   exit 0
cargo test --workspace                                    exit 0  (25 Probenziele grün,
                                                                   beide Doktests darunter)
cargo clippy --workspace --all-targets -- -D warnings     exit 0
cargo fmt --all --check                                   exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps exit 0
```

**Was die Abnahme nicht deckt.** Auftrag 1 ändert eine sichtbare Fläche, und keines der fünf Kommandos sieht sie. Ob die kleine Schrift im Editor und in der Vorschau taugt, ob die Zeilennummern neben den neuen Zeilenkästen sitzen und ob die Markdown-Überschriften über dem kleineren Fließtext noch als Überschriften lesbar sind, entscheidet das laufende Bündel. Der Entscheidungsdatensatz sagt es selbst: „Die Frage ist am laufenden Bündel zu beurteilen und nicht am Code." Das ist Nutzerarbeit.

## Offen geblieben

Die zwei Entscheidungsdatensätze stehen weiter auf `_a_`. Der Abschlussvermerk verlangt nach den Konventionen den Commit-Hash, und dieser Auftrag committet ausdrücklich nicht; das Umbenennen samt `Implemented:`-Zeile gehört deshalb hinter den Commit.
