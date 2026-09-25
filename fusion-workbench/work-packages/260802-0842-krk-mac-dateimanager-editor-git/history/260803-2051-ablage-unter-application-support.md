# Die Ablage unter Application Support (Plan, Schritt 10)

**Datum:** 260803-2051
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 10. Ablage unter Application Support`, dazu die Antwort auf `### Frage 4`
**Neu angelegt:** `crates/krk-core/src/ablage/{mod.rs,pfade.rs,atomar.rs,sitzung.rs,lesezeichen.rs}`, `crates/krk-core/tests/ablage.rs`
**Geändert:** `crates/krk-core/src/lib.rs` (`pub mod ablage;`), `crates/krk-core/src/verzeichnis/sortierung.rs` (serde-Ableitungen, Nachtrag zur Dateiliste, unten begründet)
**Nicht angefasst:** `crates/krk-ui/`, `crates/krk-bench/`, `xtask/`, `resources/`, die Plandatei, der Spec. Keine `Cargo.toml`: `serde` und `toml` stehen seit S1 in `krk-core`. Kein Commit; das Committen liegt beim Orchestrator.
**Zwei Defektdatensätze angelegt:** unten unter `## Was beim Lesen von Frage 4 aufgefallen ist`
**Stilprofil:** `stilwerk/chat-voice-de.yaml` geladen. Ein Langform-Schreibprofil gibt `fusion-rules` für den `coder` nicht aus.

## Der Zuschnitt

```text
pfade ──> mod (Ablage: laden, sichern, melden) ──> atomar
                 ^                ^
                 │                │
          lesezeichen          sitzung
```

`pfade.rs` löst `~/Library/Application Support/KRK/` auf und legt den Ordner
beim ersten Start an. Die drei Dateien sind eine Aufzählung `Datei` mit
`Datei::ALLE`, damit niemand beim Durchgehen eine vergisst.

`atomar.rs` schreibt über eine Nachbardatei und `rename`. Der Vorgang ist in
zwei öffentliche Schritte geteilt, `vorbereiten` und `Nachbardatei::umbenennen`,
und `schreiben` setzt sie zusammen. Die Teilung ist keine Bequemlichkeit für
die Prüfung, sondern die einzige Art, die Lücke zwischen beiden Systemaufrufen
überhaupt anfahrbar zu machen; siehe `## Wie der Abbruch hergestellt ist`.

`mod.rs` hält die Ablage selbst. Sie ist über den Inhalt allgemein: sie kennt
Pfad, Format und Fehlerbehandlung, nicht die Felder. Damit kann S11 seinen
Belegungstyp hier ablegen, ohne einen zweiten Ablageweg zu bauen; die
Dateiliste von S11 sieht genau das vor.

`sitzung.rs` und `lesezeichen.rs` halten zwei der drei Inhalte. Den dritten,
`keymap.toml`, baut S11.

## Wie sitzung.rs auf S12 zugeschnitten ist

S12 lässt das Fenster- und Tabmodell in diese Datei hineinwachsen und bekommt
keine zweite. Der Aufbau ist deshalb schon der von C1 und C7 und nicht der
einer bloßen Serialisierungshülle:

```text
Sitzung ── aktiv: Fensterseite
        ├─ breiten: Breiten (vier Bereiche, je Option<f64>)
        ├─ sichtbar: Sichtbarkeit (drei Bereiche)
        └─ fenster: [Dateifenster; 2]
                     ├─ aktiver_tab: usize
                     └─ tabs: Vec<Tab>
                               ├─ ordner, auswahl
                               ├─ verstecke_ausgeblendet
                               └─ sortierung: verzeichnis::Sortierung
```

Vier Festlegungen tragen die Erweiterbarkeit:

**Jede Struktur trägt `#[serde(default)]`.** Ein Feld, das S12 hinzufügt, macht
eine ältere `session.toml` nicht ungültig, sondern nimmt seinen
Auslieferungswert an. Unbekannte Felder werden übergangen statt abgelehnt.

