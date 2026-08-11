Die Abweisungsmeldung der Zwischenablage nennt "den Pfad" im Singular und steht auch bei mehreren

---

`ablage_weist_ab()` liefert "die Zwischenablage hat den Pfad nicht angenommen" (`crates/krk-ui/src/kommandos/operationen.rs:840-842`). Beide Kopierbefehle setzen diesen Satz: `ordnerpfad_kopieren` (`crates/krk-ui/src/appkit/tabelle.rs:836`), wo er stimmt, und `eintragspfad_kopieren` (`tabelle.rs:864`), wo er auch für dreißig Pfade steht.

---

**Ein Wort, und es widerspricht dem Muster der Nachbarn.** Jede andere Meldung dieser Runde unterscheidet Einzahl und Mehrzahl: `kopiermeldung` nennt bei einem Pfad den Pfad und bei mehreren ihre Zahl, `oeffnungsmeldung` bei einem Eintrag den Namen und bei mehreren ihre Zahl. Der Abweisungsfall fällt aus dieser Regel heraus.

Kein Abnahmekriterium des Specs verlangt hier eine Zahl; C1 und C2 sagen nur, dass gemeldet wird. Der Fall ist selten — `setString:forType:` weist nach einem `clearContents` kaum je ab — und genau deshalb ist er billig zu berichtigen und teuer zu übersehen.

**Vorschlag für die Behebung.** Entweder den Satz auf die Sache statt auf die Zahl beziehen ("die Zwischenablage hat den Text nicht angenommen"), was ihn für beide Aufrufer richtig macht und keine Fallunterscheidung braucht; oder die Zahl mitgeben, wie es die beiden Nachbarn tun. Der erste Weg ist der kleinere: der Kopierer legt in beiden Fällen **einen** Text ab, und "Text" ist genau das, was `text_schreiben` nimmt.

Gefunden vom `coderev` am 260811 bei der Durchsicht des Turns 1 dieses Circles.

---
Resolved: Die Meldung lautet jetzt "die Zwischenablage hat den **Text** nicht angenommen". Der
Kopierer legt in beiden Faellen genau einen Text ab — einer je Zeile, aber eine Ablage —, also
braucht der Satz keine Fallunterscheidung nach der Zahl.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1. Abgenommen mit `make check`, exit 0.
