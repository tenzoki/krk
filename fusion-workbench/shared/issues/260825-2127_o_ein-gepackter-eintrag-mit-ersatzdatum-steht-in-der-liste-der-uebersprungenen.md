# Ein gepackter Eintrag mit Ersatzdatum steht in der Liste der übersprungenen

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Schritt 3, Strang 1); `crates/krk-core/src/operation/zippen.rs:656-703` (`zeit_uebernehmen`); `crates/krk-core/src/operation/fortschritt.rs:69-79` (`Uebersprungen`), `:339-347` (`Steuerung::ueberspringen`); `crates/krk-ui/src/kommandos/operationen.rs:596-624` (`uebersprungenliste`); `crates/krk-ui/src/appkit/anwendung.rs:6680` (das Blatt); `crates/krk-core/tests/operation.rs:1554-1590` (`ein_zeitpunkt_vor_1980_faellt_auf_das_vorgabedatum_und_erzeugt_eine_zeile`)

---

## Was ist

`zeit_uebernehmen` meldet drei Lagen über `Steuerung::ueberspringen`, und in allen dreien
**wird der Eintrag gepackt**:

- `zippen.rs:657` — das Änderungsdatum war nicht zu lesen.
- `zippen.rs:671` — der Zeitpunkt liegt außerhalb von 1980 bis 2107.
- `zippen.rs:698` — ein Zusatzfeld kam nicht in den Eintrag.

`ueberspringen` ist aber nicht die Abschlussliste im Allgemeinen, sondern die Liste der
**nicht bearbeiteten** Einträge. Der Typ sagt es selbst (`fortschritt.rs:69-78`): „Ein
Eintrag, an dem die Operation gescheitert ist" und „Warum er **nicht bearbeitet** wurde, im
Klartext". Die Oberfläche sagt es dem Nutzer wörtlich (`operationen.rs:604-606`):

```
1     => "Ein Eintrag wurde übersprungen",
zahl  => format!("{} Einträge wurden übersprungen", ...)
```

Nach einem Packlauf über eine Datei mit einem Änderungsdatum von 1970 geht damit ein Blatt
auf, das sagt, ein Eintrag sei übersprungen worden — und die Datei liegt vollständig im
Archiv. Die vorhandene Probe hält genau diesen Zustand fest: sie prüft
`bericht.uebersprungen.len() == 1` **und** `archivinhalt(&archiv, "alt.txt") == "inhalt"`.

**Ein Eintrag kann dabei mehrere Zeilen erzeugen.** Schlägt die Umrechnung fehl **und**
scheitern beide `add_extra_data`, stehen für eine Datei drei Zeilen in derselben Liste, und
das Blatt zählt sie als drei übersprungene Einträge.

## Warum das zählt

Es ist keine falsche Datei im Archiv, sondern eine falsche Auskunft darüber, was geschehen
ist. Der Nutzer, dem KRK sagt „ein Eintrag wurde übersprungen", sucht im Archiv nach einer
fehlenden Datei, die darin liegt. Das ist dieselbe Sorte Schaden, die der Modulkopf von
`entpacken.rs` an anderer Stelle ausdrücklich vermeidet: dort bleibt ein Fehlschlag beim
Rechte- und Datumsetzen **stumm**, mit der ausgeschriebenen Begründung, „die Datei steht
vollständig da, und sie in der Abschlussliste als übersprungen zu nennen, wäre die falsche
Auskunft" (`entpacken.rs:346-353`). Das Packen trifft für dieselbe Lage die entgegengesetzte
Wahl, und die zwei Enden dieser Runde widersprechen sich damit.

Der Plan hat die Weiche gestellt und die Vokabel nicht geprüft: er sagt „mit einer Zeile in
der Abschlussliste, denselben Weg, den das Packen für eine Datei nimmt, deren Typ es nicht
annimmt". Jener Weg weist die Datei aber wirklich ab; dieser nicht.

## Was zu tun wäre

Nicht ohne Entscheidung, denn es sind drei Wege und keiner ist offenkundig der richtige:

1. **Stumm bleiben**, wie das Entpacken. Der Eintrag steht mit dem Vorgabedatum da; ein
   Datum, das der Nutzer in der Dateiliste als falsch erkennt, ist die kleinere Auskunft als
   ein Blatt, das etwas Unwahres sagt.
2. **Ein eigener Meldeweg** neben `Uebersprungen`, etwa „steht da, trägt aber das
   Vorgabedatum". Das ist die genaue Antwort und kostet einen weiteren Wert in
   `Meldung`, ein zweites Feld im `Bericht` und ein zweites Blatt oder eine zweite Überschrift.
3. **Die Vokabel weiten**: `Uebersprungen` heißt künftig „Anmerkung zu einem Eintrag", und
   das Blatt formuliert um. Billig, macht aber die Zahl im Blatt bedeutungslos, denn sie
   zählte dann Anmerkungen und keine Einträge.

Wer 1 oder 3 wählt, zieht die Probe
`ein_zeitpunkt_vor_1980_faellt_auf_das_vorgabedatum_und_erzeugt_eine_zeile` mit.

**Schwere:** mittel. Kein Datenverlust, aber eine Aussage der Oberfläche, die dem Zustand des
Archivs widerspricht, und sie tritt ohne Zutun des Nutzers auf.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`.
