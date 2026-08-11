# Durchsicht: der Codeanteil von Turn 1 (S1 bis S3), Tastenbelegung als Markdown

**Datum:** 2026-08-11, 10:00
**Sender:** coderev
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Umfang:** `git diff 5e369bb..HEAD -- crates/`, vier Commits: `39687f3` (S1), `33cc083` (S2),
`fd863e3` (S3), dazu `f1ce0f5` am Circle-Datensatz. 1728 Zeilen zu, 44 ab, über neun Dateien.
**Maßstab:** Spec `planning/260811-0753_*_spec-*.md` (38 Kriterien in C1 bis C4, zwei unter
`## Verhältnis zu den zehn Zeitzusagen`), Plan `planning/260811-0838_*_plan-*.md` (Befunde 1
bis 8, Fragen 1 bis 8, S1 bis S4), die sechs `_a_`-Datensätze unter `decisions/`.

---

## Zusammenfassung

Der Bau ist grün, und die Umsetzung folgt dem Plan eng: die Kürzung über `Path::strip_prefix`,
das unteilbare Schreiben, die Fehlerunterscheidung am Rückgabewert statt an einer Vorabprüfung,
die geteilte Gliederung, der Selektor am Delegierten und die Verbotsseite stehen alle so da, wie
der Plan sie beschreibt. **Ein Befund von Gewicht:** der Auffangzweig in `wirkung` wird als
unerreichbar geführt, und das trifft nicht zu — `belegungsmodell::bereich` und
`belegungsausgabe::wirkung` stellen zwei verschiedene Fragen, und für eine Nutzerbelegung mit
`gehalten_von` auf einer Kommandokennung fallen die Antworten auseinander. **Ein zweiter:** der
Nutzerentscheid vom 260811-0935, auf den sich der Programmtext dreimal beruft, steht im
Arbeitsbereich nirgends, und der abgenommene Spec sagt an dieser Stelle etwas anderes.

Die drei Begründungslagen der dritten Spalte sind sonst sauber auseinandergehalten. Keine Zelle
behauptet am Zweig mehr, als ihre Quelle hergibt — der `Editor`-Zweig ausdrücklich eingeschlossen.

## Zahlen

| Schwere | Anzahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 2 |
| Niedrig | 4 |

Sechs Datensätze liegen unter `issues/` dieses Circles, jeder mit `_o_`.

## Die sechs Prüfaufträge, der Reihe nach beantwortet

### 1. Der Auffangzweig `_ => ""` — die Begründung trägt nicht

**Geprüft und widerlegt.** Der Kommentar (`belegungsausgabe.rs:258-265`) begründet die
Unerreichbarkeit damit, `nach_bereichen` breche zuvor laut ab. Der Abbruch hängt an
`belegungsmodell::bereich`, und das fragt über **`Kommando::aus_kennung`**
(`belegungsmodell.rs:149`). `wirkung` fragt über **`Funktion::kommando`**
(`belegungsausgabe.rs:213`), und das ist nicht dasselbe: `Funktion::kommando`
(`krk-core/src/tasten/belegung.rs:757-762`) liefert `None`, sobald `gehalten_von` gesetzt ist —
unabhängig davon, was `Kommando::aus_kennung` sagen würde.

Am 260811-0955 gegen `krk-core` gemessen, mit einer Wegwerfkiste außerhalb des Baums:

```
angenommen: kommando()=None gehalten_von=Some("menue") aus_kennung=Some(Kopieren)
```

Eine `keymap.toml`, die `kopieren` einen Zusteller gibt, wird von `Belegung::vom_nutzer`
angenommen — `Belegung::bauen` prüft allein die Kennung gegen den Wortschatz und übernimmt
`gehalten_von` unverändert, und `konflikte` vergleicht nur innerhalb desselben Zustellers.
`bereich("kopieren")` ordnet sie ein, `nach_bereichen` bricht **nicht** ab, und `wirkung` landet
im Auffangzweig. Die Funktion steht dann in der Datei, im richtigen Abschnitt, mit ihren
Kombinationen und mit leerer dritter Zelle.

**Meine Einschätzung zur zweiten Hälfte der Frage — ist ein unerreichbarer Auffangzweig hier
richtig?** Die Frage ist an dieser Stelle anders gestellt, als sie zunächst aussieht, und der
Unterschied ist tragend:

