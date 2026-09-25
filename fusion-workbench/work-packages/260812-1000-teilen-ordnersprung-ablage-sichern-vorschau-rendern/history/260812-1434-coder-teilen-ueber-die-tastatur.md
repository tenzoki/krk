# Schritt 5: Teilen über die Tastatur

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 5
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0

---

## Was gebaut wurde

`shift+cmd+s` gibt die betroffenen Einträge an die Freigabedienste des Systems.
Worauf der Befehl wirkt, entscheidet der Fokus; die Verzweigung darüber ist eine
reine Rechnung ohne AppKit, die Übergabe selbst eine Hülle um
`NSSharingServicePicker`.

**`crates/krk-ui/src/appkit/teilen.rs` (neu)**

Die eine Berührung mit den Freigabediensten, im Zuschnitt von
`appkit/standardprogramm.rs`. Der Modulkopf begründet, warum es kein Zusatz zu
`zwischenablage.rs` ist: das Teilen legt nichts in die Zwischenablage, und es
gibt weiterhin genau eine Hülle um `NSPasteboard`. Drei öffentliche Stücke und
ein privates:

- `worauf(fokus) -> Quelle` ist die Fokusverzweigung, vollständig über alle fünf
  Werte und ohne Auffangzweig, mit drei Antworten: `BetroffeneEintraege` für
  Dateifenster und `Anderswo`, `AngezeigteDatei` für Vorschau und Editor,
  `Nichts` für die Leiste.
- `anbieten(pfade, flaeche, rechteck) -> bool` baut das `NSArray` aus je einem
  `NSURL`, erzeugt den Auswähler über `initWithItems:` und ruft
  `showRelativeToRect:ofView:preferredEdge:` mit `NSRectEdge::MinY`. Leere Liste:
  nichts geschieht, `false`. Der Rückgabewert trägt `#[must_use]`, weil sein
  stilles Fallenlassen die Meldung an die Statuszeile verschluckte.
- `eintrag_anfuegen(menue, pfade, mtm)` ist der eine Menübauer über
  `standardShareMenuItem`. Er steht bereit und hängt an keiner Fläche; das ist
  Schritt 6.
- `auswaehler_bauen` ist die eine Stelle, an der ein Pfad zu einem `NSURL` wird.
  Beide Hüllen gehen durch sie, damit es nicht zwei Wahrheiten darüber gibt, was
  KRK dem System übergibt.

Der offene Dialog wird festgehalten, in einem `thread_local!` mit
`RefCell<Option<Retained<NSSharingServicePicker>>>`. Der Doc-Kommentar sagt,
warum: `showRelativeToRect:` kehrt sofort zurück, der Dialog gehört danach dem
Auswähler, und ein `Retained`, das am Ende von `anbieten` fällt, nähme ihm seinen
Besitzer. `thread_local!` und kein `static`, weil `Retained` nicht `Sync` ist und
die Sache ohnehin dem Hauptfaden gehört.

**`crates/krk-ui/src/appkit/mod.rs`**

`mod teilen;`, der Modulkopf um einen Absatz, die Skizze um einen Pfeil von
`anwendung`, die Zählung von vierundzwanzig auf fünfundzwanzig Module, und in der
Aufstellung der `use crate::`-Ziele die neue Zeile: `teilen` verzweigt über
`crate::kommandos::fokus`.

**`crates/krk-core/src/tasten/belegung.rs`**

- `Kommando::Teilen`, eingeordnet neben `MitStandardprogrammOeffnen`.
  `Kommando::KENNUNGEN` wächst von 74 auf 75; die Feldbreite in der Typangabe hat
  den Bau angehalten, bis der Eintrag stand.
- `wirkungsbereich` → `Wirkungsbereich::Ueberall`, mit der Begründung am Zweig.
  Sie ist eine andere als bei den vier Befehlen darüber, und der Kommentar sagt
  es: bei jenen hängt die Quelle nicht am Fokus, bei diesem hängt sie daran, und
  gerade deshalb muss er überall durchkommen. Der Doc-Kommentar der Funktion
  führte vier Befehle außerhalb ihrer sechs Regeln; er nennt jetzt einen fünften.

**`crates/krk-ui/src/belegungsmodell.rs`**

`bereich_des_kommandos` → `Funktionsbereich::Dateioperationen`, neben
`mit_standardprogramm_oeffnen` und `eintragspfad_kopieren`, mit dem Satz, warum
die Gegend der Anwendung und nicht die Herkunft der Quelle entscheidet.

**`crates/krk-ui/src/kommandos/operationen.rs`**

`nichts_zu_teilen()` neben `nichts_zu_kopieren` und `nichts_zu_oeffnen`, dazu eine
Probe. Der Modulkopf sagt jetzt, worin das Teilen vom Zuschnitt der
Dateioperationen abweicht und worin nicht.

**`crates/krk-ui/src/appkit/anwendung.rs`**

Ein Zweig in `kommando_ausfuehren` auf die neue `teilen(fokus)`. Sie verzweigt
über die drei Werte von `teilen::worauf` und holt zu jedem seine Pfade und seinen
Anker; eine zweite Fokusabfrage entsteht nicht, der Wert kommt aus der einen
Abfrage weiter oben. Dazu zwei Änderungen an vorhandenem Code, beide unten
begründet.

## Drei Punkte, die in den Bericht gehören

**`nichts_zu_teilen` geht nicht durch `nichts_betroffen`, und das ist eine
Abweichung vom Plan mit einem Grund.** Der Plan stellt den Satz „neben
`nichts_zu_kopieren` und `nichts_zu_oeffnen`", was den gemeinsamen Rumpf
nahelegt. Der liefert „nichts zu *verb*: nichts markiert und nichts ausgewählt".
Für das Teilen wäre der zweite Halbsatz in zwei von drei Lagen falsch: mit dem
Fokus in der Leiste hat der Nutzer ein Lesezeichen ausgewählt vor sich und läse,
es sei nichts ausgewählt, und in der Vorschau ohne Datei gibt es keine Markierung,
von der die Rede sein könnte. Der Satz nennt deshalb das **Ergebnis** und keine
Ursache, wie es C2 derselben Runde für den Ordnersprung ausdrücklich verlangt:
**„nichts zu teilen: hier steht nichts, was an die Freigabedienste ginge"**. Er
stimmt in allen drei Lagen und deckt dazu die vierte, in der ein Bereich noch gar
nicht gebaut ist. Die Funktion heißt und steht, wo der Plan sie hinstellt; nur
ihr Rumpf ist ein eigener.

**Die Fokusverzweigung wohnt in `teilen.rs` und nicht beim Delegierten.** Der
Plan legt sie in `anwendung.rs::teilen()` und verlangt zugleich eine Probe „als
reine Tafel über die fünf Werte". Beides zusammen geht dort nicht: `krk-ui` hat
kein Bibliotheksziel, und eine Verzweigung in einer Methode des
Anwendungsdelegierten ist ohne Fenster nicht zu prüfen. Sie steht deshalb als
`teilen::worauf` in der Datei, die dem Befehl ohnehin gehört, und der Delegierte
verzweigt danach nur noch über die drei Werte, die sie liefert. Es bleibt bei
**einer** fünfwertigen Fallunterscheidung, sie ist ohne AppKit prüfbar, und sie
hält den Bau an, sobald ein sechster Fokuswert entsteht.

**`fokusansicht` liefert jetzt eine `NSView` statt eines `NSResponder`.** Der
Anker des Dialogs ist die Ansicht des fokussierten Bereichs und ihr `bounds`, und
`fokusansicht` ist nach dem Modulkopf von `anwendung.rs` die eine Zuordnung von
einem Fokuswert auf seine Ansicht. Eine zweite daneben, die dasselbe noch einmal
als `NSView` beantwortet, wären genau die zwei Wahrheiten, vor denen jene Stelle
warnt. Alle vier Zweige liefern ohnehin eine Ansicht, und `fokus_setzen` kommt
mit dem engeren Typ aus. Der Doc-Kommentar trägt die Begründung, der Modulkopf
nennt das Teilen als dritten Nutzer neben Lesen und Setzen.

Daneben eine kleine Zusammenlegung: das Ablesen der vier Eingaben für
`angezeigtedatei::welche` steht jetzt einmal in `Anwendungsdelegierter::angezeigte_datei`
statt zweimal. Der Ordnersprung aus Schritt 3 und das Teilen fragen dieselbe
Methode; C2, viertes Kriterium, verlangt genau das für die Rechnung, und die
Ablesung daneben zweimal zu führen hätte die Zusage von der anderen Seite
untergraben.

## Am SDK gegengelesen

Alle Untergrenzen dieser Datei sind am SDK gelesen und nicht aus dem Plan
übernommen. Sie stimmen mit ihm überein, Zeilennummern eingeschlossen
(`MacOSX.sdk`, AppKit-Kopfdateien):

| Berührung | Ort | Ab |
|---|---|---|
| `NSSharingServicePicker` | `NSSharingService.h:253` | 10.8 |
| `initWithItems:` | `NSSharingService.h:261` | keine eigene Angabe |
| `showRelativeToRect:ofView:preferredEdge:` | `NSSharingService.h:271` | keine eigene Angabe |
| `standardShareMenuItem` | `NSSharingService.h:281` | **13.0**, die höchste der Datei |
| `NSView.bounds` | `NSView.h:139` | keine Angabe |
| `NSMenu.insertItem:atIndex:` | `NSMenu.h:89` | keine Angabe |
| `NSMenu.numberOfItems` | `NSMenu.h:118` | keine Angabe |
| `NSMenuItem.separatorItem` | `NSMenuItem.h:27` | keine Angabe |
| `NSRectEdge` | `Foundation/NSGeometry.h:38` | keine Angabe |

