# K7 — drei Nutzerantworten vom 260907-1210 (Punkte 11, 14, 15)

**Agent:** coder
**Datum:** 260907-1226
**Status:** Complete
**HEAD bei Beginn:** `cf232e2`

---

## Auftrag 1 — der Doppelklick auf einen Ordner ohne Leserecht meldet

Datensatz: `260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`

### Was erhoben wurde

Der Auftrag hat verlangt, **zuerst** zu prüfen, ob der Verzeichnisleser die
Auskunft schon liefert, statt einen Systemaufruf hinzuzufügen. Er tut es.

Die Kette: `krk_core::verzeichnis::sys::Schwungleser::oeffnen` geht über
`ohne_warten_oeffnen` und scheitert bei einem Ordner ohne Leserecht mit
`EACCES`; `lesefaden` meldet `Abschluss::Fehler(io::Error)` über den Kanal;
`krk-ui`s `tabs::lesemeldungen_einziehen` (`crates/krk-ui/src/tabs.rs:1565`)
schreibt daraus `tab.meldung`; `appkit::tabelle` gibt sie als
`Quellen::tabmeldung` in die Statuszeile. Der Zweig steht seit `537fda5` (S12)
im Baum.

Gemessen und nicht gelesen: eine Wegwerfprobe an `Tabliste::ordner_setzen`
gegen einen Ordner mit Modus `0o000` lieferte nach **einem** Einzugstakt
`"…/gesperrt ließ sich nicht vollständig lesen: Permission denied (os error
13)"`.

**Damit war die Prämisse des Datensatzes falsch, und zwar schon zum Zeitpunkt
seiner Abfassung.** Die vom Nutzer gewählte Antwort ist erfüllt, und der
mitentschiedene Preis — ein `stat`/`open` auf jedem Ordnereinstieg, mit L3 und
L10 daran — ist nicht zu zahlen. Ein Abnahmelauf aus diesem Anlass entfällt.

### Was geändert wurde

- `crates/krk-core/src/verzeichnis/verweisziel.rs:87-105` — der letzte Abschnitt
  des Modulkopfs behauptete „Ein Doppelklick auf einen gewoehnlichen
  `Typ::Ordner` ohne Leserecht ist heute wortlos". Berichtigt: er nennt jetzt
  den Weg über den Leser, die Probe, den Nutzerentscheid und den Umstand, dass
  L3 und L10 unberührt bleiben.
- `crates/krk-ui/src/appkit/tabelle.rs:2493-2512` — neuer Abschnitt an
  `in_zeile_einsteigen`: das Leserecht wird hier nicht geprüft und trotzdem
  gemeldet, mit dem Satz, dass eine Prüfung an dieser Stelle die zweite
  Wahrheit wäre und einen Systemaufruf je Einstieg kostete.
- `crates/krk-ui/src/kommandos/pfadeingabe.rs:69-81` — der Kommentar an der
  `read_dir`-Prüfung sagt jetzt, dass der Doppelklick ebenfalls meldet und die
  Prüfung hier den Preis dafür ist, dass der Pfadsprung **nicht wechselt**.
- `crates/krk-ui/src/tabs.rs:2771-2830` — neue Probe
  `ein_ordner_ohne_leserecht_meldet_sich_aus_dem_lesevorgang`. Sie hält das
  Verhalten, das der Übersetzer nicht hält: wer den Zweig `Abschluss::Fehler`
  für entbehrlich hält, macht sie rot.

## Auftrag 2 — Kommentare nennen den Rang einer Statuszeilen-Meldung nicht mehr als Zahl

Datensatz: `260811-1230_*_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`

### Was erhoben wurde

Erhebungskommando (ein Dreizeilenfenster über jede Kommentarzeile unter
`crates/`, damit ein Zeilenumbruch keine Stelle verdeckt; Ziffern und
ausgeschriebene Zahlen):

