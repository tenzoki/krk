# Die Probe für die flache Dateiliste liest eine Datei, C2.9 spricht über den Baum

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C2.9; `crates/krk-ui/src/tabs.rs`, `die_dateiliste_bleibt_flach_und_hat_vier_spalten`; `crates/krk-ui/src/quellbaum.rs`, `quelldateien`

---

## Befund

C2.9 sagt vier Dinge zu: keine `NSOutlineView`, kein Aufklappzeichen, **keine zweite Tabellenklasse** und keine fünfte Spalte. Die Probe prüft zwei davon, und die zweite nur in einer Datei:

```rust
assert_eq!(crate::spalten::Spalte::ALLE.len(), 4);
let quelltext = include_str!("appkit/tabelle.rs");
assert_eq!(quelltext.matches("NSOutlineView").count(), 0, …);
```

`include_str!` bindet genau `crates/krk-ui/src/appkit/tabelle.rs`. Eine `NSOutlineView` in `appkit/vorschau.rs`, in einem Blatt oder in einer neuen Datei daneben ließe die Probe grün. „Keine zweite Tabellenklasse" ist damit gar nicht geprüft — eine zweite Klasse stünde ja gerade **nicht** in `tabelle.rs`.

Die Kiste führt für genau diesen Fall `crate::quellbaum::quelldateien`, und der Modulkopf dort schreibt aus, warum eine Zählprobe den Baum liest und nicht eine Datei. Drei Proben derselben Runde tun es (`die_regel_hat_genau_einen_aufrufer`, `die_sprungmarke_steht_nirgends_mehr_im_baum`, `die_zeichenregel_und_der_vergleich_…`); diese eine nicht.

## Was zu tun wäre

Die Nadel `NSOutlineView` über `quelldateien()` zählen statt über `include_str!`, und dabei die Nadel zusammengesetzt schreiben, weil die Probe dann in dem Baum liegt, den sie liest. Der Bezug auf `spalten::Spalte::ALLE` bleibt, wie er ist: er zählt den Aufzählungstyp und nicht seinen Namen im Text.

---
Resolved: `die_dateiliste_bleibt_flach_und_hat_vier_spalten` zählt die Nadel jetzt über `crate::quellbaum::quelldateien()` und nicht mehr über `include_str!("appkit/tabelle.rs")`. Die Nadel steht mit `concat!` zusammengesetzt da, weil die Probe in dem Baum liegt, den sie liest; gelesen werden nur Code-Zeilen, also nicht die Nennungen im Doc-Kommentar daneben. Der Bezug auf `spalten::Spalte::ALLE` bleibt unverändert: er zählt den Aufzählungstyp und nicht seinen Namen im Text.

`NSTableView` ist ausdrücklich **keine** Nadel, und der Doc-Kommentar sagt jetzt warum: KRK hat mehrere Tabellen — die Belegungsansicht und das Blatt zum Stapelumbenennen —, und eine Zählung darüber sähe sie als Fundstellen. Gefragt ist die Aufklappansicht, und die heißt in AppKit `NSOutlineView`.

Die verbleibende Blindheit steht am Doc-Kommentar: eine Aufklappansicht, die niemand so nennt, weil sie von Hand aus Zeilen mit Einzug gebaut wäre, fände diese Zählung nicht.

Berührte Datei: `crates/krk-ui/src/tabs.rs`.
