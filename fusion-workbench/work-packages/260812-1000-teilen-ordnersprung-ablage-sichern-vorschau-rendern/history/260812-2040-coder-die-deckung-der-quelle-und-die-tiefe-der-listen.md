# Zwei Defekte der Markdown-Zerlegung: die Deckung der Quelle und die Tiefe der Listen

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Issues:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1805_c_quelltext-ohne-ereignis-verschwindet-spurlos-aus-der-gerenderten-vorschau.md` (behoben); `…/260812-1805_c_listen-verlieren-merkzeichen-nummerierung-und-verschachtelungstiefe.md` (behoben); `…/260812-1805_o_der-ueberschneidungssatz-in-textmerkmale-anwenden-gilt-seit-markdown-rs-nicht-mehr.md` (teilweise, bleibt offen)
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 445, nachher 454

---

## Die Messwerte des `coderev` stimmen

Alle acht Ausgaben der drei Datensätze sind vor der Behebung am Baum
nachgemessen worden, mit `markdown::rendern` in einer temporären Probe. Jede
stimmt zeichengenau, die vier überlappenden Listenbereiche `(0,4)`, `(5,24)`,
`(10,19)`, `(18,11)` eingeschlossen.

## Defekt 1: kein Quellbyte fällt heraus

Die Auffangregel hing an einem **Ereignis**; Quelltext ohne Ereignis fiel durch
sie hindurch. An ihre Stelle tritt ein Stand in der Quelle, `Zerlegung::gelesen`,
und zwei Sätze, von denen keiner eine Ereignisart kennt:

```
   Quellbyte
       │
       ├─ kein Element offen  ──> luecke_bis: woertlich, Leerraum faellt weg
       │                          (auch nach dem Durchgang, bis str::len)
       │
       └─ Element offen  ──┬─ Element hat Zeichen geliefert
                           │     ──> seine Auszeichnungszeichen, gehoeren weg
                           │
                           └─ Element hat kein Zeichen geliefert
                                 ──> schliessen: sein Quellbereich, woertlich
```

Der erste Satz fragt allein, ob `offen` leer ist, der zweite allein, ob die
Länge null ist. Die Fallunterscheidung ist damit trennscharf und vollständig
über die **Zeichen** der Datei und nicht mehr nur über `Event` und `Tag`.

Die Kehrseite ist mitgeprüft: ohne die Grenze „nur auf Dokumentebene" trüge die
Regel das `[` und das `][ref]` eines Verweises in Kurzform wieder in den Text.

## Defekt 2: Merkzeichen, Nummer und Tiefe

Zuschnitt 1 und 2 des Datensatzes zusammen. `punkt_oeffnen` schreibt `• ` oder
die Nummer der geordneten Liste in den Text, innerhalb des Bereichs der
Listenzeile, damit der Einzug das Zeichen mitnimmt. `Auszeichnung::Listenzeile`
trägt `tiefe: u8`, und `einzugsmerkmal` rechnet `tiefe * LISTENEINZUG`, gedeckelt
bei acht Ebenen — acht mal 20 Punkte sind die Mindestbreite eines Bereichs der
Fensterzeile.

Die Tiefe wird aus den offenen Elementen **gezählt** und nicht in einem zweiten
Zähler mitgeführt: jedes einrückende Element trägt eine `Ebene`, ihre Zahl ist
die Tiefe. Ein Zitatblock zählt mit. Die Nummer einer geordneten Liste steht in
der `Ebene` der Liste und wird vom Punkt verbraucht.

Das geht über den beschlossenen Umfang hinaus: der Datensatz
`decisions/260812-1000_a_welchen-umfang-von-markdown-rendert-die-vorschau.md`
nennt verschachtelte Listen unter den drei teuren Bestandteilen, die Möglichkeit
1 nicht enthält. Die Begründung dort war, sie brauchten „eine Einrücktiefe, die
die vorhandene Auszeichnungsmechanik nicht kennt"; die Mechanik kennt sie jetzt.
Den Umfangsdatensatz nachzuziehen ist Sache des Nutzers.

## Defekt 3: zwei von drei Punkten

Der SAFETY-Kommentar in `textmerkmale.rs` ist berichtigt, die Reihenfolge bei
gleicher Länge ist festgelegt: `Offen` trägt einen `rang`, und `abschliessen`
sortiert nach Anfang, absteigender Länge und dann dem Rang. Die Ordnung ist total
und hängt nicht mehr an der Stabilität der Sortierung.

Nicht gebaut ist das Zusammenlegen der Schrift — fett **und** kursiv über
`NSFontDescriptor`-Merkmale statt eines Ersetzens. Das ist eine
Verhaltensänderung an AppKit in einer Datei ohne eine einzige Probe, und ohne
Vordergrundlauf nicht zu sehen. Der Datensatz bleibt offen und nennt den Rest.

## Berührte Dateien

- `crates/krk-ui/src/markdown.rs` — die Deckung, die Ebenen, der Rang, neun
  neue Proben, vier angepasste
- `crates/krk-ui/src/hervorhebung.rs` — `Listenzeile { tiefe }`, die
  Formatansicht gibt immer 1
- `crates/krk-ui/src/appkit/textmerkmale.rs` — `einzugsmerkmal(tiefe)`,
  `EINZUGSGRENZE`, der berichtigte SAFETY-Kommentar

Nicht angefasst: `Vorschaumodell::laedt_noch`, alles an L7, `statuszeile.rs`.
Die Zerlegung läuft weiter auf dem Arbeitsfaden `krk-vorschau`.
