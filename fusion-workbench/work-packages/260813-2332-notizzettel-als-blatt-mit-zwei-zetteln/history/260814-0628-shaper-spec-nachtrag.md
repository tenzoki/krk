# Shaper: der Spec der neunten Runde ist nachgezogen

**Datum:** 2026-08-14 06:28 bis 06:45
**Status:** Complete
**Modus:** user-direct, am laufenden Circle `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln`
**Auftrag:** den Spec `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` nach der Diagrammprüfung vom 260814-0000 und den drei Nutzerantworten vom 260814-0005 nachziehen. Keine Klärungsfragen, kein Bau.

---

## Was hereinkam

Die Diagrammprüfung `reviews/260814-0000-conceptrev-spec-notizzettel-als-blatt-mit-zwei-zetteln.md`, Spruch `acceptable`, mit fünf Befunden N1 bis N5. Dazu drei bindende Nutzerantworten: `shift+cmd+w` sichert vor dem Schließen des Fensters, die unlesbare Zetteldatei geht über `Zugang::beiseite_legen` beiseite, und die Freigabe des Spec folgt erst nach dem Nachtrag.

## Was am Baum erhoben wurde

Sieben Stellen sind gelesen und nicht aus der Prosa übernommen worden.

| Stelle | Was sie sagt |
|---|---|
| `crates/krk-ui/src/kommandos/zulaessigkeit.rs:172-201` | `immer_erreichbar` steht als erster Operand und kurzschließt Blattstand und Ersthelferbefund; die Liste führt drei Befehle |
| `crates/krk-ui/src/appkit/anwendung.rs:3484-3490` | `cmd+n` ruft `makeKeyAndOrderFront` und `activate`, das Blatt bleibt stehen |
| `crates/krk-ui/src/appkit/anwendung.rs:3508-3514` | `shift+cmd+w` ruft `performClose:` am Hauptfenster |
| `crates/krk-ui/src/appkit/anwendung.rs:5372-5384` | `beenden_erlauben` liefert `TerminateCancel`, wenn der Editor einen ungesicherten Stand hält und ein Blatt steht |
| `crates/krk-core/src/ablage/mod.rs:447` und `:497` | `beiseite_legen` ist privat und hat genau einen Aufrufer, `Zugang::laden` |
| `crates/krk-core/tests/baum.rs:178-206` | die Probe zum atomaren Schreiben zählt fünf Quelldateien auf |
| `crates/krk-core/src/text/datei.rs:153` | `EDITORGRENZE` steht an genau einer Stelle, 16 MB |
| `resources/default-keymap.toml:516-518` | `shift+cmd+w` liegt auf `fenster_schliessen` |

Der Entscheid, auf den der Auftrag mit dem Datum 260813-1125 verweist, liegt als
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-1110_i_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`.
Die Antwort darin ist am 260813-1055 gefallen, die Datei trägt 1110. Inhaltlich ist es der gemeinte
Entscheid: `fenster_schliessen` steht seither ausdrücklich auf `immer_erreichbar`.

## Was geändert wurde

Ein Dokument, der Spec. Der Abschnitt „Was der Nachtrag vom 260814 geändert hat"
am Ende des Spec zählt sie im Einzelnen auf. Der Kern:

- Vierter Sicherungsmoment in Directive, C1, C4 und im Bild der Sicherungsmomente.
- Die Zusage zur unlesbaren Zetteldatei in C5, mit sieben neuen Abnahmekriterien.
- Beide Bilder neu gezeichnet, mit vollständigen Fallunterscheidungen an jedem Verzweigungspunkt.
- Drei neue Feststellungen in der Ausgangslage, die Zählung dort von fünfzehn auf achtzehn.
- Die offene Nutzerfrage ist geschlossen und durch einen Verweis auf den beantworteten Datensatz ersetzt.

Beide Mermaid-Blöcke sind mit `@mermaid-js/mermaid-cli` 11.16.0 nach SVG gerendert. Dabei kam ein
Verhalten heraus, das den Entwurf geändert hat: **Mermaid zieht mehrere Übergänge eines Zustands
auf sich selbst zu einem zusammen und lässt die übrigen stillschweigend fallen.** Die erste Fassung
von Bild 2 trug drei solcher Übergänge, und zwei fehlten im gerenderten SVG. Sie stehen jetzt als
ein Übergang mit dreizeiliger Beschriftung; alle Beschriftungen sind im SVG nachgezählt.

## Was gefilert wurde

Zwei Defektdatensätze, beide im aktiven Circle.

- `issues/260814-0628_o_diagrammbefunde-haben-keinen-eigentuemer-und-bleiben-deshalb-liegen.md` — das Muster hinter dem dritten Auftreten desselben Befunds. Die Begründung für einen eigenen Datensatz steht darin, ebenso der Hinweis, dass `shared/issues/` der sachlich richtige Ort wäre.
- `issues/260814-0637_o_die-directive-im-circle-datensatz-nennt-drei-sicherungsmomente-der-spec-vier.md` — der Circle-Datensatz nennt weiter drei Sicherungsmomente. Der Shaper darf den Abschnitt `## Directive` außerhalb des Aktivierungsmodus nicht schreiben.

## Was nicht angefasst wurde

Die sieben beantworteten Klärungsfragen, `immer_erreichbar`, `waehrend_blatt_erlaubt`, `zulaessigkeit::zulaessig`,
der Circle-Datensatz, der Entscheidungsdatensatz zum unlesbaren Zettel und der Quellbaum. Kein Bau gefahren.
