# Durchsicht: Turn 3, die Behebung der achtunddreissig Defekte

**Sender:** coderev
**Gegenstand:** `git diff 38a02b2..HEAD -- crates Cargo.toml` — 14 Commits, 16 Dateien, 3 994 hinzugefuegte und 588 entfernte Zeilen
**Umfang:** die geaenderten Zeilen und ihr unmittelbarer Zusammenhang. Nicht der ganze Editor, nicht die Runde 1.
**Geraet:** dasselbe Referenzgeraet, macOS 15.7.7 (Build 24G720)

---

## Zusammenfassung

**Der Diff ist tragfaehig.** Der Bau haelt, alle 744 Proben in 16 Zielen laufen durch, Clippy und `fmt` melden nichts. Die drei Umbauten, nach denen eigens gefragt war, halten in ihrem Kern: das Fortschreiben der Einfaerbung ist an 18 940 gemessenen Faellen gleichwertig zum vollen Durchgang, die `fcntl`-Bindung ist richtig deklariert und der Deskriptor leckt auf keinem Weg, und die Rueckgaengig-Mechanik laesst auf keinem der acht Schreibwege einen Stapel stehen, der auf Bereiche einer anderen Datei zeigt.

**Sieben Verhaltensbefunde**, einer davon Hoch. Zwei sind aelter als dieser Diff und stehen hier, weil die Durchsicht sie am Nachbarweg des Behobenen gefunden hat. Ein Befund ist latent: der Fehlerzweig ist da und die Abweichung ist gemessen, aber ich habe den Zweig mit dem eingebundenen Sprachsatz nicht erreichen koennen.

**Ein Befund eines Hilfsagenten ist geprueft und widerlegt.** Die Behauptung, ein `cmd+z` erreiche das `Editormodell` nicht, ruht auf einer Messung an einer `NSTextView` auf TextKit 2. KRK laeuft auf TextKit 1, und dort verschickt jedes `undo` und jedes `redo` genau ein `textDidChange:`, mit dem schon zurueckgenommenen Text. Was davon uebrigbleibt, ist etwas anderes und steht als Befund 3.

## Zahlen

| Schwere | Zahl | davon aus diesem Diff |
|---|---|---|
| Kritisch | 0 | — |
| Hoch | 1 | 1 |
| Mittel | 6 | 4 |
| Niedrig | 0 | — |

Keine Niedrig-Befunde. Diese Sitzung hat dreissig davon behoben; ein weiterer Schwung ueber Kommentarformulierungen hilft nicht, und ich habe keinen gefunden, der es wert waere.

---

## Befunde nach Thema

### Der Rueckgaengigverlauf

**1 — Hoch — der Stapel haelt je eigener Handlung eine ganze Abschrift, und er ist unbegrenzt.**
`issues/260810-1241_o_der-rueckgaengigstapel-haelt-je-eigener-handlung-eine-ganze-abschrift-und-ist-unbegrenzt.md`

`Umkehrpunkt.stand` ist ein `String` (`editor.rs:590-596`), der Punkt wandert in den Block und der Block in den Stapel (`editor.rs:1613-1628`), und `setLevelsOfUndo` steht nirgends im Baum — `levelsOfUndo` ist ab Werk `0`, also unbegrenzt. `Verlauf::Traegt` laesst den bestehenden Stapel ausdruecklich stehen, also summieren sich die Abschriften.

An einer Datei von 16 MB und 100 Ersetzungen mit `shift+cmd+r` — dem Weg, den C5 mit „der wievielte gerade angesteuert ist" selbst anbietet — sind das rund 1,6 GB, frei erst mit dem naechsten Dateiwechsel. Der Doc-Kommentar nennt die Zahl je Handlung; die Summe ueber einen Stapel ohne Tiefengrenze nennt er nicht.

Derselbe Datensatz fuehrt den zweiten Teil: `alle_treffer_ersetzen` baut die Abschrift, bevor es die Trefferzahl kennt (`editor.rs:2086-2098`). Ein `ctrl+cmd+r` ohne Treffer kopiert 16 MB und wirft sie fort.

**2 — Mittel — ein `cmd+z` nach einem Ersetzen loescht den Suchlauf.**
`issues/260810-1244_o_ein-cmd-z-nach-einem-ersetzen-loescht-den-suchlauf-den-das-ersetzen-eigens-aufgebaut-hat.md`