**Die Sortierung ist die vorhandene aus `verzeichnis::sortierung`**, nicht eine
zweite daneben. Das ist der Nachtrag zur Dateiliste: `Schluessel`, `Richtung`
und `Sortierung` haben serde-Ableitungen bekommen, mit `rename_all =
"lowercase"`, damit in der TOML-Datei `schluessel = "geaendert"` steht und nicht
`"Geaendert"`. Eine eigene Aufzählung in der Ablage wäre eine zweite Wahrheit
darüber, wonach KRK sortieren kann.

**`[Dateifenster; 2]` und nicht `Vec`.** C1 kennt genau zwei gleichrangige
Dateifenster. Eine `session.toml` mit einer anderen Zahl ist beschädigt und
führt zum Auslieferungszustand; eine Prüfung deckt genau das ab.

**Die Auswahl steht als Name, nicht als Zeilennummer.** Zwischen Beenden und
Neustart kann sich der Ordnerinhalt geändert haben, und eine Zeilennummer zeigte
dann auf einen anderen Eintrag.

Was noch fehlt, fehlt mit Absicht: die Bildlaufposition je Tab, die Markierung
mehrerer Einträge und die Tabs des Vorschaufensters gehören zu Fähigkeiten, die
S12 und S19 erst bauen. Sie kommen als Felder in genau diese Strukturen.

## Die Bündelung des Sitzungszustands

`Sitzungsschreiber` hält die Zusage "höchstens alle 2 s und einmal beim
Beenden". **Ein Schreibweg, zwei Auslöser**, dasselbe Muster, mit dem
`### Frage 3` das Auffrischen löst:

- `vormerken(sitzung, jetzt)` meldet eine Änderung und schreibt, wenn der Takt
  es zulässt.
- `abgleichen(jetzt)` ist der Takt, der einen liegengebliebenen Stand nachträgt,
  wenn keine weitere Änderung mehr kommt. Ohne diesen zweiten Auslöser bliebe
  eine letzte Änderung bis zum Beenden ungeschrieben.
- `beenden(jetzt)` schreibt ohne Rücksicht auf den Takt, und ein zweiter Aufruf
  schreibt nicht noch einmal.

Alle drei laufen in dieselbe private Funktion; einen zweiten Weg auf die Platte
gibt es nicht. Die Zeit kommt als `Instant` von außen und nicht aus
`Instant::now`. Damit ist die Bündelung ohne Warten prüfbar, und die Prüfungen
laufen in Millisekunden statt in Sekunden.

Scheitert ein Schreibvorgang, bleibt der vorgemerkte Stand vorgemerkt und der
Fehler geht als `io::Result` an den Aufrufer. Verschluckt wird nichts.

## Der eine Ausgabeweg der Meldung

Der Plan schreibt die Standardfehlerausgabe vor, und das Bündel hat keine; der
Punkt liegt als offene Entscheidung
`decisions/260803-2025_o_wie-zeigt-krk-dem-nutzer-fehler.md`. Umgesetzt ist
deshalb der Plantext, aber so, dass die Antwort eine Zeile kostet:

1. `Ablage::laden` **gibt** keine Meldung aus, sondern **liefert** sie:
   `Geladen<T> { wert, ersetzung: Option<Ersetzung> }`. Es gibt keinen
   Ladevorgang, der scheitert; es gibt einen, der eine Ersetzung zurückgibt.
2. `Ersetzung` trägt Datei und Grund und formatiert sich über `Display` zu
   einer einzeiligen Meldung, die die Datei benennt. Die mehrzeilige Auszeichnung
   des TOML-Lesers wird dabei auf eine Zeile gepresst, Zeile und Spalte bleiben
   erhalten.
