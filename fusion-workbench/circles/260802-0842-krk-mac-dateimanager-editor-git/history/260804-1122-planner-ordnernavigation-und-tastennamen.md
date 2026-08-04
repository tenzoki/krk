# Planner: Ordnernavigation neu belegt, Kombinationsschreibweise erweitert

**Datum:** 260804-1122
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`

## Auftrag

Der Nutzer hat am 260804 drei Belegungen bestimmt: den Aufstieg in den übergeordneten Ordner auf `cmd+left` neben dem vorhandenen `cmd+up`, den Einstieg allein auf `cmd+right`, und die Belegungsansicht auf `f1`. Zwei der drei Kombinationen ließen sich in der Kombinationsschreibweise nicht einmal hinschreiben, weil die Tastentabelle des Parsers die Links- und Rechts-Pfeile und `f1` nicht kennt. Spec und Plan waren nachzuziehen, `resources/default-keymap.toml` ausdrücklich nicht: die Datei gehört dem `ontocoder`.

## Was entschieden wurde

**Der Umfang der Erweiterung: acht Namen, nach der Regel "ganze Tastengruppen".** Gebraucht sind drei, aufgenommen werden `left`, `right`, `f1`, `f2` und `f9` bis `f12`. Zwei Gruppen sind betroffen, der Pfeilblock und die Funktionstastenreihe F1 bis F12, und beide werden geschlossen. Die Regel trägt sich aus zwei Richtungen. Ein Name kostet eine Zeile in einer konstanten Tabelle und ist keine Belegung; er löst nichts aus, solange keine Tastenliste ihn nennt. Eine Lücke dagegen kostet mehr als eine fehlende Vorbelegung: die Belegungsansicht aus C3 weist eine Kombination zu, indem der Nutzer sie drückt, und `Kombination::aus_tastendruck` liefert für eine Taste ohne Namen `None`. Jede fehlende Taste ist damit eine, die der Nutzer nicht belegen kann, was der Grundhaltung aus C3 widerspricht. Dass eine halbe Gruppe teuer ist, hat dieses Projekt bereits bezahlt: genau der Zustand "Pfeile hoch und runter ja, links und rechts nein" hat die Bereichsbreiten aus C7 auf `ctrl+b` und `ctrl+s` gedrängt.

**Die Satzzeichen bleiben draußen.** Ein virtueller Tastencode benennt eine Stelle auf der Tastatur, und bei den Satzzeichen läuft die Beschriftung dieser Stelle je nach Tastaturbelegung weit auseinander: `kVK_ANSI_LeftBracket` trägt auf einer deutschen Tastatur ein `ü`, `kVK_ANSI_Quote` ein `ä`, `kVK_ANSI_Slash` ein `-`. Dieselbe Abweichung hat das Projekt für die Buchstaben einmal sehenden Auges hingenommen, bei `cmd+y` unter der Taste Z, und nur deshalb, weil mit F3 ein zweiter Weg danebenliegt. Für elf Satzzeichen ohne zweiten Weg trägt das Argument nicht. Die Reihe endet aus demselben Grund bei F12: F13 aufwärts gibt es allein auf der Tastatur mit Zehnerblock.

**Die Erweiterung des Parsers ist ein eigener Schritt und gehört nicht in die Dateiliste von S13.** Ausschlaggebend ist eine Reihenfolge, keine Zuständigkeit. Die Auslieferungsbelegung ist über `include_str!` einkompiliert und wird beim ersten Zugriff gelesen; stünde `cmd+left` in der Datei, bevor der Parser `left` kennt, scheiterte die Prüfung `cargo test -p krk-core --test belegung`, und die Anwendung bräche beim Start ab. Die Codeänderung muss also vor der Datenänderung liegen. In S13 gefaltet käme sie hinter S12 zu stehen, und die Datenänderung, die der Nutzer unmittelbar nachziehen lassen will, hinge daran. Der Plan trägt deshalb **S11b** (`coder`, Parser) und **S11c** (`ontocoder`, Belegungsdatei) mit der Kante S11b nach S11c. Beide setzen allein den abgenommenen S11 voraus und sind sofort lauffähig.

**Das frei gewordene `return` fällt im Dateifenster auf die Sprungmarke durch und bleibt dort wirkungslos.** Der Weg ist mechanisch nachgelesen: `Belegung::nachschlag` findet keine Funktion, die Maske ist leer, also antwortet `Nachschlag::Sprungmarke`, und `behandeln` reicht den Tastendruck weiter, statt ihn zu schlucken. Wirkungslos ist er unter einer Vorschrift, die S13 ohnehin braucht: die Sprungmarke nimmt nur Zeichen auf, die ein Dateiname tragen kann. Ohne sie schöbe `return` ein Wagenrücklaufzeichen in den Suchpuffer, und dasselbe gälte für jede unbelegte Funktionstaste. Die Vorschrift ist damit keine Sonderregel für `return`, sondern die Regel, die die Sprungmarke braucht. Ein Eintrag `return` mit leerer Tastenliste entsteht nicht; freie Tasten führt die Belegungsdatei nicht auf, so wie sie `shift+delete`, `cmd+c` und `cmd+v` nicht aufführt.

**C3 braucht eine Ergänzung für F1, und sie ist klein.** Die vorhandene Formulierung deckt F1 nicht ab, weil zwei Sätze wörtlich auf F3 bis F8 lauten: die Beschriftungsregel der Belegungsansicht und die Festlegung in den Randbedingungen, was ein Funktionstastenname in diesem Dokument bezeichnet. Beide sind auf die Funktionstasten verallgemeinert, die Beschriftungsregel auf F1 bis F12. Dazu kommt ein Absatz, der den Befund vom 260802 auf F1 überträgt: KRK belegt den Tastencode und nicht die Fingerhaltung, also sind Fn+F1 und ein nacktes F1 nicht unterscheidbar, und `kVK_F1` mit 122 steht als **dokumentiert**, nicht als gemessen. Gemessen bleiben genau drei Funktionstasten.

## Nachweise

- Die acht Tastencodes sind am 260804-1122 aus `kVK_*` in `HIToolbox.framework/Headers/Events.h` des macOS-SDK gelesen, derselben Quelle, aus der S11 die vorhandenen bezogen hat: F1 122, F2 120, F9 101, F10 109, F11 103, F12 111, Links 123, Rechts 124. Die Werte für Hoch (126) und Runter (125) aus der vorhandenen Tabelle stimmen mit derselben Quelle überein.
- Die Konfliktfreiheit ist geprüft, nicht angenommen: `resources/default-keymap.toml` führt 55 Kombinationen, keine doppelt, und `cmd+left`, `cmd+right` und `f1` kommen in keiner Tastenliste vor.
- Keine der acht neuen Zahlen kollidiert mit einem Code der vorhandenen Tabelle; die Prüfung `jeder_name_und_jeder_code_steht_genau_einmal` deckt das nach S11b maschinell ab.
- Der Abhängigkeitsgraph der Schritte steht nach dem Nachzug auf 28 Knoten und 42 Kanten, zyklenfrei, kein Knoten ohne Kante, ohne ausgehende Kante allein S23 und S6b. Nachgerechnet, nicht geschätzt.

## Geänderte Dateien

- `planning/260802-1036_o_spec-navigator-geruest.md` — Kopfvermerk, zwei Abnahmekriterien und zwei Festlegungen in C2, ein Absatz und drei Abnahmekriterien in C3, die Beschriftungsregel, zwei Festlegungen in C3, zwei Absätze in den Randbedingungen und ein Absatz unter den offenen Nutzerentscheidungen.
- `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` — Kopfvermerk, Directive, zwei neue Knoten und vier neue Kanten im Abhängigkeitsgraphen samt Begründung, die neuen Schritte S11b und S11c, Nachträge an S9, S13 und S20, drei Einträge im Register der Defekte und Entscheidungen, Diagramm-Selbstprüfung nachgerechnet.
- `issues/260803-2045_c_die-kombinationsschreibweise-kennt-die-links-und-rechts-pfeile-nicht.md` — geschlossen, mit ausgeschriebener Auflösung aller drei Punkte.
- `decisions/260804-1122_o_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md` — neu, offen.
- `issues/260804-1122_o_der-fokusvorbehalt-fuer-tastenbefehle-steht-nur-fuer-die-loeschtasten.md` — neu, offen.

Nicht angefasst: `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`. Kein `[DONE]`-Vermerk und keine der zehn Zahlen aus C8 sind berührt. Nicht committet.

## Was als nächstes ansteht

S11b und S11c, in dieser Reihenfolge. Der `coder` trägt acht Namen in `crates/krk-core/src/tasten/parser.rs` nach und wechselt in einer vorhandenen Prüfung das Beispiel, weil `cmd+left` dort heute als unbekannter Tastenname dient. Der `ontocoder` trägt danach die drei Belegungen und vier Stellen im Kopfkommentar der Belegungsdatei nach.
