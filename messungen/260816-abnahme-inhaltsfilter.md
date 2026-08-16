# Abnahmeliste: der Inhaltsfilter der Dateiliste (elfte Runde)

**Datum:** 260816-2030
**Für wen:** den Nutzer. Der Durchgang unten ist am laufenden Bündel zu fahren, und kein Agent
kann ihn fahren — aus dem Hintergrund gestartet weist die Wirkungsbereichs-Prüfung jeden
fokusgebundenen Befehl ab.
**Plan:** `fusion-workbench/circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Strang G
**Spec:** `fusion-workbench/shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md`, 57 Abnahmekriterien
**Baumstand:** die acht Commits der Runde, `5c7f5b9` bis `c8fd829`

---

## Der Baumanteil ist gefahren

`make check` — **exit 0**, „alle vier gruen". Gefahren am 260816-2000 über den fertigen Stand
nach F2, also `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --all --check` in einem
Zug. Derselbe Lauf ist nach jedem der acht Bauschritte einzeln grün gewesen; die Belege stehen
in den Sitzungsprotokollen unter
`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/history/`.

Eine einzelne Probe fährt man mit `cargo test -p krk-core <name>` oder
`cargo test -p krk-ui --bin krk <name>` nach. **`cargo` liegt nicht auf dem Standard-PATH**,
sondern unter `$HOME/.cargo/bin`.

---

## Die Zahlen, am Spec nachgezählt

Nicht aus dem Absatz des Spec übernommen, sondern an seinen nummerierten Kästchen gezählt,
Fähigkeit für Fähigkeit:

| | Zahl |
|---|---|
| Kriterien insgesamt | **57** — C1 zwölf, C2 zehn, C3 neun, C4 zehn, C5 fünf, C6 neun, dazu die zwei aus `## Verhältnis zu den zehn Zeitzusagen` |
| allein am Baum belegt (Probe oder Diff) | **20** |
| Baum **und** Bündel | **35** |
| allein am Bündel | **2** — C3.9 und Z1 |
| ohne jeden Beleg | **0** |
| **Kriterien mit Bündelanteil, also Nutzerarbeit** | **37** |

Je Fähigkeit, damit die Summe nachzuzählen ist statt geglaubt zu werden:

| | insgesamt | nur Baum | Baum + Bündel | nur Bündel |
|---|---|---|---|---|
| C1 | 12 | 1 | 11 | 0 |
| C2 | 10 | 1 | 9 | 0 |
| C3 | 9 | 4 | 4 | 1 |
| C4 | 10 | 2 | 8 | 0 |
| C5 | 5 | 2 | 3 | 0 |
| C6 | 9 | 9 | 0 | 0 |
| Z | 2 | 1 | 0 | 1 |
| **Summe** | **57** | **20** | **35** | **2** |

**Der Spec dieser Runde trägt keine Spalte `(Probe)`/`(Bündel)` an seinen Kriterien.** Der Plan
sagt unter G2, er führe sie schon; das trifft für die Runde 10 zu und für diese nicht. Die
Kennzeichnung unten ist deshalb hier hergeleitet, aus der Lage jedes Kriteriums am Baum, und
nicht abgeschrieben. Sie ist Teil dieses Dokuments und nicht des Spec.

Die 37 Kriterien mit Bündelanteil sind unten **27 nummerierte Beobachtungen und drei
Farbbeobachtungen F1 bis F3**; eine Beobachtung deckt teils mehrere Kriterien, und die
Kriterientafel am Ende nennt für jedes die Nummer.

---

## Vorher: der Prüfordner

Der Durchgang läuft an vier Orten. Drei davon werden hier angelegt; **es wird nirgends
gelöscht**, alles wird nur gelesen, und am Ende räumt ein Kommando alles wieder ab.

**Die gesuchte Zeichenfolge ist `notiz`, fünf Zeichen.** Fünf mit Absicht: sie liegt über
beiden Schwellen, also über den drei Zeichen ohne „Deep" und über den fünf mit „Deep". Der
Vierzeichen-Fall heißt `noti`, der Zweizeichen-Fall `no`.

### Die Kommandos

Sie sind auf diesem Gerät gefahren worden und laufen durch; einfach als Block ins Terminal.

```sh
BASE=~/krk-abnahme-11
rm -rf "$BASE"; mkdir -p "$BASE/flach" "$BASE/baum" "$BASE/gross"

# ── Ort 2: der flache Prüfordner ─────────────────────────────────────────────
cd "$BASE/flach"
printf 'nichts besonderes\n'                           > notiz-im-namen.txt
printf 'irgendwo steht notiz mittendrin\n'             > traeger.txt
printf 'hier steht nichts dergleichen\n'               > stumm.txt
printf 'NOTIZ in Grossbuchstaben\n'                    > grossbuchstabe.txt
printf 'Ein Korb voller \xc3\x84pfel\n'                > umlaut.txt
{ printf 'notiz und dahinter '; printf '\xff\xfe'; printf ' kein gueltiges UTF-8\n'; } > binaer.bin
printf 'der Name traegt die Folge, der Inhalt nicht\n' > notiz-gesperrt.txt
printf 'der Inhalt traegt notiz, der Name nicht\n'     > gesperrt-traeger.txt
chmod 000 notiz-gesperrt.txt gesperrt-traeger.txt
ln -s traeger.txt verweis.txt
mkfifo roehre
{ printf 'notiz am Anfang\n'; head -c 1200000 /dev/zero | tr '\0' 'x'; printf '\n'; } > riesig.log

# ── Ort 3: der kleine Unterbaum ──────────────────────────────────────────────
cd "$BASE/baum"
mkdir -p eins/zwei/drei nurname leer
printf 'in der dritten Ebene steht notiz\n' > eins/zwei/drei/tief-traeger.txt
printf 'ohne Fund\n'                        > eins/zwei/drei/stumm-tief.txt
printf 'ohne Fund\n'                        > nurname/notiz-hier.txt
printf 'ohne Fund\n'                        > leer/gar-nichts.txt

# ── Ort 4: ein großer Baum, damit ein Lauf lange genug läuft ─────────────────
cd "$BASE/gross"
fuellung=$(head -c 8192 /dev/zero | tr '\0' 'x')
for a in $(seq 1 100); do
  mkdir -p "ordner-$a"
  for b in $(seq 1 200); do printf '%s\n' "$fuellung" > "ordner-$a/datei-$b.txt"; done
done
cp "$BASE/flach/riesig.log" "$BASE/flach/stumm.txt" "$BASE/gross/"

ls -la "$BASE/flach"; du -sh "$BASE/gross"
```

