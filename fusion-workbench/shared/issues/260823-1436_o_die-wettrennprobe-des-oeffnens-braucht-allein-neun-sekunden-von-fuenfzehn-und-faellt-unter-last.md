# Die Wettrennprobe des Öffnens braucht allein neun Sekunden von fünfzehn und fällt unter Last

---

`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`crates/krk-core/tests/text.rs:802`)
hat eine feste Wanduhr-Frist von 15 Sekunden. Allein gefahren braucht sie 8,3 bis 9,2 Sekunden.
Unter `cargo test --workspace`, wo alle Prüfziele nebeneinander laufen, ist sie in einem von drei
Läufen über die Frist gekommen und rot geworden. Die Meldung, mit der sie abbricht, behauptet
dabei eine Ursache, die aus der Beobachtung nicht folgt.

---

**Gemessen, mit erhaltener Ausgabe.**

## Der rote Lauf

`cargo test --workspace`, Baumstand `b58e9d1`, 260823 gegen 14:10:

```
test ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an ... FAILED

---- ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an stdout ----
thread 'ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an' (140044333)
panicked at crates/krk-core/tests/text.rs:870:19:
die Durchlaeufe sind nach 15 Sekunden nicht fertig geworden;
das Oeffnen haengt an der benannten Roehre

test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;
finished in 15.06s
error: test failed, to rerun pass `-p krk-core --test text`
```

## Die Gegenmessung

Dieselbe Probe allein, fünf Läufe unmittelbar danach, derselbe Baumstand:

| Lauf | Ausgang | Dauer |
|---|---|---|
| 1 | ok | 9,16 s |
| 2 | ok | 9,05 s |
| 3 | ok | 8,71 s |
| 4 | ok | 8,38 s |
| 5 | ok | 8,34 s |

Zwei weitere `cargo test --workspace` danach: grün. `cargo clippy --workspace --all-targets --
-D warnings` und `cargo fmt --all --check` sind sauber.

**Der Abstand zur Frist beträgt allein gefahren rund 40 Prozent.** Das ist der ganze Vorrat, aus
dem die Probe die Gleichzeitigkeit eines vollen Workspace-Laufs bestreiten muss.

## Warum die Frist so knapp sitzt

Die Probe fährt zwei Fäden gegeneinander (`text.rs:806-808`): ein Lesefaden ruft
`datei::oeffnen` mindestens `DURCHLAEUFE = 20_000` mal, ein Tauscherfaden legt dabei mindestens
`MINDESTENS_GETAUSCHT = 2_000` mal abwechselnd eine Datei und eine benannte Röhre unter denselben
Pfad. Die Frist steht als `empfaenger.recv_timeout(Duration::from_secs(15))` (`text.rs:858`).

Sowohl die Zahl der Durchläufe als auch die Frist sind absolut gesetzt. Der Lesefaden macht
20 000 Systemaufrufe, der Tauscherfaden dazu Hardlink- und Umbenennungsaufrufe, und beide
konkurrieren unter `cargo test --workspace` mit jedem anderen Prüffaden desselben Laufs um
dieselben Kerne. Die Frist misst damit nicht die Zusage der Probe, sondern die Belegung der
Maschine.

## Die Meldung nennt eine Ursache, die nicht gemessen ist

```rust
Err(_) => panic!(
    "die Durchlaeufe sind nach 15 Sekunden nicht fertig geworden; \
     das Oeffnen haengt an der benannten Roehre"
),
```

Der `Err`-Zweig deckt zwei verschiedene Sachverhalte ab: das Öffnen hängt wirklich an der Röhre
(die Zusage ist verletzt), oder die Maschine war zu langsam für 20 000 Durchläufe in 15 Sekunden
(die Zusage ist unberührt). Die Meldung entscheidet sich für den ersten, ohne ihn von dem zweiten
trennen zu können. Wer den roten Lauf ohne die Gegenmessung liest, sucht einen Defekt in
`ohne_warten_oeffnen`, den es nicht gibt.

Die Probe hat die Trennung sogar in der Hand und nutzt sie nicht: `gelaufen` und
`gelaufene_tausche` werden mitgezählt, aber im `Err`-Zweig nicht ausgegeben. Ein echter Hänger an
der Röhre bliebe bei einer kleinen Zahl von Durchläufen stehen, ein Zeitmangel käme mit einer
großen Zahl an — nur weiß der `Err`-Zweig sie nicht, weil der Lesefaden erst nach dem letzten
Durchlauf sendet.

## Was das für `260823-1210` heißt

`shared/issues/260823-1210_o_ein-make-check-von-neun-ist-mit-2-abgebrochen-und-hat-sich-nicht-wiederholt.md`
hält fest, dass ein `make check` von neun mit Rückgabewert 2 abgebrochen ist, ohne dass die
Ausgabe erhalten war, und nennt als zu tun: „Den nächsten roten Lauf **mit** seiner Ausgabe
festhalten." Dieser Datensatz hält einen roten Lauf mit seiner Ausgabe fest. Er nennt einen
dritten Kandidaten neben den zwei dort vermuteten, und einer, der die Beobachtung dort trägt: ein
Abbruch ohne Codeänderung, der sich in acht weiteren Läufen nicht wiederholt, ist genau die
Gestalt einer lastabhängigen Frist.

**Bewiesen ist das nicht.** Die Ausgabe des Laufs vom 260823-1205 ist nicht erhalten, also bleibt
offen, ob dort dieselbe Probe rot war. `260823-1210` ist **nicht angefasst**; ob er mit diesem
Datensatz geschlossen werden kann, gehört dorthin und nicht hierher.

## Vorschlag

Zwei Sachen, die getrennt zu haben sind:

1. **Die Frist von der Maschine lösen.** Die Zusage der Probe ist „das Öffnen hängt nicht", und
   die hängt an der Zahl der Durchläufe, nicht an der Wanduhr. Denkbar: die Frist an der
   allein gemessenen Dauer bemessen statt an einer runden Zahl, oder den Lesefaden seinen
   Fortschritt melden lassen und die Frist auf „seit N Sekunden kein Fortschritt" stellen. Die
   zweite Form misst wirklich einen Hänger und nicht die Belegung.
2. **Die Meldung auf das stellen, was gemessen ist.** Mindestens die erreichte Zahl der
   Durchläufe und der Tausche mit ausgeben, statt eine Ursache zu benennen. Solange die zwei
   Fälle nicht getrennt sind, gehört in die Meldung, dass sie es nicht sind.

**Schwere:** Medium. Kein Defekt am ausgelieferten Erzeugnis: `krk-core` verhält sich richtig, die
Probe misst nur an der falschen Größe. Die Schwere kommt daher, dass `make check` das
Abnahmekommando dieses Baums ist und die Auslieferungskette an ihm hängt; eine Abnahme, die ohne
Codegrund rot werden kann, ist um so viel weniger wert.

**Gefunden:** coderev, Auslieferungsdurchsicht `28cbb7b..b58e9d1`, Baumstand `b58e9d1`

**Domain:** code

**Cross-references:** `shared/issues/260823-1210_o_…` (nicht angefasst),
`shared/issues/260810-1925_*_eine-probe-schreibt-ins-echte-temporaerverzeichnis-*`

---
Resolved:
