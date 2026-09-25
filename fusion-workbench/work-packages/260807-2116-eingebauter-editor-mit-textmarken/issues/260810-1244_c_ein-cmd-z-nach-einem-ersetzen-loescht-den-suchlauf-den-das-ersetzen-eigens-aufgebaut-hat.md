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

---
Resolved: Weg 1, der empfohlene. `Editorbereich::umkehren` merkt sich den
Suchtext vor dem Ruf an `Editormodell::bearbeiten` und bildet den Suchlauf danach
im wiederhergestellten Stand neu:

```rust
let gesucht = modell.suchlauf().map(|lauf| lauf.gesucht().to_owned());
let gewandelt = modell.bearbeiten(wiederhergestellt);
if let Some(gesucht) = gesucht {
    let ab_versatz = koordinaten::in_bytes(modell.stand(), punkt.auswahl.location);
    let _ = modell.suche_starten(&gesucht, ab_versatz);
}
```

**Gerechnet wird ueber `suche_starten` und nicht mit einer eigenen Trefferliste
daneben** — derselbe Weg, den `cmd+f` und die beiden Ersetzungswege gehen. Die
Stelle, ab der gesucht wird, ist die, an die dasselbe Rueckgaengig die
Schreibmarke setzt (`punkt.auswahl.location`, umgerechnet in Bytes); damit
steuert der wiederhergestellte Suchlauf den Treffer an, an dem der Nutzer nach
dem `cmd+z` steht, und `cmd+g` sowie `shift+cmd+r` laufen von dort weiter.

**`bearbeiten` bleibt unangefasst**, wie der Datensatz es verlangt: es kann
Tippen und Rueckgaengig nicht unterscheiden, und beim Tippen ist das Loeschen
richtig. Der Anlass ist beim Aufrufer bekannt, genau wie bei `Verlauf`. Der Weg 2
(der Umkehrpunkt traegt den Suchlauf mit) ist nicht genommen: er waere eine
zweite Wahrheit darueber, welche Treffer zu welchem Text gehoeren, und der Stapel
haelt nach `260810-1241` bewusst nur noch den geaenderten Bereich.

**Was daran Nutzerarbeit bleibt**, ist die Wirkung am laufenden Buendel: dass der
Kopf nach `cmd+f`, `shift+cmd+r`, `cmd+z` wieder „Treffer n von 7" meldet und ein
zweites `shift+cmd+r` nicht mehr `Editormeldung::KeineSuche` antwortet. Am Code
ist es nachgezeichnet und an den Wegen belegt, die es benutzt; gefahren ist es
nicht, aus dem Grund, der in `CLAUDE.md` unter „Was man nicht sieht" steht. Eine
Probe ohne Fenster kann es nicht halten, weil `umkehren` eine Methode der
AppKit-Klasse ist und die Textflaeche braucht.

Verification: `cargo build --workspace` exit 0, `cargo test --workspace` exit 0,
`cargo clippy --workspace --all-targets` exit 0,
`cargo fmt -p krk-ui -- --check` exit 0.
