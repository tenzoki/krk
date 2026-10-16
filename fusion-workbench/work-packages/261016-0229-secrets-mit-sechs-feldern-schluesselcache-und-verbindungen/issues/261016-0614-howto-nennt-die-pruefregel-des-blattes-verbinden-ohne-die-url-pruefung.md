`HowTo.md` nennt die Prüfregel des Blattes „Verbinden“ ohne die neue URL-Prüfung
---
Seit `9d1b111` bleibt „Anlegen“ auch abgeschaltet, wenn aus Protokoll und Adresse keine URL entsteht (Leerzeichen im Rechnernamen, Port aus Buchstaben). `HowTo.md` zählt die Bedingungen auf und nennt diese nicht, und die zugesagte Grundzeile bleibt für sie beim Tippen leer.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Akzeptanz (9) von Schritt 7)

## Defekt

- `HowTo.md:1747-1750`: „„Anlegen“ bleibt abgeschaltet, bis der Name nicht leer ist, das Protokoll eines aus der Tabelle unten ist, die Adresse nicht leer ist und kein `://` enthält und kein Feld einen Zeilenumbruch trägt; die Zeile unter den Feldern nennt den Grund.“
- Der Code fragt seit `9d1b111` zusätzlich `verbinden::url_aus` (`crates/krk-ui/src/appkit/blaetter/verbindung.rs:155-165`, `Entwurfsfehler::KeineUrl`), und der Knopf folgt `pruefen(...)` (`:270-273`).
- Beim Tippen zeigt die Grundzeile allein `Entwurfsfehler::Umbruch` (`:279-281`). Bei `KeineUrl` ist „Anlegen“ grau, und die Zeile bleibt leer, bis der Nutzer die Eingabetaste drückt (`bestaetigung_pruefen`, `:289-295`). Wer nach `HowTo.md` den Grund in der Zeile sucht, findet keinen.

Für einen leeren Namen oder eine leere Adresse ist das Schweigen beim Tippen gewollt (Kommentar an `:276-278`). `KeineUrl` gehört nicht zu dieser Art: ein Leerzeichen im Rechnernamen wird durch Weitertippen nicht gültig.

## Richtung der Behebung

`HowTo.md` nennt die vierte Bedingung, etwa „und Protokoll und Adresse zusammen eine gültige Adresse ergeben (kein Leerzeichen im Rechnernamen)“. Zusätzlich entweder zeigt die Grundzeile `KeineUrl` schon beim Tippen wie `Umbruch`, oder `HowTo.md` sagt, dass der Grund erst mit der Eingabetaste erscheint. Den Kommentar an `:276-278`, der auch ein unbekanntes Protokoll als „nie gültig“ nennt, ohne dass der Code es beim Tippen zeigt, gleich mit nachziehen.

## Akzeptanz

- Die Aufzählung in `HowTo.md` deckt jede Abweisung von `blaetter::verbindung::pruefen`.
- Was die Grundzeile beim Tippen zeigt, sagen Kommentar und `HowTo.md` gleich.
- `make check` grün.

---
Resolved: `HowTo.md`, Abschnitt „Eine Verbindung anlegen“, nennt die vierte Bedingung (Protokoll und Adresse ergeben eine gueltige Adresse: kein Leerzeichen im Rechnernamen, Port nur aus Ziffern), dass Leerzeichen, Umlaute, `#` und `?` im Pfad erlaubt sind, und wann die Grundzeile den Grund zeigt. Die Grundzeile zeigt `Entwurfsfehler::KeineUrl` jetzt schon beim Tippen wie `Umbruch` (`crates/krk-ui/src/appkit/blaetter/verbindung.rs`, Rueckruf von `textaenderung_melden`); der Kommentar dort nennt nicht mehr das unbekannte Protokoll, sondern sagt wie `HowTo.md`, dass leerer Name, leere Adresse und unbekanntes Protokoll erst mit der Eingabetaste erscheinen. `#` und `?` weist das Blatt nicht ab (siehe den Datensatz zur Verbindungs-URL), also steht keine solche Abweisung in `HowTo.md`.
