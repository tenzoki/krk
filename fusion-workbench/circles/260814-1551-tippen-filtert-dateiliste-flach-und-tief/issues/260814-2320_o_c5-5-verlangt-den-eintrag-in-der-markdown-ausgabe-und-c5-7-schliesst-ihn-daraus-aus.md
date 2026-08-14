# C5.5 verlangt den Eintrag in der Markdown-Ausgabe, und C5.7 schließt ihn daraus aus

**Status:** Open
**Domain:** Spec der Filter-Runde, Fähigkeit C5
**Filed by:** ontocoder, beim Umsetzen von E2
**Related:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C5.5 und C5.7; `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritt E2; `crates/krk-ui/src/belegungsausgabe.rs:178`

## Befund

Zwei Abnahmekriterien derselben Fähigkeit widersprechen sich.

**C5.5** sagt: „Die Belegungsansicht führt ihn, und die Markdown-Ausgabe der Runde 3 führt
ihn mit. **(Probe** über `--tasten-protokoll` und über die Ausgabe**)**"

**C5.7** sagt: „Der Nutzer kann ihm eine Kombination geben, wie jeder Funktion.
Ausgeliefert wird keine."

Die Markdown-Ausgabe nimmt eine Funktion nur auf, wenn sie mindestens eine Kombination
trägt (`crates/krk-ui/src/belegungsausgabe.rs:178`,
`.filter(|funktion| !funktion.tasten().is_empty())`). Eine ab Werk unbelegte Funktion
erscheint dort nicht; die Probe `eine_funktion_ohne_kombination_erscheint_nicht`
(`belegungsausgabe.rs`) hält genau das fest. Solange C5.7 gilt, ist die zweite Hälfte von
C5.5 nicht erfüllbar.

Der Widerspruch ist nicht neu erfunden, sondern derselbe, den die drei Spaltenschalter
schon tragen. Der Kopfkommentar von `resources/default-keymap.toml` schreibt ihn seit dem
260812 aus: „In der Markdown-Ausgabe der Runde 3 stehen sie dagegen nicht." Der Umfang der
Ausgabe ist ein Nutzerentscheid vom 260811-0110, gegen die Empfehlung des Datensatzes
(`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_*_was-steht-in-der-ausgabe-und-wonach-ist-sie-gegliedert.md`,
Möglichkeit 1), und sein Preis steht dort benannt: eine unbelegte Funktion verschwindet
aus der Datei.

## Warum das jetzt auffällt

E2 ist umgesetzt, und die Belegungsansicht führt den Eintrag: sie zeigt jede Funktion, ob
belegt oder nicht. Die erste Hälfte von C5.5 hält also. Die zweite hält nicht und kann
nicht halten, ohne dass entweder eine Kombination ausgeliefert wird (gegen C5.7 und gegen
den Nutzerentscheid vom 260814-1610) oder der Umfang der Markdown-Ausgabe geändert wird
(gegen den Nutzerentscheid vom 260811-0110).

Die Abnahme von C5.5 als „(Probe)" ist damit heute nicht schreibbar. Wer sie schriebe,
schriebe eine Probe, die entweder rot bleibt oder ihre Zusage stillschweigend halbiert.

## Was zu tun ist

Der Spec ist nachzuziehen, nicht der Code. Vorschlag für C5.5:

> Die Belegungsansicht führt ihn. In der Markdown-Ausgabe der Runde 3 steht er nicht,
> solange er ab Werk keine Kombination trägt; weist der Nutzer ihm eine zu, findet er ihn
> danach auch dort. **(Probe** über `--tasten-protokoll` und über die Ausgabe**)**

Damit sagt C5.5 dasselbe wie der Kopfkommentar der Belegungsdatei über die drei
Spaltenschalter, und die Probe ist in beiden Richtungen schreibbar.

`resources/default-keymap.toml` trägt die Aussage bereits im Kommentar über dem neuen
Eintrag; sie steht dort als Auskunft an den Leser der Datei und ersetzt die Berichtigung
des Spec nicht.
