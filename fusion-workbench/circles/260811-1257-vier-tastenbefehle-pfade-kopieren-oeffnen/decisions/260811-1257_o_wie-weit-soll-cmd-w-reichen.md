# Wie weit soll Cmd+W reichen: nur über den Fokus hinaus oder auch durch ein stehendes Blatt?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md

---

## Question

Der Entwurf verlangt, dass Cmd+W "in jedem Fenster wirkt", und nennt zwei Beobachtungen: es wirkt nicht in der Belegungsansicht und nicht in Blättern. Beide Beobachtungen stimmen, haben aber verschiedene Ursachen, und eine der beiden ist eine bewusste Sperre.

Die erste Ursache ist der Wirkungsbereich. `Kommando::TabSchliessen` trägt `Wirkungsbereich::Tabbereich` (`crates/krk-core/src/tasten/belegung.rs`, der Zweig der vier Tabbefehle), wirkt also allein mit dem Fokus in einem Dateifenster oder in der Vorschau. In der Leiste und im Editor löst Cmd+W deshalb nichts aus.

Die zweite Ursache ist die Blattsperre. Die Belegungsansicht ist kein eigenes Fenster, sondern ein Blatt am Hauptfenster (`crates/krk-ui/src/appkit/belegungsansicht.rs:3`), und solange irgendein Blatt steht, lässt `waehrend_blatt_erlaubt` (`crates/krk-ui/src/kommandos/operationen.rs:208`) allein `Kommando::Abbrechen` durch. Der Anwendungsdelegierte setzt das durch, damit kein Befehl hinter einer stehenden Rückfrage im Ordner dahinter wirkt. Cmd+W durch diese Sperre zu lassen heißt, eine zweite Taste neben `esc` zu vergeben, und es heißt nicht, eine vergessene Zeile nachzutragen.

## Options

1. **Nur die Fokuslücke schließen.** Cmd+W wirkt zusätzlich mit dem Fokus in der Leiste und im Editor und schließt dort den aktiven Tab der aktiven Fensterseite (`Fenstermodell::aktiv()`, `crates/krk-ui/src/fenstermodell.rs:318`). Die Blattsperre bleibt, Blätter schließt weiter `esc`.
   - Pro: eine einzige Änderung, ein neuer oder erweiterter Wirkungsbereich, keine Berührung der Sperre, die vor Befehlen hinter Rückfragen schützt.
   - Contra: in der Belegungsansicht bleibt Cmd+W wirkungslos, und genau das war eine der zwei Beobachtungen im Entwurf.
2. **Fokuslücke schließen und Cmd+W zusätzlich durch die Blattsperre lassen**, wo es dasselbe tut wie `esc`: das stehende Blatt abbrechen.
   - Pro: deckt beide Beobachtungen des Entwurfs; Cmd+W schließt dann tatsächlich das, was gerade vor dem Nutzer steht.
   - Contra: `waehrend_blatt_erlaubt` bekommt einen zweiten erlaubten Befehl und ist damit keine Ein-Zeilen-Regel mehr. Ein Abbruch über zwei Tasten ist außerdem in der Belegungsansicht mehrdeutig, weil dort die Eingabetaste auf "Fertig" liegt und `esc` je nach Aufnahmezustand zwei verschiedene Dinge tut.
3. **Wie 1, und im Editor schließt Cmd+W die geöffnete Datei statt eines Tabs.**
   - Pro: entspricht der Erwartung aus Textprogrammen, wo Cmd+W das Dokument schließt.
   - Contra: eine Taste mit zwei Bedeutungen, abhängig vom Fokus. Der Editor kennt drei Anlässe der Nachfrage vor Verlust eines ungesicherten Standes (C4 der Editor-Runde); ein vierter Anlass käme dazu und wäre neue Verhaltensfläche statt einer geschlossenen Lücke.

## Constraints

- Cmd+W bleibt auf `tab_schliessen`, `fenster_schliessen` bleibt auf Shift+Cmd+W. Der Nutzer hat am 260811-1250 festgelegt, dass die Entscheidung `260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md` nicht umgekehrt wird.
- C3 schließt zwei Funktionen auf einer Kombination aus.
- Ein neuer Wert in `Wirkungsbereich` verlangt je eine Zeile in `Kommando::wirkungsbereich`, in `Wirkungsbereich::beschriftung` und in der Ausgabe der Tastenbelegung als Markdown.

## Recommendation

Option 1. Die Blattsperre ist keine Lücke, sondern die Regel, die einen Befehl hinter einer stehenden Rückfrage verhindert; sie wurde in der Editor-Runde einmal für einen Defekt gehalten und war keiner (`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1102_*`). Option 1 schließt die Lücke, die der Entwurf benennt, ohne diese Regel anzufassen.

---
Answered:
Implemented:
Deferred:
Superseded by:
