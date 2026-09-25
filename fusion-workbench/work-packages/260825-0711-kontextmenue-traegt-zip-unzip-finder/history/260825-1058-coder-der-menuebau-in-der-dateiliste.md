# Coder: Der Menübau in der Dateiliste

**Datum:** 2026-08-25 10:58
**Status:** Complete
**Agent:** coder
**Baumstand:** `dd80e81` plus die Änderungen dieses Schritts

## Auftrag

Schritt 6 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`:
`menuNeedsUpdate:` in `crates/krk-ui/src/appkit/tabelle.rs` baut nach dem Leeren zuerst die drei
eigenen Einträge und ruft **danach** `teilen::eintrag_anfuegen`. Jeder Eintrag bekommt Titel und
Marke aus `kommandos::kontextmenue::Kontextbefehl`, als Ziel die Quelle selbst und als Handlung
**einen** Selektor `kontextbefehl:`. Dazu die Methode `kontextbefehl:` an `DateifensterQuelle`,
die `sender.tag()` liest und über `Kontextbefehl::von_menuemarke` zurückrechnet, und ein
getippter Rückrufhalter nach dem Vorbild von `Abwurfmelder` und `Vorgangsfrage`.

Der Auftrag nannte `tabelle.rs` als einzige Datei und sperrte `terminal.rs`,
`standardprogramm.rs`, `kommandos/operationen.rs` und `appkit/anwendung.rs` für den zweiten
Coder, der gleichzeitig Schritt 5 fuhr.

## Was entstanden ist

In `crates/krk-ui/src/appkit/tabelle.rs`:

- `pub type Kontextmelder = Box<dyn Fn(Kontextbefehl)>` neben `Abwurfmelder`.
- Das neunte Feld `kontextbefehl: RefCell<Option<Kontextmelder>>` in `QuelleIvars`, wahlfrei wie
  die acht darüber, mit dem Setzer `kontextmelder_setzen`.
- Der Selektor `kontextbefehl:` im `define_class!`-Block von `DateifensterQuelle`. Sein Rumpf
  steht daneben in `kontextbefehl_melden(marke)`, nach der Bauform von `validateDrop:`.
- `eigene_kontexteintraege_anfuegen(menue)`: eine Schleife über `Kontextbefehl::ALLE`, je Wert ein
  Eintrag mit Titel, Marke und Ziel, angehängt über `NSMenu::addItem`.
- In `menuNeedsUpdate:` eine Zeile zwischen `removeAllItems()` und `teilen::eintrag_anfuegen`.
- Eine Probe, `der_kontextmenue_selektor_hat_einen_empfaenger_und_einen_setzer`.

Die Reihenfolge ist am Code von `teilen::eintrag_anfuegen` nachgeprüft und nicht geglaubt: jener
Bauer setzt seinen Trenner bei `numberOfItems() > 0` an Stelle 0 und danach den Freigabeeintrag
an Stelle 0. Drei angehängte Einträge ergeben damit „Teilen, Trenner, Zip, Unzip, Im Finder
öffnen". Ohne betroffene Einträge kehrt jener Bauer vorzeitig zurück, und dann steht das Menü
ohne führenden Trenner da; auch das ist richtig.

## Zwei Stellen, an denen der Plan im Baum nicht hielt

**Erstens, und es hat den Schritt aufgehalten: der Plan verlangt einen zweiten Erzeuger für
`NSMenuItem`, und eine stehende Zusage der Runde 7 verbietet genau das.** Der Schritt sagt, der
Modulkopf von `tabelle.rs` habe `NSMenuItem::initWithTitle:action:keyEquivalent:` zu belegen,
also den Erzeuger dort selbst zu rufen. C2.10 der Runde 7 sagt zu, dass **genau eine** Stelle im
ganzen Quellbaum ein `NSMenuItem` anlegt, und die Probe
`appkit::menue::tests::es_gibt_eine_stelle_je_anlage_und_uebersetzung` hält die Zusage über zwei
Nadeln, `NSMenuItem::alloc(` und `initWithTitle_action_keyEquivalent(`. Der erste Bau nach dem
Wortlaut des Plans ließ sie mit zwei statt einer Fundstelle rot werden.

Genommen ist der Weg über die vorhandene Hülle: `appkit::menue::ohne_kuerzel` ist von
dateiprivat auf `pub(super)` gestellt, und der Menübau der Dateiliste ruft sie. Die Probe bleibt
bei einer Fundstelle, und sie sagt in ihrem eigenen Kopf, dass die Hüllen `befehl` und
`ohne_kuerzel` nicht mitzählen, weil sie nichts anlegen — mehrere Aufrufer sind dort ausdrücklich
vorgesehen. **Damit ist `crates/krk-ui/src/appkit/menue.rs` die zweite angefasste Datei**, obwohl
der Auftrag nur eine nannte; sie war nicht gesperrt, und die Änderung ist auf die Sichtbarkeit
und zwei Erläuterungen beschränkt.

Die Folge für den Modulkopf: belegt sind `NSMenuItem` selbst (`NSMenuItem.h:23`), `tag` in beiden
Richtungen (`:96`), `target` (`:93`) und `NSMenu::addItem:` (`NSMenu.h:92`). `setAction:` (`:94`)
und der Erzeuger (`:38`) stehen ausdrücklich **nicht** in der Liste dieser Datei, mit dem Grund
daneben: der Selektor geht durch die Hülle in den Erzeuger, und beide sind im Kopf von `menue.rs`
belegt. Alle sechs Deklarationen sind am 260825 im SDK nachgelesen und tragen kein
`API_AVAILABLE`, stehen also seit 10.0.

**Zweitens: der Setzer `kontextmelder_setzen` hat bis Schritt 7 keinen Aufrufer.** `krk-ui` hat
kein Bibliotheksziel, also meldet der Übersetzer ihn als unbenutzt, und `-D warnings` hält den
Bau an. Er trägt deshalb ein `#[expect(dead_code, reason = …)]`, das Schritt 7 nennt — dieselbe
Form, die Schritt 4 in `kommandos/kontextmenue.rs` und `kommandos/operationen.rs` schon verwendet,
und mit demselben Ablaufdatum: der erste Aufrufer lässt die Erwartung unerfüllt und den Bau
anhalten, bis die Zeilen weg sind. Anders als dort steht hier kein `cfg_attr(not(test), …)`, denn
eine Probe kann diesen Setzer nicht rufen: sie bräuchte den Hauptfaden, den `libtest` nicht
hergibt.

Der Modulkopf von `kommandos/kontextmenue.rs` bleibt unberührt: seine Erwartung am Modul ist
weiter erfüllt, weil `Entpackbefund`, `ist_zipname`, `archivname`, `ordnername_zum_archiv` und
`entpackziel` bis Schritt 7 ohne Aufrufer bleiben.

## Was die neue Probe hält, und warum es eine gibt

Der Plan sagt, Schritt 6 bekomme keine Probe: ein `NSMenu` zu bauen verlangte den Hauptfaden. Das
gilt für die Form des Menüs und ist so geblieben. Eine Lücke bleibt aber ohne Fenster prüfbar,
und sie ist die Falle, die der Plan selbst als das Risiko dieser Runde benennt: `sel!` baut einen
Selektornamen und fragt **nicht**, ob eine Methode ihn beantwortet. Ein Vertipper zwischen
`#[unsafe(method(kontextbefehl:))]` und `sel!(kontextbefehl:)` übersetzt, besteht jede Probe,
steht im Menü und tut nichts.

`der_kontextmenue_selektor_hat_einen_empfaenger_und_einen_setzer` liest den Quellbaum über
`quellbaum::quelldateien`, lässt Kommentarzeilen fallen und zählt beide Hälften einzeln: genau
eine Erklärung, genau ein Setzen. Die Nadeln stehen zusammengesetzt da, wie bei
`es_gibt_genau_einen_menuebauer` in `appkit/teilen.rs`. Gegengeprobt: ein einzelnes vertauschtes
Zeichen im `sel!` lässt sie rot werden, und der Baum ist danach zurückgestellt.

## Zwei Stellen, die Schritt 7 nachzuziehen hat

Beide liegen in `crates/krk-ui/src/appkit/anwendung.rs`, das für diesen Schritt gesperrt war, und
sind **nicht** gebaut:

1. Beim Aufbau der Oberfläche ist je Fensterseite ein `Kontextmelder` über
   `DateifensterQuelle::kontextmelder_setzen` zu setzen, der den Delegierten **schwach** hält und
   die Seite mitführt, wie die sechs Rückrufe daneben. Ohne ihn tut jeder der drei Einträge
   nichts: `kontextbefehl_melden` fällt still durch sein `let Some(melden)`.
2. Mit dem ersten Aufrufer ist das `#[expect(dead_code, …)]` an jenem Setzer zu entfernen; der
   Übersetzer fordert es ein.

## Prosa und Häuser

Der Modulkopf von `tabelle.rs` sagte „**Das Kontextmenue der Liste baut diese Datei nicht**". Das
ist mit diesem Schritt falsch geworden und umgeschrieben: den Freigabeeintrag baut sie weiter
nicht, ihre drei eigenen schon. Drei Absätze stehen jetzt dort: die Zuständigkeit, die
Reihenfolge samt ihrem Grund, und die Sperre über einen Selektor und drei Marken. Der Kommentar
an `setMenu:` sagte „bekommt seinen einen Eintrag" und sagt jetzt „seine vier".

**Eine fremde Prosastelle ist stehen geblieben und gehört nachgezogen.** Der Kopf von
`crates/krk-ui/src/appkit/teilen.rs` sagt unter „Ein Menue, ein Bauer, drei Flaechen": „Die
Dateiliste, der Editor und die Vorschau beantworten allein, welche Eintraege betroffen sind; sie
bauen kein Menue." Für den Freigabeeintrag stimmt der Satz unverändert, und die zwei Zählproben
jener Datei bleiben grün; als Aussage über die Dateiliste stimmt er seit diesem Schritt nicht
mehr. Die Datei stand nicht im Auftrag, und der Satz ist deshalb unberührt geblieben.

## Abnahme

`make check` läuft grün, Exit 0, 785 Proben in `krk-ui` statt 784 vor dem Schritt. Der Lauf
schließt die Arbeit des zweiten Coders an Schritt 5 mit ein, die inzwischen als `dd80e81`
eingecheckt ist.

Dass die vier Einträge in dieser Reihenfolge erscheinen und der Klick ankommt, sieht der Nutzer
am gebauten Bündel; ohne den Rückruf aus Schritt 7 tut noch keiner der drei etwas.
