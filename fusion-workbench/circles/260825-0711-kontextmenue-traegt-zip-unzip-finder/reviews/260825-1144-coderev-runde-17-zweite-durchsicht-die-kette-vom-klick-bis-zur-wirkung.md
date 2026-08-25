# Codedurchsicht: Runde 17, zweite Durchsicht — die Kette vom Klick bis zur Wirkung

**Reviewed-range:** `6ad9198..6faaa91`
**Not-opened:** none

**Geoeffnet:** alle sechs Commits des Bereichs als Unterschied, dazu jede der vierzehn geaenderten
Codedateien am Baumstand an den beruehrten Stellen. Gelesen sind ausserdem die fuenf geschlossenen
Defektdatensaetze der ersten Durchsicht samt ihrer `Resolved:`-Notizen, die fuenf beantworteten
Entscheidungsdatensaetze und die zwei einschlaegigen Abschnitte von
`shared/history/260824-2120-orchestrator-session.md`. Hier gefahren am 260825:
`cargo fmt --all --check` (Exit 0), `cargo clippy --workspace --all-targets -- -D warnings`
(Exit 0), `cargo test --workspace` (Exit 0, kein Fehlschlag in 21 Probenzielen).

Diese Durchsicht schliesst an `260825-0942-coderev-runde-17-zip-unzip-und-die-regel-des-kontextmenues.md`
an, die `428fbc4..423d5f2` abdeckt; zusammen tilen die beiden den ganzen Sitzungsbereich.

---

## Zusammenfassung

Vier der fuenf Befunde der ersten Durchsicht sind an der Wurzel behoben, und drei davon besser als
vorgeschlagen. Der fuenfte, B3, ist im Mechanismus behoben und in seiner **Zusage** nicht: der
Modulkopf von `zippen.rs` verspricht, dass die Quellen des Laufs nie angetastet werden, und
begruendet es mit einem Argument ueber Codestellen, das ueber Pfadwerte nichts sagt. Ein zweiter
Zip-Lauf ueber denselben Ordner stellt den Gegenfall von selbst her. Daneben stehen drei kleinere
Befunde, darunter eine Luecke in der Dreierkette der neuen Proben.

## Summen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 2 |
| Gering | 1 |

## Die fuenf Befunde der ersten Durchsicht, einzeln nachgelesen

**B1 — Archivname aus Punkten. Behoben, und die Abweichung vom Vorschlag ist die bessere Wahl.**
Der Vorschlag lautete "eine Pruefung in `paar`"; gebaut ist sie in `ordnername_zum_archiv`
(`crates/krk-ui/src/kommandos/kontextmenue.rs:425-437`) und in `archivname`
(`:386-397`), beide ueber die neue Funktion `brauchbarer_stamm` (`:283-343`), die
`krk_core::operation::umbenennen::name_pruefen` fragt und auf `ERSATZSTAMM` zurueckfaellt. Die
Begruendung im Doc-Kommentar traegt: die Zusage "das ist ein Name" gehoert der Funktion, die den
Namen herausgibt, und `ordnername_zum_archiv` ist `pub`. Nachgelesen, dass `name_pruefen`
(`crates/krk-core/src/operation/umbenennen.rs:68-82`) alle vier gefaehrlichen Gestalten abweist:
`.`, `..`, den leeren und den nur aus Leerzeichen bestehenden Namen und jeden Namen mit
Schraegstrich — das Letzte schliesst zugleich, dass ein absoluter Stamm `Path::join` den ganzen
Pfad ersetzt. `paar` (`:503-506`) ist der einzige Weg zu einem Zielpfad, und er geht durch die
geprueften Funktionen. Die Probe `kein_entpackziel_verlaesst_den_angezeigten_ordner` prueft ueber
zehn Namen die **Gestalt** des Ergebnisses statt einer Liste erwarteter Namen und begruendet, warum
beide Bedingungen noetig sind. Nicht zugedeckt.

