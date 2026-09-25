# Shaper, anticipated-circle: vier Tastenbefehle für Pfade, Öffnen und Cmd+W

**Datum:** 2026-08-11
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete

## Auftrag

Ein Entwurf des Nutzers, übergeben über `/fusion:direct`: vier tastaturgesteuerte Befehle für KRK, die die vorhandene Kommando-Maschinerie erweitern. Ordnerpfad kopieren, Eintragspfad kopieren, Cmd+W über den Tabbereich hinaus wirksam machen und eine Datei mit dem Standardprogramm öffnen, ausgelöst per Doppelklick und per Tastenkombination. Der Auftrag verlangte ausdrücklich, den Circle auch dann anzulegen, wenn Fragen offen bleiben.

## Was am Baum geprüft wurde

Der Entwurf nannte einen Bestand als geprüft, und drei seiner Zahlen sind hier nachgezählt worden: `resources/default-keymap.toml` führt 71 Funktionen, die Aufzählung `Kommando` 65 Varianten, `Wirkungsbereich` sieben Werte. Alle drei stimmen.

Zwei Feststellungen darüber hinaus korrigieren den Entwurf:

Die Belegungsansicht ist **kein Fenster**, sondern ein Blatt am Hauptfenster (`crates/krk-ui/src/appkit/belegungsansicht.rs:3`). Dass Cmd+W dort nicht wirkt, liegt nicht am Wirkungsbereich, sondern an `waehrend_blatt_erlaubt` (`crates/krk-ui/src/kommandos/operationen.rs:208`), das bei stehendem Blatt allein `Kommando::Abbrechen` durchlässt. Die Lücke, die der Entwurf als eine beschreibt, sind zwei mit zwei verschiedenen Ursachen, und eine davon ist eine bewusste Sperre, die in der Editor-Runde schon einmal für einen Defekt gehalten wurde und keiner war.

Für "den Pfad des markierten Eintrags" gibt es im Baum bereits eine Regel: `betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:157`) gibt der Markierung den Vorrang vor der Auswahl und trägt sie einmal für alle vier Dateioperationen. Der Entwurf ließ offen, ob der Pfadkopierer sie erbt.

Weiter geprüft: `auswahl_oeffnen` (`crates/krk-ui/src/appkit/tabelle.rs:955`) filtert auf `ist_ordner()`, und im Dateifenster steht keine Doppelklick-Behandlung; der Einstieg in einen Ordner liegt auf dem nackten Rechts-Pfeil, die Eingabetaste ist ab Werk frei. Die Zwischenablage ist reine Quelle, und ihr Modulkopf sagt das ausdrücklich zu (`crates/krk-ui/src/appkit/zwischenablage.rs`); diese Runde bricht die Zusicherung und schreibt den Kopf mit um. Cmd+C und Cmd+V sind über `gehalten_von = "menue"` belegt, was nach dem Vorgang von Cmd+A kein Konflikt sein muss.

## Was angelegt wurde

- Circle `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/` mit dem Datensatz `_a_circle.md` (vorgesehen) und den sechs Artefaktverzeichnissen.
- Vier Entscheidungsdatensätze in `decisions/` dieses Circles, alle offen: die Reichweite von Cmd+W, was der Pfadkopierer bei stehender Markierung nimmt, was ein Doppelklick auf einen Ordner tut, und welche vier Kombinationen ab Werk gelten. Jeder trägt zwei bis drei Möglichkeiten und eine Empfehlung.

Kein Spec. Ein vorgesehener Circle hält eine Absicht fest, und der Spec entsteht bei der Aktivierung.

## Abweichung von der Vorgabe für die Historie

Die Vorgabe des Agentenprompts schreibt die Historie nach `$OUT_HISTORY`. Der Auflöser zeigt dort auf den laufenden Circle `260809-2040-tastenbelegung-als-markdown-in-downloads`, mit dessen Directive diese Sitzung nichts zu tun hat. Nach der Herkunftsregel gehört die Datei in den Circle, dessen Directive sie hervorgebracht hat, und das ist der hier angelegte. Sie liegt deshalb in seinem `history/`, und das Kopffeld `Active session history:` seines Datensatzes nennt sie.
