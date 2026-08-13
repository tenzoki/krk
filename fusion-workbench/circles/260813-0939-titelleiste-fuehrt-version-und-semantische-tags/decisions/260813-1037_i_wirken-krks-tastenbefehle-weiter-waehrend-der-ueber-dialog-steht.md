# Wirken KRKs Tastenbefehle weiter, während der Über-Dialog steht?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_a_bekommt-krk-einen-eintrag-ueber-krk-im-anwendungsmenue.md` (die beantwortete Frage, aus der diese folgt); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_*_die-blattregel-sieht-den-freigabedialog-nicht.md` (derselbe blinde Fleck, offen); `crates/krk-ui/src/appkit/anwendung.rs` (`blatt_steht`, `lage`); `crates/krk-ui/src/appkit/ereignisse.rs` (`ersthelfer_gehoert_appkit`)

---

## Question

Der Nutzer hat am 260813-1010 den Standard-Über-Dialog von AppKit gewählt, und der ist kein Blatt. Die Zulässigkeitsregel von KRK kennt aber nur zwei Arten, ein Fenster im Vordergrund zu bemerken: `blatt_steht` fragt `NSWindow::attachedSheet` am Hauptfenster, und `ersthelfer_gehoert_appkit` fragt, ob der Ersthelfer des Schlüsselfensters eine Textklasse ist. Ein freistehendes Panel fällt durch beide Fragen hindurch. Steht der Über-Dialog vorn, kann deshalb ein Tastendruck einen Befehl im Fenster dahinter auslösen, und `F5` startete dann eine Kopieroperation, `delete` räumte in den Papierkorb.

Ob es wirklich eintritt, ist am Baum nicht zu entscheiden. Der Ausgang hängt daran, welchen Ersthelfer AppKit im Panel einsetzt: setzt es eines der `NSTextField`, die Name und Version tragen, antwortet `ersthelfer_gehoert_appkit` mit ja und die Tasten gehören zufällig AppKit; setzt es die Inhaltsansicht, antwortet es mit nein und der Befehl läuft. **Gemessen ist keines von beidem**, und gemessen werden kann es nur am laufenden Bündel im Vordergrund, also in Nutzerarbeit. Die Frage muss deshalb vor dem Bau entschieden werden und nicht nach der ersten Beobachtung.

Diese Runde legt die Frage nicht an, sie legt eine zweite Stelle an, an der sie eintritt. Die erste ist der Freigabedialog der Runde 6, und der zugehörige Defekt ist offen.

## Options

1. **Der Dialog kommt, und die Runde erbt den blinden Fleck.** Kein Eingriff in die Zulässigkeitsregel; der offene Defekt zum Freigabedialog trägt den Fall mit.
   - Pro: die Antwort vom 260813-1010 bleibt so billig, wie sie gewählt wurde. Ein Menüeintrag ohne Kürzel, ein Selektor, sonst nichts.
   - Contra: die Runde stellt wissentlich eine zweite Fläche auf, hinter der Befehle wirken, und dieses Projekt schreibt den Fehler lieber auf, als ihn zu bauen. Der Über-Dialog steht zudem länger vorn als der Freigabedialog: der Nutzer liest darin, statt ihn wegzuklicken.
2. **Die Runde schließt die Lücke einmal und allgemein.** Die Zulässigkeitsregel bekommt die Frage, ob das Schlüsselfenster das Hauptfenster von KRK oder ein daran hängendes Blatt ist; ist es keines von beidem, wirkt kein Befehl.
   - Pro: eine Bedingung an einer Stelle, kein Sonderfall für den Über-Dialog, und der offene Defekt zum Freigabedialog fällt mit weg. Die Regel steht seit der Runde 7 als eine reine Funktion in `zulaessigkeit::zulaessig` und hat genau die drei Frager, die sie brauchen.
   - Contra: sie ändert das Verhalten in Lagen, die niemand gemessen hat, und die Abnahme dafür liegt beim Nutzer. Der Preis ist damit nicht die Zeile, sondern die Abnahme. Die Blattlage braucht Sorgfalt: ein anhängendes Blatt **ist** das Schlüsselfenster, und `waehrend_blatt_erlaubt` muss weiter durchkommen, sonst ist der Abbruch aus dem Blatt heraus nicht mehr erreichbar.
3. **Der Über-Eintrag fällt aus dieser Runde.** Namen und Version stehen allein in der Titelleiste; der Menüeintrag kommt in der Runde, die den blinden Fleck schließt.
   - Pro: die Runde bleibt bei dem, was ihre Directive nennt, und trägt keinen ungemessenen Fall ein.
   - Contra: der auf dem Mac erwartete Ort bleibt leer, und der Nutzer hat den Eintrag am 260813-1010 ausdrücklich bestellt.

## Constraints

- Die Zulässigkeitsregel steht seit der Runde 7 an genau einer Stelle, und drei Frager lesen sie: der Kommandozweig des Abgriffs, der Zeichenzweig und die Ausgrauung des Menüs. Eine zweite Fassung daneben ist ausgeschlossen.
- Ein Blatt hängt am Hauptfenster und ist zugleich das Schlüsselfenster. Jede Regel über das Schlüsselfenster muss diesen Fall gesondert durchlassen, sonst sperrt sie den Abbruch aus.
- Der Abnahmelauf verlangt KRK im Vordergrund. Was hier gemessen werden müsste, ist Nutzerarbeit, und keine Möglichkeit kann daran vorbei.
- Der Über-Dialog gehört macOS. KRK setzt darin keinen Ersthelfer und kann sein Verhalten nicht vorschreiben.

## Recommendation

Möglichkeit 2. Der Grund ist nicht der Über-Dialog, sondern die Zahl der Stellen: mit ihm sind es zwei Flächen, hinter denen Befehle wirken, und die zweite entsteht in derselben Runde, in der die erste noch offen ist. Eine Bedingung, die nach dem Schlüsselfenster fragt, ist kein Sonderfall für den Dialog, sondern die Verallgemeinerung der Frage, die `ersthelfer_gehoert_appkit` schon stellt; sie schließt beide Flächen und jede dritte, die eine spätere Runde aufstellt.

Gegen Möglichkeit 1 spricht, dass ihr Preis unbekannt ist. Solange nicht gemessen ist, welchen Ersthelfer das Panel führt, ist „wir erben den blinden Fleck" keine Aussage über das Verhalten, sondern eine über die Kenntnis. Möglichkeit 3 ist der ehrliche Rückfall, wenn der Nutzer die Abnahme aus Möglichkeit 2 in dieser Runde nicht fahren will.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md, Abschnitt "Spec-Tor und die vierte Frage" — Antwort: Möglichkeit 2, die Runde schließt die Lücke einmal und allgemein in der Zulässigkeitsregel; der Defekt zum Freigabedialog der Runde 6 fällt mit weg.

---
Implemented: c3ada4d — Möglichkeit 2 ist gebaut. `Lage` trägt das vierte Feld `schluesselfenster_gehoert_krk` (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:152`), `zulaessig` fragt es innerhalb des `durchgelassen`-Ausdrucks (`:172-180`), und der Anwendungsdelegierte erhebt das Schlüsselfenster einmal je Eingabe (`crates/krk-ui/src/appkit/anwendung.rs:2623-2639`, gereicht in `lage` `:2664`). Die Tafel deckt 280 Fälle statt 140 (`zulaessigkeit.rs:435`); mit `schluesselfenster_gehoert_krk == false` steht in allen sieben Zeilen `ALLES_ABGEWIESEN`. Abgeglichen am 260813-1345.

**Zwei Aussagen dieses Datensatzes hat der Bau widerlegt, die Antwort selbst nicht.** Der Abschnitt `## Question` nennt `F5` und `delete` als Beispiele; beide tragen `Wirkungsbereich::Dateifenster` und kommen schon vor dieser Runde nicht durch (`issues/260813-1110_o_der-entscheid-zum-ueber-dialog-nennt-zwei-befehle-die-heute-schon-nicht-durchkommen.md`, offen). Der Vorteilssatz zu Möglichkeit 2 sagt, der Defekt zum Freigabedialog der Runde 6 falle mit weg; der Wähler ist keine eigenes Fenster, also erreicht die neue Bedingung ihn nicht (`issues/260813-1110_o_die-schluesselfensterfrage-erreicht-den-freigabewaehler-nicht-weil-er-kein-fenster-ist.md`, offen). Der Datensatz `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1529_*_die-blattregel-sieht-den-freigabedialog-nicht.md` steht deshalb weiter offen und trägt seit A3 einen Nachtrag über die Reichweite der neuen Regel.
