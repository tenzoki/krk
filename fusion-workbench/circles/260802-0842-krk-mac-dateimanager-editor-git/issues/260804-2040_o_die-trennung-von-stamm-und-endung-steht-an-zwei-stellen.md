Die Trennung von Stamm und Endung eines Dateinamens steht an zwei Stellen

---

Zwei Module beantworten dieselbe Frage: wo hört der Stamm eines Dateinamens auf
und wo fängt die Endung an.

- `crates/krk-core/src/operation/umbenennen.rs`, `namen_teilen` (privat, aus
  S15): eine eigene Rechnung über `rfind('.')`, für `freier_name`.
- `crates/krk-core/src/umbenennen/regel.rs`, `stamm_und_endung` (aus S17): der
  Weg über `std::path::Path::file_stem` und `extension`, für die fortlaufende
  Nummerierung.

---

Beide liefern heute dasselbe Ergebnis; das ist geprüft (`archiv.tar.gz` →
Stamm `archiv.tar`, `.gitignore` → Stamm ohne Endung, `liesmich` → keine
Endung). Genau das ist die Lage, in der eine Abweichung später unbemerkt
entsteht: die erste Änderung an einer der beiden Stellen findet keine Prüfung,
die sie gegen die andere hält.

S17 konnte es nicht auflösen: `operation/umbenennen.rs` steht in der Dateiliste
des Plans als **lesend**, und `namen_teilen` ist privat. Die Auflösung ist
klein: `namen_teilen` öffentlich machen (oder nach `crate::verzeichnis` ziehen)
und `stamm_und_endung` durch einen Aufruf ersetzen. Die Signaturen
unterscheiden sich leicht — `namen_teilen` liefert die Endung mit ihrem Punkt
als `&str`, `stamm_und_endung` als `String` —, das ist beim Zusammenlegen zu
vereinheitlichen.

Gefunden bei der Umsetzung von Schritt 17.
