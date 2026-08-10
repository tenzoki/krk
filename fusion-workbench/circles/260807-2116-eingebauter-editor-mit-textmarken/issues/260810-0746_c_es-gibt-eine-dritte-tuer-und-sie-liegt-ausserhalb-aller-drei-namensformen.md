# Es gibt eine dritte Tür zu denselben Einstellungen, und sie liegt außerhalb aller drei Namensformen

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:2616` (`FORMEN`), `:2665-2735` (`EINSTELLUNGEN`), Modulkopf `:129-137` („Die zweite Namensform … eine zweite Tuer zu denselben")
**Cross-references:** `issues/260810-0416_c_zwei-weitere-textveraendernde-automatiken-stehen-an-und-die-probe-sieht-sie-nicht.md`, Spec C4

---

## Der Befund

Der Commit trägt als Kernbild: die Form `set…Type:` ist „zum größten Teil keine
zweite Menge von Einstellungen, sondern eine zweite **Tür** zu denselben". Das
Bild stimmt und ist gemessen. Es zählt aber eine Tür zu wenig.

`NSTextView` trägt `setEnabledTextCheckingTypes:` — eine Bitmaske vom Typ
`NSTextCheckingTypes`, die dieselben fünf Automatiken hält wie die fünf
`set…Enabled:`-Zeilen in `textflaeche_bauen`. Der Name endet auf `Types:` und
nicht auf `Type:`; `FORMEN` fängt ihn deshalb nicht, und `EINSTELLUNGEN` führt
ihn nicht.

Gemessen, nicht der Dokumentation entnommen, auf demselben Gerät (macOS 15.7.7,
Build 24G720). Ein ObjC-Programm baut eine `NSTextView`, fährt die fünf Zeilen
aus `textflaeche_bauen` und legt danach die dritte Tür um:

```
frisch:                          enabledTextCheckingTypes=0x23c1  quoteEnabled=1
nach KRKs fuenf Zeilen:          enabledTextCheckingTypes=0x2001  quoteEnabled=0  smartQuotesType=1 (No)
nach setEnabledTextCheckingTypes(NSTextCheckingAllTypes):
                                 quoteEnabled=1 dashEnabled=1 replEnabled=1 corrEnabled=1
                                 smartQuotesType=2 (Yes)
```

Und umgekehrt, zum Beleg, dass es dieselbe Sache ist:

```
Bitmaske vorher 0x23c1, nach setAutomaticQuoteSubstitutionEnabled:NO -> 0x2381
(das Quote-Bit 0x40 ist gefallen)
```

Ein einziger Aufruf von `setEnabledTextCheckingTypes:` macht also **fünf** der
sieben abgeschalteten Automatiken wieder an und setzt `smartQuotesType` von
`No` auf `Yes` zurück.

## Warum das die Aussage des Commits berührt

Zwei Stellen sind betroffen, beide im Modulkopf:

1. Die Überschrift „**Die zweite Namensform** ist zum größten Teil keine zweite
   Menge von Einstellungen, sondern eine zweite Tür zu denselben" zählt zwei
   Türen. Es sind drei, und die dritte ist die einzige, die alle fünf auf
   einmal öffnet.
2. „`NSTextView` traegt sechsundzwanzig Einstellungen der Formen `set…Enabled:`,
   `set…Type:` und `set…Behavior:`" ist als Satz richtig und als Aufzählung
   der Türen zu C4 unvollständig.

Das ist derselbe Fehlermodus, den `260809-1650` und `260810-0416` schon zweimal
hatten: die Aufzählung hörte an einer Namensform auf, und die nächste
Einstellung derselben Wirkung trug eine andere. Er ist damit zum dritten Mal
belegt — das ist kein Einzelfund mehr, sondern die Aussage des Modulkopfs
„Die Namensform ist nicht der Schnitt, den die Sache verlangt" in konkreter
Form.

## Was heute hält

Kein gemessener Textverlust. `setEnabledTextCheckingTypes:` wird in KRK nirgends
gerufen — geprüft: `grep -rn "enabledTextCheckingTypes" crates/` findet nichts.
Der Befund ist eine offene Flanke gegen C4 für den nächsten, der die Bitmaske
anfasst, und eine Ungenauigkeit im Bild, das der Modulkopf zeichnet.

## Vorschlag

`setEnabledTextCheckingTypes:` in `EINSTELLUNGEN` aufnehmen. Die Einordnung ist
keine der vier bestehenden sauber: sie ist keine `ZweiteTuerZu` **einer**
Einstellung, sondern eine Sammeltür zu fünfen. Entweder trägt `ZweiteTuerZu`
künftig mehrere Ziele, oder es kommt eine fünfte Antwort dazu. Der Modulkopf
zählt dann drei Türen statt zwei.

---
Resolved: `setEnabledTextCheckingTypes:` ist eingeordnet, die Namensform `Types:`
fängt den Selektor, und der Modulkopf zählt drei Türsorten statt zwei.

**Von den beiden angebotenen Wegen ist der zweite genommen**: es kommt eine
fünfte Antwort dazu, `Einordnung::SammeltuerZu(&[…])`, statt `ZweiteTuerZu`
mehrere Ziele tragen zu lassen. Der Grund ist nicht Geschmack, sondern die
Messung: eine Tür auf **eine** Einstellung wird paarweise in beiden Richtungen
nachgemessen, eine Tür auf **mehrere** über den Vergleich zweier Masken. Zwei
Messverfahren in einer Variante wären eine Fallunterscheidung nach der Länge
eines Feldes.

Gesetzt wird die Maske nicht, aus dem Grund, den `260810-0512` nennt. Die neue
Probe `die_sammeltuer_ist_eine_sicht_auf_dieselben_bits` hält beides im Baum
fest: die Maske an KRKs Fläche hat gegenüber einer frischen Fläche nur Bits
verloren und keines dazugewonnen, und ihr Werkswert ändert jede Einstellung, die
die Aufstellung als Ziel nennt. Zahlenwerte von Apple stehen dabei nirgends;
gemessen wird der Unterschied zwischen zwei Flächen.

**Der Datensatz irrt in einer Zahl, und die Änderung folgt der Messung.** Er
sagt, ein einziger Aufruf mache „**fünf** der sieben abgeschalteten Automatiken
wieder an". Nachgemessen sind es **vier**: Anführungszeichen, Bindestriche,
Textersetzung, Rechtschreibkorrektur. Die fünfte Zeile seiner eigenen
Messausgabe, `smartQuotesType` von `No` auf `Yes`, ist dieselbe Automatik durch
ihre zweite Tür und keine fünfte Automatik. Dafür ist eine Wirkung dazugekommen,
die der Datensatz nicht nennt: der Werkswert der Maske schaltet
`grammarCheckingEnabled` **aus**. `SammeltuerZu` führt deshalb fünf Ziele — die
vier tippenden Automatiken und die Grammatikprüfung —, und der Modulkopf nennt
beide Richtungen. Der Kern des Befundes bleibt unberührt: die Aufzählung hörte
an einer Namensform auf, und die Sammeltür lag außerhalb.
