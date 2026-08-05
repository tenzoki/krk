Ein verdeckter Tab auf einem ausgeworfenen Datenträger behält seinen toten Pfad

---

S14 holt beim Auswerfen eines Datenträgers den **sichtbaren** Tab jedes
betroffenen Dateifensters auf das Benutzerverzeichnis herunter. Ein verdeckter
Tab, der auf denselben Datenträger zeigt, bleibt stehen. Wechselt der Nutzer
später auf ihn, sieht er eine leere Liste; erst der nächste Lesevorgang meldet
den Grund in der Statuszeile.

---

**Warum es so gebaut ist.** C9 formuliert die Zusage am Dateifenster: "Wird ein
eingehängtes Volume während der Arbeit ausgeworfen, meldet das betroffene
Dateifenster den Verlust und wechselt auf einen erreichbaren Ordner." Die
Meldung kann nur der sichtbare Tab tragen, weil die Statuszeile zum
Dateifenster gehört und nicht zum Tab. Für den verdeckten Tab gäbe es also
einen Wechsel ohne Meldung, und `crates/krk-ui/src/auffrischung.rs` müsste
dafür eine zweite Regel bekommen.

**Wie groß der Schaden ist.** Klein. Der verdeckte Tab hält einen Pfad, den es
nicht mehr gibt; beim Hinwechseln liest KRK ihn, scheitert und meldet den
Grund. Das ist derselbe Weg, den jeder inzwischen gelöschte Ordner nimmt, und
er ist nicht still. Der Fall tritt außerdem nur ein, wenn der Nutzer zwei Tabs
desselben Dateifensters auf denselben Datenträger gestellt hat.

**Was zu entscheiden wäre.** Drei Möglichkeiten: es so lassen und die
Einschränkung in C9 ausschreiben; jeden Tab herunterholen und die Meldung
weiterhin nur einmal je Dateifenster zeigen; oder den verdeckten Tab beim
nächsten Hinwechseln stillschweigend auf das Benutzerverzeichnis stellen. Die
zweite ist die geradlinigste, verlangt aber eine Naht in
`crates/krk-ui/src/tabs.rs`, die es heute nicht gibt.

**Aufgefallen bei:** der Umsetzung von S14 am 260804,
`history/260804-1451-s14-dateisystem-beobachtung-und-datentraegerwechsel.md`.

---
Resolved: Möglichkeit 2 des Datensatzes, jeden Tab herunterholen und die Meldung weiterhin nur einmal je Dateifenster zeigen.

**Warum nicht die anderen beiden.** Möglichkeit 1 (es so lassen und die Einschränkung in C9 ausschreiben) verlangte eine Änderung am Spec, den dieser Durchgang nicht anfassen durfte, und sie schriebe eine Lücke fest, statt sie zu schließen. Möglichkeit 3 (den verdeckten Tab beim nächsten Hinwechseln stillschweigend umstellen) wäre ein zweiter Auslöser für dieselbe Sache, an einer anderen Stelle und mit einer anderen Bedingung: der Tabwechsel müsste dann wissen, welche Datenträger seit wann fort sind. Möglichkeit 2 hält den Auswurf an einem Ereignis und an einer Stelle.

**Die Naht, die der Datensatz in `tabs.rs` erwartet, ist `Tabliste::verdeckten_tab_setzen`.** Sie ersetzt einen verdeckten Tab durch einen ungelesenen auf dem Ausweichziel und startet **keinen** Lesevorgang: auf keinem Schirm steht etwas, das nachzuziehen wäre, und die zweite Stufe der Lesereihenfolge ist zu diesem Zeitpunkt längst gelaufen. Gelesen wird er, sobald der Nutzer auf ihn wechselt, über das vorhandene `ungelesenen_aktiven_nachlesen` in `waehlen`. Sortierung und Filter des Tabs bleiben erhalten, wie bei `ordner_setzen`.

**Die Entscheidung bleibt AppKit-frei.** `crate::auffrischung` bekommt zwei neue Fragen an die `Dateifenstersicht`, `tabordner` und `sichtbarer_tab`, und `wechseln(seite, ziel)` wird zu `tab_wechseln(seite, stelle, ziel)`. Welcher Tab getroffen ist, rechnet weiter `liegt_auf` in dieser Datei; ob dabei gelesen wird, entscheidet `Dateifenster::tab_ordner_setzen` an der Ansicht. Die Tableiste zieht dort nach, weil sie den Ordnernamen je Tab zeigt.

**Die Meldung sagt jetzt, was umgezogen ist.** Der alte Satz "das Dateifenster zeigt jetzt X" wäre falsch, wenn allein ein verdeckter Tab umgezogen ist. `auswurfmeldung` unterscheidet vier Fälle: nur der sichtbare, der sichtbare und ein verdeckter, der sichtbare und mehrere verdeckte, nur verdeckte. Eine Meldung je Dateifenster bleibt es, wie C9 es formuliert: die Statuszeile gehört dem Fenster und nicht dem Tab.

Drei neue Prüfungen in `crates/krk-ui/src/auffrischung.rs`, alle ohne Fenster: `ein_verdeckter_tab_auf_dem_datentraeger_zieht_mit_um`, `sichtbarer_und_verdeckte_tabs_ziehen_zusammen_um`, `ein_dateifenster_ohne_getroffenen_tab_bekommt_keine_meldung`.

**Nachgemessen am laufenden, signierten Bündel am 260805-0930**, an einem eigens angelegten Datenträger. `hdiutil create -size 10m -fs APFS -volname KrkPruef`, eingehängt als `/Volumes/KrkPruef` mit den Ordnern `fotos` und `musik`. Ein Dateifenster mit fünf Tabs, sichtbar `man`, zwei verdeckte Tabs auf dem Datenträger. Abgelesen über die Bedienungshilfen an der Tableiste und an der Statuszeile:

```
vorher  tabs:  krk-s17b[0] man[1] fotos[0] musik[0] man[0]
vorher  zeile: (leer)
$ hdiutil detach /Volumes/KrkPruef
nachher tabs:  krk-s17b[0] man[1] k1[0] k1[0] man[0]
nachher zeile: KrkPruef wurde ausgeworfen; 2 verdeckte Tabs zeigen jetzt /Users/k1
```

Beide verdeckten Tabs sind umgezogen, der sichtbare steht unverändert, und die Meldung behauptet nichts über ihn. Danach mit Ctrl+Tab auf einen der umgezogenen Tabs gewechselt: `zeilen=12`, das Benutzerverzeichnis steht da, keine leere Liste. Genau das war der Schaden des Defekts.

Der Prüfdatenträger ist ausgehängt und `/tmp/krk-pruef.dmg` entfernt; `ls /Volumes/` zeigt nur noch `Macintosh SSD`.

Geprüft am 260805-0930: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