**B2 — Roehre mit Schreiber. Behoben, und die Probe kann den Fall jetzt treffen.** `datei_packen`
fragt `metadata()` am offenen Deskriptor (`crates/krk-core/src/operation/zippen.rs:344-357`), und
zwar **vor** `start_file`, sodass ein ausgelassener Eintrag keine leere Zeile im Archiv
hinterlaesst. Die neue Probe `eine_benannte_roehre_mit_schreiber_haelt_das_packen_nicht_an`
(`crates/krk-core/tests/operation.rs:1660-1712`) haengt einen `O_RDWR`-Schreiber an die Roehre und
haelt ihn ueber den ganzen Lauf; gewartet wird mit Frist ueber den neuen Helfer `bericht_mit_frist`,
damit ein Rueckfall den Befund **meldet**, statt den Testlauf stehen zu lassen. Die alte Probe
bleibt als der leichtere Fall stehen und sagt in ihrem Kopf, warum sie den schwereren nicht treffen
konnte. Beide Prosastellen sind nachgezogen, auch die in `verzeichnis/sys.rs:821-835`, die die
Berichtigung ausdruecklich als Berichtigung ausschreibt. `#![deny(unsafe_code)]` bleibt unberuehrt:
der Fix braucht kein `unsafe`.

**B3 — "Ueberschreiben" beim Packen. Im Mechanismus behoben, in der Zusage nicht.** Der Weg ist
gebaut, wie der Nutzer ihn gewaehlt hat: `zippen::lauf` und `zielarchiv_klaeren` nehmen den
`Papierkorb` entgegen, der Zweig ruft `papierkorb.in_den_papierkorb(ziel)`
(`crates/krk-core/src/operation/zippen.rs:224`), `loeschen::baum_entfernen` ist aus dem `use` der
Datei verschwunden. Die Attrappe der Proben hat eine zweite Fassung bekommen, die wirklich umhaengt
(`fs::rename`), damit der Lauf danach weiterarbeiten kann, und `raeumend` ist die staerkere Wahl:
was in der Ablage ankommt, ist vollstaendig da, und genau daran ist abzulesen, dass kein rekursives
Loeschen im Spiel war. Die **Zusage** dagegen haelt nicht — siehe B6 unten.

**B4 — `#[must_use]`. Behoben, und der Nebenvorschlag ist mitgenommen.** Die Marke steht am Typ
`Packschritt` (`zippen.rs:109-115`) und, wie vorgeschlagen, auch an `Zielentscheid`
(`crates/krk-core/src/operation/mod.rs:129-136`), beide mit Begruendung und mit Verweis auf
`Ablauf`, sodass die drei als eine Gruppe lesbar sind.

**B5 — die zwei Prosastellen zu `enclosed_name`. Behoben.** Modulkopf
(`crates/krk-core/src/operation/entpacken.rs:47-64`) und Rumpfkommentar (`:229-233`) trennen jetzt
die zwei Ausgaenge: `..` liefert `None` und laesst den Eintrag aus, ein fuehrender Schraegstrich
wird abgestreift und der Eintrag entsteht im Zielordner. Der Modulkopf nennt die Probe, die beide
Ausgaenge ausschreibt.

## Befunde nach Themen

### Thema 1: eine Zusage, die auf Codestellen zeigt statt auf Werte

**B6 — "Ueberschreiben" raeumt eine Quelle des Laufs in den Papierkorb, wenn der Archivname ihrem
Namen gleicht. Hoch.**

`crates/krk-core/src/operation/zippen.rs:44-54` sagt zu:

> Die Quellen des Laufs faellt dieser Zweig ohnehin nie an — beide Stellen, die hier etwas
> wegnehmen (`zielarchiv_klaeren` und `halbes_archiv_wegraeumen`), liegen auf dem Zielpfad und
> keine auf `auftrag.quellen`.

Der Nachsatz ist wahr und traegt die Zusage nicht. Entscheidend ist nicht, welche Variable die
Loeschstelle liest, sondern ob der **Wert** von `ziel` mit einem Quellpfad zusammenfaellt. Er kann
es:

```
Ordner "Projekte", mehrere Eintraege markiert
  archivname(&pfade, ordner)          -> <ordner>/Projekte.zip     (Ordnername + ".zip")
  Auftrag::zippen(pfade, ziel)        -> ziel wird aus pfade NICHT herausgenommen
  zweiter Lauf, Projekte.zip markiert -> ziel ∈ quellen
  "Ueberschreiben"                    -> eine Quelle geht in den Papierkorb
```

Belegt Zeile fuer Zeile: `crates/krk-ui/src/appkit/anwendung.rs:6112` (die Quellen),
`:6118` (das Ziel aus denselben Quellen), `:6122-6127` (beide unveraendert in den Auftrag),
`crates/krk-ui/src/kommandos/kontextmenue.rs:386-397` (mehrere Eintraege ergeben den Ordnernamen —
belegt von der Probe `der_archivname_haengt_die_endung_an`, `:686-689`),
`crates/krk-core/src/operation/auftrag.rs:160-162` (keine Aussonderung) und
`crates/krk-core/src/operation/zippen.rs:224` (kein Vergleich gegen `auftrag.quellen`).

Zerstoert wird nichts: seit dieser Runde geht der Eintrag in den Papierkorb. Falsch ist die Zusage,
und sie ist die, die der Nutzer der Antwort ausdruecklich mitgegeben hat.

**Die Probe deckt genau die andere Haelfte ab.**
`ueberschreiben_raeumt_allein_den_gleichnamigen_eintrag_in_den_papierkorb`
(`crates/krk-core/tests/operation.rs:1488-1530`) baut den Nachbarn so, dass er **anders** heisst als
das Archiv: Quelle `Projekte`, Ziel `Projekte.zip`. Sie belegt "der aehnlich heissende Nachbar
bleibt" und laesst "die Quellen des Laufs nie" ungeprueft — also die Haelfte, die im Modulkopf als
die selbstverstaendliche dasteht.

**Dieselbe Gestalt beim Entpacken, und diese Runde stellt sie selbst her.** Die vierte
Nutzerentscheidung haengt die Endung an: aus `a.zip` wird `a.zip.zip`. Werden danach beide
markiert und mit Unzip genommen, rechnet `paar` fuer das zweite Archiv den Zielordner
`<ordner>/a.zip`, also den Pfad der ersten Quelle desselben Laufs. Der Unterschied: das Blatt nennt
den Zielpfad, und keine geschriebene Zusage steht dagegen.

Datensatz: `issues/260825-1144_o_ueberschreiben-raeumt-eine-quelle-des-laufs-in-den-papierkorb-wenn-der-archivname-ihrem-namen-gleicht.md`

### Thema 2: die Kette vom Klick bis zur Wirkung

Gefragt war, ob die drei neuen Proben zusammen jeden Weg abdecken. Die Kette hat sechs Glieder:

```
Rechtsklick ─> menuNeedsUpdate: ─> eigene_kontexteintraege_anfuegen
                                     │ Titel + Marke + Selektor + Ziel
Klick ─> kontextbefehl: ─> kontextbefehl_melden ─> von_menuemarke ─> Rueckruf
                                                                      │
                                          kontextbefehl_ausfuehren ───┘ ─> drei Zweige
```

**Fuenf davon sind gehalten, und zwei ausserhalb der drei genannten Proben:**

- Der Selektor: `der_kontextmenue_selektor_hat_einen_empfaenger_und_einen_setzer`
  (`crates/krk-ui/src/appkit/tabelle.rs:5299`) zaehlt Erklaerung und Setzer je genau einmal, und weil
  beide Nadeln denselben Namen tragen, faellt auch ein einseitiger Vertipper auf.
- Der Rueckruf: `der_kontextmelder_wird_beim_aufbau_gesetzt`
  (`crates/krk-ui/src/appkit/anwendung.rs:9012`) zaehlt die eine Aufrufstelle und benennt in ihrem
  Kopf selbst, was sie nicht sieht.
- Der Rundweg ueber die Marke: `der_rundweg_ueber_die_marke_schliesst` und
  `keine_marke_steht_zweimal` (`crates/krk-ui/src/kommandos/kontextmenue.rs:588-620`), dazu
  `die_null_und_alles_daneben_benennen_keinen_befehl` — die Zaehlung beginnt bei eins, damit ein
  `NSMenuItem` ohne gesetzte Marke nicht auf "Zippen" hinauslaeuft. Sauber gebaut und sauber
  begruendet.
