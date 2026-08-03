# Wie steuert KRK aus Rust eine Schnittstelle an, die es erst ab macOS 26 gibt?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1134_i_sprache-und-ui-werkzeugkasten.md` (Zeile 59, die Zusage, die diese Frage erzeugt), `circles/260802-0842-krk-mac-dateimanager-editor-git/analyses/260802-1134-sprache-und-ui-werkzeugkasten.md` Befund 6, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` Schritte S1 und S5

---

## Question

Der Technologiedatensatz sagt zu: "KRK unterstützt macOS 26 und bleibt rückwärtskompatibel bis macOS 15. Das Mindest-Zielsystem ist macOS 15; Schnittstellen ab macOS 26 werden zur Laufzeit abgefragt und haben jeweils einen Ersatzweg." Wie diese Laufzeitabfrage in Rust aussieht, hält der Datensatz offen. Der Nutzer hat den Punkt ausdrücklich als zweite von zwei vor dem Plan zu prüfenden Annahmen benannt.

Swift hat für diesen Fall eine Sprachform: `if #available(macOS 26, *)`. Der Übersetzer kennt die Verfügbarkeitsangabe jeder Schnittstelle, warnt bei einem ungeschützten Aufruf und bindet neue Symbole schwach. Rust hat davon nichts, und `objc2` kann das nicht nachreichen, weil es eine Bibliothek ist und kein Übersetzer.

**Geprüfter Stand der Bibliothek, 260802-1428.** Die Frage wird im Vorhaben `madsmtm/objc2` als Ausgabe 266 geführt, sie ist offen, und der Autor beschreibt das Problem in eigenen Worten: "We should have some way of doing the same as `@available` in Objective-C; however, since we are not a compiler like `clang` is, this is quite tricky!" Eine Umsetzung ist als Änderungsvorschlag 212 in Arbeit und noch nicht eingegangen. Geplant sind vier Fähigkeiten: eine Übersetzungszeit-Prüfung über ein Attribut `#[cfg_available(...)]`, eine Zusicherung zur Laufzeit im Fehlersuchbau, ein Makro `available!(...)` für die bedingte Ausführung und eine Berücksichtigung beim Überschreiben von Methoden mit bekannter Verfügbarkeitsangabe. Der Autor hält fest, dass es angesichts der heutigen Grenzen von Rust vermutlich auf die Laufzeitprüfung hinauslaufen wird. Quelle: https://github.com/madsmtm/objc2/issues/266, am 260802 abgerufen.

**Warum die Frage jetzt gestellt und nicht jetzt beantwortet wird.** Runde 1 braucht keine Schnittstelle über macOS 15 hinaus; die Prüfung steht im Plan unter `## Offene Fragen`. Die Zusage aus dem Technologiedatensatz gilt aber für den ganzen Circle, und Editor und Git-Anbindung folgen in denselben. Die Frage muss beantwortet sein, bevor eine spätere Runde die erste neuere Schnittstelle anspricht, nicht vorher.

## Options

1. **Laufzeitabfrage von Hand, ohne Bibliotheksunterstützung** — `NSProcessInfo.isOperatingSystemAtLeastVersion` für die Systemversion, `respondsToSelector:` für einzelne Methoden, `NSClassFromString` für ganze Klassen, `dlsym` für C-Funktionen. Jede Fundstelle trägt ihren Ersatzweg selbst.
   - Pro: funktioniert heute, ohne auf die Bibliothek zu warten. Deckt alle vier Arten von Schnittstellen ab, die vorkommen können.
   - Contra: nichts erzwingt die Prüfung. Ein vergessener Schutz fällt erst auf dem älteren System auf, und dort als Absturz. Genau die Warnung, die Swifts Übersetzer gibt, fehlt.

2. **Schwaches Binden über den Binder, plus Laufzeitabfrage** — zusätzlich zu Möglichkeit 1 die betroffenen Rahmenwerke mit `-weak_framework` binden, sodass ein auf macOS 15 fehlendes Symbol den Start nicht verhindert.
   - Pro: nötig, sobald eine neuere **C-Funktion** angesprochen wird. Ohne schwaches Binden scheitert der Prozessstart auf dem älteren System, und zwar bevor irgendeine Laufzeitprüfung greifen könnte.
   - Contra: eine Binder-Einstellung je Rahmenwerk, die niemand sieht, der nur den Rust-Quelltext liest. Objective-C-Methoden und -Klassen brauchen sie nicht, weil die Nachricht erst beim Senden aufgelöst wird.

