# F1 — Der Durchlauf als eigenes Modul neben dem Leser

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang F, Schritt F1
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C3.1, C3.4, C3.8, C3.9, C3.10, C3.13
**Verification:** `make check` — exit 0

## Was entstanden ist

`crates/krk-core/src/verzeichnis/durchlauf.rs`, ein Modul neben dem Leser und nicht unter
ihm. Es beantwortet je Ordner des angezeigten Ordners genau eine Frage, nämlich die des
letzten Zweigs im Prüfschritt des Ordnermodells: liegt unter ihm ein Treffer?

```text
Durchlauf::starten(auftraege, ordner, filter_klein, generation)
   │
   ├── Faden "krk-durchlauf-<n>"      Auftrag { index: u32, name: String }
   │      │
   │      └─> sync_channel(1024) ──> befunde()
   │                                 Befundmeldung { index: u32, treffer: bool }
   └── Arc<AtomicBool>  <── abbrechen(), und Drop setzt es
```

Das zweite Bild des Spec steht Zweig für Zweig in `unterbaum_entscheiden`. Die drei
Ausgänge sind drei `return`: `Some(true)` beim ersten Namenstreffer, `None` an der
Abbruchgrenze, `Some(false)` an den drei Stellen des negativen Befunds.

## Die fünf Zusagen, Zeile für Zeile

**Der erste Treffer beendet den Auftrag.** Der `return Some(true)` steht in der
Eintragsschleife, nicht am Ende einer Ebene. Der Ebenenstapel fällt mit ihm, und mit ihm
jeder offene `Schwungleser`; der Rest unter dem Treffer wird nie gelesen.

**Der negative Befund hat drei Quellen, und alle drei sind derselbe Rückgabewert.** Die
symbolische Verknüpfung steht am Kopf der Funktion, das gescheiterte Öffnen unmittelbar
danach, der abgeschrittene Unterbaum ohne Fund hinter der Schleife. Keiner der drei meldet
einen Fehler, keiner hält den Durchlauf an. Von „noch nicht entschieden" trennt sie, dass
sie überhaupt eine Meldung schicken: ein abgebrochener Durchlauf schickt keine.

**Kein mitgeführter Zustand über besuchte Ordner.** Es gibt keine Menge und keine Liste
besuchter Pfade. Sie hätte nichts zu verhindern, weil in keine Verknüpfung abgestiegen
wird.

**Das Abbruchkennzeichen wird an der Stapelgrenze gelesen.** Genau ein
`abbruch.load(...)` steht im Modul, und es steht zwischen „der Stapel ist aufgebraucht"
und „den nächsten holen". Beim Absteigen wird es nicht gelesen — der Abstieg erreicht die
Grenze allerdings sofort, weil eine frisch aufgelegte Ebene mit leerem Stapel beginnt.

**Keine Tiefengrenze und kein Deckel.** Das Modul erklärt keine einzige Konstante; die
Stapelgröße kommt aus `leser::STAPELGROESSE`. Der Abstieg läuft über `Vec<Ebene>` und nicht
über die Rekursion des Fadens.

## Zwei Entscheidungen, die eine Begründung brauchen

**Der Stapel ist nicht der Schwung.** Ein Schwung des `Schwungleser` ist so groß, wie der
Antwortpuffer trägt, also oft mehrere tausend Einträge; ein Stapel ist 1.024. Hinge die
Abbruchgrenze am Schwung, passierte ein Ordner mit fünfzigtausend Einträgen sie nur eine
Handvoll Mal statt neunundvierzig Mal. Jede `Ebene` führt deshalb neben dem Stapel einen
`vorrat`, in den die Schwünge laufen und aus dem die Stapel geschnitten werden — dieselbe
Bauart wie `gesammelt`/`split_off` in `leser::lesen_und_senden`.

**Die Verknüpfungsfrage geht über `lstat(2)` und das ist keine Rückkehr zur Pfadprüfung.**
Der Defekt `260809-1652` hat die Typprüfung an den Deskriptor gezogen, weil eine Prüfung am
Pfad ein Fenster zwischen Prüfung und Öffnen ließ und an einer benannten Röhre blockierte.
Hier ist die Frage eine andere: „ist der Pfad **selbst** eine Verknüpfung?" ist an einem
Deskriptor gar nicht zu stellen, denn wer geöffnet hat, ist ihr schon gefolgt. `lstat(2)`
öffnet nichts und kann deshalb auch nicht blockieren. Der Kommentar an
`unterbaum_entscheiden` schreibt das aus, damit die Stelle nicht als Rückfall gelesen wird.
Für die Einträge **innerhalb** eines Ordners fällt keine zusätzliche Frage an: dort liefert
der Leser den Typ mit.

## Was geprüft ist und was nicht

| Kriterium | Probe | offen |
|---|---|---|
| C3.1 keine zweite Lesemechanik | `der_durchlauf_liest_ueber_den_schwungleser_und_setzt_keine_grenze` | Zahl der Fäden und Kanäle fällt erst mit F2 an |
| C3.4 Abbruch ohne Absteigen | `der_abbruch_greift_in_einem_ordner_ohne_unterordner` | „an **jeder** Stapelgrenze" statt „einmal je Auftrag" ist an dieser Schnittstelle nicht messbar |
| C3.8 keine Tiefengrenze | `der_durchlauf_kennt_keine_tiefengrenze` (200 Ebenen), Konstantenzählung am Baum | — |
| C3.9 Verknüpfung | `eine_verknuepfung_auf_einen_ordner_meldet_kein_treffer` | — |
| C3.10 nicht zu öffnen | `ein_nicht_lesbarer_ordner_gilt_als_kein_treffer` | — |
| C3.13 drei Wege, ein Befund je Auftrag | `jeder_auftrag_bekommt_genau_einen_befund` | — |

**C3.4 ist zur Hälfte gemessen und zur Hälfte nicht, und der Grund ist die Schnittstelle.**
Der Durchlauf meldet je Auftrag genau einmal; zwischen zwei Stapeln desselben Ordners sagt
er nichts. Eine Probe kann sich deshalb nicht mit dem Fortschritt **innerhalb** eines
Ordners verabreden, und damit ist eine Fassung, die das Kennzeichen einmal je Auftrag
läse, von der gebauten nicht zu unterscheiden. Was die Probe misst, ist die Aussage, für
die das Kriterium den Prüfordner ohne Unterordner ausdrücklich verlangt: der Abbruch hängt
nicht am Absteigen. Ein Ordner mit 5.000 Einträgen und keinem Unterordner bleibt
unentschieden, statt vollständig durchzulaufen und `treffer: false` zu melden.

## Zwei Grenzen, die das Bild nicht nennt

Beide sind Folgen des Bildes und keine Abweichungen von ihm; sie stehen hier, weil sie erst
beim Bauen sichtbar werden.

**Ein offener Deskriptor je Ebene.** Die Kante `war es der Ordner des angezeigten Ordners?
— nein → weiter im übergeordneten Ordner` verlangt, dass der übergeordnete Ordner nach der
Rückkehr weitergelesen wird; also bleibt sein `Schwungleser` offen. Ein 300 Ebenen tiefer
Baum hält 300 Deskriptoren. Auf diesem Gerät ist die weiche Grenze 1.048.576 und
`kern.maxfilesperproc` 61.440, also unkritisch; bei einer engeren Grenze liefe ein tiefer
Baum in denselben Zweig wie ein nicht zu öffnender Ordner und meldete „kein Treffer".

**Ein Unterbaum tiefer als `PATH_MAX`.** Der Abstieg hängt den Namen an den vollen Pfad an
und öffnet damit. Übersteigt der zusammengesetzte Pfad 1.024 Bytes, scheitert das Öffnen,
und der Zweig „lässt er sich öffnen? — nein" greift. Die Tiefenprobe steht deshalb bei 200
Ebenen und nicht bei 2.000.

## Was im Baum doppelt steht, und für wie lange

`traegt_die_folge` in `durchlauf.rs` und der Vergleich in `Ordnermodell::sichtbar` sind
dieselbe Regel an zwei Stellen. Schritt A2 des Plans zieht den Vergleich als reine Funktion
nach `verzeichnis::filter`; die Aufrufstelle im Durchlauf zieht dann mit. Der Doc-Kommentar
an der Funktion nennt A2 beim Namen, damit die zweite Fassung nicht als Absicht gelesen
wird.

## Dateien

- `crates/krk-core/src/verzeichnis/durchlauf.rs` — neu: `Auftrag`, `Befundmeldung`,
  `Durchlauf`, `durchlauffaden`, `unterbaum_entscheiden`, `Ebene`, `Kandidat`
- `crates/krk-core/src/verzeichnis/mod.rs` — `pub mod durchlauf`, `pub use`, Modulkopf und
  Bild von sieben auf acht Module nachgezogen
- `crates/krk-core/tests/verzeichnis.rs` — acht Proben und zwei Helfer
