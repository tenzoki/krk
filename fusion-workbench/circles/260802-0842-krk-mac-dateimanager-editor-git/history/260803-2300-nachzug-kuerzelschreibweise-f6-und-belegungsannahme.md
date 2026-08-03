# Nachzug: Schreibweise der Kürzel, F6 ohne das Umbenennen, Annahme der Auslieferungsbelegung

**Datum:** 260803-2300
**Agent:** planner
**Status:** Complete
**Auslöser:** Zwei Nutzerentscheidungen vom 260803-2110, dazu die beiden Defektdatensätze, die der `ontocoder` beim Schreiben von `resources/default-keymap.toml` angelegt hat.
**Geändert:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, `planning/260802-1036_o_spec-navigator-geruest.md`
**Neu angelegt:** `decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`, `issues/260803-2300_c_die-aenderungen-von-schritt-9-lassen-c4-aus-der-aufzaehlung-aus.md`
**Geschlossen:** drei Defektdatensätze, siehe unten
**Nicht angefasst:** `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`. Kein Commit. Kein `[DONE]` an S9 oder an einem anderen Schritt.
**Stilprofile:** `stilwerk/chat-voice-de.yaml` und `stilwerk/default-voice-de.yaml` geladen.

## Aufgabe 1: die Schreibweise der vier Kürzel

**Das Abnahmekriterium von Schritt 9 stand in einer Reihenfolge, die derselbe Schritt verbietet.** Es verlangte die sechs Zeilen der C3-Tabelle als `f3`+`cmd+y`, `f5`+`cmd+shift+k`, `f6`+`cmd+shift+v`, `f7`+`cmd+shift+n`, `f8`+`cmd+opt+delete` und `delete`+`cmd+delete`. Die Kombinationsschreibweise desselben Schrittes schreibt `[ctrl+][opt+][shift+][cmd+]<taste>` in genau dieser Reihenfolge vor und damit `shift+cmd+k`, `shift+cmd+v`, `shift+cmd+n` und `opt+cmd+delete`. Vier der sechs Zeichenketten verletzten die Vorschrift; die beiden übrigen tragen nur eine Zusatztaste und waren nicht berührt.

Nachgezogen ist das Kriterium zugunsten der Schreibweise. Der Grund steht jetzt im Plan und nicht mehr allein im Defektdatensatz: der Parser aus Schritt 11 liest `resources/default-keymap.toml`, die Reihenfolge der Zusatztasten ist sein Vertrag, und eine Kombination in anderer Reihenfolge wäre für ihn eine andere Kombination. Zwei Reihenfolgen nebeneinander verlangten ihm eine Sonderregel ab, die keine Fähigkeit dieser Runde braucht.

**Die Durchsicht auf dieselbe Verwechslung fand zwei weitere Fundstellen und ließ beide stehen.** Die Unterscheidung, die der Nutzer verlangt hat, trägt: übersetzt wird die Prosaform allein dort, wo ein Kriterium eine Zeichenkette wörtlich prüft.

| Fundstelle | Form | Prüft sie eine Zeichenkette? | Behandlung |
|---|---|---|---|
| Plan, S9, Abnahmekriterium | `cmd+shift+k` und drei weitere | ja, gegen `resources/default-keymap.toml` | übersetzt |
| Spec, C3, Tabelle und Prosa | "Cmd+Shift+K" | nein, Mac-Prosaform für den Leser | unverändert |
| Plan, S7, Abnahmekriterium | `cmd+shift+k` | nein, benennt die Prüfung `cmd_shift_k_behaelt_beide_bits` aus `crates/krk-core/tests/tasten.rs` | unverändert |

Der Fund bei S7 ist geprüft und nicht erschlossen: `grep -n "shift" crates/krk-core/tests/tasten.rs` liefert die Zeile `fn cmd_shift_k_behaelt_beide_bits()` und die Zusicherung `assert_eq!(druck.maske.to_string(), "command+shift")`. Weder der Testname noch die Zusicherung enthält die Kombinationsschreibweise aus S9. Ein `grep` nach `cmd+shift`, `cmd+opt`, `shift+cmd` und `opt+cmd` über `crates/`, `resources/` und `xtask/` liefert 16 Treffer, alle in `resources/default-keymap.toml`. Vierzehn davon sind Tastenlisten und tragen ausnahmslos die vorgeschriebene Reihenfolge. Die beiden übrigen sind Kommentarzeilen in der Mac-Prosaform: die eine erklärt die Übersetzung von "Cmd+Shift+K" in die Schreibweise der Datei, die andere nennt das Finder-Kürzel für versteckte Dateien als "cmd+shift+Punkt". Beide beschreiben und belegen nicht; der Parser liest sie nicht. `resources/` blieb in diesem Nachzug unangetastet.

**Eine dritte Präzisierung an derselben Zeile war nötig.** Das Kriterium verlangte, die Zeichenketten `shift+delete`, `cmd+c` und `cmd+v` kämen in keiner Tastenliste vor. Als roher Abgleich auf Teilzeichenketten kann das nicht aufgehen: `shift+cmd+v` ist das vom Spec vorgeschriebene Kürzel für das Verschieben und enthält `cmd+v`. Das Kriterium prüft jetzt ausdrücklich den vollständigen Eintrag mit `grep -F '"cmd+v"'`. Der `ontocoder` hatte bereits so geprüft und das in seinem Sitzungsdatensatz festgehalten; das Kriterium sagte es nur nicht.

## Aufgabe 2: F6 trägt allein das Verschieben

**Entschieden vom Nutzer am 260803-2110: F6 verschiebt, `shift+f6` und `shift+cmd+u` benennen um.** Damit gilt die Kürzel-Tabelle aus C3, und die Formulierung "F6 Verschieben und Umbenennen" ist aus dem Abnahmekriterium desselben Abschnitts gestrichen. Der Grund, den der Defektdatensatz nennt, trägt die Wahl: C4 führt das Umbenennen als eigene Fähigkeit, und C3 schließt zwei Funktionen auf einer Kombination aus.

Geändert sind im Spec drei Stellen von C3:

1. Das Abnahmekriterium der Norton-Zuordnung nennt für F6 nur noch das Verschieben.
2. Ein neues Abnahmekriterium darunter schreibt aus, dass das Umbenennen eine eigene Funktion aus C4 mit eigener Zeile in der Belegungsansicht ist, ab Werk auf Shift+F6 und Cmd+Shift+U.
3. Eine Festlegung begründet die Trennung und hält fest, woher `shift+f6` kommt: es ist die Norton- und Total-Commander-Form für das Umbenennen und hält die Nähe zur Nachbarfunktion, ohne deren Kombination zu teilen.

**Der Plan trug dieselbe Formulierung an keiner Stelle.** Ein `grep` über die Plandatei nach "Verschieben und Umbenennen" und nach "F6" liefert vier Treffer, keiner davon mit der Zusammenlegung. Außerhalb von Spec und Plan steht die Formulierung noch in `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`, dort aber als Beschreibung der Norton-Zuordnung selbst und nicht der von KRK ausgelieferten Belegung. Die Aussage über Norton Commander bleibt richtig; der Datensatz ist deshalb unverändert geblieben.

## Aufgabe 3: die Annahme der Belegung, festgehalten als Entscheidungsdatensatz

**Die Annahme steht als eigener Datensatz und nicht als Vermerk in C3.** Drei Gründe gaben den Ausschlag, und der erste wiegt am schwersten.

Erstens verdoppelte ein Vermerk in C3, der die Belegungen aufführte, die Datendatei. Die Belegung hat genau eine Quelle, `resources/default-keymap.toml`, und der Plan begründet das in `### Frage 4` ausführlich: sie wird über `include_str!` eingebettet und ist damit der Auslieferungszustand, auf den der Zurücksetzen-Befehl aus C3 zurückführt. Eine zweite Aufstellung im Spec liefe beim ersten Nachzug auseinander. Genau diese Doppelung hat dieses Vorhaben schon einmal eingefangen, bei der Versionsnummer in `Cargo.toml` und `Info.plist`.

Zweitens braucht die Annahme eine Zustandsspur, die ein Spec-Absatz nicht trägt. Das "erstmal" des Nutzers ist ein Ablaufdatum ohne Datum: die Annahme gilt für diese Runde und kann später überholt werden. Die Markervokabel der Entscheidungsdatensätze bildet das ab, ein Absatz im Spec nicht. Wer eine der 39 Kombinationen ändert, schreibt einen Datensatz, der diesen hier überholt, und die Spur bleibt lesbar.

Drittens ist die Annahme kein Abnahmekriterium. Sie beschreibt keine prüfbare Eigenschaft der Anwendung, sondern eine Festlegung über die Herkunft von Daten. C3 nennt sie deshalb in einem Satz und verweist; der Datensatz trägt die Sache.

**Was der Datensatz ausdrücklich festhält**, weil der Nutzer es verlangt hat:

- Angenommen ist der Dateibestand von `resources/default-keymap.toml` im Stand des Commits `d1a8ab1`, mit 46 Funktionen und 52 Kombinationen. Der Bestand ist seither unverändert, geprüft mit `git diff d1a8ab1 HEAD -- resources/default-keymap.toml`, Ausgabe leer.
- Angenommen hat der Nutzer, am 260803-2110, nach Durchsicht aller 46 Funktionen, mit der Formulierung "passt erstmal so".
- Der Spec legt sieben der 46 Belegungen selbst fest: die sechs Zeilen der Kürzel-Tabelle in C3 und F4 als unbelegt. Die übrigen 39 stammen aus dieser Annahme und nicht aus dem Spec.
- Die drei Wahlregeln des `ontocoder` gelten mit angenommen und binden künftige Ergänzungen der Datei: Mac-Gewohnheit, sonst Norton- oder Total-Commander-Form, sonst der Anfangsbuchstabe des deutschen Verbs.
- Zwei offene Punkte an derselben Datei sind von der Annahme nicht abgedeckt und im Datensatz benannt: die zwei Parteien auf Cmd+W und die fehlenden Namen für die Links- und Rechts-Pfeile.

