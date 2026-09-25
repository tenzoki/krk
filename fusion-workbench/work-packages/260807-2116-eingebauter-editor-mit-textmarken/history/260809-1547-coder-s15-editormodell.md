# S15: `editormodell` — der Stand des Editors ohne AppKit

**Status:** Complete
**Agent:** coder
**Datum:** 260809-1547
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 15 (Phase C)

---

## Was entstanden ist

`crates/krk-ui/src/editormodell.rs` (neu, 1180 Zeilen mit Proben) und die
Einbindung in `crates/krk-ui/src/main.rs`.

Das Modell hält sieben Sachen und rechnet keine davon selbst nach:

| Feld | Was es hält | Wer es rechnet |
|---|---|---|
| `pfad` | die gehaltene Datei, `None` ohne | — |
| `stand` | der Text als eine `String` | `krk_core::text::datei::oeffnen` |
| `abweichung` | ob ungesicherter Stand offensteht (C4) | — |
| `ansicht` | roh oder Format (C3) | — |
| `typ` | ob die Datei die Markdown-Zutaten verlangt (C3) | aus dem Pfad |
| `suchlauf` | Suchtext, Trefferliste, angesteuerter Treffer (C5) | `krk_core::text::suche` |
| `stempel` | Änderungszeit und Größe beim Öffnen oder Sichern (C4) | `stat(2)` |

Dazu der `Ladevorgang`: ein Faden je Anfrage, ein `sync_channel(1)`, die
überholte Antwort verfällt still, weil ihr Empfänger fällt. Derselbe Zuschnitt
wie `Ladevorgang` in `vorschaumodell.rs`, und aus demselben Grund ohne
Generationsprüfung.

## Der Beleg, dass ein Ansichtswechsel nichts verliert

Zwei Belege, ein struktureller und ein gemessener.

**Strukturell:** `Editormodell` trägt genau **ein** `String`-Feld. `Ansicht`
steht daneben und sagt allein, wie die Textfläche denselben Stand darstellt.
`ansicht_umschalten` fasst weder `stand` noch `abweichung` noch `suchlauf` an —
es ist eine Zuweisung auf ein `Copy`-Feld. Es gibt keinen zweiten Textbestand,
in den etwas verlorengehen könnte; das zehnte Abnahmekriterium von C3 ist damit
eine Eigenschaft des Typs und keine Zusage der Sorgfalt.

**Gemessen:** `ein_ansichtswechsel_verliert_keinen_ungesicherten_stand` öffnet
eine Datei, tippt einen ungesicherten Absatz hinein, schaltet auf die
Rohansicht und wieder zurück und prüft nach jedem Wechsel Stand,
Abweichungsmarke und gehaltene Datei. Dazu
`die_ansichtswahl_bleibt_ueber_einen_dateiwechsel_stehen`: die Wahl gehört nach
C3 nicht der Datei, ein Dateiwechsel setzt sie nicht zurück.

## Die vier Abnahmekommandos