- Titel und Marke gegen eine von Hand geschriebene Tafel, samt Vollstaendigkeitsprobe ueber `ALLE`.
- Die Verzweigung selbst ist ohne Auffangzweig (`anwendung.rs:6081-6087`), ein vierter Wert haelt
  also den Bau an.

**Das sechste Glied ist nicht gehalten**, und es ist gerade das, fuer das die dritte Probe gebaut
wurde: welcher Befehl welche Wirkung ausloest. Siehe B7.

**B7 — Die Probe "Befehl → Zweig → Wirkung" prueft Vorhandensein statt Paarung. Mittel.**
`crates/krk-ui/src/appkit/anwendung.rs:9071` stellt zwei voneinander unabhaengige Fragen an
denselben Rumpf:

```rust
verzweigung.contains(befehl) && verzweigung.contains(zweig)
```

`verzweigung` ist der ganze Rumpf von `kontextbefehl_ausfuehren`, also alle drei Zeilen auf einmal.
Waeren `Zippen` und `Entpacken` auf die Zweige des jeweils anderen gelegt, traegt der Rumpf
weiterhin alle drei Befehlsnamen und alle drei Zweignamen, und die zweite Haelfte der Probe prueft
danach unveraenderte Zweigrumpfe. Alle drei Durchgaenge gruen, und der Eintrag "Zip" entpackt. Der
Doc-Kommentar der Probe behauptet mehr, als der Rumpf prueft.

Der Vorschlag im Datensatz ist eine Zeile: den Rumpf zeilenweise lesen und verlangen, dass genau
**eine** Zeile beide Nadeln traegt.

Datensatz: `issues/260825-1144_o_die-probe-befehl-zweig-wirkung-prueft-vorhandensein-statt-paarung-und-bleibt-bei-vertauschten-zweigen-gruen.md`

### Thema 3: eine Entscheidung, zwei Lesarten

**B8 — Die Sitzungsgeschichte sagt "Gekuerzt wird das Blatt allein beim Zip"; der Baum kuerzt auch
beim Entpacken eines einzelnen Archivs. Mittel.**
`crates/krk-ui/src/kommandos/operationen.rs:482-491` liefert fuer `Art::Entpacken { ziele }` den
Wert `ziele.len() == 1`; ein Unzip ueber ein Archiv bekommt damit die gekuerzte Gestalt. Der Satz in
`shared/history/260824-2120-orchestrator-session.md`, Punkt 5, sagt das Gegenteil.

Die Codefassung ist die konsequentere Lesart — die zwei Gruende des Datensatzes (Ueberspringen faellt
mit Abbrechen zusammen, das Ankreuzfeld hat keinen weiteren Fall) haengen an der Zahl der Ziele und
nicht an der Vorgangsart, und der Doc-Kommentar schreibt genau das aus. Es sind trotzdem zwei
Lesarten, und der betroffene Datensatz steht vor dem Hochstufen. Solange die Frage offen ist, traegt
`decisions/260825-0711_*_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`
kein `Implemented:`: die Gestalt ist gebaut, ihr Geltungsbereich ist es nicht.

Datensatz: `issues/260825-1144_o_die-sitzungsgeschichte-sagt-gekuerzt-wird-allein-beim-zip-der-baum-kuerzt-auch-bei-einem-einzelnen-archiv.md`

### Thema 4: eine Zeile Prosa

**B9 — `crates/krk-ui/src/appkit/tabelle.rs:199` beginnt mit `//! //!`. Gering.** Die einzige Stelle
dieser Art im Baum; der Absatz zur Verfuegbarkeit von `clickedRow` verliert damit seine
Auszeichnung. Er gehoert zu dem Abschnitt, der in diesem Vorhaben die einzige Traegerin der
Untergrenzen-Angabe ist.

Datensatz: `issues/260825-1144_o_ein-doppeltes-kommentarzeichen-in-tabelle-rs-entwertet-den-absatz-zu-clickedrow.md`

## Die Tastenbelegung des gekuerzten Blattes: geprueft, haelt

