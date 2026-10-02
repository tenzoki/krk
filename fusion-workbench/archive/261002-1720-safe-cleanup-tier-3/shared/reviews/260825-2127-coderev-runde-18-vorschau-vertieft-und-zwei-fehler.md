# Durchsicht der Runde 18: die Vorschau vertieft, und zwei Fehler

**Reviewed-range:** `20eccd4..8478753`
**Not-opened:** `resources/default-readers.toml`

**Durchgesehen von:** coderev, Kai Stalmann <kai@stalmann.org>
**Am:** 260825-2127
**Gelesen gegen:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (zehn Schritte, alle `[DONE]`) und die sieben Entscheidungsdatensätze `shared/decisions/260825-1725_a_*.md`
**Ausgelassen:** `resources/default-readers.toml`, weil `ontorev` sie in einem eigenen Durchgang liest.

---

## Zusammenfassung

Die Runde hält, was sie zusagt, und sie hält es an ungewöhnlich vielen Stellen mit Proben
statt mit Prosa. `make check` läuft grün, in allen vier Teilen am 260825-2127 selbst
gefahren: `cargo fmt --all --check` sauber, `cargo clippy --workspace --all-targets` ohne
eine einzige Meldung, `cargo test --workspace` mit 1.472 bestandenen Proben und keiner
gescheiterten. `Cargo.lock` ist im ganzen Bereich unverändert und führt weiterhin kein `cc`
und außer `windows-sys` kein `-sys`-Paket. Die Untergrenzenangabe fehlt in genau den zwei
begründeten Dateien unter `crates/krk-ui/src/appkit/` und in keiner dritten.

Sieben Befunde, keiner davon ein Fehler im gepackten oder entpackten Ergebnis. Der schwerste
ist eine Auskunft der Oberfläche, die dem Zustand des Archivs widerspricht: ein Eintrag, den
KRK gepackt hat, steht danach im Blatt „Ein Eintrag wurde übersprungen". Die zwei anderen
mittleren betreffen beide dieselbe Sorte Lücke — eine Zusage der Runde, die keine Probe hält.

---

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 3 |
| Gering | 4 |

Dazu ein `Also seen:` an einem schon offenen Datensatz. Fünf Datensätze der Runde
(260825-1859 bis 260825-2107) sind vor dieser Durchsicht gelesen worden; nichts daraus ist
hier wiederholt.

---

## Befunde nach Thema

### Thema 1: das Packen meldet den falschen Vorgang

**M1 — Ein gepackter Eintrag mit Ersatzdatum steht in der Liste der übersprungenen.**
`shared/issues/260825-2127_o_ein-gepackter-eintrag-mit-ersatzdatum-steht-in-der-liste-der-uebersprungenen.md`

`zeit_uebernehmen` meldet drei Lagen über `Steuerung::ueberspringen`
(`crates/krk-core/src/operation/zippen.rs:657`, `:671`, `:698`), und in allen dreien wird der
Eintrag gepackt. `Uebersprungen` sagt von sich selbst „Ein Eintrag, an dem die Operation
gescheitert ist … Warum er **nicht bearbeitet** wurde"
(`crates/krk-core/src/operation/fortschritt.rs:69-78`), und das Blatt sagt es dem Nutzer
wörtlich (`crates/krk-ui/src/kommandos/operationen.rs:604-606`, gezeigt über
`crates/krk-ui/src/appkit/anwendung.rs:6680`).

Die vorhandene Probe hält den Zustand ausdrücklich fest: sie prüft in einem Atemzug
`bericht.uebersprungen.len() == 1` und `archivinhalt(&archiv, "alt.txt") == "inhalt"`
(`crates/krk-core/tests/operation.rs:1564-1576`). Die Datei ist also im Archiv **und** in der
Liste der nicht bearbeiteten.

Bemerkenswert ist, dass das andere Ende derselben Runde die entgegengesetzte Wahl trifft und
sie begründet: `entpacken.rs:346-353` lässt einen Fehlschlag beim Datumsetzen stumm, „die
Datei steht vollständig da, und sie in der Abschlussliste als übersprungen zu nennen, wäre
die falsche Auskunft". Genau dieser Satz trifft auf das Packen zu.

Dazu kann ein Eintrag bis zu drei Zeilen erzeugen, und das Blatt zählt sie als drei Einträge.