**Die Projektregel aus `CLAUDE.md` greift hier gar nicht.** Sie handelt von Fallunterscheidungen
über Aufzählungen dieses Projekts, bei denen der Übersetzer die Vollständigkeit erzwingt und ein
`_`-Zweig genau diese Erzwingung wegnimmt. Der `match` in `wirkung` läuft über `&str`. Ein
`&str`-`match` ist in Rust **nie** ohne Auffangzweig übersetzbar. Es geht hier also nicht um eine
verlorene Bauunterbrechung — die gäbe es an dieser Stelle unter keinen Umständen — sondern
allein darum, was der Zweig tut, wenn er greift. Der `coder` hat sich der Regel nicht entzogen;
sie ist auf diesen `match` nicht anwendbar.

**Falsch ist etwas anderes, und es wiegt schwerer als die Regelfrage.** Die leere Zelle hat in
dieser Datei bereits eine gesetzte Bedeutung: `text_alles_auswaehlen` bleibt leer, weil S1 die
Ableitung gebrochen hat, und ein eigener Datensatz hält fest, dass das ein Ergebnis und kein
Versäumnis ist. Der Auffangzweig liefert für einen völlig anderen Sachverhalt denselben Ausgang.
„Hier ist nichts entschieden" und „hier hat niemand nachgesehen" sind in der Datei danach nicht
mehr unterscheidbar — und der Nutzer, für den diese Spalte gebaut wurde, hat keinen Weg, das zu
merken. Das ist die teuerste Fehlerform, die der Spec kennt, nur von der anderen Seite: nicht
eine Zusicherung, die zu stark ist, sondern zwei verschiedene Aussagen, die in derselben leeren
Zelle zusammenfallen.

**Mein Rat, und er löst die Ursache statt des Symptoms:** die beiden Fallunterscheidungen
deckungsgleich machen, statt den Auffangzweig zu bearbeiten. Fragte `wirkung` dieselbe Frage wie
`bereich`, wäre der Zweig tatsächlich durch den lauten Abbruch in `nach_bereichen` gedeckt, und
er dürfte dann so laut abbrechen wie dieser. Der Preis steht im Doc-Kommentar von `wirkung` und
ist ernstzunehmen: die Zusage hinge wieder daran, dass `Kommando::KENNUNGEN` die sechs
Textbefehle nicht nennt. Diese Abwägung ist eine Nutzerfrage, und ich lege sie vor, statt sie zu
treffen. **Was nicht geht: ein `panic!` ohne diese Angleichung** — es brächte KRK an einer von
Hand geschriebenen, formal zulässigen `keymap.toml` zum Absturz.

Der Vollständigkeit halber: `jede_kennung_ohne_kommando_wird_vom_menue_zugestellt` läuft über
`Belegung::auslieferung()`. Ihr Doc-Kommentar sagt trotzdem, sie fange eine Funktion, „bevor sie
eine leere Zelle in der Datei erzeugt". Für eine Nutzerbelegung gilt der Satz nicht.

Datensatz: `issues/260811-0955_*_der-auffangzweig-in-wirkung-ist-erreichbar-*.md`.

### 2. Die drei Begründungslagen — der Programmtext trägt sie, mit zwei Ungenauigkeiten daneben

**Die 65 mit Kommando** lesen aus `beschriftung()` ab (`belegungsausgabe.rs:213-215`), ohne
Zwischenschritt und ohne zweite Quelle. Die Probe hält es für jede einzelne fest und zählt
gegen `Kommando::KENNUNGEN.len()` gegen (`:597-614`).

**`text_ausschneiden`/`text_kopieren`/`text_einfuegen`** tragen „Textfelder und Editor"
(`:226`). Die Messung steht in `menue.rs` als `GEMESSEN` und läuft mit.

**`text_alles_auswaehlen`** bleibt leer (`:244`), mit dem Verweis auf den Datensatz und dem
ausdrücklichen Satz, was die Messung **nicht** entschieden hat. Genau richtig gefasst.