Der große Baum kostet **20.000 Dateien und rund 160 MB** und braucht etwa zwanzig Sekunden.
Ist ein Inhaltslauf darüber schneller durch, als man ihm zusehen kann, tut es jeder größere
Baum unter dem Benutzerordner genauso — `~/Projects` oder `~/Library/Caches` — und die
Beobachtungen 18 bis 22 laufen dort ebenso. Nur die zwei kopierten Dateien fehlen dann, und
mit ihnen zwei der vier Satzteile aus Beobachtung 18.

### Was jede Datei im flachen Ordner leistet

| Datei | Name trägt `notiz` | Inhalt trägt `notiz` | wozu |
|---|---|---|---|
| `notiz-im-namen.txt` | ja | nein | der Namenstreffer, ungedämpft |
| `traeger.txt` | nein | ja | der Inhaltstreffer, gedämpft |
| `grossbuchstabe.txt` | nein | ja, als `NOTIZ` | die Schreibung zählt nicht |
| `stumm.txt` | nein | nein | steht nie; trägt die Markierung |
| `umlaut.txt` | nein | `Äpfel` | `apfel` findet sie nicht, `äpfel` schon |
| `binaer.bin` | nein | ja, aber kein gültiges UTF-8 | KRK nimmt sie nicht als Text an |
| `riesig.log` | nein | ja, im ersten Byte | 1.200.017 Bytes, über der Grenze von 1 MB |
| `notiz-gesperrt.txt` | ja | nein | kein Leserecht, der Name entscheidet trotzdem |
| `gesperrt-traeger.txt` | nein | ja | kein Leserecht, also unlesbar |
| `roehre` | nein | — | benannte Röhre ohne Schreiber |
| `verweis.txt` | nein | zeigt auf `traeger.txt` | symbolische Verknüpfung |

**Zwei Eigenschaften, die keine Fehlfunktion sind**, und die man kennen muss, bevor man
hinsieht:

- **`verweis.txt` steht bei ausgeschaltetem „Deep" immer**, gleich wie „Content" steht und
  gleich was der Filtertext sagt. Eine symbolische Verknüpfung zählt für den Prüfschritt der
  Sichtbarkeit als Ordner, und ein Ordner steht bei flacher Suche immer. Die Regel ist die der
  Runde 10 und unverändert.
- **`roehre` steht nie**, sobald ein Filtertext steht. Sie ist keine Verknüpfung und kein
  Ordner, bekommt also einen Inhaltsauftrag, und eine benannte Röhre ist keine gewöhnliche
  Datei.

### Das Bündel und der Vordergrund

```sh
cd /Users/k1/Projects/productive/krk
make bundle                 # baut und signiert target/KRK.app
open target/KRK.app
```

Aus einem Terminalfenster im Vordergrund geöffnet steht KRK im Vordergrund; ein Doppelklick im
Finder tut es auch.

### Am Ende

```sh
chmod 644 ~/krk-abnahme-11/flach/notiz-gesperrt.txt ~/krk-abnahme-11/flach/gesperrt-traeger.txt
rm -rf ~/krk-abnahme-11
```

Das `chmod` steht davor, weil `rm -rf` an einer Datei ohne Rechte in einem Ordner mit Rechten
zwar durchkommt, der Ordner selbst aber lesbar bleiben soll, falls jemand vorher nachsieht.

---

## Der Durchgang: 27 Beobachtungen an vier Orten

```text
Ort 1  das Fenster, wie es startet     Beobachtung 1 bis 5    nichts wird angefasst
Ort 2  ~/krk-abnahme-11/flach          Beobachtung 6 bis 14   nur gelesen
Ort 3  ~/krk-abnahme-11/baum           Beobachtung 15 bis 17  nur gelesen
Ort 4  ~/krk-abnahme-11/gross          Beobachtung 18 bis 25  nur gelesen
```

### Ort 1 — das Fenster, wie es startet

| # | Was zu tun ist | Was zu sehen sein muss | Kriterium |
|---|---|---|---|
| 1 | KRK starten und die Bereichsleiste am Fensterfuß ansehen | Die Leiste zeigt **zehn** Kästchen. `Content` steht **unmittelbar rechts von** `Deep`, mit dem kleinen Abstand dazwischen und nicht mit dem großen Gruppenabstand; die Leiste bleibt bei ihren 18 Punkten Höhe | C2.1 |
| 2 | In die Dateiliste klicken, damit der Fokusrahmen dort steht, dann auf `Content` klicken und wieder | Der Haken springt um. **Der Fokusrahmen bleibt, wo er war**, und der Fenstertitel ändert sich nicht. Das Kästchen bekommt keinen Tastaturfokus | C2.2 |
| 3 | Das Hauptmenü öffnen und den Bereich „Dateilisting" aufsuchen | Dort steht **„Inhaltssuche ein- und ausschalten"** direkt unter „Tiefe Suche ein- und ausschalten", **ohne Kürzel** dahinter, und der Eintrag ist anklickbar | C2.8 |
| 4 | `F1` drücken (Tastaturbelegung anzeigen) und nach „Inhaltssuche" suchen | Die Funktion steht in der Liste, mit **leerer** Tastenspalte, und lässt sich von Hand belegen. Sie ist nicht als reserviert ausgewiesen | C2.7 |
| 5 | Menü „Anwendung" → „Tastenbelegung als Markdown sichern", danach im Terminal `grep -c Inhaltssuche ~/Downloads/KRK-Tastenbelegung.md` | Die Datei entsteht, und `grep` liefert **0**. Die Funktion fällt aus der Markdown-Ausgabe heraus, weil sie keine Kombination trägt — genau wie „Tiefe Suche" | C2.8 |

### Ort 2 — der flache Prüfordner

**Vor Beobachtung 6:** in `~/krk-abnahme-11/flach` gehen, `Deep` **aus**, `Content` **aus**,
kein Filtertext.

