# Gilt die strenge Bestandsregel auch für `session.toml` und `keymap.toml`?

---
**Domain:** code
**Filed by:** bugfixer
**Cross-references:** `shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`, `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1204_*_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`, `crates/krk-core/src/ablage/pfade.rs` (`Datei::leerbefund`), `crates/krk-core/src/ablage/mod.rs` (Abschnitt „Beschädigt heißt nicht ‚ungültiges TOML'")

---

## Question

Der Ladeweg der Ablage stellt seit dem 260821 die weitere Frage: nicht mehr „ist das gültiges
TOML", sondern „hat die gelesene Datei den Bestand hergegeben, den sie trägt". Sie zerfällt in
zwei Hälften, und die zweite ist je Datei zu beantworten.

Die erste Hälfte, **ein oberster Schlüssel, den der Leser nicht kennt**, ist mit
`#[serde(deny_unknown_fields)]` an der jeweiligen Struktur beantwortet. `Belegungsdatei`
(`crates/krk-core/src/tasten/belegung.rs:1587`) und `Einstellungsdatei`
(`crates/krk-core/src/ablage/einstellungen.rs:125`) tragen ihn seit jeher, `Lesezeichenliste`
(`crates/krk-core/src/ablage/lesezeichen.rs`) seit dem 260821. **`Sitzung`
(`crates/krk-core/src/ablage/sitzung.rs:319-321`) trägt ihn nicht.**

Die zweite Hälfte, **kein einziger oberster Schlüssel**, beantwortet `Datei::leerbefund`
(`crates/krk-core/src/ablage/pfade.rs`) je Datei. Heute trägt allein `bookmarks.toml` dort
`Leerbefund::Beschaedigt`; `keymap.toml`, `session.toml`, `settings.toml` und die zwei Zettel
tragen `Leerbefund::Vorgabe`. Die Antwort für `bookmarks.toml` ist gemessen: eine leere
`Lesezeichenliste` serialisiert zu `eintraege = []`, also zu einem obersten Schlüssel, und eine
Datei ohne einen einzigen kann deshalb nicht aus KRKs Feder stammen.

Für `session.toml` und `keymap.toml` ist dieselbe Messung **nicht gemacht**, und der
Defektdatensatz sagt ausdrücklich, die Regel sei dort zu entscheiden und nicht abzuleiten. Für
`settings.toml` gilt sie nicht: die Datei läuft nie über `Zugang::sichern`, also kann kein
Schreibvorgang ihren Bestand fortnehmen.

Der Lauf vom 260821 hat für die drei übrigen die Fassung gesetzt, die **nichts am Verhalten
ändert**. Das ist eine Vorläufigkeit und keine Antwort: wer die Frage nie stellt, hat sie der
Sache nach mit „nein" beantwortet, ohne dass es jemand aufgeschrieben hätte.

## Options

1. **Es bleibt bei `Leerbefund::Vorgabe` für alle drei, und `Sitzung` bekommt kein
   `deny_unknown_fields`.** Die heutige Fassung wird zur Antwort erklärt.
   - Pro: `keymap.toml` und `settings.toml` ändert der Nutzer von Hand, und ein Leerräumen bis
     auf die Kommentare ist dort eine plausible Handlung mit klarer Bedeutung, nämlich „nimm
     die Vorgabe". Eine Schadensmeldung darauf wäre falsch. `session.toml` ist an jeder
     Struktur mit `#[serde(default)]` ausdrücklich auf Nachsicht gegenüber einer älteren
     Fassung gebaut; drei Proben in `crates/krk-core/tests/ablage.rs` (`:511`, `:551`, `:611`)
     verlassen sich darauf.
   - Contra: Der Sitzungszustand ist der Bestand, den der Nutzer am häufigsten unbemerkt
     verlöre — Ordner, Tabs, geöffnete Editordatei. `session.toml` geht über `Zugang::sichern`
     und wird im Takt geschrieben, also greift genau der Verlauf, gegen den die Runde 6 gebaut
     hat.

2. **`session.toml` wird streng, `keymap.toml` und `settings.toml` bleiben nachsichtig.**
   `Sitzung` bekommt `deny_unknown_fields`, `Datei::Sitzung` bekommt
   `Leerbefund::Beschaedigt`.
   - Pro: Trennt nach dem einen Kriterium, das die Sache trägt: schreibt KRK die Datei selbst,
     oder pflegt der Nutzer sie von Hand? Eine Datei, die KRK schreibt, hat eine bekannte
     Gestalt, und jede Abweichung davon ist ein Befund.
   - Contra: `deny_unknown_fields` an `Sitzung` bricht die Rückwärtsrichtung, die
     `#[serde(default)]` heute nur in einer Richtung sichert: eine `session.toml`, die eine
     **spätere** Fassung von KRK mit einem neuen obersten Feld geschrieben hat, gälte in einer
     früheren als beschädigt. Beim Zurückspringen auf eine ältere Version verlöre der Nutzer
     seine Sitzung — mit Sicherung und Meldung, aber er verlöre sie.

3. **Alle vier werden streng.** Auch `keymap.toml` und `settings.toml`.
   - Pro: Eine Regel statt dreier, und die Frage steht nicht bei jeder neuen Ablagedatei erneut
     an.
   - Contra: Nimmt dem Nutzer die Möglichkeit, eine von Hand gepflegte Datei leerzuräumen, ohne
     sie zu löschen. Für `settings.toml` schützt sie gegen nichts, weil kein Schreibvorgang
     dort etwas fortnehmen kann.

## Constraints

- Kein zweiter Mechanismus neben `Zugang::laden` und `atomar::beiseitepfad`. Beide Hälften der
  Regel münden heute in denselben Zweig `Grund::Beschaedigt` und damit in
  `Zugang::beiseite_legen`; jede Antwort hält das ein.
- Die Antwort steht je Datei sichtbar in `Datei::leerbefund`, einer vollständigen
  Fallunterscheidung ohne Auffangzweig. Eine siebte Ablagedatei hält den Bau dort an.
- Was die strenge Lesart für `session.toml` kostet, ist an einer Messung zu entscheiden und
  nicht am Papier: schreibt KRK je eine `session.toml` ohne obersten Schlüssel? Für
  `bookmarks.toml` ist die entsprechende Messung gefahren und steht als Probe
  `eine_leere_liste_steht_als_oberster_schluessel_in_der_datei` im Baum.

## Recommendation

Option 2, aber erst nach der Messung, die die dritte Bedingung nennt, und mit einer Probe für
die Rückwärtsrichtung. Der Sitzungszustand ist der Bestand mit der höchsten Schreibfrequenz und
damit der, den die Lücke am teuersten macht; die von Hand gepflegten Dateien dagegen haben eine
legitime leere Gestalt, die `bookmarks.toml` nicht hat.

---
Abgleich 260821-1532 (reconciler, Baumstand `4e810f9`): **offen, und eine Randbedingung ist seit
`d771ec6` überholt. Der Marker ist nicht bewegt worden.**

**Die Frage selbst steht unverändert.** `Datei::leerbefund`
(`crates/krk-core/src/ablage/pfade.rs:234-241`) gibt weiter `Leerbefund::Beschaedigt` allein für
`Datei::Lesezeichen` zurück; `keymap.toml`, `session.toml`, `settings.toml` und die zwei Zettel
tragen `Leerbefund::Vorgabe`. `Sitzung` (`crates/krk-core/src/ablage/sitzung.rs`) trägt weiter
kein `#[serde(deny_unknown_fields)]`. Die Messung, die die dritte Randbedingung verlangt —
schreibt KRK je eine `session.toml` ohne obersten Schlüssel —, ist nicht gefahren. Es gibt keine
Antwort, also bleibt der Marker `_o_`.

**Die erste Randbedingung stimmt seit `d771ec6` nicht mehr.** Sie lautet: „Beide Hälften der
Regel münden heute in denselben Zweig `Grund::Beschaedigt` und damit in
`Zugang::beiseite_legen`; jede Antwort hält das ein." Der Zweig `Grund::Beschaedigt` trägt beide
Hälften weiter, der Weg dahinter trennt sie: die Hälfte „kein einziger oberster Schlüssel" gibt
seit `d771ec6` `Beiseite::Nicht` zurück und ruft `beiseite_legen` nicht mehr
(`crates/krk-core/src/ablage/mod.rs:607-624`). Gesichert wird allein die Hälfte, in der der Leser
einen Fehler über einen Text meldet, der einen Bestand tragen kann.

**Was das für die Optionen ändert, und was nicht.** Die Zusage „kein zweiter Mechanismus" hält
weiter — es ist eine Zeile weniger geworden und keine Stelle mehr, und
`nur_benannte_dateien_erreichen_das_atomare_schreiben` zählt unverändert fünf. Die Rechnung von
Option 2 verschiebt sich dagegen: eine streng gestellte `session.toml` ohne obersten Schlüssel
bekäme eine Meldung und den Auslieferungszustand, aber **keine Sicherung**. Der Grund, mit dem
die Empfehlung Option 2 trägt — „der Sitzungszustand ist der Bestand mit der höchsten
Schreibfrequenz und damit der, den die Lücke am teuersten macht" —, ist damit für diese Hälfte
schwächer, als er am 260821-0142 war. Für die andere Hälfte, `deny_unknown_fields`, ist er
unberührt: dort wird weiter gesichert.

Verwandt: `shared/issues/260821-1401_*_der-leerbefund-zweig-verschweigt-eine-dastehende-sicherung-die-den-bestand-traegt.md`
— derselbe Zweig, eine Auskunft, die mit derselben Zeile weggefallen ist.

---
Answered: 260905-2008-orchestrator-session.md `## Fuenf weitere Entscheidungen am 260907-1355 beantwortet` — Moeglichkeit 2: die Sitzungsdatei wird streng, Tastenbelegung und Einstellungen bleiben nachsichtig — **aber erst nach der Messung, die der Datensatz verlangt** (schreibt KRK je eine Sitzungsdatei ohne obersten Eintrag?). Die Rueckwaertsrichtung braucht dabei eine Probe: eine Sitzungsdatei aus einer spaeteren Fassung darf in einer aelteren nicht die Sitzung kosten; ruled by user, Kai Stalmann <kai@stalmann.org>.

---
Messung 260907-1407 (coder, Baumstand `c3ade60`): **nein, KRK schreibt keine
`session.toml` ohne obersten Schlüssel.** Jeder Schreibweg der Datei mündet in
`Sitzungsschreiber::schreiben` → `Zugang::sichern(Datei::Sitzung, …)` → `toml::to_string`;
der Messmodus geht ausdrücklich denselben Weg, `krk-bench` spielt gelesene Bytes zurück
statt zu serialisieren, und `nur_benannte_dateien_erreichen_das_atomare_schreiben` hält
fest, dass keine weitere Datei des Baums an `atomar::schreiben` herankommt. Die ärmste
überhaupt konstruierbare `Sitzung` serialisiert zu sechs obersten Schlüsseln: `aktiv` und
`zettel` tragen kein `skip_serializing_if`, die drei Tische und die Tischfolge
`[[fenster]]` stehen unbedingt daneben. Eine leere Tabliste, ein fehlender Ordner, ein
Abbruch mitten im Schreiben und der Erstlauf ändern daran nichts — der Text entsteht ganz
im Speicher, bevor etwas auf die Platte geht, und die fehlende Datei ist der erste Start.
Die Messung steht als Probe
`jede_geschriebene_session_toml_traegt_einen_obersten_schluessel` im Baum.

---
Implemented: 260907-1407-coder-die-messung-vor-der-strengen-sitzungsdatei-und-iconutil.md
— `Datei::Sitzung` trägt `Leerbefund::Beschaedigt`; die Rückwärtsrichtung hält die Probe
`eine_session_toml_aus_einer_spaeteren_fassung_behaelt_ihre_sitzung`.

**Eine Hälfte von Möglichkeit 2 ist dabei nicht gebaut, und das folgt aus der Antwort
selbst.** `Sitzung` bekommt **kein** `#[serde(deny_unknown_fields)]`: die Antwort bindet die
Strenge an die Bedingung, dass eine `session.toml` aus einer späteren Fassung in einer
früheren die Sitzung nicht kostet, und genau das kostet die Marke — dieser Datensatz nennt
es unter Contra seiner eigenen Möglichkeit 2. Die zwei Hälften der Bestandsregel greifen
`session.toml` deshalb verschieden weit, und der Schnitt ist entscheidbar: einen
**fehlenden** obersten Schlüssel schreibt KRK nie, einen **unbekannten** schreibt vielleicht
die nächste Fassung. Ob eine Fassungsangabe in der Datei die erste Hälfte doch erlaubte,
ist die eigene Frage
`260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md`.

**Was der Nachtrag vom 260821-1532 vorhergesagt hat, ist eingetreten:** der Hauptgrund
dieses Datensatzes trägt für die gebaute Hälfte schwächer, als er bei seiner Ablage tat.
Seit `d771ec6` wird eine Datei ohne obersten Schlüssel nicht mehr gesichert, sondern nur
gemeldet; der Gewinn ist eine Meldung und keine Sicherung. Für die andere Hälfte,
`deny_unknown_fields`, wäre er unberührt gewesen — die ist aber nicht gebaut.
