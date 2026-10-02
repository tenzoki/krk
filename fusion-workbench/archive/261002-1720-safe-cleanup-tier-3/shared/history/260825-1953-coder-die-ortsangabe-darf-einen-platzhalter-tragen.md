# Die Ortsangabe darf einen Platzhalter tragen

**Datum:** 2026-08-25
**Agent:** coder
**Status:** Complete
**Auftrag:** Schritt 5, Strang 2 des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` — „Die Ortsangabe
darf einen Platzhalter tragen"
**Grundlage:** `shared/decisions/260825-1725_a_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner.md`,
Möglichkeit 1, freigegeben am 260825-1740: die Ortsangabe trägt den Platzhalter, **kein
fünfter Baustein**

## Was gebaut ist

**`crates/krk-core/src/leseprofil/mod.rs`.** `Ortsangabe` trägt statt eines `Vec<String>` zwei
Felder: `vor` und `nach: Option<Vec<String>>`. `None` heißt „kein Platzhalter", `Some` mit
leerer Folge heißt „einer, und dahinter steht nichts" — das ist die Angabe `*`. Die zwei
Zustände sind verschieden und dürfen nicht zusammenfallen; ein einzelnes `Vec` hätte sie nicht
auseinanderhalten können. Dazu drei Leser: `teile()` (die Stücke bis zum Platzhalter, ohne
Platzhalter alle), `hinter_dem_platzhalter()` und `traegt_platzhalter()`.

`aus_angabe` weist unverändert den absoluten Pfad, das leere Stück, `.` und `..` ab und
zusätzlich den **zweiten** Platzhalter. `Ortsmangel` bekommt dafür den vierten Wert
`MehrerePlatzhalter`; die Aufzählung bleibt vollständig ohne Auffangzweig, also hat der Bau an
`Ortsmangel::grund` angehalten und die Zeile ist bewusst eingetragen worden.

Offen ist ein Name nur, wenn das **ganze** Stück aus dem Stern besteht: `a*b` bleibt ein
gewöhnlicher Name. Der Preis steht in der Dokumentation, wie ihn die Cons-Liste des Entscheids
nennt — ein Ordner, der `*` heißt, ist als Ort nicht mehr benennbar.

Die Konstante `PLATZHALTERSTUECK` steht neben `PLATZHALTER`, und ihre Dokumentation trennt die
beiden ausdrücklich: jenes ist das Zeichenpaar `--` der Anzeige, dieses das Stück in
`readers.toml`.

**`crates/krk-core/src/leseprofil/bausteine.rs`.** Die neue Aufzählung `Ort` ist der aufgelöste
Ort, an dem ein Baustein arbeitet, und zugleich der Schlüssel des Merkers aus Schritt 4:

- `Ort::Einer(PathBuf)` — ein einzelnes Verzeichnis, wie bisher.
- `Ort::Gestreut { ueber, hinter }` — jeder Unterordner von `ueber`, je um `hinter`
  verlängert.

Damit ist der Merker aus Schritt 4 ohne Umbau tragfähig: zwei Zeilen mit derselben
Platzhalterangabe treffen denselben Wert und teilen sich die eine Sammlung, und
`staende: RefCell<Vec<(Ort, Option<Rc<Lesestand>>)>>` ist der einzige Unterschied zu vorher.
Gebucht wird ein Leselauf je **verschiedenem Ort**, und ein Platzhalterlauf ist ein Ort.

`Lauf::gestreut_lesen` legt die Sammlung an: es holt den Ordner vor dem Platzhalter über
`stand_am` — also über denselben Merker, weshalb er höchstens einmal gelesen wird, auch wenn
eine andere Zeile ihn ebenfalls nennt —, nimmt daraus jeden Eintrag vom Typ `Ordner`, hängt
`hinter` an und liest dort. `abgeschnitten` steht, sobald die Elternlesung abgeschnitten war,
eine Teillesung abgeschnitten war oder die Sammlung `HOECHSTENS_EINTRAEGE` erreicht; jede
Teillesung bekommt als Deckel den **Rest** der Schranke, nicht die ganze.

Das `Rc` aus Schritt 4 zahlt sich hier ein zweites Mal aus: die Sammlung fragt den Merker nach
ihrem Elternordner, **während** sie selbst gerade in ihn aufgenommen wird. Das trägt nur, weil
kein Ausleihen der `RefCell` über den Aufruf hinaus offen steht.

Die zwei Schranken, an denen C3.13 hängt, sind an einer Stelle zusammengezogen:
`Lauf::innerhalb` löst auf und hält gegen die Wurzel. Zwei Rufer, die Ortsangabe eines
Bausteins und **jeder einzelne Treffer** einer Sammlung.

`am_ort` und `in_einem_ordner` sind zwei Stellen statt einer, und die Naht ist dieselbe, die
der Modulkopf ohnehin zieht: `am_ort` reicht dem Rechner allein den Lesestand und trägt damit
beide Ortsformen, `in_einem_ordner` reicht ihm Pfad **und** Stand und weist eine Sammlung ab.
Zählung und Vorhandensein gehen durch die erste, die jüngsten N und das Feld durch die zweite.
Ein Pfad, den es für eine Sammlung nicht gibt, wird damit nicht erfunden.

**`crates/krk-core/src/leseprofil/datei.rs`.** `ortsangabe_ohne_platzhalter` ist die zweite
Hälfte derselben Aussage, beim Laden: `juengste` und `feld` bekommen ihre Ortsangabe über sie
und werden mit eigener Meldung abgewiesen, wenn ein Platzhalter darin steht. Die Zeile behält
ihre Beschriftung und verliert ihren Baustein — die dritte Reichweite der Prüfung. Die Meldung
nennt den Tisch in Anführungszeichen, damit sie sich von einer Beschriftung unterscheidet, die
zufällig „juengste" enthält.

**Die Modulköpfe.** `bausteine.rs` bekommt den Abschnitt „Ein Platzhalterlauf ist ein Ort und
kostet einen Leselauf": was er greift, warum die Einheit der Begrenzung wechselt, welcher
Preis dafür bewusst gezahlt wird (ein Leselauf öffnet nicht mehr genau ein Verzeichnis), dass
der Ordner davor selbst ein Ort ist, und was `abgeschnitten` an einer Sammlung heißt. Der
Abschnitt „Was ein Name entscheidet und was eine Datei" sagt jetzt, dass dieselbe Naht den
Platzhalter trägt; der Deskriptorabschnitt sagt, warum C6.9 unberührt bleibt (die Sammlung
hält die **Einträge** des Elternordners und keinen Deskriptor darauf, wie
`verzeichnis/durchlauf.rs` seine Unterordner als Pfad vormerkt). `datei.rs` nennt die zwei
neuen Abweisungen in der dritten Reichweite; `HOECHSTENS_EINTRAEGE` sagt, dass es seit dieser
Runde einen Ort und nicht ein Verzeichnis begrenzt.

## Was gemessen ist

**`crates/krk-core/src/leseprofil/mod.rs`**, zwei neue Proben neben der erweiterten alten:
`ein_platzhalter_zerlegt_die_ortsangabe_in_zwei_haelften` (die drei Formen `*`, `*/issues`,
`circles/*/planning`, und dass „kein Stern" und „Stern ohne Rest" verschieden sind) und
`ein_zweiter_platzhalter_wird_abgewiesen_und_ein_stern_im_namen_nicht`.

**`crates/krk-core/tests/leseprofil.rs`**, fünf neue Proben und ein neuer Prüfbestand
`circlespeicher`: drei Rundenordner mit Zustandsmarker, zwei davon mit Defektspeicher, dazu
eine Datei und **zwei** Verknüpfungen.

| Probe | was sie abnimmt |
|---|---|
| `eine_ortsangabe_mit_zwei_platzhaltern_nimmt_der_zeile_ihren_baustein` | drei Formen, je mit Meldung, die Profil, Beschriftung und Grund nennt; die Zeile daneben mit **einem** Platzhalter bleibt unberührt |
| `juengste_und_feld_nehmen_keinen_platzhalter_an` | zwei Meldungen, jede mit ihrem eigenen Tisch; Zählung und Vorhandensein nehmen ihn an |
| `der_platzhalter_legt_die_eintraege_aller_unterordner_zu_einem_stand_zusammen` | `*` und `*/issues` liefern Zahlen; die Runde ohne Defektspeicher wird übergangen und macht die Zeile nicht zum Platzhalterwert; **drei** Leseläufe, obwohl drei Zeilen den Ordner davor nennen; null Öffnungen |
| `eine_verknuepfung_an_der_stelle_des_platzhalters_wird_uebergangen` | beide Verknüpfungen, mit Nichtleerheitsprüfung davor |
| `eine_sammlung_ueber_der_grenze_nennt_die_treffer_und_nicht_die_grenze` | `Wert::UeberGrenze(1200)` bei einer Schranke von 2.000, und zwei Leseläufe statt einem je Ordner |

**Zwei Verknüpfungen und nicht eine, und das ist die Lehre dieser Aufgabe.** Die erste
Gegenprobe blieb grün: eine Verknüpfung, die aus dem erkannten Ordner **hinaus** führt, fällt
schon an der aufgelösten Prüfung, und die Typfrage — die Stelle, an der C3.13 laut Plan „durch
Bauart" hält — wurde dabei überhaupt nicht gemessen. Der Prüfbestand trägt deshalb eine zweite
Verknüpfung, die auf einen Rundenordner **innerhalb** der Wurzel zeigt; sie kommt durch die
Auflösung und wird allein von der Typfrage übergangen.

**Die letzte Probe hängt an keiner Lesereihenfolge.** Drei Unterordner mit je 1.000 Einträgen,
davon je 600 Treffer: welche zwei von ihnen die Schranke füllen, gibt das Dateisystem vor, und
die Antwort ist in jeder Reihenfolge dieselbe. Ein Muster, das alles trifft, hätte 2.000
geliefert und damit gerade nicht belegt, dass die Zahl die der Treffer und nicht die der
Grenze ist.

**Gegenprobe, fünfmal gefahren.** Jede Wirkung einzeln herausgenommen, danach aus der
Sicherungskopie wiederhergestellt:

| Wirkung herausgenommen | was rot wird |
|---|---|
| der Platzhalter greift jeden Eintrag außer Dateien | `eine_verknuepfung_an_…` (4 und 5 statt 3 und 3), `der_platzhalter_legt_…` |
| jede Teillesung bucht ihren eigenen Leselauf | `der_platzhalter_legt_…` (8 statt 3), `eine_sammlung_ueber_der_grenze_…` (4 statt 2) |
| der Ordner vor dem Platzhalter geht am Merker vorbei | `der_platzhalter_legt_…` (5 statt 3) |
| `juengste` und `feld` nehmen die Ortsangabe wie bisher | `juengste_und_feld_nehmen_keinen_platzhalter_an` |
| die Abweisung des zweiten Platzhalters fällt weg | `ein_zweiter_platzhalter_…` und `eine_ortsangabe_mit_zwei_platzhaltern_…` |

## Was aufgefallen ist

`shared/issues/260825-1953_o_ein-platzhalterlauf-oeffnet-bis-zu-zweitausend-verzeichnisse-…`:
die Eintragsschranke begrenzt die gesammelten **Einträge** und nicht die geöffneten
**Verzeichnisse**, und die zwei fallen bei leeren oder fehlenden Unterordnern auseinander. Die
Obergrenze steht trotzdem, aber an anderer Stelle als gedacht — der Ordner vor dem Platzhalter
wird selbst gedeckelt gelesen, also höchstens 2.000 Treffer je Zeile. Der Entscheid nennt den
Preis in seiner Cons-Liste; der Datensatz zieht die Zahl daraus und schlägt vor, sie in
Schritt 9 zu messen, bevor jemand etwas daran baut.

## Was nicht angefasst ist

`resources/default-readers.toml` — der Kommentarblock zur Ortsangabe wird nach dem Plan in
Schritt 8 nachgezogen, und der Satz „Ein Baustein mit `ordner` kostet genau einen Leselauf"
ist dort schon seit Schritt 4 falsch. `HOECHSTENS_LESELAEUFE` steht unverändert auf zwölf,
`HOECHSTENS_EINTRAEGE` auf 2.000. `Baustein` trägt weiter genau vier Werte; Festlegung A7 ist
unberührt. Kein Bau unter `crates/krk-ui/`, `operation/` oder `verzeichnis/`.

## Abnahme

`make check` — exit 0.
