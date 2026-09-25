# C6.5 und die Festlegung A5 berichtigt, der Satz der abgeschnittenen Zählung geprüft

**Datum:** 2026-08-24 17:22
**Agent:** analyst
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Auftrag:** Der Befund `issues/260824-1651_o_c6-5-a5-und-planschritt-6-…`. Zuerst feststellen, welche
Seite recht hat, der gebaute Wortlaut oder der zugesagte; danach die vier Stellen berichtigen und den
Datensatz schließen. Keine Datei unter `crates/` oder `resources/` anfassen, dort arbeiten zwei
andere Agenten. Nichts committen.

---

## Ergebnis

**Der gebaute Wortlaut ist der richtige, und Spec und Plan ziehen nach.** Der zugesagte Satz „über
2.000" bildet sich aus der Konstanten `HOECHSTENS_EINTRAEGE` und ist deshalb nur für die
**ungefilterte** Zählung wahr. Eine Zählung trägt nach C3 ein Muster; für einen Speicher mit 2.101
Datensätzen, von denen einer offen ist, hieße die Zeile „über 2.000 offene Defekte". Gemessen am
260824-1214 hat genau das dort gestanden, wo die Anzeige heute „mindestens 1" sagt. Der gebaute Satz
`mindestens {Treffer} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)` nennt die Treffer
und den Abbruch getrennt, ist in beiden Hälften wahr und hält die Zusage aus Schritt 6, dass die
Grenze aus der Konstanten kommt. Kein Auftrag an den `coder` folgt aus dieser Hälfte.

| Datei | Zeilen |
|---|---|
| `planning/260824-0613_o_spec-…` | 4, 86 (A5), 309 (C6.5), 319–326 (die neue Berichtigung unter der Kriterienliste von C6) |
| `planning/260824-0640_o_plan-…` | 4, 198 (`### Was eine unvollständige Lesung sagen darf`), 319 (Schritt 6) |
| `issues/260824-1651_c_…` | `Resolved:` eingetragen, `_o_` → `_c_` mit `git mv` |
| `issues/260824-1722_o_…` | neu abgelegt, siehe unten |

Die Berichtigung steht **neben** dem freigegebenen Wortlaut, in der Form der sieben übrigen; A5
bleibt im Wortlaut stehen und trägt einen Verweis, wie A7 ihn seit dem 260824-1505 trägt. Im Plan ist
der Text ersetzt und die alte Fassung in der Klammer aufgehoben, nach dem Vorbild der Berichtigung
vom 260824-1508 an Zeile 243.

## Der weitere Fund

`crates/krk-core/src/leseprofil/bausteine.rs:47-48` trägt im Modulkopf denselben Satz, den der Plan
trug: die abgeschnittene Zählung „kann sagen, dass es mehr sind als die gezählten". Der
Doc-Kommentar von `Wert::UeberGrenze` in `mod.rs:526-528` schreibt in derselben Kiste aus, dass
„über 1" gerade **nicht** wahr wäre. Zwei Doc-Kommentare desselben Moduls widersprechen einander an
der Stelle, an der ein Leser die Regel über die Teillesung nachschlägt. Abgelegt als
`issues/260824-1722_o_der-modulkopf-der-bausteine-sagt-die-abgeschnittene-zaehlung-duerfe-mehr-sagen-der-wert-widerspricht.md`,
Schwere gering, Domain code. Nicht mitbehoben, weil `crates/` in diesem Lauf gesperrt war.

## Was bewusst stehen geblieben ist

- **Die Zeile „Obergrenze gelesener Einträge bei 2.000: Festlegung A5"** unter `**Decisions made:**`
  von C6. Sie hält fest, woraus C6.5 abgeleitet ist, und die Grenze selbst ist unverändert; dieselbe
  Zurückhaltung wie bei den Berichtigungen vom 260824-1224 und 260824-1505.
- **Die Tabellenzeile „Einträge je Verzeichnisleselauf | 2.000 | C6.5, A5"** im Plan und die vier
  weiteren Stellen, die die Zahl 2.000 als **Grenze** nennen (Plan 153, 196, 307, 369). Sie nennen
  nicht den Satz der Anzeige und sind unberührt richtig.
- **Der fehlende Tausenderpunkt in der Anzeige.** `als_text` schreibt `2000`, der Spec schreibt in
  Prosa „2.000". So schreibt KRK jede Zahl, die es zeigt, `vorschaumodell::zu_gross_text`
  eingeschlossen; ein Punkt allein an dieser Stelle wäre die Abweichung. Kein Datensatz dafür.

## Nächster Schritt

Der neue Defekt an den `coder`, zusammen mit den übrigen offenen aus der Durchsicht vom 260824-1700.
Nichts ist committet.
