# Vollbaum-Durchsicht R8: appkit/tabelle.rs und appkit/editor.rs

**Reviewed-range:** `004ff72..7ac511a`
**Not-opened:** none
**Sender:** coderev
**Verification:** beide Dateien vollstaendig gelesen (tabelle.rs 5.569 Zeilen, editor.rs 5.319 Zeilen, `wc -l` am 260826); jede Zeilenangabe am Baum abgelesen; Untergrenzen am SDK unter `$(xcrun --show-sdk-path)` nachgelesen; keine Datei im Quellbaum geaendert, nichts uebersetzt.

Eine Vollbaum-Durchsicht ohne Codeaenderung im Bereich: der Quelltext steht seit `004ff72` unveraendert, die Commits bis `7ac511a` tragen nur Werkbankdateien.

## Summary

Beide Dateien halten jede Zusage, die `CLAUDE.md` ueber sie macht: der Auffangzweig in `Tabelle::kommando_ausfuehren`, die drei Faelle des Doppelklicks, die eine Senke des Tippens, die Byteschranke des Rueckgaengigstapels mit `const _: () = assert!`, `Stapellast` in `Drop`, `Umkehrpunkt` mit Bereich statt Stand, kein `setLevelsOfUndo`, die eine Anmeldung der Textflaeche, eine Nummernspalten-Klasse, die fortgeschriebene Einfaerbung. Was 689 Commits lang niemand als Ganzes gelesen hat, ist Prosa, die den Stand von vor drei bis zehn Runden beschreibt, und zwei Stellen, an denen das Projekt seine eigene Regel nicht anwendet: `#[must_use]` an den Antworten und die Messbarkeit der Blockfreigabe.

## Totals

Critical 0 / High 0 / Medium 2 / Low 12. Vierzehn Datensaetze unter `shared/issues/260826-1327_o_*`, eine `Also seen`-Zeile an `circles/260814-1551-…/issues/260815-0020_o_verdeckten-tab-setzen-…`.

## Die Zusagen aus CLAUDE.md, gegen den Baum gehalten

### Tabelle

| Zusage | Befund |
|---|---|
| `kommando_ausfuehren` endet auf Auffangzweig | haelt: `tabelle.rs:1713-1714` `_ => return false`. **28 der 79 Varianten** haben einen eigenen Zweig (`:1679-1712`); 51 nicht — Liste unten. |
| Doppelklick, drei Faelle | haelt: `:2014-2031` ueber `Einstieg` (`:720-731`, `#[must_use]`, drei Werte). Verknuepfung auf Ordner: `in_zeile_einsteigen` (`:2302-2334`) loest ueber `verweisziel::bestimmen`, das `std::fs::metadata` nimmt (`verweisziel.rs:168-176`) und damit auch eine Kette Verknuepfung → Verknuepfung → Ordner auf. Kein vierter Fall rutscht durch. |
| Tippen filtert, keine Sprungmarke | haelt: `filterzeichen_tippen` (`:2058-2068`) ist die eine Senke; "Sprungmarke" nur noch als Geschichte in `:2035`. `pfad_anspringen`/`eintrag_anspringen` (`:2426`, `:2441`) sind C10, nicht die Sprungmarke. |
| Filtertext ueberlebt jeden Ordnerwechsel | haelt fuer den sichtbaren Tab (`tabs.rs:653-670`). **Fuenfter Weg:** `tab_ordner_setzen` (`:1377-1387`) → `verdeckten_tab_setzen` (`tabs.rs:485-495`) laesst Filtertext, Deep und Content fallen — schon offen als `260815-0020`, `Also seen` angehaengt. |
| Rueckschritt mit zwei Bedeutungen | die Tabelle nimmt den Anschlag **nicht** entgegen: kein `Anschlag` in der Datei; `letztes_filterzeichen_weg` (`:2959`) und `filter_steht` (`:2929`) sind die zwei Fragen, die der Anwendungsdelegierte stellt. Die Regel bleibt an ihrem einen Rufer. |
| `auswahl_auf_namen` fragt `liest()` zuerst | jede namentliche Auswahl geht ueber `eintrag_waehlen` (`:2464-2470`) oder `ordner_lesen(pfad, Some(name))` (`:1488`, wird `zustand.auswahl`); die einzige Auswahl ueber einen Index am Bestand ist `lesen_abbrechen` (`:1657-1665`) **nach** `abbrechen`, also am abgeschlossenen Modell. Kein Weg am alten Bestand vorbei. |
| Abwurf: Rueckfrage und Papierkorb | der Abwurf beschliesst Kopieren oder Verschieben (`Abwurfvorgang`, `:3706-3709`), nie ein Loeschen; `anwendung.rs:6161-6175` startet den Auftrag ohne Blatt, wie F5/F6 ueber `uebertragen` (`anwendung.rs`). Die Rueckfrage der Runde 12 gilt Loeschwegen (`circles/260817-0833-…/_c_circle.md:36`); ein Papierkorb-Weg existiert im Abwurf nicht. Konsistent. |

**Die 51 Kommandos ohne Zweig in `Tabelle::kommando_ausfuehren`** (fuer den Abgleich mit `anwendung.rs`): Abbrechen Bearbeiten Beenden BelegungAnsehen BereichVerbreitern BereichVerschmaelern DateiAnlegen EditorAlleErsetzen EditorAnsichtUmschalten EditorErsetzen EditorRueckwaertsSuchen EditorRundweg EditorSchliessen EditorSichern EditorSuchen EditorUmschalten EditorWeitersuchen EditorZeileSpringen ErstesFensterUmschalten FensterEinblenden FensterSchliessen FensterWechseln FokusDateifenster FokusEditor FokusLeiste FokusVorschau InhaltssucheUmschalten InPapierkorb Kopieren LeisteUmschalten LesezeichenAnlegen LesezeichenHoch LesezeichenLoeschen LesezeichenRunter LesezeichenUmbenennen Notizzettel OrdnerAngleichen OrdnerAnlegen OrdnerDerDatei SpalteDatumUmschalten SpalteGroesseUmschalten SpalteTypUmschalten Teilen TerminalOeffnen TiefeSucheUmschalten UmbenennenStapel Verschieben VorschauUmschalten WeitereInstanz ZweitesFensterUmschalten ZwischenablageAnsehen.

### Editor

| Zusage | Befund |
|---|---|
| Budget in Bytes, `STAPELBUDGET = EDITORGRENZE`, `const _` | haelt: `editor.rs:879`, `:885`. |
| `Stapellast` traegt in `Drop` ab | haelt: `:949-964`, mit `saturating_sub` und `debug_assert`. |
| `Umkehrpunkt` traegt den Bereich | haelt: `:761-774`, `:789-798`; Probe `:3639-3670` misst 3 Bytes an 16 MB. |
| Schranke = Budget plus eine Handlung | haelt: `verlauf_fuer_umbau` `:977-983` (`>` statt `>=`), Probe `:3700-3784`. |
| `setLevelsOfUndo` nirgends | haelt: drei Treffer, alle in Prosa (`:136`, `:740`, `:3794`). |
| Freigabe des Blocks messbar? | **ja, heute schon**: `verwalter_ohne_fenster` (`:3874`) liefert den Verwalter, den `:906-915` fuer unerreichbar erklaert. Medium, eigener Datensatz. |
| eine der zwei angemeldeten Textflaechen | haelt: `textflaeche()` (`:1592`) hat drei Rufer, alle in `anwendung.rs` (`:2358`, `:2588`, `:2593`); der Editor meldet sich nirgends selbst an. |
| `new_unchecked` nur im Pruefmodul | haelt fuer den Ort: drei Treffer in `krk-ui`, alle unter `#[cfg(test)]` (`editor.rs:3875`, `:4730`, `blaetter/mod.rs:1110`). Haelt **nicht** fuer die Zahl: es ist nicht nur `an_einer_flaeche`, und `:3866` sagt "sonst nirgends". Low. |
| eine Nummernspalten-Klasse | haelt: `Nummernspalte::einhaengen` (`:3181`), `nummernspalte::spalte_neu_zeichnen` (`:3042`); keine eigene Zaehlung. |
| Einfaerbung inkrementell | haelt fuer die Rechnung: `einfaerbungsstand` als Vorlage (`:2925-2927`). Der **Text** geht je Anschlag als Abschrift mit (`:2913`); ungezaehlt. Low. |
| Untergrenzen-Abschnitt | keine Beruehrung ueber 15.0. `NSUndoManager` fehlt in der Liste, `registerUndoWithTarget:handler:` (10.11, `NSUndoManager.h:161`) fehlt unter den "fuenf Methoden"; `setContainerSize:` seit 10.11 weich abgekuendigt (`NSTextContainer.h:119-121`). Zwei Low. tabelle.rs: alle Angaben stimmen gegen das SDK, `NSTableViewStyle` (11.0) bleibt die juengste. |
| kein eigenes `#![allow(unsafe_code)]` | haelt: einziger Treffer `appkit/mod.rs:1`. |

## Findings by theme

### 1. Die eigene Regel nicht angewandt (Medium)

- **`#[must_use]`**: editor.rs 0 von ~25 reinen Antworten, tabelle.rs 14 von ~34; `kommando_ausfuehren -> bool` wird in `anwendung.rs:7861`/`:7889` nackt fallengelassen. Datensatz `…_must-use-fehlt-in-editor-rs-ganz-…`. Querschnitt mit vier Datensaetzen dieser Sitzung.
- **Blockfreigabe messbar**: `…_die-freigabe-des-rueckgaengig-blocks-ist-mit-verwalter-ohne-fenster-messbar-…`.

### 2. Prosa, die einen aelteren Stand beschreibt (Low)

- tabelle.rs: fuenf statt sechs Raenge, "einzige Quelle ohne Feld" statt zwei (`:788`, `:857`, `:3222`).
- tabelle.rs: Aktion "auch beim Fokusverlust" (`:3845-3848`) gegen die Messtafel `:4338-4346`.
- tabelle.rs: Doc-Block der Abwurfmeldungs-Probe an der falschen Probe (`:5266-5305`).
- tabelle.rs: Kostenargument in `abwurf_pruefen` (`:3627-3631`) gegen `beschreibbarkeit` je Bewegung (`:3665`).
- editor.rs: `stand_erneuern` "drei Rufer", sechs im Baum; `bauen`/`schliessen` wiederholen den Rumpf (`:1550-1552`, `:1768-1773`).
- editor.rs: `Editormeldung` "zwei von sechs gebaut" ueber einer Tafel mit sieben "gebaut" (`:487-500`).
- editor.rs: `new_unchecked` "hier und sonst nirgends" (`:3866`) gegen `:4730`.
- editor.rs: zwei tote Verweise auf `die_sieben_abgeschalteten_…` (`:4255`, `:4586`), Umlaut in `:3425`, "messt" `:3689`/`:3788`.

### 3. Untergrenzen und Abkuendigung (Low)

- `NSUndoManager`/`registerUndoWithTarget:handler:` (10.11) fehlen im Abschnitt von editor.rs; `enabledTextCheckingTypes` (10.6) in den Proben.
- `setContainerSize:` weich abgekuendigt seit 10.11 (`editor.rs:2864`).

### 4. Nutzersichtbar (Low)

- Zwei Statuszeilentexte in ASCII-Umschrift (`tabelle.rs:2409-2416`) neben Umlauten in derselben Datei.
- Zweite Abschrift des Standes je Anschlag in der Formatansicht (`editor.rs:2913`), ungezaehlt.

## Cross-cutting observations

- **Zaehlende Prosa veraltet auch in diesen zwei Dateien**, in derselben Weise wie in CLAUDE.md: "fuenf Raenge", "drei Rufer", "vier Aufrufer", "zwei von sechs". Die Dateien halten zugleich vor, wie es besser geht — Zaehlproben ueber `quellbaum::aufrufstellen` (`tabelle.rs:5057-5099`, `:5524-5568`). Wo eine Zahl in einem Doc-Kommentar steht und keine Probe sie haelt, ist sie in dieser Durchsicht in acht von acht Faellen falsch gewesen.
- **`let _ =` gegen nackten Ruf**: dieselbe Antwort (`kommando_ausfuehren`) wird in `anwendung.rs` einmal mit `let _ =` (`:844`) und zweimal nackt (`:7861`, `:7889`) fallengelassen. Das ist genau der Zustand, den `#[must_use]` beendet.
- **Kein toter Zweig, kein verschluckender Auffang, kein `unwrap` mit echtem Fehlerfall** in beiden Dateien: die `expect`s stehen in Proben oder an einer Invariante, die der Aufbau haelt (`tabelle.rs:4795`); die stillen Ausgaenge (`kontextbefehl_melden` `:1817-1826`, `Namensfeld::delegierter` `:4606-4609`, `umkehrung_anmelden` ohne Verwalter `:2101-2103`) sind je begruendet und im Doc benannt. Die `RefCell`-Regel des Modulkopfs (keine Ausleihe ueber einen AppKit-Aufruf) haelt an jeder gelesenen Stelle.

## Recommended sequencing

1. `#[must_use]` an Typen und Antworten beider Dateien, dann `make check` — eine Stunde, faengt sofort die zwei nackten Rufe im Messmodus.
2. Die Freigabemessung als siebte Verwalterprobe — deckt die Annahme, an der die Tiefe des Verlaufs haengt.
3. Die zwoelf Low-Datensaetze in einem Aufraeumlauf; keiner haelt eine Auslieferung an.
