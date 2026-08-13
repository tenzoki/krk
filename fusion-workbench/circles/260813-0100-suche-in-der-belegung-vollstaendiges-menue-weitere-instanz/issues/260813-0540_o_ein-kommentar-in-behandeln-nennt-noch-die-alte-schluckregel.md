Ein Kommentar in `behandeln` nennt noch die alte Schluckregel

---

S3 hat die Grenze gedreht: geschluckt wird, was **zulässig** war, und nicht mehr, was
**gewirkt** hat. Der Modulkopf von `crates/krk-ui/src/appkit/ereignisse.rs:154-166` schreibt
die neue Regel samt Begründung aus, und der Doc-Kommentar von `kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:2555-2566`) ebenfalls.

Der Kommentar am `match` in `behandeln` ist stehengeblieben und sagt das Gegenteil:

> `crates/krk-ui/src/appkit/ereignisse.rs:528-530` — „Belegt und gebaut. Eine Funktion ohne
> Kommando ist belegt, aber in dieser Runde noch nicht gebaut; siehe den Modulkopf:
> **geschluckt wird nur, was auch ausgefuehrt wurde**."

Der Verweis auf den Modulkopf zeigt damit auf eine Stelle, die das Gegenteil sagt. Der erste
Halbsatz („eine Funktion ohne Kommando … noch nicht gebaut") gilt weiter und gehört stehen zu
bleiben; falsch ist allein die Begründung dahinter.

---

**Schwere:** gering. Kein Verhalten betroffen; ein Kommentar, der einen Leser in die Irre
führt, und zwar an genau der Stelle, an der die Runde die Regel gewechselt hat.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/ereignisse.rs:528-530`

**Domain:** code