**Schwere: mittel.** Kein Datenverlust; eine Aussage der Oberfläche, die dem Archiv
widerspricht, ohne Zutun des Nutzers.

**L1 — Eine unlesbare Zugriffszeit nimmt dem Änderungsdatum beide Zusatzfelder, stumm.**
`shared/issues/260825-2127_o_eine-unlesbare-zugriffszeit-nimmt-dem-aenderungsdatum-beide-zusatzfelder.md`

`zippen.rs:678` schreibt die zwei Zusatzfelder nur, wenn Änderungs- **und** Zugriffszeit in
vier Byte passen. Der Kommentar zwei Zeilen darüber nennt die Zugriffszeit „die Zugabe und
nicht der Gegenstand"; die Bedingung lässt die Zugabe den Gegenstand verhindern. Der Eintrag
fällt dann auf das MS-DOS-Feld allein zurück — den Zustand, den die Messtabelle in der
Wurzel-`Cargo.toml:189-193` als unzureichend ausweist —, und es geschieht ohne eine Zeile in
der Abschlussliste. Die Behebung ist eine Zeile: die Zugabe fällt auf den Gegenstand zurück,
so wie sie es zwei Zeilen darüber für die *fehlende* Zugriffszeit schon tut.

**L2 — Das erweiterte Zeitfeld steht mit vollem Rumpf auch im Hauptverzeichnis.**
`shared/issues/260825-2127_o_das-erweiterte-zeitfeld-steht-mit-vollem-rumpf-auch-im-hauptverzeichnis.md`

Nachgelesen in `zip-8.6.0/src/write.rs:2491-2523`: der lokale Zusatzfeldblock wandert
unverändert in den Zentraleintrag, also auch das neun Byte lange `0x5455`. `inference:` die
Info-ZIP-Festlegung sieht dort allein die Änderungszeit vor; **das ist in diesem Durchgang
nicht gegen die Quelle geprüft**. Beide gemessenen Werkzeuge liefern richtig, also ist der
Befund latent und nicht akut.

### Thema 2: eine Zusage, die keine Probe hält

Zwei Befunde derselben Bauart, an ganz verschiedenen Stellen. Die Runde arbeitet sonst
durchweg so, dass jede Zusage über den Baum eine Zählprobe bekommt; an diesen zwei Nähten
fehlt sie, und beide Male steht die Behauptung trotzdem im Text.

**M2 — Ein dritter Weg nach `aktives_setzen` hält den Bau nicht an, und keine Probe fängt
ihn.**
`shared/issues/260825-2127_o_ein-dritter-weg-nach-aktives-setzen-haelt-den-bau-nicht-an-und-keine-probe-faengt-ihn.md`

`crates/krk-ui/src/appkit/tabelle.rs:752-754` sagt: „Ein dritter Weg in `angefasst` hält damit
den Bau an, statt sich stillschweigend den einen oder den anderen Fall auszusuchen." Das
stimmt nicht. `angefasst` nimmt `Rangmitnahme` als **Argument**; ein dritter Aufrufer
übersetzt, sobald er einen der zwei Werte hinschreibt — also gerade, indem er sich einen
aussucht. Vollständig ohne Auffangzweig ist die Fallunterscheidung *in* `aktives_setzen`
(`anwendung.rs:4577-4585`), und die hält gegen einen dritten Wert, nicht gegen einen dritten
Weg.

Was gegen den dritten Weg hält, ist die Probe `die_zwei_anfasswege_unterscheiden_sich_in_der_rangmitnahme`
(`anwendung.rs:8901-8920`) — und sie zählt allein in `tabelle.rs`. Die eigentliche Naht liegt
eine Ebene höher: `aktives_setzen` hat zwei Aufrufer (`anwendung.rs:1289`, `:4656`), die
Commit-Botschaft von `d3da6e3` sagt die Zahl ausdrücklich zu, und **keine Probe hält sie**.
`keine_vierte_tuer_schreibt_das_aktive_dateifenster` zählt `fenster_wechseln` und
`aktiv_setzen` außerhalb von `fenstermodell.rs`; ein dritter Aufrufer von `aktives_setzen`
ändert an beiden Zahlen nichts, weil er dasselbe eine `aktiv_setzen` *innerhalb* von
`aktives_setzen` mitbenutzt.

