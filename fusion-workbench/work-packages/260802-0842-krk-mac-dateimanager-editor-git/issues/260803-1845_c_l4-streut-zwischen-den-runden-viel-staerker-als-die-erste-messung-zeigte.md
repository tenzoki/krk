L4 streut zwischen den Runden viel stärker als die erste Messung zeigte

---

Die Nachmessung vom 260803-1641 hat L4 (Prozessstart bis bedienbares Fenster) in einer von fünf Runden bei einem 95. Perzentil von 715,185 ms und einem größten Einzelwert von 916,460 ms gemessen. Die Zusage von 1000 ms hält damit in jeder Runde, aber der Abstand ist in dieser einen Runde auf den Faktor 1,4 geschrumpft. Die erste Messung vom 260803-1554 hatte für L4 über fünf Runden eine Spanne von 294,555 bis 303,540 ms ausgewiesen, also den Faktor 3,3 mit einer Streuung von neun Millisekunden.

Bericht: `messungen/260803-1641-durchstich.txt`, Abschnitte `Das 95. Perzentil Runde fuer Runde` und `Einzelwerte`.

---

**Es ist kein einzelner Ausreißer, sondern eine ganze Runde.** In Runde 2 liegen die letzten dreizehn der zwanzig Prozessstarts zwischen 420,051 ms und 916,460 ms, während die Runden 3, 4 und 5 durchgehend zwischen 263,147 ms und 348,856 ms bleiben. Ein einzelner langsamer Start hätte das 95. Perzentil nicht bewegt, denn es ist der neunzehnte Wert der sortierten Reihe von zwanzig.

**Die Codeänderung dieses Schrittes scheidet als Ursache aus.** Geändert wurde allein die Auswertung in `crates/krk-bench/src/messen.rs`, also die Rechnung, die aus vorliegenden Einzelwerten ein Urteil bildet. Weder `krk-ui` noch `krk-core` noch das Bündel sind angefasst; die Rohmessung ist Zeile für Zeile dieselbe wie am 260803-1554. Nachgeprüft am Diff: die Datei `crates/krk-bench/src/messen.rs` ist die einzige geänderte Quelldatei.

**Die Ursache ist aus der Messung heraus nicht feststellbar.** `inference:` Naheliegend ist Fremdlast auf dem Gerät während der Startphase von Runde 2. Dafür spricht, dass L2 in derselben Runde 2 mit 53,079 ms ebenfalls seinen schlechtesten Wert hatte (sonst 41,176 bis 45,381 ms). Dagegen spricht, dass L3 in Runde 2 mit 120,695 ms seinen **besten** Wert hatte. L4 wird in einer eigenen Phase gemessen, die den zwanzig Prozessstarts vorausgeht, und L2, L3 und L10 danach in einem einzigen Prozess; die beiden Phasen einer Runde liegen also in verschiedenen Zeitfenstern, und ein Lastereignis kann die eine treffen und die andere nicht. Beweisen lässt sich das aus dem Bericht nicht, weil er die Systemlast nicht mit erhebt.

**Warum das trotzdem festgehalten gehört.** Die Zusage hält, das Gate von Schritt 8 ist bestanden, und dieser Defekt stellt das nicht in Frage. Er betrifft S22, die vollständige Messreihe auf dem Referenzgerät. S22 nimmt L4 unter der Prüfsitzung aus zwei Dateifenstern mit je zwei Tabs ab, und C8 sagt dort ausdrücklich den Kaltstart zu, nicht den warmen. Beide Verschärfungen verbrauchen von dem Abstand, der hier in einer Runde von fünf schon auf 285 ms zusammengeschmolzen war. Eine Messreihe, deren Streuung zwischen zwei Läufen um den Faktor dreißig wechselt, trägt kein belastbares Urteil an einer Grenze, die so nah liegt.

Was zu tun ist, vor S22 und nicht jetzt:

- Feststellen, ob die Streuung von außen kommt. Der einfachste Weg ist, die Messreihe zweimal zu fahren, einmal auf einem ruhigen Gerät und einmal unter bekannter Last, und die beiden Berichte zu vergleichen.
- Kommt sie von außen, gehört eine Bedingung in den Messplan von S22: die Messreihe läuft auf einem Gerät ohne konkurrierende Arbeit, und der Bedingungskopf weist das aus. Der Bericht erhebt heute acht Angaben und keine davon beschreibt die Last.
- Kommt sie nicht von außen, ist die Ursache im Startpfad von KRK zu suchen, und dann ist es ein eigener Defekt mit eigener Untersuchung.

Nicht zu tun ist eine Reparatur auf Verdacht. Der Startpfad ist ohne festgestellte Ursache nicht der richtige Ort für einen Eingriff, und der Nutzer hat die Reihenfolge der offenen Defekte festgelegt.

---

**Nachtrag 260804-2318 (`planner`): der Plan trägt die Bedingung jetzt, der Defekt bleibt offen.**

Die Zusage von 1000 ms ist **nicht** angetastet. Sie hält in jeder gefahrenen Runde, und eine Zahl zu senken, die das Werkzeug erreicht, verschenkte eine Eigenschaft. Was den Defekt trägt, ist die unerklärte Streuung, und dagegen steht jetzt eine Vorschrift statt einer Absichtserklärung:

- S22 im Plan bekommt einen eigenen Absatz. Die Messreihe läuft auf einem Gerät ohne konkurrierende Arbeit, und sie wird zweimal gefahren, einmal ruhig und einmal unter bekannter Last, damit der Vergleich die Frage "kommt die Streuung von außen" beantwortet.
- Das Abnahmekriterium von S22 verlangt beide Berichte und den Vergleich der beiden L4-Spannen im Begleittext.
- Die neunte Kopfangabe, die Systemlast vor und nach dem Lauf, steht in der Dateiliste von S21 bei `crates/krk-bench/src/bericht.rs`. Der Bericht erhob sie bisher nicht, und genau deshalb war die Ursache aus ihm nicht feststellbar.
- Eine Reparatur auf Verdacht am Startpfad findet nicht statt; das steht ausdrücklich im Plan.

Offen bleibt der Defekt, weil die zwei Läufe noch nicht gefahren sind. Er schließt mit S22, oder er wird durch einen eigenen Defekt abgelöst, falls der Vergleich zeigt, dass die Streuung nicht von außen kommt.

---
Resolved: S22 hat die Streuung per Vergleich geklärt, sie kommt von außen. Ruhig liegen die fünf Runden-Perzentile von L4 innerhalb von 19,2 ms (378,9 bis 398,1 ms); unter benannter Last (sechs yes-Schleifen, Lastdurchschnitt bis 9,3) steigt das Niveau um rund 50 % und die Spannweite verdreifacht sich, mit einem 761,6-ms-Ausläufer in der Form der Auffälligkeit vom 260803-1641. Die Planvorschrift (ruhiges Gerät, Lastkennzahl im Bedingungskopf) genügt. Beleg: messungen/260805-2207-MacBookPro15-1-abnahme-begleittext.md.
