Der Doc-Kommentar von `luecke_bis` sagt weiter, der Leerraum falle zeilenweise weg — auf Dokumentebene tut er das seit diesem Turn nicht mehr

---

`Zerlegung::luecke_bis` trägt in seinem Doc-Kommentar
(`crates/krk-ui/src/markdown.rs:727-731`) unverändert den Satz:

> **Leerraum faellt weg**, und mit ihm, was die Umgebung auf jeder Zeile
> wiederholt ([`ohne_umgebungszeichen`]).

Der Satz gilt seit `2c0b2a6` nur noch innerhalb eines Elements. Auf
Dokumentebene ruft die Funktion `ohne_umgebungszeichen` nicht mehr, sondern
schneidet mit `trim()` allein an den beiden Enden der ganzen Lücke; der Einzug
einer Zeile bleibt dort stehen.

---

**Gelesen** (`crates/krk-ui/src/markdown.rs`, Stand `2c0b2a6`):

| Stelle | Was sie sagt |
|---|---|
| `:727-731` Doc-Kommentar von `luecke_bis` | „Leerraum faellt weg, und mit ihm, was die Umgebung auf jeder Zeile wiederholt" — ohne Einschränkung |
| `:754-757` Rumpfkommentar derselben Funktion | „Auf Dokumentebene wiederholt keine Umgebung etwas, also ist der Einzug dort Inhalt und bleibt stehen" |
| `:758-762` Rumpf | `if self.offen.is_empty() { Cow::Borrowed(luecke.trim()) } else { Cow::Owned(ohne_umgebungszeichen(luecke)) }` |

Doc-Kommentar und Rumpfkommentar derselben Funktion sagen Verschiedenes, und
der Rumpf gibt dem zweiten recht.

**Die beiden anderen Stellen sind mit demselben Commit nachgezogen worden**,
diese eine nicht:

- der Modulkopf (`:71-76`) trägt jetzt „**Der Einzug einer Zeile bleibt dort
  stehen**, denn hier wiederholt keine Umgebung etwas",
- der Doc-Kommentar von `ohne_umgebungszeichen` (`:412-417`) trägt jetzt
  „**Gerufen wird nur innerhalb eines Elements** … `Zerlegung::luecke_bis`
  bleibt deshalb beim blossen `str::trim`".

**Gemessen** (dass die Ausgabe dem Rumpf und nicht dem Doc-Kommentar folgt):

```
Quelle : "[ZIEL]: http://z.example\n      \"Titel\"\n"
2c0b2a6: "[ZIEL]: http://z.example\n      \"Titel\""
```

**Was zu tun ist:** den Satz im Doc-Kommentar von `luecke_bis` an derselben
Grenze teilen, an der der Rumpf ihn teilt — innerhalb eines Elements fällt
weg, was die Umgebung wiederholt, auf Dokumentebene schneidet `trim()` nur an
den Enden.

**Gewicht: niedrig.** Buchführung, kein Verhalten. Es ist aber genau der
Befundtyp, den dieser Turn an drei anderen Stellen behoben hat — eine Zusage
im Code, die etwas anderes sagt als der Code —, und der Doc-Kommentar ist die
Stelle, die ein Leser zuerst liest.

**Herkunft:** Circle der Runde 6, Turn 5, `2c0b2a6`.
