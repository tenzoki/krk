Das Abnahmekriterium von S13b verlangt einen grünen Test, den erst S13c grün machen kann

---

S13b schreibt das neue Feld `gehalten_von` in `resources/default-keymap.toml`.
Der Parser weist unbekannte Felder ab, und die eingebettete
Auslieferungsbelegung stürzt damit beim ersten Zugriff ab. Das
Abnahmekriterium von S13b verlangt trotzdem, dass
`cargo test -p krk-core --test belegung` mit 0 endet. Das ist nach der eigenen
Aufteilung des Plans unmöglich: S13b trägt die Daten ein, S13c bringt dem
Parser das Feld bei.

---

## Der Nachweis

`crates/krk-core/src/tasten/belegung.rs:662-670`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Eintrag {
    id: String,
    name: String,
    tasten: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reserviert_fuer: Option<String>,
}
```

`gehalten_von` steht dort nicht, und `deny_unknown_fields` macht daraus einen
Lesefehler statt eines stillen Überlesens. `belegung.rs:64-69` liest die Datei
über `include_str!` und bricht bei einem Lesefehler ab:

```rust
static AUSLIEFERUNG: LazyLock<Belegung> = LazyLock::new(|| {
    let datei: Belegungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
        .expect("die eingebettete Auslieferungsbelegung ist kein gueltiges TOML");
```

Betroffen ist damit nicht eine Prüfung, sondern jede, die `Belegung` anfasst,
und ebenso jeder Programmstart.

Zweitens meldet sich `crates/krk-core/tests/belegung.rs`, Prüfung
`die_ab_werk_freien_kombinationen_kommen_nicht_vor`. Sie führt `cmd+c` und
`cmd+v` als ab Werk frei auf und verlangt, dass beide auf `Unbelegt` oder
`Sprungmarke` fallen. Seit S13b tragen sie die Textbefehle des Menüs
"Bearbeiten"; die Zusage, die diese Prüfung schützt, ist mit dem
Nutzerentscheid vom 260805-0000 abgelöst.

## Warum es zählt

Es ist kein Streit über den Zuschnitt der Schritte. Die Aufteilung ist richtig
und der Plan begründet sie: die Belegungsdatei gehört dem `ontocoder`, das
Menü und der Parser gehören dem `coder`, und die Datenänderung kommt einmal
und vor dem Schritt, der sie liest. Falsch ist allein das Abnahmekriterium,
das sich einen Zustand zusagen lässt, den der eigene Zuschnitt ausschließt.
Wer S13b nach Vorschrift abnimmt, muss den Schritt für gescheitert erklären,
obwohl die Daten stimmen.

Dieselbe Form trägt bereits
`issues/260804-1649_c_das-appkit-abnahmekriterium-von-s15-ist-so-nicht-erfuellbar.md`.

## Was zu tun ist

Am Plan: das Abnahmekriterium von S13b auf das zurücknehmen, was ohne Code
prüfbar ist — gültiges TOML, die Zahl der Blöcke, die Kennungen und
Kombinationen, die Konfliktfreiheit am vollständigen Eintrag, `plutil`. Den
grünen Test schuldet S13c, dessen Abnahmekriterium ihn ohnehin schon nennt.

Am Code, durch S13c und dort bereits vorgesehen: `Eintrag` bekommt das Feld
`gehalten_von: Option<String>`, und `Funktion` reicht es durch. Nicht
vorgesehen, aber fällig: die Prüfung
`die_ab_werk_freien_kombinationen_kommen_nicht_vor` verliert `cmd+c` und
`cmd+v` aus ihrer Liste, und ihr Kommentar zieht auf den Entscheid vom
260805-0000 nach. `shift+delete` und `return` bleiben.

## Dringlichkeit

Bindet die Abnahme von S13b, nicht dessen Ausführung. Der rote Zustand hält
bis S13c und ist die angekündigte Folge der Aufteilung; der Nutzer hat ihn beim
Auftrag zu S13b ausdrücklich als Befund für S13c vorweggenommen.

---

**Aufgefallen bei:** der Ausführung von S13b am 260805-0637, beim Lesen des
Parsers vor dem Schreiben des neuen Feldes.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S13b, S13c),
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1649_c_das-appkit-abnahmekriterium-von-s15-ist-so-nicht-erfuellbar.md`

---
Resolved: Das Abnahmekriterium von S13b verlangt den grünen Lauf nicht mehr. Der Plan schreibt stattdessen aus, dass `cargo test -p krk-core --test belegung` von der Abnahme dieses Schrittes bis zur Abnahme von S13c rot bleibt, und warum: `Eintrag` trägt `#[serde(deny_unknown_fields)]` und kennt `gehalten_von` nicht, und `AUSLIEFERUNG` liest die Datei über `include_str!` mit einem `expect` daneben, sodass ein Lesefehler jede Prüfung mitreißt. Das Feld beizubringen ist Code und gehört S13c, dessen Abnahmekriterium den grünen Lauf jetzt ausdrücklich als "ab hier zum ersten Mal seit S13b wieder grün" führt. Dieselbe Form trug der geschlossene Defekt `260804-1649_c_das-appkit-abnahmekriterium-von-s15-ist-so-nicht-erfuellbar.md`.