```sh
python3 - <<'PY'
import re, pathlib
wort = r'(?:ein(?:en|em|er|es|e)?|zwei|drei|vier|fuenf|sechs|sieben|acht|neun|zehn|[0-9]+)'
eng  = re.compile(r'(Rang|Raenge|Ranges|Rangs)', re.I)
for pfad in sorted(pathlib.Path('crates').rglob('*.rs')):
    zeilen = pfad.read_text().splitlines()
    for i, z in enumerate(zeilen, 1):
        s = z.strip()
        if not (s.startswith('//') or s.startswith('*')):
            continue
        fenster = ' '.join(x.strip().lstrip('/!').strip() for x in zeilen[i-1:i+2])
        if not eng.search(fenster):
            continue
        t = re.search(r'(?:Rang|Rangs|Raenge)\s+(?:von\s+)?' + wort + r'\b|'
                      + wort + r'\s+(?:Raenge|Rang)\b', fenster, re.I)
        if t:
            print(f'{pfad}:{i}: {t.group(0)!r}')
PY
```

**Gefunden: 22 Absätze in vier Dateien**, nicht „rund neun Absätze in drei
Dateien" wie der Datensatz schätzte. Die vierte Datei ist `crates/krk-ui/src/tabs.rs`.

Aussortiert wurden drei Nachbarschaften mit demselben Wort und anderer
Bedeutung: die Warngrund-Rangfolge in `kommandos/loeschwarnung.rs`, die drei
Ränge des Abbruchbefehls (`appkit/anwendung.rs`, `kommandos/operationen.rs`) und
ein „Strang 1" in `anwendung.rs`.

Zwei der gefundenen Zahlen waren **falsch**: `tabs.rs` sagte „Rang 5 von 6"
(seit der Runde 20 sind es sieben), und eine Probe in `statuszeile.rs` sagte
„ein Rang von sechs".

### Was geändert wurde

