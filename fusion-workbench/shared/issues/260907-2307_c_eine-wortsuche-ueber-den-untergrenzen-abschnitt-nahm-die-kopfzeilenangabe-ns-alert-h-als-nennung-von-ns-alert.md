# Eine Wortsuche über den Untergrenzen-Abschnitt nahm die Kopfzeilenangabe `NSAlert.h` als Nennung von `NSAlert`

---
Die erste Fassung von `steht_als_wort` (`crates/krk-core/tests/baum.rs`) fragte allein nach
Wortgrenzen. Damit deckte die Belegangabe `NSAlert.h:22`, die im Abschnitt für die
Aufzählung `NSAlertStyle` steht, die Nennung der Klasse `NSAlert` mit ab — und zwei
Klassen, die keine Datei nennt, standen grün.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`,
`260907-2256_*_die-untergrenzen-abschnitte-nannten-196-hereingeholte-namen-nicht-und-27-von-42-appkit-dateien-waren-betroffen.md`

## Wie es aufgefallen ist

An der Gegenprobe. `NSAlert` wurde im Abschnitt von `appkit/hinweis.rs` durch einen
erfundenen Namen ersetzt, und die Probe blieb grün: `NSAlert.h:22` stand daneben und
erfüllte die Wortsuche. Ohne diese Gegenprobe wäre die Probe mit einer Lücke ausgeliefert
worden, über der sie grün steht — genau der Fall, den der Auftrag zu K16 als schlimmer als
keine Probe bezeichnet.

## Was die verschärfte Regel dann fand

Ein Name unmittelbar vor `.h` zählt seither nicht als Nennung. Mit dieser Einschränkung
wurden zwei weitere echte Lücken sichtbar, die die lose Regel verdeckt hatte:

- `appkit/betrachter.rs` holt `NSNotification` herein, der Abschnitt belegte nur
  `NSNotification.h:37` für `NSNotificationCenter`.
- `appkit/editor.rs` holt `NSView` herein, der Abschnitt belegte nur `NSView.h:33` für
  `NSAutoresizingMaskOptions`.

## Abnahmetest

Ein Name aus einem Abschnitt entfernt, während seine Kopfzeile darin stehen bleibt, macht
`jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt` rot. Am 260907 an
`appkit/hinweis.rs` gefahren.

---
Resolved: `steht_als_wort` lässt einen Treffer unmittelbar vor `.h` nicht mehr gelten; der
Grund und die Gegenprobe stehen an ihrem Doc-Kommentar. Die zwei dadurch sichtbar
gewordenen Lücken sind in `appkit/betrachter.rs` und `appkit/editor.rs` geschlossen, beide
Angaben am SDK gelesen (`NSNotification.h:15` und `NSView.h:81`, beide ohne eigene
Verfügbarkeitsangabe).
