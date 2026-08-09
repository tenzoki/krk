# Die Zusage über den gehaltenen Stand hat einen zweiten Eingang ohne Normalisierung

---
**Domain:** code
**Schwere:** High
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/editormodell.rs:553-557` (`bearbeiten`), `:683-707` (`treffer_ersetzen`), `:717-735` (`alle_treffer_ersetzen`)
**Cross-references:** `crates/krk-core/src/text/datei.rs:22-45` (Modulkopf), `crates/krk-core/src/text/mod.rs:27-33`, `crates/krk-core/src/text/zeilen.rs:122-152`, `decisions/260808-0021_a_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`, S9, S15, S26, S37

---

## Der Befund

S9 hat die Zusage formuliert und die eine Normalisierungsstelle gebaut:

> **Der gehaltene Stand des Editors ist gültiges UTF-8 ohne Bytefolgenmarke und
> mit `\n` als einzigem Zeilenende.** (`datei.rs:24-25`)
>
> **Wer Text in den Stand bringt, der nicht aus [`einlesen`] kommt, führt ihn
> durch [`in_gehaltene_form`].** (`datei.rs:39-40`)

Die Prüfung hält: `in_gehaltene_form` wird im Programmtext an genau einer Stelle
gerufen, in `einlesen` (`datei.rs:319`). Eine zweite Normalisierungsstelle ist
nicht entstanden.

**Es sind aber drei Eingänge in den Stand entstanden, die keine Normalisierung
haben.** Alle drei stehen seit S15 in `editormodell.rs`:

```rust
// editormodell.rs:553-557
pub fn bearbeiten(&mut self, neuer_stand: String) {
    self.stand = neuer_stand;      // ungeprüft
    …
}

// editormodell.rs:688
let ersetzung = suche::einen_ersetzen(&self.stand, &gesucht, ersatz, angesteuert);

// editormodell.rs:723
let ersetzung = suche::alle_ersetzen(&self.stand, &gesucht, ersatz);
```

Keiner der drei ruft `in_gehaltene_form`, keiner nennt die Pflicht in seinem
Doc-Kommentar.

`datei.rs:41-45` benennt als anstehenden Fall nur einen:

> Der Fall, der ansteht, ist der Ersatztext des Suchen-und-Ersetzens aus C5
> (Schritt 37): er kommt aus einem Eingabefeld und kann ein `\r` tragen, wenn er
> hineinkopiert wurde.

Der größere Fall fehlt: **der Stand, den die `NSTextView` zurückgibt.** Eine
`NSTextView` bewahrt eingefügten Text zeichengetreu auf. Wer Text aus einem
Windows-Projekt in den Editor einfügt, bringt `\r\n` in den `NSTextStorage`, und
`bearbeiten` nimmt ihn unverändert entgegen.

## Warum das zählt

Drei Stellen rechnen auf die Zusage, ohne sie zu wiederholen (`datei.rs:32-37`),
und alle drei rechnen dann falsch:

- **`Zeilenindex::inhalt_der_zeile`** zieht vom Anfang der nächsten Zeile genau
  **ein** Byte ab, mit der ausgeschriebenen Begründung „Genau ein Byte, weil der
  gehaltene Stand kein `\r\n` mehr trägt" (`zeilen.rs:144-146`). Bei `\r\n`
  liefert sie den Zeileninhalt mit einem angehängten `\r`. Eine Textmarke aus C6,
  die nach dem Einfügen gesetzt wird, merkt sich diesen Inhalt und findet sich
  beim Sprung selbst nicht wieder: `wiederfinden` vergleicht ganze Zeilen.
- **`suche`** sucht buchstäblich. Ein Suchtext, der ein Zeilenende überspannt,
  findet nichts mehr.
- **`sicherungsform`** wandelt bewusst keine Zeilenenden (`datei.rs:356-358`).
  Das `\r\n` geht damit auf die Platte, und die Aussage des Modulkopfes ist
  gebrochen:

  > KRK schreibt beim Sichern **immer** Unix-Zeilenenden … unabhängig von der
  > Form, die die Datei mitbrachte. (`datei.rs:49-52`)

  Das ist die Antwort des Nutzers vom 260808-0043, und sie hält dann nicht mehr.

Der Fall ist heute latent: `bearbeiten` hat noch keinen Aufrufer, weil die
Anbindung der Textfläche an das Modell mit S26 kommt, und die beiden
Ersetzungswege bekommen ihren Befehl mit S37. Latent heißt aber nicht harmlos —
die Verpflichtung steht in einem Modulkopf einer anderen Kiste, und der Schritt,
der sie einlösen müsste, weiß heute nichts davon.

## Vorschlag

Die Normalisierung gehört an den Eingang des Modells und nicht an jeden Aufrufer:

```rust
pub fn bearbeiten(&mut self, neuer_stand: String) {
    self.stand = datei::in_gehaltene_form(neuer_stand);
    …
}
```

`in_gehaltene_form` gibt einen Text, der die Form schon hat, ohne eine einzige
Kopie zurück (`datei.rs:335-337`); der Normalfall kostet einen `starts_with` und
einen `contains('\r')`. Bei einer Datei an der 16-MB-Grenze ist das ein
Durchlauf je Tastendruck. Ob das gegen die Zusagen aus C8 trägt, ist zu messen;
die Alternative wäre, die Wandlung an die Stellen zu setzen, an denen fremder
Text hereinkommt (Einfügen, Ersetzen), und das wären wieder mehrere Stellen
statt einer.

Für die beiden Ersatztexte ist die Antwort einfacher, weil sie kurz sind:
`treffer_ersetzen` und `alle_treffer_ersetzen` führen ihren `ersatz` durch
`in_gehaltene_form`, bevor sie ihn an `suche` geben.

In jedem Fall gehört der Fall „Text aus der `NSTextView`" in die Aufzählung in
`datei.rs:39-45`, die heute nur S37 nennt.

Gemeldet von: `coderev`, Durchsicht Turn 2.