- `crates/krk-ui/src/appkit/statuszeile.rs:225-240` — **die Regel steht einmal**,
  als neuer Abschnitt „Kein Kommentar im Baum nennt einen Rang als Zahl" am
  Doc-Kommentar von `Rang`. Sie nennt den Nutzerentscheid, die Übersetzung
  („Rang 1" heißt `Rang::Befehlsantwort`), die Ausnahme für die Verdrängung
  (ein Satz statt einer Ziffer) und den gemessenen Grund.
- `crates/krk-ui/src/appkit/statuszeile.rs:341-346` — `Quellen`: „die vier
  oberen / die zwei unteren" durch die Namen ersetzt.
- `crates/krk-ui/src/appkit/statuszeile.rs:355-370` — die sechs Feldkommentare
  „Rang 1:" … „Rang 6:" tragen jetzt die Variante als Doc-Link. Die alte
  Nummerierung war zudem irreführend: sie zählte allein die
  Dateifenster-Ränge, sodass „Rang 6" den `Rang::Markierungsstand` meinte, der
  in `Rang::ALLE` der siebte ist.
- `crates/krk-ui/src/appkit/statuszeile.rs:648-656` — die Verdrängungsstelle
  („steht auf Rang 3, wartet … (Rang 1) ab") ist ein Satz mit Namen geworden.
  Dabei ist „Jede der acht Quellen mit eigenem Feld" zu „Jede Quelle mit
  eigenem Feld" geworden: dieselbe Aufteilung, dieselbe Veraltung.
- `crates/krk-ui/src/appkit/statuszeile.rs:1489` — „ein Rang von sechs" →
  „einer der Raenge aus `Rang::ALLE`".
- `crates/krk-ui/src/appkit/anwendung.rs` — neun Absätze:
  `:1543`, `:1570`, `:2988`, `:4049`, `:5515`, `:7375`, `:7699`, `:7744`,
  `:8322`. Dazu `:8321` „Eine sechste Quelle … entsteht dabei nicht" → „Eine
  weitere Quelle".
- `crates/krk-ui/src/appkit/tabelle.rs` — acht Absätze: `:595`, `:600`, `:614`,
  `:1009`, `:1035`, `:3630`, `:3838`, `:3912`.
- `crates/krk-ui/src/tabs.rs:817-824` — die Stelle mit „Rang 5 von 6". Die
  Rangfolge steht jetzt als Satz mit Namen, und wer über dem Filterstand steht,
  ist ausgeschrieben statt gezählt.

Die Doc-Links sind in `anwendung.rs` als
`[`Rang::X`](statuszeile::Rang::X)` geschrieben, weil dort allein das Modul und
nicht der Typ eingeführt ist; `RUSTDOCFLAGS="-D warnings" cargo doc` prüft sie.

## Auftrag 3 — die Begründung des Auffrischungsaufschubs nachziehen

Datensatz: `260807-0010_*_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`

### Was erhoben wurde

Der Vorlauf meldete, die Begründung sei „nachgezogen, aber anders formuliert".
Geprüft: **halb.** Der Doc-Kommentar an `schiebt_auffrischung_auf` nannte die
Schließung an der Lesestelle vom 260807 und leitete daraus ab, die Einordnung
beantworte nur noch, „ob eine Auffrischung waehrend des Vorgangs ueberhaupt
lohnt". Das ist schwächer als der Nutzerentscheid: der Aufschub fängt weiterhin
eine **Fehlfunktion** ab und nicht bloß eine unlohnende Arbeit.

Zusätzlich gefunden: der Doc-Kommentar an `auffrischung_aufgeschoben` sagte im
Präsens „ein Lesevorgang leert sein Ordnermodell, bevor er den ersten Stapel
anhaengt" — seit dem 260807 falsch.

### Was geändert wurde

- `crates/krk-ui/src/auffrischung.rs:322-350` — neuer Abschnitt „Warum der
  Aufschub trotzdem bleibt" an `schiebt_auffrischung_auf`, mit den zwei
  gerechneten Fällen (300 ms Sammelverzögerung gegen 43 ms je 10.000 und 492 ms
  je 100.000 Einträge; bis rund 60.000 unruhig, darüber für die ganze Laufzeit
  unsortiert und unvollständig), dem Nutzerentscheid und dem Satz, dass jede
  weitere Vorgangsart hier einzuordnen ist und der Übersetzer das erzwingt —
  samt der Einschränkung, dass er die **Proben** nicht erzwingt.
- `crates/krk-ui/src/auffrischung.rs:305-306` — „bevor er seinen ersten Stapel
  angehaengt hat" → „bevor der vorige fertig ist": die alte Formulierung
  beschrieb den Mechanismus von vor dem 260807.
- `crates/krk-ui/src/auffrischung.rs:394-406` — die Präsens-Stelle an
  `auffrischung_aufgeschoben` ins Präteritum gesetzt, mit einem Absatz, der
  sagt, was der Aufschub **heute** abfängt.

---

## Abgelegte Datensätze

- `260907-1226_*_weist-der-doppelklick-auf-einen-ordner-ohne-leserecht-ab-oder-geht-er-hinein-und-meldet-danach.md`
  (Entscheidung, offen) — die Ungleichheit, die nach der Antwort vom 260907-1210
  bleibt: der Pfadsprung weist ab, der Doppelklick geht hinein und meldet
  danach. Drei Möglichkeiten, eine davon spart einen Systemaufruf.
- `260907-1226_*_zwei-prosastellen-nennen-den-doppelklick-auf-einen-ordner-ohne-leserecht-wortlos-der-baum-meldet-seit-s12.md`
  (Defekt, offen) — der Modulkopf von `verweisziel.rs` und der Datensatz
  `260815-1749_*` haben beide behauptet, der Doppelklick sei wortlos. Der
  Modulkopf ist berichtigt; der Datensatz bleibt als Aufzeichnung stehen, sein
  Nachsatz nennt den Irrtum.

## Nachgezogene Entscheidungsdatensätze

Alle drei von `_a_` auf `_i_`, je mit `Implemented:`-Nachsatz:

- `260807-0010_*_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`
  (im Circle der Runde 1)
- `260811-1230_*_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`
  (im Circle der Runde 3)
- `260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`
  (im gemeinsamen Speicher)

Die Nachsätze nennen Dateipfade statt eines Commit-Hashs, weil der Orchestrator
committet und der Hash zur Schreibzeit nicht vorlag. Beide Formen stehen im
Bestand.

## Abnahme

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0, 24 Testziele grün, 0 fehlgeschlagen |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo fmt --all --check` | exit 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | exit 0 |

Nicht committet; das tut der Orchestrator.

## Bahnentreue

Weder `crates/krk-ui/src/appkit/blaetter/` noch `crates/krk-core/tests/belegung.rs`
noch `crates/krk-ui/src/belegungsausgabe.rs` angefasst.
