Der Neuerungsvergleich entscheidet „beschädigt" am rohen TOML und nicht am Leser der Datei

---

`ablage::neuerungen::eine_datei` fragt `Zugang::laden::<toml::Table>` und wertet ein `Some`
in `ersetzung` als `Befund::Ersetzt`. Ein `toml::Table` nimmt jedes syntaktisch gültige TOML
an; ob die Datei ihrem **eigentlichen** Leser genügt, entscheidet eine Ebene darüber, und
die fragt niemand. Eine Datei, die KRK verworfen und durch den Auslieferungszustand ersetzt
hat, steht deshalb in mehreren erreichbaren Fällen als `Befund::Verglichen` im Blatt, mit
Namenslisten und einem Preissatz, die über eine Datei sprechen, mit der KRK gar nicht
arbeitet.

---

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** Durchsicht der Runde 24, Bereich `feecd6c..d9535c4`
**Cross-references:** `260812-1204_o_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md` — dieselbe Naht zwischen `Zugang::laden` und dem Leser darüber, andere Folge
**Betroffen:** `crates/krk-core/src/ablage/neuerungen.rs:348-374` (`eine_datei`),
`crates/krk-core/src/ablage/neuerungen.rs:65-89` (der Modulkopf, der die Zusage
ausschreibt), `crates/krk-core/tests/ablage.rs:4561-4629`
(`ein_unbekannter_eintrag_macht_settings_und_keymap_beschaedigt`)

## Der Befund

Der Modulkopf von `neuerungen.rs` sagt es als allgemeine Eigenschaft (Zeilen 68-74):

> Bei `settings.toml` und `keymap.toml` ist ein Eintrag, den die Auslieferungsfassung nicht
> kennt, dagegen ein Schaden und keine Abweichung […] Eine solche Datei gibt ihrem
> eigentlichen Leser ihren Bestand gar nicht her […] und sie kommt deshalb hier nicht bis
> zum Vergleich, sondern trägt `Befund::Ersetzt`.

Gebaut ist davon **ein** Fall. `eine_datei` fängt den unbekannten Eintrag über
`eigene_eintraege_moeglich` ab (`neuerungen.rs:364-366`). Jede andere Art, dieselben Dateien
zu verderben, geht durch:

- `keymap.toml`: `Belegung::vom_nutzer` scheitert an vier Werten von `Belegungsfehler`
  (`crates/krk-core/src/tasten/belegung.rs:1840-1856`). `UnbekannteFunktion` ist gefangen;
  `Schreibweise` (eine Kombination in falscher Schreibweise), `FunktionDoppelt` und
  `Konflikt` sind es nicht. Alle drei machen die Datei für `belegung::laden`
  (`belegung.rs:1766-1788`) beschädigt und lassen KRK auf der Auslieferungsbelegung
  weiterlaufen; für `neuerungen` ist die Datei gültiges TOML mit lauter bekannten `id`, also
  `Befund::Verglichen`.
- `settings.toml`: `deny_unknown_fields` an `Einstellungsdatei` ist gefangen, ein falscher
  **Typ** an einem bekannten Schlüssel nicht. `toml::Table` nimmt ihn an.
- `readers.toml`: `deny_unknown_fields` steht an `Profildatei`, an `Zeilendatei` und an
  jedem der vier Bausteintische (`crates/krk-core/src/leseprofil/datei.rs:27-31`). Ein
  Buchstabendreher in einem Bausteintisch verwirft die ganze Datei, und KRK arbeitet ohne
  jedes Profil weiter. `neuerungen` sieht gültiges TOML und vergleicht.

## Was der Nutzer davon liest

Beim selben Start zwei Sätze, die einander widersprechen: die Startmeldung des eigentlichen
Lesers („diese Ablagedatei wurde beiseite gelegt und durch den Auslieferungszustand
ersetzt") und die Startzeile dieser Runde („Neu in dieser Fassung: N Einträge in
keymap.toml"). Das Blatt auf Abruf setzt darauf den Preissatz aus `preis`
(`neuerungen.rs:647-650`): „Eine Funktion, die Ihre Datei nicht führt, kostet ihre
ausgelieferten Tastenkombinationen." Tatsächlich führt KRK in diesem Fall **jede**
ausgelieferte Kombination, denn es arbeitet auf der Auslieferungsfassung, und der Nutzer
verliert nicht die genannten Einträge, sondern seine eigenen.

## Warum die Probe das nicht fängt

`ein_unbekannter_eintrag_macht_settings_und_keymap_beschaedigt` legt genau den einen
gefangenen Fall hin — einen unbekannten obersten Schlüssel in `settings.toml`, eine
unbekannte `id` in `keymap.toml` — und prüft ihn. Ihr Doc-Kommentar sagt, sie halte „die zwei
Stellen, an denen die Bauart wirklich hängt". Sie hält zwei von sechs. Die Verallgemeinerung
im Modulkopf steht damit unbewacht.

## Abnahme

Der Befund einer Ablagedatei kommt von ihrem eigenen Leser und nicht von einem zweiten
Vergleich daneben: `Befund::Ersetzt` steht genau dann, wenn `belegung::laden`,
`einstellungen::laden` bzw. `leseprofile::laden` für dieselbe Datei eine `Ersetzung` liefern.
Der Start hat diese Antworten im selben Durchgang bereits (`sitzung_laden` in
`crates/krk-ui/src/appkit/anwendung.rs`), sie sind also zu reichen und nicht neu zu erheben.

Je eine Probe in `crates/krk-core/tests/ablage.rs` für einen Fall, den die heutige Fassung
durchlässt: eine `keymap.toml` mit einer Kombination in falscher Schreibweise, eine
`settings.toml` mit falschem Typ an `terminal`, eine `readers.toml` mit einem verschriebenen
Bausteintisch. Jede muss `Befund::Ersetzt` liefern, und `traegt_unterschied()` muss falsch
sein.

Der Modulkopf sagt danach, was der Code tut, und die Begründung für die leere Gegenrichtung
steht neben dem Mechanismus, der sie trägt, statt neben einem, der sie nur in einem von
sechs Fällen trägt.
