# Durchsicht: Bündel B, das Profilmodell ohne Fenster

**Reviewed-range:** `9c859db..abe1a31`
**Not-opened:** none

**Datum:** 2026-08-24, 12:20
**Absender:** `coderev`
**Gegenstand:** die drei Commits `f013227`, `a327d08`, `abe1a31` der Runde 16, Planschritte 3, 5
und 6
**Maßstab:** Spec `planning/260824-0613_o_spec-…` (C1, C2, C3, C6), Plan
`planning/260824-0640_o_plan-…` (Schritte 3, 5, 6), `CLAUDE.md`

Die vorige Durchsicht (`reviews/260824-1014-coderev-turn-1-…`) trug `**Not-opened:** none`, es
war also keine Dateiliste zu übernehmen. Die zwei Bereiche stoßen aneinander: jene endete auf
`b76800b`, dieser beginnt auf `9c859db`, und dazwischen liegt allein jene Durchsicht selbst.

---

## Summary

Bündel B ist sauber gebaut: `make check` läuft in allen vier Teilen grün, die zwei Durchgänge
der Erkennung sind überschneidungsfrei und vollständig, der Deskriptorhaushalt aus C6.9 hält
nachweislich, und die Zahlen aus C6.7 sind am Code nachzurechnen und stimmen. Fünf Befunde sind
neu, keiner davon ein Fehlverhalten im Ausgelieferten; vier betreffen Zusagen ohne Halter und
Anzeigen, die weniger sagen als sie sollen, einer eine Probe, die grün ist, weil sie nichts
fordert. **Kein Auslieferungshindernis, und kein Planschritt ist aufzuhalten.**

Die drei Befunde aus Turn 2 sind einzeln nachgeprüft und stimmen alle drei; der zum verankerten
Dach ist der folgenreichste und trifft Schritt 7 unmittelbar.

---

## Totals

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 4 |
| Niedrig | 1 |

Alle fünf sind als eigene Datensätze unter `issues/` dieses Circles abgelegt.

---

## Befunde nach Thema

### Thema 1: Zusagen, die kein Halter im Kern trägt

**Befund 1.1 — `zusammenfassen` nimmt auch eine Datei an, und C2.6 hängt allein am künftigen
Rufer.** Schwere: mittel. Datensatz
`issues/260824-1214_o_zusammenfassen-nimmt-auch-eine-datei-an-…`.

Gemessen in einer Wegwerfprobe, nicht hergeleitet:

```text
Profil:   pfad = 'irgendeine',  eine Zeile  zaehlung = { }
Aufruf:   zusammenfassen(&profile, <ordner>/irgendeine.md)
Ergebnis: Some(Zusammenfassung { name: "irgendeine.md", zeilen: [("Zahl", Nicht)] })
```

`bausteine.rs:135` löst mit `std::fs::canonicalize` auf, was auch für eine Datei gelingt, und
der erste Erkennungsdurchgang (`erkennung.rs:104-112`) sieht allein auf den Pfadtext. Ein
Profil mit Kennzeichendatei kann eine Datei nicht treffen, ein Profil mit Pfadmuster schon.
C2.6 verlangt das Gegenteil. Der Doc-Kommentar von `zusammenfassen` (`bausteine.rs:107-122`)
zählt drei Lagen für `None` auf und diese nicht.

Die Zusage steht heute an einer einzigen Stelle: im Nebensatz von Planschritt 9, der noch nicht
gebaut ist. Die Aufstellung `## Was der Übersetzer einfordert` im Plan führt C2.6 nicht, und
keine Probe im Kern hält sie. Zwei Wege stehen offen, und der Datensatz schreibt beide aus: die
Vorbedingung im Doc-Kommentar ausschreiben, oder die Frage im Kern entscheiden.

**Befund 1.2 — die Probe zur Teillesung lässt zwei Ergebnisse zu.** Schwere: mittel. Datensatz
`issues/260824-1218_o_die-probe-zur-teillesung-…`.

`tests/leseprofil.rs:1238-1242`:

```rust
assert!(
    matches!(werte[1].1, Wert::Vorhanden(true) | Wert::Nicht),
```

Beide zugelassenen Werte sind die beiden möglichen Antworten des Bausteins; die Zusicherung kann
nicht rot werden. Planschritt 6 verlangt an derselben Stelle ausdrücklich „mit Treffer `ja`".
Der Grund für die Hilfskonstruktion ist redlich — die Lesereihenfolge des Dateisystems ist nicht
zugesagt, und der eine Treffer `der-eine-treffer.txt` kann jenseits der 2.000 liegen. Zu retten
ist sie ohne Abschwächung: ein Muster wählen, das auf so viele Einträge passt, dass jede
Zweitausend-Teilmenge einen davon enthält, etwa `'\.md$'` bei 2.001 gleichnamig gebildeten
Dateien.

Die zwei anderen Anwendungen derselben Regel sind sauber belegt.

### Thema 2: Anzeigen, die weniger sagen als sie sollen

**Befund 2.1 — die abgeschnittene Zählung zeigt „über <Treffer>", C6.5 und Schritt 6 verlangen
„über 2.000".** Schwere: mittel. Datensatz `issues/260824-1215_o_die-abgeschnittene-zaehlung-…`.

Gemessen an einem Ordner mit 2.101 Einträgen, davon einer mit `_o_` im Namen:

```text
zaehlung = { ordner = "viele", muster = '_o_' }   ->  "Mit Muster: über 1"
zaehlung = { ordner = "viele" }                   ->  "Ohne Muster: über 2000"
```

`zaehlen` (`bausteine.rs:390-401`) gibt die **gefilterte** Zahl in `Wert::UeberGrenze` weiter,
`als_text` (`mod.rs:548`) schreibt sie aus. Der Plan verlangte an dieser Stelle die Konstante:
„die Sätze der Anzeige (`über 2.000`) entstehen aus der Konstante und nicht aus einer zweiten
Zahl im Text".

**Die gebaute Fassung ist sachlich die bessere**, und das gehört zum Befund: aus der Konstanten
gebildet hieße die Zeile „über 2.000 offene Defekte" für einen Speicher mit einem einzigen
offenen Defekt, und das wäre falsch. Der Befund ist ein anderer: „über 1" sagt dem Nutzer nicht,
dass etwas weggelassen wurde. Es liest sich als „mindestens zwei". Genau die Auskunft, die
`Lesestand::abgeschnitten` trägt und die `Wert::UeberGrenze` von `Wert::Zahl` unterscheidet,
verschwindet in der Anzeige, sobald die Zahl klein ist.

Zu berichtigen sind zwei Stellen: der Satz der Anzeige und der Wortlaut von C6.5 samt Schritt 6.
Derselbe Zuschnitt wie beim Befund `260824-1124_o_c4-3-…`: der Bau ist entschieden, die
Buchführung nicht.

### Thema 3: die Gestalt der Datei nimmt zwei Fehler ungleich auf

Beide Befunde dieses Themas entspringen einer Bauentscheidung, `#[serde(untagged)]` über
`Bausteindatei`, und sie ziehen in entgegengesetzte Richtungen: der eine Fehler wird schweigend
angenommen, der andere kostet die ganze Datei.

**Befund 3.1 — zwei Bausteintische in einer Zeile werden schweigend angenommen, der untere
fällt weg.** Schwere: niedrig bis mittel. Datensatz
`issues/260824-1216_o_zwei-bausteintische-in-einer-zeile-…`.

```toml
  [[profil.zeile]]
  beschriftung = "Beides"
  zaehlung = { }
  vorhandensein = { muster = 'y' }
```

liefert `Zaehlung`, keine Meldung, kein Hinweis. C3 sagt „genau einen Baustein". Der
Doc-Kommentar an `Bausteindatei` (`datei.rs:119-120`) beschreibt das Verhalten zutreffend und
begründet es nicht.

Der Plan hatte die Prüfung auf „genau eine Bausteinangabe" vorgesehen, aber nur im
**Ausweichweg** für den Fall, dass die Rundreise über `flatten` und `untagged` fiele. Sie ist
grün gelaufen, der Ausweichweg blieb liegen, und mit ihm diese Prüfung.

