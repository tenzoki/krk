# Die Zeichengleichheit der zwei Werkbankpaare wird je Durchsicht von Hand gemessen und von nichts gehalten

---
**Domain:** data
**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml` (die vier Blöcke `fusion-Werkbank: die Wurzel`, `Projektwurzel mit fusion-Werkbank`, `flight-Werkbank: die Wurzel`, `Projektwurzel mit flight-Werkbank` und die Doppelungshinweise darüber); `shared/issues/260825-2126_c_der-doppelungshinweis-steht-bei-flight-nur-ueber-einem-der-beiden-bloecke.md` (der `Resolved:`-Vermerk misst die Gleichheit von Hand); `shared/history/260826-0810-ontocoder-die-vier-flight-speicher-tragen-jetzt-ihre-datumszeile.md` (Abschnitt „Die zwei Blöcke laufen nicht auseinander")

---

## Was ist

Die Profildatei führt zwei Paare von Blöcken, die dieselben Zeilen zweimal tragen: das
fusion-Wurzelprofil und die fusion-Projektwurzel mit je sieben Zeilen, das
flight-Wurzelprofil und die flight-Projektwurzel mit seit `180fc53` je elf. Die Datei
sagt über beide Paare ausdrücklich, dass sie auseinanderlaufen können und nichts sie
aneinanderhält, und nennt den Hinweis über beiden Blöcken als die gewählte Antwort.

Damit ist die Gleichheit eine Behauptung, die jemand nachmisst — und sie ist inzwischen
dreimal von Hand nachgemessen worden: einmal im Datensatz `260825-2126` für den Stand
davor, einmal vom Ontocoder nach `180fc53`, einmal von dieser Durchsicht. Alle drei
Messungen fahren dieselbe Rechnung: das vorangestellte `fusion-workbench/`
beziehungsweise `flight-workbench/` wegnormalisieren und vergleichen. Alle drei sind
grün; nach `180fc53` sind beide Paare zeichengleich.

## Warum das zählt

Der Hinweis über den Blöcken warnt den Bearbeiter. Er greift nur, wenn der Bearbeiter ihn
liest, und er greift überhaupt nicht bei dem Fall, für den eine Vorkehrung da sein
müsste: eine Änderung, die einen der beiden Blöcke trifft und den anderen vergisst, fällt
nirgends auf. Die Datei lädt weiter, `pruefen` beanstandet nichts, jede Probe bleibt grün,
und die zwei Profile beantworten dieselbe Frage verschieden — an der Werkbankwurzel
anders als an der Projektwurzel darüber.

Die Datei begründet ihre Zurückhaltung mit „eine Vererbung oder eine Vorlage wäre ein
neuer Mechanismus", und das ist richtig und bleibt es: eine Vorlage im Datenformat wäre
eine zweite Maschine für zwei Blöcke. Eine Probe ist beides nicht. Sie fasst die Datei
nicht an, ändert am Format nichts und kostet zur Laufzeit nichts — sie hält bloß die
Rechnung fest, die drei Durchsichten von Hand gefahren haben.

## Was zu tun wäre

Eine Probe in `crates/krk-core/tests/leseprofil.rs`, die über `AUSLIEFERUNGSTEXT` läuft,
je Paar die zwei Blöcke ausschneidet, im zweiten `ordner = "<werkbank>", ` streicht und
`ordner = "<werkbank>/` auf `ordner = "` zurückführt, und danach auf Gleichheit prüft.

Die zweite Ersetzung ist die, an der die naive Fassung scheitert: die Feldzeilen tragen im
Wurzelprofil **keine** Ortsangabe und bekommen im Projektwurzelprofil den ganzen
Schlüssel `ordner` dazu, sie werden ihm also nicht vorangestellt. Fünf der sieben
fusion-Zeilen und drei der elf flight-Zeilen sind von dieser Art; wer nur das Präfix
normalisiert, bekommt eine Probe, die an einem gesunden Stand rot ist. Siehe dazu den
Datensatz `260826-0904_*_der-doppelungshinweis-sagt-vor-der-ortsangabe-…`, der diese
Ungenauigkeit an der Prosa führt.

**Zuständig:** `coder`, denn der Eingriff ist eine Probe und keine Zeile der Profildatei.

**Schwere:** niedrig. Beide Paare stimmen heute; die Vorkehrung fehlt, nicht die
Übereinstimmung.

---
Resolved: Die Probe steht, in `crates/krk-core/tests/leseprofil.rs`:
`die_zwei_bloecke_eines_werkbankpaares_tragen_dieselben_zeilen`, mit dem Helfer
`zeilenbloecke` daneben. Sie laeuft ueber `AUSLIEFERUNGSTEXT`, schneidet je Paar die
Zeilenbloecke aus, normalisiert im zweiten und vergleicht.

**Die zwei Ersetzungen sind beide da**, wie dieser Datensatz sie beschreibt: der ganze
Schluessel `ordner = "<werkbank>", ` faellt an den Feldzeilen weg, die im Wurzelprofil keine
Ortsangabe tragen, und `ordner = "<werkbank>/` geht an den Zaehl- und Juengste-Zeilen auf
`ordner = "` zurueck. Der Doc-Kommentar schreibt beide Gestalten aus und nennt die naive
Fassung, die an einem gesunden Stand rot waere.

Kopf und `kennzeichen` sind ausgenommen und nicht vergessen: dort unterscheiden sich die
zwei Bloecke eines Paares notwendig, das eine erkennt die Werkbankwurzel, das andere den
Ordner darueber. Daneben haelt die Probe, dass im normalisierten zweiten Block der
Werkbankname ueberhaupt nicht mehr vorkommt — eine dritte Gestalt der Ortsangabe wuerde
damit auffallen, statt still durch die Normalisierung zu fallen.

**Gemessen, dass sie nicht leer gruen ist.** Am 260908 mit einer Beschriftung `"Runden!"` im
Projektwurzelprofil gefahren: rot, mit beiden Bloecken im Vergleich. Die Datei ist danach
unveraendert zurueckgestellt. An diesem Stand sind beide Paare zeichengleich.

Der Doppelungshinweis in der Datei bleibt stehen: die Begruendung gegen eine Vererbung oder
Vorlage ist richtig und wird von einer Probe nicht beruehrt.
