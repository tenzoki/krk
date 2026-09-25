# S18: Lesezeichen- und Geräteleiste, und der Wirkungsbereich je Kommando (C5)

**Datum:** 2026-08-05, 17:30
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`

## Auftrag

Schritt 18 des Plans, der erste der Phase E: die Leiste links, in die S12 einen leeren Bereich der `NSSplitView` gestellt hat. Oben die Lesezeichen aus `bookmarks.toml`, unten das Benutzerverzeichnis und die eingehängten Datenträger. Anlegen, Umbenennen, Löschen, Reihenfolge und Fokuswechsel über die Tastatur, acht Abnahmekriterien aus C5.

Der teurere Teil war nicht die Leiste, sondern der **Wirkungsbereich**: mit einem zweiten fokussierbaren Bereich wird die Frage "darf dieser Befehl hier wirken" für jedes Kommando fällig, und der Plan verlangt dafür eine Eigenschaft im Kern statt vier oder fünf Abfragen an vier oder fünf Aufrufstellen.

## Was entstanden ist

**Drei neue Dateien.** `crates/krk-ui/src/leistenmodell.rs` hält Inhalt, Reihenfolge und Auswahl der Leiste, ohne eine `objc2`-Zeile. `crates/krk-ui/src/appkit/leiste.rs` hält die `NSTableView` im Seitenleistenstil samt Datenquelle und Delegiertem. `crates/krk-ui/src/kommandos/fokus.rs` hält die eine Regel, die aus Wirkungsbereich und Fokus ein Ja oder Nein macht.

**Der Wirkungsbereich, im Kern.** `Kommando::wirkungsbereich` in `crates/krk-core/src/tasten/belegung.rs` ist eine **vollständige Fallunterscheidung ohne Auffangzweig**. Damit erzwingt der Übersetzer, was das Abnahmekriterium verlangt: ein neues Kommando übersetzt nicht, bevor es seinen Bereich genannt hat, und mehr als einen kann keines tragen. Ein Auffangzweig hätte einem vergessenen Kommando still den Bereich des Nachbarn gegeben.

Drei Werte, `Dateifenster`, `Leiste`, `Ueberall`, und die Grenze zwischen ihnen ist die Frage, wer den Befehl ausführt: was das Fenstermodell trägt, wirkt überall; was ein Dateifenster trägt, braucht dessen Fokus; was die Leiste trägt, den ihren. Drei Befehle folgen der Regel nicht und stehen deshalb ausgeschrieben: `auswahl_hoch` und `auswahl_runter` bewegen die Auswahl des Bereichs vor dem Nutzer und gehören keinem allein, und `lesezeichen_anlegen` liest den Ordner des aktiven Dateifensters und schreibt in die Leiste, braucht also keinen von beiden im Fokus.

**Die Abfrage aus S16 ist aufgegangen.** `loeschtaste_wirkt` und der Typ `Fokus` sind aus `kommandos/operationen.rs` verschwunden, ebenso die beiden Aufrufe in `in_den_papierkorb` und `endgueltig_loeschen`. `grep -rn "self\.fokus()" crates/krk-ui/src` liefert genau eine Zeile, `anwendung.rs:1039`.

**Sieben neue Kommandos.** Die Kennungen standen seit S9 in `resources/default-keymap.toml` und trugen bis heute kein Kommando: `lesezeichen_anlegen`, `lesezeichen_umbenennen`, `lesezeichen_loeschen`, `lesezeichen_hoch`, `lesezeichen_runter`, `fokus_leiste`, `fokus_dateifenster`. `Kommando::KENNUNGEN` wächst von 42 auf 49.

**Die Lesezeichen in `ablage/lesezeichen.rs`.** Anlegen, Umbenennen, Löschen, Verschieben und die Gültigkeitsprüfung sind in die Datei aus S10 hineingewachsen; eine zweite Lesezeichendatei ist nicht entstanden. Dazu eine eigene Namensregel: ein Lesezeichenname ist eine Beschriftung und kein Eintrag im Dateisystem, "Projekte/2026" ist zulässig, leer nicht.

**Die Aufzählung bei der Beobachtung.** `volumes::eingehaengte` steht neben der `NSWorkspace`-Beobachtung aus S14, und ein Modul beantwortet die ganze Frage "welche Datenträger gibt es gerade". Das Benutzerverzeichnis kommt nicht von dort, weil es kein Datenträger ist; zusammengesetzt wird beides in `anwendung::orte`.

## Vier Entscheidungen, die der Plan offen ließ

**Der Fokus wird von AppKit gelesen und nicht selbst geführt.** Der Plan nennt den Fokuswechsel "Zustand des Fenstermodells". Umgesetzt ist er anders: `Anwendungsdelegierter::fokus` liest den Ersthelfer des Fensters, und die beiden Fokusbefehle setzen ihn über `makeFirstResponder:`. Der Grund ist derselbe, aus dem der Plan an anderen Stellen zweite Bestände vermeidet: ein Kennzeichen im Fenstermodell wäre eine zweite Wahrheit, die jeder Mausklick in eine der drei Listen nachzuziehen hätte, und die erste Abweichung zwischen beiden fände keine Prüfung. AppKit weiß es ohnehin, und der Fokusvorbehalt für Textfelder aus S13 liest schon heute denselben Ersthelfer.

**Der Fokuswert wird einmal erhoben und zweimal gebraucht.** Zuerst als Vorbehalt, dann als **Adresse**: was weder dem Fenster als ganzem gehört noch abgewiesen ist, geht an den Bereich, der den Fokus hat. Ohne eine Adresse gäbe es keinen Ort, an den der Auf- und der Ab-Pfeil zu richten wären, denn beide Bereiche sind Listen mit einer Auswahl. Der Diff zeigt trotzdem genau eine Stelle, die **vor dem Ausführen** nach dem Fokus fragt; die Adresse ist keine zweite Abfrage, sondern derselbe Wert in seiner zweiten Rolle. Der Unterschied ist an `bereichskommando` ausgeschrieben.

**Die Auswahl ist der Befehl.** C5 sagt "Die Auswahl eines Eintrags setzt den Ordner des aktiven Dateifensters", also gibt es keinen zweiten Tastendruck zum Öffnen: jede Bewegung der Auswahl navigiert, mit der Maus wie mit der Tastatur, über dieselbe Senke.

**Ein Blatt für zwei Namen.** `blaetter/namenseingabe.rs` hat mit `frei_zeigen` eine zweite Einstiegsstelle bekommen, die den Namen nicht gegen die Regeln des Dateisystems prüft und eine Vorbelegung annimmt; `zeigen` aus C4 läuft jetzt darüber und legt seine Prüfung darum. Zwei Eingabeblätter für einen Namen wären zwei Erscheinungsbilder und zwei Tastaturbedienungen für dieselbe Frage gewesen.

## Was am laufenden Bündel geprüft ist

Alle acht Abnahmekriterien aus C5, einzeln, mit `hdiutil`-Abbild und Bildschirmaufnahmen. Die Einzelheiten stehen im Bericht an den Nutzer. Dazu die vier Abnahmekommandos (`make check`), `cargo test -p krk-core` mit der neuen Prüfung auf genau einen Wirkungsbereich je Kommando, und die drei `grep`-Zusagen: eine Fokusabfrage, keine `objc2`-Zeile außerhalb von `appkit/`, zwei `unsafe`-Ausnahmen.

**Ein Defekt fiel dabei auf und ist behoben.** Wechselt ein Lesezeichen seine Gültigkeit, während der Nutzer in der Leiste steht, zeichnet die Leiste neu, und `reloadData` nimmt der `NSTableView` ihre Auswahl, während das Modell seine behält. Die blaue Zeile verschwand unter der Hand des Nutzers, und der nächste Pfeil sprang scheinbar aus dem Nichts weiter. `gueltigkeit_nachziehen` setzt die Auswahl seither wieder.

**Die Sonde ist zurückgenommen.** Der Prüfordner, das Abbild, das Hilfsskript, die Bildschirmaufnahmen und die beim Prüfen angelegte `bookmarks.toml` sind gelöscht; vor dieser Sitzung gab es keine. Der Ablageordner enthält wieder allein `session.toml`.

## Angelegte Datensätze

- `issues/260805-1730_o_die-gueltigkeit-eines-lesezeichens-veraltet-zwischen-zwei-anlaessen.md` — die Gültigkeitsmarke wird an drei Anlässen geprüft, nicht laufend. Die Zusage aus C5 hält; die Marke kann zwischendurch falsch stehen.
- `decisions/260805-1730_o_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md` — `shift+cmd+l` weist heute stumm ab, wenn die Leiste ausgeblendet ist. Drei Möglichkeiten, Empfehlung: einblenden, nach dem Vorbild von Shift+F3 aus C10.

## Berührte Dateien

Neu: `crates/krk-ui/src/leistenmodell.rs`, `crates/krk-ui/src/appkit/leiste.rs`, `crates/krk-ui/src/kommandos/fokus.rs`.

Geändert: `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/src/tasten/mod.rs`, `crates/krk-core/src/ablage/lesezeichen.rs`, `crates/krk-core/src/ablage/mod.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-ui/src/main.rs`, `crates/krk-ui/src/kommandos/mod.rs`, `crates/krk-ui/src/kommandos/operationen.rs`, `crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/aufteilung.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/appkit/volumes.rs`, `crates/krk-ui/src/appkit/blaetter/namenseingabe.rs`.

Drei davon nennt die Dateiliste des Plans nicht: die neue `kommandos/fokus.rs` mit ihrer Einbindung in `kommandos/mod.rs`, `appkit/blaetter/namenseingabe.rs` und `appkit/tabelle.rs`. Die Begründungen stehen im Bericht an den Nutzer.
