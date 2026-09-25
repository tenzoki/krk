# Der Eingabefokus liegt beim Start im aktiven Dateifenster

---
**Status:** Complete
**Agent:** coder
**Datum:** 260805-1901
**Defekt:** `issues/260805-1845_c_beim-start-liegt-der-fokus-in-der-leiste-und-nicht-im-dateifenster.md`
**Herkunft:** Prüflauf von Schritt 18c, verursacht von Schritt 18

---

## Was der Defekt war

Nach dem Aufbau der Oberfläche setzte niemand den Eingabefokus. AppKit vergab
ihn beim ersten Anzeigen selbst, an die erste Ansicht der
Schlüsselansichtskette, und das ist seit S18 die Lesezeichen- und Geräteleiste.
`Anwendungsdelegierter::fokus` lieferte deshalb `Fokus::Leiste`, und jeder der
31 Befehle mit `Wirkungsbereich::Dateifenster` wurde stumm abgewiesen, bis der
Nutzer den Fokus einmal von Hand setzte. Ein Nutzer, der KRK startet und die
Pfeiltaste drückt, sah eine Anwendung, die nichts tut.

## Wo der Fokus jetzt gesetzt wird

`oberflaeche_aufbauen` setzt ihn als **letzte Zeile des Aufbaus**, nach
`makeKeyAndOrderFront` und vor den Startmeldungen:

```rust
self.fokus_setzen(fokus::BEIM_START);
```

Drei Entscheidungen stecken darin.

**Nach `makeKeyAndOrderFront` und nicht davor.** AppKit vergibt den Ersthelfer
beim ersten Anzeigen des Fensters selbst. Eine Zeile davor wäre überschrieben
worden, und der Defekt stünde unverändert da, mit einer Zeile Programmtext, die
aussieht, als behöbe sie ihn.

**Über `fokus_setzen` und nicht mit einem eigenen `makeFirstResponder`.** Der
Modulkopf hält seit S18 fest, dass es genau eine Stelle gibt, die den Fokus
setzt, und genau eine, die ihn liest. Der Aufbau ist jetzt der dritte Aufrufer
dieser einen Stelle, neben den beiden Fokusbefehlen aus C5 und dem Ausblenden
der Leiste. Damit erbt er den Vorbehalt, den `fokus_setzen` schon trägt: in eine
ausgeblendete Leiste geht der Fokus nicht.

**Eine eigene Zeile am Ende des Aufbaus und keine Zeile der
Sitzungswiederherstellung.** Die Wiederherstellung aus S10 setzt, was
gespeichert ist. Der Fokus ist nicht gespeichert (siehe unten), also hat er
dort nichts zu suchen; er ist ein fester Anfangszustand und keine
wiederhergestellte Angabe. Aus der Sitzung kommt allein, **welches** der beiden
Dateifenster ihn bekommt: `fokus_setzen(Fokus::Dateifenster)` fragt das
Fenstermodell nach dem aktiven, und das kommt aus `session.toml`.

## Warum der Fokus nicht gespeichert wird

C7 zählt auf, was Beenden und Neustart überstehen soll: Tabs, Ordner, Auswahl,
Breiten, Sichtbarkeit und Sortierung. Der Fokus steht nicht in der Liste, und
das ist die richtige Wahl, nicht bloß eine Auslassung. Er ist keine Einstellung,
die der Nutzer trifft, sondern der Ort, an dem er zuletzt getippt hat. Ein Start
in die Leiste, weil die letzte Handlung vor dem Beenden ein `shift+cmd+l` war,
wäre für ihn nicht vorhersagbar; C1 macht die beiden Dateifenster zur Mitte der
Anwendung, und dort fängt jede Sitzung an.

Der Startzustand ist damit immer derselbe. Festgehalten ist er als Konstante
`kommandos::fokus::BEIM_START` — in dem Modul ohne AppKit, damit die Zusage
ohne Fenster prüfbar ist und nicht nur als Kommentar an einer Aufrufstelle
steht.

## Der zweite uninitialisierte Zustand

Gesucht war, ob beim Aufbau noch etwas anderes ungesetzt bleibt und nur deshalb
nicht auffällt, weil ein späteres Ereignis es nachholt. Einer war es:

**`Fenstermodell::aus_sitzung` übernahm das aktive Dateifenster ungeprüft.**
`umschalten` hält zur Laufzeit die Zusicherung, dass ein ausgeblendetes
Dateifenster nicht das aktive sein kann; beim Ausblenden des rechten wandert die
Aktivität nach links. `session.toml` kommt aber nicht von dort. Die Datei ist
nach C7 zum Lesen und Ändern von Hand gedacht, und `aktiv = "rechts"` neben
`zweites_dateifenster = false` liest `serde` anstandslos ein. Der Nutzer fände
dann seine Auswahl, seinen Eingabefokus und jede Dateioperation in einem
Dateifenster, das er nicht sieht — und mit der Zeile oben nun auch buchstäblich
den Fokus. `aus_sitzung` stellt die Zusicherung deshalb selbst her, mit einer
Prüfung ohne Fenster daneben.

Zwei weitere Kandidaten habe ich geprüft und für richtig befunden. Die
Markierung des aktiven Dateifensters wird beim Aufbau über
`aufteilung_nachziehen` gesetzt und hängt nicht an einem Ereignis. Die Leiste
startet ohne Auswahl, und das ist beabsichtigt: `auswahl_bewegen` fängt bei der
ersten wählbaren Zeile an, gleich in welche Richtung.

## Was **nicht** betroffen war

