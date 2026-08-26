Der freie Name gibt nach tausend Versuchen einen belegten Namen heraus

---

`freier_name` sucht bis `HOECHSTE_KOPIE` und liefert danach genau den Namen, den es im letzten
Durchgang als belegt vorgefunden hat. Die Funktion heisst "freier Name" und antwortet dann mit
einem belegten; der Rufer kann den Unterschied nicht sehen, denn es gibt keinen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-core/src/operation/umbenennen.rs:141-158`:

```rust
for nummer in 1..=HOECHSTE_KOPIE {
    let vorschlag = …;
    if !ziel.with_file_name(&vorschlag).exists() {
        return vorschlag;
    }
}
format!("{stamm} Kopie {HOECHSTE_KOPIE}{endung}")
```

Die letzte Zeile bildet denselben Namen, den die letzte Runde der Schleife eben gepruefet und
als vorhanden befunden hat.

## Was der Rueckgabewert danach anrichtet

Einziger Rufer ist `Steuerung::konflikt_loesen` (`operation/fortschritt.rs:358-360`) fuer
`Konfliktregel::AutomatischUmbenennen`. Der Name geht als `Konfliktantwort::UmbenennenIn` an drei
Stellen weiter, und keine fragt ein zweites Mal nach:

- `ziel_klaeren` (`operation/mod.rs:446-452`) → `Zielentscheid::Nach(ziel.with_file_name(name))`
- `zielarchiv_klaeren` (`operation/zippen.rs:300-306`) → dasselbe, danach schneidet
  `File::create` (`zippen.rs:208`) den Treffer ab
- `zielordner_klaeren` (`operation/entpacken.rs:217-223`) → dasselbe, danach schreibt
  `create_dir_all` in den getroffenen Ordner hinein

Der Doc-Kommentar an `HOECHSTE_KOPIE` (`umbenennen.rs:128-134`) begruendet die Grenze als Schutz
gegen eine Endlosschleife, "falls das Dateisystem jeden Namen als vorhanden meldet". Genau in
dieser Lage liefert die Funktion dann den einen Namen, von dem sie sicher weiss, dass er
vorhanden ist.

## Verhaeltnis zu einem schon offenen Datensatz

`shared/issues/260825-1130_*_ein-selbst-getippter-name-im-konfliktblatt-kann-einen-belegten-treffen-und-wird-ohne-rueckfrage-ueberschrieben.md`
haelt denselben Ausgang fuer den **selbst getippten** Namen fest und schreibt dort ausdruecklich:
"Bei `Konfliktregel::AutomatischUmbenennen` liefert `freier_name` einen freien Namen, und die
Zusage haelt." Dieser Datensatz zeigt die eine Lage, in der sie es nicht tut. Er ist deshalb kein
zweiter Datensatz zu derselben Sache, sondern die Berichtigung ihrer Voraussetzung.

## Was zu tun waere

Der Fall braucht einen Rueckgabewert, der "kein freier Name gefunden" ausdruecken kann —
`Option<String>` —, und der Rufer in `konflikt_loesen` uebersetzt ihn dann in
`Konfliktantwort::Ueberspringen` mit einem Grund. Ein belegter Name als Ergebnis ist die einzige
Antwort, die schlechter ist als gar keine.

## Umfang

`krk-core`, `operation/umbenennen.rs` und `operation/fortschritt.rs`.
