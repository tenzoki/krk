Der Entpackschnitt ist kein Festpunkt: ein Archiv faellt wegen eines Beanspruchers, der selbst gefallen ist

---

`ohne_die_eigenen_ziele` (`crates/krk-ui/src/kommandos/kontextmenue.rs:612`) rechnet die Zielliste
**einmal** ueber alle Paare und filtert danach. Ein Archiv, dessen Zielordner nur von einem Paar
beansprucht wird, das selbst herausfaellt, faellt mit heraus — obwohl der Anspruch nach dem Schnitt
niemanden mehr hat. Aus `{a.zip, a.zip.zip, a.zip.zip.zip}` bleibt ein Paar, wo zwei kollisionsfrei
liefen. Der Doc-Kommentar schreibt das als beabsichtigt aus (Zeile 611), und die Nutzerzusage deckt
es nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1249, in der dritten Durchsicht der Runde 17
(`6faaa91..ddd41ff`).**

## Die Rechnung

```rust
fn ohne_die_eigenen_ziele(paare: Vec<(PathBuf, PathBuf)>) -> Vec<(PathBuf, PathBuf)> {
    let ziele: Vec<PathBuf> = paare.iter().map(|(_, ziel)| ziel.clone()).collect();
    paare
        .into_iter()
        .filter(|(archiv, _)| !ist_ziel_des_laufs(archiv, &ziele))
        .collect()
}
```

`ziele` entsteht vor dem Filter und wird danach nicht nachgezogen. Fuer die drei markierten Archive
`a.zip`, `a.zip.zip` und `a.zip.zip.zip` in einem Ordner rechnet `paar`
(`kontextmenue.rs:577-580`):

| Archiv | Zielordner |
|---|---|
| `a.zip` | `a` |
| `a.zip.zip` | `a.zip` |
| `a.zip.zip.zip` | `a.zip.zip` |

`ziele` = `{a, a.zip, a.zip.zip}`. Der Filter wirft `a.zip` (steht in `ziele`) und `a.zip.zip`
(steht in `ziele`) und behaelt `a.zip.zip.zip`. Uebrig bleibt **ein** Paar.

## Warum das mehr schneidet als die Zusage

Die Zusage des Nutzers lautet: „Trifft das Ziel eines Laufs eine seiner eigenen Quellen, faellt
diese Quelle aus dem Lauf." Nach dem ersten Schnittdurchgang ist `a.zip.zip` **keine Quelle des
Laufs mehr**, also ist `a.zip` auch kein Ziel des Laufs mehr. Es faellt fuer einen Anspruch, den
niemand mehr erhebt.

Nachgerechnet, was ohne den Ueberschuss geschaehe: `a.zip.zip.zip` entpackt in den neuen Ordner
`a.zip.zip`, `a.zip` in den neuen Ordner `a` — zwei Ziele, keine Beruehrung, kein Konfliktblatt.
Der Nutzer bekaeme zwei entpackte Archive statt eines.

Der Doc-Kommentar behauptet die Kuerzung ausdruecklich als richtig: „Aus `{a.zip, a.zip.zip,
a.zip.zip.zip}` bleibt das letzte" (`kontextmenue.rs:611`). Die Zeile steht im Beweis dafuer, dass
die Liste nicht leer zurueckkommt — der Beweis traegt, die daraus gezogene Verhaltensaussage nicht.

## Wie die Lage entsteht

Die anhaengende Endungsregel dieser Runde stellt sie selbst her: jeder Zip-Lauf ueber ein `a.zip`
legt `a.zip.zip` daneben. Zwei Laeufe ergeben die Kette aus drei. Die Kette ist selten, aber sie ist
diesem Baum eigen und kommt nicht von aussen.

## Was die Proben halten

`ein_archiv_das_zielordner_eines_anderen_ist_faellt_aus_den_quellen` (`kontextmenue.rs:1147`)
prueft die Kette der Laenge **zwei**, in der die Kuerzung richtig ist.
`drei_betroffene_archive_ergeben_drei_zielordner` (`kontextmenue.rs:980`) prueft drei Archive **ohne**
Kettenbezug. Die Kette der Laenge drei prueft keine Probe.

## Vorschlag

Zwei Wege.

1. **Den Schnitt zum Festpunkt machen.** Wiederholt filtern, bis sich nichts mehr aendert, oder die
   Paare von hinten nach vorn durchgehen und die Zielliste dabei fortschreiben. Bei zwei bis drei
   Paaren kostet das nichts, und die Regel entspricht danach dem Wortlaut der Zusage.
2. **Es so lassen und den Doc-Kommentar berichtigen.** Er sagt heute, das Verhalten sei die
   Zusage; richtig waere: der Schnitt ist absichtlich grob, weil eine Kette von drei den Aufwand
   nicht wert ist. Dann gehoert dazu, dass der Nutzer erfaehrt, welche Archive der Lauf nicht
   angefasst hat — siehe den Datensatz
   `260825-1249_*_der-schnitt-nimmt-markierte-eintraege-aus-dem-lauf-und-kein-wort-erreicht-den-nutzer.md`.

**Schwere:** mittel. Kein Verlust, aber der Lauf tut in einer erreichbaren Lage weniger, als der
Nutzer markiert hat, und der Doc-Kommentar erklaert das zur Absicht.

**Betroffen:** `crates/krk-ui/src/kommandos/kontextmenue.rs` (`ohne_die_eigenen_ziele`,
`entpackziel`).