Beide Gestalten und beide Tafeln nachgelesen, dazu die zwei Funktionen, aus denen die Tasten
gerechnet werden.

```
  mehrere Ziele  Überschreiben  Überspringen  Umbenennen  Abbrechen
                 Cmd+Return     Return        Opt+Return  Esc

  ein Ziel       Überschreiben  Umbenennen    Abbrechen
                 Cmd+Return     Opt+Return    Return
```

- Die Eingabetaste liegt in keiner Gestalt auf "Überschreiben". `bestaetigungsstelle`
  (`crates/krk-ui/src/appkit/blaetter/mod.rs:482-487`) sucht die Schaltflaeche mit `Taste::Eingabe`
  und faellt sonst auf `abbruchstelle`; in der gekuerzten Gestalt trifft sie "Abbrechen". Die Probe
  `die_eingabetaste_traegt_in_keiner_gestalt_das_ueberschreiben` haelt es fuer beide Gestalten.
- `mit_schaltflaechen` setzt **jeder** Schaltflaeche ihre Taste ausdruecklich
  (`blaetter/mod.rs:647-656`), also erbt die erste nicht die Vorgabe von `NSAlert`.
- In der gekuerzten Gestalt traegt keine Schaltflaeche `Taste::Escape`. Der Weg der Escape-Taste ist
  der Abbruchbefehl ueber `Blattgriff::abbrechen`, und der Griff wird gehalten
  (`crates/krk-ui/src/appkit/anwendung.rs:6386`). `abbruchstelle` und `bestaetigungsstelle` fallen
  dort auf dieselbe Schaltflaeche; `beide_gestalten_lassen_ueber_abbrechen_liegen` prueft genau das.
- Der Eingabewaechter des Namensfeldes rechnet aus denselben zwei Funktionen. Tabuliert der Nutzer
  in das Feld, tun Return und Escape dort dasselbe wie ausserhalb, naemlich abbrechen.
- Das Ankreuzfeld: `wahl_fuer_alle_zeigen` wird in der gekuerzten Gestalt nicht gerufen,
  `suppressionButton()` liefert dann `nil`, und `zeigen_mit_wahl` liest daraus `false`
  (`blaetter/mod.rs:799-801`). Die Rueckrechnung fragt seit dieser Runde die **Antwort** statt der
  Stelle, weil die Stelle des Abbruchs je Gestalt eine andere ist — richtig gesehen.
- `tastenhinweis` sagt je Gestalt dasselbe, was `schaltflaechen` anlegt, und eine Probe haelt es.

Kein Befund.

## `entpackbefund()`: eine Ausleihe, keine zweite Regel

Geprueft, weil die Frage ausdruecklich gestellt war. `DateifensterQuelle::entpackbefund`
(`crates/krk-ui/src/appkit/tabelle.rs:1750-1755`) hat drei Zeilen Rumpf: Tabmodell ausleihen,
`operationen::betroffene` rufen, `kontextmenue::entpackziel` rufen. Sie rechnet nichts nach, sie
faellt keine Entscheidung, und beide Regeln bleiben dort, wo sie ohne AppKit pruefbar sind — genau
der Zuschnitt von `betroffene_eintraege` daneben. Dass sie `operationen::betroffene` selbst ruft,
statt `self.betroffene_eintraege()` zu nehmen, ist begruendet und richtig: so gehen Auswahl und
Bestand durch **eine** Ausleihe, und der Ausfuehrende bekommt keinen zweiten Weg an das
Ordnermodell.

Ein `#[must_use]` traegt sie nicht — wie `betroffene_eintraege` daneben auch nicht. Beide sind reine
Abfragen ohne Nebenwirkung; ein fallen gelassener Rueckgabewert taete nichts und bliebe nicht
unbemerkt falsch. Die Bindung aus `CLAUDE.md` trifft sie nicht, und die zwei sind untereinander
gleich behandelt. Kein Befund.

## Die fuenf Entscheidungen, gegen den Baum gelesen

| Entscheidung | Umgesetzt | Wo |
|---|---|---|
| Archivname haengt die Endung an | ja | `kontextmenue.rs:386-397`, Probe `der_archivname_haengt_die_endung_an`; erreicht ueber `anwendung.rs:6118` |
| Archiv an der Endung erkannt, ohne Dateizugriff | ja | `kontextmenue.rs:277-281` (`ist_zipname`), gerufen aus `entpackziel`; kein `open` im Modul |
| Zielordner steht da: dieselbe Rueckfrage, Papierkorb statt Loeschen | ja | `entpacken.rs`, `zielordner_klaeren`; erreicht ueber `entpackauftrag_stellen` |
| Konfliktblatt bei genau einer Zieldatei: drei Antworten, Return auf Abbrechen | **teils** | `konflikt.rs:126-156` und `anwendung.rs:6350-6368`; der Geltungsbereich ist strittig, siehe B8 |
| Unzip nimmt `betroffene` und entpackt jedes Archiv darin | ja | `kontextmenue.rs:466-497` liefert eine Liste, `Auftrag::entpacken` trennt sie; bei mehreren Archiven greift `erzeugt_genau_ein_ziel` nicht, das volle Blatt samt Ankreuzfeld erscheint |

Vier tragen ein `Implemented:` zu Recht. Der vierte Datensatz sollte es erst tragen, wenn B8
beantwortet ist.

## Die Projektbindungen

Alle geprueft, alle halten.

- **`#[must_use]`**: `Packschritt` und `Zielentscheid` haben es bekommen (B4). Die fuenf neuen
  `pub`-Funktionen in `kontextmenue.rs` tragen es. `erzeugt_genau_ein_ziel` traegt es samt
  Begruendung.
- **`#![deny(unsafe_code)]` in `krk-core`**: unberuehrt. Der B2-Fix kommt mit `File::metadata` aus,
  `verzeichnis/sys.rs` hat nur Prosa bekommen, die Ausnahme dort ist nicht erweitert.
- **Jedes `unsafe` in `appkit/` mit Begruendung**: die zwei neuen Stellen in `tabelle.rs`
  (`#[unsafe(method(kontextbefehl:))]` und `setTarget:`) tragen je einen SAFETY-Block, und der
  zweite nennt den Grund, aus dem der Ring offen bleibt: `target` ist eine schwache Eigenschaft.
- **Abschnitt "Ab welchem macOS die angesprochenen Klassen stehen"**: in jeder Datei unter
  `appkit/` ausser `koordinaten.rs` und `mod.rs`, den zwei begruendeten Ausnahmen; am 260825
  nachgezaehlt. `tabelle.rs` hat die fuenf neuen Beruehrungen einzeln nachgetragen, mit
  Zeilenangaben am SDK, und nennt ausdruecklich den Erzeuger, den die Datei **nicht** ruft.
- **Vollstaendige Fallunterscheidungen ohne Auffangzweig**: `kontextbefehl_ausfuehren`,
  `Kontextbefehl::titel`, `menuemarke`, `erzeugt_genau_ein_ziel` und `brauchbarer_stamm` (das die
  vier `Namensfehler` einzeln aufzaehlt, statt `Err(_)` zu schreiben). Der `_ =>`-Zweig in `antwort`
  (`konflikt.rs`) ist ueber `usize` nicht zu vermeiden und faellt auf "Abbrechen", also auf den
  ungefaehrlichen Ausgang; begruendet steht es daneben.
- **Prosa deutsch**: durchweg.
- **L9 (kein Dateisystemzugriff, der den Hauptfaden waehrend eines laufenden Vorgangs anhaelt)**:
  beide neuen Auftraege gehen ueber `operation::starten` auf den Arbeitsfaden. Das neue Modul
  `kontextmenue` fasst kein Dateisystem an; `entpackziel` rechnet auf `Ordnermodell::zeilen`, also
  auf dem Bestand im Speicher, und `ist_zipname` liest keinen Inhalt. `im_finder_zeigen` ruft
  `ordner_fehlt`, also **ein** `metadata()` auf dem Hauptfaden — dieselbe Stelle und dieselbe
  Begruendung wie der Terminal-Befehl aus C11, aus dem sie stammt, und kein Vorgang laeuft dabei.