Alle vier mit `export PATH="$HOME/.cargo/bin:$PATH"` gefahren, alle vier grün:

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0, keine Warnung |
| `cargo test --workspace` | 0, fünfzehn Testziele, kein Fehlschlag; `krk-ui` steht bei 219 Proben, davon 19 neu |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo fmt --all --check` | 0 |

Dazu das zweite Abnahmekriterium des Schrittes: `grep -c 'objc2'
crates/krk-ui/src/editormodell.rs` liefert **0**.

## Die neunzehn Proben

Die beiden namentlich geforderten:

- `die_abweichung_kommt_mit_der_aenderung_und_geht_mit_dem_sichern` — frisch
  geöffnet keine Abweichung, nach `bearbeiten` eine, nach `sichern` wieder
  keine, und die Datei auf der Platte trägt danach den neuen Inhalt.
- `ein_zweiter_ladevorgang_laesst_den_ersten_verfallen` — zwei `oeffnen` ohne
  Einzug dazwischen; danach steht die zweite Datei, `laedt_noch` ist falsch, und
  ein weiterer `einziehen` liefert nichts nach. Die Probe hängt an keiner
  Wettlage: der erste Empfänger fällt in dem Augenblick, in dem `oeffnen` den
  zweiten Vorgang einsetzt, und danach **kann** die Meldung des ersten Fadens
  nicht mehr ankommen, gleichgültig wie schnell er war.

Die übrigen siebzehn decken ab: der leere Editor; der Ansichtswechsel und die
Ansichtswahl über einen Dateiwechsel; eine abgewiesene Datei, die den
gehaltenen Stand stehen lässt (C2); ein gescheitertes Sichern, das den Grund
nennt und den Stand nicht wegwirft (C4); ein Editor ohne Datei, der nichts zu
sichern hat; der Stempel gegen eine Änderung von außen und gegen eine
verschwundene Datei (C4); das Schließen; die Suche mit Zählung, Umlauf,
Rückwärtslauf und Beenden (C5); eine Suche ohne Treffer; das Ersetzen einzeln
und im Zug, darunter der Fall, in dem der Ersatztext den Suchtext enthält; eine
Bearbeitung, die den Suchlauf beendet; die Typerkennung aus der Endung.

## Sechs Abweichungen von der Schrittbeschreibung

Sie stehen ausführlich im Plan unter `#### 15.` als Umsetzungsvermerk. Kurz:

1. **`Dateityp` trägt zwei Werte und nicht die drei aus C3.** "Code" heißt nach
   dem sechsten Abnahmekriterium von C3 genau "die eingebundene Kiste kennt eine
   Sprache dafür" und ist aus dem Pfad allein nicht entscheidbar. Das Modell
   stellt deshalb nur die entscheidbare Frage, `Markdown` gegen `Sonstiges`.
2. **Der Modulkopf nennt die Bindungskiste nicht beim Namen**, weil das
   Abnahmekriterium dieses Schrittes und das von S16 den Namen in genau dieser
   Datei zählen und 0 erwarten. Die vier Nachbarmodelle liefern für denselben
   `grep` je 1, alle vier für den Satz, der die Grenze behauptet.
3. **`#![allow(dead_code)]` am Modulkopf**, mit S16 als benanntem ablösendem
   Schritt. Ohne die Zeile stünde der Arbeitsbereich zwischen S15 und S16 rot,
   weil `make lint` mit `-D warnings` fährt.
4. **Der ungesicherte Stand ist eine Marke und kein Vergleich.** Preis benannt.
5. **Vier Methoden greifen S25, S31 und S37 vor** (`sichern`,
   `fremd_geaendert`, `treffer_ersetzen`, `alle_treffer_ersetzen`), weil sie
   den Zustand halten, den dieser Schritt trägt.
6. **`main.rs` ist an zwei Stellen über die Einbindungszeile hinaus
   mitgezogen**: "vier Bereiche" war seit S13 überholt, und `leistenmodell`
   fehlte in der Aufzählung.

## Was der nächste Schritt vorfindet

S16 baut `appkit/editor.rs` und leiht sich dieses Modell in einem `RefCell`. Es
findet vor:

- `stand()` für das einmalige Einsetzen in den `NSTextStorage` und
  `bearbeiten()` für den Rückweg aus `textDidChange:`.
- `einziehen()` als Abholstelle für den Takt von 1/60 s, wie
  `appkit/vorschau.rs` es für die Vorschau fährt. Der Rückgabewert
  `Ladeausgang` sagt der Ansicht, ob sie den Stand neu setzt oder eine
  Abweisung in die Statuszeile schreibt.
- `ansicht()` und `typ()` für die beiden Darstellungen aus S33.
- `hat_ungesicherten_stand()` als die eine Frage vor den vier Anlässen aus C4.

Die Zeile `#![allow(dead_code)]` fällt mit S16; sie sagt es an Ort und Stelle.

## Angelegte Datensätze

Keine. Kein Defekt und keine offene Frage sind bei diesem Schritt angefallen;
die sechs Abweichungen sind Umsetzungsentscheidungen mit geschriebener
Begründung am Code und im Plan, keine offenen Punkte.

## Nicht angefasst

`crates/krk-ui/src/appkit/menue.rs` war für einen parallel laufenden Schritt
reserviert und ist unberührt.
