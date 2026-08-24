Zwei Bausteinbeschreibungen sagen weniger, als der Baustein tut

---

Die Erklärung des Bausteins `feld` (`resources/default-readers.toml:96-99`) verschweigt, welche
Datei gewinnt, wenn `datei` mehrere trifft; die Erklärung von `vorhandensein` (`:105-106`) sagt
„Der Wert ist „ja" oder „nein"" und lässt den dritten Ausgang weg. Beide Sätze sind das, woran der
Nutzer sein eigenes Profil baut.

---

**Erstens: `feld` hat bei mehreren Treffern keine zugesagte Wahl.**

`bausteine.rs:380-384` schreibt es aus: „Die erste passende Datei ist die erste in der
Lesereihenfolge, und die gibt das Dateisystem vor. Ein Muster, auf das mehrere Einträge passen, hat
damit keine zugesagte Wahl unter ihnen; die mitgelieferten Profile verankern ihre Dateimuster
deshalb an beiden Enden."

Nachgezählt am 260824-1653: alle fünf `datei`-Muster der Auslieferungsfassung sind an beiden Enden
verankert — `'^\.fusion-setup$'` (dreimal), `'^\.active-circle$'`, `'^orchestrator-live\.md$'`,
`'^_._circle\.md$'`. Die Datei ist also selbst in Ordnung. Wer nach ihrem Vorbild
`datei = '\.md$'` schreibt, bekommt eine beliebige Datei des Ordners und keine Meldung darüber.
Der Satz, der ihn davor bewahrt, steht im Rust-Quelltext und nicht in der Datei, die er liest.

**Zweitens: `vorhandensein` hat einen dritten Ausgang.**

`Lauf::zielordner` (`bausteine.rs:274-284`) löst den in `ordner` genannten Unterordner über
`std::fs::canonicalize` auf. Gibt es ihn nicht, liefert die Rechnung `None`, und die Zeile zeigt
`--` statt „nein". Für einen Nutzer sind „der Ordner ist da und enthält nichts Passendes" und „den
Ordner gibt es nicht" zwei verschiedene Auskünfte, und nur die erste heißt „nein".

**Am Bestand dieser Werkbank tritt der dritte Ausgang heute nirgends ein**, nachgezählt an den
zwei Zeilen „Spec" und „Plan" des Rundenprofils: alle achtzehn Rundenverzeichnisse führen ein
`planning/`, gemessen 8 mal „ja" und 10 mal „nein" bei „Spec", 14 zu 4 bei „Plan", und kein
einziger Platzhalter. Der Fall ist latent und nicht beobachtet.

**Vorschlag.** Zwei Sätze, je einer an seinem Baustein: bei `feld` ein Hinweis, dass ein Muster,
auf das mehrere Dateien passen, keine bestimmte davon wählt und deshalb an beiden Enden zu
verankern ist; bei `vorhandensein` ein dritter Ausgang, „gibt es den genannten `ordner` nicht,
zeigt die Zeile ihren Platzhalter und nicht „nein"".

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.

---
Resolved: Beide Bausteinbeschreibungen tragen jetzt, was der Baustein tut.

Bei `feld` (`resources/default-readers.toml:103-106`) steht der Absatz: passen mehrere Dateien auf
`datei`, gewinnt die erste in der Lesereihenfolge, die das Dateisystem vorgibt, und welche davon es
wird, ist nicht zugesagt; ein `datei`-Muster gehört deshalb an beiden Enden verankert, wie die fünf
ausgelieferten es sind.

Bei `vorhandensein` (`:112-116`) steht der dritte Ausgang: gibt es den genannten `ordner` nicht,
zeigt die Zeile ihren Platzhalter `--` und nicht „nein", denn „der Ordner ist da und enthält nichts
Passendes" und „den Ordner gibt es nicht" sind zwei verschiedene Auskünfte.

Am 260824-1739 nachgerechnet: die fünf `datei`-Muster der Auslieferungsfassung sind unverändert an
beiden Enden verankert, und über die achtzehn Rundenverzeichnisse tritt der dritte Ausgang weiter
nirgends ein — „Spec" 8 zu 10, „Plan" 14 zu 4, kein Platzhalter. Der Fall bleibt latent, und der
Satz, der den Nutzer davor bewahrt, steht jetzt in der Datei, die er liest.