`umkehren` (`editor.rs:1645-1649`) geht ueber `Editormodell::bearbeiten`, und das setzt `self.suchlauf = None` (`editormodell.rs:945`). Beide Ersetzungswege bauen den Suchlauf eigens neu auf, damit `cmd+g` und `shift+cmd+r` weiterlaufen (`editormodell.rs:1186ff`, `:1223ff`); das Rueckgaengig wirft ihn weg. Nach `cmd+f`, `shift+cmd+r`, `cmd+z` antwortet ein zweites `shift+cmd+r` mit `Editormeldung::KeineSuche`.

Neu mit diesem Diff: vorher leerte jedes Ersetzen den Stapel, ein `cmd+z` tat also nichts. Jetzt tut es etwas — und dabei eines zu viel. Es ist zugleich der Fall, in dem die Trefferliste **ausrechenbar** ist: der Text ist der von vorher.

**3 — Mittel — dass ein `cmd+z` ueberhaupt im Modell ankommt, haengt an TextKit 1, und das steht nirgends als tragend.**
`issues/260810-1243_o_dass-ein-cmd-z-ueberhaupt-im-modell-ankommt-haengt-an-textkit-1-und-das-steht-nirgends-als-tragend.md`

Gemessen, dreimal reproduziert, einziger Unterschied der Zugriff auf `layoutManager`:

| Aufbau | `undo` aendert den Text | `textDidChange:` |
|---|---|---|
| ohne Zugriff auf `layoutManager` (TextKit 2) | ja | **0x** |
| mit `_ = sicht.layoutManager` (TextKit 1) | ja | 1x |

KRK ist TextKit 1, aber als **Nebenwirkung**: `merkmale_zuruecksetzen` fasst `layoutManager` beim Aufbau an (`editor.rs:1068` → `:2181` → `:2283`), und die Nummernspalte tut es aus einem anderen Grund (`nummernspalte.rs:89-90`). Der Modulkopf fuehrt in `editor.rs:1833-1837` die Wege auf, die Flaeche und Stand zeichengleich halten, und nennt `textDidChange:` als „die eine Stelle, die AppKit dafuer vorsieht". Dass diese Stelle auf TextKit 2 nicht feuert, steht nirgends.

Heute ist nichts kaputt — nachgebaut mit KRKs Aufbaureihenfolge feuert jedes `undo` und `redo` einmal, und der Rueckweg sieht den schon zurueckgenommenen Text. Wer die Nummernspalte auf `NSTextLayoutManager` nachzieht, bekommt einen gruenen Bau, 744 gruene Proben und ein `cmd+s`, das den zurueckgenommenen Text sichert.

**Nicht als Befund gefuehrt und ausdruecklich geprueft:** dass das CRLF-Richten ueber `Verlauf::TraegtNurDiese` den Verlauf **vor** dem Einfuegen mitnimmt, ist eine benannte und begruendete Entscheidung (`editor.rs:1727-1751`), samt Abwaegung gegen den Eingangsfilter ueber `textView:shouldChangeTextInRanges:replacementStrings:`. Die Begruendung traegt: die Bytefolgenmarke faellt nach ihrer Stelle im **ganzen** Text, und ein Eingangsfilter kennt nur das eingefuegte Stueck. Der Preis ist hoch — ein Einfuegen aus einer Windows-Quelle ist kein Grenzfall —, aber er ist entschieden und nicht uebersehen.

### Die Syntaxhervorhebung

**4 — Mittel — nach einem gescheiterten `parse_line` hebt das Fortschreiben Zustaende auf, die nicht zu ihrer Zeile gehoeren.**
`issues/260810-1242_o_nach-einem-gescheiterten-parse-line-hebt-das-fortschreiben-zustaende-auf-die-nicht-zu-ihrer-zeile-gehoeren.md`

`rechnen` legt den Haltepunkt **vor** der Abfrage auf `faerben` an und ohne sie (`hervorhebung.rs:965-974` gegen `:985`). Nach `Err(_) => faerben = false` wachsen `zustand` und `stapel` nicht mehr, und jeder weitere Haltepunkt traegt den Zustand der Abbruchzeile und behauptet ihn fuer seine eigene. Dazu veraendert `ParseState::parse_line` sich auf dem Weg zum `Err` schon (`syntect-5.3.0/src/parsing/parser.rs:223-230`), der eingefrorene Zustand ist also nicht einmal der der Abbruchzeile.

