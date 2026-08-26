Zwei Proben schreiben `editor_rundweg` der Editor-Runde zu, die ihn anders genannt hat

---

Zwei Proben führen die Kennung `editor_rundweg` unter einer Überschrift, die sie der Editor-Runde
zuschreibt. Die Editor-Runde hat den Eintrag angelegt — unter dem Namen `editor_aus_vorschau`.
Den heutigen Namen hat der Nutzerentscheid vom 260823-0942 gegeben.

---

**Am Baum gelesen.** Der `coder` hat eine der beiden Stellen in seinem Bericht selbst als unsicher
benannt; die zweite steht daneben und ist in `28cbb7b` mitgeändert worden.

## Die zwei Stellen

`crates/krk-core/tests/belegung.rs:1968-1997` — Überschrift und Rumpf:

```
/// Die Kennungen, die die Editor-Runde der Belegungsdatei hinzugefuegt hat,
/// stehen darin.
…
fn die_kennungen_der_editor_runde_stehen_in_der_auslieferungsbelegung() {
    …
        "editor_rundweg",
```

`crates/krk-ui/src/belegungsmodell.rs:1445-1449`:

```
fn jede_neue_kennung_der_editor_runde_ist_umbelegbar() {
    const NEUE_KENNUNGEN: [&str; 13] = [
        "editor_rundweg",
```

## Was daran falsch ist und was nicht

Die Proben prüfen weiterhin genau das Richtige: dass die Kennung in der Auslieferungsbelegung
steht und umbelegbar ist. Falsch ist allein die Zuschreibung. Der Eintrag stammt aus der
Editor-Runde, sein Name nicht.

Das ist keine Wortklauberei in einem Baum, der seine Runden als Belegkette führt: wer später
fragt, was die Editor-Runde geliefert hat, bekommt hier eine Kennung genannt, die es damals nicht
gab, und findet sie in keinem Datensatz jener Runde.

## Empfehlung

Die Zuschreibung an einer Stelle richtigstellen, statt sie zu tilgen. In `belegung.rs` genügt ein
Satz im Doc-Kommentar: „`editor_rundweg` steht seit dem 260823-0942 unter diesem Namen; die
Editor-Runde hat den Eintrag als `editor_aus_vorschau` angelegt." Für die zweite Probe reicht
derselbe Satz oder ein Name ohne Rundenbezug.

**Schwere:** Low.

**Filed by:** coderev

---

In Arbeit: 260823-1137 durch coder. Beide Zuschreibungen sind richtiggestellt und
nicht getilgt, wie der Datensatz es empfiehlt. In `crates/krk-core/tests/belegung.rs`
traegt der Doc-Kommentar von
`die_kennungen_der_editor_runde_stehen_in_der_auslieferungsbelegung` jetzt einen Absatz:
den Eintrag hat die Editor-Runde angelegt, seinen Namen der Nutzerentscheid vom
260823-0942; bis dahin hiess er `editor_aus_vorschau`. In
`crates/krk-ui/src/belegungsmodell.rs` steht derselbe Satz an
`jede_neue_kennung_der_editor_runde_ist_umbelegbar`. Die Proben selbst sind unveraendert.
`resources/default-keymap.toml` fuehrt die Umbenennung schon; dort war nichts zu tun.
Bleibt zum Schliessen mit dem Commit.

---
Resolved: `52fba42` — behoben, `make check` gibt 0 zurück. Durchsicht: die Befunde stammen aus
`shared/reviews/260823-0735-coderev-einblenden-erreicht-den-schirm.md` und
`shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md`; was im Einzelnen getan ist, steht
im Protokoll `shared/history/260823-1137-coder-acht-befunde-aus-zwei-durchsichten.md`.
