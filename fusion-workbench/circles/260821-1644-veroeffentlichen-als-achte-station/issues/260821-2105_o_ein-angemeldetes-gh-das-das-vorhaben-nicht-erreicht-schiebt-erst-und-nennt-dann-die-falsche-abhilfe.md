# Ein angemeldetes `gh`, das das Vorhaben nicht erreicht, schiebt erst und nennt dann die falsche Abhilfe

---
**Status:** open
**Domain:** code
**Filed by:** coderev
**Baumstand:** `26212b1`
**Gefunden bei:** Abnahmeprüfung der fünfzehn Nutzerkriterien, 260821-2105
**Betrifft:** `xtask/src/veroeffentlichung.rs`

---

## Der Befund

`gh_pruefen` (`xtask/src/veroeffentlichung.rs:176`) stellt zwei Fragen: lässt sich `gh`
starten, und meldet `gh auth status` null. Beide beantworten die äußere Voraussetzung
für ein *Konto*. Keine beantwortet sie für das *Vorhaben*. Ein `gh`, das vorhanden und
angemeldet ist, dessen Gegenstelle aber nicht erreichbar oder nicht adressierbar ist,
kommt durch die Vorprüfung, und der Lauf packt und schiebt, bevor er es erfährt.

Das ist gemessen und nicht erschlossen. Am 260821-2101 ist der eigenständige Weg in einem
Wegwerfklon gegen eine Ersatzgegenseite gefahren worden, deren `origin` kein GitHub-Wirt
ist:

```
Tag geprueft: v0.5.6 steht auf HEAD.
Ticket geprueft: das Buendel unter …/target/KRK.app traegt die Beglaubigung angeheftet.
Gepackt: …/target/KRK-0.5.6.zip
Geschoben: HEAD und refs/tags/v0.5.6 stehen auf origin.
xtask: Das Release v0.5.6 liess sich nicht anlegen (exit status: 1): none of the git
remotes configured for this repository point to a known GitHub host. …
```

Gepackt und geschoben ist beides, bevor der Befund fällt.

## Warum das mehr ist als eine fehlende Prüfung

Der zweite Teil wiegt schwerer als der erste. Die Abbruchmeldung
(`veroeffentlichung.rs:626` und, im Wortlaut nah, `spaet_ohne_gh_meldung`, `:241`) sagt:

> Was fehlt, ist allein die Releaseseite. Derselbe Aufruf noch einmal holt sie nach und
> schiebt dabei nichts zweites.

Diese Auskunft trägt genau eine Ursache und nicht zwei. Für einen Zeitüberlauf oder eine
abgerissene Verbindung stimmt sie: der nächste Aufruf holt die Seite nach. Für ein
unerreichbares Vorhaben stimmt sie nicht: die Ursache ist stehend, und derselbe Aufruf
scheitert für immer gleich. Der Nutzer bekommt an der Stelle, an der er eine Diagnose
braucht, die Aufforderung, es noch einmal zu versuchen.

Ein Zweig für zwei Lagen, die einander ausschließen — die eine behebt sich durch
Wiederholung, die andere nie —, ist der Fall, den `rules/critical-stance.md` §4 als
unvollständige Fallunterscheidung führt.

## Was daneben mitzieht

`release_steht` (`veroeffentlichung.rs:654`) fragt allein den Rückgabewert von
`gh release view`. Ein unerreichbares Vorhaben liefert einen Wert ungleich null, also
lautet die Antwort „steht nicht", und der Lauf geht zum Anlegen über. Der Doc-Kommentar
nennt das die sichere Richtung, und für die Frage nach dem Überschreiben ist es das auch:
`gh release create` weist ein bestehendes Release ab. Für die Frage nach der äußeren
Voraussetzung ist es keine sichere Richtung, sondern dieselbe Lücke ein zweites Mal.

## Was nicht behauptet wird

Kein Abnahmekriterium fällt daran. C5.1 nennt „fehlt `gh`", C5.2 nennt „`gh` da, nicht
angemeldet", und beide halten; nachgemessen am 260821-2058 und 260821-2101. Was nicht
hält, ist der Satz, mit dem der Spec C5 begründet: „Eine fehlende Voraussetzung soll
auffallen, solange noch nichts geschehen ist."

Es entsteht auch kein Verlust. Geschoben werden HEAD und der eine Tag, beide aus dem
lokalen Stand; überschrieben wird nichts.

## Vorschlag zur Richtung

Zwei Wege, und der zweite ist der kleinere.

1. **Eine dritte Frage in `gh_pruefen`**, die das Vorhaben adressiert statt das Konto —
   etwa `gh repo view --json name`. Sie stünde vor dem ersten Wirken und fiele damit unter
   dieselbe Zusage wie die zwei vorhandenen. Kosten: ein dritter Netzruf je Lauf und eine
   vierte Frage an ein fremdes Werkzeug.
2. **Die Meldung des gescheiterten Anlegens trennt ihre zwei Ursachen.** Der Rückgabewert
   allein trennt sie nicht; die Standardfehlerausgabe von `gh` steht aber schon zur
   Verfügung und wird bereits mitgeführt. Der Satz „derselbe Aufruf holt sie nach" gehört
   dann an den Zweig, an dem er stimmt, und der andere Zweig nennt, dass die Ursache
   stehend ist.

Welcher Weg richtig ist, entscheidet der Planer. Der zweite behebt den schwereren Teil
des Befunds und ändert keine Voraussetzung der Kette.

## Belege

- `xtask/src/veroeffentlichung.rs:176` — `gh_pruefen`, zwei Fragen.
- `xtask/src/veroeffentlichung.rs:241` — `spaet_ohne_gh_meldung`, „derselbe Aufruf" (`:246`).
- `xtask/src/veroeffentlichung.rs:626` — die Meldung des gescheiterten Anlegens.
- `xtask/src/veroeffentlichung.rs:654` — `release_steht`, Rückgabewert allein.
- Lauf vom 260821-2101 gegen eine Ersatzgegenseite, Ausgabe oben zitiert.