**`text_rueckgaengig`/`text_wiederholen`** tragen „Editor" (`:256`). Der Zweigkommentar sagt
ausdrücklich, dass S1 hier nichts entscheiden konnte und der Beleg von woanders kommt.
`setAllowsUndo(true)` steht am Baum, geprüft: `crates/krk-ui/src/appkit/editor.rs:3376`. **Der
Zweig behauptet nichts über Textfelder** — er sagt „Editor" und nicht „Textfelder und Editor",
und das ist genau das, was `setAllowsUndo` hergibt. Der Warnung aus dem Auftrag ist damit
Genüge getan; die Untersagung wirkt hier sogar in die vorsichtige Richtung, denn im Feldeditor
dürfte Rückgängig ebenfalls etwas tun, und der Text sagt es nicht zu.

Zwei Ungenauigkeiten stehen daneben, beide im Text und keine im Verhalten:

- Die Tabelle im Modulkopf (`:51`) führt „Textfelder und Editor" als **gemessen**. Gemessen ist
  die eine Hälfte — welche Klassen `cut:`, `copy:` und `paste:` beantworten. Die andere Hälfte,
  dass der Feldeditor eines `NSTextField` eine `NSTextView` ist, ist AppKit-Wissen und keine
  Messung; `responds_to` legt keine Instanz an und sagt über den Ersthelfer nichts. Der **Zweig
  selbst** schreibt die Kette sauber aus, der Modulkopf verkürzt sie. Der Datensatz `260811-0930`
  verkürzt sie ebenfalls („jetzt ist es gemessen").
- Die Begründungslagen sind dreimal verschieden gezählt: „drei verschiedene Quellen" über einer
  vierzeiligen Tabelle, „Dritte Lage" an zwei verschiedenen Sachverhalten, und in der Probe
  wieder anders.

Datensatz: `issues/260811-0957_*_gemessen-reicht-fuer-textfelder-weiter-als-die-messung-*.md`.

**Dazu ein Befund, der nicht im Code steht, sondern neben ihm fehlt.** Der Programmtext beruft
sich an drei Stellen auf einen „Nutzerentscheid vom 260811-0935". `grep -rn "0935"` über den
Circle und über `shared/` liefert am 260811-0956 keinen Treffer: kein Entscheidungsdatensatz,
kein Sitzungsbericht, kein Nachtrag im Spec. C3 des abgenommenen Specs sagt weiterhin, alle
sechs trügen „Textfelder und Editor", und der Plan schreibt in seiner Risikotabelle, eine
berichtigte Beschriftung gehöre „an das Gate, nicht in den stillen Bau". Die Berichtigung mag am
Gate gefallen sein — ihr Beleg fehlt. Datensatz:
`issues/260811-0956_*_der-nutzerentscheid-vom-260811-0935-steht-allein-im-programmtext.md`.

### 3. `gekuerzt_fuer_anzeige` — so gebaut, wie der Plan es verlangt

`crates/krk-core/src/ablage/pfade.rs:117-126`. Der Vergleich läuft über `Path::strip_prefix`,
nicht über Zeichenketten. Alle vier Fälle stehen da und sind alle über `strip_prefix`
entschieden:

- unter dem Benutzerverzeichnis → `~/` und der Rest
- **ist** das Benutzerverzeichnis → `~`, ohne Schrägstrich (`Ok(rest) if rest.as_os_str().is_empty()`)
- außerhalb → `Err(_)` → `display()`, unverändert
- ohne Benutzerverzeichnis → `display()`, unverändert, kein `Option` im Rückgabewert

Die Probe `die_kuerzung_fuer_meldungen_zieht_nur_ganze_pfadbestandteile_ab`
(`crates/krk-core/tests/ablage.rs:190-231`) hält alle fünf Fälle, den benannten eingeschlossen:
`/Users/kai-alt/Downloads` gegen `/Users/kai` bleibt ausgeschrieben. Kein Fall fasst das echte
Benutzerverzeichnis an.

Dazu geprüft, was der Plan unter Frage 8 c) verlangt: **jeder** Pfad, den `Ausgang::meldung_mit`
schreibt, geht durch die Kürzung, nicht nur der der Erfolgsmeldung
(`belegungsausgabe.rs:313`, die Hülle `kurz`). Die Probe `die_meldungen_tragen_den_pfad_mit_tilde`
prüft für die drei Fehlerfälle mit Pfad zusätzlich, dass der ausgeschriebene Pfad **nicht**
danebensteht. `Ausgang` selbst trägt den ungekürzten Pfad, wie der Plan es sagt.

