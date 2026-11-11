# Durchsicht: Kopieren, Singlestrip und Format vor einer Protokolldatei, vor der Auslieferung

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `52ad05b5..b000bc71`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Urteil:** revise

Die eine nicht geöffnete Datei ist das Ereignisprotokoll der Werkbank und trägt keinen Code; die zwei Steuerdateien des Bereichs sind gelesen (der Defekt zur Zeitprobe steht offen, der Plan in Arbeit mit den Schritten 1 bis 7 erledigt und Schritt 8 offen). Jede geänderte Datei außerhalb der Werkbank ist geöffnet, die meisten als Diff des Bereichs, `appkit/knopfzeile.rs` ganz. Die Regeldatei `.claude/rules/coding-was-man-nicht-sieht.md` ist in ihren vier geänderten Absätzen gelesen und nicht in voller Länge.

## Zusammenfassung

Der Umbau hält, was der Plan zusagt: kein Weg verdirbt Text am Schnitt zwischen UTF-16 und Bytes, die Geltung nimmt in keiner Lage den falschen Zweig, und die Reihe folgt Datei und Form. Zwei Befunde betreffen, was die Umformung selbst aus dem Text einer Protokolldatei macht. Keiner verliert Daten auf der Platte, keiner stürzt ab, keiner wirkt an einer fremden Datei.

`make check` lief am Stand `b000bc71` grün durch („alle fuenf gruen“, Exit 0).

## Zahlen je Schwere

- Critical: keiner
- High: keiner
- Medium: einer
- Low: einer

## Befunde

### Was die Umformung aus dem Text macht

**M1, Medium. Singlestrip ohne Auswahl leert ein Protokoll, das mit einer Markdown-Überschrift beginnt, und meldet nichts.**
Datensatz: `261112-0002-singlestrip-ohne-auswahl-leert-ein-protokoll-das-mit-einer-markdown-ueberschrift-beginnt-und-meldet-nichts.md`.

- `crates/krk-ui/src/appkit/editor.rs`, `protokoll_umgeformt`: ohne Auswahl ist der Ausschnitt der ganze Stand.
- `crates/krk-core/src/text/glaetten.rs`, `befehl`: ein `#` am Wortanfang ist ein Kommentar, und der Rumpf verlässt die Schleife mit `break`. Aus `# Treffen\n\nText\n` wird der leere Text.
- `protokoll_umformen` antwortet nach einem Umbau `None`, die Statuszeile bleibt still.
- `HowTo.md` verweist unter `### Protokolle in minutes/` auf den Abschnitt der Quicknote und nennt die Folge für eine Überschrift nicht.

Gelesen am Quelltext, nicht im laufenden Bündel ausgeführt. `cmd+z` nimmt es zurück, und die Datei ändert sich erst mit `cmd+s`. Die Wirkung ist die wörtliche Übertragung aus der Quicknote, die der Auftrag verlangt; dort steht aber ein kopierter Befehl und kein Markdown. Drei Richtungen stehen im Datensatz, die billigste ist ein Satz in `HowTo.md`.

**L1, Low. Format und Singlestrip stellen eine CRLF-CSV in `minutes/` auf LF, auch wenn sonst nichts zu ändern ist.**
Datensatz: `261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md`.

- `glaetten::zerlegen` wandelt `\r\n` und `\r` in `\n`.
- `umgeformt` vergleicht `nachher != vorher`, also gilt ein reiner Zeilenendewechsel als Änderung.
- `datei::in_stand_form` lässt den Stand einer CSV unverändert, und `sichern` schreibt ihn Byte für Byte.

Betroffen ist allein die Rohansicht einer `.csv` oder `.tsv` mit `\r\n` in `minutes/`. Die Zusage dagegen steht in `261019-1122-gilt-der-stand-mit-den-zeilenenden-der-datei-als-antwort-auf-die-sicherungsform-einer-csv.md`.

## Geprüft und ohne Befund

**Schnitt zwischen UTF-16 und Bytes.** `protokoll_umgeformt` schneidet beide Enden über `koordinaten::in_bytes`. Die Funktion antwortet für eine Stelle in einem Ersatzzeichenpaar mit dem Anfang des Zeichens und für eine Stelle hinter dem Text mit `text.len()`; beide Schnitte liegen auf Zeichengrenzen, und `stand[anfang..ende]` kann nicht abbrechen. `location + length` ist mit `saturating_add` gebildet. Die Probe `die_umformung_am_protokoll_rechnet_an_auswahl_oder_ganzem_stand` deckt ein Zeichen außerhalb der BMP vor der Auswahl, einen Bereich hinter dem Textende und einen Beginn mitten im Paar.

**Ein Schreibweg.** `protokoll_umformen` ruft allein `umbau_anwenden`. Der Umkehrpunkt entsteht dort gegen den Stand, den das Modell danach hält, auch wenn `Editormodell::wandelt` zugreift. Die Auswahl danach ist gegen die Länge des neuen Standes geklemmt, und `stelle_zeigen` rechnet über `koordinaten::in_utf16`. Die Quelltextprobe `das_protokoll_wird_ueber_den_umbauweg_umgeformt_und_die_quicknote_ueber_ihren` hält, dass weder `stand_einsetzen` noch `stand_erneuern` noch `quicknote.ersetzen` im Rumpf steht.

