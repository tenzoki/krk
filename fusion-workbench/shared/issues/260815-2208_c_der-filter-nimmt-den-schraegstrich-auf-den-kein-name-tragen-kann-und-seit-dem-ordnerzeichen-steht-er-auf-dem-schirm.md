Der Filter nimmt den Schrägstrich auf, den kein Name tragen kann — und seit dem Ordnerzeichen steht er auf dem Schirm

---

`traegt_ein_dateiname` beantwortet ausweislich seines Doc-Kommentars die Frage „Ob ein
Dateiname dieses Zeichen tragen kann" (`krk-core/src/verzeichnis/filter.rs:53-65`). Für den
Schrägstrich antwortet es „ja", und das ist falsch: `name_pruefen` weist ihn ab
(`krk-core/src/operation/umbenennen.rs:72-74`), und der Kopf des neuen `ORDNERZEICHEN` sagt
es noch einmal — „er kann in keinem Namen vorkommen, den ein Dateisystem hergibt"
(`tabelle.rs:334`). Zwei Stellen des Baums widersprechen sich.

---

**Schwere:** niedrig. Der Widerspruch bestand vor `3b128c3`; sichtbar wird er erst jetzt.
Der Nutzer liest in der Liste `Bilder/`, tippt zum Filtern `bilder/` — und bekommt eine leere
Liste, weil `traegt_die_folge` gegen `eintrag.name` vergleicht. Vor dem Ordnerzeichen stand
in der Namensspalte kein Zeichen, das der Filter annimmt und kein Name tragen kann.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-core/src/verzeichnis/filter.rs:53-65`
**Domain:** code

## Zwei Wege

1. **Die Zeichenregel um den Schrägstrich erweitern.** Eine Zeile in `traegt_ein_dateiname`,
   und der Doc-Kommentar bekommt eine dritte Klasse neben Steuerzeichen und privatem
   Bereich. Der Filtertext bleibt dann bei einem Tastendruck auf `/` unverändert stehen,
   genau wie bei der Eingabetaste — dieselbe Regel und keine neue. Der zweite Rufer der
   Zeichenregel, die Tippsuche der Belegungsansicht aus der Runde 7, verlöre damit ebenfalls
   den Schrägstrich; ob dort ein Kürzel ihn braucht, ist vor dem Eingriff zu prüfen.
2. **Nichts tun und den Doc-Kommentar auf das zurücknehmen, was die Funktion prüft.** Sie
   siebt Steuerzeichen und den Bereich `U+F700`–`U+F8FF` und behauptet keine
   Vollständigkeit; nur ihre erste Zeile behauptet sie.

Weg 1 ist der integrale, wenn die Belegungssuche ihn verträgt. Weg 2 ist ehrlich und billig,
lässt die Bedienfolge aber bestehen.

---

**Resolved:** 260815-2240, Weg 1. Die Vorpruefung ist am Dateibestand gefahren und faellt
zugunsten von Weg 1 aus: `resources/default-keymap.toml` traegt den Schraegstrich in
keiner Zeile ausserhalb der Kommentare, und die Tastentabelle
`krk_core::tasten::parser::TASTEN` fuehrt ihn unter ihren 61 Namen ueberhaupt nicht, also
kann auch keine zugewiesene Kombination ihn je anzeigen. Die Belegungssuche verliert damit
nichts.

`traegt_ein_dateiname` weist den Schraegstrich jetzt ab; der Doc-Kommentar zaehlt drei
Klassen, und der Modulkopf haelt die Bedienfolge fest. Zwei Proben in `filter.rs`: die
Zeile selbst, und die eine Richtung, in der der Widerspruch stand — was die Zeichenregel
aufnimmt, muss `operation::name_pruefen` durchlassen. Der Doc-Kommentar von
`Suchlage::zeichen_anhaengen` (`krk-ui/src/belegungsmodell.rs`) zaehlt die Klassen
mit auf und ist nachgezogen. Eine zweite Zeichenregel ist nicht entstanden; die
Zaehlprobe haelt weiter je eine Regel und je zwei Rufer.

`make check` — exit 0. Verlauf:
`shared/history/260815-2240-coder-der-filter-weist-den-schraegstrich-ab.md`
