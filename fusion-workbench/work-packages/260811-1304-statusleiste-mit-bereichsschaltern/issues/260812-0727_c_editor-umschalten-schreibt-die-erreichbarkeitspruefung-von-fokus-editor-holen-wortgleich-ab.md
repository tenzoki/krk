`editor_umschalten` schreibt die Erreichbarkeitsprüfung von `fokus_editor_holen` wortgleich ab

---

Die neue Funktion `Anwendungsdelegierter::editor_umschalten` besteht aus vier Zeilen, die
Zeichen für Zeichen in `Anwendungsdelegierter::fokus_editor_holen` stehen, und einer fünften,
die sich unterscheidet. Der Doc-Kommentar sagt „Dieselbe Bedingung trägt `fokus_editor_holen`"
und benennt damit die Doppelung, ohne sie aufzulösen. Zwei Fassungen einer Bedingung laufen
beim nächsten Nachjustieren auseinander, und keine Probe hielte sie aneinander.

---

**Schwere:** niedrig (kein falsches Verhalten heute; beide Fassungen sind gleich)
**Gefunden:** coderev, zweite Durchsicht der Runde, Bereich `8ffaac2..0342445`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs`
**Domain:** code

## Die beiden Fassungen

`fokus_editor_holen` (`crates/krk-ui/src/appkit/anwendung.rs:1466`):

```rust
let ausgeblendet = !self.ivars().modell.borrow().sichtbar(Bereich::Editor);
let haelt_datei = self
    .ivars()
    .editor
    .get()
    .is_some_and(|editor| editor.haelt_datei());
if ausgeblendet && !haelt_datei {
    return false;
}
self.fokus_holen(Fokus::Editor)
```

`editor_umschalten` (`:4389`):

```rust
let ausgeblendet = !self.ivars().modell.borrow().sichtbar(Bereich::Editor);
let haelt_datei = self
    .ivars()
    .editor
    .get()
    .is_some_and(|editor| editor.haelt_datei());
if ausgeblendet && !haelt_datei {
    return false;
}
self.bereich_umschalten(Bereich::Editor)
```

Der Unterschied ist die letzte Zeile. Die Bedingung darüber ist dieselbe Aussage: **der Editor
ist ansprechbar, wenn er steht oder wenn er eine Datei hält.**

## Warum das in diesem Baum ein Befund ist

Die Runde hat für genau diese Bauart mehrfach eine Stelle geschaffen statt einer zweiten
Fassung: `breite_in` neben `sichtbar_in` (`fenstermodell.rs`, angelegt, damit
`wuensche_nachfuehren` die Zuordnung nicht ein drittes Mal ausschreibt), `spaltenfach`
(`bereichsleiste.rs`, gerechnet statt hingeschrieben), `kennung` als eine Funktion für Aufbau
und `spalte_verbergen`. Die erste Durchsicht dieser Runde hat mit Befund 3 genau eine solche
Doppelung geschlossen (`issues/260812-0539_c_die-zuordnung-von-bereich-auf-sichtbarkeit-steht-seit-schritt-3-zweimal-gleich-da.md`),
und `aufteilung::sichtbar_im` ist dafür ersatzlos gestrichen worden. Derselbe Commitbereich
legt hier eine neue an.

## Vorschlag

Eine private Funktion neben den beiden, etwa `editor_ist_ansprechbar(&self) -> bool`, und beide
rufen sie. Der Rumpf beider Aufrufer schrumpft auf zwei Zeilen, und der Satz „dieselbe
Bedingung" im Kommentar wird von einer Behauptung zu einer Tatsache.

---
Resolved: Behoben am 260812-0745 auf dem im Datensatz vorgeschlagenen Weg.
`Anwendungsdelegierter::editor_ist_ansprechbar` (`crates/krk-ui/src/appkit/anwendung.rs:1488`)
ist jetzt die eine Fassung der Bedingung; `fokus_editor_holen` und `editor_umschalten` fragen
sie und schrumpfen auf je vier Zeilen.

Die Bedingung steht dabei in ihrer bejahenden Form — **der Editor ist ansprechbar, wenn er
steht oder wenn er eine Datei hält** —, statt zweimal als `ausgeblendet && !haelt_datei` mit
vorgezogenem `return false`. Beide Aufrufer lesen dieselbe Aussage, und der Satz "dieselbe
Bedingung" im Doc-Kommentar ist von einer Behauptung zu einer Tatsache geworden: der
Kommentar an `editor_umschalten` verweist auf die Funktion, und ihr eigener nennt beide
Aufrufer samt Kriterium.

**Keine Probe.** `anwendung.rs` trägt kein Prüfmodul, und die Funktion liest zwei Ivars des
Delegierten; eine Probe dafür bräuchte eine Instanz und damit
`MainThreadMarker::new_unchecked` (`issues/260810-1001_*`, als Lage angenommen;
`decisions/260810-1044_*`, zurückgestellt). Die Doppelung selbst ist stattdessen strukturell
weg: es gibt nur noch eine Stelle, die auseinanderlaufen könnte, nämlich keine.

Abnahme: `make check` Exit 0.
