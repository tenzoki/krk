# Ordnernavigation auf die nackten Pfeiltasten umbelegt

**Status:** Complete
**Agent:** ontocoder
**Datum:** 260805-1356
**Herkunft:** Nutzerentscheid vom 260805 im Wortlaut: "Ich möchte die Belegung der Nav Tasten für Ordner doch anders: Pfeiltasten OHNE CMD, also einfach links rechts oben unten zum navigieren, link: raus aus dem Folder, rechts: rein in den Folder"
**Geänderte Datei:** `resources/default-keymap.toml` (einzige Datei des Auftrags)
**Neuer Defekt:** `issues/260805-1356_o_die-belegungspruefung-bindet-cmd-right-noch-an-das-oeffnen.md`

---

## Was geändert wurde

### Zwei Tastenlisten

| `id` | vorher | nachher |
|---|---|---|
| `oeffnen` | `["cmd+right"]` | `["right"]` |
| `ordner_aufwaerts` | `["cmd+left", "cmd+up"]` | `["left", "cmd+up"]` |

`cmd+up` bleibt stehen. Der Nutzer hat es am 260804 ausdrücklich als Finder-Gewohnheit gewollt, es ist eine andere Taste als die beiden Seitwärtspfeile, und der Auftrag berührt es nicht. `cmd+left` und `cmd+right` stehen in keiner Tastenliste mehr; die nackten Pfeile ersetzen genau sie.

`up` und `down` bleiben unberührt bei `auswahl_hoch` und `auswahl_runter`. Der Nutzer nennt sie in einem Atemzug mit links und rechts, meint aber die vorhandene Bewegung der Auswahl in der Liste. Nachgeprüft am geparsten Dateibestand: die vier Pfeil-Einträge sind `auswahl_hoch` mit `["up"]`, `auswahl_runter` mit `["down"]`, `oeffnen` mit `["right"]` und `ordner_aufwaerts` mit `["left", "cmd+up"]`.

Die Datei bleibt bei 56 Funktionen und 63 Kombinationen. Jede der beiden Listen tauscht eine Kombination gegen eine andere, keine kommt hinzu und keine fällt weg.

### Drei Stellen im Kommentartext

**1. Zeile 49 bis 52, die ab Werk freien Kombinationen.** Der Satz begründete die freie Eingabetaste damit, dass der Einstieg in einen Ordner "seither allein auf cmd+right" liegt. Er nennt jetzt beide Stationen: bis zum 260805 `cmd+right`, seither das nackte `right`. Die Aussage über die Eingabetaste selbst ist unverändert; sie bleibt frei.

**2. Zeile 62 bis 68, der Fokusvorbehalt.** Die Stelle, die der Auftrag zu Recht als Stolperstelle benennt. Sie ist geschärft und um einen eigenen Absatz gewachsen, statt nur ihr Beispiel zu tauschen; die Begründung steht unten unter "Der Satz über Textfelder".

**3. Zeile 202 bis 205, der Kommentar an `ordner_aufwaerts`.** Er begründete `cmd+left` als "Hauptweg" mit einem Vorbild aus der Norton-Reihe. Der Grund für die nackten Pfeile ist ein anderer und einfacher: eine Ordnernavigation ohne Zusatztaste ist schneller als eine mit. Die Richtungslogik der Seitwärtspfeile bleibt als zweites Argument stehen, weil sie von der Zusatztaste unabhängig ist. Der Kommentar hält außerdem fest, dass die Zusatztaste ersatzlos wegfällt, damit ein späterer Leser die Zwischenstufe vom 260804 nicht für den heutigen Stand hält.

`oeffnen` bekommt weiterhin keinen eigenen Kommentar. Der gemeinsame Grund steht wie bisher an `ordner_aufwaerts` und nennt `right` dort mit.

## Der Satz über Textfelder stimmt noch, wird aber zu knapp

Der Kopfkommentar sagte seit S13b, eine Kombination könne im Textfeld etwas anderes bedeuten als im Dateifenster, und belegte das mit `cmd+left`. Die Aussage ist nach der Umbelegung **nicht falsch**, und der Vorbehalt selbst greift unverändert: `Belegung::nachschlag` in `crates/krk-core/src/tasten/belegung.rs:404` wird vom Abgriff erst gefragt, nachdem der Fokus geklärt ist, also vor dem Nachschlag und ohne Rücksicht auf die Kombination. Eine nackte Taste trägt er damit genauso wie eine mit Zusatztaste.

Zu knapp wird er trotzdem, aus zwei Gründen. Erstens war `cmd+left` als Beispiel gewählt, weil es eine Kombination ist, und genau diese Kombination gibt es nicht mehr. Zweitens, und das wiegt schwerer: eine nackte Pfeiltaste in einem Eingabefeld ist der alltäglichste Fall überhaupt. Solange das Beispiel eine Zusatztaste trug, konnte ein Leser den Vorbehalt für eine Feinheit halten. Der Kommentar sagt jetzt ausdrücklich, dass der Vorbehalt vor dem Nachschlag fragt und nicht nach der Kombination, nennt `left` und `right` als den Alltagsfall und hängt die Pfadeingabe aus C2 daran: ohne den Vorbehalt bliebe ein getippter Pfad dort nicht zeichenweise begehbar.

## Die Sprungmarke ist berührt, aber nicht zu ändern

Der Kopfkommentar erklärt die Sprungmarke in Zeile 83 bis 85 als "Rückfall für jede Taste ohne Zusatztaste, die keiner Funktion zugeordnet ist". Der Satz **nennt keine Pfeiltasten als Beispiel** und bleibt deshalb unverändert; der Auftrag knüpft die Nachziehpflicht genau daran.

Er bleibt auch sachlich richtig. `Belegung::nachschlag` liefert `Nachschlag::Sprungmarke` nur, wenn keine Funktion trifft **und** die Maske leer ist (`crates/krk-core/src/tasten/belegung.rs:417-421`). Vor der Änderung fielen `left` und `right` dort hinein, nach ihr treffen sie eine Funktion und erreichen die Sprungmarke gar nicht mehr. Folgenlos ist das, weil die Sprungmarke nur Zeichen aufnimmt, die ein Dateiname tragen kann, und Pfeiltasten dort ohnehin herausfallen. Die Prüfung `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke` läuft unverändert durch.

## Die Prüfung fällt, und zwar im Code

`cargo test -p krk-core --test belegung` beendet **nicht** mit 0:

    test result: FAILED. 31 passed; 1 failed; 0 ignored

Fehlschlag ist `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` mit `cmd+right trifft keine Funktion` (`crates/krk-core/tests/belegung.rs:702`). Die Prüfung führt in Zeile 698 die Zeile `("cmd+right", Kommando::Oeffnen)`, und diese Kombination gibt es nach der Umbelegung nicht mehr.

Das Abnahmekriterium des Auftrags ist damit nicht erfüllbar, ohne `crates/` anzufassen, und der Auftrag schließt `crates/` ausdrücklich aus. Gemeldet als `issues/260805-1356_o_die-belegungspruefung-bindet-cmd-right-noch-an-das-oeffnen.md` für den `coder`, mit der einen Zeile, die zu ziehen ist.

**Es ist die zweite Auflage desselben Defekts.** `issues/260804-1214_c_die-belegungspruefung-bindet-return-noch-an-das-oeffnen.md` beschreibt denselben Vorgang mit `return` statt `cmd+right`. `oeffnen` ist innerhalb eines Tages dreimal gewandert, von `return` über `cmd+right` auf `right`, und jedes Mal ist dieselbe Prüfung an derselben hingeschriebenen Kombination gebrochen. Der Defektdatensatz hält das als Frage an den `coder` fest, ohne sie zu beantworten: die Zusage der Prüfung lautet nur, dass ein gebautes Kommando überhaupt an seiner ausgelieferten Taste hängt, und ein Beispiel, das die Kombination aus der Belegung liest statt sie zu wiederholen, trüge dieselbe Zusage.

