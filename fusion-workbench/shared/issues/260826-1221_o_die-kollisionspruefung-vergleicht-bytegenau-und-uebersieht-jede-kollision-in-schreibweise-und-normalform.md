Die Kollisionspruefung vergleicht bytegenau und uebersieht jede Kollision in Schreibweise und Normalform

---

`kollision::pruefen` fragt den Bestand ueber ein `HashSet<&str>` und zaehlt die neuen Namen ueber
eine `HashMap<&str, usize>`, beide bytegenau. Der Datentraeger, auf dem KRK laeuft, unterscheidet
weder Gross- und Kleinschreibung noch die Normalform eines Umlauts. Die Vorschau meldet deshalb
"keine Kollision" fuer einen Namen, den das Dateisystem schon traegt; der Nutzer bekommt statt
der Vorschau, die den Stapel ungefaehrlich machen soll, eine Zeile in der Abschlussliste.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-core/src/stapelumbenennen/kollision.rs:78-101`:

```rust
let vorhanden: HashSet<&str> = bestand.iter().map(String::as_str).collect();
let mut haeufigkeit: HashMap<&str, usize> = HashMap::with_capacity(neue.len());
…
if Some(neu.as_str()) != alt && vorhanden.contains(neu.as_str()) {
    return Some(Kollision::Bestehender);
}
if haeufigkeit.get(neu.as_str()).copied().unwrap_or_default() > 1 {
    return Some(Kollision::Doppelt);
}
```

Betroffen sind beide Faelle: `Bestehender` gegen den Ordnerbestand und `Doppelt` gegen die
uebrigen neuen Namen desselben Stapels.

## Gemessen, nicht vermutet

Am 260826 auf `/System/Volumes/Data`, dem Datentraeger dieses Geraets, in einem Wegwerfordner
ausserhalb des Quellbaums:

- **Schreibweise.** Ein `a.txt` neben einem `b.txt`, dann `renamex_np("a.txt", "B.TXT",
  RENAME_EXCL)` → `rc=-1, errno=17 (File exists)`. Der Bestand traegt `b.txt`, die Vorschau
  vergleicht gegen `B.TXT` und findet nichts.
- **Normalform.** Eine Datei, deren Name als `a` plus kombinierendes Trema (NFD) angelegt wurde,
  ist unter der zusammengesetzten Form (NFC) auffindbar: `os.path.exists` mit dem NFC-Namen
  liefert `True`, waehrend `os.listdir` den NFD-Namen zurueckgibt. Bestand und neuer Name koennen
  also in verschiedenen Normalformen dastehen und dennoch denselben Eintrag benennen.

## Was daran haengt und was nicht

**Es geht nichts verloren.** Die Ausfuehrung laeuft ueber
`operation::umbenennen` (`operation/umbenennen.rs:89-97`) und damit ueber
`im_datentraeger_verschieben(pfad, &ziel, true)`, also `renamex_np` mit `RENAME_EXCL`. Der
gemessene `EEXIST` weist den einzelnen Eintrag ab, der Inhalt der getroffenen Datei bleibt
unangetastet (im selben Lauf nachgesehen: `b.txt` traegt weiter `BBB`, `a.txt` steht noch da).

**Kaputt ist die Zusage der Vorschau.** Der Modulkopf von
`stapelumbenennen/vorschau.rs:3-5` nennt sie "das, was das Umbenennen im Stapel ungefaehrlich
macht", und C4 verlangt sie vor der Ausfuehrung. Eine Vorschau, die drei Faelle aufzaehlt und in
zweien davon bytegenau vergleicht, sagt bei genau den Namen nichts, bei denen der Nutzer es am
wenigsten erwartet — die Zeile sieht anders aus, und der Eintrag ist trotzdem belegt.
`Vorschau::kollisionen` (`vorschau.rs:56-61`) zaehlt sie folglich zu niedrig, und die
Zusammenfassung des Blattes nennt eine Zahl, die der Lauf danach widerlegt.

## Wo dieselbe Frage schon einmal aufgeschlagen ist

`shared/issues/260825-1425_*_der-schnitt-sieht-einen-zerlegt-geschriebenen-umlaut-als-anderen-eintrag-als-den-zusammengesetzten.md`
haelt dieselbe Wurzel an einer **anderen** Stelle fest, naemlich am Schnitt des Packziels in
`krk-ui`. Dieser Datensatz ist nicht derselbe Ort, aber er ist dieselbe Frage, und wer eine
Antwort baut, sollte beide Stellen zugleich ansehen: eine Namensgleichheit, die das Dateisystem
bejaht und KRK verneint, taucht ueberall auf, wo KRK zwei Namen vergleicht.

## Was zu tun waere

Der Vergleich braucht eine Fassung, die Schreibweise und Normalform zusammenfuehrt, und sie
gehoert an **eine** Stelle, nicht in jeden Rufer. Ob KRK dafuer eine eigene Faltung baut oder die
Frage an das Dateisystem gibt (ein `symlink_metadata` je neuem Namen), ist eine Abwaegung
zwischen Genauigkeit und Kosten: die Vorschau rechnet heute ohne einen einzigen Systemaufruf
(`stapelumbenennen/mod.rs:36-39` haelt gerade das als Eigenschaft fest), und ein Aufruf je Zeile
naehme sie ihr.

## Umfang

`krk-core`, `stapelumbenennen/kollision.rs`. Die Vorschau selbst bleibt unberuehrt.

Also seen: 260826-1338 by coderev — die Blattseite stellt die Vorschau ohne Vorbehalt als das dar, was der Befehl täte: `stapelumbenennen.rs:398-401` („Die Vorschau zeigt, was der Befehl täte … Einträge mit einem Hinweis bleiben stehen") und `zusammenfassung` (`:452-460`, „N werden umbenannt, K bleiben stehen") tragen die zu niedrige Zahl aus `Vorschau::kollisionen` unverändert zum Nutzer; eine Behebung im Kern reicht, das Blatt rechnet nichts selbst.