3. **Auf die Bibliotheksunterstützung warten und die Frage bis dahin vermeiden** — keine Schnittstelle über macOS 15 hinaus ansprechen, bis Ausgabe 266 gelöst ist.
   - Pro: kein eigener Mechanismus, kein Wartungsaufwand.
   - Contra: bindet die Runden an einen fremden Zeitplan. Der Änderungsvorschlag 212 ist seit Längerem in Arbeit und trägt kein Datum.

4. **Eine eigene, schmale Hülle im Projekt** — eine Funktion `ab_macos(major, minor) -> bool` und eine Konvention, dass jede Fundstelle einer neueren Schnittstelle durch sie geht und ihren Ersatzweg unmittelbar daneben führt.
   - Pro: eine Stelle, an der die Systemversion abgefragt wird, statt verstreuter Vergleiche. Ablösbar, sobald die Bibliothek nachzieht.
   - Contra: erzwingt immer noch nichts. Löst das Problem des schwachen Bindens für C-Funktionen nicht mit.

## Constraints

- Das Mindest-Zielsystem bleibt macOS 15, solange das `MacBookPro15,1` von 2018 das Abnahmegerät ist. Das Gerät erhält macOS 26 nicht.
- Der Bau setzt `MACOSX_DEPLOYMENT_TARGET=15.0` und weist es am Binärformat nach (`vtool -show-build-version` meldet `minos 15.0`). Beides ist im Plan als Schritt S1 und S5 festgelegt und gilt unabhängig von der Antwort auf diese Frage.
- Die Maxime "supersimpel" wirkt als Ausschlussgrund. Eine Antwort, die je Schnittstelle eine eigene Sonderregel mit eigenem Rückfallweg erzeugt, verfehlt sie; ein Ersatzweg je neuerer Schnittstelle ist dagegen genau das, was der Technologiedatensatz zusagt, und kein Verstoß.
- **Runde 1 ist nicht betroffen und wird von dieser Frage nicht blockiert.** Der Plan hat die von ihm benutzten Schnittstellen durchgesehen; die höchste Untergrenze liegt bei macOS 14.0 (`CADisplayLink` an einer `NSView`), alle übrigen deutlich darunter.

## Recommendation

**Wir empfehlen Möglichkeit 1 zusammen mit Möglichkeit 2, sobald die erste neuere Schnittstelle tatsächlich gebraucht wird, und keine Festlegung vorher.**

Die beiden gehören zusammen und sind keine Alternativen: die Laufzeitabfrage entscheidet, ob aufgerufen wird, das schwache Binden entscheidet, ob der Prozess überhaupt startet. Welche der beiden Hälften nötig ist, hängt von der Art der Schnittstelle ab, und die kennt heute niemand, weil die betreffende Runde noch nicht zugeschnitten ist. Eine Festlegung ohne diesen Zuschnitt wäre eine Vorwegnahme.

Möglichkeit 4 empfehlen wir dann als Zugabe, nicht als eigene Antwort: eine einzige Stelle für die Versionsabfrage kostet nichts und macht die Fundstellen auffindbar, sobald es mehr als eine gibt.

Möglichkeit 3 empfehlen wir nicht. Der Änderungsvorschlag 212 trägt kein Datum, und die Zusage im Technologiedatensatz an einen fremden Zeitplan zu hängen, verschiebt das Risiko, ohne es zu verkleinern.

Die Abwägung stützt sich auf den am 260802 geprüften Stand der Ausgabe 266 und auf die dokumentierten Mechanismen von `dyld` und der Objective-C-Laufzeitumgebung. Eine eigene Messung gibt es nicht. Die Entscheidung liegt beim Nutzer und ist nicht eilig.

---
Answered:
Implemented:
Deferred:
Superseded by:
