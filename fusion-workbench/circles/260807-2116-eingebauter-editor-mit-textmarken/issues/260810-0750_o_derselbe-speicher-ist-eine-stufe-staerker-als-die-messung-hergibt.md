# „Derselbe Speicher" ist eine Stufe stärker als die Messung hergibt

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:129-137` (Modulkopf), `:2628-2632` (`Einordnung::ZweiteTuerZu`), Commit-Nachricht `d9fc2c8`
**Cross-references:** `issues/260810-0748_o_die-kopplung-der-zehn-paare-traegt-den-commit-und-ist-im-baum-durch-nichts-gehalten.md`

---

## Der Befund

Modulkopf und Commit-Nachricht sagen: „Zehn der dreizehn `set…Type:` sind
**derselbe Speicher** wie ein `set…Enabled:` daneben." Gemessen ist etwas
Schwächeres: die beiden legen einander um. Das ist mit „derselbe Speicher"
verträglich, aber auch mit „der boolesche Wert ist eine verlustbehaftete Sicht
auf den dreiwertigen".

Der Unterschied ist nicht akademisch. `NSTextInputTraitType` hat drei Werte —
`Default` (0), `No` (1), `Yes` (2) —, der Wahrheitswert hat zwei. `Default`
lässt sich über die erste Tür **nicht** herstellen, und das Lesen der ersten
Tür löst `Default` in eine Systemvorgabe auf, die je Einstellung verschieden
ausfällt. Gemessen auf macOS 15.7.7 (Build 24G720):

```
typeDEFAULT -> bool = 1   fuer acht Paare
typeDEFAULT -> bool = 0   fuer linkDetectionType und dataDetectionType
```

Wären es dieselben Bits, gäbe es diesen Auflösungsschritt nicht.

## Was daran hängt

Für den operativen Schluss des Commits **nichts**: `bool = NO` setzt den Typ auf
`No` und nagelt ihn fest, in allen zehn Fällen gemessen. Die zwei neuen Zeilen
statt zwölf sind richtig.

Die Formulierung lädt aber zum Rückschluss in die Gegenrichtung ein — „dann
sagt mir der Wahrheitswert, worauf der Typ steht" —, und der ist falsch: bei
`Default` sagt er nur, wie das System heute entscheidet. Genau diesen
Unterschied hat der Commit an anderer Stelle selbst gebraucht: die Begründung
für die beiden neuen Zeilen lautet, `Default` überlasse dem System die Wahl und
`No` sei die Absage. Der Satz über die zehn Paare nimmt diesen Unterschied
wieder zurück.

## Vorschlag

„Derselbe Speicher" durch das ersetzen, was gemessen ist: *jede der beiden
Türen legt die andere um; die erste kann den Zustand `Default` weder
herstellen noch anzeigen.* Kostet eine Zeile im Modulkopf und einen Halbsatz im
Doc-Kommentar von `ZweiteTuerZu`.
