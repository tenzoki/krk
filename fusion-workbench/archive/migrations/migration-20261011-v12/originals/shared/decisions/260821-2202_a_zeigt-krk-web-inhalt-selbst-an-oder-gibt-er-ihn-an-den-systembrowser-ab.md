# Zeigt KRK Web-Inhalt selbst an, oder gibt er ihn an den Systembrowser ab?

---
**Domain:** code
**Filed by:** user (über den Orchestrator, Sitzung 260820-2200)
**Cross-references:** `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_d_circle.md`

---

## Question

Der vorgesehene Circle `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` sagt zu,
dass KRK eine Web-Adresse in einem eigenen Betrachter im Vorschaufenster anzeigt statt sie
abzugeben, samt Tastenbedienung und Sprungmarken auf jedem sichtbaren Link. `Opt+Cmd+G` würde
danach in KRK öffnen statt im Systembrowser.

Der Circle stand siebzehn Tage vorgesehen, ohne gefahren zu werden. Die Frage ist, ob die
Zusage überhaupt eingelöst werden soll.

## Options

1. **Bauen wie vorgesehen** — eigener Betrachter im Vorschaufenster.
   - Pro: Web-Inhalt bleibt in KRKs Tastenbedienung, kein Fensterwechsel.
   - Contra: eine andere Größenordnung als die bisherigen Runden. Darstellungsmittel,
     Tastenbedienung, Sprungmarken, dazu die ungeklärte Frage der Bündelrechte — `Info.plist`
     führt kein `NSAppTransportSecurity`, eine Berechtigungsdatei gibt es nicht, und signiert
     wird mit `--options runtime`.
2. **Beim Systembrowser bleiben** — `Opt+Cmd+G` gibt weiter ab, wie heute.
   - Pro: löst kein Problem, das der Nutzer hat; der Systembrowser kann alles, was ein
     eingebauter Betrachter könnte, und mehr.
   - Contra: der Fensterwechsel bleibt.
3. **Zurückstellen und später entscheiden.**
   - Contra: die Frage ist entscheidbar, und ein zurückgestellter Datensatz fällt aus der
     Suche nach aktiver Grundlage heraus.

## Constraints

Ein eingebauter Betrachter berührt die Bündelrechte und damit die Auslieferung. Solange das
ungemessen ist, trägt keine Planung.

---
Answered: Der Nutzer hat am 260821 Option 2 gewählt: **das Abgeben an den Systembrowser
genügt, ein eingebauter Betrachter löst kein echtes Problem.** Der Circle
`260804-0933-eingebauter-web-betrachter-im-vorschaufenster` ist daraufhin auf `_d_` gesetzt
worden; seine Schließungsnotiz zitiert diesen Datensatz. Die Wahl ist eine Absage an die
Zusage und keine Vertagung — der Marker `_d_` ist die nächstliegende Entsprechung, die das
Circle-Vokabular kennt, und nicht die genaue.
