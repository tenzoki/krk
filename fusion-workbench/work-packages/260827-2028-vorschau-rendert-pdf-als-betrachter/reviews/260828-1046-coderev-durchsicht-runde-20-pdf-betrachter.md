# Durchsicht der Runde 20: die Vorschau rendert PDF als Betrachter

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `2033626..48cd818`
**Not-opened:** none
**Sender:** coderev
**Massstab:** Spec `planning/260828-0649_o_spec-vorschau-rendert-pdf-als-betrachter.md` (C1 bis C5, Z1, Z2, A1 bis A10, sieben Constraints), Plan `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md` (elf Schritte, zehn Entscheidungen, Risikotabelle), CLAUDE.md, `rules/review-contract.md`.
**Gelesen:** alle 19 geaenderten Dateien der sieben Code-Commits (`1df8b8d`, `2aee690`, `22b8442`, `ae349d1`, `9d2e457`, `5ff1ee4`, `8a8e638`) als Diff gegen `2033626`, `betrachter.rs` vollstaendig am Arbeitsbaum; dazu die Nachbarstellen `anwendung.rs` (`ist_eigene_textflaeche`, `vorschau_fuellen`, `statuszeile_nachziehen`), `fenstermodell.rs` (`sichtbar_in`, `teilt_flaeche_mit`), `krk-core/src/zwischenablage.rs`, `krk-core/src/tasten/parser.rs` (`zeichen_der_stelle`), der Absturzbericht `analyses/260828-0912-…txt` und die Bugfix-Spur `history/260828-0917-…md`.
**Gemessen, nicht nur gelesen:** `cargo test --workspace` (exit 0, alle Probenziele gruen, 840 Proben allein in `krk-ui`), `cargo clippy --workspace --all-targets` (keine Warnung), `cargo fmt --all --check` (sauber). Die Verfuegbarkeitsangaben in `betrachter.rs` gegen `PDFView.h` und `PDFDocument.h` des installierten SDK gehalten (Zeilen 53, 134, 178, 194, 195, 199, 369, 375; 139, 164, 231, 238): alle stimmen. `Cargo.lock` traegt kein `cc` und ausser `windows-sys` kein `-sys`-Paket. `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` liefert zehn Rufer, keiner davon neu. `resources/default-keymap.toml` traegt 88 `[[funktion]]`-Bloecke und 93 Kombinationen, wie die Kopfzeile sagt; `Kommando::KENNUNGEN` hat 82 Eintraege, `Kommando` 82 Varianten. Kein Abnahmelauf: der verlangt KRK im Vordergrund.

## Summary

Die Runde ist sauber gebaut. Die sieben Aufzaehlungen, die sie anfasst oder anlegt (`Wirkungsbereich`, `Rang`, `Herkunftsart`, `Herkunft`, `Deutung`, `Zoom`, `Flaeche`, dazu `Inhalt`), werden ueberall vollstaendig und ohne Auffangzweig verzweigt; die drei Pflichtstellen der drei Kommandos stehen, der Ausfuehrungszweig steht eigens und ist von einer Probe gehalten; `copy:` geht durch die eine Huelle; der Deskriptorhaushalt bleibt bei `bis_zur_grenze_lesen`; jeder Rueckgabewert, dessen Fallenlassen unbemerkt bliebe, traegt `#[must_use]`. Der behobene Absturz (`8a8e638`) ist an der Wurzel behoben, und das Muster kann sich am jetzigen Stand nicht wiederholen (Abschnitt „objc2-Sicherheit"). Was bleibt, sind vier Befunde ohne Freigabewirkung: eine falsche Zahl in CLAUDE.md, ein wiederholtes Deuten beschaedigter Dateien, und zwei kleine Doppelungen quer durch den Baum.

## Totals

Critical 0 / High 0 / Medium 1 / Low 3.

## Findings by theme

### 1. Normative Flaeche: CLAUDE.md nennt fuer `Wirkungsbereich` sieben Werte (Medium)

`CLAUDE.md:81`: „`Wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) traegt sieben Werte". Seit `2aee690` sind es acht (`belegung.rs`, `Vorschau` zwischen `Navigator` und `Ueberall`; nachgezaehlt mit `awk '/^pub enum Wirkungsbereich/,/^}/'`). Der Plan hat das in seiner Risikotabelle vorausgesagt und dem Kurator zugewiesen; ohne Datensatz bleibt es dort aber unauffindbar, und die Datei hat dieselbe Sorte Zahl schon viermal in vier Tagen falsch getragen (`shared/issues/260812-2253_*`).

**Die zweite Meldung der Coder trifft nicht zu.** `CLAUDE.md:137` sagt „Es sind seit der Runde 14 zwei" eigene Textflaechen. `ist_eigene_textflaeche` (`anwendung.rs:2608-2620`) vergleicht weiterhin genau zwei Flaechen, `editor.textflaeche()` und `vorschau.textflaeche()`; der Betrachter ist dort nicht angemeldet, und das ist nach Constraint 6 des Specs richtig, weil `PDFView` keine der drei Textklassen von AppKit ist. Die Zahl zwei ist richtig und bleibt es. Ein Kurator, der beide Meldungen ungeprueft uebernimmt, macht die Datei an der zweiten Stelle falsch.

Datensatz: `issues/260828-1046_o_claude-md-nennt-sieben-werte-fuer-wirkungsbereich-der-baum-traegt-acht.md`. Fuer den Kurator, nicht fuer den Coder.

### 2. Der Merkposten haelt nur den Erfolg (Low)

`betrachter.rs:499-537`, `dokument_setzen`: bei `Beschaedigt` und `Gesperrt` kehrt die Funktion vor `*self.ivars().bytes.borrow_mut() = Some(…)` zurueck. Jeder weitere `anzeigen`-Durchlauf mit denselben Bytes (Tabwechsel hin und zurueck, `einziehen` waehrend eines Ladens, `kommando_ausfuehren` der vier Tabbefehle) reicht sie erneut an `PDFDocument::initWithData:` auf dem Hauptfaden. Bei einer abgeschnittenen Datei knapp unter 64 MB ist das ein wiederholter Leselauf ueber die Bytes, jedes Mal ohne Ergebnis. Kein Kriterium des Specs ist verletzt; der Doc-Kommentar (`:489-492`) beschreibt das Verhalten sogar. Fix: den Merkposten als `Option<(Arc<Vec<u8>>, Deutung)>` fuehren und bei `ptr_eq` die gemerkte Antwort liefern; `pdf_zeigen` verzweigt dann wie heute.

Datensatz: `issues/260828-1046_o_dokument-setzen-merkt-nur-den-erfolg-und-deutet-eine-beschaedigte-datei-bei-jedem-anzeigen-neu.md`.

### 3. Die Regel „nur http und https" steht zweimal (Low, cross-cutting)

`krk-core/src/zwischenablage.rs:65` entscheidet fuer den Sprung aus der Zwischenablage, dass allein `http` und `https` ein `Ziel::Web` ergeben; `betrachter.rs:638-640` (`ist_webschema`) trifft dieselbe Entscheidung fuer den Klick im PDF mit einer zweiten Funktion. Beide berufen sich auf C9 der Runde 1, und `zwischenablage::im_browser_oeffnen` (`krk-ui/src/appkit/zwischenablage.rs:281-284`) behauptet im Doc-Kommentar, „die Deutung im Kern" ziehe die Grenze, was seit dieser Runde nur noch fuer einen der zwei Rufer gilt. Wer C9 einmal lockert, aendert eine Stelle und vergisst die andere. Fix: die Schemaregel als `pub fn ist_webschema(&str) -> bool` in `krk_core::zwischenablage` herausziehen, beide Stellen rufen sie, der Doc-Kommentar an `im_browser_oeffnen` nennt beide Rufer.

Datensatz: `issues/260828-1046_o_die-regel-nur-http-und-https-steht-im-kern-und-im-betrachter-je-einmal.md`.

### 4. Der Variantenleser steht dreimal (Low, cross-cutting)

`varianten_der_aufzaehlung` (`crates/krk-core/tests/gemeinsam/mod.rs:411`), `varianten` (`betrachter.rs:662-675`) und der eingebettete Block in `jeder_wirkungsbereich_hat_einen_stellvertreter` (`kommandos/zulaessigkeit.rs:484-494`) lesen alle drei „`pub enum X {` bis `}`, ohne Kommentar- und Attributzeilen, bis zum Komma". Dass die Kernfassung `krk-ui` nicht erreicht, ist bekannt und im Kommentar erklaert; dass innerhalb von `krk-ui` zwei Abschriften nebeneinander stehen, ist neu mit dieser Runde und vermeidbar: `crate::quellbaum` ist genau das Modul, das die Proben der Kiste ueber den Quelltext teilen (`quelldateien` ruft es schon an beiden Stellen). Fix: `quellbaum::varianten(inhalt, name)` einmal, beide Proben rufen es; die Kernfassung bleibt, wie sie ist.

Datensatz: `issues/260828-1046_o_der-variantenleser-steht-in-krk-ui-zweimal-neben-der-kernfassung.md`.

## Was gehalten ist, mit Beleg

**objc2-Sicherheit in `betrachter.rs`.** Beide Klassen sind `MainThreadOnly`; jede `unsafe`-Stelle traegt ihre Bedingung. Der Rueckverweis auf das Vorschaufenster ist ein `Weak` (`:248`, `:459`), der Seitenmelder des Vorschaufensters haelt es ebenfalls schwach (`vorschau.rs`, `pdf_zeigen`), der Melder des Anwendungsdelegierten den Delegierten schwach (`anwendung.rs:1163`); kein Ring. Der Beobachter fuer `PDFViewPageChangedNotification` wird in `neu` mit `object: Some(&betrachter)` angemeldet und in `Drop` ohne Gegenstand abgemeldet (`:607-624`); `Drop` ruft keine ueberschriebene Methode. Der Delegierte lebt als `Retained` im ivar so lange wie die Ansicht; `PDFView.h:178` haelt ihn `weak`, also verschwindet der Zeiger mit dem Objekt.

**Das Rekursionsmuster ist geschlossen und nicht nur umgangen.** Die Ursache (`analyses/260828-0912`) war: `PDFView` beantwortet Selektoren, die wie Delegiertenmethoden heissen, und reicht darin an den Delegierten weiter, sobald der `respondsToSelector:` bejaht; eine Unterklasse als eigener Delegierter bejaht alles, was sie erbt. `Verweisdelegierter` ist eine `NSObject`-Unterklasse, die genau einen Selektor registriert (`PDFViewWillClickOnLink:withURL:`, `:292`). Fuer jeden weiteren Selektor des Protokolls, `PDFViewWillChangeScaleFactor:toScale:` (`PDFView.h:375`) eingeschlossen, antwortet sie mit nein, und `PDFView` reicht nicht weiter. Ein zweiter Selektor wuerde nur dann gefaehrlich, wenn jemand ihn am Betrachter statt am Delegierten anmeldete; der SAFETY-Block an `Pdfbetrachter` (`:323-335`) sagt es. Ich sehe keine Stelle im Baum, die das tut.

**Vollstaendigkeit ohne Auffangzweig.** `Wirkungsbereich::beschriftung`, `fokus::wirkt`, die zwei Tafeln (`fokus.rs`, `zulaessigkeit.rs`), `Rang::art`, `Rang::herkunft`, `Quellen::text`, `zeile`, `zeilentext`, `Vorschaufenster::flaeche_zeigen`, `fokusansicht`, `pdf_zeigen` ueber `Deutung`, `Pdfbetrachter::zoomen` ueber `Zoom`, `zeigt_dateitext`, `anzeigen` und `einzufaerben` ueber `Inhalt`: alle als `match` ohne `_ =>`. Die Probe `zoom_und_deutung_tragen_je_genau_drei_werte` zaehlt die Auffangzweige in `betrachter.rs` und findet null.

**Die drei Pflichtstellen und der Ausfuehrungszweig.** `wirkungsbereich` (`belegung.rs:1034-1036`), `KENNUNGEN` (`:854-859`), `bereich_des_kommandos` (`belegungsmodell.rs:320-323`), drei eigene Zweige in `kommando_ausfuehren` (`anwendung.rs:3439-3441`), gehalten von `zoomproben::die_drei_zoombefehle_haben_genau_hier_ihren_zweig` und `die_dateiliste_traegt_keinen_zoomzweig`.

**Die eine Huelle.** `NSPasteboard` steht als Codezeile allein in `zwischenablage.rs`; `vorschau.rs:302,536-537` und `abwurf.rs` nennen `NSPasteboardType` und `NSPasteboard` als Parametertyp der bestehenden Abfangstelle, wie vor der Runde. `betrachter.rs` nennt die Klasse nur im Kommentar; die Probe `nspasteboard_steht_nicht_im_betrachter_und_copy_genau_einmal` haelt es.

**Deskriptorhaushalt.** `laden` (`vorschaumodell.rs:822-838`) ruft `bis_zur_grenze_lesen(pfad, BILDGRENZE)`; die Rufer von `ohne_warten_oeffnen` sind dieselben zehn wie vor der Runde. Die Bytes gehen als `Arc<Vec<u8>>` ueber den Kanal, `PDFDocument::initWithData:` oeffnet nichts (Plan, Entscheidung 9).

**`#[must_use]`.** `dokument_setzen` (`betrachter.rs:498`), `Pdfbetrachter::zoomen` (`:546`), `Vorschaufenster::zoomen` (`vorschau.rs`). Die drei `let _ =` in `betrachter.rs` (`:300`, `:369`) sind begruendet: der Betrachter hat keine Statuszeile, und die Textanzeige meldet ihr Scheitern seit der Runde 14 ebenso wenig.

**Statuszeilen-Rangordnung (A5).** `Rang::ALLE` fuehrt `Seitenzaehler` zwischen `Filterstand` und `Markierungsstand`; die Proben `der_seitenzaehler_steht_zwischen_filterstand_und_markierungsstand`, `ein_stehender_filtertext_verdraengt_…`, `vorgang_befehlsantwort_und_fenstermeldung_stehen_ueber_dem_seitenzaehler` und `bei_ausgeblendeter_vorschau_bewirbt_sich_der_seitenzaehler_nicht` decken C4.4 bis C4.6. `sichtbar_in(…, Bereich::Vorschau)` ist dieselbe Funktion wie fuer die Dateifenster (`fenstermodell.rs:305`), und weil Vorschau und Editor sich die Flaeche teilen (`teilt_flaeche_mit`), zieht der Zaehler sich zurueck, sobald der Editor steht.

**Wiedereintritt.** `anzeigen` gibt die Ausleihe des Vorschaumodells vor dem ersten AppKit-Aufruf frei (`vorschau.rs:1254-1263`), ruft `seiten_melden` als letztes, und der Melder erreicht `statuszeile_nachziehen`, das allein `self.ivars().modell.borrow()` haelt. Die Rufer von `datei_anzeigen` und `zwischenablage_anzeigen` im Anwendungsdelegierten (`anwendung.rs:1742`, `1754`, `1767`) halten zu diesem Zeitpunkt keine Ausleihe. Feuert `PDFViewPageChangedNotification` schon in `setDocument:`, liest `seitenzaehler` die noch nicht umgestellte `Flaeche` und antwortet `None`; der Ruf am Ende von `anzeigen` holt den Zaehler nach.

**`zeichen_des_namens` und die US-Belegung.** Die eine Regel (`parser.rs:305-317`) hat drei Frager (`Taste::kennung`, `zeichen_als_kennung`, `menue::zeichen_der_taste`), und die alte Abschrift im Menue ist weg. `zeichen_der_stelle` (`:420-429`) laeuft ueber `Taste::zeichen` und liefert fuer die Zehnerblockcodes 69 und 78 damit `+` und `-`, also stimmt auch das synthetische Ereignis der Messstrecke. Die US-Haelfte von C3.2 bleibt an `decisions/260828-0712_o_…` haengen, wie der Plan es ausschreibt. Eine Beobachtung zur dortigen Moeglichkeit 1, nicht gemessen: das Menuekuerzel der drei Eintraege ist ueber `zeichen_der_taste` das Zeichen `+`, und der Abgriff reicht ein unbelegtes `shift+cmd+=` unveraendert an AppKit weiter; ob AppKit den Eintrag dann selbst ausloest, ist genau die Frage, die der Datensatz als `inference:` fuehrt.

**Untergrenzen.** `betrachter.rs` traegt den Abschnitt, 10.13 ist die hoechste Angabe, und die Zeilenangaben stimmen mit dem SDK ueberein (siehe oben). `vorschau.rs`, `mod.rs` und `statuszeile.rs` sprechen keine neue Klasse an.

**Cargo.** `objc2-pdf-kit` ohne Vorgabemerkmale mit Begruendung in der Wurzel-`Cargo.toml`; `Cargo.lock` bleibt frei von `cc` und `-sys` ausser `windows-sys`.

## Cross-cutting observations

- **Zwei Abschriften derselben Grenze (Befund 3) und zwei Abschriften desselben Lesers (Befund 4)** sind derselbe Fehlertyp: eine Regel, die in `krk-core` wohnt oder wohnen koennte, wird in `krk-ui` nachgebaut, weil die Kiste kein Bibliotheksziel hat oder weil der Rufer die Kernfassung nicht kannte. Fuer den Leser gibt es innerhalb von `krk-ui` schon den Sammelplatz `quellbaum.rs`; fuer die Schemaregel den Kern.
- **Das Deuten auf dem Hauptfaden** (`dokument_setzen`, bis 64 MB) liegt in derselben ungemessenen Endbedingung wie alles an der Vorschau seit der Runde 16; die offene Frage `circles/260823-2208-…/decisions/260824-1900_*` bindet und ist hier nicht zu beantworten. Befund 2 macht den Preis fuer beschaedigte Dateien nur unnoetig wiederkehrend.
- **Der geerbte Auffrischungsdefekt** (`shared/issues/260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-…`) wird von dieser Runde weder kleiner noch groesser: nach einem Neulesen sind es andere Bytes, `Arc::ptr_eq` greift nicht, und das Dokument wird neu gedeutet. Der Plan sagt es in seiner Risikotabelle.
- **Nicht pruefbar ohne Buendel, und vom Abnahmelauf nicht ausdruecklich genannt:** ob `PDFView` bei `fokus_vorschau` (`shift+cmd+y`) den Ersthelferrang annimmt (`fokusansicht` liefert seit dieser Runde den Betrachter). C5.6 prueft den Klick; der Befehlsweg steht in keiner Zeile von Schritt 11. Kein Befund, eine Luecke im Nachweis.

## Recommended sequencing

Kein Freigabeblocker. Befund 1 geht an den Kurator am Tor von `/fusion:cleanup` und nicht an den Coder. Die Befunde 2 bis 4 sind Aufraeumarbeit fuer eine Behebungssitzung; keine davon aendert Verhalten, das der Abnahmelauf geprueft hat, und keine muss vor dem Abschluss der Runde erledigt sein.
