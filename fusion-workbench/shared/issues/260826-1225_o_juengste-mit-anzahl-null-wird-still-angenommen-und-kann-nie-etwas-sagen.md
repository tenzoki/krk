`juengste` mit `anzahl = 0` wird still angenommen und kann nie etwas sagen

---

`gekappte_anzahl` (`crates/krk-core/src/leseprofil/datei.rs:524-528`) deckelt nach oben auf
`HOECHSTENS_JUENGSTE` und kennt keine untere Schranke. Eine `readers.toml` mit
`juengste = { anzahl = 0 }` kommt damit durch den Pruefschritt, ohne Meldung. In
`Lauf::juengste` (`bausteine.rs:645-648`) folgt `kandidaten.truncate(0)`, die Liste ist leer,
und die Zeile traegt in **jeder** Zusammenfassung `Wert::Nicht`, also den Platzhalter `--`.
Der Nutzer sieht eine Zeile, die ihm nie etwas beantwortet, und erfaehrt nicht, warum.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/leseprofil/datei.rs:524-528` (`gekappte_anzahl`),
`crates/krk-core/src/leseprofil/bausteine.rs:645-648`
**Tree state:** `004ff72`
**Domain:** code

## Warum das aus der Regel des Moduls herausfaellt

Der Modulkopf von `datei.rs:44-86` nennt drei Reichweiten der Abweisung und begruendet die
mittlere so: „**Das ganze Profil faellt weg**, wenn eines seiner beiden Erkennungsmuster sich
nicht uebersetzen laesst (C2.7) oder wenn es keines von beiden nennt. Ein Profil, das seinen
Ort nicht erkennt, ist nicht halb brauchbar, sondern gar nicht." `pruefen` setzt das um
(`datei.rs:363-370`) und schreibt die Meldung „es nennt weder ein Pfadmuster noch eine
Kennzeichendatei und koennte damit nie treffen".

`anzahl = 0` ist derselbe Satz eine Ebene tiefer: eine Zeile, die nie etwas sagen kann. Sie
faellt in keine der drei Reichweiten, und der Nutzer bekommt keinen der Saetze, die das Modul
fuer genau diesen Fall schon formuliert hat.

## Warum die Kappung nach oben nicht die Antwort nach unten vorgibt

Die Kappung nach oben ist ausdruecklich begruendet (`mod.rs:143-148`, C6.3): eine zu grosse
Zahl „ist keine falsche Angabe, sondern eine, die mehr verlangt, als die Zusammenfassung
hergibt". Fuer die Null gilt das Gegenteil — sie verlangt nichts und bekommt nichts, und
zwischen „ich will null Eintraege sehen" und „ich habe mich vertippt" unterscheidet die Datei
nicht. Dieselbe Erwaegung hat bei `zeigt` zur strengen Antwort gefuehrt (`datei.rs:262-270`):
„`zeigt = \"titelchen\"` ist keine Angabe, sondern ein Vertipper, und den still auf `titel` zu
bringen hiesse, dem Nutzer etwas anderes zu zeigen, als er geschrieben hat."

## Zu entscheiden ist die Reichweite und nicht das Ob

Die dritte Reichweite („die Zeile behaelt ihre Beschriftung und verliert ihren Baustein") passt
und braucht eine Zeile in `baustein_pruefen` oder in `gekappte_anzahl`; sie liefert dieselbe
Anzeige wie heute, aber mit der Meldung dazu. Die weiteste Reichweite waere hier zu weit: eine
Null nimmt keiner anderen Zeile etwas weg.

**Gefunden:** coderev, Vollbaum-Durchsicht von `crates/krk-core/src/{ablage,leseprofil}/` am
260826-1225.