Der Marker steht auf "umgesetzt" und nicht auf "beantwortet". Antwort und Umsetzung fallen hier zusammen, weil der Nutzer eine bereits geschriebene und committete Datei durchgesehen und angenommen hat; ein Zwischenstand "beantwortet, noch nicht umgesetzt" hat nie bestanden.

## Ein dritter Defekt, gefunden und im selben Zug behoben

**Die Aufzählung in den `Änderungen` von Schritt 9 überging C4.** Sie nannte "alle Funktionen aus C1, C2, C5, C6 und C7" und deckte damit vier Funktionen nicht ab, die keine Norton-Taste tragen: eine leere Datei anlegen, umbenennen, im Stapel umbenennen und eine laufende Operation abbrechen. Das erste Abnahmekriterium von C2 verlangt für jede Funktion aus C1 bis C7 mindestens einen Tastenbefehl; wörtlich befolgt hätte Schritt 9 es verfehlt.

Der Punkt fiel bei Aufgabe 2 auf, weil das Umbenennen eine der vier ist. Die Umsetzung war nicht betroffen: der `ontocoder` hat die vier Einträge geschrieben und die Abweichung in `history/260803-2045-auslieferungsbelegung-als-datentabelle.md` offengelegt, ohne einen Defektdatensatz anzulegen. Aufgeschrieben gehört es dennoch, weil die Aufzählung in Schritt 9 die Prüfliste jeder späteren Durchsicht ist: bliebe sie unvollständig, meldete die nächste Durchsicht die vier C4-Einträge als überzählig.

Datensatz `issues/260803-2300_c_die-aenderungen-von-schritt-9-lassen-c4-aus-der-aufzaehlung-aus.md`, angelegt und im selben Zug geschlossen.

## Stand der Defektdatensätze zu Schritt 9

| Datensatz | Stand |
|---|---|
| `260803-2045_c_abnahmekriterium-von-schritt-9-schreibt-die-kuerzel-in-einer-anderen-reihenfolge-als-die-schreibweise-erlaubt.md` | geschlossen |
| `260803-2045_c_c3-nennt-f6-verschieben-und-umbenennen-die-belegungstabelle-nur-verschieben.md` | geschlossen |
| `260803-2300_c_die-aenderungen-von-schritt-9-lassen-c4-aus-der-aufzaehlung-aus.md` | geschlossen |
| `260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md` | offen, hängt an der Entscheidung zum letzten Fenster |
| `260803-2045_o_die-kombinationsschreibweise-kennt-die-links-und-rechts-pfeile-nicht.md` | offen, vom Nutzer für diese Runde in Kauf genommen |

Die beiden offenen sind in der Aufstellung `## Angelegte Defekte und Entscheidungen` des Plans jetzt mit ihrem Stand aufgeführt; sie fehlten dort bisher.

## Was S9 zum `[DONE]` noch fehlt

Aus Sicht dieses Nachzugs: nichts am Plan und nichts an der Datei. Das Abnahmekriterium ist in allen Teilen erfüllbar, und die Datei erfüllt es. Der Vermerk selbst ist ausdrücklich nicht gesetzt; der Nutzer setzt ihn.

Die beiden offen bleibenden Defekte stehen dem nicht entgegen. Der eine, Cmd+W, beschreibt keinen Fehler der Datei, sondern eine Kollision zwischen der Datei und dem Menü aus S6, deren Auflösung an der Entscheidung zum letzten Fenster hängt und in S12 fällt. Der andere, die fehlenden Pfeiltastennamen, beschreibt eine Lücke der Schreibweise aus Schritt 9; die Datei umgeht sie mit `ctrl+b` und `ctrl+s`, was das Abnahmekriterium nicht verletzt, und der Nutzer nimmt die Lücke für diese Runde in Kauf.

## Was der `coder` an S11 davon merkt

Nichts. Die Beschreibung von Schritt 11 ist inhaltlich unangetastet geblieben. Was sich für ihn ändert, ist eine Bestätigung und keine Änderung: die Reihenfolge `[ctrl+][opt+][shift+][cmd+]` gilt unverändert und ist jetzt auch im Abnahmekriterium von Schritt 9 die einzige Form, sodass Parser und Prüfvorschrift dasselbe verlangen.
