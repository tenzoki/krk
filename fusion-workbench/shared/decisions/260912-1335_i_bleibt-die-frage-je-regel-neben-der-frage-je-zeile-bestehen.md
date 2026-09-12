# Bleibt `Regel::ist_wirkungslos` neben `Vorschauzeile::wird_umbenannt` bestehen?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`, `260912-1149_*_was-geschieht-mit-einem-oeffentlichen-namen-ohne-rufer-im-betriebscode.md`

---

## Question

`Regel::ist_wirkungslos` (`crates/krk-core/src/stapelumbenennen/regel.rs`) beantwortet für die
ganze Regel dieselbe Frage, die `Vorschauzeile::wird_umbenannt`
(`crates/krk-core/src/stapelumbenennen/vorschau.rs:38-40`) je Zeile beantwortet: eine Regel ist
genau dann wirkungslos, wenn keine Zeile umbenannt wird. Zwei Antworten auf dieselbe Frage,
und `Vorschau::auszufuehren` benutzt die zweite zum Filtern, während die erste nur ihre eigene
Probe hat.

Der Datensatz `260826-1221_*` hat das am 260826 als Teilbefund festgehalten und offengelassen.
Die Sichtbarkeitsfrage daran ist am 260912 entschieden und in `00fb99a` umgesetzt — die
Methode trägt jetzt `pub(crate)` und `#[cfg(test)]`. **Die Doppelung ist davon nicht berührt**
und wird hier als eigene Frage geführt, weil sie keine Sichtbarkeitsfrage ist.

## Options

1. **`ist_wirkungslos` fällt.** Wer die Frage für die ganze Regel braucht, stellt sie über die
   Vorschau: keine Zeile wird umbenannt.
   - Pro: eine Antwort auf eine Frage, wie es dieses Projekt sonst hält.
   - Contra: die Probe, die heute an ihr hängt, muss über die Vorschau gehen und braucht dafür
     einen Bestand, den sie sich bauen muss; sie prüft heute die Regel allein.
2. **`ist_wirkungslos` wird auf die Vorschau zurückgeführt** statt eigenständig zu rechnen.
   - Pro: eine Quelle für die Antwort, der Name bleibt für den, der ihn lesbar findet.
   - Contra: eine Regel ohne Dateien hätte dann keine Vorschau und keine Antwort; heute
     antwortet sie aus ihren eigenen Feldern und braucht keine Dateien.
3. **Beide bleiben, und der Grund kommt an beide Stellen.** Die Fragen sind verwandt, aber
   nicht dieselbe: die eine rechnet aus den Feldern der Regel, die andere aus einem konkreten
   Namen.
   - Pro: billigster Zug, und die zwei Rechenwege haben verschiedene Voraussetzungen.
   - Contra: die Doppelung bleibt, und der nächste Leser stellt die Frage wieder.

## Constraints

- `Vorschauzeile::wird_umbenannt` hat einen Betriebsrufer (`Vorschau::auszufuehren`);
  `ist_wirkungslos` hat seit `00fb99a` nur noch die innere Probe und existiert im
  ausgelieferten Bau nicht mehr.
- Ein Verhalten ändert keine der drei Möglichkeiten. Es geht um die Frage, wie viele Wege
  dieselbe Auskunft geben.

---

## Berichtigung 260912-1355 — die Voraussetzung der Frage ist falsch

Die Frage oben übernimmt aus `260826-1221_*` den Satz, eine Regel sei **genau dann**
wirkungslos, wenn keine Zeile umbenannt wird. Nachgerechnet an den zwei Rümpfen stimmt das nur
in einer Richtung.

```
Regel::ist_wirkungslos      suchen.is_empty() && nummerierung.is_none()
Vorschauzeile::wird_umbenannt   kollision.is_none() && neu != alt
```

Die erste Frage stellt sich an die **Felder der Regel** und braucht keine einzige Datei; die
zweite an das **Ergebnis** und braucht die markierten Namen und den Bestand des Ordners.

- **Hin gilt es:** ist die Regel wirkungslos, dann ist `neu == alt` für jede Zeile, also wird
  keine umbenannt.
- **Zurück gilt es nicht.** Ein Suchtext, den kein Name trägt, lässt jede Zeile unverändert —
  keine wird umbenannt, und `ist_wirkungslos` ist trotzdem falsch, denn `suchen` ist nicht
  leer. Dasselbe bei einer Regel, die Namen ändern würde, deren Zeilen aber sämtlich eine
  Kollision tragen: `wird_umbenannt` ist überall falsch, die Regel ist nicht wirkungslos.

**Damit ist es keine Doppelung**, sondern eine Implikation in eine Richtung, und die zwei
Namen beantworten verschiedene Fragen: „ändert diese Regel überhaupt etwas" gegen „wird diese
Zeile angefasst". Die Möglichkeiten 1 und 2 oben ruhen auf der falschen Gleichsetzung und
tragen nicht.

Der Fehler stammt aus dem Befund vom 260826 und ist beim Anlegen dieses Datensatzes
ungeprüft übernommen worden.

---
Answered: `260912-1335_*_bleibt-die-frage-je-regel-neben-der-frage-je-zeile-bestehen.md` `## Berichtigung 260912-1355` — beide bleiben, denn es ist keine Doppelung: `ist_wirkungslos` fragt die Felder der Regel, `wird_umbenannt` das Ergebnis je Zeile, und die Implikation gilt nur in eine Richtung. An beide Stellen kommt der Satz, der die zwei Fragen auseinanderhält, damit der nächste Leser sie nicht wieder für dieselbe hält; ruled by user, Kai Stalmann <kai@stalmann.org>.

---
Implemented: cec8869 — beide Doc-Kommentare tragen je einen Absatz aus ihrer Blickrichtung, der die andere Methode namentlich nennt, die Richtung der Implikation benennt und die Gegenrichtung am nicht getroffenen Suchtext widerlegt. Die Aussage ist vor dem Schreiben nachgefahren und nicht nachgelesen worden: die drei Fälle sind durch das echte `vorschau()`, `Regel::anwenden` und `kollision::pruefen` geschickt, samt Kontrollfall mit einer wirksamen Regel, und die Hinrichtung ist am Rumpf von `Regel::anwenden` als total gezeigt. Der Verweis von `vorschau.rs` auf `ist_wirkungslos` steht als blosse Nennung und nicht als Doc-Link, weil `cargo doc` ohne `cfg(test)` baut. `make check` mit Exit 0.
