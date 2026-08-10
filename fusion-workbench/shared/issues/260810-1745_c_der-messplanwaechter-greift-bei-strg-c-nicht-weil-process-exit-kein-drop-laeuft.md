Der Messplanwächter greift bei Strg+C nicht, weil `process::exit` kein `Drop` laufen lässt

---

`plan_schreiben` (`crates/krk-bench/src/messen.rs`) gibt seit dem 260810 einen
`Messplanwaechter` zurück, der den Messplan im `Drop` löscht. Das deckt den Erfolgsweg,
jedes `?` und die Panik-Abwicklung ab. Es deckt **einen** Ausgang nicht ab:
`signalwache_starten` endet in `std::process::exit`, und dabei läuft kein `Drop`. Drückt der
Messende während eines Laufs Strg+C, bleibt `krk-messplan-<pid>.toml` weiter im
Temporärverzeichnis liegen.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Umsetzung des Defekts
`shared/issues/260810-1330_*_der-messplan-bleibt-liegen-wenn-eine-runde-abbricht.md`
**Betroffen:** `crates/krk-bench/src/messen.rs`
**Domain:** code

## Was schon steht

Der Rest ist im Doc-Kommentar der Struktur `Messplanwaechter` festgehalten, also an der
Stelle, an der ihn der nächste Leser findet. Dieser Datensatz hält ihn zusätzlich fest, damit
er nicht allein im Quelltext steht.

Der Vorgängerdefekt hat die Lücke von neun auf einen Ausgang verkleinert. Vorher blieb die
Datei bei **jedem** Abbruch liegen, und der Abbruch ist bei dieser Messstrecke der gewöhnliche
Fall, weil ein Lauf aus dem Hintergrund `NICHT_IM_VORDERGRUND` meldet. Übrig ist der Fall, in
dem der Messende selbst abbricht.

## Warum es nicht mit erledigt wurde

Die Sitzungssicherung löst dasselbe Problem für die `session.toml` des Nutzers, und zwar über
eine statische `SICHERUNG`, die die Signalwache abarbeitet. Den Messplan dort einzuhängen wäre
ein Eingriff in das dokumentierte „wer zuerst kommt" dieser Mechanik. Das ging über den
Auftrag hinaus und ist eine Entwurfsfrage, keine Ergänzung.

## Denkbarer Weg

Den Pfad des Messplans in dieselbe statische Sicherung eintragen, die die Signalwache ohnehin
abarbeitet, statt einen zweiten Signalpfad zu bauen. Zu klären ist dabei die Reihenfolge: die
Sitzungssicherung spielt die Sitzung des Nutzers zurück, das Löschen des Messplans ist davon
unabhängig, und ob beide in einer Liste stehen oder die Sicherung zwei Aufgaben trägt, gehört
mitentschieden. `crates/krk-bench/src/wegwerfordner.rs` trägt dieselbe Frage und beantwortet
sie heute ebenfalls nicht.

## Dringlichkeit

Gering. Der Schaden ist eine liegengebliebene Datei je Strg+C, mit Prozesskennung im Namen,
also ohne Kollision zwischen zwei Läufen. Kein Abnahmekriterium und keine der zehn Zeitzusagen
aus C8 sind berührt.

---
Nachtrag 260810-1850, Turn 2: Der Weg ist untersucht und **nicht** umgesetzt, weil er die
dokumentierte Zusage der Sitzungssicherung beruehrt. `SICHERUNG` ist ein
`Mutex<Option<Sitzungssicherung>>`, also auf genau eine Nutzlast typisiert, und traegt den Satz
"genau einer von beiden spielt zurueck: wer zuerst kommt". Drei Wege stehen zur Wahl, und die
Wahl gehoert dem Nutzer; sie ist als Entscheidungsdatensatz abgelegt:
`shared/decisions/260810-1850_*_wie-kommt-der-messplan-bei-strg-c-weg-ohne-die-zusage-der-sitzungssicherung-zu-brechen.md`.

Dabei ist ein Nebenbefund aufgefallen, der jede der drei Optionen betrifft: `signalwache_starten`
laeuft erst mit `Sitzungssicherung::anlegen()` bei `messen.rs:1034`, `plan_schreiben` schon bei
`messen.rs:1029`. In dieser Spanne faengt heute gar nichts ein Strg+C ab. Wer die Frage
beantwortet, zieht die Reihenfolge dieser beiden Aufrufe mit.

Dieser Defekt bleibt offen und wartet auf die Antwort.

---
Resolved: Der Nutzer hat am 260810-1915 Option 4 des Entscheidungsdatensatzes
`shared/decisions/260810-1850_*_wie-kommt-der-messplan-bei-strg-c-weg-…` gewaehlt, und sie ist
umgesetzt: nicht der abbrechende Lauf raeumt seinen Plan ab, sondern der naechste.
`Messplanwaechter::neu` loescht beim Anlegen jede `krk-messplan-*.toml` im Temporaerverzeichnis,
die nicht die eigene ist.

Vier Stuecke in `crates/krk-bench/src/messen.rs`: die Konstante `PLANRUMPF` haelt den Namensrumpf
an einer Stelle, weil ihn jetzt zwei tragen; `fremde_plaene_raeumen` laeuft das Verzeichnis ab
und ueberspringt den eigenen Namen; `Messplanwaechter::in_verzeichnis` ist die Naht, ueber die
`neu` das Temporaerverzeichnis uebergibt und die Probe ein eigenes; dazu eine Probe
`ein_neuer_waechter_raeumt_fremde_plaene_ab_und_laesst_den_eigenen_stehen`, die das echte
Temporaerverzeichnis nicht anfasst.

`SICHERUNG`, die Signalwache und die Reihenfolge von `plan_schreiben` und `signalwache_starten`
sind unberuehrt. Die Bauform ist die von `Wegwerfordner::neu`, mit der einen Abweichung, dass
hier ein Verzeichnis abzulaufen ist statt eines vorhersagbaren Namens — fremde Plaene tragen
fremde Prozesskennungen.

**Der Kommentar behauptet nicht, das Problem sei weg.** Er sagt beides: der eigene Prozess raeumt
bei einem Signal nicht ab, der naechste Lauf tut es, und zwischen einem Strg+C und dem naechsten
Lauf steht die Datei weiterhin da. Neu ist allein, dass es bei einer bleibt statt bei neun.

**Beleg:** die neun Altbestandsdateien vom 260805 bis 260807 sind weg. Abgeraeumt hat sie
allerdings `make check` und kein Messlauf, und daraus ist ein eigener Defekt geworden:
`shared/issues/260810-1925_*_eine-probe-schreibt-ins-echte-temporaerverzeichnis-…`.
Abgenommen mit `make check`, exit 0.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 3.