3. `ablage::melden(&Ersetzung)` ist die **einzige** Funktion im Kern, die eine
   Ersetzung an den Nutzer gibt, und die einzige Stelle mit `eprintln!`.
   `Geladen::gemeldet()` ist die Bequemlichkeit, die beides verbindet.

Wechselt der Ausgabeweg, ändert sich der Rumpf von `melden`. Kein Aufrufer und
keine Prüfung hängt daran, weil alle über den Wert gehen.

Der Nebeneffekt für die Abnahme: die Prüfungen können die Meldung als Wert
prüfen, statt eine Ausgabe abzufangen. Genau eine Prüfung geht trotzdem über den
echten Kanal, damit die Kette vollständig belegt ist, siehe unten.

## Wie der Abbruch hergestellt ist

Die zweite Abnahmeforderung lautet: ein Abbruch zwischen Schreiben und
Umbenennen lässt die alte Datei unverändert. Ein Abbruch lässt sich nicht durch
Abwarten herstellen, und ein nachgespielter im laufenden Testprozess trüge nur
den Namen des Falls.

**Der Prüfling stirbt wirklich.** Die Prüfung startet dieselbe Testdatei ein
zweites Mal über `std::env::current_exe()`, mit
`--exact --ignored --nocapture <name>` und einer Umgebungsvariablen als Auftrag.
Das Kind ruft `atomar::vorbereiten` auf, prüft, dass die Nachbardatei steht, und
ruft dann `std::process::abort()`. `abort` führt kein `Drop` aus und lässt dem
Prozess keine Gelegenheit aufzuräumen; das ist der Absturz, den die Zusage
meint. Die Kindprobe trägt `#[ignore]`, damit ein gewöhnlicher Lauf sie nicht
anfasst, und kehrt ohne ihre Umgebungsvariable sofort zurück.

Der Elternteil prüft danach vier Dinge, und die ersten beiden sind es, die die
Prüfung von einer Namensträgerin unterscheiden:

1. **Das Kind ist an einem Signal gestorben**, `ExitStatus::signal() == Some(6)`,
   also `SIGABRT`, und nicht ordentlich zurückgekehrt.
2. **Es war über das Schreiben hinaus**: die Nachbardatei liegt da und trägt den
   Inhalt des Kindes. Ohne diese Zusicherung bestünde die Prüfung auch dann,
   wenn das Kind schon vor dem Schreiben gestorben wäre.
3. Das Ziel ist Byte für Byte das alte, und über die Ablage gelesen ergibt es
   die alte Sitzung ohne Meldung.
4. Die liegengebliebene Nachbardatei stört den nächsten Schreibvorgang nicht und
   verschwindet mit ihm.

**Zwei Gegenproben belegen, dass die Prüfung greift.** Beide sind gelaufen und
zurückgenommen:

| Eingriff | Erwartet | Beobachtet |
|---|---|---|
| `vorbereiten` schreibt unmittelbar auf das Ziel statt auf die Nachbardatei | die Prüfung scheitert | gescheitert, dazu sieben weitere |
| das Kind ruft `abort` **vor** `vorbereiten` | die Prüfung scheitert an Punkt 2 | gescheitert mit "das Kind ist gestorben, bevor es geschrieben hat" |

Dasselbe Verfahren trägt eine zweite Kindprobe: sie lädt eine kaputte
`session.toml` über `Geladen::gemeldet()`, und der Elternteil liest die
Standardfehlerausgabe des Kindes mit und sucht darin die Zeile. Damit ist nicht
nur der Meldungswert belegt, sondern der Kanal, den der Plan heute vorschreibt.

## Abnahme

`cargo test -p krk-core --test ablage` beendet mit 0: 16 Prüfungen laufen, zwei
sind die Kindproben und werden von ihren Elternteilen gestartet.

Die drei geforderten Punkte:

| Forderung | Prüfung |
|---|---|
| Schreiben und Wiedereinlesen aller drei Dateien ergibt denselben Inhalt | `alle_drei_dateien_ueberstehen_schreiben_und_wiedereinlesen` |
| ein Abbruch zwischen Schreiben und Umbenennen lässt die alte Datei unverändert | `ein_abbruch_zwischen_schreiben_und_umbenennen_laesst_die_alte_datei_unveraendert` samt Kindprobe |
| eine syntaktisch kaputte Datei führt zum Auslieferungszustand und zu einer Meldung, nicht zu einem Abbruch | `eine_kaputte_datei_fuehrt_zum_auslieferungszustand_und_zu_einer_meldung`, `die_ersetzung_erscheint_auf_der_standardfehlerausgabe` |

**Zur ersten Forderung: alle drei heißt alle drei.** Die Belegung aus
`keymap.toml` gehört S11; damit die Zusage trotzdem an drei Dateien und nicht an
zweien geprüft wird, läuft der dritte Weg mit einem Stellvertretertyp aus der
Testdatei, über dieselbe Ablage und denselben Pfad. Geprüft ist damit der
Mechanismus, nicht der spätere Inhalt.

Dazu neun weitere Prüfungen: der Ort unter `Application Support` samt der drei
Dateinamen, der erste Start mit Anlage des Ordners, die Wiederholbarkeit des
Starts, der Auslieferungszustand der Sitzung gegen C1, die Lesbarkeit der
geschriebenen TOML-Datei, gültiges TOML mit falscher Gestalt, eine nicht lesbare
Datei, der Takt von zwei Sekunden in drei Prüfungen und das atomare Schreiben in
zwei.

Die vier üblichen Kommandos beenden mit 0: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets`.

`grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-core/src`
nennt weiterhin genau eine Datei, `verzeichnis/sys.rs`. Die neuen Module
brauchen keine Ausnahme.

## Was beim Lesen von Frage 4 aufgefallen ist

Zwei Punkte, beide als Defektdatensatz abgelegt statt nebenbei behoben:

- `issues/260803-2051_o_ersetzt-in-frage-4-und-s10-laesst-offen-ob-die-kaputte-datei-ueberschrieben-wird.md`
  — "durch den Auslieferungszustand ersetzt" lässt offen, ob nur der geladene
  Zustand oder auch die Datei auf der Platte gemeint ist. Für `keymap.toml`, die
  der Nutzer laut `### Frage 4` von Hand ändern soll, ist der Unterschied
  teuer: unter der zweiten Lesart kostet ein Tippfehler seine ganze Belegung.
  Die Umsetzung folgt der ersten Lesart und hält das im Modulkopf und in einer
  Prüfung fest.
- `issues/260803-2051_o_die-keymap-zeile-der-tabelle-in-frage-4-ist-unlesbar.md`
  — die Tabellenzelle zu `keymap.toml` ist grammatisch defekt und liest sich
  beim ersten Mal umgekehrt. Der Sachverhalt steht in S11 richtig; allein die
  Formulierung trägt ihn nicht.

## Was diese Umsetzung nicht getan hat

Keine Zeile in `crates/krk-ui/`: S10 ist reine `krk-core`-Arbeit, und die
Sitzungswiederherstellung hängt S12 ein. `resources/default-keymap.toml` ist
nicht gelesen worden; das tut S11. Der `[DONE]`-Vermerk im Plan ist nicht
gesetzt, das liegt beim Auslöser dieser Aufgabe.

**Eine Doppelung, die stehen bleibt.** `tests/ablage.rs` bringt einen eigenen
`Pruefordner` mit, wie `tests/verzeichnis.rs` einen hat. Integrationsprüfungen
sind in Rust je eine eigene Kiste; ein gemeinsames Modul dafür verlangte eine
Datei, die keine Dateiliste nennt, und den Umbau einer bereits abgenommenen
Prüfdatei. Der hiesige Pruefordner trägt nur, was S10 braucht, und ist gut
zwanzig Zeilen lang.
