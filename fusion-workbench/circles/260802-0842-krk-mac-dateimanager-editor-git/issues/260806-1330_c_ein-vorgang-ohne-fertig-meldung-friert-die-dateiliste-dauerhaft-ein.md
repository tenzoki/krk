Ein Vorgang ohne Fertig-Meldung friert die Dateiliste dauerhaft ein

---

Seit `fd5e3c5` überspringt die Dateisystemwache die Ordner des laufenden
Vorgangs (`crates/krk-ui/src/appkit/anwendung.rs:1228-1232`). Der Aufschub endet
allein in `vorgang_beenden`, und das ist die einzige Stelle, die
`ivars.vorgang` leert (`anwendung.rs:2267`). Bleibt der Vorgang stehen, bleibt
der Ordner für die ganze Laufzeit von KRK von jeder Auffrischung
ausgeschlossen — auch von fremden Änderungen, die C9 zusagt.

---

**Der Weg dorthin.** `vermitteln` (`anwendung.rs:2634-2665`) schleift, bis
`Meldung::Fertig` kommt oder der Kanal schließt:

```rust
while let Ok(meldung) = lauf.meldungen().recv() {
    let fertig = matches!(meldung, Meldung::Fertig(_));
    ...
    if fertig { break; }
}
lauf.warten();
```

Schließt der Kanal **ohne** `Fertig`, wird `stand.bericht` nie gesetzt, der
Hauptfaden erreicht `vorgang_beenden` nie, und `ivars.vorgang` bleibt für immer
`Some`. Der Kanal schließt ohne `Fertig` genau dann, wenn der Arbeitsfaden aus
`krk_core::operation::starten` (`crates/krk-core/src/operation/mod.rs:120-127`)
vor seiner letzten Zeile abbricht, also bei einer Panik in `ausfuehren`.

`inference:` Ich habe keinen konkreten Panikpfad in `ausfuehren` gefunden — die
Produktivpfade unter `crates/krk-core/src/operation/` tragen kein `expect` und
kein `unwrap` außerhalb der Prüfungen. Der Fall ist also unwahrscheinlich, aber
nicht ausgeschlossen, und `vermitteln` behandelt ihn heute stillschweigend.

**Warum er jetzt schwerer wiegt als vorher.** Vor dieser Runde kostete ein
hängengebliebener Vorgang die stehengebliebene Fortschrittszeile und die
Abweisung des nächsten Operationsbefehls: ärgerlich, aber sichtbar und auf die
Operationen beschränkt. Jetzt hängt zusätzlich die Richtigkeit der angezeigten
Dateiliste daran. Die Liste zeigt ohne jeden Hinweis einen überholten Stand,
und der Nutzer hat keinen Weg, sie ohne Ordnerwechsel wieder in Gang zu
bringen.

**Der Fix, an einer Stelle.** `vermitteln` setzt beim Verlassen der Schleife
ohne `fertig` einen Abschlussbericht ab, der den Abbruch benennt — dieselbe
Bahn, die der reguläre Abschluss geht. Damit räumt `vorgang_beenden` in jedem
Fall auf: es nimmt die Fortschrittszeile weg, frischt die Ordner auf und leert
`ivars.vorgang`. Ein zweiter Aufräumweg entsteht dabei nicht.

**Betrifft:** `krk-ui` (`appkit/anwendung.rs`), Grenze zu `krk-core`
(`operation/mod.rs`). C4 und C9. Keine Zeitzusage aus C8 berührt.

---
Resolved: Schließt der Meldekanal ohne Fertig, trägt abbruch_ohne_meldung_nachtragen einen Abschluss::Abgebrochen-Bericht auf derselben Bahn nach, und vorgang_beenden räumt auf. Ein Ordner kann damit nicht mehr dauerhaft von der Auffrischung ausgeschlossen bleiben.
