Die MS-DOS-Zeitprobe misst in einer Zone ohne Sommerzeit nichts, und nichts hält ihre Voraussetzung

---

`das_msdos_feld_traegt_die_ortszeit_des_quelldatums` (`crates/krk-core/tests/operation.rs:1370-1418`) soll den Fehler fangen, den `ditto(1)` macht: einen Zonenversatz **je Lauf** statt je Zeitpunkt. Sie fängt ihn nur, wenn das Prüfgerät in einer Zone mit Sommerzeit steht. Unter `TZ=UTC`, in Asien oder auf einem Gerät, dessen Zonendatenbank fehlt, liefern der Sommer- und der Winterzeitpunkt denselben Versatz, und ein Packlauf mit dem Fehler bestünde die Probe. Keine Zusicherung hält die Voraussetzung fest.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/operation.rs:1370-1418`, `:1266-1276` (die zwei Zeitpunkte samt Begründung); daneben `:1463-1502` (`der_rundweg_erhaelt_das_aenderungsdatum_…`, dieselbe Zonenabhängigkeit, dort aber ohne diese Zusage)

## Was der Doc-Kommentar zusagt

```
/// **Die zwei Zeitpunkte sind der Gegenstand und nicht die Verdopplung einer
/// Aussage.** Sie liegen in verschiedenen Halbjahren; in einer Zone mit
/// Sommerzeit gilt an ihnen ein verschiedener Versatz. Ein Packlauf, der den
/// Versatz einmal je Lauf holte, kaeme bei einem der beiden eine Stunde daneben
/// heraus, und die Probe wuerde rot.
```

„In einer Zone mit Sommerzeit" ist eine Bedingung, und der Satz nennt sie. Der Rumpf prüft sie nicht.

## Warum sie ohne die Bedingung nichts sagt

Der Rumpf rechnet die Erwartung mit **derselben** Funktion aus, die der Packlauf nimmt (`operation.rs:1387`, gegen `src/operation/zippen.rs:701`):

```rust
let erwartet = krk_core::verzeichnis::sys::ortszeit(zeitpunkt(sekunden))…;
let steht_da = archivzeit(&archiv, &format!("quelle/{name}"));
assert_eq!( (steht_da.year(), …), (erwartet.jahr, …) );
```

Das ist richtig so — die Zusage ist „je Zeitpunkt gerechnet", nicht „richtig gerechnet"; letzteres hält `tests/zeit.rs` mit festen Kalenderwerten. Die Trennschärfe kommt allein daraus, dass `ortszeit(SOMMER)` und `ortszeit(WINTER)` **verschiedene** Versätze liefern. Tun sie das nicht, ist die Zusicherung eine Tautologie: beide Seiten kommen aus derselben Rechnung, und ein Packlauf mit einem Versatz je Lauf träfe denselben Wert.

Die Zonen, in denen das eintritt, sind nicht exotisch: `UTC` (die Vorgabe vieler Bauumgebungen), jede Zone Asiens, jede Zone ohne Umstellung, und jedes Gerät, auf dem `/usr/share/zoneinfo` fehlt — den letzten Fall benennt `tests/zeit.rs:185-186` ausdrücklich als denkbar.

## Die Vorlage steht daneben und ist nicht abgeschrieben

`tests/zeit.rs` löst genau dieses Problem: es startet Kindprozesse mit gesetztem `TZ` und rechnet nicht in der Zone des Geräts (`zeit.rs:10-19, 66-77`). Der Modulkopf dort sagt, warum: „Eine Zusicherung auf einen festen Kalenderwert haengt an der Zone des Pruefgeraets." `operation.rs` läuft in der Zone des Geräts und hat dieselbe Abhängigkeit, ohne sie zu benennen.

## Was der Baum sonst an dieser Stelle tut

Diese Datei und ihre Nachbarn kennen das Mittel gegen eine Probe, die unter bestimmten Umständen nichts misst, und setzen es an einem Dutzend Stellen ein — die Gegenprobe, die den Aufbau selbst prüft:

- `operation.rs:1544-1547`: „die Probe misst nicht, was sie messen soll: das erweiterte Zeitfeld steht doch da"
- `umfang.rs:195-199`: „der Baum hinter der Verknuepfung ist nicht gross; die Probe sagte nichts aus"
- `belegung.rs:786-789`, `:857-860`, `:1000-1003`: `assert!(geprueft > 0, …)`

Hier fehlt sie.

## Richtung

Eine Zeile am Anfang der Probe, in der Form, die die Nachbarn schon fahren:

```rust
assert_ne!(
    ortszeit(zeitpunkt(SOMMER)).stunde, ortszeit(zeitpunkt(WINTER)).stunde,
    "diese Zone kennt keine Sommerzeit; die Probe kann den Fehler je Lauf nicht sehen"
);
```

Das macht die Probe auf einem UTC-Gerät **rot** statt still bedeutungslos. Ob das die richtige Antwort ist oder ob die Probe stattdessen wie `zeit.rs` in einen Kindprozess mit gesetztem `TZ` umziehen soll, ist eine Abwägung: die erste Fassung kostet eine Zeile und bricht jeden Lauf in UTC, die zweite kostet die Kindprobenform und misst überall. Der Baum baut heute nur auf einem Referenzgerät; die erste Fassung genügt dafür und sagt wenigstens, wenn sie nicht mehr genügt.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
