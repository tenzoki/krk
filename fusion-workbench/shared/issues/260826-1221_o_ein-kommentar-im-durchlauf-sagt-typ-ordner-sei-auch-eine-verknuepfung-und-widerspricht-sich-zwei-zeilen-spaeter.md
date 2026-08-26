Ein Kommentar im Durchlauf sagt, `Typ::Ordner` sei auch eine Verknuepfung, und widerspricht sich zwei Zeilen spaeter

---

`crates/krk-core/src/verzeichnis/durchlauf.rs:548-554` steht ueber der Fallunterscheidung, die
entscheidet, was auf den Vormerkstapel wandert. Er behauptet zuerst, `Typ::Ordner` trage auch
Verknuepfungen auf Ordner, und sagt zwei Zeilen spaeter, der Zweig `Verknuepfung` trenne die
beiden. Beides zugleich kann nicht gelten, und die erste Haelfte ist am Baum falsch.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/verzeichnis/durchlauf.rs:548-554`
**Tree state:** `004ff72`
**Domain:** code

## Was dasteht

```rust
// crates/krk-core/src/verzeichnis/durchlauf.rs:548-554
// Die Fallunterscheidung ueber den Typ ist vollstaendig und hat
// keinen Auffangzweig. `Ordner` ist auch eine Verknuepfung auf
// einen Ordner; es ist derselbe Schnitt, den die Sichtbarkeit
// zieht. Erst der Zweig fuer `Verknuepfung` trennt die beiden,
// und er steht am Kopf dieser Funktion fuer den Auftrag und
// hier fuer den Abstieg: ...
match kandidat.typ {
    Typ::Ordner => offen.push(lesestand.pfad.join(&kandidat.name)),
```

## Warum die erste Haelfte falsch ist

`kandidat.typ` stammt aus `Schwungleser::naechster_schwung` und damit aus
`typ_aus_objtype` (`crates/krk-core/src/verzeichnis/sys.rs:421-427`):

```rust
fn typ_aus_objtype(roh: u32) -> Typ {
    match roh {
        VDIR => Typ::Ordner,
        VLNK => Typ::Verknuepfung,
        _ => Typ::Datei,
    }
}
```

`getattrlistbulk(2)` folgt einer Verknuepfung nicht — `eintrag.rs:22-24` sagt es fuer den Typ
ausdruecklich, `verweisziel.rs:14-16` wiederholt es. Ein Eintrag, der auf einen Ordner zeigt,
kommt als `VLNK` und damit als `Typ::Verknuepfung`. **`Typ::Ordner` traegt an dieser Stelle nie
eine Verknuepfung**, und `offen.push` erreicht folglich keine.

## Was der Satz vermutlich sagen wollte

Der Schnitt, auf den er sich beruft, ist der der **Sichtbarkeit**, und dort stimmt er:
`zeilengrund_von` (`modell.rs:757`) fragt `eintrag.ist_ordner() || eintrag.ist_verknuepfung()`
und fasst die zwei damit tatsaechlich zusammen. Der Kommentar traegt diese Aussage an eine
Stelle, an der sie ueber einen anderen Wert gemacht wird — ueber `kandidat.typ`, wo sie nicht
gilt.

## Warum das nicht folgenlos ist

Der Modulbaum stuetzt seine Zusagen an mehreren Stellen darauf, dass in eine Verknuepfung
weder abgestiegen noch hineingelesen wird (C3.7, C3.9; `durchlauf.rs:141-143` begruendet damit,
dass es keine Besuchtliste braucht, und `durchlauf.rs:490-501` den Kurzschluss am Auftrag).
Wer den Kommentar beim Wort nimmt, liest, dass `offen.push` Verknuepfungen mitnimmt, und
schliesst daraus auf eine Ringgefahr, die es nicht gibt — oder umgekehrt auf eine Deckung des
Abstiegs, die er nicht hat. Es ist dieselbe Sorte Fehlbefund, die
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`
hervorgebracht hat.

## Richtung

Der Satz wird auf den Wert bezogen, ueber den er spricht: an dieser Stelle sind `Ordner` und
`Verknuepfung` zwei getrennte Werte, und die Zusammenfassung der beiden gehoert der
Sichtbarkeit in `modell.rs:757` und nicht dem Typ des Lesers. Der Rest des Kommentars — der
Zweig `Verknuepfung` traegt nichts bei, und die Fallunterscheidung ist ohne Auffangzweig — steht
richtig da und bleibt.
