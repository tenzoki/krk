Ein Tippfehler in einem Bausteintisch kostet alle Profile, und die Meldung nennt ihn nicht

---

Der Modulkopf von `datei.rs` sagt: „Ein Tippfehler **innerhalb** eines Bausteins faellt damit
auf, ein zusaetzlicher Schluessel neben der Beschriftung nicht." Das erste ist zu schwach
formuliert für das, was geschieht. Ein `mustre = 'y'` in einer `zaehlung` lässt die unmarkierte
Auswahl an allen vier Varianten scheitern, `toml::from_str` bricht ab, die ganze `readers.toml`
gilt als beschädigt, **jedes** Profil ist weg — und die Meldung lautet „data did not match any
variant of untagged enum Bausteindatei", nennt also weder den falsch geschriebenen Schlüssel
noch das Profil.

---

**Gemessen am 260824-1216 an diesem Baum**, Stand `abe1a31`, in einer Wegwerfprobe unter
`crates/krk-core/tests/`, die danach wieder entfernt wurde.

## Vier Eingaben, eine Meldung

| Eingabe | Meldung |
|---|---|
| `zaehlung = { mustre = 'y' }` (Tippfehler im Tisch) | `data did not match any variant of untagged enum Bausteindatei`, Zeile der `[[profil.zeile]]` |
| `zaehlungg = { }` (Tippfehler im Tischnamen) | dieselbe |
| eine Zeile ganz ohne Bausteintisch | dieselbe |
| `juengste = { anzahl = -3 }` (Zahl außerhalb des Bereichs) | dieselbe |

Vier verschiedene Fehler, ein Satz, und keiner der vier nennt seinen Gegenstand. Die Stelle ist
`crates/krk-core/src/leseprofil/datei.rs:106-192`.

## Warum `deny_unknown_fields` hier weniger leistet als anderswo

An einem gewöhnlichen `struct` liefert `deny_unknown_fields` die Meldung „unknown field
`mustre`, expected `ordner` or `muster`". Innerhalb einer Variante einer unmarkierten Auswahl
liefert es das nicht: es nimmt die Variante bloß aus dem Bewerberfeld, und `serde` meldet am
Ende allein, dass keine Variante gepasst hat. Die einzelnen Fehlermeldungen der vier Varianten
verwirft `serde` unterwegs. `deny_unknown_fields` an den vier Tischen wirkt also — aber es
**meldet** nicht, es scheidet nur aus.

## Warum die Reichweite die eigentliche Frage ist

Die Reichweite ist zulässig: C1.6 ordnet eine Datei, die „nicht die erwartete Gestalt" trägt,
`Grund::Beschaedigt` zu, die Datei wird beiseitegelegt und KRK arbeitet ohne Profile weiter.
Der Modulkopf zieht daneben aber eine sorgfältige Linie zwischen zwei Reichweiten — das ganze
Profil gegen die eine Zeile — und ordnet die Muster und Ortsangaben einer Zeile ausdrücklich der
kleineren zu. Ein Tippfehler in einem Tisch derselben Zeile fällt aus dieser Ordnung heraus und
kostet mehr als beide: die ganze Datei. Das steht in keiner der beiden Aufzählungen, und der
Satz „faellt damit auf" liest sich, als wäre es der milde Fall.

## Was zu tun ist

Der Bau ist nicht ohne Weiteres zu ändern; die Meldung stammt aus `serde` und nicht aus KRK. Zu
tun ist zweierlei, und beides ist Prosa:

1. **Den Modulkopf von `datei.rs` berichtigen.** Der Abschnitt `# Wo deny_unknown_fields steht
   und wo nicht` bekommt den Satz, den er heute nicht trägt: ein Tippfehler innerhalb eines
   Tisches nimmt die Variante aus dem Bewerberfeld, damit scheitert die ganze Datei, und die
   Meldung von `serde` nennt allein die Zeile und nicht den Schlüssel.
2. **Die Kommentarzeilen von `resources/default-readers.toml`** (Schritt 7, C5.10) sagen es dem
   Nutzer an der Stelle, an der er die Datei bearbeitet: ein Schreibfehler in einem Baustein
   kostet die ganze Datei, nicht die eine Zeile.

Ob KRK darüber hinaus eine eigene, benennende Meldung bauen soll, ist eine Entscheidung und
keine Nebensache: sie hinge an derselben Zwischenstufe, die der Befund
`260824-1216_o_zwei-bausteintische-…` für seinen Fall nennt.

**Schwere:** mittel. Kein Datenverlust und kein Absturz, die Datei wird beiseitegelegt und
bleibt erhalten. Der Preis ist, dass ein Nutzer mit einem Buchstabendreher alle sechs
Zusammenfassungen verliert und aus der Meldung nicht erfährt, wo er suchen soll.

**Gefunden:** coderev, bei der Durchsicht von Bündel B am 260824-1217.

**Betroffen:** `crates/krk-core/src/leseprofil/datei.rs` (Modulkopf, `Bausteindatei`),
`resources/default-readers.toml` (steht noch aus, Schritt 7)

**Domain:** code

---
Resolved: Punkt 1 des Datensatzes, und die Meldung ist dabei besser geworden, als der Datensatz
erwartet hat.

Die Aussage „der Bau ist nicht ohne Weiteres zu ändern; die Meldung stammt aus `serde`" traf für
die unmarkierte Auswahl zu und nicht mehr für das, was an ihrer Stelle steht. Mit den vier
benannten `Option`-Feldern in `Zeilendatei` (siehe
`260824-1216_c_zwei-bausteintische-…`) wird die Meldung des Tisches nicht mehr verworfen: `serde`
meldet jetzt `unknown field \`mustre\`, expected \`ordner\` or \`muster\`` statt `data did not
match any variant of untagged enum Bausteindatei`. Alle vier Eingaben der Tabelle im Datensatz
tragen damit ihren eigenen Gegenstand, und die drei prüfbaren davon hält
`ein_verschriebener_schluessel_nennt_sich_in_der_meldung`
(`crates/krk-core/tests/leseprofil.rs`): Tippfehler im Tisch, Tippfehler im Tischnamen, und der
zusätzliche Schlüssel neben der Beschriftung, der vorher gar nicht auffiel.

**Die Reichweite ist unverändert und jetzt ausgeschrieben.** Ein solcher Fehler kostet weiter die
ganze Datei; nach C1.6 ist das zulässig. Der Modulkopf von `leseprofil::datei` führt statt zwei
Reichweiten **drei** und nennt diese ausdrücklich als die weiteste, samt der Feststellung, dass
ein Buchstabendreher in einem Bausteintisch in sie fällt und nicht in die kleinste. Der Satz
„fällt damit auf", der den milden Fall nahelegte, steht nicht mehr da.

**Punkt 2 ist ausgelagert und nicht erledigt:** die Kommentarzeilen von
`resources/default-readers.toml` gehören zu Schritt 7 und dem `ontocoder`, und die Datei steht in
diesem Baum noch nicht. Neuer Datensatz:
`issues/260824-1242_o_die-kommentarzeilen-der-auslieferungsfassung-sagen-nicht-dass-ein-schreibfehler-die-ganze-datei-kostet.md`.
Die im Datensatz genannte Entscheidung über eine eigene, benennende Meldung von KRK ist damit
gegenstandslos: sie hing an derselben Zwischenstufe, die jetzt dasteht.
