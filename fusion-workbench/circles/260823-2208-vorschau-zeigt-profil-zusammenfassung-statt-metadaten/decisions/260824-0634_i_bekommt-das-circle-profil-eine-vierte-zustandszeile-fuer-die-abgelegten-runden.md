# Bekommt das Circle-Profil eine vierte Zustandszeile fuer die abgelegten Runden?

---
**Domain:** code
**Filed by:** planner
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (Festlegung A7, Kriterium C5.6), `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`

---

## Question

Die Festlegung A7 ist am Spec-Tor bestaetigt: der Zustand eines Circles wird ueber den
Baustein „Vorhandensein" ausgedrueckt, mit je einer Zeile fuer vorgesehen, aktiv und
geschlossen. Das Markervokabular der Werkbank kennt aber sechs Zustaende und nicht drei:
`_a_` vorgesehen, `_t_` aktiv, `_c_` kohaerent geschlossen, `_b_` beschraenkt geschlossen,
`_s_` ueberholt und `_d_` zurueckgestellt.

Drei Zeilen decken vier davon ab, wenn die dritte Zeile `_c_` und `_b_` zusammenfasst. Fuer
`_s_` und `_d_` bleibt keine Zeile uebrig, und der Baustein „Vorhandensein" antwortet auf
jede der drei Zeilen mit „nein". Eine so abgelegte Runde saehe in der Zusammenfassung aus wie
eine Runde, ueber deren Zustand die Werkbank nichts sagt.

**Der Fall ist am Bestand belegt und nicht ausgedacht.** `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/`
traegt seit dem 260821-2202 den Marker `_d_`; der Nutzer hat jenen Circle abgesagt. Von den
achtzehn Circle-Verzeichnissen dieser Werkbank faellt heute genau eines aus den drei Zeilen
heraus.

Die Frage gehoert vor die Auslieferungsfassung `resources/default-readers.toml`, weil ihre
Antwort eine Zeile jenes Profils setzt oder nicht setzt. Sie haelt keinen Planschritt auf:
der Plan baut die drei Zeilen aus A7, und eine vierte waere ein Zusatz von zwei Zeilen TOML
ohne eine Zeile Rust.

## Options

1. **Es bleibt bei drei Zeilen** — vorgesehen, aktiv, geschlossen. Eine ueberholte oder
   zurueckgestellte Runde antwortet dreimal mit „nein".
   - Pros: A7 steht unveraendert, so wie der Nutzer sie am Tor bestaetigt hat. Die
     Auslieferungsfassung bleibt kuerzer, und der Bausteinsatz bleibt unberuehrt.
   - Cons: Fuer eines der achtzehn Circle-Verzeichnisse ist die Anzeige stumm. Der Nutzer
     kann „abgelegt" nicht von „die Zeilen dieses Profils passen hier nicht" unterscheiden.
2. **Eine vierte Zeile „abgelegt"** — ein weiterer Vorhandensein-Baustein auf
   `^_[sd]_circle\.md$`.
   - Pros: Jeder der sechs Marker faellt in genau eine Zeile, und die Anzeige ist ueber alle
     achtzehn Verzeichnisse vollstaendig. Der Bausteinsatz bleibt bei vier, denn eine vierte
     Zeile ist kein fuenfter Baustein; A7s tragende Haelfte bleibt damit gewahrt.
   - Cons: A7 nennt ausdruecklich drei Zeilen, also weicht die Antwort von der bestaetigten
     Festlegung ab. Die Zusammenfassung eines Circles waechst um eine Zeile, und drei der vier
     Zustandszeilen tragen immer „nein".
3. **Eine Zeile statt dreier, und sie zaehlt statt zu antworten** — der Zustand faellt aus der
   Zusammenfassung, und an seine Stelle tritt nichts.
   - Pros: Die Zusammenfassung eines Circles wird um zwei Zeilen kuerzer, und keine Zeile
     traegt eine Antwort, die fast immer „nein" lautet.
   - Cons: C5.6 verlangt den Zustand ausdruecklich, und ohne ihn faellt eine der sechs
     skizzierten Auskuenfte aus der Runde. Diese Moeglichkeit kippt A7 statt sie zu ergaenzen.

## Constraints

Der feste Bausteinsatz bleibt bei vier Bausteinen; A7 ist an dieser Stelle bindend und in
dieser Runde nicht verhandelbar. Der Baustein „Vorhandensein" liefert „ja" oder „nein" und
keinen Text, also kann keine Zeile den Marker selbst anzeigen. Der Zustand steht im
**Dateinamen** des Circle-Datensatzes und in keinem Feld darin; ein Feldbaustein erreicht ihn
deshalb nicht.

## Recommendation

Moeglichkeit 2. Der Preis von A7 ist bereits bezahlt, sobald der Zustand ueber Vorhandensein
ausgedrueckt wird; eine vierte Zeile kostet zwei Zeilen TOML und keine Zeile Rust und macht
die Fallunterscheidung ueber die sechs Marker vollstaendig. Der Grund, aus dem A7 drei Zeilen
nennt, ist die Zahl der Zustaende, die der Spec vor Augen hatte, und nicht eine Obergrenze;
die Messung am Bestand zeigt, dass es sechs sind.

---
Answered: 260824-1505 — Möglichkeit 2. Das Rundenprofil bekommt eine vierte
Zustandszeile „Abgelegt", ein Vorhandensein auf `^_[sd]_circle\.md$`. Damit
fällt jeder der sechs Marker in genau eine Zeile.

**Der Preis ist gemessen und beträgt null.** Ein Vorhandensein mit `muster` und
ohne `ordner` prüft die Liste des erkannten Ordners, die für die Erkennung
ohnehin gelesen ist: kein zusätzlicher Verzeichnisleselauf, keine
Dateiöffnung. Das Rundenprofil steht bei acht Zeilen und elf von vierundzwanzig
Öffnungen; die vierte Zustandszeile ändert an beiden Zahlen nichts. Damit
kostet die Antwort zwei Zeilen TOML und keine Zeile Rust, wie der Datensatz es
vorausgesagt hat.

**A7 weicht ab und wird berichtigt statt überschrieben.** Die Festlegung nennt
drei Zeilen; C5.6 bekommt eine Berichtigung **neben** ihren ursprünglichen
Wortlaut, in der Form, die diese Runde am 260824-1250 für vier Kriterien schon
angewandt hat. Der Grund, aus dem A7 drei Zeilen nennt, ist die Zahl der
Zustände, die der Spec vor Augen hatte, und keine Obergrenze; am Bestand
gemessen sind es sechs.

**Die Frage stand mit einer falschen Zahl da, und die Berichtigung gehört zur
Antwort.** Der Abschnitt `## Question` sagt, von den achtzehn
Circle-Verzeichnissen falle „heute genau eines" aus den drei Zeilen. Es sind
zwei: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` und
`260816-2255-befehle-absetzen-und-makros-speichern`, beide `_d_`, nachgezählt
am 260824-1508 mit `ls circles/*/_d_circle.md`. Kein Datensatz trägt heute `_a_`
oder `_s_`. Der Befund lag beim Beantworten als
`issues/260824-1313_o_der-datensatz-zur-vierten-zustandszeile-nennt-ein-verzeichnis-…`
im selben Speicher und ist dem Nutzer beim Vorlegen der Frage nicht genannt
worden; der Orchestrator hat die falsche Zahl weitergereicht. **Die Antwort
bleibt davon unberührt**, denn die falsche Zahl beziffert den Nutzen der vierten
Zeile zu klein und nicht zu groß. Der ursprüngliche Wortlaut in `## Question`
bleibt stehen: er ist der Beleg dafür, auf welcher Grundlage die Frage gestellt
wurde.

Implemented: 260824-1650, Commit `b5bf2e3`, Schritt 14 des Plans. Die Zeile „Abgelegt" steht in `resources/default-readers.toml` hinter „Geschlossen" und vor der Directive-Zeile. Nachgemessen an den achtzehn Circle-Verzeichnissen dieser Werkbank: die vier Zustandszeilen treffen 0, 1, 15 und 2, jedes Verzeichnis bejaht genau eine. Der Haushalt ist unverändert.
Deferred:
Superseded by:
Retired:
