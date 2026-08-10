Ein `cmd+z` nach einem Ersetzen loescht den Suchlauf, den das Ersetzen eigens aufgebaut hat
---
`umkehren` geht ueber `Editormodell::bearbeiten`, und das setzt `self.suchlauf = None`. Die beiden Ersetzungswege bauen den Suchlauf danach eigens neu auf, damit `cmd+g` und `shift+cmd+r` weiterlaufen; das Rueckgaengig wirft ihn weg. Der Nutzer bezahlt ein `cmd+z` mit seiner laufenden Suche, und niemand sagt es ihm an.
---
**Schwere:** Mittel
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/editormodell.rs`

## Belegstellen

`crates/krk-ui/src/appkit/editor.rs:1645-1660`, in `umkehren`:

```rust
let gewandelt = self
    .ivars()
    .modell
    .borrow_mut()
    .bearbeiten(punkt.stand.clone());
```

`crates/krk-ui/src/editormodell.rs:941-947`:

```rust
pub fn bearbeiten(&mut self, neuer_stand: String) -> bool {
    let war_gehalten = datei::ist_in_gehaltener_form(&neuer_stand);
    self.stand = datei::in_gehaltene_form(neuer_stand);
    self.abweichung = true;
    self.suchlauf = None;
    !war_gehalten
}
```

Dass der Suchlauf nach einem Ersetzen weiterlaufen soll, ist keine Vermutung: beide Ersetzungswege bauen ihn eigens neu auf (`editormodell.rs:1186ff` fuer den einzelnen Treffer, `:1223ff` fuer alle). Genau das ist der Zweck, den `cmd+z` hier zunichte macht.

## Fehlszenario

1. `cmd+f`, Suchtext „foo", der Kopf meldet „1 von 7".
2. `shift+cmd+r` ersetzt den ersten Treffer. Der Suchlauf steht neu, der Kopf meldet den naechsten.
3. `cmd+z`. Der Text ist zurueck — und der Suchlauf ist fort.
4. Ein zweites `shift+cmd+r` antwortet `Editormeldung::KeineSuche`, also „es laeuft keine Suche" (`editor.rs:2081-2083`). Ein `cmd+f` bietet das Suchfeld leer an, weil der Suchtext aus dem Modell kommt.

Der Nutzer wollte eine Ersetzung zuruecknehmen und hat seine Suche verloren.

## Warum es neu ist

Vor diesem Diff leerte die eine Schreibstelle den Rueckgaengigstapel bei jedem Anlass; ein `cmd+z` nach einem Ersetzen tat nichts, also konnte es auch den Suchlauf nicht wegnehmen. Mit `Verlauf::Traegt` tut es jetzt etwas — und dabei eines zu viel.

## Vorschlag

Der Suchlauf ist nach einem Rueckgaengig eines Ersetzens genau derjenige Fall, in dem die Trefferliste **ausrechenbar** ist: der Text ist der von vorher, also ist die Trefferliste die von vorher. Zwei Wege:

1. `umkehren` baut den Suchlauf nach dem `bearbeiten` neu auf, mit demselben Suchtext und derselben angesteuerten Stelle. Das ist der Weg, den die beiden Ersetzungswege schon gehen, und er steht damit nicht zum zweiten Mal woanders.
2. Der Umkehrpunkt traegt den Suchlauf mit. Das ist weniger Rechnung, kostet aber eine zweite Wahrheit darueber, welche Treffer zu welchem Text gehoeren, und der Stapel haelt ohnehin schon zu viel (siehe `issues/260810-1241_*_der-rueckgaengigstapel-haelt-je-eigener-handlung-eine-ganze-abschrift-…`).

Weg 1 ist der empfohlene. Was **nicht** zu tun ist: `bearbeiten` den Suchlauf stehen lassen zu lassen — beim Tippen ist das Loeschen richtig, und `bearbeiten` kann Tippen und Rueckgaengig nicht unterscheiden. Der Anlass ist beim Aufrufer bekannt, genau wie bei `Verlauf`.