Gemessen an einer Abschrift des Moduls mit eingesetztem Abbruch: der volle Durchgang faerbt bis Zeichen 777, der fortgeschriebene bis 3 283, 64 Stuecke Unterschied. Damit bricht die Zusage, die das Modul selbst als „die eine Zusage, an der das Fortschreiben haengt" bezeichnet.

**Der Zweig ist heute vermutlich unerreichbar**, und das gehoert zur Einordnung: alle 213 Sprachdefinitionen aus `two_face::syntax::extra_newlines()` gegen 25 harte Probezeilen und, weil `MissingContext` genau dort entsteht, jede von ihnen als Zaun in Markdown — 0 Fehler in beiden Reihen. Die Behebung ist eine Zeile, und ob der Zweig erreichbar wird, entscheidet die naechste Fassung von `two-face` und nicht KRK.

**Was der Diff hier richtig macht, und es ist der grosse Teil.** Die Zusage „von vorn gleicht fortgeschrieben" habe ich unabhaengig von den Proben des Moduls gemessen, an einer Abschrift von `hervorhebung.rs` und mit demselben Maßstab (Wirkung Zeichen fuer Zeichen):

- **18 000 Laeufe** mit zufaelligen Aenderungen, Rust und Markdown, 3 bis 143 Zeilen, sechs Aenderungen je Datei hintereinander: **0 Abweichungen**.
- **940 gezielte Faelle** rings um jede Haltepunktgrenze in einer Datei mit zwoelf Haltepunkten — Anfuehrungszeichen, geoeffneter und geschlossener Blockkommentar, offene Rohkette, Zeile von 20 000 Zeichen, leere Zeile, Zeile entfernt, Zeile eingefuegt, je an `k*32-2` bis `k*32+2`; Blockkommentare ueber 1, 31, 32, 33, 96 und 167 Zeilen; Markdown-Zaeune an Haltepunktgrenzen; Schlussumbruch dazu und fort; leerer Text in beide Richtungen: **0 Abweichungen**.

Die Faelle, nach denen eigens gefragt war, sind also gemessen und halten: mehrzeilige Konstrukte ueber eine Haltepunktgrenze, eine Aenderung genau an einem Haltepunkt, Zeilenenden, sehr lange Zeilen.

**5 — Mittel — die Formatansicht nimmt gesetzte Merkmale des Textspeichers nie wieder heraus.**
`issues/260810-1245_o_die-formatansicht-nimmt-gesetzte-merkmale-des-textspeichers-nie-wieder-heraus.md`
*Aelter als dieser Diff; `formatierung_anwenden` ist darin nur im Kommentar angefasst.*

`formatierung_anwenden` ruft `addAttributes:range:` (`editor.rs:2461`) und nimmt nichts heraus. Die voruebergehenden Merkmale werden vorweg geleert (`:2471`), die des Textspeichers nicht. `merkmale_zuruecksetzen` nimmt nur den Absatzstil heraus und ueberlaesst die Schrift dem `setFont:` in `grundschrift_setzen` — und beide haben denselben einen Aufrufer, `darstellung_nachziehen`, der bei vier Anlaessen laeuft und beim Tippen nicht.

Gemessen in AppKit: `# Kopf` wird zur Ueberschrift gesetzt, das `#` geloescht, und „Kopf" traegt weiter 25,6 pt fett statt 16,0 pt. Dasselbe fuer den Absatzeinzug einer entfernten Liste und die feste Schrift eines entfernten Zauns.

### Der Kern: Deskriptor, `Cow`, Umlauf

**Keine Befunde.** Sieben Dateien geprueft, jede der sieben Fragen beantwortet:

- Die `fcntl`-Deklaration ist **variadisch** (`sys.rs:697`, `fn fcntl(fd: c_int, befehl: c_int, ...) -> c_int`), also richtig fuer arm64; das befuerchtete Register-gegen-Stapel-Problem tritt nicht auf. `open` ist gar nicht selbst gebunden, sondern laeuft ueber `OpenOptions::custom_flags` (`sys.rs:739-742`), also ueber die Bindung der Standardbibliothek. Die sieben uebrigen `extern "C"`-Deklarationen sind nicht variadisch und gegen `copyfile.h` und `sys/attr.h` richtig.
- `O_NONBLOCK` ist auf allen fuenf Wegen abgenommen oder mit dem Deskriptor gestorben; zwischen `open` und dem Abnehmen steht kein weiteres `return`.
- Kein Leck und kein doppeltes Schliessen: einziger Eigentuemer ist `std::fs::File`, kein `from_raw_fd`, kein manuelles `close`. Alle Fehlerausgaenge von `oeffnen` lassen `datei` fallen.
- `errno` wird unmittelbar hinter dem gescheiterten `fcntl` gelesen, ohne Fremdaufruf dazwischen. `EINTR` ist auf jedem Weg behandelt, wo es auftritt.
- `#![allow(unsafe_code)]` steht in `krk-core` allein in `sys.rs:66`. `datei.rs` enthaelt null `unsafe` und loest es ueber `ohne_warten_oeffnen(&Path) -> io::Result<File>` — keine Rohzeiger, kein roher Deskriptor, `fcntl` und `blockierend_stellen` privat. Aus sicherem Code laesst sich darueber kein undefiniertes Verhalten ausloesen.
- Die `Cow`-Umstellung normalisiert **nichts weniger** als vorher. `ist_in_gehaltener_form` (`datei.rs:396-398`) ist genau die Negation dessen, was der Owned-Zweig tut, und trug dieselbe Bedingung schon vor dem Umbau als kurzen Weg. Die eine geaenderte Aufrufstelle (`datei.rs:507`) braucht nur `.len()`.
- Der `suche.rs`-Teil ist verhaltensgleich: `stelle % len` deckt sich fuer `erster_ab` und `naechster` mit dem alten `if stelle < len`, weil beide nie mehr als `len` uebergeben, und `(davor + len - 1) % len` ist Zeile fuer Zeile das alte `checked_sub(1).or_else(...)`.

**6 — Mittel — die Typpruefung am Pfad ist im Vorschauweg geblieben, und dort blockiert sie.**
`issues/260810-1247_o_die-typpruefung-am-pfad-ist-im-vorschauweg-geblieben-und-dort-blockiert-sie.md`
*Betrifft `vorschaumodell.rs`, den Navigator der Runde 1. Steht hier als Gegenstueck zu `260809-1652`; ein Abgleich darf ihn nach `shared/issues/` verschieben.*

Der behobene Defekt hat ein Geschwister, das unbehoben ist. `typ_von` (`vorschaumodell.rs:580-589`) ordnet Roehre, Zeichengeraet, Blockgeraet und Socket in `Typ::Datei` ein, alle melden `st_size == 0` und fallen damit durch die Groessenschranke, und dann steht `std::fs::read(pfad)` (`:542` und `:555`) — genau das blockierende Oeffnen ohne Schranke, das `datei::oeffnen` seit diesem Diff nicht mehr tut.

Gemessen an einer frisch angelegten Roehre ohne Schreiber: `typ_von` liefert `Typ::Datei`, die Groessenschranke greift nicht, `std::fs::read` blockiert. Die Schreibmarke auf eine solche Roehre zu bewegen laesst den Faden `krk-vorschau` fuer die Lebensdauer des Programms stehen — einer je beruehrter Roehre. Auf `/dev/zero` waechst der Puffer ohne Grenze, weil hier keine `take()`-Schranke steht.

### Der Pruefcode

**7 — Mittel — eine Probe liest ein Merkmal, das die Untergrenze macOS 15.0 nicht fuehrt.**
`issues/260810-1246_o_eine-probe-liest-ein-merkmal-das-die-untergrenze-macos-15-0-nicht-fuehrt.md`

`der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl` liest `allowsWritingToolsAffordance` (`editor.rs:4278-4284`), und `merkmal` bricht mit einer Panik ab, wenn der Name fehlt (`editor.rs:4004`). Im SDK gibt es dazu genau eine Fundstelle, und sie steht an `NSTextField` mit `API_AVAILABLE(macos(15.4))`. An `NSTextView` ist der Zugang undokumentiert.

Damit bindet die Probe den Bau weiter an die Fassung des pruefenden Geraets — auf 15.0 bis 15.3, die das Projekt unterstuetzen soll, oder sobald Apple den Zugang fortnimmt. `260810-0417` ist folglich **halb** geschlossen: der Hinweis-statt-Fehlschlag-Zuschnitt gilt allein fuer die Nachbarprobe.

