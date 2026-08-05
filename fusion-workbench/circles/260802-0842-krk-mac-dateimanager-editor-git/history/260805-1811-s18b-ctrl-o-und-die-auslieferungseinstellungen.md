# S18b: ctrl+o in der Auslieferungsbelegung und die Auslieferungseinstellungen

Status: Complete
Agent: ontocoder
Datum: 260805-1811
Circle: 260802-0842-krk-mac-dateimanager-editor-git
Auftrag: `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` `#### 18b.`
Grundlage: Spec `### C11`, Plan `### Frage 4`, `decisions/260805-1623_a_taste-und-einstellbarkeit-des-terminal-befehls.md`

## Was der Schritt verlangt hat

Zwei Datenartefakte. Ein 57. Funktionseintrag `terminal_oeffnen` auf `ctrl+o` in
`resources/default-keymap.toml`, und die neue Datei `resources/default-settings.toml` mit
genau einem Wert, `terminal = "com.apple.Terminal"`. Die eigentliche Arbeit liegt in den
Kommentaren der zweiten Datei: sie beantworten den Einwand gegen die Bündelkennung, den
`### Frage 4` benennt und ausdrücklich nicht mit einer zweiten Namensform beantwortet.

## Die Belegungsdatei

Der neue Eintrag steht in einem eigenen Abschnitt `── C11: das Terminal im angezeigten
Ordner ──`, eingefügt zwischen dem C10-Block und der Belegungsansicht. Damit läuft die
Reihenfolge der Abschnitte in diesem Teil der Datei weiter aufsteigend, und der Eintrag
steht nicht in einem Abschnitt, dessen Überschrift er widerlegt.

```
[[funktion]]
id = "terminal_oeffnen"
name = "Ordner im Terminal öffnen"
tasten = ["ctrl+o"]
```

Der Eintragskommentar trägt zwei Punkte: das Total-Commander-Vorbild samt den zwei vom
Nutzer verworfenen Möglichkeiten mit Verweis auf den Datensatz, und dass kein Cmd-Kürzel
danebensteht, weil die Zwei-Wege-Regel aus C3 den sechs Funktionen der Norton-Reihe gilt
und nicht jeder Funktion.

Zwei Stellen im Kopfkommentar sind nachgezogen:

| Vorher | Nachher |
|---|---|
| `Faehigkeiten C1 bis C7 sowie C10` | `Faehigkeiten C1 bis C7, C10 und C11` |
| `56 Funktionen mit zusammen 63 Kombinationen` | `57 Funktionen mit zusammen 64 Kombinationen` |

Die Quellenzeile war ohne C11 nicht mehr wahr, sobald der Eintrag steht; sie ist deshalb
mitgezogen, obwohl der Plan nur die Zählangabe nennt.

**Eine Abweichung vom Abnahmetext.** Das Abnahmekriterium begründet den Anker in
`grep -c '^tasten = .*ctrl+o'` damit, dass der Kopfkommentar die Kombination ebenfalls
nenne. Er nennt sie nicht: die Begründung für `ctrl+o` gehört an den Eintrag, wo sie
neben der Kombination steht, und nicht in den Kopf, wo sie eine zweite Stelle über
dieselbe Sache wäre. Der Anker ist damit eine Vorsichtsmaßnahme statt einer Notwendigkeit,
und der Zähler liefert 1, geankert wie ungeankert.

## Die Einstellungsdatei

`resources/default-settings.toml` ist neu und trägt einen Wert. Der Kopfkommentar sagt,
woher die Datei kommt und wohin sie geht (`include_str!`, Anlage beim ersten Start durch
S18c, danach kein Schreibvorgang mehr), nennt Spec und Datensatz als Quelle und trägt die
Aufnahmeregel.

**Die Aufnahmeregel, wie sie in der Datei steht.** Aufgenommen wird ein Wert, der drei
Bedingungen zugleich erfüllt: er ist keine Tastenbelegung, KRK schreibt ihn im Betrieb
nicht selbst, und er hat in dieser Runde keine Oberfläche. Dazu die drei Ausschlüsse mit
ihrem jeweiligen Grund, damit die Regel nicht als Geschmack gelesen wird: was KRK selbst
fortschreibt, gehört nach `session.toml`, weil es sich beim Arbeiten ändert und nicht beim
Einrichten; eine Tastenbelegung gehört nach `keymap.toml`, weil der Zurücksetzen-Befehl
aus C3 sonst eine Einstellung mit zurücksetzte; und sobald eine Ansicht einen Wert ändern
kann, gehört er nicht mehr hierher, weil mit der Ansicht ein Schreibpfad käme und ein
Schreibpfad die Kommentare löscht, die den Sinn der Datei ausmachen.

Der Wertkommentar trägt die drei vom Plan verlangten Punkte:

1. Der Wert ist eine Bündelkennung und kein Pfad. Grund: eine Kennung überlebt Verschieben,
   Umbenennen und den Ersatz durch eine neuere Fassung, und Terminal.app selbst ist der
   Fall, seit sie unter `/System/Applications/Utilities/` liegt.
2. `mdls -name kMDItemCFBundleIdentifier -raw /Applications/<Name>.app` liest die Kennung
   jeder installierten Anwendung aus.
3. Die eingestellte Anwendung muss Ordner annehmen, erkennbar an `public.directory` unter
   `CFBundleDocumentTypes` in ihrer Bündelbeschreibung.

## Die beiden Kennungen sind selbst ausgelesen

Der Auftrag verlangt ausdrücklich, keine Kennung hinzuschreiben, die nicht selbst
ausgelesen ist. Beide sind es, auf diesem Gerät, am 260805-1811:

```
mdls -name kMDItemCFBundleIdentifier -raw /System/Applications/Utilities/Terminal.app
  → com.apple.Terminal
mdls -name kMDItemCFBundleIdentifier -raw /Applications/Ghostty.app
  → com.mitchellh.ghostty
```

Weitere Kennungen nennt die Datei nicht. Für iTerm, WezTerm, Alacritty und kitty steht
nichts darin; keine davon ist auf diesem Gerät installiert, und eine geratene Kennung sähe
geprüft aus.

Mitgeprüft ist die dritte Kommentaraussage, weil sie sonst eine Behauptung über zwei
benannte Anwendungen wäre:

```
plutil -extract CFBundleDocumentTypes json -o - <App>/Contents/Info.plist | grep public.directory
```

Beide Bündel führen `public.directory`.

## Nachweis

| Abnahmepunkt | Ergebnis |
|---|---|
| `cargo test -p krk-core --test belegung` | 36 von 36, Rückgabewert 0 |
| `make check` (Bau, Test, Clippy, fmt) | „alle vier gruen" |
| `grep -c '^tasten = .*ctrl+o' resources/default-keymap.toml` | 1 |
| Neue `[[funktion]]`-Einträge im Diff | 1 hinzu, 0 fort |
| Zahlen im Kopfkommentar | 57 und 64 |
| `default-settings.toml` gültiges TOML mit genau einem Wert | ja, geparst mit der Kiste `toml` 1 |
| Kennungen in den Kommentaren | genau die beiden geprüften |

Die Konfliktfreiheit ist zweifach belegt: über `die_auslieferungsbelegung_ist_konfliktfrei`
und `cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt` aus der Prüfdatei, und
unabhängig davon über ein Wegwerfprogramm unter `/tmp`, das beide Dateien mit der Kiste
`toml` parst und die Paare aus Kombination und Zusteller zählt. Es meldet 57 Funktionen,
64 Kombinationen, keine doppelte Kombination bei gleichem Zusteller und `cmd+a` als den
einen Fall mit zwei Zustellern. Verglichen ist am vollständigen Eintrag und nicht an der
Teilzeichenkette. Das Programm liegt außerhalb des Baums und ist nicht Teil der Änderung.

Die Zeilenbreite von 79 Zeichen aus der vorhandenen Datei ist in beiden Dateien gehalten,
die Trennzeile des neuen Abschnitts eingeschlossen. Die Kommentare beider Dateien bleiben
bei der Umschrift `ae`/`oe`/`ue`, die `default-keymap.toml` durchgängig führt; Umlaute
stehen nur in Werten (`name = "Ordner im Terminal öffnen"`) und in Zitaten daraus.

## Was S18c wissen muss

**Die Anlage beim ersten Start muss die eingebettete Zeichenkette wörtlich schreiben, nicht
eine Serialisierung von `Einstellungen`.** Der Plan sagt „samt deren Kommentaren", und
`serde` kennt keine Kommentare. Die drei vorhandenen Ablagedateien gehen über
`toml::to_string` einer Struktur; diese vierte darf es nicht, sonst entsteht beim ersten
Start eine Datei mit einer Zeile und ohne die 50 Kommentarzeilen, die ihren Zweck
ausmachen. Der atomare Schreibweg aus S10 trägt das unverändert, die Nutzlast ist eine
andere.

**Der Wert wandert nicht in die Belegungsdatei zurück.** `terminal_oeffnen` trägt dort nur
die Kombination; welche Anwendung gerufen wird, sagt allein `settings.toml`. Der
Abschnittskommentar in `default-keymap.toml` schreibt das aus, damit ein späterer Leser
nicht am falschen Ort sucht.

**Der Eintrag trägt heute kein Kommando.** Das ist derselbe Zustand, in dem
`belegung_ansehen` seit S11c steht, und die Prüfung
`jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` misst in der Richtung
Kommando → Belegung; ein Eintrag ohne Kommando bricht sie nicht. Erst S18c dreht das um,
indem es `Kommando::TerminalOeffnen` samt Kennung anlegt.

## Geänderte Dateien

- `resources/default-keymap.toml` (25 Zeilen hinzu, 3 fort)
- `resources/default-settings.toml` (neu, 54 Zeilen)

Kein Eingriff in `crates/`, `xtask/`, den Plan oder den Spec. Keiner der acht offenen
Defekte ist angefasst. Nicht committet, wie beauftragt; den `[DONE]`-Vermerk setzt der
Auftraggeber.

## Anmerkung zum Setup

`fusion-rules ontocoder` hat `stilwerk/chat-voice-de.yaml` ausgegeben und kein
`default-voice-de.yaml`. Für diesen Agenten ist das erwartet: das Langform-Schreibprofil
geht an die Prosa-Agenten. Diese Datei folgt dem Muster der vorhandenen Historieneinträge
dieses Circles.
