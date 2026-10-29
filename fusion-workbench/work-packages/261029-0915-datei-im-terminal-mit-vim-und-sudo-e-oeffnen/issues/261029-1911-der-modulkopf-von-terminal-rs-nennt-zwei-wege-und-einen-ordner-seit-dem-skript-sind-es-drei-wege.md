Der Modulkopf von `terminal.rs` sagt „Zwei Wege“ und „Ordner“, seit dem Skriptweg sind es drei Wege und nicht immer ein Ordner
---
`crates/krk-ui/src/appkit/terminal.rs` hat mit `skript_uebergeben` einen dritten Rufer des einen Rufs an `NSWorkspace` bekommen, der eine `.command`-Datei und keinen Ordner übergibt. Der neue Absatz darunter sagt das. Titel, Diagramm und Überschrift sagen weiter das Alte.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

Stand `7a69a051`:

- `terminal.rs:1`: `//! Wie ein Ordner an eine ueber ihre Buendelkennung benannte Anwendung kommt.`
- `terminal.rs:6`: das Diagramm führt allein `Ordner ──> openURLs:…`.
- `terminal.rs:9`: `//! # Zwei Wege stellen diese Frage`. Der Absatz darunter sagt „Beide gehen durch [`ordner_oeffnen`]“, und der Absatz ab Zeile 24 beginnt mit „Seit dem 261029 stellt ein dritter Weg die Frage“.

Ein Leser, der nach Überschriften springt, bekommt die falsche Zahl. Der Plan (Schritt 5) hatte nur verlangt, den dritten Rufer unter der bestehenden Überschrift zu nennen. Wörtlich ist das erfüllt, der Kopf widerspricht sich aber jetzt selbst.

Abnahme: Titelzeile, Diagramm und Überschrift sprechen von einem Pfad (Ordner oder Skript) und nennen keine Zahl, die der Absatz darunter widerlegt. Wird der Wortlaut gekürzt, bleibt der Satz, dass `ordner_oeffnen` in Name und Vertrag unverändert ist (die Probe in `anwendung.rs` sucht `terminal::ordner_oeffnen(`). `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` bleibt grün.
