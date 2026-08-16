# F1: Die abgesetzte Zeile

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt F1
**Baumstand vor der Arbeit:** `3dd799a` plus A1 `5c7f5b9`, C1 `4a54212`, A2 `7283d55`, B1 `32fd038`, D1 `09baffd`, Strang E `37ca972`
**Vorbedingungen:** C1 — `Ordnermodell::steht_wegen_des_inhalts` steht seit `4a54212`
**Erfüllt:** C5.1, C5.2, C5.3, C5.4, C5.5 (die probengestützte Hälfte; die Bündelhälfte bleibt Nutzerarbeit)
**Nicht committet:** auf Ansage des Nutzers.

## Was entstanden ist

Eine Datei, `crates/krk-ui/src/appkit/tabelle.rs`, drei Stellen.

**Der Ableser.** `DateifensterQuelle::zeile_steht_wegen_des_inhalts` steht
unmittelbar neben `zeile_markiert` und ist Zeile für Zeile dessen Bauart:
Ausleihe der Tabs, Modell des aktiven Tabs, `eintragsindex(zeile)`, dann die
Frage an das Modell. Die Regel selbst wird nicht nachgebaut — sie steht als
`Ordnermodell::steht_wegen_des_inhalts` im Kern und trägt ihre vier
Vorbedingungen selbst. Ein Zeilenindex außerhalb des Bestands liefert `None`
und damit `false`, wie bei `zeile_markiert`.

**Die dreiwertige Farbwahl.** `DateifensterDelegierter::zellenansicht` hatte
eine zweiwertige Farbwahl und hat jetzt eine dreiwertige, in genau der
Reihenfolge, die der Datensatz ausschreibt: markiert → `systemOrangeColor`;
sonst Inhaltstreffer → `secondaryLabelColor`; sonst → `labelColor`. Die
`else if`-Kette ist zugleich die Kurzschlussregel: bei einer markierten Zeile
wird `zeile_steht_wegen_des_inhalts` gar nicht erst gerufen, und der Aufruf,
der den Namen einmal kleinschreibt, entfällt für sie.

**Die Schriftwahl ist unverändert zweiwertig** und hängt weiter allein an
`markiert`. Fett gehört der Markierung; ein drittes Kennzeichen für die
Dämpfung wäre der dritte Zustand, den C5.4 ausschließt.

**Beide Eigenschaften werden weiterhin in jedem Durchgang gesetzt**, auch im
nicht zutreffenden Fall. Die Zellenansichten sind wiederverwendet, und eine
ungesetzte Eigenschaft bliebe die des vorigen Eintrags. Die neue Aussage hält
sich an dieselbe Regel: `setTextColor` bekommt in jedem Durchgang genau einen
der drei Werte.

**Die Begründung steht als Kommentarblock im Rumpf** und nicht als
Doc-Kommentar an der Methode. Beides wäre hier erlaubt — `zellenansicht` liegt
in einem gewöhnlichen `impl` und nicht in `define_class!` —, aber der
vorhandene Block zur Markierung steht schon im Rumpf, und die neue Aussage
gehört daneben und nicht an einen zweiten Ort.

## Was nicht entstanden ist

- **Kein Zeichen in der Namensspalte.** `ORDNERZEICHEN` bleibt das einzige
  Zeichen, das die Anzeigeform anhängt; `die_anzeigeform_hat_genau_zwei_leser`
  ist unberührt und grün. Das ist Möglichkeit 2 des Nutzerdatensatzes, und sie
  ist nicht gewählt worden.
- **Keine Auswahlfarbe.** KRK schreibt keine, und diese Änderung fängt nicht
  damit an. Eine ausgewählte Zeile bleibt blau unterlegt.
- **Kein Beobachter der Erscheinung.** Alle drei Farben sind dynamische
  Systemfarben.
- **`statuszeile.rs` ist nicht angefasst.** Sie gehört F2.

## Die AppKit-Berührung, und sie ist neu

`secondaryLabelColor` ist in dieser Datei neu; `NSColor` als Klasse ist es
nicht. Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` im
Modulkopf trägt die Farbe deshalb in seiner 10.10-Zeile nach, die bisher
`labelColor` (`NSColor.h:201`) und `systemOrangeColor` (`:253`) führte;
`secondaryLabelColor` steht an `NSColor.h:202`. Die Fundstelle ist dieselbe,
die `crates/krk-ui/src/appkit/leiste.rs:53-54` schon führt. Die jüngste
Berührung der Datei bleibt `NSTableViewStyle` mit 11.0, also vier
Hauptfassungen unter der Untergrenze des Bündels.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, darunter
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo fmt --all --check`. Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist im selben Lauf
durchgelaufen und nicht angefasst.

**Keine neue Probe, und das ist die Ansage des Plans.** Die Regel hinter der
Farbe steht in `krk-core` und ist von C1 abgenommen (drei Proben in
`crates/krk-core/tests/verzeichnis.rs`). Was F1 hinzufügt, ist die Verdrahtung
Zeile → Eintragsindex → Modellfrage und die Farbwahl selbst; beides braucht
eine `DateifensterQuelle` und damit AppKit-Objekte auf dem Hauptfaden, den
`libtest` nicht hergibt.