**Fläche und Stand.** Die Formatansicht ändert Merkmale und keine Zeichen (`Ansicht::Format`), die Textfläche trägt also in beiden Ansichten den Stand. `protokolldatei`, `typ` und `pfad` setzt das Modell an einer Stelle, im Erfolgszweig von `uebernehmen`, und `schliessen` nimmt alle drei zurück; während eines laufenden Ladens gelten Kennzeichen, Stand und Fläche gemeinsam der alten Datei.

**Geltung.** `datei_passt` sagt für `Notiztext` ja in `Editorform::Quicknote` und sonst nach `protokolltext`; `form_passt` sagt in jeder Tabellenform nein. `notiztext_kopieren` und `notiztext_umformen` fragen `quicknote_offen()`, dieselbe Quelle, aus der `form()` `Editorform::Quicknote` ableitet, die zwei können also nicht auseinanderlaufen. `quicknote_leeren` bleibt bei `Wirkungsbereich::Quicknote`. Die Probe `die_drei_notiztextbefehle_wirken_in_der_quicknote_und_vor_dem_text_einer_protokolldatei` schreibt die Erwartung aus, ohne `protokolltext` zu rufen, und läuft über jede Lage.

**Die Reihe.** `EditorIvars::protokollknoepfe` hält die `Knopfzeile` so lange wie der Bereich ihre Ansicht; der Melder hält den Editorbereich schwach, die Reihe hält die Textfläche stark, und kein Glied davon hält den Editorbereich. `protokollknoepfe_nachfuehren` steht vor der Rückkehr bei leerer Schrittliste. `flaechenrahmen()` rechnet wie der Bau (`AUFBAUGROESSE` minus `statuszeile::HOEHE`), und `textrolle.setFrame` steht im Betriebscode an dieser einen Stelle. In der Quicknote liegt jede Schaltfläche an derselben Stelle wie vorher: die Reihe steht bei `rahmen.size.height - KNOPFZEILE`, die Schaltfläche darin bei `(KNOPFZEILE - hoehe) / 2`.

**Projektregeln.** `NSPasteboard` nennt `knopfzeile.rs` nicht, und `protokoll_kopieren` schreibt über `zwischenablage::text_schreiben`. Jeder neue nutzersichtbare Text steht in `de.rs`, `fr.rs` und `en.rs`. `knopfzeile.rs` trägt den Untergrenzen-Abschnitt mit jedem hereingeholten Namen. Die Längen von `Text::ALLE`, `Zahlwort::ALLE`, `STELLVERTRETER`, `BESCHRIFTUNGEN` und der Tafeln sind nachgezogen. `unsafe` steht allein unter `appkit/`.

**Anleitung und Regeldatei.** Die Sätze in `HowTo.md` und die vier geänderten Absätze der Regeldatei stimmen mit dem Baum überein, bis auf die Lücke aus M1.

## Was diese Durchsicht nicht entscheiden kann

- **Rückgängig nach Tippen.** `Quicknote::ersetzen` ruft vor der Ersetzung `breakUndoCoalescing`, `umbau_anwenden` ruft es nicht (`grep -n breakUndoCoalescing crates/krk-ui/src/appkit/*.rs` nennt allein `quicknote.rs`). Ob ein Klick auf „Format“ unmittelbar nach getipptem Text und zwei `cmd+z` danach den Text sauber zurückbringen, zeigt allein das laufende Bündel. Der Weg ist älter als dieser Bereich (`treffer_ersetzen` geht ihn ebenso); als Defekt ist nichts abgelegt, weil nichts belegt ist. Speculation: die Tippgruppe der Fläche könnte über den Umbau hinweg weiterlaufen.
- **Eine Verknüpfung auf `secrets.txt` in `minutes/`.** Der Doc-Kommentar von `protokoll_kopieren` sagt, eine Protokolldatei sei „nie `secrets.txt`“. Über einen harten oder symbolischen Verweis `minutes/secrets.txt` gilt das nicht, weil `sonderdatei_genau` Gerät und Inode vergleicht. Inference: die Rohansicht trüge dann die drei Schaltflächen. Die Folge ist die Zwischenablage, die `260926-0033_*_darf-text-aus-secrets-txt-in-die-zwischenablage.md` erlaubt; abgelegt ist nichts.
- **Klick, Lage der Reihe im Fenster, Fokus nach dem Klick.** Die Hauptfadenproben laufen ohne Fenster; der Plan nennt das unter „Where this work stops“ als Nutzerarbeit.

## Übergreifendes

Beide Befunde haben eine Wurzel: `glaetten` ist für einen aus dem Terminal kopierten Text geschrieben und trifft vor einem Protokoll auf Dateien mit eigener Grammatik (Markdown, CSV mit Zeilenenden). Die Erkennung der Protokolldatei zählt jede Endung, die Umformung kennt keine.

## Reihenfolge

1. Vor der Auslieferung, wenn der Nutzer es so will: M1, mindestens der Satz in `HowTo.md`, weil die Anleitung im Releasepaket mitreist.
2. Danach: L1.

Keiner der zwei Befunde hält die Auslieferung nach den genannten Maßstäben auf (Datenverlust, Absturz, falsche Wirkung an fremder Datei).