**Befund 3.2 — ein Tippfehler in einem Bausteintisch kostet alle Profile, und die Meldung nennt
ihn nicht.** Schwere: mittel. Datensatz `issues/260824-1217_o_ein-tippfehler-in-einem-…`.

Vier verschiedene Eingabefehler liefern denselben Satz:

| Eingabe | Meldung |
|---|---|
| `zaehlung = { mustre = 'y' }` | `data did not match any variant of untagged enum Bausteindatei` |
| `zaehlungg = { }` | dieselbe |
| eine Zeile ganz ohne Bausteintisch | dieselbe |
| `juengste = { anzahl = -3 }` | dieselbe |

Keiner der vier nennt seinen Gegenstand, und die ganze Datei gilt danach als beschädigt. Der
Modulkopf von `datei.rs` sagt an dieser Stelle: „Ein Tippfehler **innerhalb** eines Bausteins
faellt damit auf". Er fällt auf, aber teurer und stummer, als der Satz erwarten lässt:
`deny_unknown_fields` innerhalb einer Variante einer unmarkierten Auswahl **meldet** nicht, es
scheidet die Variante nur aus dem Bewerberfeld aus, und `serde` verwirft die Einzelmeldungen.

Die Reichweite selbst ist zulässig, C1.6 deckt sie. Was nicht stimmt, ist der Modulkopf: er
zieht sorgfältig eine Linie zwischen zwei Reichweiten und ordnet die Muster und Ortsangaben
einer Zeile der kleineren zu, während ein Tippfehler in einem Tisch derselben Zeile aus beiden
herausfällt und mehr kostet als jede von ihnen.

---

## Die drei Befunde aus Turn 2, einzeln nachgeprüft

| Datensatz | Nachgeprüft | Ergebnis |
|---|---|---|
| `260824-1042_o_…-ist-eine-fuenfte` | `datei.rs:271-292`, `zeile_pruefen` | **stimmt.** Ein unübersetzbares `muster` in `zaehlung`, `juengste`, `vorhandensein` und ein `datei` in `feld` sind als Zeilenabweisung eingeordnet; der Plan zählt an dieser Stelle vier Fälle auf. Die Einordnung folgt der Regel, die der Plan für seine vier selbst zieht, und ist die richtige. |
| `260824-1124_o_zwei-feldmuster-…-mit-dach` | Semantik der Kiste `regex`, Probe `das_feld_zieht_die_erste_fanggruppe_…` | **stimmt, und es ist der folgenreichste der drei.** `regex` verankert `^` und `$` ohne `m` an Anfang und Ende der ganzen Eingabe; die Probe trägt seit `abe1a31` `(?sm)` mit einem Verweis auf den Datensatz. Zwei der sechs Muster aus Schritt 7 können nie treffen, und beide gehören zu den Zusammenfassungen, die der Nutzer bei der Abnahme zuerst ansieht. **Schritt 7 ist ohne diese Berichtigung nicht zu bauen.** |
| `260824-1124_o_c4-3-…-verlangt-einen-absatz` | `mod.rs:444-459`, Probe `der_text_setzt_einzeilige_werte_hinter_…` | **stimmt.** Die Unterscheidung in `als_text` ist überschneidungsfrei und vollständig, und der Wortlaut von C4.3 ist enger als die gebaute Anzeige. Zu berichtigen ist der Spec und nicht der Code. |

---

## Was geprüft wurde und gehalten hat

Diese Liste steht hier, damit die Befunde nicht als das Ganze gelesen werden.

**Die Zusage aus C2.8 hält, und sie hält aus dem Grund, den der Plan nennt.** `Regex::new` steht
im ganzen Baum genau **einmal**, in `datei.rs:300`; nachgezählt mit `grep -rn "Regex::new"
crates xtask`. Jedes Muster wird beim Laden übersetzt und die übersetzte Fassung behalten; keine
Stelle in `erkennung.rs` oder `bausteine.rs` übersetzt beim Anzeigen. Die Laufzeitzusage ist
damit eine Eigenschaft der Kiste `regex` und keine Vorhersage über ein Muster. Der Baum führt
`fancy-regex` an keiner Stelle unmittelbar.

**Der feste Bausteinsatz ist vier und nicht fünf.** `Baustein` (`mod.rs:265-301`) trägt vier
Werte ohne Auffangzweig, `baustein_pruefen` (`datei.rs:271-292`) und `Lauf::rechnen`
(`bausteine.rs:273-298`) unterscheiden vollständig über sie, `Bausteindatei` trägt vier
Varianten. **Kein Baustein liefert einen Dateinamen** — die einzige Stelle, an der ein Name zum
Wert wird, ist `titel` (`bausteine.rs:437-442`) als Rückfall für eine Datei, die keinen Titel
hergibt, und das ist der Nutzerentscheid aus C3.6 und kein fünfter Weg. Festlegung A7 ist
eingehalten.

**Die Erkennungsregel ist überschneidungsfrei und vollständig.** `erkennen`
(`erkennung.rs:99-135`) fährt zwei Durchgänge, der erste ganz vor dem zweiten; C2.3 fällt
daraus, statt danebenzustehen. Der Abschluss wird höchstens einmal gerufen, `vorrat` hält seine
Antwort für die übrigen Profile des zweiten Durchgangs. Ohne Einträge endet der zweite Durchgang
unentschieden und nicht negativ, derselbe Rückgriff wie `ist_deskriptormangel`. Sieben Proben
belegen C2.1 bis C2.4 und die zwei Bauarteigenschaften.

**Die Zahlen aus C6.7 sind am Code nachzurechnen und stimmen.** Das Circle-Profil: ein
Erkennungslauf, `planning` zweimal, `decisions`, `history` — fünf Leseläufe; ein Feldbaustein auf
dem Circle-Datensatz und zehn Verlaufsdateien — elf Öffnungen. Die Wurzel: Erkennungslauf,
`circles`, `shared/issues` — drei Leseläufe; drei Felder auf `.fusion-setup`, je eines auf
`.active-circle` und `orchestrator-live.md` — fünf Öffnungen. Beide unter den Zusagen. Der
Erkennungslauf zählt mit, weil `Lauf::lesen` ihn bucht, bevor gelesen wird.

**`oeffnungen_nehmen` bucht an jeder Rufstelle ganz oder gar nicht.** Zwei Rufer:
`juengste` (`bausteine.rs:333-336`, `wie_viele` in einem Zug) und `feld`
(`bausteine.rs:361`, genau eine). `Haushalt::oeffnungen_nehmen` (`mod.rs:607-616`) prüft die
Summe vor der Buchung und lässt den Zähler bei Ablehnung stehen; `checked_add` fängt den
Überlauf. Die Probe `der_haushalt_deckelt_die_oeffnungen_und_nimmt_sie_ganz_oder_gar_nicht`
belegt beides.

**Der Deskriptorhaushalt aus C6.9 hält.** `Schwungleser` hält seinen `File` im Wert und schließt
ihn beim Verlassen von `lesen_hoechstens`; `Lesestand` trägt Einträge und keinen Deskriptor.
`anlesen` öffnet und schließt innerhalb seines Aufrufs. Keine Stelle in `bausteine.rs` hält
einen Ordner offen, während sie eine Datei liest: `am_ort` liefert erst den fertigen
`Lesestand`, und `juengste` liest daraus eine Datei nach der anderen. Eine Liste offener Dateien
entsteht nicht.

**Die eine Regel über die Teillesung steht einmal da und wird dreimal angewandt**, an einer
Stelle als Kommentar (`bausteine.rs:34-58`) und in `zaehlen`, `vorhandensein`, `juengste`. Zwei
der drei Anwendungen sind belegt, die dritte nicht — siehe Befund 1.2.

