# Der dritte Fokusbefehl und das Hervorholen

**Agent:** coder
**Datum:** 260807-0922
**Auftrag:** R2a aus Turn 26 — die beiden Nutzerentscheide vom 260807 am Eingabefokus umsetzen
**Status:** Complete, mit einem angekündigten roten Punkt (siehe „Abnahme")

## Was umzusetzen war

Zwei Entscheide des Nutzers, beide am Eingabefokus:

1. `decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md` — Möglichkeit 1: ein Kommando `fokus_vorschau` auf `shift+cmd+y`.
2. `decisions/260805-1730_*_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md` — Möglichkeit 2: `fokus_leiste` blendet eine ausgeblendete Leiste ein und setzt den Fokus, statt stumm abzuweisen.

Die beiden Datensätze sind nicht angefasst; der `planner` beantwortet sie.

## Eine Regel für beide Entscheide

Die beiden Aufträge sehen wie zwei Änderungen aus und sind eine. Der zweite legt fest, was ein Fokusbefehl tut, wenn sein Bereich nicht auf dem Schirm steht; der erste bringt einen dritten Fokusbefehl. Sie getrennt zu bauen hieße, `shift+cmd+l` das Hervorholen zu geben und `shift+cmd+y` die stumme Abweisung — zwei Antworten auf dieselbe Frage, drei Tage nach dem Entscheid.

Gebaut ist deshalb **eine** Regel an einer Stelle:

```text
Kommando::FokusLeiste ─┐
Kommando::FokusVorschau├─> fokus_holen ─> fokus::holt_hervor(Fokus) ─> Option<Bereich>
Kommando::FokusDatei… ─┘                        │
                                   Fenstermodell::einblenden  (holt hervor, blendet nie aus)
                                                 │
                                        fokus_setzen(Fokus)
```

`fokus::holt_hervor` (`crates/krk-ui/src/kommandos/fokus.rs:124`) nennt für jeden Fokusort den Bereich, den der Befehl vorher hervorholt: die Leiste für `Fokus::Leiste`, die Vorschau für `Fokus::Vorschau`, nichts für das Dateifenster. Das aktive Dateifenster ist nie ausgeblendet — das linke lässt C7 nicht ausblenden, und mit dem rechten wandert die Aktivität auf das linke.

`Fenstermodell::einblenden` (`crates/krk-ui/src/fenstermodell.rs:257`) trägt die Asymmetrie: sichtbar bleibt sichtbar, ausgeblendet kommt hervor, ausgeblendet wird nie. Sie steht neben `umschalten` und nicht bei den Befehlen, die sie brauchen — die Sichtbarkeit gehört C7 und wohnt im Fenstermodell, wie der Auftrag es verlangt hat.

**Dasselbe Muster stand schon da, aber nicht als Regel.** `shift+f3` aus C10 (`zwischenablage_ansehen`) hatte die vier Zeilen „wenn nicht sichtbar, dann umschalten" inline im Rumpf stehen. Sie sind jetzt derselbe Aufruf: `self.bereich_einblenden(Bereich::Vorschau)`. Damit gibt es einen Weg statt zweier, und die Reihenfolge im Rumpf ist im Kommentar festgehalten, weil sie trägt (der Vorschau-Nachtrag muss vor dem Einblenden gelöscht sein, sonst überschriebe er den gerade gezeigten Inhalt).

## Der Nachzug, den das Einblenden brauchte

`bereich_umschalten` hing drei Nachzüge an jedes Umschalten: die Dateisystemwache beim zweiten Dateifenster, das Zurückholen des Fokus aus einem ausgeblendeten Randbereich, das Nachladen der wieder eingeblendeten Vorschau. Alle drei brauchte auch das Einblenden. Sie stehen jetzt einmal als `nach_dem_sichtbarkeitswechsel` (`crates/krk-ui/src/appkit/anwendung.rs:1658`), gerufen von `bereich_umschalten` und `bereich_einblenden`. Eine zweite Liste daneben wäre die erste Abweichung zwischen zwei Wegen in denselben Zustand gewesen.

Die Nachzüge sind nach dem **Bereich** unterschieden und nicht nach dem Befehl. Das Zurückholen des Fokus feuert beim Einblenden nie, weil sein Vorbehalt „und der Bereich ist jetzt unsichtbar" lautet.

## Die Einordnung in den vollständigen Fallunterscheidungen

Der Auftrag nannte drei Stellen ohne Auffangzweig. **Zwei davon halten den Bau an, die dritte nicht.**

| Stelle | Einordnung |
|---|---|
| `Kommando::wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs:429`) | `Wirkungsbereich::Ueberall`, im Arm der beiden anderen Fokusbefehle. Ein Befehl, der den Fokus **holt**, kann nicht voraussetzen, wo er steht: trüge er den Bereich, in den er führt, wäre er allein von dort aus erreichbar und damit nutzlos. |
| `bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs:198`) | `Funktionsbereich::Vorschau`, nicht `LeisteUndFokus`. Begründung unten. |
| `schiebt_auffrischung_auf` (`crates/krk-ui/src/auffrischung.rs:181`) | **Nicht betroffen.** Die Fallunterscheidung läuft über `krk_core::operation::Art`, die fünf Operationsarten aus C4, und nicht über `Kommando`. Ein neues Kommando hält dort nichts an; die Datei enthält überhaupt keine `Kommando`-Zeile. Sie ist unverändert. |