Der Datensatz fuehrt daneben eine Auflegung fuer `decisions/260810-0959`: die vier Einstellungen unter `NochOffen` sind nicht vier von einer Art, sondern zwei oeffentliche auf macOS 15.0 und zwei undokumentierte.

**Sonst nichts am Pruefcode.** Die vier Instanzproben habe ich ueber ihre bekannte offene Frage hinaus angesehen: sie bauen, lesen, setzen, lassen fallen, die Sperre serialisiert sie, ein vergifteter Mutex wird bewusst weitergenommen. Ausser Befund 7 nichts.

---

## Uebergreifende Beobachtungen

**Das Muster des Diffs ist „die eine Stelle", und es traegt.** Elf der vierzehn Commits ziehen eine Aussage von zwei Orten auf einen: die Umlaufregel in `suche.rs`, der Name in `datei.rs`, die Herkunft als Argument statt als `Cell`, der Verlauf als Wert statt als Vermutung, die Aufzaehlung der Automatiken auf zwei Quellen. In allen elf Faellen ist die Zusammenlegung richtig, und in keinem habe ich eine dritte Stelle gefunden, die zurueckblieb.

**Die eine Ausnahme ist Befund 6, und sie ist die lehrreichste.** Der Defekt `260809-1652` wurde als Frage am Editor gestellt und am Editor beantwortet. Dass dieselbe Frage an der Vorschau haengt, hat niemand gefragt — beide lesen die Bytes einer Datei, die der Nutzer nur ausgewaehlt hat. Wenn eine Behebung eine Regel aendert („der Typ wird am Deskriptor erhoben"), gehoert die Suche nach den anderen Stellen dazu, die dieselbe Regel brauchen.

**Zwei Befunde sind Nebenwirkungen von Zusagen, die eine andere Datei gibt.** Befund 3 (TextKit 1) und Befund 5 (die Merkmale) haben dieselbe Bauart: Modul A verlaesst sich stillschweigend darauf, dass Modul B einen Nebeneffekt hat. `nummernspalte.rs` faellt auf TextKit 1 zurueck, um Nummern zu zeichnen — und traegt damit den Rueckweg des Rueckgaengig. `grundschrift_setzen` schreibt die Schrift ueber den ganzen Speicher, um die Ansicht zu setzen — und nimmt damit Auszeichnungen zurueck, aber nur bei vier von fuenf Anlaessen. Beide Male ist die Zusage richtig und der Ort ihrer Aufschreibung falsch.

**Der Speicher ist das Thema, das dieser Diff zweimal halb loest.** `260810-0424` hat die drei Abschriften des 16-MB-Textes im Richten der Flaeche auf eine gebracht, und `c5d6e43` hat die Eingangskopie der Wandlung gespart — beides gemessen und beides richtig. Im selben Diff entsteht mit dem Umkehrpunkt eine neue Abschrift je Handlung, ohne Tiefengrenze (Befund 1). Die Frage „was darf ein Editor an seiner Grenze von 16 MB an Speicher halten" ist an zwei Stellen mit Ja und an einer mit Nein beantwortet.

---

## Reihenfolge

**Vor dem Abnahmelauf** — sie beruehren, was der Lauf messen soll:

1. Befund 1 (Stapeltiefe). Eine Zeile `setLevelsOfUndo`, dazu die Trefferzahl vor die Abschrift. Ohne sie kann der Abnahmelauf an einer grossen Datei am Speicher scheitern und niemand weiss, warum.
2. Befund 2 (Suchlauf nach `cmd+z`). Betrifft C5 unmittelbar; ein Abnahmekriterium ueber „der wievielte gerade angesteuert ist" laeuft daran vorbei oder darauf auf.
3. Befund 7 (die Probe am undokumentierten Merkmal). Sie kann `cargo test` auf einem Geraet rot faerben, das die Untergrenze einhaelt, und dann steht der Lauf still, bevor er beginnt.

**Danach, in dieser Reihenfolge:**

4. Befund 5 (die stehengebliebenen Merkmale). Sichtbar in der Formatansicht bei jeder Markdown-Bearbeitung, und die Behebung ist zwei Nachrichten je Durchgang.
5. Befund 6 (der Vorschauweg). Ein hängender Faden je beruehrter Roehre, und das Mittel liegt fertig im Baum.
6. Befund 3 (TextKit 1). Zwei Zeilen Aufschreibung und eine Zeile Probe; kein Verhalten aendert sich.
7. Befund 4 (der Haltepunkt nach dem Abbruch). Eine Zeile, dazu eine Probe mit eingesetztem Abbruch. Der Zweig ist heute nicht erreichbar; die Behebung kostet weniger als die naechste Pruefung, ob er es noch ist.

**Nichts davon haelt den Abschluss der Runde auf.** Kein Befund ist ein Datenverlust auf einem Weg, den ein Nutzer ohne Absicht geht, und kein Befund ist ein Absturz.

---

## Was ich nicht sicher beantworten kann

- **Ob der Fehlerzweig in `parse_line` erreichbar ist.** Zwei Messreihen ueber alle 213 Sprachdefinitionen finden keinen Weg. Das ist ein Befund ueber diese Fassung von `two-face` und keiner ueber die naechste.
- **Ob AppKit auf macOS 26 dasselbe tut.** Alle Messungen stammen von 15.7.7. Insbesondere „`undo` verschickt auf TextKit 1 ein `textDidChange:`" ist eine Messung und keine zugesagte Eigenschaft. Befund 3 haengt davon nicht ab — er sagt, dass KRK sich darauf verlaesst, ohne es aufzuschreiben.
- **Ob die Fehlszenarien der Befunde 1, 2 und 5 am laufenden Buendel genau so aussehen.** Sie sind am Code nachgezeichnet, und die AppKit-Annahmen darunter sind gemessen; gefahren ist keines. Das Buendel im Vordergrund zu bedienen ist Nutzerarbeit, aus dem Grund, der in `CLAUDE.md` unter „Was man nicht sieht" steht.
- **Ob `O_NONBLOCK` an einer gewoehnlichen Datei auf einer SMB- oder FUSE-Flaeche `EAGAIN` liefert.** Der Kommentar in `sys.rs:721-725` markiert es selbst als ungemessen. Das Abnehmen macht die Frage gegenstandslos; bestaetigen kann ich die Praemisse nicht.

## Abnahmelauf

```
cargo build --workspace                    gruen
cargo test --workspace                     16 Ziele, 744 Proben, 0 Fehlschlaege, 1 uebergangen
cargo clippy --workspace --all-targets     keine Meldung
cargo fmt --all --check                    keine Ausgabe
```

---

## Anmerkung des Abgleichs, 260810-1404

**Alle sieben Befunde dieser Durchsicht sind geschlossen.** Der Abschluss-Abgleich der Sitzung 260810-0845 hat jeden Datensatz gegen den Baum gelesen; keiner behauptet eine Behebung, die im Code fehlt. Die Reihenfolge, die diese Durchsicht unter „Danach, in dieser Reihenfolge" empfiehlt, ist eingehalten worden.

| Befund | Datensatz, jetzt `_c_` | Commit |
|---|---|---|
| 1 — Hoch, Stapeltiefe und Abschrift | `issues/260810-1241` | `bf0fe18` (Bereich statt ganzer Stand), `0140df7` (Budget in Bytes) |
| 2 — Suchlauf nach `cmd+z` | `issues/260810-1244` | `bf0fe18` |
| 3 — TextKit 1 nirgends als tragend | `issues/260810-1243` | `bf0fe18` |
| 4 — Haltepunkt nach gescheitertem `parse_line` | `issues/260810-1242` | `bf0fe18` |
| 5 — stehengebliebene Merkmale | `issues/260810-1245` | `bf0fe18` |
| 6 — Typprüfung im Vorschauweg | `issues/260810-1247` | `bf0fe18` |
| 7 — Probe am undokumentierten Merkmal | `issues/260810-1246` | `bf0fe18` |

**Die Befundtexte tragen die Datensätze mit dem Marker `_o_`, wie sie beim Schreiben standen; alle sieben tragen heute `_c_`.** Die Verweise sind nicht nachgezogen, weil eine Durchsicht ein Zeitstand ist und nicht fortgeschrieben wird.

**Zwei Befunde haben einen eigenen Folgedatensatz hinterlassen, und der ist offen.** Befund 1 hat `issues/260810-1341_o_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md` erzeugt, Befund 7 hängt über `issues/260810-1001_o_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md` an der offenen Nutzerfrage `decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.

Der Satz „Nichts davon hält den Abschluss der Runde auf" ist nach dem Abgleich unverändert richtig, und jetzt aus einem stärkeren Grund: es hält nichts davon mehr offen.
