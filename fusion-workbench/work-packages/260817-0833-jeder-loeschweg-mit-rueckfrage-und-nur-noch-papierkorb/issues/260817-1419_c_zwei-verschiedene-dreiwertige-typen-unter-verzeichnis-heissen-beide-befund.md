Zwei verschiedene dreiwertige Typen unter `verzeichnis` heißen beide `Befund`

---

`krk_core::verzeichnis::Befund` (neu, `Ja`/`Nein`/`Unentschieden`) und
`krk_core::verzeichnis::modell::Befund` (seit der Runde 10, `Unentschieden`/`Treffer`/`KeinTreffer`)
sind zwei verschiedene Typen mit demselben Namen, im selben Modulbaum, beide dreiwertig, beide
mit einer Variante `Unentschieden`. Sie unterscheiden sich für den Leser um einen Pfadabschnitt.

---

**Schwere:** Niedrig. Der Übersetzer trennt sie: es sind zwei Typen, und eine Verwechslung
übersetzt nicht. Der Preis ist Lesbarkeit, und er trifft die Stelle, an der dieses Projekt
seine Regel „eine Benennung pro Sache" führt.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `crates/krk-core/src/verzeichnis/befund.rs:95`,
`crates/krk-core/src/verzeichnis/modell.rs:191`, `crates/krk-core/src/verzeichnis/mod.rs:74-82`, `:101`
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

```
krk_core::verzeichnis::Befund          befund.rs:95    Ja | Nein | Unentschieden
krk_core::verzeichnis::modell::Befund  modell.rs:191   Unentschieden | Treffer | KeinTreffer
```

Der neue ist über `mod.rs:101` als `pub use befund::Befund;` re-exportiert. Der ältere ist
**nicht** re-exportiert (`mod.rs:106` führt aus `modell` allein `Markierungsstand` und
`Ordnermodell`), also gibt es keinen Namenskonflikt im Übersetzer. Beide sind im Gebrauch:

```
crates/krk-ui/src/appkit/papierkorb.rs:116   use krk_core::verzeichnis::Befund;
crates/krk-core/tests/verzeichnis.rs:17      use krk_core::verzeichnis::modell::{Befund, Ordnermodell};
```

Der neue Absatz im Modulkopf von `verzeichnis/mod.rs` (`:74-82`) erklärt `befund` ausführlich
und nennt den gleichnamigen Nachbarn nicht. Er sagt „Es traegt die dreiwertige Antwort
[`Befund`]" — in einem Modulkopf, dessen Modulbaum zwei dreiwertige `Befund` führt.

Dazu kommt `durchlauf::Befundmeldung`, über `mod.rs:102` re-exportiert und dritter Träger des
Wortstamms in demselben Namensraum.

## Warum das mehr ist als Geschmack

Beide beantworten eine dreiwertige Frage an einen Pfad, und beide führen `Unentschieden` mit
derselben Bedeutung: „KRK konnte es nicht entscheiden". `befund.rs:34-42` verweist selbst auf
die Herkunft dieser Haltung im `durchlauf`, also auf die Maschine, die den anderen `Befund`
füllt. Ein Leser, der den Verweis verfolgt, landet bei einem Typ mit demselben Namen und
anderen Varianten, und nichts an der Stelle sagt ihm, dass es zwei sind.

Bündel C legt drei weitere Prüfungen auf den neuen Typ (`arbeitsbaum`, `umfang`, `volumes`),
davon zwei in `krk-core/src/verzeichnis/` — also in demselben Verzeichnis wie der ältere.

## Richtung

Einer der beiden bekommt einen sprechenden Namen. Der ältere heißt der Sache nach
`Trefferbefund` oder `Filterbefund` (er beantwortet „trägt der Unterbaum die Folge"), der neue
`Zielbefund` (er beantwortet Fragen an ein Löschziel). Der ältere hat mehr Aufrufstellen, der
neue ist zwei Commits alt und billiger umzubenennen.

Die kleine Form: ein Satz im Modulkopf von `verzeichnis/mod.rs`, der die beiden nebeneinander
stellt und sagt, welcher welche Frage beantwortet. Sie kostet nichts und nimmt dem nächsten
Leser den Umweg; sie nimmt der nächsten Erweiterung aber nicht die Verwechslungsgefahr.

---
Resolved: 260817-1504 — Der neue Typ heißt `krk_core::verzeichnis::Loeschzielbefund`, sein Modul
`loeschzielbefund` (Datei `crates/krk-core/src/verzeichnis/loeschzielbefund.rs`).
`modell::Befund` aus der Runde 10 behält seinen Namen unverändert; unter `verzeichnis` trägt
damit kein zweiter Typ mehr denselben Namen, nachgezählt mit einer Suche über alle
Typdeklarationen des Modulbaums (`grep -rhoE '^[[:space:]]*(pub(\([^)]*\))?[[:space:]]+)?(enum|struct|trait|union|type)[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' crates/krk-core/src/verzeichnis | awk '{print $NF}' | sort | uniq -d`
liefert keine Zeile.

Warum der neue und nicht der ältere, warum der Wortstamm bleibt und warum `Zielbefund`
verworfen ist, steht im Modulkopf von `loeschzielbefund.rs` unter
`# Warum der Typ nicht Befund heisst`. Der Modulkopf von `verzeichnis/mod.rs` nennt den
gleichnamigen Nachbarn jetzt ausdrücklich: `modell::Befund`, `Befundmeldung` und
`Inhaltsbefund` gehören zusammen, `Loeschzielbefund` nicht zu ihnen.

`durchlauf::Befundmeldung` bleibt, wie es ist: es meldet den Filterbefund und gehört damit zur
Familie des älteren Typs, nicht zu der des neuen. Verhalten ist an keiner Stelle geändert;
`make check` läuft grün.
