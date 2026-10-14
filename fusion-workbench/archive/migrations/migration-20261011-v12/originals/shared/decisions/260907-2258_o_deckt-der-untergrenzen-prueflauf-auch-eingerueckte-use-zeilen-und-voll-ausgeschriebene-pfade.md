# Deckt der Untergrenzen-Prüflauf auch eingerückte `use`-Zeilen und voll ausgeschriebene Pfade?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`,
`260907-2256_*_die-untergrenzen-abschnitte-nannten-196-hereingeholte-namen-nicht-und-27-von-42-appkit-dateien-waren-betroffen.md`

---

## Frage

Der Prüflauf `jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt`
(`crates/krk-core/tests/baum.rs`) liest die `use`-Zeilen **auf oberster Ebene** einer
Datei. Zwei Formen holen daneben einen Namen aus einer Frameworkbindung herein, und keine
davon sieht er:

1. Eine eingerückte `use`-Zeile innerhalb eines Moduls. Am 260907 stehen drei:
   `menue.rs:895`, `editor.rs:3357` und `editor.rs:3358`, alle drei im Prüfmodul am
   Dateiende.
2. Ein voll ausgeschriebener Pfad im Rumpf. Am 260907 steht einer:
   `blaetter/mod.rs:298`, `&objc2_foundation::NSNotification` als Argumenttyp.

Von den neun so hereingeholten Namen nannte der Abschnitt seiner Datei sieben schon von
selbst; **zwei nicht**, `NSTextInputTraitType` und `NSWritingToolsBehavior` aus
`editor.rs:3357`, dazu `NSNumber`. Der zweite von ihnen trägt
`API_AVAILABLE(macos(15.0))` und liegt damit genau auf dem Zielsystem — die Sorte Zahl,
für die der Abschnitt da ist. Nachgetragen sind sie von Hand; der Prüflauf hätte sie nicht
eingefordert und fordert auch die nächste nicht ein.

Die Frage ist, ob der Prüflauf diese zwei Formen mitnehmen soll.

## Optionen

1. **So lassen, die Blindheit bleibt benannt.** Beide Formen stehen im Doc-Kommentar von
   `frameworknamen` und in dem der Probe.
   - Pro: kostet nichts; die Fläche ist klein (vier Zeilen im ganzen Teilbaum) und beide
     Formen sind ungewöhnlich genug, dass sie beim Lesen auffallen.
   - Contra: eine Probe, die grün über einer Lücke steht, ist genau das, was der Auftrag
     zu K16 als schlimmer als keine Probe bezeichnet. Die Lücke war heute schon einmal
     wirksam, und der Name, der durchfiel, trug 15.0.
2. **Eingerückte `use`-Zeilen mitnehmen.** Die Nadel verliert die Bedingung „beginnt an
   Spalte 0" und nimmt jede Zeile, deren gestutzter Anfang `use objc2_` lautet.
   - Pro: schließt die drei heutigen Stellen; drei Zeilen Änderung.
   - Contra: fordert damit auch für einen Namen eine Untergrenze ein, den allein ein
     Prüfmodul anspricht und der nie im ausgelieferten Bündel landet. Ob das erwünscht
     ist, ist die eigentliche Frage: der Abschnitt schützt vor dem Absturz auf dem
     Referenzgerät, und `cargo test` läuft auf demselben Gerät.
3. **Zusätzlich den voll ausgeschriebenen Pfad mitnehmen.** Eine zweite Nadel über
   `objc2_<kiste>::<Name>` im Rumpf.
   - Pro: schließt die letzte bekannte Form.
   - Contra: der Rumpf enthält auch Kommentare und Zeichenketten, in denen ein solcher
     Pfad besprochen wird; die Nadel bräuchte dieselbe Kommentarunterscheidung, die
     `im_code` in `baum.rs` schon führt, und würde eine Nennung in einem Doc-Kommentar
     sonst als Import zählen.

## Constraints

Keine Option darf die Zusage des Prüflaufs erweitern: er hält, **dass** ein Name im
Abschnitt steht, nie **ob** die Zahl daneben stimmt. Die Richtigkeit der Zahl bleibt nach
`260811-2050` eine Zusage des Menschen.

## Empfehlung

Möglichkeit 2, Möglichkeit 3 nicht. Die drei eingerückten Zeilen sind billig zu fassen und
haben heute nachweislich einen 15.0-Namen durchgelassen. Der voll ausgeschriebene Pfad
braucht dagegen die Kommentarunterscheidung mit und bringt eine Fehlbefundquelle mit, für
die es im ganzen Teilbaum eine einzige Fundstelle gibt.
