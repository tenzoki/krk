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

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. Der Kommentar am `match` in `behandeln` sagt nicht mehr „geschluckt wird nur, was auch ausgefuehrt wurde". Der erste Halbsatz bleibt stehen, wie der Datensatz es verlangt: eine Funktion ohne Kommando ist belegt und in dieser Runde noch nicht gebaut. Die Begruendung dahinter ist jetzt die von S3 und widerspricht dem Modulkopf nicht mehr — ein Tastendruck darauf faellt an AppKit zurueck, und das ist keine Ausnahme von der Schluckregel, sondern ihre Anwendung: geschluckt wird, was zulaessig war, und ein Nachschlag ohne Kommando kommt bei der Zulaessigkeitsfrage gar nicht erst an.

Mitgenommen ist die Beobachtung ohne eigenen Datensatz aus derselben Durchsicht: `behandeln` rief `getipptes_zeichen(ereignis)` zweimal, einmal fuer den Faenger und einmal im Sprungmarkenzweig. Der Wert steht jetzt einmal in einer Bindung. Das ist ein Fremdaufruf weniger auf dem Tastendruckpfad, an dem L1 haengt.
