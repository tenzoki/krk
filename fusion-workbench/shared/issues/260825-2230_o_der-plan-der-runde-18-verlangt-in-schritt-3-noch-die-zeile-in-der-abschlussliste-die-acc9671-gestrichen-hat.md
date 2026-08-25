# Der Plan der Runde 18 verlangt in Schritt 3 noch die Zeile in der Abschlussliste, die `acc9671` gestrichen hat

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md:243` (Entwurf), `:251` (Abnahmekriterium); `shared/issues/260825-2127_*_ein-gepackter-eintrag-mit-ersatzdatum-steht-in-der-liste-der-uebersprungenen.md` (Weg 1, `Resolved:`); `crates/krk-core/src/operation/zippen.rs:650-692` (`zeit_uebernehmen`, ohne `Steuerung`); `crates/krk-core/tests/operation.rs:1559` (`ein_zeitpunkt_vor_1980_faellt_auf_das_vorgabedatum_und_bleibt_aus_der_abschlussliste`); Commit `acc9671`

---

## Was ist

Der Plan ist aktiv (`_p_`) und sagt in Schritt 3 an zwei Stellen das Gegenteil dessen, was
der Baum seit `acc9671` tut:

- `:243` — „Der Rückfall ist `DateTime::DEFAULT` **mit einer Zeile in der Abschlussliste**,
  denselben Weg, den das Packen für eine Datei nimmt, deren Typ es nicht annimmt".
- `:251`, Abnahmekriterium — „Ein Zeitpunkt vor 1980 fällt auf `DateTime::DEFAULT` zurück und
  erzeugt genau eine Zeile in der Abschlussliste; eine Probe hält beides."

Seit `acc9671` erzeugt derselbe Fall **keine** Zeile, und die Probe, die das Kriterium hielt,
heißt jetzt `…_und_bleibt_aus_der_abschlussliste` und prüft `uebersprungen.is_empty()`. Der
Datensatz `260825-2127_*_ein-gepackter-eintrag-…` hat in seinem `Resolved:` Weg 1 gewählt
und den Plan dabei nicht nachgezogen; er hatte den Widerspruch selbst benannt („Der Plan hat
die Weiche gestellt und die Vokabel nicht geprüft").

## Warum das zählt

Der Plan trägt für Schritt 3 schon einen Nachtrag (`260825-1859`), der ein halb erfülltes
Kriterium ausschreibt. Ein Kriterium, das erfüllt war und durch eine spätere Behebung
absichtlich **unerfüllt** wurde, steht ohne Nachtrag da. Wer den Plan gegen den Baum abgleicht,
findet ein gebrochenes Abnahmekriterium und keinen Hinweis, dass der Bruch gewollt ist.

## Was zu tun wäre

Ein zweiter Nachtrag an Schritt 3, in der Form des ersten: das Kriterium `:251` ist mit
`acc9671` bewusst umgekehrt, der Grund steht am Doc-Kommentar von `zeit_uebernehmen` und im
Datensatz `260825-2127_*_ein-gepackter-eintrag-…`. Der Satz `:243` bleibt als Entwurfsstand
stehen und bekommt denselben Verweis. Kein Code ist zu ändern.

**Schwere:** gering. Prosa in einem aktiven Plan, die dem Baum widerspricht; kein
Verhalten hängt daran.

**Gefunden:** coderev, bei der Nachdurchsicht der Behebungsrunde gegen `ecd7e4b..1ac5dde`.