### 4. Die Verbotsseite — vollständig eingehalten

| Zusage | Befund |
|---|---|
| `resources/default-keymap.toml` behält 71 Blöcke | `grep -c '^\[\[funktion\]\]'` → **71**; die Datei steht nicht im Diff |
| `crates/krk-ui/src/fenstertitel.rs` unangetastet | steht nicht im Diff |
| `Kommando::KENNUNGEN` bei 65 | `belegung.rs:462` → `[(Kommando, &'static str); 65]` |
| Der Doc-Kommentar von `antwort_zeigen` behält seine vier Ränge | `anwendung.rs` trägt genau drei Hunks — `use`, der `define_class!`-Zweig, die neue Methode. `antwort_zeigen` ist nicht berührt; der Defekt `issues/260811-0838_o_antwort-zeigen-*.md` bleibt unangetastet offen, wie der Plan es vorsieht |

Dazu: `Wirkungsbereich` bleibt bei sieben Werten, `Bereich` und `Fokus` sind nicht berührt.

### 5. `deny(unsafe_code)` und die Verfügbarkeitsangaben — beides in Ordnung

`#![deny(unsafe_code)]` steht in `krk-core/src/lib.rs:1`, `krk-ui/src/main.rs:1` und
`krk-bench/src/main.rs:1`. Die Ausnahme `#![allow(unsafe_code)]` steht an genau zwei Stellen:
`krk-core/src/verzeichnis/sys.rs:71` und `krk-ui/src/appkit/mod.rs:1`. Nichts ist dazugekommen.
`belegungsausgabe.rs` nennt keine `objc2`-Kiste (`grep -c objc2` → 0), wie der Plan es zusagt.

**Die Aussage zu `NSMenuItem::separatorItem` trifft zu.** `NSMenuItem` steht seit macOS 10.0.
Der Modulkopf von `menue.rs` führt sie seit `fd863e3` im eigenen Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen` und nennt dort auch die sechs Klassen,
die die neue Messung anspricht — `NSTableView`, `NSTextView`, `NSTextField`, `NSScrollView`,
`NSWindow`, `NSApplication` —, alle seit 10.0. Die Gewohnheit ist eingehalten und für die neu
angesprochenen Klassen ausgeweitet, nicht nur für die eine, nach der der Auftrag fragt.

### 6. Der Selektor `tastenbelegungSichern:` — der Form nach richtig, die Kette erreicht ihn

Die Form ist Zeile für Zeile die der drei bestehenden (`anwendung.rs:536-545` gegen `:496-534`):
Doc-Kommentar, `// SAFETY:`-Vermerk über die Signatur, `#[unsafe(method(...))]`, ein Argument
`_absender: Option<&AnyObject>`, und der Rumpf reicht an eine gewöhnliche Methode daneben weiter.
Die Benennung folgt der Reihe `beenden:`, `fensterEinblenden:`, `fensterSchliessen:`.

Der Menüeintrag entsteht über `ohne_kuerzel` und nicht über `befehl` (`menue.rs:286-291`), setzt
kein Ziel (`roher_befehl` setzt keines, `:450-456`) und steht vor dem Beenden, durch
`NSMenuItem::separatorItem` getrennt. Die Antwortkette endet bei `NSApplication` und ihrem
Delegierten, und dieser Weg ist im Programm dreifach belegt — die drei bestehenden Einträge
gehen ihn seit der Runde 1 beziehungsweise seit C7.

Anders als bei `fensterSchliessen:` und `beenden:` besteht hier kein Zweitform-Risiko: der
Selektor ist eigen und AppKit kennt ihn nicht. Ob die Kette ihn **bei stehendem Blatt** erreicht,
ist aus dem Baum nicht entscheidbar und steht zu Recht in S4.

Die Methode selbst (`anwendung.rs:2258-2262`) trägt keine Blattabfrage, und der Doc-Kommentar
begründet, warum nicht — richtig gegen C1 gelesen. Die Belegung kommt aus
`self.ivars().belegung`, nicht aus `fuer_den_betrieb()`; Befund 1 und 2 des Plans sind damit
gebaut. Die Leihe endet mit der Anweisung, bevor `modell` geliehen wird.

## Weitere Befunde

### `main.rs` nennt elf Module neben `appkit`, es sind zwölf