**Warum `fokus_vorschau` unter „Vorschau" steht und nicht unter „Leiste und Fokus".** Die Gliederung der Belegungsansicht fragt nach der Gegend der Anwendung und nicht nach dem Mechanismus; das steht schon zweimal in derselben Datei. `leiste_umschalten` steht unter „Leiste und Fokus" und nicht unter „Fenster", `vorschau_umschalten` unter „Vorschau" und nicht unter „Fenster" — mit dem ausgeschriebenen Grund „wer die Vorschau sucht, sucht unter Vorschau". Derselbe Satz ordnet den Fokusbefehl hierher. Unter „Vorschau" findet der Nutzer jetzt alle drei Befehle, die das Vorschaufenster angehen: einblenden, Zwischenablage zeigen, Fokus hinein. Unter „Leiste und Fokus" stünde er zwischen fünf Lesezeichenbefehlen.

Die Gegenposition wäre, alle drei Fokusbefehle beieinanderzuhalten. Sie verliert gegen den Grundsatz, der in der Datei schon steht; der Bereichsname „Leiste und Fokus" meint den Wechsel zwischen Leiste und Dateifenster aus C5, und das ist nachgeführt.

## Die Frage, die der Nutzer nicht gestellt bekommen hat

**Blendet `fokus_vorschau` eine ausgeblendete Vorschau ein? Ja.** Die Antwort folgt aus beiden Entscheiden zusammen und ist so gebaut.

Was der Nutzer gewählt hat, ist keine Aussage über eine Taste, sondern über einen Befehlstyp: wer den Fokus in einen Bereich verlangt, verlangt damit, ihn zu sehen. Für die Vorschau davon abzuweichen hieße, `shift+cmd+y` stumm abzuweisen, während `shift+f3` dasselbe Fenster hervorholt und `shift+cmd+l` seine Leiste — drei Befehle auf denselben zwei Randbereichen mit zwei Antworten. Das ist die Sorte Sonderregel, die „supersimpel" ausschließt.

Dazu kommt: für die Vorschau ist die Asymmetrie im Spec **bereits beschlossen**. C10 sagt für `shift+f3` ausdrücklich, dass es die Vorschau einblendet und nie ausblendet. Der Fokusbefehl fügt dort nichts Neues hinzu, er nimmt dieselbe Zeile.

Widersprechen kann der Nutzer an einer Stelle: `holt_hervor` in `crates/krk-ui/src/kommandos/fokus.rs`. Ein `Fokus::Vorschau => None` dort kehrt die Antwort für die Vorschau um, ohne die Leiste zu berühren.

## Die Kennung des neuen Kommandos

**`fokus_vorschau`**, in `Kommando::KENNUNGEN` (`crates/krk-core/src/tasten/belegung.rs:363`). Die Aufzählung ist von 52 auf 53 Einträge gewachsen.

Der Eintrag in `resources/default-keymap.toml` fehlt noch — die Datei gehört dem `ontocoder`. Der Block, den sie braucht, steht wörtlich im Issue `260807-0922_o_das-kommando-fokus-vorschau-steht-im-code-und-noch-nicht-in-der-auslieferungsbelegung.md`, samt Beleg, dass `shift+cmd+y` sonst nirgends vergeben ist.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/tasten/belegung.rs` | `Kommando::FokusVorschau` neu (`:295`), mit der Begründung der Taste im Doc; `KENNUNGEN` von 52 auf 53 (`:363`); `wirkungsbereich` ordnet ihn zu `Ueberall` und begründet, warum alle drei Fokusbefehle dort stehen müssen (`:425-431`); Modulkopf: „zwei fokussierbare Bereiche" war seit S19 falsch |
| `crates/krk-ui/src/kommandos/fokus.rs` | `holt_hervor` neu (`:124`) mit beiden Nutzerentscheiden im Doc; Modulkopf um die Gegenrichtung ergänzt; `Fokus::Vorschau`-Doc nachgeführt („einen Tastenbefehl gibt es noch nicht" stimmt nicht mehr); drei Proben neu |
| `crates/krk-ui/src/fenstermodell.rs` | `einblenden` neu (`:257`), die eine Stelle der Asymmetrie; zwei Proben neu (`:537`, `:560`) |
| `crates/krk-ui/src/appkit/anwendung.rs` | `fokus_holen` neu (`:1053`), der Weg aller drei Fokusbefehle; `bereich_einblenden` neu (`:1642`); `nach_dem_sichtbarkeitswechsel` aus `bereich_umschalten` herausgezogen (`:1658`); `zwischenablage_ansehen` nimmt denselben Weg statt eigener vier Zeilen (`:799`); `Kommando::FokusVorschau` in der Zuleitung (`:1510`), `FokusLeiste` und `FokusDateifenster` von `fokus_setzen` auf `fokus_holen` umgestellt; zwei Doc-Blöcke nachgeführt |
| `crates/krk-ui/src/belegungsmodell.rs` | `bereich_des_kommandos` ordnet `FokusVorschau` zu `Funktionsbereich::Vorschau` ein (`:198`); die Doc-Zeilen der beiden betroffenen Bereiche nachgeführt |

`resources/default-keymap.toml` ist **nicht** geändert. Spec und Plan sind **nicht** geändert.

## Proben

Fünf neue, alle ohne Fenster:

- `der_fokusbefehl_in_die_vorschau_wirkt_aus_jedem_bereich` (`fokus.rs:293`) — die Kennung `fokus_vorschau` löst zu `Kommando::FokusVorschau` auf, und der Befehl wirkt aus jedem der vier Fokuszustände. Ohne die zweite Hälfte wäre er aus der Leiste heraus nicht erreichbar.
- `jeder_fokusbefehl_holt_seinen_bereich_hervor` (`fokus.rs:316`) — die Zuordnung Fokusort → hervorzuholender Bereich, alle vier Werte einzeln.
- `ein_fokusbefehl_auf_einen_ausgeblendeten_bereich_blendet_ihn_ein` (`fokus.rs:333`) — beide Hälften des Entscheids an einem Stück, für Leiste und Vorschau: ausgeblendeter Bereich, `holt_hervor` nennt ihn, `einblenden` holt ihn hervor, danach ist die Bedingung erfüllt, unter der `fokus_setzen` den Fokus hineinlässt. Das Setzen selbst braucht ein Fenster und steht nicht hier.
- `das_einblenden_holt_hervor_und_blendet_nie_aus` (`fenstermodell.rs:537`) — für die drei ausblendbaren Bereiche beide Richtungen. Die zweite Hälfte trägt die Zusage: ohne sie wäre aus dem Fokusbefehl ein zweites Umschalten geworden.
- `das_letzte_dateifenster_ist_immer_schon_eingeblendet` (`fenstermodell.rs:560`) — die Begründung dafür, dass `holt_hervor(Fokus::Dateifenster)` nichts liefert.

## Abnahme

**`make check` ist rot, an genau einer Zusage, und der Grund ist die Arbeitsteilung.**

`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (`crates/krk-core/src/tasten/belegung.rs:993`) ist die Brücke zwischen `Kommando::KENNUNGEN` und `resources/default-keymap.toml`. Der Code trägt `fokus_vorschau`, die Belegungsdatei noch nicht, und die Prüfung tut genau das, wofür es sie gibt.

Der Baum lässt sich nicht in einem Schritt grün halten: der Code muss vor der Datei stehen (eine Kennung ohne Kommando wäre eine Funktion, die in der Belegungsansicht steht und nichts tut), und die Datei gehört dem `ontocoder`. Der rote Punkt zwischen beiden Schritten ist der Preis der Trennung nach Dateibesitz.

**Nachgewiesen ist die Umsetzung mit einem Probelauf.** Der `[[funktion]]`-Block aus dem Issue wurde vorübergehend in `resources/default-keymap.toml` gesetzt, `make check` lief damit in allen vier Kommandos grün (Bau, 522 Proben über alle Ziele, keine fehlgeschlagene, Clippy mit `-D warnings`, `fmt --check`), und der Block wurde danach wieder entfernt. `shift+cmd+y` löst dabei keinen Konflikt aus — die Konflikterkennung hätte den Bau der eingebetteten Auslieferungsbelegung sonst abgebrochen.

Nach dem Nachtrag des `ontocoder` ist `make check` ohne weiteres Zutun grün.

## Was daneben auffiel

**Die dritte genannte Fallunterscheidung ist keine.** `schiebt_auffrischung_auf` läuft über `operation::Art` und nicht über `Kommando`; ein neues Kommando hält dort nichts an. Der Auftrag ging von drei Stellen aus, es sind zwei. Nichts zu tun, aber die Annahme sollte nicht in den nächsten Auftrag mitwandern.

**Der Spec braucht drei Nachzüge, und der `planner` fährt sie parallel.** Beim Bauen gesehen und hier festgehalten, damit sie nicht durchrutschen:

- **C2, viertes Abnahmekriterium:** „Ein Tastenbefehl wirkt dann und nur dann, wenn der Eingabefokus in einem Dateifenster oder in der Lesezeichenleiste steht." Der Satz kennt den dritten Bereich nicht. Seit S19 wirken die vier Tabbefehle auch bei Fokus in der Vorschau, und seit dieser Umsetzung führt ein Tastenbefehl dorthin. Der Satz ist heute falsch, nicht bloß unvollständig.
- **C5:** sagt „Ein Tastenbefehl setzt den Eingabefokus in die Leiste" und schweigt zum ausgeblendeten Fall. Der Entscheid vom 260807 füllt die Lücke; der Spec sollte sie geschlossen zeigen.
- **C7:** hat die Sichtbarkeit bisher allein bei den Umschaltbefehlen. Es gibt jetzt drei Befehle außerhalb von C7, die einen Bereich einblenden können (`shift+f3`, `shift+cmd+l`, `shift+cmd+y`), und keiner blendet aus. Das ist eine Aussage über C7 und gehört dorthin.

**Kein Stilprofil für Langform geladen.** `fusion-rules coder` gab `stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`; dieser Bericht folgt dem Hausstil der übrigen Historie.
