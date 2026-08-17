# Durchsicht: Bündel A, die unbedingte Rückfrage (`664a0fd..472eb81`)

**Sender:** coderev
**Datum:** 260817-1105
**Reviewed-range:** `664a0fd..472eb81`
**Not-opened:** none — jede Codedatei des Bereichs ist geöffnet. `fusion-workbench/orchestrator-events.jsonl` liegt im Bereich und ist bewusst ausgeschlossen statt ungeöffnet: maschinengeschriebenes Ereignisprotokoll, kein Code. Der Ausschluss steht hier, damit die Datei nicht ein drittes Mal mitwandert.
**Grundlage:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` (C1, C2, `## Die vier Zusagen…`),
`planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md` (Bündel A, `## Current State`),
`_t_circle.md`, `history/260817-1028…`, `history/260817-1050…`, `history/260817-1104…`

## Zusammenfassung

**Die Schutzschwelle hält.** Nach diesem Bündel gibt es keinen Befehl in KRK mehr, der eine
Datei ohne stehende Rückfrage vom Datenträger nimmt; die drei Wege in `in_den_papierkorb`
und der eine in `endgueltig_loeschen` gehen alle durch denselben Rumpf, und `auftrag_stellen`
trägt keine Löschart mehr. Der behauptete eine Aufrufer ist nachgezählt und stimmt. Sieben
Befunde stehen daneben, einer davon hoch: das Blatt fällt bei einer unbekannten Antwort auf
die **zerstörende** Schaltfläche zurück, und das ist genau die Richtung, in die eine
Sicherung nicht fallen darf.

## Zählung

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 1 |
| Niedrig | 5 |

## Was geprüft ist und hält

**Kein Räumen ohne Rückfrage.** Vollständig am Baum nachgegangen, nicht abgeleitet:

```
Kommando::InPapierkorb (anwendung.rs:2894)
  └─ papierkorb_oder_zeichen_zurueck   drei Ausgänge, drei Rufe:
       ├─ :4513 kein Anschlag  ─┐
       ├─ :4516 Zusatztaste    ─┼──> in_den_papierkorb  ── einziger Ruf-Ort
       └─ :4541 Regel sagt ja  ─┘         └─> loeschen_nach_rueckfrage ──> Blatt
Kommando::EndgueltigLoeschen (:2895)
  └─ endgueltig_loeschen (:4558)         └─> loeschen_nach_rueckfrage ──> Blatt

auftrag_stellen(Art)  ── genau ein Ruf-Ort (:4431), und der schickt allein
                         Art::Kopieren und Art::Verschieben
```

Die weiteren Wege, an die der Auftrag gedacht war, tragen nichts bei:

- **Menüeintrag.** `krkKommando:` (`anwendung.rs:745`) geht über `kommando_ausfuehren`, also
  über denselben Zweig. Ein zweiter Ausführungsweg besteht nicht.
- **Melder der Bereichsleiste.** Er kann `Kommando::InPapierkorb` gar nicht senden; siehe
  Befund 4.
- **Kontextmenü der Dateiliste.** `menuNeedsUpdate:` (`tabelle.rs:651-655`) leert das Menü
  und hängt allein den Teilen-Eintrag an. Kein Löscheintrag.
- **Rückschritt-Regel.** Unberührt. Alle drei Ausgänge münden entweder in denselben
  `in_den_papierkorb` oder tun nichts.
- **Gehaltene Taste.** Der erste Anschlag stellt das Blatt, jeder weitere wird von
  `zulaessigkeit::zulaessig` über `blatt_steht` abgewiesen. `Kommando::InPapierkorb` steht
  weder in `waehrend_blatt_erlaubt` (Probe `operationen.rs:1284`) noch in
  `immer_erreichbar` (`zulaessigkeit.rs:197-202`).
- **Zwei Bäume ohne Papierkorb im Kern.** `loeschen::baum_entfernen` behält seine zwei
  Aufrufer, die Konfliktantwort „Überschreiben" (`operation/mod.rs:245`) und das Verschieben
  über die Datenträgergrenze (`verschieben.rs:123`). Beide sind kein Löschbefehl, und der
  Spec nennt sie ausdrücklich.
- **Messmodus.** `kopierziel_leeren` (`messmodus.rs:346-354`) räumt allein unterhalb von
  `self.kopierziel`, also im Messplatz.