| # | Was zu tun ist | Was zu sehen sein muss | Kriterium |
|---|---|---|---|
| 6 | `notiz` tippen | Es stehen genau **drei** Zeilen: `notiz-im-namen.txt`, `notiz-gesperrt.txt` und `verweis.txt`. Die Statuszeile sagt `Filter „notiz“: 3 von 11 angezeigt`. Das ist die Ausgangslage der Runde 10, unverändert | Ausgangslage |
| 7 | Bei stehendem `notiz` `Content` einschalten | Es kommen **zwei** Zeilen dazu, `traeger.txt` und `grossbuchstabe.txt`, und beide sind **gedämpft** dargestellt. Es kommen **nicht** dazu: `binaer.bin`, `gesperrt-traeger.txt`, `riesig.log`, `roehre`, `stumm.txt`, `umlaut.txt` | C1.1, C1.6, C1.7, C1.9, C2.9, C5.1 |
| 8 | `notiz-gesperrt.txt` in dieser Liste ansehen und die Statuszeile lesen | Die Zeile steht **ungedämpft** da — ihr Name entscheidet, ihr Inhalt wird nicht gelesen —, und in der Statuszeile erscheint **keine Meldung** über eine unlesbare Datei | C1.3 |
| 9 | `Content` wieder ausschalten | `traeger.txt` und `grossbuchstabe.txt` verschwinden sofort, die drei Zeilen aus Beobachtung 6 bleiben | C2.9 |
| 10 | `Content` einschalten, `Esc`, dann `no` tippen (zwei Zeichen) | Die Liste zeigt dasselbe wie bei ausgeschaltetem `Content`: nur die Namenstreffer und `verweis.txt`. `traeger.txt` steht **nicht** da. Ein drittes Zeichen (`not`) holt es zurück | C1.2 |
| 11 | `Esc`, dann `apfel` tippen; danach `Esc` und `äpfel` tippen | `apfel` findet `umlaut.txt` **nicht**. `äpfel` findet sie. Der Vergleich faltet keine Umlaute — dieselbe Regel wie am Namen | C1.4 |
| 12 | `Esc`, `Deep` **ein**, `Content` **ein**, `notiz` tippen | `verweis.txt` steht **nicht** mehr da, obwohl sie auf `traeger.txt` zeigt. In eine Verknüpfung wird nicht abgestiegen, und ihr Ziel wird nicht gelesen. Danach `Deep` wieder aus | C3.7 |
| 13 | `Esc`, `Deep` aus, `Content` aus. `stumm.txt` auswählen und **Leertaste** drücken (markieren). Dann `notiz` tippen und `Content` einschalten | Die Statuszeile sagt: `Filter „notiz“: 5 von 11 angezeigt, eine Datei zu groß, eine Markierung ausgeblendet`. **Der Satz ist nicht rot** und steht in der einen Statuszeile; eine zweite Anzeige entsteht nicht | C4.8, C4.9, C4.10 |
| 14 | Ein Dateifenster auf `/dev` stellen, `Content` **ein**, `Deep` **aus**, `zeroo` tippen (fünf Zeichen, trifft keinen Namen) | KRK **hält nicht an** und antwortet weiter auf Tasten. Die Liste bleibt leer oder zeigt nur Namenstreffer; `zero` steht nicht darin. Danach zurück nach `~/krk-abnahme-11/flach` | C1.8, C1.9 |

**Zu Beobachtung 14, damit sie nicht mehr verspricht, als sie zeigt.** `/dev/zero` wird nicht
gelesen, weil die Typprüfung am offenen Deskriptor es als „keine gewöhnliche Datei" abweist —
nicht, weil die Größengrenze griffe. Der Spec nennt `/dev/zero` unter C1.8 als Probe; was die
Beobachtung wirklich zeigt, ist die Typprüfung samt der Zusage, dass KRK an nichts in `/dev`
hängenbleibt. Dass die Grenze beim **Lesen** gehalten und nicht nur aus der Größenauskunft
vorhergesagt wird, ist am Baum belegt und nicht am Bündel: die Probe
`die_huelle_liefert_die_bytes_und_haelt_ihre_grenze` prüft eine Datei genau auf der Grenze und
eine ein Byte darüber, und der Rumpf liest mit `take(grenze + 1)` und weist zurück, wenn ein
Byte zu viel ankommt.

### Ort 3 — der kleine Unterbaum

**Vor Beobachtung 15:** in `~/krk-abnahme-11/baum` gehen, `Deep` **ein**, `Content` **ein**.

| # | Was zu tun ist | Was zu sehen sein muss | Kriterium |
|---|---|---|---|
| 15 | `notiz` tippen (fünf Zeichen) | `eins` steht — der einzige Treffer darunter ist der **Inhalt** von `eins/zwei/drei/tief-traeger.txt`, drei Ebenen tief. `nurname` steht ebenfalls (Namenstreffer darunter). `leer` steht **nicht** | C3.1, C3.3 |
| 16 | Ein Zeichen zurücknehmen, sodass `noti` steht (vier Zeichen) | `eins` **verschwindet**: mit `Deep` liegt die Schwelle bei fünf Zeichen, und darunter entscheidet allein der Name. `nurname` bleibt stehen. Das fünfte Zeichen wieder tippen holt `eins` zurück | C2.10, C3.2 |
| 17 | Bei stehendem `notiz` und `Deep` ein: `Content` ausschalten | `eins` verschwindet **sofort** und bleibt weg. `nurname` verschwindet **mit** und steht gleich darauf wieder da: der Befundvektor sagt, *dass* etwas unter einem Ordner lag, und nicht *warum*, also fällt er ganz und der neue Lauf entscheidet neu. Bei diesem kleinen Baum ist das ein Aufblitzen; zu sehen sein muss, dass `nurname` **steht**, wenn die Hand von der Maus ist | C2.9, C4.4 |

### Ort 4 — der große Baum

**Vor Beobachtung 18:** in `~/krk-abnahme-11/gross` gehen, `Deep` **ein**, `Content` **ein**,
kein Filtertext. `stumm.txt` auswählen und mit der **Leertaste** markieren.

| # | Was zu tun ist | Was zu sehen sein muss | Kriterium |
|---|---|---|---|
| 18 | `notiz` tippen und **während der Lauf läuft** die Statuszeile lesen | Alle **vier** Satzteile stehen zugleich: `Filter „notiz“: N von 102 angezeigt, Inhalt wird gelesen, eine Datei zu groß, eine Markierung ausgeblendet`. Wenn der Lauf durch ist, **vergeht der Teil „, Inhalt wird gelesen"**, und die anderen drei bleiben stehen | C4.8 |
| 19 | Während der Lauf läuft: mit den Pfeiltasten durch die Liste gehen, mit dem Tabbefehl in einen anderen Tab und zurück, das andere Dateifenster anklicken, an der Bereichsleiste ein Kästchen umlegen | **Alles antwortet ohne Verzug.** KRK steht nicht, der Regenbogenzeiger erscheint nicht, die Auswahl bewegt sich, der Tabwechsel geschieht | C3.9, Z1 |
| 20 | Filtertext löschen. `notiz` tippen und **während der Lauf läuft** ein sechstes Zeichen anhängen; dann eines zurücknehmen; dann `Esc` | Jedes davon **beendet den Lauf sofort** und beginnt gegebenenfalls einen neuen. Das getippte Zeichen erscheint **ohne Wartezeit** in der Statuszeile — es wartet nie auf den Durchlauf des vorigen Filtertexts. Nach `Esc` ist die Liste wieder vollständig, und der Zusatz „, Inhalt wird gelesen" ist weg | C4.1, C4.2, C4.3, C4.6 |
| 21 | `notiz` tippen und **während der Lauf läuft** `Content` ausschalten; erneut anstoßen und `Deep` ausschalten | Beides **beendet den Lauf**. Der Lesehinweis verschwindet, und die Zeilen, die allein wegen ihres Inhalts standen, fallen weg — **Dateien wie Ordner**, und ohne auf einen neuen Unterbaumlauf zu warten | C2.9, C4.4 |
| 22 | `notiz` tippen und **während der Lauf läuft** in einen der `ordner-*` einsteigen; danach mit dem Tabbefehl den Tab wechseln, während wieder ein Lauf läuft | Der Lauf der verlassenen Ansicht **endet**. Im neuen Ordner stehen Filtertext und `Content` unverändert, und er **beginnt sofort** damit, seine Dateien zu lesen — der Lesehinweis erscheint dort ohne Zutun | C1.12, C2.4, C4.5 |
| 23 | Beim Tippen von `notiz` genau hinsehen, welche Zeilen zuerst dastehen | Die Liste beginnt bei den **Namenstreffern** und wächst danach um die Inhaltstreffer. Eine Datei, deren Inhalt noch nicht gelesen ist, steht **nicht** vorsorglich da | C1.10, C1.11 |
| 24 | Zwei Tabs anlegen, in einem `Content` einschalten, im anderen nicht, und zwischen ihnen wechseln | Das Kästchen `Content` in der Bereichsleiste **zieht beim Tabwechsel den Stand des sichtbaren Tabs nach**. Dasselbe beim Wechsel des aktiven Dateifensters | C2.3 |
| 25 | Mit gesetztem `Content` KRK beenden und neu starten | Nach dem Neustart steht `Content` **aus**, in jedem Tab. Der Stand übersteht die Sitzung nicht | C2.5 |
| 26 | `Deep` **ein**, `Content` **ein**, `notiz` tippen und den Lauf **ganz durchlaufen lassen**. Dann `Content` ausschalten und **auf die Ordnerzeilen sehen** | Die Ordnerzeilen, die auf einem Befund standen, sind **im selben Augenblick** weg — nicht erst, wenn der neue Lauf sie eingeholt hat. Wer die Uhr mitlaufen lässt: zwischen Klick und leerer Liste liegt kein Lauf über den Unterbaum. Was zurückkommt, sind die Ordner mit einem **Namens**treffer darunter, und zwar so schnell, wie der neue Lauf sie findet | C2.9 |
| 27 | `Esc`, `Deep` **ein**, `Content` **ein**, in einem Ordner mit einem `.git` (etwa dem Projektbaum von KRK) `notiz` tippen. Danach die versteckten Einträge einblenden | Beim ersten Tippen wird **kein** versteckter Eintrag gelesen: kein `.git`, kein `.DS_Store`. Der Lauf ist damit spürbar kürzer als vor dem 260816. Das Einblenden **stößt einen neuen Lauf an** — der Lesehinweis erscheint wieder —, und danach stehen auch versteckte Inhaltstreffer da | C1.9, C2.9 |

### Die Farben, in beiden Farbtafeln

Diese drei Beobachtungen gehören zu Ort 2 und stehen getrennt, weil sie **zweimal** zu fahren
sind: einmal in der hellen und einmal in der dunklen Farbtafel. Umgeschaltet wird in den
Systemeinstellungen unter „Erscheinungsbild".

Herzustellen ist in `~/krk-abnahme-11/flach` diese eine Liste: `Deep` aus, `Content` ein,
Filtertext `notiz`, `traeger.txt` markiert oder nicht — je nach Zeile unten.

| # | Was zu tun ist | Was zu sehen sein muss | Kriterium |
|---|---|---|---|
| F1 | In **beiden** Tafeln: `notiz-im-namen.txt` und `traeger.txt` nebeneinander ansehen | `traeger.txt` ist **sichtbar gedämpfter** als `notiz-im-namen.txt`, und der Unterschied fällt beim Hinsehen auf, nicht erst beim Suchen. **Alle vier Spalten** tragen die Dämpfung, nicht nur der Name | C5.1 |
| F2 | In **beiden** Tafeln: `traeger.txt` markieren (Leertaste), dann auswählen | Markiert ist sie **orange und fett und nicht gedämpft** — die Markierung schreibt, die Dämpfung weicht. Ausgewählt ist sie blau unterlegt, und die gedämpfte Schrift bleibt darauf **lesbar** | C5.2 |
| F3 | Die Liste stehen lassen und **im laufenden Betrieb** die Farbtafel umschalten | Die Dämpfung zieht mit, ohne dass die Liste neu gelesen wird oder die Auswahl springt. Alle drei Farben sind dynamische Systemfarben und wechseln zusammen mit der Tafel | C5.3 |

**Was für F1 und F2 schon gemessen ist**, damit die Beobachtung weiß, was sie noch beiträgt.
Ein weggeworfenes Programm auf dem Hauptfaden hat die drei Farben in beiden Tafeln aufgelöst
und gegen den Listenhintergrund gerechnet:

| Farbtafel | `labelColor` | `secondaryLabelColor` | Kontrast der Dämpfung |
|---|---|---|---|
| hell | 0,153 | 0,502 | **3,95 : 1** |
| dunkel | 0,865 | 0,602 | **5,89 : 1** |

Die drei Farben sind in beiden Tafeln paarweise verschieden, und die Dämpfung liegt deutlich
über der Schwelle, unter der `tertiaryLabelColor` gelegen hätte (1,88 : 1 hell). **Was die
Zahlen nicht sagen, ist, ob der Unterschied auf dem Schirm als Absetzung gelesen wird** — und
genau das ist der Beitrag von F1.

---

## Die 57 Kriterien im Einzelnen

Je Kriterium die hergeleitete Kennzeichnung und der Nachweis. Bei **Probe** steht der Name der
Prüfung mit ihrer Datei, bei **Diff** das Kommando oder die Stelle, bei **Bündel** die Nummer
der Beobachtung von oben.