**`#[must_use]` ist gesetzt, wo es der Baum sonst setzt.** Zwölf Stellen im neuen Modul, darunter
`pruefen`, `zusammenfassen`, `erkennen`, `als_text`, beide Buchungen des Haushalts und jeder
Bausteinrechner. Dass die reinen Erzeuger (`Profil::neu`, `Zeile::neu`, `Zusammenfassung::neu`)
es nicht tragen, ist **kein** Bruch: keine der dreizehn `pub fn neu` in `krk-core` trägt es.

**Die vollständigen Fallunterscheidungen tragen keinen Auffangzweig.** `Baustein`, `Wert`,
`Ortsmangel`, `Bausteindatei` und die vier `match` darüber sind erschöpfend.

**`make check` läuft in allen vier Teilen grün**, gefahren am 260824-1210: `cargo build`,
`cargo test --workspace` (alle Ziele grün, kein `ignored` außer den vorhandenen),
`cargo clippy --workspace --all-targets` ohne eine Warnung, `cargo fmt --all --check` sauber.

---

## Querschnitt

**Vier der fünf Befunde sind Zusagen ohne Halter, und das ist die Handschrift dieses Bündels.**
Der Code ist an jeder Stelle, an der er entscheidet, sorgfältig und begründet; was fehlt, ist
jeweils der Halter für eine Aussage, die daneben steht — C2.6 hängt an einem Nebensatz eines
ungebauten Schritts, „genau ein Baustein" an keiner Prüfung, die Teillesungsregel zur Hälfte an
einer Zusicherung, die nichts fordert. Die Aufstellung `## Was der Übersetzer einfordert, und was
er nicht einfordert` im Plan ist genau für diese Frage gebaut und führt keinen der vier Fälle.
**Wer den Plan an dieser Stelle ergänzt, bekommt für die künftigen Bündel mehr, als die vier
Einzelberichtigungen zusammen tragen.**

**Zwei Befunde derselben Bauentscheidung zeigen in entgegengesetzte Richtungen.** `untagged`
nimmt den einen Eingabefehler schweigend an und lässt den anderen die ganze Datei kosten. Die
Wahl ist richtig getroffen und ihre Vorlage abgenommen; ihr Preis steht aber nur in der Hälfte
im Modulkopf, die dem Nutzer entgegenkommt.

**Dreimal in dieser Runde ist der Bau entschieden und die Buchführung nicht.** Der Befund
`260824-1124_o_c4-3-…` aus Turn 2, der Befund 2.1 dieser Durchsicht und, in schwächerer Form,
der Befund `260824-1042_o_…-fuenfte`. Alle drei folgen demselben Muster: die Umsetzung hat die
bessere Antwort gefunden, und der Spec oder der Plan sagt weiter die alte. Wer sie einzeln
nachzieht, zieht sie dreimal nach; wer sie zusammen nachzieht, einmal.

---

## Reihenfolge

**Vor Schritt 7** (der Auslieferungsfassung), weil er sonst zwei Muster einbaut, die nie treffen:

1. `260824-1124_o_zwei-feldmuster-…-mit-dach` — die zwei Muster in Plan und Datei berichtigen.

**Vor Bündel D**, weil dort der Rufer entsteht, an dem C2.6 hängt:

2. `260824-1214_o_zusammenfassen-nimmt-auch-eine-datei-an` — den Weg wählen und ihn gehen.

**Vor Schritt 12** (den Zählproben), weil er dieselbe Probendatei anfasst:

3. `260824-1218_o_die-probe-zur-teillesung-…` — die Zusicherung festnageln.

**Wann es passt**, ohne einen Schritt aufzuhalten:

4. `260824-1215_o_die-abgeschnittene-zaehlung-…` — Anzeige und Wortlaut zusammenführen.
5. `260824-1217_o_ein-tippfehler-in-einem-bausteintisch-…` — den Modulkopf berichtigen; die
   Nutzerhälfte gehört in die Kommentarzeilen von Schritt 7.
6. `260824-1216_o_zwei-bausteintische-…` — entscheiden, ob geprüft oder ausgeschrieben wird.

Kein Befund hält einen Planschritt auf, und keiner ist ein Auslieferungshindernis.