Wer künftig `aktives_setzen(seite, Rangmitnahme::Appkit)` an einer Stelle schreibt, an der
AppKit den Rang nicht bewegt, stellt den Zustand wieder her, den `fd361d7` und `d3da6e3`
gerade beseitigt haben. Das wäre dasselbe Auseinanderfallen zum dritten Mal; die zwei ersten
Male haben je eine Runde gekostet.

**M3 — Die Probe zu C6.7 misst nicht mehr das größte mitgelieferte Profil.**
`shared/issues/260825-2127_o_die-probe-zu-c6-7-misst-nicht-mehr-das-groesste-mitgelieferte-profil.md`

`ausgelieferte()` hat genau einen Rufer (`crates/krk-core/tests/leseprofil.rs:2898`): die
Probe `die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`. Sie misst das
Rundenprofil (4/11) und das Wurzelprofil (3/5), beide auf die Zahl genau. Nach Leseläufen ist
aber keines von beiden mehr das größte: das neue Profil des gemeinsamen Speichers kostet
zehn. Derselbe Befund steht schon in
`shared/issues/260825-2107_*_der-l7-entscheid-…`, dort aber allein als Berichtigung eines
Satzes in einem Entscheidungsdatensatz; **die Probe, die den überholten Satz in ihrer
Überschrift führt, ist dabei nicht angefasst worden.**

Damit ist das Profil, das mit 10 von 12 Leseläufen am nächsten an seiner Schranke liegt, das
einzige ohne Probe. Die Risikotabelle des Plans nennt die Lage („Zwei Läufe Luft sind wenig")
und mindert sie mit einer Messung, die einmal von Hand gefahren wurde und nicht wiederkehrt.
Bei Überschreitung gibt es keinen Fehler, sondern ein `--` an den Zeilen, die nicht mehr
drankamen — also dieselbe Anzeige wie „dort steht nichts".

### Thema 3: Prosa, die auf ihren eigenen Nachfolger zeigt

**L3 — Der Modulkopf von `sys.rs` sagt zweimal, `ortszeit` habe keinen Rufer im Baum.**
`shared/issues/260825-2127_o_der-modulkopf-von-sys-rs-sagt-zweimal-ortszeit-habe-keinen-rufer-im-baum.md`

`crates/krk-core/src/verzeichnis/sys.rs:22-23` und `:39-45`. Der Absatz schreibt seine eigene
Bedingung aus: „Wer den Satz später liest und die zwei Rufer im Baum findet, streicht diesen
Absatz." Die zwei Rufer sind seit `e922c9e` und `66c779c` da (`zippen.rs:715`,
`bausteine.rs:788`), beide in derselben Runde, und der Absatz steht noch. Die Kästchenzeile
ist die einzige der sechs Schnittstellen ohne genannten Aufrufer, obwohl sie zwei hat.

**L4 — Die Kindproben in `tests/zeit.rs` bleiben grün, wenn ihr Name nicht trifft.**
`shared/issues/260825-2127_o_die-kindproben-in-tests-zeit-rs-bleiben-gruen-wenn-ihr-name-nicht-trifft.md`

`crates/krk-core/tests/zeit.rs:81-89` prüft allein `ergebnis.status.success()`. Am gebauten
Prüfziel nachgemessen: ein Filter, der kein Verfahren trifft, liefert `running 0 tests`,
`test result: ok` und Rückgabewert 0. Die zwei Elternproben — nach eigenem Modulkopf „die
einzige Probe, die den Sommerzeitfall überhaupt prüfen kann" — blieben nach einer Umbenennung
grün und messen nichts.

Die Datei sagt, sie schreibe die Form von `tests/ablage.rs` ab (`zeit.rs:16-19`). Sie schreibt
die eine Hälfte ab: **jeder** Elternteil in `ablage.rs` liest zusätzlich eine Spur, die das
Kind geschrieben hat (`ablage.rs:2581`, `:2595`, `:2813`, `:2825`), und die fehlt, wenn kein Kind lief.
Die Vorlage ist dicht, die Abschrift nicht.

---

## Was geprüft und in Ordnung befunden ist

Die sieben Stellen des Auftrags, jede mit dem, was ich dazu wirklich gelesen habe.

**1. `ortszeit` und die Bindung von `localtime_r(3)`.** Die Abbildung von `struct tm` ist
vollständig und in der Reihenfolge des Headers, einschließlich der zwei BSD-Erweiterungen;
die Größenprobe `struct_tm_hat_die_groesse_aus_time_h` hält 56/8 fest, dieselbe Bauform wie
für `Attrlist` daneben. Der `unsafe`-Block hat genau die zwei Zeiger und keinen dritten
Argumentweg. **Zeitpunkte vor 1970 tragen:** `epochensekunden` rundet nach unten und nicht zur
Null hin, die Probe hält `-0,5 s → -1` und `-1,5 s → -2` auseinander, und die Kindprobe unter
`TZ=UTC` hält `-1` gegen `1969-12-31 23:59:59`. **Die Sommerzeitzusage trägt wirklich durch
Bauart:** die Berliner Kindprobe rechnet zwei Zeitpunkte, die beide auf 12:00 UTC liegen, in
**einem** Lauf um und erwartet 13:00 und 14:00; ein Versatz je Lauf könnte sie nicht bestehen.
Dazu der Augenblick der Umstellung und die Sekunde davor. Der Rückgabewert trägt `#[must_use]`
am Typ. Die drei Prosastellen zur Zahl der Schnittstellen (`lib.rs`, `verzeichnis/mod.rs`,
`sys.rs`) sind nachgezogen; ein `grep` findet keine vierte mit der alten Zahl.

**2. Der `Rc` im Lesestand.** Die Begründung trägt, und sie ist nachprüfbar: `erkennen` nimmt
die Einträge als `&[Eintrag]` mit der Lebensdauer des Laufs entgegen, und `Lauf::eintraege`
gibt genau das über `wurzelstand.get_or_init(…).as_deref()` heraus — eine `RefCell` kann das
nicht. **Ausleihfehler zur Laufzeit sind keine zu erwarten:** in `stand_am`
(`bausteine.rs:380-396`) endet der `Ref` aus `borrow()` am Semikolon seiner eigenen `let`-Zeile,
`lesen` läuft ohne gehaltene Ausleihe, und `borrow_mut()` steht allein. Der einzige
verschachtelte Weg ist `stand_am(Gestreut) → lesen → gestreut_lesen → stand_am(Einer)`, und
er ist gerade der Fall, für den das `Rc` da ist: die äußere Ausleihe ist zu diesem Zeitpunkt
längst zurückgegeben. Ein wiedereintretendes `get_or_init` gibt es nicht, weil `lesen` den
Wurzelstand nicht anfragt.

**3. Der Platzhalterlauf und C3.13.** **Beide Hälften halten, und ich habe beide selbst
gelesen.** Die Bauart: `gestreut_lesen` filtert auf `eintrag.typ == Typ::Ordner`
(`bausteine.rs:449-453`), und `verzeichnis::eintrag::Typ` führt `Verknuepfung` als eigenen
Wert mit dem ausdrücklichen Satz „Der Leser folgt ihr nicht, er meldet die Verknüpfung
selbst" (`eintrag.rs:22-24`). Eine Verknüpfung kann also gar nicht in den Filter geraten. Die
Gegenprobe: `eine_verknuepfung_an_der_stelle_des_platzhalters_wird_uebergangen`
(`tests/leseprofil.rs:1812`) führt **zwei** Verknüpfungen, `hinaus` auf einen fremden Ordner
und `drinnen` auf einen Rundenordner innerhalb der Wurzel. Die erste fällt an `innerhalb`, die
zweite allein am Typfilter; ohne die zweite bliebe die Probe grün, und der Doc-Kommentar sagt
das ausdrücklich. Die Meldung des Coders, seine erste Gegenprobe sei aus genau diesem Grund
grün geblieben, ist damit bestätigt und behoben. Eine Anmerkung ohne Gewicht: die Aussage
„durch Bauart und **nicht** durch eine zusätzliche Prüfung" ist eine Halbwahrheit — jeder
Treffer geht zusätzlich durch `innerhalb` (`bausteine.rs:464`). Das ist ein Gürtel neben den
Hosenträgern und kein Fehler.

