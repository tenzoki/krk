# Ein dritter Weg nach `aktives_setzen` hält den Bau nicht an, und keine Probe fängt ihn

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/decisions/260825-1725_*_nimmt-ein-klick-auf-die-tableiste-des-anderen-dateifensters-den-ersthelferrang-mit.md`; `crates/krk-ui/src/appkit/tabelle.rs:752-754` (der Satz), `:756-769` (`Rangmitnahme`), `:3414-3421` (`angefasst`); `crates/krk-ui/src/appkit/anwendung.rs:1289`, `:4577` (`aktives_setzen`), `:4656`, `:8863`, `:8901` (die zwei Proben), `:8946` (`keine_vierte_tuer_schreibt_das_aktive_dateifenster`); Commits `fd361d7`, `d3da6e3`

---

## Was ist

Die Runde 18 setzt eine Regel ohne Ausnahme: **jeder Weg, der `Fenstermodell::aktiv`
umschreibt, nimmt den Ersthelferrang mit, und zwar dort, wo AppKit ihn nicht selbst bewegt.**
Der Doc-Kommentar von `Rangmitnahme` sagt, was sie trägt (`tabelle.rs:752-754`):

```
/// **Zwei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.** Ein
/// dritter Weg in `angefasst` haelt damit den Bau an, statt sich stillschweigend
/// den einen oder den anderen Fall auszusuchen.
```

**Der Satz stimmt nicht.** `angefasst` nimmt `Rangmitnahme` als Argument; ein dritter
Aufrufer übersetzt tadellos, sobald er einen der zwei Werte hinschreibt — also gerade, indem
er sich einen von beiden aussucht. Vollständig ohne Auffangzweig ist die Fallunterscheidung
**in** `aktives_setzen` (`anwendung.rs:4577-4585`), und die hält gegen einen dritten *Wert*,
nicht gegen einen dritten *Weg*.

Was gegen den dritten Weg hält, ist eine Probe und nicht der Übersetzer:
`die_zwei_anfasswege_unterscheiden_sich_in_der_rangmitnahme` (`anwendung.rs:8901`) zählt
in `tabelle.rs` je einen `angefasst(Rangmitnahme::Appkit)` und `angefasst(Rangmitnahme::Krk)`
und insgesamt zwei Rufe von `angefasst`.

## Wo die Lücke wirklich sitzt

Nicht bei `angefasst`, sondern eine Ebene darüber. `aktives_setzen` hat heute zwei Aufrufer
(`anwendung.rs:1289` und `:4656`), und die Commit-Botschaft von `d3da6e3` sagt das auch
zu — „`aktives_setzen`, das weiter genau zwei Aufrufer hat". **Keine Probe hält diese Zahl.**
Die Zählprobe, die die Vollständigkeit tragen soll
(`keine_vierte_tuer_schreibt_das_aktive_dateifenster`), zählt `fenster_wechseln` und
`aktiv_setzen` außerhalb von `fenstermodell.rs`. Ein dritter Aufrufer von `aktives_setzen`
ändert an beiden Zahlen nichts: er ruft weiterhin dasselbe eine `aktiv_setzen` **innerhalb**
von `aktives_setzen`.

Damit gilt: wer künftig `self.aktives_setzen(seite, Rangmitnahme::Appkit)` an einer dritten
Stelle schreibt, an der AppKit den Rang gerade **nicht** bewegt, stellt genau den Zustand
wieder her, den `fd361d7` und `d3da6e3` beseitigt haben — und weder der Übersetzer noch eine
der vier Proben wird rot. Das ist derselbe Fehler zum dritten Mal, und die zwei ersten Male
haben je eine Runde gekostet.

## Was zu tun wäre

Zwei Dinge, und das erste ist eine Zeile:

1. **Den Satz an `Rangmitnahme` berichtigen.** Was hält, ist die Probe und nicht der Bau; der
   Modulkopf von `crate::quellbaum` verlangt ausdrücklich, dass die verbleibende Blindheit am
   Doc-Kommentar benannt und nicht im Namen überschrieben wird.
2. **Die zugesagte Zahl belegen.** Eine Aufruferzählung auf `aktives_setzen` — genau die Form,
   die `quellbaum` für „ein Abnahmekriterium sagt die Zahl selbst zu" vorsieht — steht neben
   den vier vorhandenen Proben und macht die dritte Tür rot. Sie ist billiger als jede andere
   Absicherung und misst die Stelle, an der die Regel wirklich hängt.

**Schwere:** gering bis mittel. Heute stimmt der Baum; zugesagt ist, dass er es bleibt, und
diese Zusage hält an dieser Naht nichts.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`.
