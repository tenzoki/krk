Die AppKit-Grenzprüfung übersieht `pub use objc2…` und `use ::objc2…`
---
`ist_objc2_use` in `xtask/src/release.rs:176-182` erkennt nur Zeilen, die (nach Einrückung) mit `use` beginnen und deren Pfad mit `objc2` anfängt. Zwei gültige Rust-Schreibweisen passieren die Prüfung ungehindert:

1. `pub use objc2_app_kit::NSView;` — die Zeile beginnt mit `pub`, nicht mit `use`. Ein Modul außerhalb von `appkit/` könnte so einen AppKit-Typ reexportieren, und jeder weitere Verbraucher bräuchte danach selbst keine `use objc2`-Zeile mehr.
2. `use ::objc2::rc::Retained;` — nach `use` folgt `::`, nicht `objc2`; `starts_with("objc2")` greift nicht.

Das im Abnahmekriterium von S23 verankerte Grep (`^[[:space:]]*use +objc2`) hat dieselben zwei Lücken; Rust-Prüfung und Grep sind also konsistent zueinander, aber beide unterlaufbar. Heute gibt es keinen Verstoß (geprüft am 260806: `grep -rEn 'pub use objc2|use ::objc2' crates/krk-ui/src` liefert nichts) — die Lücke ist latent, nicht akut.
---
Kontext: S23. Die maschinelle Grenzprüfung ist die eine Zusage, die `#![deny(unsafe_code)]` nicht trägt (Defekt 260803-1530); eine Prüfung, die sich mit einem `pub` davor aushebeln lässt, trägt sie nur zu drei Vierteln. Vorschlag: `ist_objc2_use` auf ein optionales Sichtbarkeitspräfix (`pub`, `pub(crate)` …) und ein optionales `::` vor `objc2` erweitern; das Abnahmekriterium im Plan müsste dieselbe erweiterte Vorschrift nennen (Planpflege, nicht nur Code). Adressat: coder.