Das Bündel zielt auf 15.0; keine Berührung braucht eine Verfügbarkeitsprüfung zur
Laufzeit. Dazu gegengelesen, weil der `unsafe`-Block es behauptet: `NSURL` erfüllt
`NSPasteboardWriting` (`NSPasteboard.h:469`), und `initWithItems:` verlangt genau
das (`NSSharingService.h:259`). Beide Stellen stehen im SAFETY-Kommentar.

Die Deckung des Abschnitts `# Ab welchem macOS die angesprochenen Klassen stehen`
steigt von 32 der 34 Dateien unter `crates/krk-ui/src/appkit/` auf 33 der 35;
ohne ihn sind weiterhin allein `koordinaten.rs` und `mod.rs`, beide begründet.
**Die Zahl in CLAUDE.md, „31 von 33", ist überholt** — am 260812 nachgezählt mit
`grep -rL` über den Baum, nicht fortgeschrieben.

## Proben

| Probe | Kriterium |
|---|---|
| `appkit::teilen::tests::jeder_der_fuenf_fokuswerte_traegt_seine_quelle` | C1.2 |
| `appkit::teilen::tests::die_tafel_nennt_jeden_fokuswert_genau_einmal` | C1.2 |
| `appkit::teilen::tests::allein_die_leiste_findet_nichts` | C1.2 |
| `kommandos::operationen::tests::der_satz_des_teilens_nennt_die_folge_und_keine_ursache` | C1.5 |

Die neue Kennung, ihr Wirkungsbereich und ihr Funktionsbereich laufen über die
bestehenden Proben mit: `crates/krk-core/tests/belegung.rs` (45 grün),
`belegungsmodell::tests::jede_kennung_hat_einen_funktionsbereich` und die Proben
der Belegungsansicht und der Markdown-Ausgabe.

**Keine Probe an den beiden Hüllen**, und das ist Absicht: ein Aufruf öffnete bei
jedem `make check` einen Systemdialog. Derselbe Grund, aus dem
`standardprogramm::oeffnen` und `zwischenablage::text_schreiben` keine tragen.

## Abnahme

Vier Kommandos, keine Messung, kein Bündelbau, kein Vordergrundlauf:

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 0 |

**Der Workspace ist wieder grün.** Vor diesem Schritt fielen 28 Proben in
`krk-ui`, jede mit einer Meldung, die `teilen` nannte; sie stammten aus Schritt 4,
der die Kennung in die Auslieferungsbelegung eingetragen hat, bevor es ein
Kommando dazu gab. Jetzt laufen 403 Proben im Binärziel `krk` durch, gegenüber
399 vorher: drei neue in `teilen.rs`, eine neue in `operationen.rs`, und die 28
gefallenen stehen wieder.

`eintrag_anfuegen` trägt bis Schritt 6 ein
`#[expect(dead_code, reason = …)]`, weil ihn noch niemand ruft. **`expect` und
nicht `allow`:** sobald Schritt 6 den Bauer an die drei Flächen hängt, meldet der
Übersetzer die Zeile als überflüssig, und der nächste Schritt nimmt sie weg. Ein
`allow` bliebe stumm stehen.

Vom Kriterienblock C1 sind damit ohne laufendes Bündel nachgewiesen: C1.2, C1.3,
C1.5 (erste Hälfte, der Satz), C1.7 (der Bauer ist einer und steht) und C1.8 (es
gibt weiterhin genau eine `NSPasteboard`-Hülle; `zwischenablage.rs` ist
unberührt). C1.4 ist zur Hälfte nachgewiesen: die übergebene Menge wird nicht
gefiltert und kein Typ geprüft. Offen und Nutzerarbeit bleiben C1.1 (der Dialog
geht auf, AirDrop steht darin), die zweite Hälfte von C1.4 und C1.5 sowie C1.6,
das Kontextmenü, das Schritt 6 anhängt.

## Datensätze

Zwei Fragen sind von beantwortet auf umgesetzt gezogen. Der Commit steht noch
aus; die Zeilen sagen es.

- `decisions/260812-1000_i_teilt-krk-auch-ordner-oder-nur-dateien.md`
  — Möglichkeit 1: `anbieten` filtert nicht und prüft keinen Typ.
- `decisions/260812-1000_i_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`
  — beide Befehle sind jetzt in Code, `teilen` auf `shift+cmd+s` und
  `ordner_der_datei` auf `opt+cmd+o`.

`decisions/260812-1145_a_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`
bleibt beantwortet und bindet Schritt 6, nicht diesen.

Keine neuen Defekte.