Alle inhaltlichen Prüfungen der Auslieferungsbelegung laufen durch: `die_auslieferungsbelegung_ist_konfliktfrei`, `jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste`, `die_ab_werk_freien_kombinationen_kommen_nicht_vor`, `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke` und `zwei_funktionen_desselben_zustellers_auf_einer_kombination_bleiben_ein_konflikt`.

## Prüfungen

**Die nackten Pfeile waren frei, geprüft und nicht übernommen.** Vor der Änderung führte die Datei 63 Kombinationen; die vier, die `left` oder `right` als Teilzeichenkette enthalten, waren `cmd+right`, `cmd+left`, `ctrl+right` und `ctrl+left`. Kein Eintrag lautete `left` oder `right`. Verglichen wurde der vollständige Eintrag aus der geparsten Tastenliste, nicht die Teilzeichenkette: `ctrl+left` enthält `left` als Text und ist doch ein anderer Eintrag.

| Abnahme | Ergebnis |
|---|---|
| `grep -c '^\[\[funktion\]\]'` | 56, unverändert |
| `grep -F '"cmd+right"'` | 0 Treffer |
| `grep -F '"cmd+left"'` | 0 Treffer |
| `grep -F '"ctrl+left"'` | 1 Treffer (`bereich_verschmaelern`) |
| `grep -F '"ctrl+right"'` | 1 Treffer (`bereich_verbreitern`) |
| Kombinationen gesamt | 63, unverändert |
| Konflikt gleicher Zusteller | keiner |
| Doppelung über zwei Zusteller | genau `cmd+a`, wie ausgeliefert |
| `cargo test -p krk-core --test belegung` | **31 von 32**, siehe oben |

**Das Bündel ist gebaut und gestartet.** `cargo xtask bundle` läuft durch und signiert mit der Apple-Development-Identität; `target/KRK.app` steht. Dass `include_str!` die geänderte Datei eingezogen hat, ist am Binärbestand nachgesehen und nicht angenommen: `target/KRK.app/Contents/MacOS/krk` trägt `id = "oeffnen"` mit `tasten = ["right"]` und `ordner_aufwaerts` mit `tasten = ["left", "cmd+up"]`. Der Start ist geprüft, `target/KRK.app/Contents/MacOS/krk` läuft zwei Minuten ohne Abbruch und ohne eine Zeile auf stderr.

**Der Bedienversuch selbst steht aus.** Ob die nackten Pfeile im laufenden Bündel ein- und aussteigen und ob sie in der Pfadeingabe die Schreibmarke bewegen, ist von hier aus nicht prüfbar: es verlangt Tastendrücke in einem sichtbaren Fenster. Das ist die Stelle, an der die Änderung schiefgehen könnte, und sie bleibt offen. Der zweite Teil, die Schreibmarke in der Pfadeingabe, ist der Fokusvorbehalt aus S13, und der ist am 260804 am laufenden Bündel belegt worden (`history/260804-1309-s13-tastaturnavigation-vollstaendig.md`, Abnahmekriterium 4) — damals allerdings mit `cmd+left`, also mit einer Zusatztaste. Für die nackte Taste ist er abgeleitet und nicht gemessen.

## Was ausdrücklich nicht angefasst wurde

Der Auftrag begrenzt den Eingriff auf `resources/default-keymap.toml`. Nicht angefasst sind `crates/`, `xtask/`, die Plandatei und der Spec. Der Nutzer zieht sie selbst nach; die gefundenen Stellen stehen unten.

### Stellen, die die alte Belegung tragen

**Code, eine Stelle plus ein Kommentar.** Beide im Defektdatensatz beschrieben.

