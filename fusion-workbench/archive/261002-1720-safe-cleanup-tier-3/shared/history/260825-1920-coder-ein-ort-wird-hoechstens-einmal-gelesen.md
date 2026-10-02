# Ein Ort wird je Zusammenfassung höchstens einmal gelesen

**Datum:** 2026-08-25
**Agent:** coder
**Status:** Complete
**Auftrag:** Schritt 4, Strang 2 des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` — „Ein Ort wird je
Zusammenfassung höchstens einmal gelesen"
**Grundlage:** `shared/decisions/260825-1725_a_liest-eine-zusammenfassung-denselben-unterordner-einmal-oder-je-zeile.md`,
Möglichkeit 1, freigegeben am 260825-1740

## Was gebaut ist

**`crates/krk-core/src/leseprofil/bausteine.rs`.** `Lauf` merkt seine Lesungen nach
aufgelöstem Pfad. An die Stelle des einen `OnceCell<Option<Lesestand>>` treten zwei Felder:

- `staende: RefCell<Vec<(PathBuf, Option<Rc<Lesestand>>)>>` — jeder gelesene Ort, in
  Lesereihenfolge. Eine Liste und keine Abbildung, weil `HOECHSTENS_LESELAEUFE` zwölf Einträge
  zulässt und das Durchgehen bei zwölf günstiger ist als das Streuen. Ein `None` an einem
  Eintrag wird mitgemerkt: ein zweiter Versuch am selben Ort scheiterte genauso und kostete
  einen weiteren Leselauf.
- `wurzelstand: OnceCell<Option<Rc<Lesestand>>>` — derselbe Stand für den erkannten Ordner, ein
  zweites Mal gehalten. **Keine zweite Lesung**, sondern ein geliehener Handgriff auf den
  Eintrag aus `staende`: `erkennung::erkennen` nimmt die Einträge als `&'e [Eintrag]` entgegen,
  also als Ausschnitt mit der Lebensdauer des Laufs, und den gibt eine `RefCell` nicht heraus.

Der `Lesestand` steht deshalb unter `Rc`: zwei Stellen brauchen ihn, die Liste hält ihn und ein
Rufer bekommt ihn geliehen. Ohne den gemeinsamen Besitz müsste `am_ort` die `RefCell`
ausgeliehen halten, während es rechnet, und ein Baustein, der dabei einen zweiten Ort bräuchte,
liefe in einen Ausleihfehler zur Laufzeit. `Rc` ist das erste in `krk-core`; es kommt aus `std`
und ändert an `Cargo.lock` nichts.

Die eine neue Stelle, an der ein Leselauf überhaupt angefordert wird, ist `Lauf::stand_am`.
`Lauf::stand` (der erkannte Ordner) und `Lauf::am_ort` (jede Ortsangabe) gehen beide durch sie,
und `am_ort` hat seine Fallunterscheidung zwischen leerer und gesetzter Ortsangabe deshalb
verloren: `zielordner` liefert für die leere Angabe die schon aufgelöste Wurzel, also findet
`stand_am` sie unter demselben Schlüssel wie jeder Rufer davor oder danach.

**Die Trägheit steht unverändert.** Gelesen wird erst, wenn der erste Rufer den Ort braucht;
`ohne_einen_rufer_wird_der_erkannte_ordner_gar_nicht_gelesen` ist unberührt grün.

**Der Merker lebt genau so lange wie ein `Lauf`.** Er ist ein Feld und keine Statik, also endet
er mit der Zusammenfassung.

**Der Modulkopf.** Der Abschnitt „Der erkannte Ordner wird höchstens einmal gelesen" heißt
jetzt „Ein Ort wird je Zusammenfassung höchstens einmal gelesen" und trägt vier Aussagen statt
der gefallenen Asymmetrie: die Regel ohne Ausnahme, die Trägheit, die Ablesbarkeit der Zahl aus
dem Profil (als Zahl der **verschiedenen** genannten Orte) und die Lebensdauer des Merkers.
Dazu die Gegenüberstellung mit der Dateiöffnung, die die andere Wahl trifft und sie behält: ein
Ort steht als Ortsangabe im Profil und ist vor der Lesung bekannt, welche Datei ein Baustein
öffnet, entscheidet erst sein Muster an den gelesenen Einträgen. Die Zahl für das mitgelieferte
Circle-Profil ist von fünf auf vier gezogen.