**Der eine Aufrufer des Textbauers stimmt.** `frage_und_erlaeuterung` hat im ganzen Baum
genau einen Ruf-Ort, `anwendung.rs:4457`. Die Zusage im Modulkopf ist nachgezählt und nicht
geglaubt.

**Die drei bewussten Abweichungen tragen.**

1. **`zahl` und `ordner_text` bleiben `pub(crate)`.** Richtig und belegt: `zahl` wird in
   `appkit/statuszeile.rs:177` importiert, also außerhalb von `kommandos`; `pub(super)`
   übersetzte dort nicht. Beide Doc-Kommentare nennen den neuen Aufrufer, und der von `zahl`
   schreibt die Begründung für die weite Form aus (`operationen.rs:733-737`).
2. **„Zum Bestätigen Cmd+Return."** Sachlich richtig und gut begründet. Das Blatt bedient seit
   Schritt 3 beide Befehle, und `loeschwarnung` vermeidet das Wort „löschen" ausdrücklich;
   ein Hinweissatz zwei Zeilen darunter, der es doch benutzt, nähme der Unterscheidung ihre
   Wirkung. Der Satz benennt jetzt allein die Taste und bleibt nach Bündel D richtig.
3. **Die verbliebene zweite Lesung vor dem Blatt ist harmlos.** Nachgegangen und bestätigt:
   `in_den_papierkorb` liest `betroffene_eintraege()` für die Texte, `loeschen_nach_rueckfrage`
   liest es danach für seine Prüfungen und den Auftrag. Zwischen beiden Aufrufen liegt kein
   Durchgang der Ereignisschleife: keine der beteiligten Funktionen pumpt Ereignisse, und
   FSEvents wie AppKit-Rückrufe erreichen den Hauptfaden nur über die Schlange. Die beiden
   Lesungen können deshalb nicht auseinanderlaufen. Der behobene Defekt hing an der Lesung
   **nach** dem Blatt, und die ist weg. Schritt 11 nimmt die verbliebene mit; solange sie
   steht, hängt die Zusage an der Reihenfolge und nicht an einer Sperre.

**Die Fensterseite darf im Rückruf gelesen werden.** Die Begründung an `loeschauftrag_stellen`
ist im Ergebnis richtig, in ihrer Formulierung aber zu weit; siehe Befund 6. Keiner der drei
Befehle, die `immer_erreichbar` durchlässt, ändert `modell.aktiv()`: `beenden` ruft
`terminate:`, `fenster_schliessen` ruft `performClose:` (das an einem Fenster mit
anhängendem Blatt nicht schließt), `fenster_zeigen` ruft `makeKeyAndOrderFront:` und
`activate`.

**Kein `RefCell`-Konflikt auf dem Abbruchweg.** `abbrechen` (`anwendung.rs:4734`) nimmt den
`Blattgriff` aus der Zelle heraus, bevor es ihn benutzt; der Rückruf, der `offenes_blatt`
gleich darauf beschreibt, findet keine offene Ausleihe vor.

**Die vier Abnahmekommandos laufen hier grün.** Selbst gefahren am 260817-1100:
`cargo build --workspace`, `cargo test --workspace` (alle Ziele grün),
`cargo clippy --workspace --all-targets -- -D warnings` ohne Ausgabe,
`cargo fmt --all --check` ohne Ausgabe.

## Befunde

### 1 — Hoch: Eine unbekannte Antwort des Löschblattes fällt auf die zerstörende Schaltfläche

`crates/krk-ui/src/appkit/blaetter/mod.rs:567-580` rechnet die gedrückte Schaltfläche aus dem
Rückgabewert von `NSAlert` zurück und fängt eine unbekannte Antwort so ab:

```rust
// Eine unbekannte Antwort gilt als die letzte Schaltflaeche, und
// die ist in jedem Blatt dieser Runde die abbrechende. Lieber
// nichts tun als raten.
let stelle = antworten
    .iter()
    .position(|kandidat| *kandidat == antwort)
    .unwrap_or(antworten.len().saturating_sub(1));
```

Für das Löschblatt ist die letzte Schaltfläche **nicht** die abbrechende.
`loeschbestaetigung.rs:98-105` legt sie in dieser Reihenfolge an:

```rust
Schaltflaeche::neu("Abbrechen", Taste::Eingabe),
Schaltflaeche::neu(schaltflaeche, Taste::EingabeMitBefehl),
```

und der Rückruf ist `fertig(stelle == 1)`. Eine unbekannte Antwort ergibt also `stelle == 1`,
das heißt `bestaetigt == true`, das heißt `loeschauftrag_stellen`. Die Sicherung fällt in die
zerstörende Richtung.

Das Löschblatt ist die einzige Stelle im Baum, an der die Annahme des Kommentars nicht gilt.
Nachgezählt: `konflikt.rs:86-89` und `ungesichert.rs:90-92` setzen „Abbrechen" jeweils ans
Ende, `uebersprungen.rs:41` und `zettel.rs:411` tragen nur eine Schaltfläche.

Dieselbe Datei trägt daneben die **entgegengesetzte** Vorbelegung für dieselbe Frage:
`Blattgriff.abbruchcode` fällt auf `NSAlertFirstButtonReturn` zurück, also auf die **erste**
Schaltfläche (`mod.rs:599-601`). Zwei Vorbelegungen für „welche Schaltfläche ist die
ungefährliche", die einander widersprechen; für das Löschblatt trifft die eine und die
andere nicht.

**Was ich nicht belegen kann:** einen erreichbaren Auslöser. Die Kandidaten, die ich
verfolgt habe, tragen nicht. `Blattgriff::abbrechen` und `abbruchweg` schicken beide
`NSAlertFirstButtonReturn`. `performClose:` schließt ein Fenster mit anhängendem Blatt
nicht. `terminate:` beendet den Prozess, ohne den Abschlussblock zu fahren. Es bleibt eine
Vorbelegung, die in die falsche Richtung zeigt, ohne dass ich einen Weg dorthin gefunden
hätte. Der Grund, sie trotzdem hoch zu führen: das ist genau die Sorte Zusage, die diese
Runde baut, und der Spec entscheidet dieselbe Frage an anderer Stelle ausdrücklich anders
(„Unentschieden gilt als laut").

**Richtung:** Die Vorbelegung soll die abbrechende Stelle nehmen und nicht die letzte. Da
das Löschblatt keine Schaltfläche mit `Taste::Escape` trägt, ist `abbruchstelle` dort `None`;
die Zuordnung „welche Stelle ist die abbrechende" muss also unabhängig von der Escape-Taste
werden, oder das Blatt muss sie ausdrücklich mitgeben. Beides ist eine Entscheidung über
`blaetter/mod.rs` und keine über das Löschblatt allein.

Datensatz: `issues/260817-1106_o_eine-unbekannte-blattantwort-faellt-im-loeschblatt-auf-die-zerstoerende-schaltflaeche.md`

### 2 — Mittel: Der Rumpf, der die Schutzschwelle trägt, ist von keiner Probe gedeckt

`loeschen_nach_rueckfrage` und `loeschauftrag_stellen` (`anwendung.rs:4603-4697`) sind die
Mechanik, um derentwillen diese Runde läuft. Vier Eigenschaften tragen sie, und keine ist
geprüft:

1. der laufende Vorgang wird **vor** dem Blatt gemeldet,
2. die leere Auswahl kommt gar nicht bis zum Blatt,
3. ein Abbruch stellt keinen Auftrag,
4. der bestätigte Auftrag trägt die Auswahl, die im Blatt stand.

`crates/krk-ui/src/kommandos/loeschwarnung.rs` trägt fünf Proben, aber allein über die beiden
Texte. `krk-ui` hat kein Bibliotheksziel, und ein Blatt lässt sich unter `libtest` nicht
bedienen, also ist der heutige Zustand ohne Umbau nicht prüfbar. Abgenommen ist er auch nicht:
der Abnahmelauf ist Nutzerarbeit und für diese Runde nicht gefahren.

**Richtung:** Die vierstufige Reihenfolge ist eine reine Regel über vier Wahrheitswerte
(Vorgang läuft, Auswahl leer, Papierkorb vorhanden, bestätigt) und könnte als solche neben
`rueckschritt` und `loeschwarnung` stehen, wo dieses Projekt seine Regeln ohnehin hinlegt.
Bündel B setzt mit der Papierkorbprüfung eine fünfte Stufe in dieselbe Kette; wenn die Regel
umzieht, dann dort und nicht danach.

Datensatz: `issues/260817-1107_o_der-rumpf-der-schutzschwelle-traegt-keine-probe.md`

### 3 — Niedrig: Die Frage entsteht, bevor eine der beiden Sperren gefragt ist

`in_den_papierkorb` (`anwendung.rs:4454-4467`) liest die Auswahl und baut beide Texte, bevor
`loeschen_nach_rueckfrage` nach dem laufenden Vorgang und nach der leeren Auswahl fragt. In
zwei der vier Ausgänge werden die Texte verworfen, und im leeren Fall entsteht dabei der Satz
„Diese 0 Einträge in den Papierkorb räumen?", weil `frage_und_erlaeuterung` die Einzahl nur
für `1` kennt. Auf den Schirm kommt er nicht.

Kosten: zwei Durchgänge über die Auswahl je Tastendruck statt einem. Bei einer großen
Markierung sind das zwei Vektoren aus `PathBuf` auf dem Hauptfaden.

Schritt 11 zieht das Bauen der Texte in den Rumpf und nimmt beides mit. Der Befund steht
hier, damit er nicht mit Schritt 11 verlorengeht, falls dessen Zuschnitt sich ändert.

Datensatz: `issues/260817-1108_o_die-loeschfrage-entsteht-vor-beiden-sperren-und-im-leeren-fall-mit-null-eintraegen.md`

### 4 — Niedrig: Der Modulkopf nennt einen Weg in den Papierkorb, den es nicht gibt

`crates/krk-ui/src/kommandos/loeschwarnung.rs:46-50` sagt:

> die beiden Tasten, der Menueeintrag und der Melder der Bereichsleiste laufen durch ihn
> hindurch

Der Melder der Bereichsleiste kann `Kommando::InPapierkorb` nicht senden. `bereichsleiste.rs`
kennt genau elf Kommandos, und alle elf sind Umschalter: fünf Bereiche (`:164-168`), drei
Spalten (`:182-184`), die Tiefe (`:195`) und der Inhalt (`:214`).

Dieselbe Aufzählung steht seit vor diesem Bündel im Doc-Kommentar von
`papierkorb_oder_zeichen_zurueck` (`anwendung.rs:4479-4483`). Der Modulkopf hat sie von dort
übernommen. Sie stört, weil der Modulkopf die Stelle ist, an der der nächste Leser die
Aufrufer zählt.

Datensatz: `issues/260817-1109_o_zwei-stellen-nennen-den-melder-der-bereichsleiste-als-weg-in-den-papierkorb.md`

### 5 — Niedrig: Zwei Doc-Kommentare nennen `endgueltig_loeschen` als Träger eines Satzes, den es nicht mehr trägt

`anwendung.rs:5526` und `anwendung.rs:6276` verweisen beide darauf, dass
`endgueltig_loeschen` den Satz „es ist nichts ausgewählt" für die leere Auswahl führe. Seit
Schritt 3 ist die Prüfung dort nicht mehr; sie steht in `loeschen_nach_rueckfrage` (`:4620`)
und in `auftrag_stellen` (`:5093`). Beide Verweise sind durch dieses Bündel falsch geworden
und stehen nicht auf der Liste der 46 Nennungen, die Bündel E nachzieht.

Datensatz: `issues/260817-1110_o_zwei-doc-kommentare-nennen-endgueltig-loeschen-als-traeger-des-satzes-es-ist-nichts-ausgewaehlt.md`

### 6 — Niedrig: Die Begründung an `loeschauftrag_stellen` sagt „jedes Kommando außer dem Abbruch"

`anwendung.rs:4676-4680` begründet die erneute Lesung der Fensterseite damit, dass
`kommando_ausfuehren` bei stehendem Blatt „jedes Kommando ausser dem Abbruch" abweise. Es sind
vier Ausnahmen und nicht eine: `zulaessigkeit::immer_erreichbar` (`:197-202`) lässt daneben
`Beenden`, `FensterSchliessen` und `FensterEinblenden` durch.

Der Schluss hält trotzdem, und ich habe ihn einzeln nachgerechnet: keiner der drei ändert
`modell.aktiv()`. Falsch ist die Begründung, nicht das Ergebnis. Sie steht in derselben
Formulierung auch in `blaetter/mod.rs:272` und in `CLAUDE.md`; an dieser Stelle trägt sie
zum ersten Mal eine Zusage über eine zerstörende Handlung, und deshalb steht sie hier.

Datensatz: `issues/260817-1111_o_die-begruendung-an-loeschauftrag-stellen-nennt-eine-ausnahme-es-sind-vier.md`

### 7 — Niedrig: `frage_und_erlaeuterung` trägt kein `#[must_use]`

Das Projekt setzt `#[must_use]` an jeden Rückgabewert, dessen stilles Fallenlassen unbemerkt
bliebe; `rueckschritt` trägt es samt ausgeschriebener Begründung (`rueckschritt.rs:142-145`).
`frage_und_erlaeuterung` ist eine reine Funktion, ihr Ergebnis fallenzulassen ist ein Aufruf
ohne jede Wirkung, und der Übersetzer sagt dazu nichts.

Gegengewicht, das dazugehört: `operationen::loeschfrage`, die Funktion, die sie ablöst, trägt
es auch nicht. Der Befund steht trotzdem, weil dieses Modul mit Bündel C zwei weitere
Funktionen bekommt, für die der Plan `#[must_use]` ausdrücklich verlangt; drei Funktionen mit
zwei Haltungen wären die Abweichung, die niemand prüft.

Datensatz: `issues/260817-1112_o_frage-und-erlaeuterung-traegt-kein-must-use.md`

## Übergreifend

**Eine Sicherung, zwei einander widersprechende Vorbelegungen.** Befund 1 und Befund 6 sind
derselbe Fehler in zwei Größen: eine Aussage über „was im Zweifel gilt", die an einer Stelle
formuliert und an einer anderen benutzt wird, ohne dass etwas die beiden aneinanderhält. Bei
Befund 6 hält das Ergebnis, bei Befund 1 zeigt es in die zerstörende Richtung. Beide würden
von derselben Gewohnheit gefangen: die Bedingung dort hinschreiben, wo sie entschieden wird,
statt dort, wo man sich auf sie beruft.

**Vier Aussagen, die der Baum nicht trägt** (Befunde 4, 5 und 6), alle in Doc-Kommentaren,
alle in derselben Sitzung entstanden oder durch sie falsch geworden. Sie hängen nicht am
Können, sondern daran, dass eine verschobene Funktion ihre Nennungen anderswo nicht mitnimmt.
Bündel E zieht 46 solche Stellen nach; die drei hier gehören dazu, stehen aber nicht auf
seiner Liste, weil sie erst nach ihrer Aufstellung entstanden sind.

## Empfohlene Reihenfolge

**Vor der Auslieferung:** Befund 1. Er ist eine Zeile in `blaetter/mod.rs` plus die
Entscheidung, woher die abbrechende Stelle kommt, und er sitzt in der Sicherung selbst.

**Mit Bündel B:** Befund 2, weil Bündel B eine fünfte Stufe in dieselbe Kette setzt und die
Regel dann ohnehin angefasst wird. Befund 7 aus demselben Grund: Bündel B legt
`ohne_papierkorb` in dasselbe Modul.

**Mit Bündel E:** Befunde 4, 5 und 6. Sie sind derselbe Nachzug, den Bündel E ohnehin fährt.

**Mit Schritt 11:** Befund 3. Er löst sich dort von selbst auf, wenn der Zuschnitt bleibt.

---

## Abgleich 260817-1129 (reconciler)

Kein Befund dieser Durchsicht ist bestätigt-und-behoben; alle sieben stehen offen und sind
einzeln an ihrer zitierten Stelle nachgelesen. Die Einzelnachweise stehen als Zeile
`Abgleich 260817-1129` in den sieben Datensätzen unter `issues/` dieses Circles. Die
Zusammenfassung dieser Durchsicht hält am Baum: die drei Wege in `in_den_papierkorb` und der
eine in `endgueltig_loeschen` gehen weiterhin sämtlich durch `loeschen_nach_rueckfrage`
(`anwendung.rs:4606`), und `auftrag_stellen` trägt keine Löschart.

Der Bereich in der Zeile `**Reviewed-range:**` ist unberichtigt; der Befund dazu ist
`shared/issues/260817-1122_*_der-durchsichtsbereich-schliesst-seinen-ersten-commit-aus.md`.
