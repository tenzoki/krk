Die Dateiliste von S17 nennt den alten Modulpfad `umbenennen`

---

Das Modul `krk_core::umbenennen` heißt seit dem 260805 `krk_core::stapelumbenennen`, weil zwei Module des Kerns denselben Namen trugen (`issues/260804-2040_c_zwei-module-des-kerns-heissen-umbenennen.md`). Das Verzeichnis heißt jetzt `crates/krk-core/src/stapelumbenennen/`, die Abnahmedatei `crates/krk-core/tests/stapelumbenennen.rs`.

Die Dateiliste von Schritt 17 in der Plandatei nennt weiter die alten Pfade.

---

## Warum es zählt

Der Datensatz `260804-2040_c_zwei-module-des-kerns-heissen-umbenennen.md` führt die Plandatei selbst als fünften betroffenen Punkt auf. Eine Dateiliste, die auf ein Verzeichnis zeigt, das es nicht mehr gibt, führt jeden in die Irre, der den Schritt später nachliest oder seine Abnahme wiederholt.

## Was zu tun ist

In der Plandatei bei Schritt 17 jeden Pfad unter `crates/krk-core/src/umbenennen/` auf `crates/krk-core/src/stapelumbenennen/` ziehen und `crates/krk-core/tests/umbenennen.rs` auf `crates/krk-core/tests/stapelumbenennen.rs`. Zu prüfen ist dabei, ob auch der Fließtext des Schritts oder sein Abnahmekriterium den Modulnamen nennt.

## Warum es nicht gleich mitbehoben ist

Der Aufräumdurchgang vom 260805 war ausdrücklich auf `crates/` begrenzt; die Plandatei und der Spec waren ausgenommen, und ein `planner` arbeitete parallel in `planning/`.

---

Herkunft: gefunden beim Beheben von `issues/260804-2040_c_zwei-module-des-kerns-heissen-umbenennen.md` am 260805-0947. Jener Datensatz nennt die Dateiliste des Plans als betroffen.