**Am Diff abgelesen, wie der Plan es ansagt:** `grep -c "NSColor::"
crates/krk-ui/src/appkit/tabelle.rs` liefert 3 statt 2, und die Schriftwahl hat
unverändert zwei Zweige.

## Was ein weggeworfenes Programm belegt hat

Ein Programm auf dem Hauptfaden hat die drei Farben unter beiden Farbtafeln
aufgelöst und auf `textBackgroundColor` derselben Tafel gelegt, weil alle drei
teilweise durchscheinend sind. Es ist nach dem Lauf wieder entfernt.

| Farbtafel | Grund | `labelColor` | `secondaryLabelColor` | `systemOrangeColor` |
|---|---|---|---|---|
| hell | 1,000 | 0,153 | 0,502 | 1,000 / 0,584 / 0,000 |
| dunkel | 0,118 | 0,865 | 0,602 | 1,000 / 0,624 / 0,039 |

Abstand im sRGB-Würfel und Kontrast gegen den Listenhintergrund:

| Farbtafel | label/secondary | secondary/orange | label/orange | Kontrast secondary | Kontrast tertiary |
|---|---|---|---|---|---|
| hell | 0,605 | 0,712 | 0,963 | 3,95:1 | 1,88:1 |
| dunkel | 0,455 | 0,690 | 0,871 | 5,89:1 | 2,26:1 |

Zwei Auskünfte daraus. Erstens: die drei Farben sind in **beiden** Tafeln
paarweise verschieden, und die Dämpfung ist von der Grundfarbe deutlich
abgesetzt. Zweitens: die letzte Spalte belegt die zweite Bauentscheidung des
Datensatzes am gemessenen Wert. `tertiaryLabelColor` läge in der hellen Tafel
bei 1,88:1 gegen den Hintergrund, also unter jeder Lesbarkeitsschwelle für
Fließtext; `secondaryLabelColor` liegt bei 3,95:1 hell und 5,89:1 dunkel.

**Was das Programm nicht belegt:** wie die drei Zeilenarten nebeneinander in
der wirklichen Liste wirken. Zahlen sagen, dass die Farben verschieden sind,
und nicht, dass der Unterschied auf dem Schirm als Absetzung gelesen wird.

## Was Nutzerarbeit bleibt

Fünf Wirkungen sind nur am laufenden Bündel zu sehen. Kein Agent kann sie
fahren: der Abnahmelauf verlangt KRK im Vordergrund.

- **C5.1** — bei gesetztem „Content" und stehendem Filtertext steht eine Zeile,
  die allein wegen ihres Inhalts gefunden wurde, sichtbar gedämpft neben einer
  Zeile, die wegen ihres Namens steht. Zu prüfen ist, ob der Unterschied auf
  dem Schirm auffällt und nicht nur im Messwert.
- **C5.2, Markierungshälfte** — eine markierte Inhaltstrefferzeile ist orange
  und fett und **nicht** gedämpft. Das ist der angenommene Verlust: unter den
  markierten Einträgen ist ein Inhaltstreffer nicht mehr von einem
  Namenstreffer zu unterscheiden.
- **C5.2, Auswahlhälfte** — eine ausgewählte gedämpfte Zeile ist blau
  unterlegt, und die gedämpfte Schrift bleibt darauf lesbar. Der gemessene
  Kontrast gilt gegen den Listenhintergrund und nicht gegen die blaue
  Auswahlfläche; diese Lage ist ungemessen.
- **C5.3** — beide Farbtafeln, mit einem Wechsel im laufenden Betrieb.
  Umschalten, während die Liste steht, und nachsehen, ob die Dämpfung
  mitzieht, ohne dass die Liste neu gelesen wird.
- **Die drei Aussagen zusammen in einer Liste** — eine gedämpfte, eine
  markierte und eine ausgewählte Zeile untereinander, in beiden Tafeln. Vier
  Spalten tragen die Dämpfung, nicht nur der Name.

Für den Lauf reicht der Prüfordner, den G2 beschreiben wird: eine Textdatei mit
der Folge im Namen und eine mit der Folge nur im Inhalt, beide im selben
Ordner.

## Offen geblieben

Die zwei Datensätze, die F1 umsetzt, stehen weiter auf `_a_`:

- `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/decisions/260816-1359_a_welche-aussage-schreibt-die-dateizelle-wenn-markierung-und-inhaltsdaempfung-zusammentreffen.md`
- `shared/decisions/260816-1310_a_sieht-der-nutzer-ob-eine-zeile-wegen-des-namens-oder-wegen-des-inhalts-steht.md`

Beide sind mit diesem Schritt in Code umgesetzt, und beide gehören auf `_i_`
mit einer `Implemented:`-Zeile. Die Zeile zitiert den Commit-Hash, und diesen
Schritt hat der Nutzer ausdrücklich nicht committen lassen. Der Nachzug gehört
deshalb an den Commit, der F1 aufnimmt.
