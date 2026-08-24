`ablage/mod.rs` sagt „drei der fünf TOML-Dateien" tragen `deny_unknown_fields` und nennt im selben Absatz eine vierte

---

Der Modulkopf der Ablage (`crates/krk-core/src/ablage/mod.rs:145-152`) trägt seit Schritt 8 der
Runde 16 einen Absatz, der sich selbst widerspricht:

```
//! - **Ein oberster Schluessel, den der Leser nicht kennt**, ist ein `Err` und
//!   kein stiller Auslieferungszustand. Das leistet
//!   `#[serde(deny_unknown_fields)]` an der jeweiligen Struktur, und drei der
//!   fuenf TOML-Dateien tragen es: `Belegungsdatei`, `Einstellungsdatei` und
//!   seit dem 260821 auch [`Lesezeichenliste`]. `readers.toml` traegt es
//!   ebenfalls, an `leseprofil::datei::Profildatei`, und geht seit der Runde 16
//!   ueber denselben Ladeweg.
```

Vier der fünf tragen es, nicht drei. Der Satz stand vor der Runde 16 als „drei der **vier**"
da und war damit richtig; die Runde hat den Nenner von vier auf fünf gezogen und den Zähler
stehen lassen, während sie den vierten Träger im nächsten Satz nachreicht.

Die fünfte ist `session.toml`; sie trägt die Angabe bewusst nicht, und warum, ist die offene
Frage
`shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`.
`keymap.toml` geht über `Belegungsdatei`, die die Angabe trägt.

---

**Warum es zählt.** Der Absatz ist die Stelle, an der ein Leser abliest, welche Ablagedatei
einen unbekannten Schlüssel abweist und welche nicht. Wer die vier Namen zählt und die Zahl
drei liest, hält eine der vier für einen Zusatz ohne Angabe und sucht die dritte an der
falschen Stelle.

**Was zu tun ist.** Ein Satz mit einer Aufzählung und einer Zahl, die zusammenpassen: „vier der
fünf TOML-Dateien tragen es: `Belegungsdatei`, `Einstellungsdatei`, seit dem 260821
`Lesezeichenliste` und seit der Runde 16 `leseprofil::datei::Profildatei`; `session.toml` trägt
es nicht, und warum, ist die offene Frage …". Damit steht die Ausnahme benannt da und nicht als
Rest einer Subtraktion.

## Die Nachbarzeile trägt denselben Fehler und ist schon erfasst

Vier Zeilen tiefer (`crates/krk-core/src/ablage/mod.rs:158`) steht: „Die drei uebrigen tragen
[`Leerbefund::Vorgabe`]: zwei davon pflegt der Nutzer von Hand". Es sind vier übrige und drei
von Hand gepflegte, und `pfade.rs:244` schreibt es richtig aus („Die vier uebrigen
TOML-Dateien und die zwei Zettel").

**Diese Zeile ist nicht neu zu erfassen:** sie steht in der Tabelle des offenen Datensatzes
`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`,
dort unter der Zeilennummer `:154`. Mit den Nachträgen aus Schritt 8 der Runde 16 ist sie auf
`:158` gerutscht, und die Leseanweisung jenes Datensatzes („der Satz ist auf ‚die vier übrigen
TOML-Dateien und die zwei Zettel' zu lesen") stammt aus der Zeit der sechs Ablagedateien. Wer
den einen Absatz anfasst, zieht beide Sätze in einem Zug nach und schreibt die neue
Zeilennummer im dortigen Datensatz fest.

**Schwere:** niedrig.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1650.

**Betroffen:** `crates/krk-core/src/ablage/mod.rs` (Modulkopf, Zeile 145-152; die Nachbarzeile `:158` gehört zu `shared/issues/260821-1023_o_…`)

**Domain:** code

---
Resolved:
