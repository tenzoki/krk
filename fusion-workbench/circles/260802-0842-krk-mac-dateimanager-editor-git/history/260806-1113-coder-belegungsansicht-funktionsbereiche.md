# Belegungsansicht nach Funktionsbereichen gegliedert — coder

**Status:** Complete
**Agent:** coder
**Auftrag:** Nutzerauftrag vom 260806, `issues/260806-1054_p_belegungsansicht-gruppiert-nach-funktionsbereich.md` — die Belegungsansicht (F1, S20) zeigt ihre Zeilen strukturiert nach Funktionsbereich statt als flache Liste.

## Die gewählte Gliederung

Neun Bereiche, definiert als Aufzählung `Funktionsbereich` in `crates/krk-ui/src/belegungsmodell.rs`, in Anzeigereihenfolge: **Dateilisting** (20 Funktionen: Bewegung, Navigation, Markierung, Sortierung, Sichtbarkeit, Zwischenablage-Sprung), **Dateioperationen** (10, samt Terminal-Befehl C11), **Tabs** (4), **Vorschau** (2: Vorschau umschalten, Zwischenablage ansehen), **Leiste und Fokus** (8), **Fenster** (6: Fensterwechsel, zweites Fenster, Ein-/Ausblenden, Breiten), **Anwendung** (2: Belegungsansicht, Beenden), **Textbefehle** (4 Menü-Zusteller), **Editor** (1: der reservierte F4-Eintrag, sichtbar unter eigener Überschrift).

Geprüfte Bestandsgliederungen, keine trug allein: die Kommentarstruktur der Belegungsdatei ist nicht maschinenlesbar und mischt in der Norton-Reihe Vorschau, Dateioperationen und Editor; die `Kommando`-Reihenfolge kennt die fünf Funktionen ohne Kommando nicht; der `Wirkungsbereich` wirft mit `Ueberall` Fenster-, Fokus- und Anwendungsbefehle in einen Topf. Die Zuordnung steht deshalb an **einer** Stelle im Modell: `bereich(kennung)` — vollständige Fallunterscheidung über `Kommando` ohne Auffangzweig (Übersetzer erzwingt die Einordnung eines neuen Kommandos, dasselbe Muster wie `Kommando::wirkungsbereich`), daneben namentlich die fünf Funktionen ohne Kommando (bearbeiten, text_*). `resources/default-keymap.toml` ist unberührt.

## Geänderte Dateien

- `crates/krk-ui/src/belegungsmodell.rs` — `Funktionsbereich` (9 Werte, `ALLE`, `name()`), `bereich()`, `bereich_des_kommandos()` (const, ohne Auffangzweig), Zeilenliste `Vec<Zeile>` (Überschrift | Funktion) via `gliederung()`; alle stellenbasierten Methoden lösen über sie auf; neu `ueberschrift(stelle)` und `erste_funktionszeile()`; `zuruecksetzen` baut die Zeilen neu. Innerhalb eines Bereichs bleibt die Dateireihenfolge. Eine Funktion ohne Bereich bricht den Bau laut ab statt still zu verschwinden. Keine objc2-Abhängigkeit.
- `crates/krk-ui/src/appkit/belegungsansicht.rs` — Überschriften als Gruppenzeilen (`tableView:isGroupRow:`, Zellenansicht ohne Spalte, fett), nicht auswählbar (`tableView:shouldSelectRow:`, Pfeiltasten überspringen sie), Anfangsauswahl auf der ersten Funktionszeile (Zeile 0 ist jetzt Überschrift). Bedienung aus S20 unverändert.

Prüfungen: 12 in `belegungsmodell` (vorher 8) — neu `jede_kennung_hat_einen_funktionsbereich` (Vollständigkeit gegen die Auslieferungsbelegung), `die_zeilen_sind_nach_bereichen_gegliedert` (Reihenfolge, keine Doppelung, erste Funktionszeile), `eine_ueberschrift_nimmt_keine_zuweisung_an`; erweitert: eine Zeile je Funktion (+ Eindeutigkeit der bloßen Namen), Fn+-Prüfung auch über Überschriften, F4-Eintrag steht unter der Überschrift "Editor".

## Prüfung am Bündel

Temporäre Prüfsonde (KRK_PROBE, `postEvent:atStart:` über die eigene Ereignisschlange, danach restlos entfernt), Schirmabzüge im Sitzungs-Scratchpad (gr-a bis gr-e):

1. F1 → Ansicht offen, Gruppenüberschrift "Dateilisting" über den Zeilen, Auswahl auf der ersten Funktionszeile.
2. 24× Pfeil ab → Auswahl auf "In den Papierkorb räumen" unter "Dateioperationen"; die Überschrift wurde beim Wandern übersprungen (Zeile 21), "Dateilisting" schwebt beim Rollen oben (floatende Gruppenzeile). Papierkorb und Endgültig löschen als zwei Zeilen sichtbar.
3. Zuweisen (Leertaste) + f5 → Konfliktmeldung nennt beide Funktionen: "… gehoert schon der Funktion 'In das andere Fenster kopieren' (kopieren) und laesst sich nicht zusaetzlich der Funktion 'In den Papierkorb räumen' (in_papierkorb) zuweisen" — die Zeilenzuordnung stimmt über Gruppengrenzen, die Bedienung aus S20 ist unverändert.
4. 40× Pfeil ab → Listenende zeigt "Anwendung", "Textbefehle" und "Editor" mit "Bearbeiten (reserviert für den Editor)" als letzter, auswählbarer Zeile unter eigener Überschrift.
5. Kein "Fn+" in den Ansichtstexten (Bündel sichtbar + Prüfung).

Prüfstand: `keymap.toml` wurde nicht geschrieben (Konflikt abgelehnt, keine Änderung), `session.toml` inhaltlich unverändert (diff leer). Ablage sauber.

**make check:** alle vier grün (build, test, clippy -D warnings, fmt). Bündel signiert gebaut.

## Nicht getan (Grenzen des Auftrags)

Issue-Marker `_p_` → `_c_` und Commit übernimmt der Auftraggeber. `resources/default-keymap.toml` unberührt.
