# Wo merkt sich KRK, für welche Fassung es die Neuerungen schon gemeldet hat?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@qantr.com>
**Cross-references:** `260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md` — die offene Frage, die dieselbe Datei betrifft; `260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md` — trennt die drei von Hand gepflegten Dateien von den zwei geschriebenen; `crates/krk-core/src/ablage/pfade.rs` (`Datei::ALLE`, `format`, `leerbefund`, `ersatz`), `crates/krk-core/src/ablage/sitzung.rs` (`Sitzung`, `Sitzungsschreiber`), `crates/krk-core/tests/baum.rs` (`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`)

---

## Question

Die Directive dieser Runde sagt zu, die Startzeile erscheint **einmal je Fassung**. Damit
braucht KRK einen Merker: die Versionsnummer, für die es zuletzt gemeldet hat. Die eigene
Nummer liegt zur Übersetzungszeit über `env!("CARGO_PKG_VERSION")` bereit; der Merker muss
den Prozess überleben und gehört auf die Platte.

**Die drei gemeldeten Dateien scheiden aus**, und zwar aus der Zusage der Runde selbst: KRK
schreibt `keymap.toml`, `settings.toml` und `readers.toml` im Betrieb nicht. Übrig bleiben
die zwei Ablagedateien, die KRK selbst schreibt und die TOML tragen — `bookmarks.toml`
führt Ordnerverweise und kommt inhaltlich nicht in Frage —, also `session.toml`, oder eine
achte Ablagedatei.

**Entscheidbar ist die Frage in beiden Fällen.** „Habe ich für diese Fassung schon
gemeldet?" ist aus einem abgelegten Wert und der eingebackenen Zahl zu beantworten, und aus
sonst nichts: es gibt keine andere Eingabe, aus der sich das erschließen ließe. Ein dritter
Weg, der nichts ablegt und die Antwort aus dem Bestand errechnet, existiert deshalb nicht.

## Options

1. **`session.toml` bekommt ein oberstes Feld**, etwa `gemeldete_fassung`, das der
   Sitzungsschreiber mitschreibt und der Start gegen `env!("CARGO_PKG_VERSION")` hält.
   - Pro: Keine achte Ablagedatei. `Sitzung` trägt `#[serde(default)]`, eine `session.toml`
     aus der Zeit davor bleibt lesbar, und der Weg ist derselbe, den `editor`, `zettel` und
     der Git-Anteil schon gegangen sind. Kein neuer Schreibpfad: der Zwei-Sekunden-Takt
     schreibt die Datei ohnehin.
   - Contra, und das ist der schwere: **die Zusage „einmal je Fassung" gilt dann nur für
     die Instanz, die das Sitzungsrecht hält.** `Sitzungsschreiber::neu` gibt ohne das Recht
     `None` zurück, und keine andere Stelle schreibt `session.toml`. Eine zweite Instanz
     schreibt den Merker nie und meldet damit bei **jedem** Start. Das ist kein Randfall:
     die weitere Instanz ist ein gebauter Befehl aus der Runde 7.
   - Contra: Der Merker ist kein Sitzungszustand. `session.toml` hält, was sich beim
     Arbeiten ändert — Tabs, Aufteilung, offener Zettel. Was KRK dem Nutzer einmal gesagt
     hat, ändert sich beim Einrichten und nicht beim Arbeiten; die Aufnahmeregel im Kopf von
     `resources/default-settings.toml` zieht genau diese Linie.
   - Contra: Es baut die halbe Möglichkeit 2 der offenen Frage `260907-1407` und beantwortet
     sie damit halb, ohne sie zu beantworten. Dort ist eine `fassung` gemeint, die sagt,
     **welche Fassung diese Datei geschrieben hat**; hier eine, die sagt, **für welche
     Fassung gemeldet wurde**. Die beiden Werte laufen auseinander, sobald eine Fassung
     startet, nichts meldet und schreibt. Zwei ähnliche Felder nebeneinander wären die
     zweite Wahrheit, die dieser Baum überall meidet.

2. **Eine achte Ablagedatei**, die trägt, was KRK dem Nutzer schon gesagt hat, mit einem
   Schlüssel `gemeldete_fassung`. Sie wird von KRK geschrieben und trägt deshalb
   `Format::Toml`, `Leerbefund::Beschaedigt`, `Ersatz::Auslieferungszustand`.
   - Pro: Der Merker steht dort, wo er hingehört, und `session.toml` bleibt Sitzungszustand.
     Die offene Frage `260907-1407` bleibt unangetastet und wird auf ihre eigenen Gründe
     beantwortet statt als Nebenwirkung.
   - Pro: **Die Zusage gilt für jede Instanz.** Geschrieben wird über `Zugang::sichern`
     unter der Schreibsperre, und die nimmt jede Instanz. Die erste, die meldet, schreibt
     den Merker; die zweite liest ihn und meldet nicht.
   - Pro: Der Preis ist gehalten und nicht geraten. `Datei::ALLE` ist `[Datei; 7]`, also
     hält der Übersetzer die Länge an; `format`, `leerbefund` und `ersatz` sind vollständige
     Fallunterscheidungen und erzwingen je eine Antwort; und die Probe
     `keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`
     (`crates/krk-core/tests/baum.rs`) wird rot und **nennt jede Prosastelle, die
     nachzuziehen ist**. Der Baum sagt den Umfang, keine Schätzung.
   - Contra: Eine achte Datei im Ablageordner, die einen einzigen Wert trägt. Der Nutzer
     sieht sie, wenn er den Ordner öffnet, und sie erklärt sich nicht von selbst.
   - Contra: Etliche Prosastellen unter `crates/krk-core/src/ablage/` zählen heute sieben
     Ablagedateien und fünf TOML-Dateien und sind nachzuziehen. Die Zahl der Stellen sagt
     die Probe beim ersten roten Lauf.
   - Contra: Ein Löschwerkzeug nimmt den Ablageordner mit (der Fall vom 17.08.). Dann ist
     der Merker weg und die Meldung kommt ein zweites Mal. Das trifft Möglichkeit 1 genauso
     und ist deshalb kein Unterschied zwischen beiden.

3. **Kein Merker: die Zeile erscheint bei jedem Start.**
   - Pro: Nichts zu bauen, nichts zu pflegen.
   - Contra: Bricht die Directive dieser Runde („sie erscheint einmal je Fassung"). Und eine
     Zeile, die bei jedem Start dasteht, wird nach dem dritten Start nicht mehr gelesen.
     Aufgeführt, damit die Alternative benannt ist, nicht als Kandidat.

## Constraints

- KRK schreibt `keymap.toml`, `settings.toml` und `readers.toml` im Betrieb nicht. Diese
  Zusage bleibt unangetastet; sie schließt die drei als Ort des Merkers aus.
- `Datei::leerbefund` und `Datei::ersatz` bleiben vollständige Fallunterscheidungen ohne
  Auffangzweig.
- Der Merker gehört auf den Vergleichspfad **vor** der Erhebung: solange die abgelegte
  Fassung der laufenden gleicht, wird gar nicht verglichen. Das ist die Bedingung, unter der
  die Runde die Zeitzusage L4 („Prozessstart bis bedienbare Prüfsitzung", 1000 ms) im
  Dauerbetrieb nicht belastet, und sie gilt für jede der beiden Möglichkeiten.
- Was der Nutzer im Ablageordner sieht, ist Teil der Antwort: die Betriebsregel „die neue
  Fassung über die alte kopieren und die alte nicht vorher löschen" steht an drei Stellen
  für ihn, und eine achte Datei kommt in seinen Blick.

## Recommendation

**Möglichkeit 2, die achte Ablagedatei.** Der Ausschlag ist die zweite Instanz: unter
Möglichkeit 1 fällt die Zusage „einmal je Fassung" für jede Instanz ohne Sitzungsrecht
still aus, und ein still ausfallender Teil einer Zusage ist genau das, was dieser Baum an
`bookmarks.toml` schon einmal bezahlt hat. Dazu kommt, dass der Merker kein
Sitzungszustand ist und dass Möglichkeit 1 die offene Frage `260907-1407` halb beantwortet,
statt sie zu beantworten.

Der Preis von Möglichkeit 2 ist die einzige Größe hier, die niemand schätzen muss: der
Übersetzer hält drei Stellen an, und die Probe
`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien` nennt beim ersten
roten Lauf jede Prosastelle namentlich. Ein bekannter, aufgezählter Preis ist billiger als
eine Zusage mit einem Loch darin.

Für den Namen der Datei liegt `reported.toml` nahe, in der englischsprachigen
Kleinschreibung der sechs anderen, mit einem Schlüssel `gemeldete_fassung`. Der Name ist
Teil der Antwort und nicht schon entschieden.

---
Answered: `260905-2008-orchestrator-session.md` `## Fortsetzung 260910 — die Runde 24 wird geplant` — Möglichkeit 2: eine achte Ablagedatei trägt den Merker `gemeldete_fassung`, geschrieben unter der Schreibsperre und damit für jede Instanz gültig; `session.toml` bleibt Sitzungszustand und die offene Frage `260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md` unangetastet; ruled by user, Kai Stalmann <kai@stalmann.org>.

---
Implemented: 70f0d84 — `reported.toml` als achte Ablagedatei, geschrieben und gelesen über `Zugang::sichern` und `Zugang::laden` und damit unter der Schreibsperre, die jede Instanz nimmt; `session.toml` ist unangetastet, und die offene Frage `260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md` bleibt auf ihre eigenen Gründe zu beantworten.