## Was gemessen ist

**`crates/krk-core/tests/leseprofil.rs`**, vier Änderungen:

1. Der elfte Fall von `ein_baustein_kostet_hoechstens_einen_leselauf_und_im_erkannten_ordner_keinen`
   heißt „zwei Bausteine auf demselben Unterordner teilen sich eine Lesung" und erwartet `2`
   statt `3`. Die übrigen zehn Fälle stehen Zahl für Zahl unverändert.
2. `die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`: das Circle-Profil
   kostet `(4, 11)` statt `(5, 11)`, gemessen an der eingebetteten Auslieferungsfassung. Die
   Werkbankwurzel bleibt bei `(3, 5)` — ihre drei Orte waren schon verschieden.
3. Neu: `zwei_zusammenfassungen_desselben_ordners_lesen_zweimal`. Sie misst beide Hälften, die
   ein zu langlebiger Merker verriete: der zweite Lauf über denselben Ordner bucht wieder drei
   Leseläufe, **und** er sieht einen Datensatz, der zwischen den Läufen entstanden ist. Die
   erste Hälfte allein bliebe grün, wenn jemand den Merker global hielte und trotzdem buchte.
4. **Nicht im Plan vorgesehen und trotzdem nötig:**
   `dreizehn_zaehlbausteine_erreichen_die_grenze_und_der_rest_traegt_den_platzhalter` legte
   dreizehn Zählbausteine auf **denselben** Unterordner `decisions`. Nach dieser Änderung kostet
   das zwei Leseläufe statt dreizehn, die Grenze wird nicht mehr erreicht, und die Probe maß
   nichts mehr — sie schlug fehl mit `2` statt `12`. Sie legt jetzt dreizehn Ordner `grenze-00`
   bis `grenze-12` mit je drei Datensätzen an und nennt einen je Zeile. Alle Zahlen der Probe —
   zwölf Leseläufe, null Öffnungen, elf gerechnete Zeilen, `Wert::Zahl(3)` je gerechneter
   Zeile — stehen danach unverändert. Der Doc-Kommentar sagt jetzt, warum es verschiedene Orte
   sein müssen: der Haushalt begrenzt Arbeit auf der Platte, und die macht ein weiterer Ort.

**Gegenprobe, zweimal gefahren.** Beide Male gegen die neue Probe:

| Wirkung herausgenommen | was rot wird |
|---|---|
| `stand_am` befragt den Merker nicht mehr | `zwei_zusammenfassungen_…` (4 statt 3 Läufe im ersten Lauf), dazu der elfte Tabellenfall und `die_zwei_groessten_…` |
| der Merker liegt als `thread_local` neben dem Lauf und überlebt ihn | `zwei_zusammenfassungen_…` an beiden Hälften: der zweite Lauf bucht `0` statt `3` Leseläufe, dazu `ein_baustein_…` und die Deskriptorprobe |

Danach ist die Wirkung aus der Sicherungskopie wiederhergestellt; `diff` gegen die Kopie ist
leer, und `GEGENPROBE`/`UEBERLEBT` stehen nirgends mehr im Baum.

## Was nicht angefasst ist

`leseprofil/mod.rs` und `datei.rs` (Schritt 5), `resources/default-readers.toml` — der Satz
„Ein Baustein mit `ordner` kostet genau einen Leselauf" im Kommentarblock „Was eine
Zusammenfassung höchstens kostet" ist mit dieser Änderung falsch geworden und wird nach dem
Plan in Schritt 8 nachgezogen. `HOECHSTENS_LESELAEUFE` steht unverändert auf zwölf.

## Abnahme

`make check` — exit 0.
