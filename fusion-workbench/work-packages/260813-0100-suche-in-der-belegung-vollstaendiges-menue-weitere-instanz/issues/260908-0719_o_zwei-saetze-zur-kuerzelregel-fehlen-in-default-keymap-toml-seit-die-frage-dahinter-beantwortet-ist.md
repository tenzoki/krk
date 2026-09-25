Zwei Saetze zur Kuerzelregel fehlen in `default-keymap.toml`, seit die Frage dahinter beantwortet ist

---

Die Durchsicht `260813-0532-ontorev-belegungsdatei-weitere-instanz.md` hat zwei Saetze
fuer `resources/default-keymap.toml` empfohlen, und beide sind bis heute nicht geschrieben.
`260813-0416_*_zwei-menueeintraege-mit-cmd-a-und-appkit-nimmt-dem-spaeteren-das-kuerzel.md`
hat sie am 260813 ausdruecklich zurueckgestellt, weil die Nutzerfrage nach der Richtung noch
offen war und die Saetze sonst den falschen Mechanismus beschrieben haetten. Die Frage ist seit
dem 260906-2147 beantwortet und seit `menuemodell.rs:236,339` umgesetzt
(`260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`,
Marker `_i_`). Die Vorbedingung der Zurueckstellung besteht damit nicht mehr.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** data — `resources/default-keymap.toml` ist die eine Quelle jeder Tastenbelegung und
gehoert dem `ontocoder`
**Severity:** Low
**Tree state:** `fe5fe9c` samt den Aenderungen dieses Durchgangs

## Was fehlt

Am 260908 nachgesehen, `grep -n 'zugestellte_kuerzel\|Menuekuerzel' resources/default-keymap.toml`:

1. **Am Konfliktkommentar** (heute `:125-141`, der Absatz zur Nutzerentscheidung vom 260805).
   Er erklaert die Doppelbelegung allein ueber den Fokusvorbehalt: „zwei Funktionen mit
   verschiedenen Zustellern begegnen einander nie". Das traegt seit `16c0924` nicht mehr allein
   — in der **Menueleiste** begegnen sie einander sehr wohl, und dort entscheidet
   `menuemodell::zugestellte_kuerzel`, wer das Kuerzel behaelt. Die zweite Absprache gehoert
   neben die erste.
2. **Am Eintrag `alle_markieren`** (heute `:363-366`, ohne Kommentar). Sein Menueeintrag zeigt
   `Cmd+A` **nicht**, obwohl `Cmd+A` ihn ausloest; der Ereignisabgriff sieht den Tastendruck vor
   dem Menue. Wer den Menueeintrag liest, haelt den Befehl fuer unbelegt.

Beides gilt seit dem 260907 gleichlautend fuer `cmd+f` mit `editor_suchen` und
`filter_einfuegen`; ob der zweite Satz dort ebenso an einen der beiden Eintraege gehoert, ist
Teil derselben Arbeit. Eine Zahl von Doppelungen gehoert in keinen der Saetze — der Modulkopf
von `crates/krk-ui/src/menuemodell.rs` fuehrt aus demselben Grund keine.

## Abnahme

`resources/default-keymap.toml` nennt an beiden Stellen `menuemodell::zugestellte_kuerzel`
beziehungsweise die fehlende Kuerzelanzeige, und `make check` bleibt gruen — die Datei geht
durch `Belegung::auslieferung`, ein Kommentar aendert daran nichts.