**4. Die drei Zeitfelder beim Packen.** Die Umrechnung geht über `sys::ortszeit`, also je
Zeitpunkt und nicht je Lauf; die Probe `das_msdos_feld_traegt_die_ortszeit_des_quelldatums`
misst es an einem Sommer- und einem Winterzeitpunkt und rechnet die Erwartung selbst über
`ortszeit`, statt eine feste Zahl hinzuschreiben. Das Zweisekundenraster steht als
`erwartet.sekunde & !1` in der Erwartung. Das Merkmal `unreserved` ist in der Wurzel-`Cargo.toml`
mit der Messtabelle über fünf Archive begründet. **`Cargo.lock` ist im ganzen Bereich
unverändert** (`git diff --stat 20eccd4..HEAD -- Cargo.lock` liefert nichts) und führt kein
`cc` und außer `windows-sys` kein `-sys`-Paket. Zwei Randfälle bleiben, beide oben als L1 und
L2 abgelegt.

**5. Die zwei Fokusänderungen.** Der Tab-Zweig ruft `fokus_setzen(Fokus::Dateifenster)`
innerhalb des `if gewechselt`, also unter derselben Bedingung wie der Tableistenklick; die
Ausleihe endet vor dem Fokusruf. `Rangmitnahme` ist eine vollständige Fallunterscheidung ohne
Auffangzweig, und `aktives_dem_ersthelfer_nachziehen` reicht `Appkit` herein, wo der Rang der
Auslöser ist. Der Ring bricht an `aktiv_setzen == false` ab. Die vier Zählproben halten die
Reihenfolge (Wechsel vor Fokus), die Bindung an `Rangmitnahme::Krk`, die zwei Anfasswege in
`tabelle.rs` und die Zahl der direkten Schreiber außerhalb des Fenstermodells. **Was sie nicht
halten, steht als M2 oben.**

**6. Die Vorschau ohne Auswahl, gegen C4.7 und die Arbeitsfadenzusage.** Beide bleiben
unberührt, aber C4.7 hält nicht mehr in seinem eigenen Wortlaut, sondern in der Lesart, die
der Nutzerentscheid ihm gibt. C4.7 sagt „Ein Ordner, den der Nutzer nie **auswählt**, löst
keinen Verzeichnisleselauf … aus"; der Entscheid vom 260825-1740 nimmt es unter „Constraints"
ausdrücklich auf und liest es als „der angezeigte Ordner ist ein angewählter". Das ist eine
Umdeutung des Kriteriums, sie ist vom Nutzer beantwortet, und der Plan zieht sie in seinem
Abnahmekriterium nach („Ein Ordner, den der Nutzer nie **anzeigt**"). Damit ist es keine
stille Aufweichung, und ich melde es nicht als Befund. Die Arbeitsfadenzusage ist unberührt:
gemeldet wird über denselben `Auswahlmelder`, gelesen über dasselbe `datei_anzeigen`, und
`zusammenfassen` behält seinen einen Rufer. Der Preis — jede Auffrischung stößt die Vorschau
mit an — ist als `shared/issues/260825-1922_*_eine-auffrischung-…` abgelegt, und ich habe
nichts hinzuzufügen außer einer Beobachtung: während einer Auffrischung liest `auswahl_merken`
den **alten** Bestand des Ordnermodells, weil `lesevorgang_beginnen` ihn nicht vorab leert.
Der gemeldete Pfad kann also ein eben umbenannter Eintrag sein. Es ist derselbe Mechanismus,
den jener Datensatz beschreibt, und gehört dort in die Messung.

**7. Die Zählproben selbst.** Sie sind mit einer Ausnahme sorgfältig, und mehrere benennen
ihre Blindheit im eigenen Doc-Kommentar, wie es der Modulkopf von `crate::quellbaum`
verlangt. Zwei, die ich ausdrücklich gegengeprüft habe und die halten:
`die_vorschauregel_hat_einen_rufer_und_der_ordnerwechsel_meldet` prüft die **Reihenfolge**
von `auswahl_anzeigen` und `auswahl_merken` in `nach_lesebeginn` — die Vertauschung wäre
grün übersetzbar und meldete die Zeilennummer des vorigen Ordners, also ist die Probe die
einzige Sicherung dieser Zusage. Und
`die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` hält die Zahlen
**genau** (4 und 11) statt als Schranke, mit ausgeschriebener Begründung; die Zahl 4 im
Modulkopf von `bausteine.rs` ist damit belegt und keine freistehende Prosazahl. Die Ausnahme
ist L4.

---

## Was quer durch die Runde liegt

**Erstens: die Runde ersetzt an drei Stellen den Übersetzer durch eine Zählprobe, und an
einer davon fehlt die Probe.** `Rangmitnahme`, `Kontextbefehl` aus der Runde 17 und der
Bausteinsatz sind alle als vollständige Fallunterscheidung ohne Auffangzweig gebaut. Bei
`Kontextbefehl` und beim Bausteinsatz hält der Übersetzer wirklich, weil ein neuer **Wert**
hinzukommt. Bei `Rangmitnahme` kommt kein Wert hinzu, sondern ein **Aufrufer**, und dagegen
hält keine Aufzählung. Der Unterschied ist im Doc-Kommentar nicht gemacht (M2). Das ist genau
die Falle, die `CLAUDE.md` unter „Was man nicht sieht" für den Ausführungszweig beschreibt,
nur eine Ecke weiter.

**Zweitens: eine Zahl, die eine Probe genau hält, ist in diesem Baum verlässlicher als eine
Zahl in Prosa — und die Runde weiß das und wendet es ungleich an.** Die Kosten des
Rundenprofils stehen als exakte Zusicherung in einer Probe. Die Kosten des Speicherprofils,
das dreimal so nah an seiner Schranke liegt, stehen allein in einem Bericht (M3). Dieselbe
Ungleichheit trägt L3: die Zahl der Schnittstellen ist an drei Stellen nachgezogen worden, der
Absatz über die Rufer daneben nicht.

**Drittens: die zwei Enden des Archivwegs beantworten dieselbe Frage verschieden.** Das
Entpacken lässt einen Fehlschlag beim Datumsetzen bewusst stumm und schreibt den Grund
hin; das Packen meldet denselben Fall als übersprungenen Eintrag (M1). Wer das eine liest und
das andere baut, bekommt zwei Regeln für eine Sache.

**Und was nicht quer liegt, sondern auffällig gut ist:** die Runde legt fünf eigene
Defektdatensätze für das ab, was sie nicht gelöst hat, benennt in vier Doc-Kommentaren die
Blindheit ihrer eigenen Proben, und der Coder hat den grün gebliebenen Gegenversuch bei C3.13
gemeldet, statt ihn stehen zu lassen. Der Befund M3 ist nur deshalb sichtbar, weil ein
Datensatz derselben Runde die halbe Arbeit schon gemacht hat.

---

## Reihenfolge

**Vor einem Auslieferungslauf:**

1. **M1** — die Meldung beim Packen. Sie ist die einzige Stelle, an der ein Nutzer eine
   falsche Aussage zu sehen bekommt, und sie tritt bei jeder alten Datei auf. Sie braucht
   allerdings eine Entscheidung zwischen drei Wegen, siehe den Datensatz; der billigste (stumm
   bleiben, wie das Entpacken) ist eine Zeile plus eine Probe.
2. **L3** — der Modulkopf von `sys.rs`. Zwei Zeilen, und der Absatz lädt bis dahin zum
   Rückbau eines Aufrufs ein, der zwei Rufer hat.

**Aufräumen, ohne Eile:**

3. **M2** — die Aufruferzählung auf `aktives_setzen` und der berichtigte Satz an
   `Rangmitnahme`. Kein Fehler heute; die Absicherung gegen den dritten Anlauf desselben
   Fehlers.
4. **M3** — der dritte Fall in der C6.7-Probe. Ein Prüfordner mit zehn leeren Unterordnern.
5. **L1** — die Zugriffszeit fällt auf das Änderungsdatum zurück. Eine Zeile, eine Probe.
6. **L4** — die Kindprobe prüft, dass wirklich eine Probe gelaufen ist.

**Erst nach einer Nachlesung:**

7. **L2** — das erweiterte Zeitfeld im Hauptverzeichnis. Zuerst die Festlegung nachlesen; der
   Datensatz sagt, welche Datei sie entscheidet.

---

## Was diese Durchsicht nicht sagt

- **Nichts über das Gesehene.** Kein Agent kann KRK im Vordergrund fahren. Der vierteilige
  Handgriff zum Klick-Fokus, der Doppelklick auf ein gepacktes Archiv im Finder, die vier
  neuen Zusammenfassungen im laufenden Bündel und der Handgriff aus Schritt 9 stehen
  unverändert als Nutzerarbeit unter „Testing Strategy" des Plans.
- **Nichts über `resources/default-readers.toml`.** Sie ist ausgelassen und liegt bei
  `ontorev`.
- **Nichts über die zehn Zeitzusagen aus C8.** Keine spricht über die Zusammenfassung, und der
  Abnahmelauf ist nicht gefahren.