| Stelle | Was dort steht |
|---|---|
| `crates/krk-core/tests/belegung.rs:698` | `("cmd+right", Kommando::Oeffnen)` — der Fehlschlag |
| `crates/krk-core/tests/belegung.rs:153` | Kommentar: "nachdem der Einstieg in den Ordner auf cmd+right gewandert ist" |

Sonst nichts im Code. `Kommando::Oeffnen` und `Kommando::OrdnerAufwaerts` stehen in `crates/krk-core/src/tasten/belegung.rs:124` und `:126` und hängen in `crates/krk-ui/src/appkit/tabelle.rs:717-718`; beide kennen nur die Kennung, nie die Kombination. `crates/krk-core/src/tasten/parser.rs:178-179` führt `left` und `right` seit S11b in der Tastentabelle, und `crates/krk-ui/src/appkit/menue.rs:380-381` bildet sie auf die AppKit-Pfeilzeichen ab; beide Stellen sind von der Umbelegung unberührt. In `xtask/` kommt keine der beiden Kombinationen vor.

**Spec**, `planning/260802-1036_o_spec-navigator-geruest.md`:

| Zeile | Was dort steht |
|---|---|
| 136 | Der Absatz in C2, der die Belegung vom 260804-1122 ausschreibt: "Der Aufstieg trägt zwei Wege, Cmd+Links und Cmd+Auf … Der Einstieg trägt nur Cmd+Rechts." Die tragende Stelle. |
| 472 | Nennt in der Begründung zu den Bereichsbreiten, dass die Ordnernavigation auf Cmd+Links und Cmd+Rechts liegt. Die Entscheidung selbst über Ctrl+Links und Ctrl+Rechts ist davon unberührt. |

**Plan**, `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`:

| Zeile | Was dort steht |
|---|---|
| 14 | Der Nachzugsvermerk vom 260804-1122 im Kopf |
| 476, 515 | Diagrammkante `S11C -->|cmd+left und cmd+right stehen| S13` und ihre Erläuterung |
| 732 bis 733 | Die Vorher-Nachher-Tabelle von S11c |
| 736 bis 737, 740 | Begründung des Hauptwegs, Wortlaut des Nutzerentscheids, Konfliktprüfung gegen die damals 55 Kombinationen |
| 745 | Abnahmekriterium von S11c |
| 781, 783, 787 | S13: Auf- und Abstieg, der Fokusvorbehalt mit `cmd+left`/`cmd+right` als Beispiel, das Abnahmekriterium |
| 988, 993 | S18: `cmd+right` als Beispiel für ein Kommando, das in der Leiste nicht wirken darf |
| 1176 | Der Defektverweis auf den Fokusvorbehalt |

**Entscheidungsdatensätze**, zur Kenntnis und ohne Handlungsbedarf aus diesem Auftrag: `decisions/260804-1122_a_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md` (Zeilen 15, 22, 51) und `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md` (Zeile 49) nennen `cmd+left` und `cmd+right` in ihrer Begründung. Beide Antworten stehen unabhängig davon: die Bereichsbreiten liegen auf `ctrl+left` und `ctrl+right` und sind von den nackten Pfeilen nicht berührt, und der Zustellerbefund über `cmd+a` gilt für jede Kombination und jede nackte Taste gleichermaßen. Der Datensatz vom 260805-0000 nennt in Zeile 49 allerdings `cmd+left` als Beispiel für die Trennung im Textfeld, dieselbe Stelle, die im Kopfkommentar der Belegungsdatei nachgezogen wurde.

**Ein Entscheidungsdatensatz für die heutige Umbelegung fehlt.** Der Nutzerentscheid vom 260805 ist bisher nur in dieser Historiendatei und im Kommentar der Belegungsdatei festgehalten. Der vom 260804 hat keinen eigenen Datensatz bekommen, sondern steht im Plan und im Spec; das Nachziehen beider liegt beim Nutzer.

---

Herkunft: Nutzerauftrag vom 260805-1356, Umbelegung der Ordnernavigation auf die nackten Pfeiltasten. Nicht committet, wie beauftragt.
