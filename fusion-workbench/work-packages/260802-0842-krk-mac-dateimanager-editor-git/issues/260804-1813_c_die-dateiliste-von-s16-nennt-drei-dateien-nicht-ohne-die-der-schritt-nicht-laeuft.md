Die Dateiliste von S16 nennt drei Dateien nicht, ohne die der Schritt nicht läuft

---

S16 sagt zu, die Dateioperationen aus S15 "erstmals bedienbar" zu machen. Keine der Tasten aus `resources/default-keymap.toml` konnte sie erreichen, weil `Kommando` die fünf Kennungen nicht kannte. Die Umsetzung hat drei Dateien angefasst, die die Dateiliste von S16 nicht nennt.

---

## Was fehlte

`crates/krk-core/src/tasten/belegung.rs` führt die Aufzählung `Kommando` und die Tabelle `KENNUNGEN`. Sie ist die eine Brücke zwischen der Belegungsdatei und dem, was das Programm ausführt: `Funktion::kommando()` schlägt die Kennung dort nach, und der Ereignisabgriff reicht einen Tastendruck nur weiter, wenn er ein `Kommando` findet. Der Kopf der Aufzählung sagt das selbst — "Sie wächst mit den Schritten, die die übrigen Funktionen bauen".

Die fünf Kennungen `kopieren`, `verschieben`, `in_papierkorb`, `endgueltig_loeschen` und `abbrechen` standen seit S9 in `resources/default-keymap.toml`, hatten aber kein Kommando. F5, F6, Delete, F8 und Esc liefen deshalb ins Leere, und kein Abnahmekriterium von S16 wäre nachweisbar gewesen.

Zwei weitere Dateien fehlen aus demselben Grund in der Liste:

| Datei | Warum sie angefasst werden musste |
|---|---|
| `crates/krk-core/src/tasten/belegung.rs` | Fünf Varianten von `Kommando` und fünf Zeilen in `KENNUNGEN`. Rein additiv; die vorhandene Prüfung `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` deckt sie ab. |
| `crates/krk-ui/src/appkit/anwendung.rs` | Der Anwendungsdelegierte ist die einzige Stelle, die beide Dateifenster hält, und damit die einzige, die "Ordner des anderen Fensters" beantworten kann. Hier steht die Zuleitung der fünf Befehle, der Vermittlerfaden und der Aufruf von `ordner_neu_lesen`. |
| `crates/krk-ui/Cargo.toml` und `Cargo.toml` | `dispatch2` als unmittelbare Abhängigkeit, für den Weckruf des Vermittlerfadens auf die Hauptschlange. Die Kiste lag ohnehin im Baum, weil `objc2-foundation` sie führt. |

Dazu `crates/krk-ui/src/appkit/tabelle.rs`, das die Liste als "lesend" führt: es hat eine Methode `betroffene_eintraege` bekommen, weil die Markierung im Tabmodell wohnt und nur die Datenquelle sie ausleihen darf.

## Warum das ein Eintrag ist und keine Nebenbemerkung

Die Anweisung an den `coder` lautete ausdrücklich "kein Eingriff in `crates/krk-core/`". Sie ist überschritten worden, mit fünf additiven Zeilen in einer Datentabelle, weil der Schritt sonst nichts geliefert hätte, was sich abnehmen lässt. Die Entscheidung gehört dokumentiert und nicht in einer Zusammenfassung versteckt.

## Was zu tun ist

Nichts am Code. Die Dateilisten von S16 und, falls die Kennungen dort früher hingehört hätten, von S15 sind nachzuziehen, damit die nächste Durchsicht der Dateilisten nicht denselben Befund noch einmal macht.

**Aufgefallen bei:** der Umsetzung von Schritt 16 am 260804-1813.

---
Resolved: Die Dateiliste von S16 im Plan nennt jetzt `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, beide `Cargo.toml` samt `Cargo.lock`, und führt `appkit/tabelle.rs` als erweitert statt als lesend. Der Grenzübertritt nach `crates/krk-core/` steht ausdrücklich im Plan, statt dem Umsetzenden überlassen zu bleiben; die Verbotsseite einer Dateiliste bleibt bindend, siehe den neuen Abschnitt "Was eine Dateiliste zusagt, und was nicht". Nachgezogen am 260804-2318 vom `planner`.
