Sechs der zwölf Module unter `kommandos/` tragen kein einziges `#[must_use]`, darunter die vier Zulässigkeitsantworten

---

`zulaessig`, `immer_erreichbar`, `waehrend_blatt_erlaubt` und `fokus::wirkt` sind die vier Antworten, aus denen KRK entscheidet, ob eine Taste durchkommt. Keine trägt `#[must_use]`. Ein nackter Aufruf `zulaessig(kommando, lage);` übersetzt unter `-D warnings` und lässt den Befehl ungeprüft weiter — genau der Fall, für den der Nutzer am 260811-2140 das Attribut statt einer Kommentarkonvention verlangt hat. Dasselbe gilt für 20 weitere reine Antworten in denselben sechs Dateien.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Befund

Gezählt am 260826 gegen `ca8072d` mit einem `awk` über jede `pub fn`-Zeile, deren vorige Nicht-Kommentarzeile kein `#[must_use` trägt.

**Mit `#[must_use]`, sechs Module (Runden 10 bis 17):** `rueckschritt.rs`, `rundweg.rs`, `loeschwarnung.rs`, `abwurfregel.rs`, `kontextmenue.rs`, dazu vier Textfunktionen und `rechtsklick_zielzeile`/`erzeugt_genau_ein_ziel` in `operationen.rs`. Jede trägt daneben eine Begründung.

**Ohne, sechs Module (Runde 1 bis 7), 24 Stellen:**

| Datei | Zeile | Funktion | Was beim stillen Fallenlassen verloren geht |
|---|---|---|---|
| `zulaessigkeit.rs` | 177 | `zulaessig` | die ganze Zulässigkeitsfrage |
| `zulaessigkeit.rs` | 202 | `immer_erreichbar` | die Ausnahmeliste |
| `operationen.rs` | 283 | `waehrend_blatt_erlaubt` | die Blattsperre |
| `fokus.rs` | 343 | `wirkt` | der Fokusvorbehalt |
| `fokus.rs` | 214, 243, 271, 326 | `holt_hervor`, `in_bereich`, `bereich_mit_fokus`, `rahmenrolle` | Hervorholen, Fokusabfrage, Rahmenfarbe |
| `operationen.rs` | 315 | `Buendelung::melden` | der Weckruf; die Funktion **schaltet** dabei `offen` um, ein Aufruf ohne Auswertung verliert den Zeichendurchgang |
| `operationen.rs` | 133, 170, 599, 737, 868 | `anzeige_faellig`, `betroffene`, `uebersprungenliste`, `umbenennung_pruefen`, `ordner_fehlt` | Anzeigeverzug, Auswahl, Abschlussliste, Namensprüfung, Ordnerprüfung |
| `navigation.rs` | 46, 89 | `zielzeile`, `ersatzzeile` | die Zielzeile |
| `auswahl.rs` | 24, 44 | `markieren_und_weiter`, `markierungsstand_text` | die Folgezeile, der Statustext |
| `pfadeingabe.rs` | 52 | `pruefen` | das Navigationsergebnis |
| `operationen.rs` | 508, 533, 545, 564, 669, 681, 754, 893, 925, 942, 958, 970, 979, 1017, 1111, 1150 | die Textbausteine | der Text der Statuszeile |

Heute wertet jeder Rufer im Baum die Antwort aus (`anwendung.rs:901`, `:3140`; `tabelle.rs:2116`, `:2142`, `:2512`, `:2613`; `anwendung.rs:8057`, `:8107`). Der Defekt ist die fehlende Sperre gegen den nächsten Rufer, nicht ein heutiger Verlust.

## Querschnitt

Die Trennlinie ist das Datum: alle Module, die nach dem Nutzerentscheid vom 260811-2140 entstanden sind, tragen das Attribut mit Begründung, alle davor nicht. `260826-1221_o_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-…` und `260826-1223_o_tasten-und-text-tragen-kein-einziges-must-use-…` zeigen dieselbe Linie im Kern; dieser Datensatz ist ihr Gegenstück in `kommandos/`.

## Vorschlag

`#[must_use]` an alle 24, mit der Begründung im Wortlaut der Nachbarn (`rueckschritt.rs:152-156`, `loeschwarnung.rs:359`). Bei den vier Zulässigkeitsantworten mit Meldungstext: „fallengelassen läuft der Befehl ungeprüft weiter".

Schwere: mittel.
