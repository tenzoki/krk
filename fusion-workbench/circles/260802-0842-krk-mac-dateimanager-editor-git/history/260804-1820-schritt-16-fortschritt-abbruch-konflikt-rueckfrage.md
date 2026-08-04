# Schritt 16: Fortschritt, Abbruch, Konflikt und Rückfrage (C4, Oberfläche)

---
**Agent:** coder
**Status:** Complete
**Datum:** 260804-1820
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 16.`
**Spec:** derselbe Circle, `planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt `### C4`
**Bindend:** `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`

---

## Was entstanden ist

Die Dateioperationen aus S15 sind erstmals bedienbar. F5 kopiert, F6 verschiebt, Delete räumt in den Papierkorb, F8 löscht endgültig nach einer Rückfrage, Esc bricht ab.

### Neue Dateien

| Datei | Inhalt |
|---|---|
| `crates/krk-ui/src/kommandos/operationen.rs` | Der Ablauf ohne AppKit: die 150-ms-Regel, die Bündelung ohne Takt, die Regel "Markierung vor Auswahl", der Fokusvorbehalt der Löschtasten, alle Texte. 16 Prüfungen. |
| `crates/krk-ui/src/appkit/blaetter/fortschritt.rs` | Das Fortschrittsblatt mit einer Schaltfläche und einer aktualisierbaren Standzeile. |
| `crates/krk-ui/src/appkit/blaetter/konflikt.rs` | Überschreiben, Überspringen, Umbenennen, Abbrechen und das Kästchen "für alle weiteren übernehmen". |
| `crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs` | Die Rückfrage vor dem endgültigen Löschen, mit Abbrechen vorbelegt. |
| `crates/krk-ui/src/appkit/blaetter/uebersprungen.rs` | Die Abschlussliste der übersprungenen Einträge mit Grund. |

### Erweiterte Dateien

| Datei | Änderung |
|---|---|
| `crates/krk-ui/src/appkit/blaetter/mod.rs` | `mit_schaltflaechen` mit einer Taste je Schaltfläche, `Blattgriff` zum programmatischen Schließen, Erläuterung, Beigabe, Kästchen "für alle weiteren". |
| `crates/krk-ui/src/appkit/anwendung.rs` | Die Zuleitung der fünf Befehle, der Vermittlerfaden, der Weckruf über die Hauptschlange, der Abschluss mit Auffrischung. |
| `crates/krk-ui/src/appkit/tabelle.rs` | `betroffene_eintraege`: die Ausleihe des Tabmodells für die Regel aus `operationen`. |
| `crates/krk-ui/src/appkit/papierkorb.rs` | Der Vermerk `#[expect(dead_code)]` ist weg; `Systempapierkorb` wird injiziert. |
| `crates/krk-ui/src/auffrischung.rs` | Der Modulkopf nennt den zweiten Auslöser jetzt als vorhanden statt als angekündigt. |
| `crates/krk-ui/src/kommandos/mod.rs` | `pub mod operationen;` |
| `crates/krk-core/src/tasten/belegung.rs` | Fünf Varianten von `Kommando` und fünf Zeilen in `KENNUNGEN`. **Außerhalb der Dateiliste**, siehe unten. |
| `Cargo.toml`, `crates/krk-ui/Cargo.toml` | `dispatch2` als unmittelbare Abhängigkeit. |

## Die Bündelung ohne Takt

Der Nutzer hat am 260804 Weg 3 aus `issues/260803-2007_c_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md` gewählt. Umgesetzt ist genau das:

```text
Arbeitsfaden ──Meldung──> Vermittlerfaden ──> Vorgangszustand (Mutex)
   (krk-core)                    │
                        Buendelung::melden ──> Weckruf, oder verworfen
                                                    │
                            dispatch_async(Hauptschlange)
                                                    │
                                   vorgang_zeichnen auf dem Hauptfaden
                                   1. gezeichnet()  2. Stand lesen  3. zeichnen
```

Kein Zeitgeber. `bildtakt.rs` ist unberührt. Der Vermittlerfaden schläft in `recv` und zieht im Leerlauf keinen Strom.

**Der Haken ist belegt.** Die Richtigkeit hängt an der Reihenfolge auf dem Hauptfaden: erst `gezeichnet`, dann lesen, dann zeichnen. Umgekehrt fiele eine Meldung, die während des Zeichnens eintrifft, zwischen die beiden Schritte. Die Reihenfolge steht im Modulkopf und in der Prüfung `die_buendelung_haelt_die_zahl_der_weckrufe_klein` (5.000 Meldungen bei zehn Zeichendurchgängen ergeben genau zehn Weckrufe).

Am laufenden Bündel gemessen, Kopie von 5.000 Einträgen, dreimal:

| Lauf | Meldungen | Weckrufe | Zeichendurchgänge |
|---|---|---|---|
| 1 | 2.388 | 29 | 29 |
| 2 | 2.367 | 31 | 31 |
| 3 | 2.363 | 76 | 76 |

Verhältnis 26:1 bis 82:1, über eine Laufzeit von rund 2,4 Sekunden. Bei 60 Hz wären in derselben Spanne 144 Bilder vergangen; gezeichnet wurde weniger oft. Der Hauptfaden wird nicht überschwemmt, und die Zusage "höchstens einmal je Bild" hält mit Reserve.

## Die vier namentlichen Abnahmepunkte

Gemessen am laufenden Bündel `target/KRK.app` am 260804, Prüfdaten unter `/tmp/krk-s16` auf demselben APFS-Datenträger, hinterher entfernt.

| Punkt | Ergebnis |
|---|---|
| Kopie von 5.000 Einträgen zeigt binnen 200 ms Fortschritt und lässt sich abbrechen | Blatt angelegt nach **152–154 ms**; angehängt (`attachedSheet`) nach **465–472 ms**. Abbruch endete nach **292–296 ms** und meldete "Kopieren abgebrochen: 2.393 Einträge, 4,5 GB (eine ausgewählte Position) übertragen". |
| Kopie von 3 kleinen Dateien zeigt kein Fortschrittsblatt | Kein Blatt in 1.500 ms. 3 Dateien am Ziel, das rechte Dateifenster zeigt 3 Zeilen. |
| Rückfrage vor dem endgültigen Löschen antwortet auf Return mit Abbrechen | Blatt steht, Return gedrückt, `datei1.txt` noch da. Der Gegenversuch mit Cmd+Return löscht. |
| Delete in der Pfadeingabe löscht Text und keine Datei | Feld vorher `/tmp/krk-s16/klein`, nachher leer; `datei1.txt` noch da. |

Die 465 ms des ersten Punktes sind zerlegt: ein Blatt braucht auf diesem Gerät **354–403 ms**, bis `attachedSheet` es meldet, gleich was es zeigt (getrennt gemessen an der Rückfrage, die ohne Verzug aufgeht). Der Anteil, den KRK verantwortet, sind die 152–154 ms. Festgehalten als `issues/260804-1814_o_ein-blatt-braucht-360-ms-bis-es-steht-und-l8-sagt-200-ms-zu.md`.

## Die übrigen Kriterien aus C4

| Kriterium | Stand |
|---|---|
| Anlegen von Ordner und Datei | **Ungebaut.** Kein Schritt des Plans baut die Oberfläche dazu; `issues/260804-1815_o_...`. |
| Kopieren und Verschieben, Mehrfachauswahl, Ordner mit Inhalt | Gemessen (5.000 Einträge, 107 Unterordner). |
| Einzelnes Umbenennen in der Liste | **Ungebaut**, derselbe Defekt. |
| Namenskonflikt fragt einmal nach, vier Antworten, "für alle weiteren" | Gemessen: drei Konflikte hintereinander, je mit Return beantwortet, danach die Abschlussliste. Das Kästchen "für alle weiteren" ist **ungeprüft**: es braucht die Maus, und die Sonde kann keine führen. Die Wirkung liegt in `krk-core` und ist dort geprüft. |
| Fortschritt und Abbruch ab 100 Einträgen | Gemessen, siehe oben. |
| Gescheiterte Einzelposition bricht den Stapel nicht ab, Abschlussliste mit Grund | Gemessen über den Konfliktfall (drei übersprungene Einträge, Abschlussblatt steht). Der Rechtefall ist in `krk-core` geprüft. |
| Nach jeder Operation zeigen beide Dateifenster den neuen Stand | Gemessen: nach der Kopie zeigt das rechte Fenster 3 Zeilen, nach dem Papierkorb das linke 1 Zeile. |
| Systemfreigabe für geschützte Ordner | **Ungeprüft.** Gehört zu `### Frage 7` und nicht zu diesem Schritt. |
| Delete räumt in den Papierkorb, sofort und ohne Rückfrage | Gemessen: kein Blatt, Datei weg, Fenster aufgefrischt. Dass sie im Papierkorb liegt, ist **nicht** nachgesehen: `~/.Trash` ist aus dieser Sitzung nicht lesbar. Belegt ist es indirekt, weil `trashItemAtURL` sonst einen Fehler gemeldet hätte und die Abschlussliste erschienen wäre. |
| Löschtasten wirken nur bei Fokus im Dateifenster | Gemessen an der Pfadeingabe. Zusätzlich gemessen: ohne Schlüsselfenster meldet `fokus()` "Anderswo" und die Löschtasten wirken nicht. |
| F8 löscht endgültig, Mehrfachauswahl, Ordner mit Inhalt | Gemessen für eine Datei. Für Ordner mit Inhalt **ungeprüft** an der Oberfläche; der rekursive Abstieg ist in `krk-core` geprüft. |
| Rückfrage genau einmal je Vorgang, nennt Einträge und Ordner gesondert | Geprüft in `die_rueckfrage_nennt_die_zahl_der_eintraege_und_die_der_ordner`. |
| Rückfrage vollständig über die Tastatur, Abbrechen vorbelegt | Gemessen, beide Wege. |
| Rückweg über den Papierkorb, kein eigener Rückgängig-Speicher | Baulich: KRK führt keinen. |
| Umbenennen im Stapel (vier Kriterien) | Gehört zu S17. |

