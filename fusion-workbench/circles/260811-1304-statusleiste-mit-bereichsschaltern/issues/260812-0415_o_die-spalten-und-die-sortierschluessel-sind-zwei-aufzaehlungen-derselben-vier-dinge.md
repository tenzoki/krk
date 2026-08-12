Die Spalten und die Sortierschlüssel sind zwei Aufzählungen derselben vier Dinge

---

`Spalte` (`crates/krk-ui/src/appkit/tabelle.rs:179`) und `Schluessel`
(`crates/krk-core/src/verzeichnis/sortierung.rs:29`) führen beide genau vier Werte, und es
sind dieselben vier: Name, Größe, Geändert, Typ. Nichts im Baum hält sie zusammen. Wer eine
fünfte Spalte anlegt oder einen fünften Sortierschlüssel, bekommt vom Übersetzer keinen
Hinweis darauf, dass die andere Aufzählung nachzuziehen wäre.

---

**Schwere:** niedrig (kein falsches Verhalten heute, eine stille Abweichung morgen)
**Gefunden:** planner, beim Planen der Spaltenschalter dieses Circles
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-core/src/verzeichnis/sortierung.rs`
**Domain:** code

## Der Bestand, am 260812-0415 geprüft

Die Entsprechung ist heute vollständig und eins zu eins. Jede Spalte sortiert nach genau
einem Schlüssel: die vier Kommandos `SortierungName`, `SortierungGroesse`,
`SortierungDatum` und `SortierungTyp` treffen die vier Spalten, und die Spalte Typ zeigt
die Endung, nach der `Schluessel::Typ` ordnet (Nutzerentscheid vom 260806).

Aufgeschrieben ist diese Entsprechung nirgends. Es gibt keine Zuordnung von einer
Aufzählung auf die andere und keine Probe, die beide gegeneinander zählt. Beide sind
vollständige Fallunterscheidungen ohne Auffangzweig, jede für sich; der Übersetzer hält
also an, wenn **eine** von beiden wächst, aber er sagt nichts über die andere.

## Warum es diese Runde nicht behebt

Der Plan `planning/260812-0415_o_bereichsleiste-und-proportionale-breitenregel.md` zieht
den reinen Teil von `Spalte` in ein eigenes Modul, damit das Fenstermodell die Spalten
ansprechen kann, ohne AppKit zu nennen. Er fasst die Entsprechung zu `Schluessel`
ausdrücklich nicht an: eine Zusammenlegung der beiden Aufzählungen wäre ein Umbau von
`tabelle.rs` und ist von der Directive nicht verlangt.

## Was hilft

Die billige Antwort ist keine Zusammenlegung, sondern eine Probe. Eine Zuordnung
`Spalte::schluessel(self) -> Schluessel` als vollständige Fallunterscheidung, dazu eine
Probe, dass die Zuordnung in beide Richtungen aufgeht und beide Aufzählungen gleich viele
Werte tragen. Der Übersetzer hält danach bei jeder Erweiterung an, und die Frage "welche
Spalte gehört zu welcher Ordnung" hat eine Antwort statt einer Gewohnheit.

Die teure Antwort wäre, `Spalte` durch `Schluessel` zu ersetzen. Sie hat einen eigenen
Preis: eine Spalte, die nach nichts sortiert, ließe sich danach nicht mehr anlegen, ohne
den Sortierschlüssel zu verbiegen.

---

## Abgleich 260812-0801

**Zu Recht offen.** Beide Aufzählungen stehen weiter nebeneinander, und nichts hält sie zusammen:
`Schluessel` trägt vier Werte (`crates/krk-core/src/verzeichnis/sortierung.rs:29`), `Spalte`
ebenfalls vier. Es gibt keine Zuordnung von der einen auf die andere und keine Probe, die beide
gegeneinander zählt.

**Ein Ort hat sich geändert und der Datensatz nennt noch den alten.** Schritt 6 dieser Runde hat
`Spalte` aus `crates/krk-ui/src/appkit/tabelle.rs:179` nach `crates/krk-ui/src/spalten.rs:33`
gezogen; die Aufzählung ist dabei von privat auf `pub` gegangen und trägt seither zwei weitere
vollständige Fallunterscheidungen (`beschriftung`, `beschreibbar`) sowie eine dritte im
Fenstermodell (`spalte_sichtbar_in`, `fenstermodell.rs:350`). Der Umzug macht den vorgeschlagenen
billigen Weg **einfacher**, nicht schwerer: eine Zuordnung `Spalte::schluessel` läge jetzt in einer
Datei ohne AppKit, in der schon zwei solche Fallunterscheidungen stehen.

Die Zeile „Betroffen" ist damit zu lesen als `crates/krk-ui/src/spalten.rs` und
`crates/krk-core/src/verzeichnis/sortierung.rs`. Der Datensatz selbst bleibt unverändert; er hält
den Stand vom 260812-0415 fest.
