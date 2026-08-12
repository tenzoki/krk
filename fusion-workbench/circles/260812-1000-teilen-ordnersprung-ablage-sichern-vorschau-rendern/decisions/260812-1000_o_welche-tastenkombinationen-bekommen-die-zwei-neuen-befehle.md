# Welche Tastenkombinationen bekommen Teilen und Ordnersprung?

---
**Domain:** code
**Status:** open
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `resources/default-keymap.toml`; `crates/krk-core/src/tasten/belegung.rs:546` (`Kommando::KENNUNGEN`, 73 Kennungen); `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/_*_circle.md` (die Runde, die `opt+cmd+c` und `shift+cmd+c` vergeben hat)

---

## Question

Die Runde legt zwei neue Befehle an, Teilen und den Sprung in den Ordner der angezeigten Datei, und beide brauchen eine Kombination. Frei ist nicht viel: die Auslieferungsbelegung führt 79 Funktionen mit 85 Kombinationen, und von den Buchstaben unter `cmd` sind zwanzig vergeben.

Die Frage ist zu stellen, weil die Belegung dieses Projekts eine erkennbare Reihenordnung trägt und eine falsch einsortierte Kombination sie zerredet. Am Bestand vom 260812-1000 abgelesen:

- **`shift+cmd+X` wirkt auf Eintrag, Auswahl und Fokus.** `shift+cmd+c` kopiert den Pfad des Eintrags, `shift+cmd+a` hebt die Markierung auf, `shift+cmd+i` kehrt sie um, `shift+cmd+d`, `shift+cmd+l`, `shift+cmd+y` und `shift+cmd+e` setzen den Fokus.
- **`opt+cmd+X` wirkt auf Ordner und Bereiche.** `opt+cmd+c` kopiert den Pfad des angezeigten Ordners, `opt+cmd+l`, `opt+cmd+b`, `opt+cmd+left` und `opt+cmd+right` schalten Bereiche ein und aus, `opt+cmd+g` springt zur Adresse aus der Zwischenablage.
- **`ctrl+cmd+X` ist die zweite Stufe eines vorhandenen Befehls.** `ctrl+cmd+n` legt eine Datei an neben `shift+cmd+n` für den Ordner, `ctrl+cmd+u` benennt im Stapel um neben `shift+cmd+u`, `ctrl+cmd+r` ersetzt alle neben `shift+cmd+r`.

Frei sind unter anderem `shift+cmd+` b, f, j, m, o, p, q, s, t, x; `opt+cmd+` a, f, h, i, j, k, m, n, o, p, q, r, s, t, u, v, w, x, y, z; dazu die Funktionstasten `f2`, `f9`, `f11` und `f12`.

Die Frage hält keinen Planschritt auf und bindet zwei.

## Options

1. **Der vorhandenen Reihenordnung folgen: `shift+cmd+s` für Teilen, `opt+cmd+o` für den Ordnersprung.** Teilen wirkt auf die betroffenen Einträge und gehört damit in die `shift+cmd`-Reihe; der Ordnersprung liefert einen Ordner und gehört in die `opt+cmd`-Reihe, unmittelbar neben `opt+cmd+c`, das den Pfad desselben Ordners kopiert.
   - Folge: die Belegungsansicht und die Markdown-Ausgabe der Runde 3 zeigen die beiden neuen Befehle in derselben Gruppe wie ihre Verwandten. Wer `opt+cmd+c` kennt, findet `opt+cmd+o` ohne Nachschlagen.
   - Preis: `shift+cmd+s` heißt auf dem Mac üblicherweise „Sichern unter", und der Editor dieses Projekts belegt `cmd+s` mit „Sichern". Die beiden liegen einen Umschalter auseinander und bedeuten nichts miteinander. Wer die Nähe für zu eng hält, nimmt `shift+cmd+f` für „Freigeben", den Namen, den macOS dem Vorgang auf Deutsch gibt; dann steht die Nachbarschaft zu `cmd+f` für „Suchen" im Weg, was der schwächere Konflikt ist.

2. **Die Funktionstastenreihe erweitern: `f2` für Teilen, `f9` für den Ordnersprung.** Beide sind heute unbelegt.
   - Folge: KRK bekommt zwei weitere Norton-Tasten, und die Befehle sind ohne Umschalter erreichbar. Auf einem Mac mit eingeschalteten Systemfunktionen der oberen Reihe braucht der Nutzer dafür die `fn`-Taste, was den Vorteil auffrisst.
   - Preis: in der Norton-Tradition tragen `f2` und `f9` andere Bedeutungen, nämlich Benutzermenü und Menüzeile. Die Zwei-Wege-Regel aus C3 der Runde 1 gilt den sechs Funktionen der oberen Reihe; eine siebte und achte Funktionstaste ohne zweiten Weg stünde daneben, und die Runde 2 hat mit `f4` bereits einen solchen Einzelfall angelegt.

3. **Beide neuen Befehle nebeneinander in die `opt+cmd`-Reihe: `opt+cmd+s` und `opt+cmd+o`.** Die zwei Neuzugänge dieser Runde stehen dann als Paar beieinander.
   - Folge: kein Konflikt mit einer Systembedeutung, und die Belegungsansicht zeigt sie gemeinsam.
   - Preis: die Reihenordnung wird gebrochen. Teilen wirkt auf Einträge, nicht auf Ordner, und sitzt danach in der Ordnerreihe. Die Ordnung ist nirgends aufgeschrieben und lebt allein davon, dass sie eingehalten wird; die erste Ausnahme kostet sie.

## Constraints

- Jedes neue Kommando braucht eine Zeile in `Kommando::KENNUNGEN`, in `Kommando::wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs:681`) und in `bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs:166`). Alle drei sind vollständige Fallunterscheidungen ohne Auffangzweig und halten den Bau an, solange eine Stelle fehlt.
- Die Belegung hat genau eine Quelle, `resources/default-keymap.toml`. Eine Kombination, die dort nicht steht, gibt es nicht.
- Die Belegungsansicht und die Markdown-Ausgabe der Runde 3 lesen dieselbe Quelle und zeigen die Wahl unverändert an. Eine Kombination, die dort schlecht aussieht, sieht überall schlecht aus.

## Recommendation

**Wir empfehlen Möglichkeit 1**, weil sie die einzige ist, die die Reihenordnung nicht kostet. Die Ordnung ist der Grund, aus dem ein Nutzer nach vier Runden noch raten kann, wo ein Befehl liegt; sie einmal zu brechen ist billig und in der Summe teuer.

Zum Konflikt bei `shift+cmd+s`: er ist real, aber schwächer, als er aussieht. „Sichern unter" ist in KRK an keiner Stelle belegt und in keiner Runde vorgesehen, weil der Editor eine geöffnete Datei sichert und keine unter neuem Namen anlegt. Der Nutzer entscheidet, ob ihm die Nähe zu `cmd+s` genügt, um auf `shift+cmd+f` auszuweichen.

---
Answered:
Implemented:
Deferred:
Superseded by:
