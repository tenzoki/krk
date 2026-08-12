Die drei Spaltenbefehle stehen nicht in der Markdown-Ausgabe, obwohl drei Stellen es zusagen

---

`belegungsausgabe::markdown` nimmt eine Funktion nur auf, wenn sie mindestens eine
Kombination trägt. Die drei Spaltenbefehle tragen ab Werk keine, also fehlen sie in
`~/Downloads/KRK-Tastenbelegung.md`. Drei Stellen dieser Runde sagen das Gegenteil zu:
Kriterium C3.5 des Plans, der Datensatz `260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md`
und der Kopfkommentar des Spaltenblocks in `resources/default-keymap.toml`. Zwei Proben
halten inzwischen ausdrücklich das Gegenteil der Zusage fest.

---

**Schwere:** mittel (kein falsches Verhalten im Betrieb, aber ein als `(Probe)` gekennzeichnetes
Abnahmekriterium ist unerfüllt, ein Entscheidungsdatensatz trägt `Implemented:` für etwas, das
nicht umgesetzt ist, und ein Kommentar in einer ausgelieferten Datei sagt dem Nutzer etwas
Falsches)
**Gefunden:** coderev, zweite Durchsicht der Runde, Bereich `8ffaac2..0342445`
**Betroffen:** `crates/krk-ui/src/belegungsausgabe.rs`, `resources/default-keymap.toml`,
`planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md` (C3.5),
`decisions/260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md`
**Domain:** code

## Was am Baum steht

`crates/krk-ui/src/belegungsausgabe.rs:170` (`markdown`), Zeilen 175 bis 179:

```rust
let belegte: Vec<&Funktion> = stellen
    .iter()
    .filter_map(|stelle| belegung.funktionen().get(*stelle))
    .filter(|funktion| !funktion.tasten().is_empty())
    .collect();
```

Der Dokumentationskommentar darüber (`:155`) nennt die Regel und ihren Preis beim Namen:
„Aufgenommen wird eine Funktion nur, wenn sie mindestens eine Kombination trägt … eine
versehentlich unbelegte Funktion verschwindet aus der Datei, statt darin als unbelegt zu
erscheinen." Sie ist ein Nutzerentscheid der Runde 3 vom 260811-0110, festgehalten in
`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_i_was-steht-in-der-ausgabe-und-wonach-ist-sie-gegliedert.md`,
Umfang Möglichkeit 1. Jener Datensatz hält ausdrücklich fest, dass die Wahl ab Werk nichts
ändert, weil damals keine der 71 Funktionen ohne Kombination war. **Seit dem 260812 sind es
drei.**

Die Bildschirmansicht ist nicht betroffen: `belegungsmodell::nach_bereichen` ordnet jede
Funktion einem Abschnitt zu, und `gliederung` baut für jede eine Zeile. Die erste Hälfte von
C3.5 hält also, die zweite nicht.

## Die drei Stellen, die das Gegenteil sagen

1. **Plan, Kriterium C3.5** (`planning/260812-0415_p_…`, Zeile 46): „Die drei Spaltenbefehle
   stehen in der Belegungsansicht **und in der Markdown-Ausgabe der Runde 3**, tragen ab Werk
   aber keine Kombination. **(Probe)**"
2. **Datensatz `260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md`**, Abschnitt
   `## Antwort`: „Sie sind damit in `Kommando` aufgezählt, tragen einen `Wirkungsbereich` und
   stehen in der Belegungsansicht und in der Markdown-Ausgabe der Runde 3." Der Datensatz trägt
   `Implemented: 90b02d4` und den Marker `_i_`.
3. **`resources/default-keymap.toml`**, Kopfkommentar des Spaltenblocks (Zeilen 306 bis 310): „Sie
   schalten in beiden Dateifenstern zugleich und stehen wie jede andere in der Belegungsansicht
   und in der Markdown-Ausgabe."

## Zwei Proben halten das Gegenteil fest

