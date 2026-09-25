# Durchsicht Turn 2 der Editor-Runde

**Umfang:** `git diff 8ffdffd..HEAD`, neun Commits, 19 Quelldateien und
`resources/default-keymap.toml`. Maßstab: Spec C1 bis C7, Plan S4, S5, S6, S7,
S9, S10, S12, S15, S16, S19 (Teil), S21, die acht `_a_`-Entscheidungsdatensätze
und `CLAUDE.md`.

**Nachgefahren:** `cargo clippy --workspace --all-targets` (0),
`cargo test --workspace` (0). Kein Code geändert.

---

## Zusammenfassung

Die handwerkliche Qualität ist hoch: die Entwurfsantwort aus S4 sitzt genau so,
wie der Schritt sie verlangt, die eine Größen- und Typprüfung aus S10 rechnet
richtig, die vollständigen Fallunterscheidungen sind vollständig geblieben, und
die drei `allow(dead_code)`-Stellen sind mit nachgemessenen Zahlen begründet.

Die schweren Befunde liegen nicht in den einzelnen Schritten, sondern **zwischen**
ihnen. S4 hat den Fokusvorbehalt für den Editor geöffnet und S16 die Textfläche
gebaut; die Gegenseite, die `Anwendungsdelegierter::fokus` den Editor erkennen
lässt, gehört zu S17 und ist offen. In diesem Zwischenstand ist der Editor kein
Editor, sondern eine zweite Bedienung des Dateifensters. Dasselbe Muster bei
Rückgängig: S7 baut das Menü, S16 baut die Textfläche, und die eine Zeile, die
beide verbindet, hat keiner von beiden.

**8 Befunde: 4 High, 2 Medium, 2 Low.**

---

## Was geprüft und für gut befunden wurde

Die sieben Punkte des Auftrags, jeder einzeln nachgerechnet.

**1. Die Nämlichkeitsfrage (S4/S16) sitzt.** In `ereignisse.rs:443-459` steht
`if ist_editorflaeche(&ersthelfer) { return false; }` **vor** der
Klassenprüfung. `grep -c isKindOfClass crates/krk-ui/src/appkit/ereignisse.rs`
liefert 3, keine vierte Klasse. Das Modul kennt den Editor nicht: keine
`use`-Zeile auf `super::editor` oder `crate::editormodell`, nur ein Abschluss
`impl Fn(&NSResponder) -> bool`, in derselben Form wie `faenger` und `senke`.

Eine Randbemerkung ohne eigenen Befund: der Modulkopf sagt „Objektgleichheit der
Objective-C-Zeiger", umgesetzt ist `isEqual:` (`anwendung.rs:1249`). Für
`NSTextView` ist das dasselbe, weil weder `NSResponder` noch `NSView` `isEqual:`
überschreiben, und `Anwendungsdelegierter::fokus` erkennt Leiste und Vorschau
seit der Runde 1 genauso. Der Satz ist ungenau, die Sache stimmt.

**2. Eine Normalisierungsstelle, und es ist die eine.**
`in_gehaltene_form` wird im Programmtext genau einmal gerufen, in
`datei::einlesen` (`datei.rs:319`). In `editormodell.rs` und `appkit/editor.rs`
steht keine zweite Wandlung, kein `\r`, kein `U+FEFF`. Die Befürchtung des
Auftrags trifft nicht zu. Der Befund liegt woanders und ist unten geführt: es
gibt drei **Eingänge** in den Stand, die gar nicht normalisieren.

**3. Die Größenprüfung steht vor dem Lesen.** `oeffnen` fährt `metadata` →
`is_file` → Größe → `File::open` → `read_to_end` mit `take(EDITORGRENZE + 1)`
(`datei.rs:260-297`). Die Reihenfolge ist die, die S10 verlangt, und die
Schranke aus dem Umsetzungsnachtrag hält die Grenze ein, statt sie
vorherzusagen. Die Stelle ist auch wirklich die eine: beide Einstiege und der
Markensprung rufen sie.

**4. Kein Auffangzweig ist eingeschlichen.** `Kommando` (65),
`Kommando::wirkungsbereich` (7 Bereiche), `bereich_des_kommandos`,
`Wirkungsbereich` → `Fokus` in `fokus::wirkt` (5 Werte), `Bereich::seite`,
`Bereich::index`, `Abweisung::meldung`, `Editormeldung::text`,
`Editormeldung::markenstelle`, `Ansicht::andere`: alle vollständig. Die drei
`_ =>`, die `grep` in den geänderten Dateien findet, stehen über `Option<String>`
(`editormodell.rs:204`), über `&str` (`belegungsmodell.rs:144`) und über ein
Slice-Muster (`fenstermodell.rs:553`) — dort ist ein Auffangzweig unvermeidbar
und keiner von ihnen ist neu.

Zwei Verbesserungen sind sogar dazugekommen: `breite_aendern` hat seinen
`_ => Bereich::Links` verloren und fragt jetzt `Fensterseite::andere`, und
`ist_beweglich` leitet sich aus `Bereich::seite` ab, statt die Liste zweimal zu
führen.

**5. Die `allow(dead_code)`-Stellen stimmen, und zwar auf den Kopf genau.**
Nachgemessen in einer Kopie des Arbeitsbereichs, mit entfernten Zeilen:

| Stelle | Behauptung | gemessen |
|---|---|---|
| `editormodell.rs:120` | vierzehn Fundstellen | **14** |
| `editor.rs:107` und `:131` (mit `anwendung.rs:2550`) | drei Fundstellen | **3** |

Verdeckt wird ausschließlich toter Wert: alle 17 Fundstellen sind `pub` und von
den Proben angefasst. Der ablösende Schritt ist in allen drei Kommentaren
genannt (S37 beziehungsweise S22). Anzumerken bleibt, dass `editormodell.rs` ein
Modul-weites `#![allow(dead_code)]` trägt statt gezielter Attribute wie
`editor.rs`; bis S37 fällt damit auch künftiger toter Wert in dieser Datei nicht
mehr auf. Das ist eine bewusste Abwägung des Schrittes und kein Befund.

**6. Die `unsafe_code`-Grenze ist nicht gewachsen.**
`grep -rn 'allow(unsafe_code)' crates/` nennt weiterhin genau zwei Dateien:
`krk-core/src/verzeichnis/sys.rs:50` und `krk-ui/src/appkit/mod.rs:1`.
`appkit/editor.rs` trägt drei `unsafe`-Stellen (`define_class!`, `NSObjectProtocol`,
`msg_send![super(this), init]`) und keine eigene Ausnahme; es lebt von der
Ausnahme des Elternmoduls, wie die neunzehn Nachbarn.

**7. Die abgeschalteten Ersetzungen sind richtig gewählt, aber unvollständig.**
`setRichText(false)` schaltet die Rich-Text-Automatiken mit ab, darunter
Datums- und Link-Erkennung; die vier ausdrücklich genannten sind genau die, die
beim **Tippen** Zeichen ändern. Wesentliches ist nicht mit abgeschaltet: die
Einfärbung der Formatansicht aus C3 läuft über vorübergehende Merkmale des
Layoutverwalters und ist von `setRichText(false)` unberührt, und
Rechtschreib- wie Grammatikprüfung ändern den Textspeicher ohnehin nicht.
**Nicht** abgeschaltet ist `smartInsertDeleteEnabled` — die fünfte Automatik,
die beim Einfügen und Ausschneiden Leerzeichen setzt. Eigener Befund unten.

---

## Befunde nach Thema

### Der Fokus: die eine Hälfte von S4 ohne die andere

**1. `Anwendungsdelegierter::fokus` kennt den Editor nicht (High).**
`anwendung.rs:2131-2159` fragt Leiste und Vorschau ab und fällt sonst auf
`Fokus::Dateifenster` zurück. Mit der Schreibmarke im Editor wirkt damit jeder
Befehl mit `Wirkungsbereich::Dateifenster` und jeder mit
`Wirkungsbereich::Navigator`: `delete` wirft in den Papierkorb, `f5` startet
eine Kopie, `up`/`down`/`tab` bewegen die Auswahl der Dateiliste. Der Umzug der
drei Navigator-Befehle aus S5 läuft leer, und das erste wie das fünfte
Abnahmekriterium von C7 sind gebrochen. Der Doc-Kommentar behauptet das
Gegenteil, und als einzige der drei betroffenen Stellen trägt `fokus()` keinen
Platzhalterhinweis auf S17.
→ `issues/260809-1640_o_der-fokus-kennt-den-editor-nicht-…`

**2. Die Sprungmarke geht ohne Fokusprüfung ins Dateifenster (High).**
`eingabe_ausfuehren` (`anwendung.rs:1466-1480`) reicht `Eingabe::Zeichen`
**immer** an das aktive Dateifenster. Der Zeichenzweig kennt die eine
Fokusabfrage nicht, weil ein Zeichen kein Kommando ist und keinen
`Wirkungsbereich` trägt. Ein Buchstabe im Editor landet damit im Suchpuffer der
Sprungmarke und erreicht die Textfläche nie. Der Befund wird von Befund 1
**nicht** miterledigt.
→ `issues/260809-1648_o_die-sprungmarke-geht-ohne-fokuspruefung-…`

Beide zusammen sind ein Muster: der Vorbehalt aus S4 hat ein Tor geöffnet, und
keine der Stellen dahinter unterscheidet den Editor vom Dateifenster.

### Die Tastatur: zwei Menüeinträge, die nicht greifen

**3. Die Textfläche schaltet `allowsUndo` nicht ein (High).**
`textflaeche_bauen` setzt neun Eigenschaften; `setAllowsUndo` ist nicht darunter,
und der Vorgabewert ist `NO`. Damit registriert die `NSTextView` keine
Rückgängig-Handlung, und die beiden Einträge aus S7 laufen ins Leere. Die
Begründung in `menue.rs:55-61` („bringt ihren Rückgängigverwalter mit") ist
sachlich falsch.
→ `issues/260809-1644_o_die-textflaeche-schaltet-allowsundo-nicht-ein-…`

**4. Auf einer deutschen Tastatur schluckt `cmd+y` das Rückgängig (High).**
S7 legt Rückgängig auf `cmd+z` als Menükürzel, das über das **Zeichen**
anschlägt. S2, der die zeichenbasierte Nachschlagart im Abgriff baut, ist offen;
der Abgriff schlägt weiter über den Tastencode nach. Die Taste mit der Aufschrift
Z liefert auf deutscher Belegung `kVK_ANSI_Y`, findet `cmd+y` →
`vorschau_umschalten` und verbraucht das Ereignis, bevor das Menü es sieht. Auf
dem Referenzgerät liegt Rückgängig damit unter der Taste mit der Aufschrift Y.
Genau davor warnt
`decisions/260808-0140_a_die-y-tasten-liegen-auf-einer-deutschen-tastatur-…`
namentlich.
→ `issues/260809-1642_o_auf-einer-deutschen-tastatur-schluckt-cmd-y-…`

Auch hier das Muster: ein Schritt ist vor seiner sachlichen Voraussetzung
gelandet.

### Der gehaltene Stand

**5. Drei Eingänge in den Stand normalisieren nicht (High).**
`Editormodell::bearbeiten` (`editormodell.rs:553`) nimmt den Stand aus der
Textfläche ungeprüft entgegen; `treffer_ersetzen` und `alle_treffer_ersetzen`
geben ihren `ersatz` roh an `suche`. Eine `NSTextView` bewahrt eingefügtes
`\r\n` zeichengetreu auf. Dann rechnen die drei Stellen falsch, die auf die
Zusage bauen: `inhalt_der_zeile` zieht genau ein Byte ab und liefert einen
Zeileninhalt mit `\r` (womit eine Textmarke sich selbst nicht wiederfindet), die
Suche findet über Zeilengrenzen nichts, und `sicherungsform` schreibt das `\r\n`
zurück auf die Platte. Der Modulkopf von `datei.rs` nennt als anstehenden Fall
nur den Ersatztext aus S37 und übersieht den größeren.
→ `issues/260809-1646_o_die-zusage-ueber-den-gehaltenen-stand-…`

**6. Smart Insert/Delete bleibt an (Medium).**
Die fünfte textverändernde Automatik von AppKit ist nicht abgeschaltet. Sie
greift beim Einfügen und Ausschneiden, nicht beim Tippen, und ist deshalb aus
der Vierergruppe herausgefallen. `speculation:` der Vorgabewert ist nach der
Dokumentation `YES`; im laufenden Bündel nicht gemessen.
→ `issues/260809-1650_o_die-fuenfte-textveraendernde-automatik-…`

### Die Prüfung vor dem Öffnen

**7. Die Typprüfung steht auf dem Pfad und nicht auf dem Deskriptor (Medium).**
`metadata(pfad)` und `File::open(pfad)` sind zwei Auflösungen desselben Pfades.
Wird er dazwischen durch eine benannte Röhre ersetzt, blockiert `File::open` —
genau das, was der Kommentar an der Funktion ausschließen will. Die Folge ist
kein Absturz, sondern ein Arbeitsfaden, der nie endet, und ein Editor, der
kommentarlos nichts öffnet. Der Fall ist ungeprüft: die Proben decken Ordner und
Verknüpfung ab, keine Röhre.
→ `issues/260809-1652_o_die-typpruefung-steht-auf-dem-pfad-…`

### Zählungen und Zusagen in Prosa

**8. „Acht Pfeile" aus `appkit` heraus sind es nicht (Low).**
`appkit/mod.rs:74-84` zählt acht und nennt sie namentlich. `anwendung.rs` trägt
allein acht und kommt in der Aufzählung nicht vor; `leiste`, `vorschau` und
`zwischenablage` fehlen ebenfalls, und `volumes` reicht keine Pfade an
`auffrischung`, sondern zieht `leistenmodell::Ort`. Die Zählung war schon bei
sieben falsch; S16 hat sie fortgeschrieben.
→ `issues/260809-1655_o_acht-pfeile-aus-appkit-heraus-sind-es-nicht-…`

**9. Das erste Abnahmekriterium von C2 beschreibt einen aufgehobenen Zustand (Low).**
Es beruft sich auf `reserviert_fuer = "editor"` und eine leere Tastenliste; S6
hat beides entfernt.
→ `issues/260809-1657_o_das-erste-abnahmekriterium-von-c2-…`

---

## Bindende Grundlage: die acht Entscheidungsdatensätze

| Datensatz | Stand |
|---|---|
| Sprachen der Syntaxhervorhebung | in diesem Turn nicht berührt |
| Textmarke: nur eine Stelle | eingehalten (`marke.rs`, eine Zeilennummer, kein Bereich) |
| Welche Dateien der Editor öffnet | eingehalten; die bindende Zusage („kein Weg darf eine Datei verändern, die nicht verlustfrei als Text gelesen wurde") hält, `oeffnen` weist ungültiges UTF-8 ab |
| Nachfrage bei der Sitzungssicherung | in diesem Turn nicht berührt |
| Suche in der Nähe einer Textmarke | eingehalten, und der tragende Grund ist jetzt zusätzlich belegt: `die_gueltigkeitspruefung_kommt_ohne_lesen_der_datei_aus` |
| **Sicherungsform** | **latent gebrochen** (Befund 5): sobald S26 die Textfläche anbindet, kann der Editor `\r\n` sichern, während der Datensatz „immer Unix-Zeilenenden" sagt |
| **Die y-Tasten auf der deutschen Tastatur** | **gebrochen im ausgelieferten Stand** (Befund 4): S7 hat die Kombination gebaut, vor der der Datensatz gewarnt hat, und S2 ist offen |
| Gerendert bei Markdown | in diesem Turn nicht berührt |

---

## Empfohlene Reihenfolge

**Vor dem nächsten Turn:**

1. Befund 1 (`fokus()` erkennt den Editor) und Befund 2 (Sprungmarke) — sie
   gehören sachlich zu S17 und sind die Bedingung dafür, dass der Editor
   überhaupt bedienbar ist.
2. Befund 3 (`setAllowsUndo`) — eine Zeile, und ohne sie ist S7 nicht eingelöst.

**Vor S26:**

3. Befund 5 (Normalisierung am Eingang des Modells) und Befund 6
   (`setSmartInsertDeleteEnabled`) — beide werden mit der Anbindung der
   Textfläche scharf.

**Vor S42 (Abnahmelauf):**

4. Befund 4 — entweder S2 vorziehen oder als bekannte Einschränkung führen. Wer
   „Rückgängig im Editor" abnimmt, muss wissen, unter welcher Taste es liegt.

**Aufräumen, ohne Eile:** Befunde 7, 8, 9.

---

## Abgleichvermerk 260810-0805

Die vier schweren Befunde dieses Berichts sind **alle vier geschlossen**: `issues/260809-1640_c_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`, `..._c_auf-einer-deutschen-tastatur-schluckt-cmd-y-das-rueckgaengig-des-editors.md` (260809-1642), `..._c_die-textflaeche-schaltet-allowsundo-nicht-ein-und-hat-damit-kein-rueckgaengig.md` (260809-1644), `..._c_die-zusage-ueber-den-gehaltenen-stand-hat-einen-zweiten-eingang-ohne-normalisierung.md` (260809-1646).

Vom Rest des Durchgangs stehen im Speicher `260809-16xx`/`260809-17xx` fünfzehn Defekte: **zehn geschlossen, fünf offen.** Offen bleiben `260809-1610` (Zusicherung nur halb schreibbar), `260809-1652` (Typprüfung auf dem Pfad statt auf dem Deskriptor), `260809-1655` (acht Pfeile aus AppKit heraus), `260809-1728` (Modulkopf von `datei.rs`) und `260809-1746` (Probe auf die wandernden Stellen). Der letzte hängt an derselben ausstehenden Nutzerentscheidung wie `260809-1527`.

Am Bericht selbst ist nichts geändert.
