# Welche Tasten bekommen Zoom und Seitensprung des PDF-Betrachters?

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-keymap.toml` (die eine Quelle jeder Tastenbelegung); `crates/krk-core/src/tasten/belegung.rs` (`Kommando::wirkungsbereich`, `Kommando::KENNUNGEN`); `crates/krk-ui/src/kommandos/fokus.rs` (`Fokus::Vorschau`); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md` (das Vorbild); `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/_*_circle.md` (Directive)

---

## Question

Der Nutzer hat am 260827-2028 für die PDF-Vorschau einen Betrachter mit Zoom, Seitensprung und Seitenzähler gewählt. Zoom und Seitensprung sind Befehle, und jeder Befehl in KRK hängt an einer Taste aus `resources/default-keymap.toml`; die Klärungsrunde hat nicht gefragt, an welcher. Das Vorschaufenster ist ein eigener Fokusbereich, die Befehle wirken nur dort, und mehrere naheliegende Kombinationen sind im Dateifenster oder im Editor schon vergeben.

Zu belegen sind fünf Befehle: Vergrößern, Verkleinern, Seite vor, Seite zurück, Sprung auf eine Seitenzahl. Ob der Sprung ein Blatt mit Eingabefeld öffnet oder die getippten Ziffern nimmt, hängt an derselben Frage.

## Options

1. **Die macOS-üblichen Kombinationen:** `cmd+plus` und `cmd+minus` für den Zoom, `cmd+0` für die Ausgangsgröße, Pfeil links und rechts blättern, `opt+cmd+g` öffnet ein Blatt „Gehe zu Seite".
   - Pro: Vorschau.app und Safari belegen genauso; der Nutzer muss nichts lernen.
   - Contra: `cmd+plus` und `cmd+minus` könnten mit einer späteren Schriftgrößenregel des Editors kollidieren (`circles/260812-1000-…/decisions/260812-1707_*_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-…`, offen).
   - Downstream: fünf neue `Kommando`-Werte mit Wirkungsbereich Vorschau; ein Blatt nach dem Muster von `appkit/blaetter/`.

2. **Nur Zoom und Blättern, kein Sprungblatt:** `cmd+plus`, `cmd+minus`, Pfeiltasten und `seite_hoch`/`seite_runter`; die Seitenzahl in der Statuszeile ist der einzige Seitenzähler, gesprungen wird durch Blättern.
   - Pro: kein Blatt, drei Kommandos weniger, keine Eingabefrage.
   - Contra: bei langen PDF-Dateien ist Blättern kein Sprung; die Antwort c der Klärungsrunde nennt den Seitensprung ausdrücklich.
   - Downstream: der Spec müsste „Seitensprung" auf Blättern verengen, und der Nutzer hat das nicht gewählt.

3. **Getippte Ziffern springen:** wie 1, nur ohne Blatt: mit Fokus in der PDF-Vorschau springen getippte Ziffern und Return auf die Seite, die Statuszeile zeigt die Eingabe mit.
   - Pro: kein Blatt, ein Handgriff.
   - Contra: das Tippen im Dateifenster filtert seit der Runde 10, und ein zweiter Tippmodus daneben braucht eine eigene Regel dafür, was `Esc` und Rückschritt tun; die Rückschrittregel ist sicherheitsrelevant (CLAUDE.md, „Die Rückschritt-Taste trägt zwei Bedeutungen").
   - Downstream: ein zweiter Eingabepuffer neben dem Filtertext, mit eigener Anzeige in der Statuszeile.

## Recommendation

Möglichkeit 1: sie deckt die gewählte Antwort c vollständig und baut auf Bekanntem auf. Die Kollision mit einer späteren Schriftgrößenregel ist eine Frage jener Runde, nicht dieser.

## Status

Open. Zu beantworten spätestens beim Schärfen (portfolio-activation), bevor der Plan die Kommandos anlegt.
