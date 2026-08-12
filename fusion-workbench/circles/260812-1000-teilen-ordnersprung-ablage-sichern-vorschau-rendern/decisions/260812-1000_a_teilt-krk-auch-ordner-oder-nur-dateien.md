# Teilt KRK auch Ordner, oder nur Dateien?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/kommandos/operationen.rs:162` (`betroffene`, zählt Ordner mit); `crates/krk-ui/src/appkit/standardprogramm.rs` (C3 der Runde 4, der nächstliegende Vorgänger)

---

## Question

`betroffene` liefert die Pfade der markierten Einträge und zählt dabei mit, wie viele davon Ordner sind (`auswahl.ordner += usize::from(eintrag.ist_ordner())`). Ein Teilen-Befehl, der diese Menge übernimmt, bekommt damit Ordner mitgeliefert, ob er sie will oder nicht.

Der Freigabedienst des Systems nimmt einen Ordner entgegen, und was danach geschieht, hängt am Dienst: AirDrop packt ihn und sendet ein Archiv, Mail hängt ihn gepackt an, andere Dienste lehnen ihn ab. KRK erfährt das Ergebnis in keinem Fall, aus demselben Grund, aus dem die Runde 4 beim Standardprogramm die Übergabe meldet und nicht das Öffnen: die Antwort käme über einen Rückruf auf einer beliebigen Schlange, und den führt dieses Projekt nicht.

Die Frage ist zu stellen, weil sie das Abnahmekriterium des Befehls festlegt und weil die Runde 4 dieselbe Frage für ihren Öffnen-Befehl bereits einmal beantwortet hat, dort mit einer Sonderregel für Ordner: `return` auf einem Ordner geht hinein, statt ihn an das System zu geben.

Sie hält keinen Planschritt auf und bindet einen.

## Options

1. **Ordner werden mitgeteilt, ohne Sonderregel.** Die betroffene Menge geht unverändert an den Freigabedienst; was er mit einem Ordner tut, ist seine Sache.
   - Folge: der Befehl bleibt eine Zeile Weiterreichung, und `betroffene` behält genau einen Aufrufer mehr ohne Filter. Der Nutzer kann einen Projektordner über AirDrop schicken, was der häufigste Grund ist, warum jemand in einem Dateiverwalter teilt.
   - Preis: bei einem Dienst, der Ordner ablehnt, sieht der Nutzer eine Fehlermeldung des Systems, die KRK weder auslöst noch erklärt.

2. **Ordner werden übersprungen, und die Statuszeile sagt es.** Nur Dateien gehen an den Dienst; die Zahl der übersprungenen Ordner steht in der Meldung.
   - Folge: KRK sagt zu, was es einhalten kann, und die Meldung gehört in dieselbe Familie wie `oeffnungsmeldung` aus der Runde 4. Der Filter ist eine Zeile, die Zählung liefert `Auswahl` schon.
   - Preis: der häufigste Anwendungsfall fällt weg. Wer einen Ordner teilen will, bekommt von KRK die Auskunft, dass es das nicht tut, obwohl das System es könnte.

3. **Ordner werden mitgeteilt, aber ein Ordner allein löst nichts aus.** Gemischte Mengen gehen ganz durch; ist der einzige betroffene Eintrag ein Ordner, meldet die Statuszeile, dass nichts zu teilen ist.
   - Folge: dieselbe Sonderregel für Ordner, die `return` in der Runde 4 trägt, hier ein zweites Mal, mit anderem Inhalt.
   - Preis: eine Regel, die von der Zusammensetzung der Auswahl abhängt, ist an der Oberfläche nicht ablesbar. Der Nutzer markiert zwei Einträge und bekommt ein anderes Verhalten als bei einem. Das ist der Sonderfall-Wildwuchs, den dieses Projekt an anderen Stellen ausdrücklich vermeidet.

## Constraints

- Der Befehl hat genau eine Auswahlregel, und das ist `betroffene`. Eine zweite Auswahlregel neben ihr entsteht nicht.
- KRK erfährt nicht, ob der Dienst die Übergabe angenommen hat, und darf deshalb keine Zusage über das Ergebnis formulieren. Die Meldungstexte der Runde 4 (`oeffnungsmeldung`) sind die Vorlage: „an das System übergeben", nicht „geteilt".
- C9 der Runde 1, „Nur lokale Laufwerke", ist von der Frage nicht berührt: ein Freigabedienst baut keine Verbindung über ein Dateiprotokoll auf.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie ist die einzige ohne Sonderregel, und sie beantwortet die Frage dort, wo sie hingehört: der Dienst weiß, was er mit einem Ordner kann, und KRK weiß es nicht. Möglichkeit 2 nimmt dem Nutzer einen Fall weg, den das System beherrscht, und Möglichkeit 3 macht das Verhalten von der Zusammensetzung der Auswahl abhängig, was an der Oberfläche nicht erkennbar ist.

`inference:` Wir schließen aus dem Wortlaut des Wunsches, „Share (zB mit airdrop)", dass AirDrop der gemeinte Hauptfall ist, und AirDrop nimmt Ordner an. Geprüft ist das an keinem laufenden Bündel.


## Antwort 260812-1105

**Moeglichkeit 1.**

KRK reicht weiter, was markiert ist, Ordner eingeschlossen, und laesst den Freigabedienst
entscheiden, was er damit kann.

Die einzige Moeglichkeit ohne Sonderregel, und sie beantwortet die Frage dort, wo sie hingehoert:
der Dienst weiss, was er mit einem Ordner kann, KRK weiss es nicht. Eine Auswahl auf Dateien zu
beschraenken naehme dem Nutzer einen Fall weg, den das System beherrscht; das Verhalten von der
Zusammensetzung der Auswahl abhaengig zu machen waere an der Oberflaeche nicht erkennbar.

`inference:` Der Wortlaut des Wunsches nennt AirDrop als Hauptfall, und AirDrop nimmt Ordner an.
Am laufenden Buendel geprueft ist das nicht.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:
