Der Zeichenzweig setzt die Zulässigkeitsfrage im Anwendungsdelegierten von Hand aus drei Lagefeldern zusammen, und keine Probe sieht ihn

---

`CLAUDE.md` und `zulaessigkeit.rs:18-27` sagen: eine Frage, eine Stelle, zwei Frager. Für ein **getipptes Zeichen** wird dieselbe Frage ein zweites Mal beantwortet, in `Anwendungsdelegierter::eingabe_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs:2953-2987`), als Hand-Verknüpfung `lage.blatt_steht || lage.ersthelfer_gehoert_appkit` plus einem `match lage.fokus`. Die Zusammensetzung steht in AppKit-Code, hat keine Tafel, und die Aufruferzählung `beide_frager_rufen_die_eine_regel` kann sie nicht sehen, weil sie `zulaessig` nicht ruft.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Befund

`anwendung.rs:2967-2985`:

```rust
let lage = self.lage();
if lage.blatt_steht || lage.ersthelfer_gehoert_appkit {
    return false;
}
match lage.fokus {
    Fokus::Dateifenster => { … filterzeichen_tippen(zeichen) }
    Fokus::Leiste | Fokus::Vorschau | Fokus::Editor | Fokus::Anderswo => false,
}
```

Der Kommentar darüber sagt es selbst: „Dieselbe Erhebung wie im Kommandozweig, und dieselben drei Werte." `Lage` (`zulaessigkeit.rs:123-125`) nennt den Zeichenzweig ebenfalls als zweiten Leser. Das ist die Doppelung, die der Modulkopf von `zulaessigkeit.rs:24-27` für Kommandos ausschließt: zwei Fassungen an zwei Stellen, die auseinanderlaufen können.

**Der vierte Bestandteil (Schlüsselfenster gehört KRK) ist hier nur mittelbar abgedeckt:** `fokus_bei(Fremd)` liefert `Anderswo` (`anwendung.rs`, `fn fokus_bei`), und `Anderswo` fällt auf `false`. Wer die Fokusabfrage ändert, ändert damit unbemerkt die Zulässigkeit des Zeichens.

**Keine Probe hält den Zweig.** `zulaessigkeit.rs` trägt eine Tafel aus 280 Fällen für Kommandos und keine für Zeichen; `anwendung.rs` hat kein Bibliotheksziel und kann die Lage nicht ohne Fenster stellen.

## Vorschlag

`zulaessigkeit::zeichen_zulaessig(lage: Lage) -> bool` (`#[must_use]`) als zweite reine Funktion neben `zulaessig`, mit einer ausgeschriebenen Tafel aus 2 × 2 × 2 × 5 = 40 Fällen, und der Zeichenzweig ruft sie. Die Aufruferzählung dazu wie bei `rueckschritt`: genau ein Rufer. Der Modulkopf von `zulaessigkeit.rs` bekommt einen Absatz „Zwei Fragen, nicht eine": die Kommandofrage und die Zeichenfrage teilen die Lage und unterscheiden sich im Wirkungsbereich.

Schwere: mittel — kein heutiger Fehlfall gefunden, aber die einzige Zulässigkeitsentscheidung des Baums ohne Probe und ohne Zählung.
