Die Spannenstrecke hat keinen Vordergrundvorbehalt und meldet im Hintergrund die Geduld statt der Ursache
---
`NICHT_IM_VORDERGRUND` wird allein auf der Sitzungsstrecke entschieden (`messung_unmoeglich`, aus `Sitzungslage::im_vordergrund`). Die Strecke `--messmodus spannen` (L1, L2, L3, L10) trägt in ihrem `Zustand` keine Vordergrundangabe. Im Hintergrund misst sie L2, L3 und L10 vollständig, setzt dann den synthetischen Pfeil ab, den `zulaessig` ohne Schlüsselfenster abweist, und bricht nach zehn Sekunden mit „ein Tastendruck ist nach 10 s nicht am Ziel; seit dem Beginn sind N Bildgrenzen eingegangen“ ab. Die vierzig gemessenen Werte gehen mit dem Abbruch verloren, und die Meldung nennt die falsche Ursache.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/messmodus.rs:376-386`: `Zustand` trägt `sitzung: Option<Sitzungslage>`, und `:384` sagt „`None` auf den Strecken aus S8“. `im_vordergrund` steht nur in `Sitzungslage` (`:400`).
- `crates/krk-ui/src/messmodus.rs:739-745`: der Vorbehalt steht in `messung_unmoeglich`, das nur `sitzung_weiter` (`:1254`) ruft.
- `crates/krk-ui/src/messmodus.rs:1056-1068`: `Schritt::Taste` prüft allein `zustand.zeilen == 0`.
- `crates/krk-ui/src/messmodus.rs:1079-1099`: `haengt` liefert die Geduldsmeldung; bei `bilder == 0` behauptet sie „das Fenster ist vermutlich verdeckt“, bei `bilder > 0` sagt sie nichts über die Ursache.
- `crates/krk-ui/src/appkit/anwendung.rs:7814`: `Anweisung::Taste => ereignisse::pfeil_ab_senden(...)`, und `ereignisse.rs:460-466` sagt, das Ereignis nimmt denselben Weg wie ein körperlicher Druck, also über `kommando_ausfuehren` und `zulaessigkeit::zulaessig`, das ohne Schlüsselfenster ablehnt (`CLAUDE.md`, „Der Abnahmelauf verlangt KRK im Vordergrund“).
- `crates/krk-ui/src/appkit/anwendung.rs:7838-7841`: `Anweisung::Abbruch` gibt nichts aus und beendet mit 4; `ausgeben` läuft nur bei `Fertig`.
- `crates/krk-bench/src/messen.rs:865-866` fährt die Spannenstrecke weiterhin.

`CLAUDE.md` sagt unter „Was man nicht sieht“: „die Messstrecke meldet `NICHT_IM_VORDERGRUND` statt Zahlen“. Das gilt für die Sitzungsstrecke; für die Spannenstrecke gilt es nicht.

## Vorschlag

`im_vordergrund` in `Zustand` heben (die Oberfläche kennt den Wert auf beiden Strecken) und in `naechster_schritt` vor `Schritt::Taste` dieselbe Frage stellen wie `messung_unmoeglich`; dann bricht die Spannenstrecke am ersten Tastenschritt mit derselben Meldung ab statt nach zehn Sekunden mit der falschen.
