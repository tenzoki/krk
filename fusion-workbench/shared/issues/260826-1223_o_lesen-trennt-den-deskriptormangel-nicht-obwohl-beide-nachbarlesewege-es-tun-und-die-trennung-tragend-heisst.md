`lesen` trennt den Deskriptormangel nicht, obwohl beide Nachbarlesewege es tun und die Trennung „tragend" heißt

---

Von den drei Wegen durch `ohne_warten_oeffnen` trennen zwei `EMFILE`/`ENFILE` von den übrigen Fehlern und begründen das als tragend; der dritte, `lesen`, wirft sie mit allem anderen in `Textstand::KeinGueltigesZiel`. Der Editor sagt dem Nutzer dann etwas über **die Datei**, wo etwas über den **Prozess** zu sagen wäre.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Regel und wo sie steht

`crates/krk-core/src/text/datei.rs:549-557`, am Doc-Kommentar von `Lesehindernis`:

> **`Deskriptormangel` wird hier getrennt und nicht beim Aufrufer** […] Der Unterschied ist tragend und nicht bloss genauer: `EMFILE` und `ENFILE` sagen etwas ueber den Prozess und nichts ueber die Datei. Wer sie mit den uebrigen Fehlern zusammenzoege, entschiede negativ, wo nichts entschieden ist — derselbe Fehlgriff, den der Durchlauf ueber den Unterbaum seit der Runde 10 vermeidet.

`bis_zur_grenze_lesen` (`datei.rs:620-628`) und `anlesen` (`datei.rs:692-700`) halten sie, beide über `crate::verzeichnis::sys::ist_deskriptormangel`.

## Was `lesen` stattdessen tut

`crates/krk-core/src/text/datei.rs:434-440`:

```rust
let mut datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
    Ok(datei) => datei,
    Err(fehler) => {
        let fehlt = fehler.kind() == io::ErrorKind::NotFound;
        return kein_ziel(fehler.to_string(), fehlt);
    }
};
```

Jeder Fehler außer `NotFound` wird zu `Textstand::KeinGueltigesZiel { fehlt: false }`. Dessen Doc-Kommentar (`datei.rs:329-330`) sagt, was der Wert bedeutet: „Nichts, was ein Texteditor oeffnen koennte: ein Ordner, ein fehlender Pfad, ein fehlendes Leserecht, alles, was keine gewoehnliche Datei ist." Ein erschöpfter Deskriptorvorrat ist nichts davon.

`oeffnen` (`datei.rs:533-536`) übersetzt den Wert in `Abweisung::KeinGueltigesZiel`, und `Abweisung::meldung` (`datei.rs:261-266`) setzt daraus den Satz für die Statuszeile:

> `<pfad>` lässt sich nicht im Editor öffnen: Too many open files

Der Systemfehler steht mit im Satz, also ist die Auskunft nicht verloren; falsch ist der Rahmen, in den sie gestellt wird. Für den Notizzettel gilt dasselbe über `Zugang::text_laden`.

## Erreichbar

Ja, und nicht nur theoretisch. Derselbe Prozess führt seit der Runde 11 einen Durchlauf über den Unterbaum, der je Kandidat einen Dateideskriptor hinzunimmt (`CLAUDE.md`, Abschnitt „Was man nicht sieht"), und seit dem 260826 steht „Deep" ab Werk auf ein, also läuft er schon beim ersten Anschlag im Dateifenster an. Ein `f4` in dieser Lage ist genau der Fall.

## Vorschlag

Kein fünfter `Textstand`-Wert — die Aufzählung ist mit Bedacht klein. Der billigste Schnitt ist ein Feld neben `fehlt`, also `mangel: bool`, gespeist aus derselben einen Regel `sys::ist_deskriptormangel`, mit derselben Begründung, mit der `fehlt` ein Feld und kein Wert ist (`datei.rs:336-343`). Der Editor bekäme daraus einen Satz, der sagt, dass es an KRK und nicht an der Datei liegt, und der Notizzettel unterschiede „nichts da" von „gerade nicht lesbar" — er legt heute im zweiten Fall nichts beiseite und arbeitet mit einem leeren Zettel weiter.

Verwandt und nicht dasselbe: `shared/issues/260816-1932_*_ein-deskriptormangel-beendet-den-durchlauf-still-und-die-statuszeile-nimmt-den-lesehinweis-zurueck.md` behandelt den Durchlauf über den Unterbaum, dieser Befund den Leseweg des Editors und des Notizzettels.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.
