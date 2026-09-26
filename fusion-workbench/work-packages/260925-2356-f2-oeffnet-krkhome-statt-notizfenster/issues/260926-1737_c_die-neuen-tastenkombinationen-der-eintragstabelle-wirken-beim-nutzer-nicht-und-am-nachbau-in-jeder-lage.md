Die neuen Tastenkombinationen der Eintragstabelle wirken beim Nutzer nicht, am Nachbau aber in jeder erzeugbaren Lage

---

Nutzerbefund am Bündel `target/KRK.app` (gebaut 17:00 aus `0768a19`, Code wie HEAD `85eac74`): die sieben Kombinationen `shift+cmd+return` (`eintrag_hinzufuegen`), `cmd+return` (`eintrag_bearbeiten`), `opt+cmd+up` und `opt+cmd+down` (`eintrag_hoch`, `eintrag_runter`), `shift+cmd+delete` (`eintrag_loeschen`), `shift+cmd+x` (`aufgabe_abhaken`) und `shift+cmd+p` (`pin_aendern`) tun nichts. Über das Menü „Home“ wirken dieselben Befehle. Die `keymap.toml` des Nutzers ist um 17:07 mit F1, `cmd+r` und „Fertig“ zurückgesetzt und führt alle sieben mit ihrer Kombination.

**Befund: am Nachbau wirken alle sieben, und kein Weg durch den Code trennt Tastendruck und Menü.** Gemessen am 260926 zwischen 17:15 und 17:35, macOS 15.7.9, MacBookPro15,1, mit einem vorübergehenden Messhaken am Ende von `oberflaeche_aufbauen` (Wortlaut unter `spikes/tastenweg-im-buendel/messhaken.rs`, danach entfernt), HOME auf einem Wegwerfordner im Scratchpad, KRK im Vordergrund (`key=true`), Tasten über `postEvent:atStart:` in die eigene Schlange, `--tasten-protokoll` an:

| Lage | Belegung | Ereignisbau | Ergebnis |
|---|---|---|---|
| `tasks.txt` Formatansicht, Ersthelfer `Eintragstabelle`, Zeile gewählt | Kopie der Nutzerdatei | `keyEventWithType…` | alle sieben nachgeschlagen, alle zulässig, Wirkung sichtbar (Zeile angelegt, Zelle offen, verschoben, gelöscht); `pin_aendern` richtig ausgegraut |
| dieselbe | Nutzerdatei ohne die acht neuen Blöcke, dann F1, `cmd+r`, `esc` bzw. `cmd+return` („Fertig“) | beides | nach dem Zurücksetzen trägt der Abgriff die neue Belegung, alle sieben wirken |
| dieselbe | Kopie der Nutzerdatei | `CGEventCreateKeyboardEvent` + `eventWithCGEvent:` (Zeichen aus der Tastaturbelegung, Marken `numericPad`/`function` an den Pfeilen) | wie oben |
| Kopien der echten `tasks.txt` und `notes.txt` | Kopie | CG | wie oben, in Aufgaben- und Notiztabelle |
| Zelle offen (`Zelleneditor` Ersthelfer) | Kopie | CG | alle wirken; `cmd+return` übernimmt die Zelle |
| Weg des Nutzers: F2, Filter, `down`, F4 | Kopie | NSEvent | F4 setzt den Ersthelfer in die `Eintragstabelle`, keine Zeile gewählt; alle sieben zulässig |
| danach Mausklick auf den Text einer Zeile | Kopie | NSEvent + Mausereignisse | Tabelle bleibt Ersthelfer, alle wirken |
| Release-Profil | Kopie | CG | wie oben |
| Rohansicht | Kopie | CG | alle nachgeschlagen, alle ausgegraut und abgewiesen, wie `form_passt` es will |

Das Protokoll zeigt je Tastendruck `funktion=eintrag_…` beziehungsweise `aufgabe_abhaken`/`pin_aendern`: Normalisierung, Zeichen (`x`, `p`, bei Return, Pfeil und Rückschritt keines, also Nachschlag über den Code) und Nachschlag stimmen, einen Doppelgänger in der Nutzerdatei gibt es nicht. F2 wirkt weiter (Tab `krkhome`, Fokus ins Dateifenster).