`main.rs:17` sagt „Elf Module" und zählt elf auf; `hervorhebung` fehlt, und es erfüllt beide
Bedingungen (liegt neben `appkit`, nennt keine `objc2`-Kiste). Die Lücke ist älter als diese
Runde — vorher stand „Zehn", und schon damals waren es elf. S3 hat sie mitgenommen, nicht
verursacht. Der Vermerk steht trotzdem, weil die Runde genau diese Zeile angefasst hat und
`CLAUDE.md` diese Fehlerform ausdrücklich führt; am 260810 sind zwei Kommentarzahlen derselben
Art als Defekte aufgelaufen. Datensatz: `issues/260811-0958_*_elf-module-neben-appkit-sind-zwoelf-*.md`.

### Kein Sitzungsbericht zu S1 bis S3

`history/` führt sechs Dateien, keine vom `coder`. S1 macht den Bericht ausdrücklich zum Teil
seiner Abnahme („Der Sitzungsbericht trägt die sechs Antworten ausgeschrieben"). Zwei der drei
Wege, die der Plan für das Messergebnis vorsieht, sind gegangen — die Probe und der
Defektdatensatz —, der dritte fehlt. Der Inhalt ist damit nicht verloren, der Verlauf schon, und
mit ihm die Stelle, an der der Entscheid vom 0935 gefallen wäre. Datensatz:
`issues/260811-0959_*_zu-s1-bis-s3-gibt-es-keinen-sitzungsbericht-*.md`.

### Die Begründung für den Downloads-Ordner nennt das Schreiben nicht

`resources/Info.plist:178-179` beschreibt fünf Handlungen — anzeigen, öffnen, kopieren,
verschieben, umbenennen — und keine davon ist die, die diese Runde hinzugefügt hat. Der Satz ist
das, was macOS dem Nutzer in der Rückfrage vorlegt, und C2 hält den Unterschied selbst fest:
„Neu an diesem Schreibvorgang ist allein, dass KRK den Zielordner selbst wählt." Gehört zu S4,
nicht zu S3. Datensatz: `issues/260811-1000_*_die-begruendung-fuer-den-downloads-ordner-*.md`.

## Was geprüft ist und stimmt, ohne eigenen Befund

- **Der Bau.** `cargo test --workspace` → 16 Läufe, alle `ok`, 0 fehlgeschlagen (353 in
  `krk-core`, 45 in `krk-ui`, dazu die Prüfziele). `cargo clippy --workspace --all-targets --
  -D warnings` → grün. `cargo fmt --all --check` → grün. Selbst gefahren am 260811-0955.
- **S2 im Ganzen.** `Wirkungsbereich::beschriftung` ist eine vollständige Fallunterscheidung
  ohne `_`-Zweig mit genau den sieben Texten des Specs. Die Probe geht weiter als das
  Abnahmekriterium: `stelle_in_den_sieben` ist eine zweite Fallunterscheidung ohne Auffangzweig,
  damit ein achter Wert auch im Prüffeld eine Zeile abverlangt bekommt. Das ist die richtige
  Antwort auf die Frage, wie eine Probe mit einer Aufzählung mitwächst.
- **Die geteilte Gliederung.** `nach_bereichen` und `tastenliste` sind Umzüge ohne
  Verhaltensänderung; `gliederung` und `tastentext` rufen sie. Eine zweite Aufbereitung ist
  nicht entstanden, wie die Directive verlangt. Der laute Abbruch ist mitgewandert.
- **Das unteilbare Schreiben.** Über `atomar::schreiben`, ein zweiter Schreibweg entsteht nicht.
  Die Probe prüft, dass die Nachbardatei `.neu` nach dem Umbenennen fort ist und dass in beiden
  Fehlerfällen weder eine ganze noch eine halbe Datei zurückbleibt.
- **Die Fehlerunterscheidung.** Am Rückgabewert und nicht an einer Vorabprüfung (Befund 8). Der
  Auffangzweig in `in_ordner_schreiben` (`:376`) steht an `io::ErrorKind`, das
  `#[non_exhaustive]` ist — dort ist er unvermeidlich, und der Kommentar sagt es. Er verliert
  auch nichts, weil `Fehlgeschlagen` den Wortlaut mitträgt. Das ist das Gegenbeispiel zum Zweig
  in `wirkung`: hier ist der Auffangzweig richtig und richtig begründet.
- **Die Maskierung.** `maskiert` ersetzt `|` durch `\|` in allen drei Zellen; die Probe prüft,
  dass die Zeile trotz Strich drei Spalten trägt. Die Prüfhilfe `zellen` teilt an `" | "` und
  wird durch die Maskierung nicht getäuscht, weil dem Strich dann ein `\` und kein Leerzeichen
  vorausgeht.
- **Die Prüfordner-Regel.** Die Proben nehmen `crate::pruefordner::Pruefordner`; eine
  dreizehnte Fassung ist nicht entstanden. Keine Probe legt eine Datei im echten
  Downloads-Ordner an.
- **Die Rechteprobe.** `ein_fehlender_ordner_und_ein_abgelehnter_zugriff_sind_unterscheidbar`
  setzt die Rechte zurück, **bevor** sie zusichert, und sagt im Fehltext, dass ein Lauf unter
  root nichts belegt. Genau die Sorgfalt, die eine solche Probe braucht.

## Übergreifend

**Eine Bauform, zwei Fragen — das ist die eigentliche Beobachtung dieser Durchsicht.** Der
Befund zum Auffangzweig ist kein Tippfehler, sondern der Abdruck einer Asymmetrie, die schon
vorher im Baum stand: `belegungsmodell::bereich` nimmt eine Kennung, `belegungsausgabe::wirkung`
nimmt eine Funktion, und die Zustellerregel lebt an der Funktion. Solange nur die
Belegungsansicht die Gliederung benutzte, konnte das niemandem auffallen — sie zeigt keine
dritte Spalte. Die Runde 3 hat den zweiten Abnehmer gebracht und damit die Asymmetrie sichtbar
gemacht. Wer sie schließt, prüft im selben Zug, ob ein dritter Abnehmer sie wieder aufmachen
könnte.

**Der Bau folgt seiner eigenen Vorschrift genauer als der Text daneben.** Die drei Befunde
niedriger Schwere sind sämtlich Text, der weiter reicht als das, was er beschreibt: „gemessen"
für eine halb gemessene Kette, „elf" für zwölf Module, eine Nummerierung in drei Fassungen. Der
Programmtext selbst — die Zweige, die Proben, die Fehlerbehandlung — sagt an keiner Stelle mehr,
als er hergibt. Das ist die umgekehrte Verteilung zu der der Runde 2, wo zwei Zusicherungen im
Code selbst zu stark waren.

**Die Regel „vollständig ohne Auffangzweig" hat in diesem Baum drei verschiedene Anwendungsfälle**,
und diese Runde führt alle drei vor: erzwingbar über eine eigene Aufzählung
(`Wirkungsbereich::beschriftung`, richtig ohne `_`), unmöglich über eine fremde
(`io::ErrorKind`, Auffangzweig richtig und begründet) und unmöglich über `&str`
(`wirkung`, Auffangzweig unvermeidlich — dort zählt allein, was er tut). Wer die Regel künftig
zitiert, tut gut daran, den Fall mitzunennen.

## Reihenfolge

**Vor der Abnahme von Turn 1:**

1. `260811-0955` — der Auffangzweig. Die Ursache ist die Ungleichheit der beiden
   Fallunterscheidungen; welche der beiden Fragen die richtige ist, ist eine Nutzerfrage und
   sollte vor dem Bau beantwortet werden.
2. `260811-0956` — der fehlende Beleg für den Entscheid vom 0935 und der nachzuziehende Spec.
   Kostet zwei Absätze und schließt die Lücke zwischen abgenommenem Spec und Baum.

**Vor S4:**

3. `260811-1000` — die Begründung im `Info.plist`. Sie ist Teil dessen, was S4 misst.

**Aufräumen, jederzeit:**

4. `260811-0957` — „gemessen" und die dreifache Zählung.
5. `260811-0958` — elf gegen zwölf Module.
6. `260811-0959` — der Sitzungsbericht zu S1 bis S3.

Kein Befund hält S4 auf, und keiner ist ein Freigabehindernis für den Bau als solchen: die Datei
entsteht, sie trägt, was C3 verlangt, und die Fehlerfälle sind unterscheidbar. Was offen bleibt,
ist an zwei Stellen die Übereinstimmung zwischen dem, was der Text zusagt, und dem, was der Code
hergibt.
