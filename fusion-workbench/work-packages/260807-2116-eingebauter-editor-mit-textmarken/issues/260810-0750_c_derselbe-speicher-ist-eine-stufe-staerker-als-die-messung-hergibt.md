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

---
Resolved: Der Befund hält, und er hält aus zwei Gründen — einem, der ohne jede
Messung entschieden ist, und einem, der jetzt im Baum gemessen wird.

**Ohne Messung:** „beide legen einander um" ist logisch schwächer als „derselbe
Speicher". Ein dreiwertiger Typ und ein Wahrheitswert können nicht dieselben
Bits sein; die stärkere Formulierung war unabhängig von jeder Zahl zu weit. Die
Formulierung ist damit auch dann zu ersetzen, wenn man keine einzige Messung
anführt.

**Gemessen**, in der neuen Probe
`die_erste_tuer_kann_default_weder_herstellen_noch_anzeigen`, je Paar auf einer
eigenen Fläche, macOS 15.7.7 (Build 24G720):

- *Sie zeigt `Default` nicht an.* Steht die zweite Tür auf `Default`, liest die
  erste eine Systemvorgabe, die je Einstellung anders ausfällt — an acht Paaren
  `YES`, an `linkDetectionType` und `dataDetectionType` `NO`. Die beiden Zahlen
  dieses Datensatzes sind damit nachgemessen und stimmen.
- *Sie stellt `Default` nicht her.* Schreibt man den eben gelesenen
  Wahrheitswert unverändert zurück, steht die zweite Tür danach auf `Yes` oder
  `No` und nie wieder auf `Default`. Diese zweite Messung steht in dem
  Datensatz nicht und ist die entschiedenere der beiden: sie zeigt den
  Auflösungsschritt nicht nur, sie zeigt ihn als unumkehrbar.

Modulkopf und der Doc-Kommentar von `Einordnung::ZweiteTuerZu` tragen jetzt die
gemessene Aussage in der vorgeschlagenen Form. Der Vorbehalt dieses Datensatzes
zum operativen Schluss ist übernommen: für „zwei Zeilen statt zwölf" genügt die
schwächere Aussage, und der Modulkopf sagt das an derselben Stelle. Die Probe
sichert bewusst nur zu, dass die Vorgabe **nicht bei allen Paaren gleich**
ausfällt; welche zwei aus der Reihe fallen, ist eine Systemeigenschaft und kein
Gegenstand einer Zusicherung.