### C1 — Der Inhalt entscheidet über eine Datei, deren Name nicht passt

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C1.1 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::der_inhaltsfilter_wirkt_ab_drei_zeichen_und_darunter_nicht`, `…::ein_flacher_inhaltsauftrag_liest_die_datei_und_entscheidet_sie`. Bündel: **7** |
| C1.2 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::der_inhaltsfilter_wirkt_ab_drei_zeichen_und_darunter_nicht` (der Zweizeichen-Teil). Bündel: **10** |
| C1.3 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::ein_namentlicher_treffer_steht_ohne_jeden_befund`, `…::ein_namenstreffer_im_unterbaum_bleibt_ungelesen` (Datei ohne Leserecht, `treffer: true` belegt, dass nicht geöffnet wurde), `krk-ui/src/tabs.rs::eine_datei_mit_namenstreffer_bleibt_ungelesen`. Bündel: **8** |
| C1.4 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::ein_text_mit_der_folge_traegt_sie_und_einer_ohne_nicht`, `…::der_name_und_der_inhalt_geben_dieselbe_antwort` (sechs Gegenstände gegen acht Folgen, darunter `Äpfel`/`apfel` und `Cafe`/`café`). Bündel: **11** als Gegenprobe am Schirm |
| C1.5 | Probe | `krk-core/tests/verzeichnis.rs::die_folge_in_den_letzten_bytes_vor_der_grenze_wird_gefunden` — 4096 Bytes, die Folge in den letzten neun |
| C1.6 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::eine_datei_ohne_gueltiges_utf8_traegt_nichts`, `…::was_keine_gewoehnliche_datei_ist_traegt_nichts`, `…::eine_datei_ohne_leserecht_traegt_nichts`. Bündel: **7** |
| C1.7 | Probe + Diff + **Bündel** | `krk-core/tests/verzeichnis.rs::eine_datei_ueber_der_grenze_bleibt_ungelesen`, `…::eine_zu_grosse_datei_bleibt_ungelesen_und_zaehlt`, `krk-ui/src/tabs.rs::die_zahl_der_zu_grossen_dateien_steht_auch_nach_dem_ende_des_laufs` (an einem echten Prüfordner mit einer Datei über `TEXTGRENZE`). Diff: die Zahl reist an genau einer Stelle in den Kern, `tabs.rs:913-921`, und ist unverändert `vorschaumodell::TEXTGRENZE`. Bündel: **7**, **13** |
| C1.8 | Probe + **Bündel** | `krk-core/tests/text.rs::die_huelle_liefert_die_bytes_und_haelt_ihre_grenze` — darunter, genau darauf, ein Byte darüber. Der Rumpf liest `take(grenze + 1)` und weist nach dem Lesen ab (`text/datei.rs:626-632`); `grep -rn 'take(grenze + 1)' crates --include='*.rs'` nennt genau diese eine Stelle. Bündel: **14**, mit der Einschränkung im Absatz darunter |
| C1.9 | Probe + **Bündel** | `krk-core/tests/text.rs::eine_benannte_roehre_ist_keine_datei_und_haelt_die_huelle_nicht_an`, `krk-core/tests/verzeichnis.rs::was_keine_gewoehnliche_datei_ist_traegt_nichts` — beide unter Zeitschranke, die zurückkehren muss. Bündel: **7**, **14** |
| C1.10 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::der_inhaltsfilter_wirkt_ab_drei_zeichen_und_darunter_nicht` (ein `Unentschieden` steht nicht). Bündel: **23** |
| C1.11 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::ohne_grenze_wird_keine_einzige_datei_geoeffnet` — ein Inhaltsauftrag ohne Grenze bleibt unentschieden, statt still negativ zu werden. Bündel: **23** |
| C1.12 | Probe + **Bündel** | `krk-ui/src/tabs.rs::ein_ordnerwechsel_traegt_den_stand_von_content`, `…::der_inhaltsfilter_geht_auch_ohne_filtertext_hinueber`. Bündel: **22** — dass der neue Ordner **sofort** zu lesen beginnt, ist nur am Bündel zu sehen |

### C2 — Das Ankreuzfeld „Content" in der Bereichsleiste

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C2.1 | Probe + **Bündel** | `krk-ui/src/appkit/bereichsleiste.rs::die_leiste_traegt_zehn_schalter`, `…::die_zwei_letzten_schalter_heissen_deep_und_content_und_stehen_rechts_von_typ`. Bündel: **1** |
| C2.2 | Probe + **Bündel** | `krk-ui/src/appkit/bereichsleiste.rs::der_zehnte_schalter_gibt_fokus_keinen_sechsten_wert`. Der Ersthelferrang fällt ohne eigene Zeile weg: `schalter_bauen` setzt `setRefusesFirstResponder(true)` für jeden Schalter. Bündel: **2** |
| C2.3 | Probe + **Bündel** | `krk-ui/src/tabs.rs::der_filtertext_gehoert_dem_tab_und_nicht_dem_fenster` für die Zuordnung; der Nachzug hängt an denselben drei Anlässen wie „Deep" (`anwendung.rs::bereichsleiste_nachziehen`). Bündel: **24** |
| C2.4 | Probe + **Bündel** | `krk-ui/src/tabs.rs::ein_ordnerwechsel_traegt_den_stand_von_content`. Bündel: **22** |
| C2.5 | **Diff** + **Bündel** | `krk_core::ablage::sitzung::Tab` (`crates/krk-core/src/ablage/sitzung.rs:82-113`) führt weder `inhalt` noch `tief` noch den Filtertext; nichts davon geht in die `session.toml`. Bündel: **25** |
| C2.6 | Probe | `krk-core/tests/verzeichnis.rs::ohne_filtertext_aendert_der_inhaltsfilter_nichts` |
| C2.7 | Probe + **Bündel** | `krk-core/tests/belegung.rs::jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` über `OHNE_KOMBINATION_AB_WERK` (fünf Einträge seit E1), `…::jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`, `krk-core/src/tasten/belegung.rs::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`. Bündel: **4** |
| C2.8 | Probe + Diff + **Bündel** | `krk-ui/src/menuemodell.rs::jede_funktion_der_belegung_steht_genau_einmal_im_menue`, `krk-ui/src/belegungsausgabe.rs::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`, `…::eine_funktion_ohne_kombination_erscheint_nicht`. Diff: `make menue \| grep Inhaltssuche` zeigt den Eintrag unter „Dateilisting" mit `kombination=(keines)`, direkt hinter „Tiefe Suche ein- und ausschalten". Bündel: **3**, **5** |
| C2.9 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::das_ausschalten_nimmt_die_inhaltszeilen_weg_und_setzt_den_befund_zurueck` für die Datei, `…::das_ausschalten_des_inhaltsfilters_nimmt_auch_die_ordnerzeile_sofort_weg` für den Ordner, `…::das_ausschalten_nimmt_auch_eine_namentlich_begruendete_ordnerzeile_mit` für den benannten Preis, `…::ein_befund_gilt_nur_zu_seiner_frage` für die Regel dahinter. Bündel: **7**, **9**, **17**, **21**, **26**, **27** |
| C2.10 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::die_tiefe_suche_hebt_die_schwelle_auf_fuenf_zeichen`, `krk-core/src/verzeichnis/filter.rs::die_inhaltsschwelle_steht_bei_drei_und_bei_fuenf`, `krk-ui/src/tabs.rs::bei_vier_zeichen_und_deep_traegt_die_auftragsliste_keinen_inhaltsauftrag`. Bündel: **16** |

### C3 — Der Inhaltsfilter über den Unterbaum

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C3.1 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::ein_treffer_allein_im_text_entscheidet_den_unterbaum` — derselbe Baum zweimal, ohne Grenze unentschieden, mit Grenze getroffen. Bündel: **15** |
| C3.2 | Probe + **Bündel** | `krk-ui/src/tabs.rs::bei_vier_zeichen_und_deep_traegt_die_auftragsliste_keinen_inhaltsauftrag`. Bündel: **16** |
| C3.3 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::ein_treffer_tief_unten_entscheidet_den_ordner` (Runde 10), `…::ein_treffer_allein_im_text_entscheidet_den_unterbaum`. Bündel: **15** |
| C3.4 | Probe | `krk-core/tests/verzeichnis.rs::ein_namenstreffer_im_unterbaum_bleibt_ungelesen`, `krk-ui/src/tabs.rs::eine_datei_mit_namenstreffer_bleibt_ungelesen` |
| C3.5 | Probe + **Diff** | Probe für die Verzeichnisseite: `krk-core/tests/verzeichnis.rs::die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden` (Kindprobe unter `ulimit -n 64`). Diff für die Dateiseite: `text::datei::bis_zur_grenze_lesen` gibt den Dateideskriptor beim Verlassen frei, bevor `unterbaum_entscheiden` den nächsten Kandidaten öffnet — gehalten werden ein Verzeichnis- und höchstens ein Dateideskriptor. **Die Dateiseite ist nicht unter abgesenkter Grenze gemessen**, siehe „Was ungemessen bleibt" |
| C3.6 | Probe | `krk-core/tests/verzeichnis.rs::ein_deskriptormangel_beim_lesen_laesst_die_datei_unentschieden` samt Kindprobe `kind_meldet_bei_deskriptormangel_ueber_einer_datei_nichts`, unter `ulimit -n 64` |
| C3.7 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::eine_verknuepfung_im_unterbaum_wird_nicht_gelesen` (samt Gegenprobe über den echten Ort derselben Datei), `…::eine_verknuepfung_steht_nie_wegen_ihres_inhalts`. Bündel: **12** |
| C3.8 | Probe | `krk-ui/src/tabs.rs::je_tab_laeuft_nie_mehr_als_ein_durchlauf`, `…::ein_verdeckter_tab_bekommt_keinen_durchlauf` |
| C3.9 | **allein Bündel** | Keine Probe, und keine ist möglich: gefragt ist, ob die Anwendung während eines laufenden Fadens antwortet. Bündel: **19** |

### C4 — Rückmeldung und Abbruch

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C4.1 | Probe + **Bündel** | `krk-ui/src/tabs.rs::ein_weiteres_zeichen_loest_den_laufenden_durchlauf_ab`. Bündel: **20** |
| C4.2 | Probe + **Bündel** | derselbe Weg über `filtertext_setzen`; `krk-core/tests/verzeichnis.rs::ein_befund_gilt_nur_zu_seiner_frage`. Bündel: **20** |
| C4.3 | Probe + **Bündel** | **Keinem Schritt des Plans zugewiesen; hält am Baum.** `krk-ui/src/tabs.rs::ohne_seine_drei_bedingungen_beginnt_kein_durchlauf` (Abschnitt „Ohne Filtertext"). Der Weg: `abbrechen` → `filter_leeren` → `nach_filteraenderung` → `durchlauf_nachziehen_an`, das `durchlauf = None` als erste Zeile setzt. Datensatz `issues/260816-2020_o_zwei-abnahmekriterien-sind-keinem-schritt-des-plans-zugewiesen.md`. Bündel: **20** |
| C4.4 | Probe + **Bündel** | `krk-ui/src/tabs.rs::das_ausschalten_von_deep_bricht_den_durchlauf_ab`, `…::allein_content_stoesst_einen_durchlauf_an` (in beide Richtungen). Bündel: **17**, **21** |
| C4.5 | Probe + **Bündel** | `krk-ui/src/tabs.rs::ein_tabwechsel_beendet_den_durchlauf_des_verlassenen_tabs`, `…::ein_verdeckter_tab_bekommt_keinen_durchlauf`. Bündel: **22** |
| C4.6 | Probe + **Bündel** | Der Abbruch wartet nicht: `Durchlauf::drop` setzt das Kennzeichen und lässt den Empfänger fallen, ohne den Faden zu verbinden. `krk-ui/src/tabs.rs::das_abbrechen_des_fensters_nimmt_den_durchlauf_mit`. Bündel: **20** |
| C4.7 | Probe (Ort) + **ungemessen** (Spanne) | `krk-core/tests/verzeichnis.rs::die_abbruchgrenze_steht_vor_jedem_stapel_und_vor_jeder_datei` schneidet den Quelltext ohne Kommentarzeilen und zählt die Prüfstellen: zwei in `unterbaum_entscheiden`, eine im flachen Zweig davor, keine dahinter. **Die Dauer der Spanne misst nichts**, siehe „Was ungemessen bleibt" |
| C4.8 | Probe + **Bündel** | `krk-ui/src/appkit/statuszeile.rs::jede_kombination_der_vier_satzteile_steht_in_der_festgelegten_reihenfolge` (alle acht Kombinationen, Satz für Satz ausgeschrieben), `…::ohne_zu_grosse_datei_steht_der_groessenhinweis_nicht_da`, `…::der_groessenhinweis_trennt_eine_datei_von_mehreren`, `…::ohne_inhaltsdurchlauf_ist_der_satz_der_der_runde_zehn`, `krk-ui/src/tabs.rs::die_zahl_der_zu_grossen_dateien_steht_auch_nach_dem_ende_des_laufs`. Bündel: **13**, **18** |
| C4.9 | Probe | `krk-ui/src/appkit/statuszeile.rs::der_volle_satz_bleibt_ein_rang_und_kein_fehler` — `Rang::ALLE` hat unverändert sechs Werte; `…::jeder_der_sechs_raenge_hat_genau_ein_feld` |
| C4.10 | Probe + **Bündel** | `krk-ui/src/appkit/statuszeile.rs::der_volle_satz_bleibt_ein_rang_und_kein_fehler` — `Art::Vorgang`, also nicht rot; `…::der_filterstand_gilt_nicht_als_fehler`. Bündel: **13** |

### C5 — Der Treffergrund an der Zeile

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C5.1 | Probe + **Bündel** | `krk-core/tests/verzeichnis.rs::steht_wegen_des_inhalts_antwortet_nur_fuer_die_eine_lage`, `…::unter_der_schwelle_steht_keine_zeile_wegen_ihres_inhalts`. Die Verdrahtung Zeile → Eintragsindex → Modellfrage und die dreiwertige Farbwahl stehen in `krk-ui/src/appkit/tabelle.rs` und brauchen AppKit auf dem Hauptfaden. Diff: `grep -c 'NSColor::' crates/krk-ui/src/appkit/tabelle.rs` liefert 3 statt 2. Bündel: **F1** |
| C5.2 | Probe + **Bündel** | Die Reihenfolge der Farbwahl ist am Diff abzulesen (markiert → orange; sonst Inhaltstreffer → gedämpft; sonst → Grundfarbe), die Schriftwahl bleibt zweiwertig. Gemessene Kontraste siehe oben. Bündel: **F2**. **Die Auswahlhälfte ist ungemessen**, siehe unten |
| C5.3 | gemessenes Programm + **Bündel** | Ein weggeworfenes Programm auf dem Hauptfaden hat alle drei Farben in beiden Tafeln aufgelöst; die Zahlen stehen oben. Alle drei sind dynamische Systemfarben, ein Beobachter der Erscheinung entsteht nicht. Bündel: **F3** |
| C5.4 | Probe | `krk-core/tests/verzeichnis.rs::steht_wegen_des_inhalts_antwortet_nur_fuer_die_eine_lage`, `…::ein_namentlicher_treffer_steht_ohne_jeden_befund` — der Kurzschluss des Namens macht die beiden Gründe überschneidungsfrei |
| C5.5 | Probe | `krk-core/tests/verzeichnis.rs::eine_verknuepfung_steht_nie_wegen_ihres_inhalts`, `…::steht_wegen_des_inhalts_antwortet_nur_fuer_die_eine_lage` (ein Ordner bekommt die Kennzeichnung nie) |

### C6 — Der eine Vergleich und seine Zählprobe

| # | Kennzeichnung | Nachweis |
|---|---|---|
| C6.1 | Probe + Diff | `krk-core/tests/verzeichnis.rs::die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`. Diff: `grep -rl 'traegt_die_folge' crates --include='*.rs'` nennt vier Dateien — `filter.rs` als Heimat und die drei Rufer `durchlauf.rs`, `inhalt.rs`, `modell.rs` |
| C6.2 | Probe + **Diff** | **Keinem Schritt des Plans zugewiesen; hält am Baum.** `krk-core/tests/verzeichnis.rs::der_kleingeschriebene_filtertext_laeuft_mit`. Diff: `Ordnermodell::filter_uebernehmen` (`modell.rs:906-907`) ist die eine Stelle, an der `filter_klein` entsteht; `traegt_der_inhalt` nimmt ihn als `&str` und schreibt ihn nicht um. Datensatz `issues/260816-2020_o_…` |
| C6.3 | Probe | `krk-core/tests/verzeichnis.rs::die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` — namentliche Liste, Meldung nennt weiterhin den unerwarteten Rufer, nicht durch eine bloße Zahl ersetzt. Der Name der Probe ist mitgezogen worden, weil er „zwei Rufer" für beide Regeln behauptete |
| C6.4 | Probe | dieselbe Probe, Zeichenregelhälfte: `traegt_ein_dateiname` behält seine zwei Rufer |
| C6.5 | Probe + Diff | `krk-core/tests/text.rs::die_huelle_liefert_die_bytes_und_haelt_ihre_grenze` und die drei Hindernisproben daneben. Diff: `traegt_der_inhalt` ruft `bis_zur_grenze_lesen`, und das ruft `verzeichnis::sys::ohne_warten_oeffnen` — derselbe Eingang wie Editor und Vorschau, kein dritter |
| C6.6 | Probe + Diff | `krk-core/tests/text.rs::ein_ordner_ist_fuer_die_huelle_keine_datei`, `…::eine_benannte_roehre_ist_keine_datei_und_haelt_die_huelle_nicht_an`. Diff: `text/datei.rs:616-619` fragt `datei.metadata()` am **offenen Deskriptor** und nicht `lstat` am Pfad |
| C6.7 | Probe | `krk-core/tests/verzeichnis.rs::eine_datei_ohne_gueltiges_utf8_traegt_nichts` — die Folge steht als ASCII in der Datei, und sie trägt trotzdem nichts. Entschieden wird über `String::from_utf8` auf den gelesenen Bytes, nicht über eine Endungsliste |
| C6.8 | Probe | `krk-core/tests/verzeichnis.rs::im_filter_steht_keine_zeitmessung`, jetzt sieben Dateien statt fünf. **Ihre Reichweite ist benannt und begrenzt**: `issues/260816-1359_o_die-probe-gegen-zeitmessung-im-filter-erreicht-zwei-dateien-des-filterwegs-nicht.md` |
| C6.9 | Probe | `krk-core/tests/verzeichnis.rs::der_name_und_der_inhalt_geben_dieselbe_antwort` — links `Ordnermodell::name_traegt_den_filter`, rechts `traegt_der_inhalt` an einer Datei, deren ganzer Inhalt derselbe Text ist |

### Die zwei Kriterien ohne Messstrecke

| # | Kennzeichnung | Nachweis |
|---|---|---|
| Z1: Während des Lesens bleiben beide Dateifenster, die Lesezeichenleiste und die Bereichsleiste bedienbar | **allein Bündel** | Beobachtung **19**. Der Lauf steht auf einem eigenen Faden, der Hauptfaden liest im vorhandenen Einzugstakt mit `try_recv`, und der Abbruch wartet nicht — **gemessen ist es nicht**, und das ist die bewusste Wahl dieser Runde |
| Z2: Keine der zehn Zahlen aus C8 der Runde 1 wird geändert, gelockert oder umgedeutet | **Diff** | `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs \| sort -u` liefert `L1 L10 L2 L3 L4 L5 L6 L7 L8 L9` — dieselben zehn wie vor der Runde. `crates/krk-bench/` ist von keinem der acht Commits angefasst |

---

## Was ungemessen bleibt, und warum

Sechs Stellen. Sie stehen hier, damit später niemand nach einer Messung sucht, die es nicht
gibt, und keine von ihnen ist eine Auslassung.

**Der Kontrast einer gedämpften Zeile gegen die blaue Auswahlfläche.** Gemessen sind 3,95 : 1
hell und 5,89 : 1 dunkel, und beide Zahlen gelten gegen den **Listenhintergrund**. Eine
ausgewählte Zeile liegt auf der Auswahlfarbe des Systems, und die ist eine andere Fläche. KRK
schreibt keine Auswahlfarbe und fängt damit auch nicht an. Beobachtung **F2** sieht hin; eine
Zahl dazu gibt es nicht.

**Ob der Farbunterschied als Absetzung gelesen wird.** Zahlen sagen, dass die drei Farben in
beiden Tafeln paarweise verschieden sind. Sie sagen nicht, dass ein Mensch die Dämpfung beim
Überfliegen einer Liste bemerkt. Das ist der Beitrag von Beobachtung **F1** und durch nichts zu
ersetzen.

**Die Dauer eines Inhaltsdurchlaufs.** Es entsteht keine elfte Zeitzusage, und der Grund steht
im Spec: die vorhandene Messstrecke liest dünnbesetzte Prüfordner, in denen je Datei 512 echte
Bytes stehen. Ein Inhaltsdurchlauf darüber läse fast nichts. Der Inhaltsdurchlauf ist damit der
**fünfte** Gegenstand für die spätere Messrunde, und was er dafür braucht, ist ein vierter
Prüfordner mit echten Bytes.

**Die Länge der Abbruchspanne aus C4.7.** Belegt ist, **wo** geprüft wird — drei Stellen, jede
vor einer Einheit, die dauern kann. Wie lange es vom Setzen des Kennzeichens bis zum Ende des
Fadens dauert, misst nichts; dafür bräuchte es eine Uhr, und in diesem Weg steht keine (C6.8
verlangt ausdrücklich, dass keine hineinkommt). Die obere Schranke ist die Textgrenze, also
eine gelesene Datei von höchstens 1 MB, und das ist eine Zusage über die Bauart und keine
gemessene Zahl.

**Die Deskriptorseite von C3.5 für Dateien.** Dass der Durchlauf genau einen
Verzeichnisdeskriptor hält, ist unter `ulimit -n 64` gemessen. Dass der Inhaltsfilter je Datei
genau einen weiteren öffnet und ihn freigibt, bevor er den nächsten öffnet, ist am Rumpf von
`bis_zur_grenze_lesen` abzulesen und **nicht** unter abgesenkter Grenze gemessen. Gemessen ist
allein der Mangelfall (C3.6): geht ein Deskriptor aus, bleibt der Auftrag unentschieden.

**Der Abnahmelauf der zehn Zeitzusagen aus C8.** Er ist nicht Gegenstand dieser Runde. Der
letzte vollständige Lauf ist vom 260810 (`messungen/260810-1918-alle-zusagen.txt`, alle zehn
halten in allen fünf Durchgängen), und **sechs Runden liegen dazwischen**. Die Runde 11 berührt
zwei gemessene Wege, ohne sie nachzumessen: **L1** misst die Bewegung der Auswahl, und der
Prüfschritt der Sichtbarkeit bekommt zwei Zweige mehr; **L6** misst den Einstieg in einen
Unterordner, und bei gesetztem „Content" stößt jeder Einstieg einen Inhaltsdurchlauf an.

---

## Was beim Zusammentragen aufgefallen ist

Drei Befunde. Zwei tragen einen Datensatz, der dritte ist eine Ungenauigkeit im Plantext.

**Erstens: zwei Kriterien sind keinem Schritt zugewiesen, und beide halten.** C4.3 und C6.2
kommen in keinem `Erfüllt:`-Feld der zwölf Planschritte und in keinem der elf
Sitzungsprotokolle vor. Beide sind oben mit ihrem Nachweis geführt und im Durchgang enthalten.
Datensatz: `issues/260816-2020_o_zwei-abnahmekriterien-sind-keinem-schritt-des-plans-zugewiesen.md`.

**Zweitens: drei Prosastellen im Baum beschreiben einen abgelösten Stand.** `traegt_die_folge`
hat seit A2 drei Rufer, und `crates/krk-core/src/verzeichnis/filter.rs` sagt an vier Stellen
zwei — im Bild des Modulkopfs und in drei Sätzen. Dazu zwei Absätze in
`crates/krk-core/src/verzeichnis/sys.rs`, die den zweiten Aufrufer noch außerhalb der Kiste
verorten und einen statt zweier Frager von `ist_deskriptormangel` nennen. **Kein
Abnahmekriterium ist dadurch gebrochen** — C6.1 und C6.3 halten, und die Zählprobe führt
korrekt drei. Datensatz:
`issues/260816-2015_o_der-vergleich-hat-drei-rufer-und-die-prosa-an-seinem-ort-nennt-zwei.md`.

**Drittens: der Plan sagt unter G2, der Spec führe die Spalten `(Probe)` und `(Bündel)` schon.**
Er führt sie nicht; das trifft auf den Spec der Runde 10 zu. Die Kennzeichnung dieser Liste ist
deshalb hergeleitet und nicht abgeschrieben, und der Absatz unter „Die Zahlen" sagt das. Kein
Datensatz — die Herleitung steht hier, und damit ist die Zusage von G2 erfüllt.

---

## Was diese Liste nicht abnimmt

- **Den Abnahmelauf der zehn Zeitzusagen.** Er ist Nutzerarbeit, verlangt KRK im Vordergrund
  und ist nicht Gegenstand dieser Runde.
- **Die zwei offenen Defekte der Runde.** Der Rückwechsel auf einen Tab setzt seinen beendeten
  Durchlauf nicht fort
  (`issues/260816-1710_o_ein-rueckwechsel-auf-einen-tab-setzt-seinen-beendeten-durchlauf-nicht-fort.md`),
  und die Probe gegen Zeitmessung erreicht zwei Dateien des Filterwegs nicht
  (`issues/260816-1359_o_die-probe-gegen-zeitmessung-im-filter-erreicht-zwei-dateien-des-filterwegs-nicht.md`).
  Beide sind benannt und keiner ist ein Abnahmekriterium.
- **Ob ein häufiges Wort die Liste sinnvoll verkürzt.** Der Spec nimmt unter „Zwei
  Eigenschaften, die diese Runde annimmt" ausdrücklich an, dass `src` in einem Quellbaum fast
  alles stehen lässt. Das ist die Bedeutung des ODER und keine Fehlfunktion.
