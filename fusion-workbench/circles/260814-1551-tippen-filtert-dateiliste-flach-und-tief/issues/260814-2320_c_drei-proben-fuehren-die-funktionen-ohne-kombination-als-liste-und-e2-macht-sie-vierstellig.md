# Drei Proben führen die Funktionen ohne Kombination als Liste, und E2 macht sie vierstellig

**Status:** Open
**Domain:** Code, Strang E der Filter-Runde
**Filed by:** ontocoder, beim Umsetzen von E2
**Related:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritt E2; `issues/260814-2303_o_e1-und-e2-teilen-eine-zusicherung-die-eine-probe-haelt-und-lassen-den-baum-dazwischen-rot.md`; `resources/default-keymap.toml`

## Befund

E2 ist umgesetzt: `resources/default-keymap.toml` trägt den Eintrag
`tiefe_suche_umschalten` mit `tasten = []` und 84 statt 83 Funktionen.
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, die Probe aus dem
Datensatz `260814-2303`, hält damit wieder.

**Der Baum ist trotzdem rot**, und zwar an drei anderen Stellen. Drei Proben führen die
Funktionen, die ab Werk ohne Kombination ausgeliefert werden, als ausgeschriebene Liste
mit fester Länge. Der vierte Eintrag lässt jede davon fehlschlagen:

| Probe | Ort | Meldung |
|---|---|---|
| `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` | `crates/krk-core/tests/belegung.rs:237` | `tiefe_suche_umschalten traegt keine Kombination` |
| `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | `crates/krk-core/tests/belegung.rs:871` | `TiefeSucheUmschalten ist gebaut, und tiefe_suche_umschalten traegt ab Werk keine Kombination` |
| `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` | `crates/krk-ui/src/belegungsausgabe.rs:566` | `ab Werk sind andere Funktionen unbelegt als die drei Spaltenschalter` |

Die ersten beiden lesen dieselbe Liste, `OHNE_KOMBINATION_AB_WERK: [&str; 3]`
(`crates/krk-core/tests/belegung.rs:107`). Die dritte trägt ihre eigene, als
dreielementiges Literal im Rumpf der Probe. Alle drei sind Rust und gehören dem `coder`;
der `ontocoder` darf sie nicht anfassen.

`make check` steht damit auf `Error 2`. Die dritte Probe steckt hinter dem Abbruch: `make`
hält beim ersten Fehlschlag an, und `krk-ui` kommt in diesem Lauf nicht mehr dran. Sie ist
mit `cargo test -p krk-ui --bin krk belegungsausgabe` einzeln gefahren und bestätigt.

## Warum das nicht der zweite Aufguss von 260814-2303 ist

Der Datensatz `260814-2303` beschreibt den roten Zwischenstand **zwischen** E1 und E2 und
sagt ausdrücklich: „Nach E2 ist der Baum wieder grün." Das stimmt nicht. Der Zwischenstand
aus jenem Datensatz ist geschlossen; hier steht ein Stand **nach** E2, an drei anderen
Proben, aus einem anderen Grund. E2 nennt als einzige Datei
`resources/default-keymap.toml`, und mit dieser einen Datei ist der Baum nicht grün zu
bekommen.

Der Plan trägt die Ursache: Schritt E2 sagt zu, dass die Datei von 83 auf 84 Funktionen
wächst, und keiner der Schritte E1 bis E3 nennt die drei Proben. Die vierte Funktion ohne
Kombination ist der erste Fall dieser Art seit dem 260812, und die Warnung dafür stand
bereit — `crates/krk-core/tests/belegung.rs:105` sagt wörtlich: „Wer eine vierte Funktion
ohne Kombination ausliefert, traegt sie mit ihrem Datensatz hier nach."

## Was zu tun ist

Ein Schritt für den `coder`, drei Stellen, keine davon inhaltlich strittig:

1. `OHNE_KOMBINATION_AB_WERK` auf `[&str; 4]` und `"tiefe_suche_umschalten"` hinein, samt
   dem Datensatz dazu im Kommentar darüber
   (`decisions/260814-1552_a_welche-tastenkombination-schaltet-die-tiefe-suche.md`,
   Nutzerantwort vom 260814-1610). Die Begründung ist eine andere als die der drei
   Spaltenschalter, die dort steht, und gehört daneben und nicht in dieselbe Klammer.
2. Das dreielementige Literal in `crates/krk-ui/src/belegungsausgabe.rs:566` um denselben
   Eintrag erweitern; der Kommentar darüber nennt „die drei Spaltenschalter" und wird zu
   vier Funktionen.
3. Zu erwägen, ob die dritte Probe ihre Liste weiter selbst führt. Sie sagt dieselbe Sache
   wie `OHNE_KOMBINATION_AB_WERK` und steht in einer anderen Kiste, die den Wert nicht
   erreicht: `krk-ui` hat kein Bibliotheksziel, und `crates/krk-core/tests/` ist eine
   eigene Kiste. Eine dritte Stelle für dieselbe Aussage ist damit erklärbar, aber sie ist
   die dritte.

Die Zahlen im Kopf von `resources/default-keymap.toml` sind bereits nachgezogen (84
Funktionen, 90 Kombinationen); `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
hält.

---

**Resolved:** 260814-2333, vom `coder` im Nachzug E2b (kein eigener Planschritt, er fällt
zwischen E1 und E2). Punkt 1: `OHNE_KOMBINATION_AB_WERK` steht auf `[&str; 4]` und trägt
`tiefe_suche_umschalten`; der Kommentar darüber führt den Grund getrennt von dem der drei
Spaltenschalter, weil er ein anderer ist — eine offen gelassene Wahl und keine gesparte
Kombination. Punkt 2: das Literal in `crates/krk-ui/src/belegungsausgabe.rs` trägt
denselben Eintrag, sein Kommentar nennt vier statt drei. Punkt 3 ist nicht entschieden
worden, sondern zur Frage geworden:
`decisions/260814-2326_o_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`,
mit dem Befund, dass die zwei Listen heute nicht dasselbe behaupten — `krk-core` prüft eine
Richtung, `krk-ui` beide. Beide Kommentare zeigen jetzt aufeinander und auf jenen
Datensatz. `make check` ist grün, Exit 0.