**Warum der Code die Unterscheidung nicht hergibt.** `validateMenuItem:` und `kommando_ausfuehren` fragen dieselbe `zulaessigkeit::zulaessig(kommando, self.lage())`, und ein Menüklick wirkt nur, wenn sie ja sagt (`krkKommando:` geht durch `kommando_ausfuehren`). Sagt sie ja, führt der Abgriff denselben Befehl aus; sagt sie nein, graut das Menü aus. Menü wirkt und Taste nicht, das geht nur, wenn der Tastendruck den Abgriff nicht mit denselben Werten erreicht wie am Nachbau, oder wenn Taste und Menü in verschiedenen Lagen versucht wurden. Andere Anwendungen mit globalen Tastenkürzeln laufen auf dem Gerät nicht, die Systemkürzel belegen keine der sieben, und `NSUserKeyEquivalents` ist weder global noch für `org.stalmann.krk` gesetzt.

**Zwei Lagen, in denen die Tasten tatsächlich nichts tun und das Menü ebenso nicht:**
1. Ein Klick auf das Ankreuzfeld einer Zeile nimmt den Ersthelferrang nicht an. Stand der Fokus vorher im Dateifenster, bleibt er dort (`fokus=Dateifenster`), und alle sieben sind abgewiesen und ausgegraut.
2. Nach dem Öffnen ist keine Zeile gewählt (`gewaehlt=-1`); Bearbeiten, Verschieben, Löschen und Abhaken melden dann nur in der Statuszeile.

Keine der beiden erklärt „Menü wirkt, Taste nicht“.

**Nicht gemessen, und das ist die Lücke:** ein körperlicher Tastendruck. Die Shell dieses Agenten hat weder Bedienungshilfen-Freigabe noch `CGPreflightPostEventAccess`; `CGEventPostToPid` kam bei KRK nicht an (`spikes/tastenweg-im-buendel/senden.swift`).

**Nächster Schritt, Nutzerarbeit.** KRK aus dem Terminal starten, `/Users/k1/Projects/productive/krk/target/KRK.app/Contents/MacOS/krk --tasten-protokoll`, `tasks.txt` im Editor öffnen, eine Zeile anklicken, die sieben Kombinationen drücken und danach dieselben Befehle aus dem Menü „Home“ wählen. Die Zeilen im Terminal entscheiden: fehlt eine Zeile, erreicht der Druck KRK nicht; steht `funktion=eintrag_…` da und es geschieht nichts, lag der Fokus beim Drücken woanders als beim Menüklick, und die Statuszeile sagt, was der Befehl geantwortet hat.

---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-1400_*_eine-zelle-der-eintragstabelle-laesst-sich-im-laufenden-buendel-nicht-bearbeiten.md, 260926-0112_*_was-tut-return-in-einer-notizzelle-und-welche-tasten-tragen-die-editoren.md, 260926-1506_*_plan-home-menue-und-einstellbarer-ort.md

---
Resolved: 260926-1805 — Bedienung und kein Defekt im Tastenweg: beim Nutzer war keine Zeile gewählt, und ohne gewählte Zeile antworten Bearbeiten, Verschieben, Löschen und Abhaken allein in der Statuszeile. Die Untersuchung hat am Nachbau alle sieben Kombinationen in jeder erzeugbaren Lage wirken sehen, mit Belegung, Normalisierung, Nachschlag und Zulässigkeit wie beim Menü, und keinen Weg im Code gefunden, der Taste und Menü trennt; sie hat dabei die zwei Lagen gefunden, in denen die Tasten scheinbar nichts tun, und beide sind jetzt behoben. Erstens wählt die Eintragstabelle nach jedem Neuladen ohne bestehende Auswahl die erste Zeile, also nach dem Öffnen, nach dem Neuladen am neuen Ort und nach einem `umkehren` ohne Auswahl; eine bestehende Auswahl bleibt, eine leere Tabelle wählt nichts, und `Neustand::auswahl` setzt sich nach einer Handlung weiter durch (reine Regel `auswahl_nach_neuladen` in `crates/krk-ui/src/appkit/eintragsansicht.rs`, gerufen aus `Eintragsansicht::zeilen_zeigen`). Zweitens holt ein Klick ins Ankreuzfeld nach dem Abhaken den Fokus in die Tabelle, wie ein Klick auf den Text der Zeile, über `makeFirstResponder:` des Fensters und damit durch die Überschreibung in `appkit/fenster.rs`; hält die Tabelle oder eine ihrer Zellen den Rang schon, bleibt er dort (`Eintragsansicht::fokus_in_die_tabelle`, `haelt_den_fokus`). Proben: `ohne_auswahl_ist_nach_dem_neuladen_die_erste_zeile_gewaehlt`, `nach_dem_neuladen_ist_eine_zeile_gewaehlt`, `der_klick_ins_ankreuzfeld_holt_den_fokus_in_die_tabelle`. `HowTo.md` nennt beides in den zwei Tabellenabschnitten. Ein körperlicher Tastendruck ist weiterhin nicht gemessen.