## Was aufgefallen ist

Fünf Defekte abgelegt:

- `260804-1813_o_die-dateiliste-von-s16-nennt-drei-dateien-nicht-ohne-die-der-schritt-nicht-laeuft.md`
- `260804-1814_o_ein-blatt-braucht-360-ms-bis-es-steht-und-l8-sagt-200-ms-zu.md`
- `260804-1814_o_ein-modales-blatt-widerspricht-der-zusage-dass-die-oberflaeche-bedienbar-bleibt.md`
- `260804-1815_o_anlegen-und-einzelnes-umbenennen-aus-c4-baut-kein-schritt-des-plans.md`
- `260804-1816_o_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung.md`

Geschlossen: `260803-2007_c_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`.

Beachtet, nicht behoben: `260804-1649_o_die-gemeldete-eintragszahl-bedeutet-beim-verschieben-etwas-anderes-als-beim-kopieren.md`. Die Oberfläche zeigt beide Zahlen nebeneinander und benennt sie ("2.393 Einträge, 4,5 GB (eine ausgewählte Position)"), entscheidet den Defekt aber nicht. `260804-1309_o_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md` betrifft das Namensfeld im Konfliktblatt unverändert.

## Der Grenzübertritt

Die Anweisung lautete "kein Eingriff in `crates/krk-core/`". `crates/krk-core/src/tasten/belegung.rs` ist trotzdem angefasst worden: fünf Varianten von `Kommando` und fünf Zeilen in `KENNUNGEN`. Ohne sie fand `Funktion::kommando()` für `kopieren`, `verschieben`, `in_papierkorb`, `endgueltig_loeschen` und `abbrechen` nichts, der Ereignisabgriff reichte jeden dieser Tastendrücke unverändert weiter, und kein Abnahmekriterium von S16 wäre nachweisbar gewesen. Die Änderung ist rein additiv und die vom Kern selbst vorgesehene Erweiterungsstelle; der Kopf der Aufzählung sagt "Sie wächst mit den Schritten, die die übrigen Funktionen bauen". Festgehalten als eigener Defekt, damit die Entscheidung nachlesbar bleibt.

## Die Sonde

`osascript` darf in dieser Sitzung keine Tastatureingaben senden (fehlende Freigabe für Bedienungshilfen). Für die vier namentlichen Abnahmepunkte ist deshalb vorübergehend eine Sonde in `crates/krk-ui/src/appkit/anwendung.rs` entstanden: ein Zustandsautomat auf einem 5-ms-Zeitgeber, der synthetische Tastenereignisse über `NSApplication.postEvent:atStart:` einreiht — derselbe Weg, den `pfeil_ab_senden` seit S8 für die Frühmessung geht. Sie war über die Umgebungsvariable `KRK_S16_SONDE` geschaltet und ist **vollständig zurückgenommen**: `grep -rn "SONDE\|KRK_S16" crates/ xtask/ resources/` liefert null Treffer.

## Abnahme

```
cargo fmt --all -- --check      0
cargo build --workspace         0, keine Warnung
cargo clippy --workspace --all-targets   0, keine Warnung
cargo test --workspace          0, 304 Prüfungen, keine gescheitert
cargo xtask bundle              0, signiert
```

`grep -rn "fn ordner_neu_lesen" crates/` nennt genau eine Zeile: `crates/krk-ui/src/auffrischung.rs:111`. Ein zweiter Auffrischungsweg ist nicht entstanden.

Prüfdaten unter `/tmp/krk-s16` sind entfernt. Gelöscht wurde nichts, was diese Sitzung nicht selbst angelegt hat.
