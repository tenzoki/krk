Die Anfrage der Einfaerbung schreibt je Anschlag den ganzen Stand ein zweites Mal ab

---

`text_zurueckschreiben` laeuft bei jedem `textDidChange:` und ruft am Ende `einfaerbung_anfordern`; das
klont in der Formatansicht den ganzen gehaltenen Stand (`modell.stand().to_owned()`) fuer den
Arbeitsfaden — neben dem Umschreiben aus UTF-16, das der geschlossene Datensatz `260809-2322` mit 96 %
je Anschlag misst. Die zweite Abschrift ist nirgends gezaehlt; der Doc-Kommentar sagt nur, sie koste
nichts, "solange schon eine laeuft".

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/editor.rs:1955-1988` (`text_zurueckschreiben`): `:1956` das Umschreiben,
  `:1987` der Ruf `self.einfaerbung_anfordern()`.
- `:1982-1986`: "Die Anfrage kostet nichts, solange schon eine laeuft".
- `:2907-2918` (`einfaerbung_anfordern`): `modell.stand().to_owned()` — eine `memcpy` der ganzen Datei,
  bis 16 MB — sobald keine Einfaerbung laeuft; danach `Einfaerbungsvorgang::starten(vorlage, stand, …)`
  (`hervorhebung.rs:1456-1462`) nimmt den `String` als Wert.
- `circles/260807-2116-…/issues/260809-2322_c_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`
  misst allein `string().to_string()` und `bearbeiten`.

Das Fortschreiben (`hervorhebung::fortschreiben`) ist inkrementell, die Uebergabe des Textes an den
Faden nicht — die Zusage aus CLAUDE.md ("rechnet den vorigen Durchgang fort") gilt der Rechnung und
haelt; die Abschrift steht daneben und wird nicht genannt.

## Was zu tun waere

Die Zahl in die Messreihe von `:1920-1925` aufnehmen oder im Kopf von `einfaerbung_anfordern` als
angenommenen Preis benennen. Billiger wuerde es nur mit einem `Arc<str>` fuer den Stand, und das ist eine
Aenderung am `Editormodell`, die derselbe Datensatz `2322` als zu teuer fuehrt.

## Umfang

`krk-ui`, `appkit/editor.rs`; Messung.
