# Shaper: Klärungsrunde vor dem Spec der Belegungsausgabe

**Datum:** 2026-08-11, 04:46
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Status:** Unterbrochen, wartet auf sieben Nutzerantworten
**Agent:** shaper, als Unteragent dispatcht

## Warum diese Sitzung ohne Spec endet

Der Auftrag verlangt, den Nutzer über `AskUserQuestion` einzubeziehen. Der Dispatch hat
dieses Werkzeug nicht mitgegeben; die Werkzeugliste dieses Laufs führt es nicht, und eine
Suche nach ihm unter den nachladbaren Werkzeugen bleibt leer. Die Regel des Shapers für
diesen Fall ist eindeutig: als Unteragent gibt er seine gebündelten Fragen an den
Orchestrator zurück und hält an, statt ein Werkzeug zu behaupten, das er nicht hat.

Diese Datei hält fest, was am Code geprüft ist, damit der erneute Dispatch nicht bei null
anfängt. Unteragenten teilen kein Gedächtnis.

## Was am Code geprüft ist

Alles in diesem Abschnitt ist gelesen, nicht geschlossen. Die Pfade sind Projektpfade
relativ zur Projektwurzel.

**Die Belegungsansicht trägt zwei Spalten, Funktion und Belegung** (`crates/krk-ui/src/appkit/belegungsansicht.rs`,
Modulkopf Zeile 4). Die Ausgabe bekommt nach der Nutzerantwort vom 260811-0110 eine dritte.

**Die neun Funktionsbereiche stehen mit ihren Überschriften an einer Stelle.**
`Funktionsbereich::ALLE` und `Funktionsbereich::name()` in `crates/krk-ui/src/belegungsmodell.rs:104`
und `:117` liefern in dieser Reihenfolge: Dateilisting, Dateioperationen, Tabs, Vorschau,
Leiste und Fokus, Fenster, Anwendung, Textbefehle, Editor. Die Zuordnung Funktion zu Bereich
macht `bereich()` (`:146`), für Funktionen mit Kommando über die vollständige
Fallunterscheidung `bereich_des_kommandos`, für die sechs Textbefehle über ihre Kennung.

**Die sieben Wirkungsbereiche haben keine Beschriftungen.** `enum Wirkungsbereich`
(`crates/krk-core/src/tasten/belegung.rs:171`) führt `Dateifenster`, `Leiste`, `Vorschau`,
`Editor`, `Tabbereich`, `Navigator`, `Ueberall`. Es gibt keinen `impl`-Block mit einer
`name`-Funktion, wie ihn `Funktionsbereich` hat. Zwei der sieben Namen sind ohne Erklärung
unverständlich: `Tabbereich` meint Dateifenster oder Vorschau, `Navigator` meint
Dateifenster, Leiste und Vorschau, ausdrücklich nicht den Editor.

**Die sechs vom Menü zugestellten Textbefehle tragen Tasten und kein Kommando.**
`resources/default-keymap.toml:647-691` führt `text_ausschneiden`, `text_kopieren`,
`text_einfuegen`, `text_alles_auswaehlen`, `text_rueckgaengig`, `text_wiederholen`, alle mit
`gehalten_von = "menue"` und alle mit einer Kombination. Sie sind damit belegt und kommen
nach der Umfangsantwort in die Ausgabe. Einen Wirkungsbereich haben sie nicht.

**Die Beschriftung einer Kombination hat eine Quelle.** `anzeige()`
(`crates/krk-ui/src/belegungsmodell.rs:527`) setzt auf die `Display`-Form nur große
Teilanfänge: `shift+cmd+k` wird `Shift+Cmd+K`, `f3` wird `F3`.

**Die Statuszeile trägt fünf Ränge, der oberste ist die Befehlsantwort.**
`crates/krk-ui/src/appkit/statuszeile.rs:72-86`. Sie steht am Fuß eines Dateifensters, also
zweimal im Fenster. Gesetzt wird sie über `Anwendungsdelegierter::antwort_zeigen(seite, text)`
(`crates/krk-ui/src/appkit/anwendung.rs:3296`), gelöscht an **beiden** Dateifenstern zu Beginn
der nächsten Kommandoausführung (`anwendung.rs:1987-1996`). Eine Uhr hängt nicht daran: die
Antwort gilt bis zum nächsten Befehl.

**Ein Menüeintrag ohne Kürzel ist gebaute Form.** `ohne_kuerzel`
(`crates/krk-ui/src/appkit/menue.rs:335`). Das Hauptmenü hat heute drei Untermenüs, KRK,
Bearbeiten und Fenster (`menue.rs:195-277`). `befehl()` (`:310`) schlägt das Kürzel unter
einer Kennung in der Belegung nach und weicht bei unbekannter Kennung mit einer Meldung auf
`ohne_kuerzel` aus; für einen Eintrag, der bewusst keine Kennung hat, ist der direkte Aufruf
von `ohne_kuerzel` der richtige Weg und nicht der Ausweichzweig.

**Das Benutzerverzeichnis wird an einer Stelle aufgelöst.**
`pfade::benutzerverzeichnis()` (`crates/krk-core/src/ablage/pfade.rs:71`) liefert
`Option<PathBuf>` über `std::env::home_dir()`. Zwei Aufrufer hängen daran; ein dritter für
den Downloads-Ordner gehört dorthin.

**Atomares Schreiben liegt bereit.** `crates/krk-core/src/ablage/atomar.rs`.

## Was nicht geprüft ist

`inference:` Ob die Befehlsantwort in der Statuszeile sichtbar ist, während die
Belegungsansicht als Blatt steht. Das Blatt ist dokumentmodal
(`crates/krk-ui/src/appkit/blaetter/mod.rs:508`) und deckt einen Teil des Fensters ab; ob es
den Fuß eines Dateifensters verdeckt, ist nicht gemessen. Die Frage bindet die Antwort auf
Frage 7 unten.

`speculation:` Ob macOS bei einem Schreibvorgang nach `~/Downloads`, den KRK selbst anstößt,
eine Rückfrage zeigt und wie ein abgelehnter Zugriff aussieht. Der Circle-Datensatz führt den
Punkt bereits und verlangt einen Prüflauf am gebauten Bündel; der Spec nimmt ihn als
Abnahmekriterium auf.

## Die sieben Fragen, die dem Nutzer vorliegen

Sie stehen ausformuliert mit Möglichkeiten und Empfehlungen im Bericht dieses Laufs an den
Orchestrator. In Kürze:

1. Der konkrete Dateiname.
2. Ob die Datei oben einen Erzeugungszeitpunkt trägt.
3. Wie die sieben Wirkungsbereiche in der dritten Spalte beschriftet sind.
4. Was in der dritten Spalte bei den sechs vom Menü zugestellten Textbefehlen steht.
5. Wie die Erfolgsmeldung lautet.
6. Ob das Überschreiben einer vorhandenen Datei gemeldet wird.
7. Ob gemeldet wird, dass bei offener Belegungsansicht der gesicherte Stand geschrieben wurde.

## Nächster Schritt

Der Orchestrator legt die sieben Fragen dem Nutzer vor und dispatcht den Shaper erneut, mit
den Antworten und einem Verweis auf diese Datei. Der Shaper schreibt dann den Spec nach
`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/`.