## Was quer liegt

**Die zwei schwersten Befunde beider Durchsichten teilen eine Form, und sie hat sich verschoben.**
In der ersten Durchsicht sagte ein **Typ** etwas zu, was er nicht geprueft hatte (`Typ::Datei`), und
ein **Name** sah aus wie einer und war keiner. Beides ist behoben. Jetzt sagt ein **Modulkopf**
etwas zu, was sein Beleg nicht traegt: "keine Loeschstelle nennt `auftrag.quellen`" beweist eine
Aussage ueber den Quelltext und wird als Aussage ueber die Laufzeit gelesen. Der Unterschied zur
ersten Runde ist, dass diesmal die Zusage vom Nutzer stammt und nicht vom Bau.

**Zwei Proben zeigen dieselbe Schwaeche, und sie ist die des Zaehlens am Quelltext.** B7 zaehlt
Fundstellen in einem Rumpf statt in einer Zeile; die Zusage aus B6 wird von einer Probe gehalten,
deren Aufbau den gefaehrlichen Fall ausschliesst. Beide Male ist nicht die Sorgfalt das Problem,
sondern der Zuschnitt der Frage: eine Probe, die nur nach Vorhandensein fragt, kann die Zuordnung
nicht sehen, und eine Probe, die den Nachbarn anders benennt, kann den Zusammenfall nicht sehen.
`crate::quellbaum` schreibt diese Grenze in seinem Kopf selbst aus ("die verbleibende Blindheit am
Doc-Kommentar der Probe benennen statt sie im Namen der Probe zu ueberschreiben") — B7 ist der Fall,
in dem der Doc-Kommentar es nicht tut.

**Die Ausnahme mit Ablaufdatum hat gehalten, was ihr Kopf versprach.** Kein `expect(dead_code)`
steht mehr in `kontextmenue.rs` oder `operationen.rs`; die verbliebenen Fundstellen im Baum sind
Prosa ueber die gefallene Ausnahme. Der Bau haette angehalten, waere eine stehengeblieben.

**Eine Beobachtung ohne Datensatz.** `kontextbefehl_ausfuehren` ist ein zweiter Eingang in die
Operationsmaschine, der **nicht** durch `kommandos::zulaessigkeit` geht und deshalb auch nicht durch
die Blattsperre, die `CLAUDE.md` beschreibt. Das ist folgerichtig — ein Menueklick ist kein
Tastendruck, und `vorgang_laeuft_schon` steht in beiden auftragstellenden Zweigen. Dass ein
stehendes Blatt den Rechtsklick nicht durchlaesst, haengt allein daran, dass AppKit ein
fensterbezogenes Blatt modal fuehrt: *inference*, am laufenden Buendel nicht nachgemessen, und die
Art von Annahme, die dieses Vorhaben sonst ausschreibt. Genannt, damit sie beim Abnahmelauf
mitgelesen wird.

## Reihenfolge

1. **B6** vor dem Rundenabschluss. Die Zusage stammt vom Nutzer und steht heute falsch begruendet im
   Baum. Entweder der Fall wird geschlossen, oder die Zusage wird auf das eingeschraenkt, was sie
   traegt — beides ist eine bewusste Entscheidung und keine Zeile nebenbei.
2. **B8** vor dem Hochstufen der Entscheidungsdatensaetze. Sie kostet eine Nutzerantwort, nicht mehr.
3. **B7** mit dem naechsten Zug an derselben Datei. Der Fix ist drei Zeilen, und ohne ihn traegt die
   Kette an ihrem letzten Glied eine Probe, die den einen Fehler nicht sieht, gegen den sie gebaut
   ist.
4. **B9** ist Aufraeumen.

Ausserhalb dieser Durchsicht bleiben die zwei bekannten offenen Datensaetze der Runde: der
Zeitstempel 1980 an jedem gepackten Eintrag (`issues/260825-0838`) und der selbst getippte Name im
Konfliktblatt (`shared/issues/260825-1130`, um eine Zeile ergaenzt: nach einem Abbruch loescht
`halbes_archiv_wegraeumen` die getroffene Datei endgueltig).
