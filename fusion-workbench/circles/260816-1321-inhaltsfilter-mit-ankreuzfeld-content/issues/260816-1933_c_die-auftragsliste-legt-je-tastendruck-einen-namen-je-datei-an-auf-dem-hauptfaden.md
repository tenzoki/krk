Die Auftragsliste legt je Tastendruck einen Namen je nicht passender Datei an, und sie tut es auf dem Hauptfaden

---

Die Runde hat entschieden, keine elfte Zeitzusage zu setzen, und hat die Kosten dieser Wahl
benannt: „Die Dauer eines Inhaltsdurchlaufs ist nirgends zugesagt." Das trifft den
**Arbeitsfaden**. Eine zweite Kostenstelle ist mitgewachsen und in Spec, Plan und den zwölf
Sitzungsprotokollen nirgends genannt: die Arbeit, die das Tippen selbst auf dem
**Hauptfaden** auslöst.

**Der Weg eines getippten Zeichens.** `DateifensterQuelle::nach_filteraenderung`
(`crates/krk-ui/src/appkit/tabelle.rs:1339-1342`) ruft bei **jedem** Zeichen
`durchlauf_nachziehen`, und das ruft `auftraege(&tab.modell)`
(`crates/krk-ui/src/tabs.rs:900`). `auftraege` (`tabs.rs:1077-1098`) läuft über den ganzen
Bestand:

```rust
.filter(|(index, _)| !modell.name_traegt_den_filter(*index as u32))
.filter_map(|(index, eintrag)| {
    let art = if eintrag.ist_ordner() || eintrag.ist_verknuepfung() {
        tief.then_some(Auftragsart::Unterbaum)
    } else {
        inhalt_wirkt.then_some(Auftragsart::Inhalt)
    }?;
    Some(Auftrag { index: index as u32, name: eintrag.name.clone(), art })
})
```

Je Eintrag steht darin `name_traegt_den_filter`, und das schreibt den Namen einmal klein
(`verzeichnis/filter.rs:123`, `name.to_lowercase()`), also **eine Zeichenkette je Eintrag**.
Je überlebendem Eintrag kommt `eintrag.name.clone()` dazu, **eine weitere Zeichenkette**.

**Was daran neu ist.** Bis zur Runde 10 überlebte diesen Filter nur ein Ordner, und auch
das nur bei eingeschaltetem „Deep": `tief.then_some(...)`. Bei ausgeschaltetem „Deep" lief
gar kein Durchlauf, `auftraege` wurde nicht gerufen und kostete nichts. Seit dieser Runde
überlebt **jede gewöhnliche Datei**, deren Name nicht passt, sobald „Content" wirkt — auch
bei ausgeschaltetem „Deep", weil `durchlauf_nachziehen_an` seine Bedingung auf
`(!tief && !inhalt_wirkt)` erweitert hat (`tabs.rs:897`).

**Gerechnet am Prüfordner der Messstrecke.** Der große Prüfordner trägt 100.000 Einträge.
Bei drei getippten Zeichen und gesetztem „Content" entstehen dort je Tastendruck rund
100.000 kleingeschriebene Namen für `auftraege`, dazu rund 100.000 Namenskopien und ein
`Vec<Auftrag>` derselben Länge — und das zusätzlich zu den 100.000 Umschreibungen, die
`sicht_neu_aufbauen` über denselben `name_traegt_den_filter` ohnehin schon macht
(`verzeichnis/modell.rs:602`). Dazu kommt je Tastendruck ein `thread::Builder::spawn`
(`verzeichnis/durchlauf.rs:248-261`), dessen Faden der nächste Tastendruck sofort wieder
abbricht.

**Warum keine der zehn Zusagen das fängt.** Der Spec schreibt es selbst aus: L1 misst
zwanzig Pfeil-ab-Ereignisse (`crates/krk-ui/src/messmodus.rs:820`), kein getipptes Zeichen;
L2, L3 und L10 rufen den Prüfschritt nie. Der Namensfilter der Runde 10 war deshalb schon
ungemessen, und der Inhaltsfilter erbt die Lage. Der Unterschied ist, dass die geerbte
Lage jetzt eine andere Größenordnung Arbeit trägt.

---

**Was die Behebung abwägen muss, und deshalb steht hier keine Vorschrift.** Drei Wege sind
am Baum sichtbar, und sie kosten Verschiedenes: die Auftragsliste erst nach einer Ruhezeit
zu bilden (eine Frist, und im Filter steht seit der Runde 10 ausdrücklich keine Zeitmessung
— C6.8 und `verzeichnis/filter.rs:34-35` wären berührt); die Namen als Index statt als
Kopie zu übergeben und den Bestand mit dem Faden zu teilen (`Auftrag.name` fiele weg, der
Faden bräuchte einen geteilten Bestand); oder die Lage zu messen und anzunehmen, wie die
Runde es für die Dauer des Durchlaufs schon getan hat. Der dritte Weg braucht den vierten
Prüfordner mit echten Bytes, den der Spec für die spätere Messrunde benennt.

Gefunden bei der Durchsicht der elften Runde, Bereich `9f5ced5..b9ab8ae`.

---
Resolved: 260816-2230, an derselben Wurzel wie `260816-1930` und `260816-1931`. Von den drei im Datensatz benannten Wegen ist keiner gegangen worden; der Umbau, der die anderen beiden Befunde abträgt, nimmt die Kosten mit.

**Die doppelte Namensfrage ist weg.** `Ordnermodell::name_traegt_den_filter` hatte drei Rufer, und alle drei liefen über den ganzen Bestand: der Neuaufbau der Sicht, die Auftragsliste in `krk-ui`, und — je gezeichneter Zelle — `steht_wegen_des_inhalts`. Der Prüfschritt legt sein Ergebnis jetzt je Eintrag als `Zeilengrund` ab, und alle drei lesen diesen Wert. Die Frage hat damit **einen** Rufer, `zeilengrund_von`, und wird einmal je Eintrag und je Frage gestellt statt einmal je Frager.

**Der ungezählte Posten war der größere.** Der Zeilengrund hängt nicht am Befund; ein eintreffender Befund baut die Sicht neu auf, ohne die Namensfrage noch einmal zu stellen. Vorher lief dieser Gang über 100.000 Einträge bei **jedem** Einzugstakt, während eines Durchlaufs also bis zu sechzigmal in der Sekunde, auf dem Hauptfaden. Der Datensatz oben nennt ihn nicht; er ist beim Umbau mit weggefallen.

**Die Namenskopie ist weg.** `Auftrag` trägt seit dieser Änderung nur noch `index` und `art`. Den Namen hält das Ordnermodell ohnehin, und `Ordnermodell::bestand` reicht ihn als `Arc<Vec<Eintrag>>` an `Durchlauf::starten` weiter — ein Zählerschritt statt 100.000 Zeichenketten je Tastendruck. Geschrieben wird der Bestand über `Arc::make_mut`, und tief kopiert wird dabei nie: ein Durchlauf entsteht erst nach dem Lesevorgang, und `ersatz_einloesen` setzt einen frischen `Arc` ein, statt den alten zu leeren.

Damit bleibt je Tastendruck **ein** Gang über den Bestand mit einer Zeichenkette je Eintrag — der, den `sicht_neu_aufbauen` schon vor der Runde 10 machte und den der Datensatz oben ausdrücklich als vorbestehend ausnimmt. Der Fadenstart je Tastendruck bleibt ebenfalls; er ist eine Zuteilung und kein Gang über den Bestand.

Prüfbar gemacht als Zählprobe im Stil dieses Projekts: `crates/krk-core/tests/verzeichnis.rs::die_namensfrage_des_filters_hat_einen_rufer` zählt die Dateien unter `src/`, die `name_traegt_den_filter` nennen, und erwartet genau `krk-core/src/verzeichnis/modell.rs`. Sie fällt, sobald ein zweiter Ort die Frage wieder stellt. Dass der Auftrag keinen Namen mehr trägt, hält der Übersetzer.

**Was ungemessen bleibt:** wie viele Millisekunden das am Referenzgerät ausmacht. Es ist keine elfte Zeitzusage gesetzt, und die Lage ist dieselbe wie beim Namensfilter der Runde 10 — L1 misst zwanzig Pfeil-ab-Ereignisse, kein getipptes Zeichen. Gezählt ist, was gezählt werden kann: drei Gänge über den Bestand je Tastendruck sind einer geworden, und der Gang je Einzugstakt ist keiner mehr.

Abnahme: `make check` — exit 0.