Die Vermutung lag nahe, dass die Messstrecke denselben Defekt trägt: L1 löst
zwanzig synthetische Pfeil-ab-Ereignisse aus, und die Messung zählt einen
Tastendruck erst, wenn die Auswahl umspringt. Gemessen ist das Gegenteil. Zwei
`durchstich`-Läufe über kleine Prüfordner, einer mit und einer ohne die neue
Zeile, liefern beide `L1 100.0 % (20/20)`. Der Grund: das gemessene Fenster ist
nicht das Schlüsselfenster der Sitzung, `fokus` liefert `Fokus::Anderswo`, und
dieser Wert geht nach der Regel aus S18 an das Dateifenster. Die beiden
Prüfberichte sind gelöscht, weil ihre Prüfordner mit 400 Einträgen die
Bedingungen aus C8 nicht erfüllen und ein Bericht unter `messungen/` sonst als
Messung gelesen würde.

## Was am laufenden Bündel geprüft ist

**Ohne vorher eine Taste zu drücken, die den Fokus setzt** — das ist die
Prüfung, die der Defekt verlangt, und die jeder bisherige Prüflauf umgangen hat.

Geprüft mit einer **vorübergehenden Sonde** nach dem Muster von S13, S16, S16b,
S17 und S18c: ein Zustandsautomat auf einem 700-ms-Zeitgeber, der synthetische
Tastenereignisse über `NSApplication.postEvent:atStart:` in die **eigene**
Ereignisschlange stellt. Dieser Weg und kein `osascript`: bei S18 war ein
synthetischer Tastendruck einmal in ein fremdes Fenster gelaufen, weil KRK den
Vordergrund verloren hatte, und ein Ereignis in der eigenen Schlange kann das
per Bau nicht. Die Sonde druckte je Takt den Fokus, das aktive Dateifenster und
den angezeigten Ordner; gelesen wurde über `open --stdout`.

Die Sitzung stand auf `/private/tmp/krk-fokus-pruefung` mit `auswahl =
"unterordner"`, das linke Dateifenster aktiv.

| Takt | mit der neuen Zeile | ohne sie (Gegenprobe) |
|---|---|---|
| 0 | `fokus=Dateifenster`, Ordner `…/krk-fokus-pruefung` | `fokus=Leiste`, derselbe Ordner |
| 1 | `right` gesendet | `right` gesendet |
| 2 | Ordner `…/krk-fokus-pruefung/unterordner` | Ordner unverändert |
| 3 | `ctrl+o` gesendet | `ctrl+o` gesendet |
| 4 | `fokus=Anderswo` (Terminal ist vorn) | `fokus=Leiste`, nichts gestartet |

Terminal.app lief vorher nicht und startete um 18:58:44; die Sitzung darin trägt
`cwd=/private/tmp/krk-fokus-pruefung/unterordner`, gelesen über `lsof -a -p
<shell> -d cwd`. Die Gegenprobe startete keine.

Die Gegenprobe ist der eigentliche Wert dieses Prüflaufs: sie zeigt, dass die
Sonde den Unterschied überhaupt sehen kann.

**Die Sonde ist zurückgenommen.**
`grep -rniE 'SONDE_FOKUS|SONDENSCHRITT|sonde_weiter|sonde_einrichten|sonde_senden|sonde_rechts|sonde_ctrl|sondeSchritt'`
über `crates/`, `xtask/`, `resources/` und das `Makefile` liefert null Treffer.
Der Prüfordner ist gelöscht, `session.toml` steht Byte für Byte wieder auf dem
Stand vor der Sitzung, und die beim Prüfen angelegte `settings.toml` ist
entfernt; der Ablageordner enthält wieder allein `session.toml`. Das
zurückgebaute Bündel ist neu gebaut, signiert und einmal gestartet.

## Abnahmekommandos

`make check` fährt alle vier grün. Zwei neue Prüfungen, beide ohne Fenster:

| Prüfung | Was sie hält |
|---|---|
| `kommandos::fokus::tests::nach_dem_start_wirkt_jeder_befehl_des_dateifensters` | Beim Startfokus wirkt jeder Befehl mit `Wirkungsbereich::Dateifenster`. Sie zählt die Befehle nicht auf, sondern geht über `Kommando::KENNUNGEN`, deckt also auch jeden späteren mit ab. |
| `fenstermodell::tests::ein_ausgeblendetes_dateifenster_kommt_nicht_als_aktives_aus_der_sitzung` | Eine von Hand geänderte `session.toml` macht kein unsichtbares Dateifenster zum aktiven. |

Der Fokus selbst liegt weiter in AppKit und nicht im Modell — der Ersthelfer des
Fensters ist die eine Wahrheit darüber, und ein Feld daneben wäre eine zweite,
die jeder Mausklick nachzuziehen hätte. Prüfbar ohne Fenster ist deshalb die
**Regel**, nicht der Zustand; genau dafür steht `BEIM_START` in dem Modul ohne
AppKit.

## Berührte Dateien

Geändert: `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/fenstermodell.rs`, `crates/krk-ui/src/kommandos/fokus.rs`.

Nur gelesen und unverändert: `crates/krk-ui/src/appkit/ereignisse.rs`,
`crates/krk-ui/src/appkit/aufteilung.rs`, `crates/krk-ui/src/appkit/fenster.rs`,
`crates/krk-ui/src/appkit/leiste.rs`, `crates/krk-ui/src/appkit/tabelle.rs`,
`crates/krk-ui/src/leistenmodell.rs`, `crates/krk-ui/src/messmodus.rs`,
`crates/krk-core/src/ablage/sitzung.rs`,
`crates/krk-core/src/tasten/belegung.rs`.

Nicht angefasst, wie beauftragt: `resources/`, `crates/krk-bench/`, `xtask/`,
die Plandatei und der Spec.

## Nicht committet

Wie beauftragt. Der Commit liegt beim Orchestrator.
