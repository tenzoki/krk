# Der Tastenweg in die Vorschau in der Auslieferungsbelegung (R2c, Turn 26)

**Agent:** ontocoder
**Status:** Complete
**Quellen:**
- `issues/260807-0922_*_das-kommando-fokus-vorschau-steht-im-code-und-noch-nicht-in-der-auslieferungsbelegung.md`
- `decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md`

**Zum Stilprofil:** `fusion-rules ontocoder` gab allein `fusion-workbench/stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`. Für diesen Bericht gilt deshalb kein Langform-Schreibprofil; das Fehlen ist hier vermerkt, wie `rules/agent-setup.md` es verlangt.

---

## Der Eintrag

`resources/default-keymap.toml` führt seit dieser Änderung den dritten Fokusbefehl, im C5-Block hinter `fokus_dateifenster` und vor der Überschrift zu C7:

```toml
[[funktion]]
id = "fokus_vorschau"
name = "Fokus in das Vorschaufenster"
tasten = ["shift+cmd+y"]
```

Der Wortlaut ist der des `coder` aus dem Defekt, unverändert übernommen. Geprüft habe ich ihn an drei Stellen: die Kennung `fokus_vorschau` steht so in `Kommando::KENNUNGEN` (`crates/krk-core/src/tasten/belegung.rs`), der Name folgt der Form der beiden Nachbarn (`Fokus in die Lesezeichen- und Geräteleiste`, `Fokus zurück in das Dateifenster`), und die Kombination steht in der Schreibweise, die der Dateikopf festlegt: `[ctrl+][opt+][shift+][cmd+]<taste>`, in genau dieser Reihenfolge. `shift+cmd+y` hält sie ein.

Dazu kommt ein Kommentar unter dem Block. Er begründet den Buchstaben, wie es der Nutzerentscheid vom 260807 verlangt: die Vorschau trägt in dieser Belegung schon das `y`, denn `cmd+y` blendet sie ein und aus, und der Fokusbefehl erbt ihn, wie `l` und `d` es für die Leiste und das Dateifenster tun. `shift+cmd+v` wäre der naheliegende Buchstabe gewesen und liegt seit S9 beim Verschieben (F6). Der Kommentar sagt außerdem, warum der Eintrag im C5-Block steht und nicht unten bei C7: der Block führt die Fokusbefehle und nicht die Vorschau.

## Der zweite Nachtrag, den der Defekt nicht nannte

Zeile 30 des Dateikopfs sagt, wie viele Funktionen ausgeliefert sind. Sie stand auf `57 Funktionen mit zusammen 64 Kombinationen` und steht jetzt auf `58 Funktionen mit zusammen 65 Kombinationen`. Ohne diese Zeile wäre der Kopf der Datei nach dem Nachtrag falsch gewesen; die Vorgängerfassung dieser Zeile ist am 260805-1811 aus demselben Grund gezogen worden.

Beide Zahlen sind gezählt, nicht fortgeschrieben: die Datei geparst, die Kommentare entfernt, 58 Blöcke `[[funktion]]` und 65 Einträge in den Tastenlisten.

Eine maschinelle Prüfung hängt an den Zahlen nicht. Die Vorgängerin von `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren` schrieb sie als Literal hin und ist genau deshalb umgebaut worden (`crates/krk-core/src/tasten/belegung.rs`, Dokumentationskopf der Prüfung).

## Die Konfliktprüfung, dreifach

**Über die geparste Datei.** 65 Kombinationen, keine doppelte je Zusteller. `shift+cmd+y` steht bei genau einer Funktion. Die einzige weitere Kombination auf `y` ist `cmd+y` bei `vorschau_umschalten`, und die zwölf belegten `shift+cmd+`-Kombinationen sind `a d g h i k l n u v w y`.

**Über KRK eigene Erkennung.** `die_auslieferungsbelegung_ist_konfliktfrei` (`crates/krk-core/tests/belegung.rs`) läuft grün. Diese Prüfung ist die verlangte, weil sie die vier vom Menü zugestellten Einträge mitliest: seit dem Nutzerentscheid vom 260805-0000 stehen `cmd+x`, `cmd+c`, `cmd+v` und `cmd+a` mit `gehalten_von = "menue"` in derselben Datei und ziehen damit in die Erkennung ein. Eine reine Textsuche hätte sie zwar auch gesehen, aber nicht den Zusteller unterschieden, und genau der entscheidet: `cmd+a` steht bei zwei Funktionen und ist kein Konflikt.

**Über das gebaute Hauptmenü.** `make menue` gibt sieben Kürzel aus, `cmd+q`, `cmd+x`, `cmd+c`, `cmd+v`, `cmd+a`, `cmd+n` und `shift+cmd+w`. `shift+cmd+y` ist keines davon. Dieser dritte Weg ist der einzige, der auch die Einträge sieht, die AppKit selbst dazustellt und die in keiner Datei stehen.

## Das Abnahmekriterium von S20 trägt unverändert

Das Kriterium sagt: "nach dem Zurücksetzen stimmt sie mit `resources/default-keymap.toml` überein" (Plan, `#### 20.`).

Es trägt weiter, und zwar aus dem Aufbau heraus. `Belegung::zuruecksetzen` (`crates/krk-core/src/tasten/belegung.rs:683`) setzt die Belegung auf `Belegung::auslieferung()`, und die liest über `include_str!` genau die Datei, in die der Eintrag jetzt gehört. Der Eintrag steht damit auf beiden Seiten des Vergleichs, gleichzeitig und in derselben Reihenfolge; ein Vergleich, der links etwas fände, was rechts fehlt, ist nicht konstruierbar.

Der Befund des `ontorev` vom 260807 bleibt richtig und ist hier nicht die Gefahr: der Umweg über `toml::from_str` und `toml::to_string` verliert Kommentare, aber kein Eintrag entsteht oder verschwindet dabei. `eine_belegung_ueberlebt_schreiben_und_wiedereinlesen` prüft genau diesen Umweg und läuft grün.

## Was offen bleibt und nicht mir gehört

**Der Kommentar zur Tabellenhöhe.** `crates/krk-ui/src/appkit/belegungsansicht.rs:76` sagt "57 Funktionen und neun Bereichsueberschriften" und nennt jetzt eine Zahl zu wenig. Die Konstante `TABELLENHOEHE` steht auf 300,0 Punkten und hängt von der Zahl der Zeilen nicht ab; falsch ist allein die Zahl im Kommentar. `crates/` gehört dem `coder`, deshalb steht die Zeile unangetastet und der Nachtrag als `issues/260807-1015_o_der-kommentar-zur-tabellenhoehe-nennt-57-funktionen-und-die-belegung-fuehrt-58.md`. Eine dritte Stelle mit derselben Zahl gibt es im Baum nicht.

**Der Entscheidungsdatensatz trägt noch den Marker offen.** `decisions/260805-2216_o_tastenweg-des-fokus-in-das-vorschaufenster.md` hat keine `Answered:`-Zeile, obwohl der Nutzer die Frage am 260807 zugunsten der Möglichkeit 1 entschieden hat und Code wie Datei stehen. Der Datensatz gehört zum Nachzug des `planner`; hier steht er als Beobachtung.

**Der C2-Satz im Spec.** Der Datensatz weist auf eine zweite Spannung hin: das vierte Abnahmekriterium von C2 sagt "Dateifenster oder Lesezeichenleiste" und kennt den dritten Bereich nicht. Auch das ist Sache des `planner`.

## Abnahme

`make check` läuft in allen vier Kommandos grün und endet mit `alle vier gruen`. Die beiden Prüfungen, die an diesem Nachtrag hängen, sind namentlich grün:

- `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` — die Brücke zwischen `Kommando::KENNUNGEN` und der Datei, vor dem Nachtrag rot
- `die_auslieferungsbelegung_ist_konfliktfrei` — die Konflikterkennung über die ausgelieferte Belegung

Geändert sind zwei Zeilenbereiche in einer Datei, `resources/default-keymap.toml`. Kein Programmtext, kein Spec, kein Plan.
