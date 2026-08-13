Neun Abnahmekriterien versprechen eine Probe und haben keine

---

Der Spec kennzeichnet jedes Abnahmekriterium mit **(Probe)** oder **(Bündel)**. **(Probe)**
heißt ausdrücklich: „eine Prüfung im Baum weist es nach, ein Agent kann es abnehmen." Am
260813 sind alle 58 Kriterien gegen den Baum gelesen worden. **Vierzig tragen eine benannte
Probe, die genau das prüft, was das Kriterium zusagt.** Neun versprechen eine Probe und haben
keine, acht sind nur teilweise gedeckt.

## Ganz ohne Probe

| Kriterium | Zusage | Was stattdessen trägt |
|---|---|---|
| C2.12 | `--menue-protokoll` liest das gebaute Menü weiter aus | der Modus steht im Baum und ist am 260813-0445 gelaufen, aber keine `#[test]` hält ihn |
| C2.13 | keine Systemzusätze, keine Zweitform zu einem neuen Eintrag | derselbe Lauf, und der ist bei 81 statt 82 Einträgen gefahren — eigener Datensatz `260813-0646_*_…` |
| C2.15 | ein Befehl läuft auf einen Tastendruck hin höchstens einmal | der Plan hat es selbst auf (Bündel) verschoben, der Spec sagt (Probe). Strukturell trägt es der Code: `kommando_ausfuehren` liefert die Zulässigkeit (`crates/krk-ui/src/appkit/anwendung.rs:2599-2602`) |
| C3.12 | genau ein Anwendungsfenster je Prozess | der Modulkopf von `crates/krk-ui/src/appkit/anwendung.rs:46` sagt es, der Baum belegt es nicht |
| C4.1 | `Kommando` 75 → 76, vier Aufzählungen wachsen nicht | von Hand nachgezählt und exakt; keine Probe nennt eine der fünf Zahlen |
| C4.3 | `opt+cmd+n` vorher unbelegt, kein Besitzerwechsel | am Diff nachgeprüft und exakt; `die_auslieferungsbelegung_ist_konfliktfrei` prüft nur den Endstand |
| C4.4 | Untergrenzen-Abschnitt in jeder neuen `appkit/`-Datei | von Hand: 35 von 37. Die Deckung ist schon einmal auf 5 abgesunken |
| C4.7 | kein `cc`, kein `-sys` außer `windows-sys`; Begründung je fremder Kiste | nachgeprüft: `Cargo.lock` und alle `Cargo.toml` sind in dieser Runde unverändert |
| C4.8 | `#[must_use]` an beiden Sperrgriffen | beide tragen es mit Begründung (`crates/krk-core/src/ablage/sperre.rs:111` und `:161`); erzwungen vom Bau unter `-D warnings`, nicht von einer Probe |

## Nur teilweise gedeckt

C1.3 (dass die Spalte „Belegung" durchsucht wird, prüft keine Probe), C1.10 (die Verdrängung
der Meldung bis zum nächsten Suchzeichen), C1.12 (die Zählprobe liest nur `belegungsmodell.rs`;
ein Zeitgeber in der Ansicht bliebe unsichtbar, und dass eine Aufnahme den Suchtext unberührt
lässt, ist ungeprüft), C2.5 (keine Probe zählt, dass die Ausnahmeliste genau zwei Einträge
hat), C2.17 (folgt mittelbar aus C2.16 plus dem Verbot von `setEnabled:`), C3.3 (keine Probe
sagt, dass `WeitereInstanz` `Wirkungsbereich::Ueberall` trägt), C3.11 (kein Lauf, in dem eine
rechtlose Instanz weiterläuft, während der Halter endet).

C3.2 ist mit Absicht so und kein Befund: keine Probe schreibt `opt+cmd+n` hin, weil die
Belegungsdatei die eine Quelle jeder Kombination ist.

---

**Schwere:** gering bis mittel, je Zeile verschieden. Kein Kriterium ist **falsch** — alle
neun sind am Baum nachgeprüft und treffen zu. Der Befund ist, dass die Kennzeichnung
**(Probe)** eine Zusage über die Abnehmbarkeit macht, die neunmal nicht eingelöst ist: was von
Hand nachgezählt wurde, ist beim nächsten Schritt wieder ungeprüft.

**Zwei wiegen schwerer als die anderen.** C2.13 ist die einzige Zusage der Runde, deren
Messung schon gefahren und danach durch eine Änderung überholt worden ist. C4.4 ist die
Zusage, deren Deckung schon einmal von 33 auf 5 Dateien abgesunken und von Hand
wiederhergestellt worden ist; ob sie prüfbar gemacht wird, führt der offene Datensatz
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` mit drei
Stufen und Kosten. Diese Zeile ist also nicht neu zu entscheiden, sondern dort aufgehoben.

**Die Zahlenzeilen C4.1 und C4.3 sind derselbe Fall wie die veralteten Zahlen in `CLAUDE.md`**
(`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-…`): eine Zahl, die
jede Runde mitwächst und die keine Probe hält, veraltet zuverlässig.

**Gefunden:** reconciler, Abgleich der Runde 7, alle 58 Kriterien einzeln gegen den Baum gelesen

**Betroffen:** `shared/planning/260813-0053_*_spec-…md` (die Kennzeichnungen)

**Domain:** code

## Zwei Wege

1. **Die Kennzeichnung berichtigen.** Wo der Nachweis eine Handzählung ist, heißt das nicht
   **(Probe)**, sondern etwas Drittes. Billig, und es macht die Kennzeichnung wieder wahr.
2. **Die Proben nachziehen, wo sie billig sind.** C3.3, C3.12 und die Länge der Ausnahmeliste
   aus C2.5 sind je eine Zeile in einem vorhandenen Prüfmodul. C4.1 und C4.3 sind Zählproben in
   der Bauform, die diese Runde ohnehin überall angewandt hat.

Weg 2 ist für fünf der Zeilen billiger als die Diskussion darüber; Weg 1 bleibt für den Rest.