Beide sind mit Schritt 7 auf den Code nachgezogen worden, statt den Widerspruch zu melden
(Protokoll `history/260812-0618-coder-schritt-7-spaltensichtbarkeit.md`, Abschnitt „Zwei
Proben, die dieser Schritt zusätzlich rot gemacht hat"):

- `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
  (`crates/krk-ui/src/belegungsausgabe.rs:530`, dort Zeilen 556 bis 573) prüft jetzt, dass **genau** die drei
  Spaltenkennungen unbelegt sind, und die erste Hälfte derselben Probe zählt die Zeilen der
  Datei gegen die Zahl der belegten Funktionen. Zusammen sagt sie: die drei stehen nicht darin.
- `innerhalb_eines_abschnitts_bleibt_die_reihenfolge_der_datei` (`:621`) filtert die Erwartung
  seit dem 260812 auf die belegten Funktionen.

Dazu `eine_funktion_ohne_kombination_erscheint_nicht` (`:578`), die die Regel als solche misst
und älter ist.

## Die Wahl, die vor dem Beheben steht

Zwei Nutzerentscheide widersprechen sich, und keiner der beiden ist ein Versehen:

- **260811-0110** (Runde 3): nur belegte Funktionen in der Datei, gegen die Empfehlung des
  Datensatzes gewählt, mit dem Preis ausdrücklich benannt.
- **260812-0306** (diese Runde): die Spaltenschalter ohne Kombination, und sie „stehen … in der
  Markdown-Ausgabe".

Der zweite ist beim Beantworten offenbar nicht gegen den ersten gelesen worden. Drei Wege:

1. **Die Ausgabe nimmt unbelegte Funktionen auf** (mit leerer Zelle oder dem Wort „unbelegt").
   Das ist Möglichkeit 2 des Datensatzes von 260811-0110 und kehrt jenen Entscheid um.
2. **Die Ausgabe nimmt genau die unbelegten mit Kommando auf.** Ein Sonderfall neben der
   bestehenden Regel, also das Dickicht, das `critical-stance.md` §2 ausschließt.
3. **C3.5, der Datensatz vom 260812-0306 und der Kommentar in `default-keymap.toml` werden
   berichtigt.** Kein Code ändert sich; der Datensatz fällt von `_i_` zurück oder bekommt einen
   nachgeschobenen Satz, der den Widerspruch benennt.

Weg 3 ist der billigste und ändert an keinem Verhalten etwas; Weg 1 ist der, den ein Nutzer
erwarten dürfte, der in der Belegungsansicht drei Funktionen sieht, die in seiner ausgegebenen
Datei fehlen. Die Wahl gehört dem Nutzer und nicht dem coder: sie kehrt einen seiner Entscheide
um oder sie streicht eine Zusage.

**Unabhängig von der Wahl zu erledigen:** die Zeile `Implemented: 90b02d4` des Datensatzes
`260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md` behauptet heute etwas, das der
Commit nicht enthält.

---
Resolved: **berichtigt worden sind die drei Zusagen, nicht der Code.** Der Nutzerentscheid vom
260811-0110 („nur belegte Funktionen in der Ausgabe", Möglichkeit 1) gilt und ist der Sache nach
richtig: ein Dokument über die Tastenbelegung hat zu einer Funktion ohne Taste nichts zu zeigen.
`belegungsausgabe::markdown` bleibt unverändert, und die beiden Proben, die den heutigen Zustand
festhalten, bleiben ebenfalls.

Alle drei Stellen stehen jetzt richtig:

1. Kriterium C3.5 des Plans — korrigiert am 260812-0735, mit datierter Korrekturnotiz.
2. `decisions/260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md` — Abschnitt `## Antwort`
   trägt einen Korrekturblock, die Zeile `Implemented:` nennt die Einschränkung. Der Marker bleibt
   `_i_`: die Antwort des Datensatzes („Kommandos ja, ausgelieferte Kombination nein") ist
   umgesetzt; falsch war allein eine Nebenaussage darin.
3. `resources/default-keymap.toml`, Kopfkommentar des Spaltenblocks — nennt jetzt den Grund samt
   Quelle und sagt dem Leser, was er tun kann: wer einer der drei eine Kombination zuweist, findet
   sie danach auch in der ausgegebenen Datei.

Die Ursache war eine Zusage aus Unkenntnis: der Datensatz vom 260812-0306 ist geschrieben worden,
ohne den Entscheid der Runde 3 gelesen zu haben.

