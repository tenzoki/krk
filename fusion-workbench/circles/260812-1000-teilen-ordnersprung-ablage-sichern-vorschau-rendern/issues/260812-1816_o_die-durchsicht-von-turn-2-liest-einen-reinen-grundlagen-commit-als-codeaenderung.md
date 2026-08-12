Die Durchsicht von Turn 2 liest einen reinen Grundlagen-Commit als Codeänderung

---

Das Durchsichtsdokument
`reviews/260812-1805-coderev-turn-2-der-runde-6.md` sagt in seinem Nachtrag, Commit `4413d7a`
tausche „die Bildlaufansicht aus Schritt 11 gegen einen Kurzhinweis über `setToolTip:`". Das
ist falsch. `4413d7a` ändert genau zwei Dateien, beide Entscheidungsdatensätze:

```
260812-1105_a_… → 260812-1105_s_…   (4 Zeilen)
260812-1809_a_wie-wird-eine-meldung-lesbar-…   (85 Zeilen, neu)
```

Am Baum nachgemessen, unmittelbar nach dem Commit: `setToolTip` steht weiterhin an genau zwei
Stellen, beide in `crates/krk-ui/src/appkit/bereichsleiste.rs` und beide älter als diese Runde.
`NSScrollView` kommt in `crates/krk-ui/src/appkit/statuszeile.rs` zehnmal vor. Der Kurzhinweis
ist zu diesem Zeitpunkt **beschlossen und nicht gebaut**.

---

Der Fehler ist folgenreicher als ein Schreibfehler. Wer den Nachtrag liest, hält den Umbau für
erledigt und nimmt Schritt 11 nicht zurück; oder er sucht nach einer Bildlaufansicht, die er für
entfernt hält, und findet sie. Beides kostet einen Durchgang.

Vermutlich ist die Reihenfolge die Ursache: `4413d7a` ist während der Durchsicht gelandet, also
außerhalb des geprüften Bereichs `34ab5b5..05797d7`, und der `coderev` hat ihn aus seiner
Beschreibung gelesen statt aus seinem Inhalt. Die Commit-Nachricht sagt, was **beschlossen** ist,
und der Datensatz sagt ausdrücklich „Schritt 11 der Runde 6 wird zurückgenommen" im Futur. Aus
einer Beschreibung auf einen Inhalt zu schließen ist genau der Schritt, den eine Durchsicht nicht
tun darf.

**Zu beheben zusammen mit dem Umbau selbst.** Sobald der Kurzhinweis gebaut ist, stimmt die
Aussage des Nachtrags dem Ergebnis nach, aber nicht dem Zeitpunkt nach. Wer sie korrigiert, setzt
einen datierten Zusatz unter den Nachtrag, statt ihn umzuschreiben: das Dokument ist die
Aufzeichnung eines Standes und behält, was es damals sagte.

Der übrige Befund der Durchsicht ist davon nicht berührt. Insbesondere gilt der Befund zur
ausgeblendeten Seite der Statuszeile unverändert.
