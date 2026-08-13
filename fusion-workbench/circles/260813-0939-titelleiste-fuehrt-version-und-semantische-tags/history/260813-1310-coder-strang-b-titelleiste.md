# Coder: Strang B der Runde 8 — Namen und Version links in der Titelleiste

**Datum:** 260813-1310
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte B1, B2 und B3 aus
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
nicht mehr und nicht weniger. `kommandos/`, `menuemodell.rs`, `menue.rs`,
`xtask/` und `README.md` sind ausdrücklich unberührt geblieben; Strang C fasst
`titelzusatz.rs` in seinem Schritt C3 noch einmal an.
**Abnahme:** `make check` Exit 0 (build, test, clippy unter `-D warnings`, fmt).
Proben in `krk-ui` vorher 538, nachher 542.
**Nicht gefahren:** kein `make bundle`, kein `cargo xtask bundle`. Unter
`target/KRK.app` liegt ein beglaubigtes Bündel.

## Was gebaut wurde

**B1 — das neue Modul `crates/krk-ui/src/appkit/titelzusatz.rs`.** Zwei
Funktionen nach aussen und sonst nichts:

- `beschriftung() -> String` setzt `KRK`, ein Leerzeichen und
  `env!("CARGO_PKG_VERSION")` über `concat!` zusammen. Eine reine Funktion ohne
  AppKit, und die einzige Stelle im Baum, die Name und Version zusammensetzt.
- `bauen(mtm) -> Retained<NSTitlebarAccessoryViewController>` baut drei
  Ansichten von innen nach aussen: die Beschriftung über `labelWithString:` mit
  kleiner Systemschrift und `secondaryLabelColor` (dieselbe Bauform wie
  `appkit/statuszeile.rs`), eine blanke `NSView` als Träger mit `RAND = 8.0`
  Punkten links und rechts, und die Steuerung mit
  `layoutAttribute = NSLayoutAttribute::Left`.

`Left` und nicht `Leading`, wie der Plan es verlangt. Der Modulkopf trägt den
Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`; höchste
Untergrenze der Datei ist 10.12 (`labelWithString:`), alles übrige 10.10 oder
10.0, alle Zahlen am SDK unter `/Applications/Xcode.app/.../MacOSX.sdk`
nachgelesen und mit Datei und Zeile zitiert.

Angemeldet in `appkit/mod.rs`: die Modulliste steht danach bei 28 Namen, der
neue Name steht in der Übersichtsgrafik unter `fenster` und trägt einen eigenen
Absatz in den Modulbeschreibungen. **Die Prosazahl im Modulkopf ist berichtigt**
— dort stand „Sechsundzwanzig Module" bei tatsächlich 27; sie lautet jetzt
„Achtundzwanzig".

Weder `Cargo.toml` noch `Cargo.lock` sind angefasst: die drei nötigen Merkmale
(`NSTitlebarAccessoryViewController`, `NSViewController`, `NSLayoutConstraint`)
stehen im Vorgabesatz von `objc2-app-kit 0.3.2`. Am Manifest der Kiste
nachgelesen, nicht aus dem Plan übernommen.

**B2 — eingehängt und der Anfangstitel.** `fenster::hauptfenster` hängt den
Bereich unmittelbar nach `setContentMinSize` über
`addTitlebarAccessoryViewController` ein, an der einen Stelle, die das Fenster
aufbaut. `fenster.rs` setzt den Titel dort nicht mehr auf `KRK`, sondern auf die
**leere Zeichenkette**; ein Kommentar daneben sagt, warum die Zeile bleibt statt
zu fehlen — ein Fenster ohne `setTitle:` trägt den Vorgabetitel von AppKit.

Der Modulkopf von `fenster.rs` bekommt zwei Zusätze: einen Absatz über der
Layout-Skizze, der sagt, dass die Titelleiste darin nicht vorkommt und wo ihr
Bereich gebaut wird, und im Verfügbarkeitsabschnitt die eine Berührung, die über
10.0 liegt (`addTitlebarAccessoryViewController:`, seit 10.10).

Zwei Prosastellen ziehen nach:

- Der Kommentar in `Anwendungsdelegierter::oberflaeche_aufbauen` sagte,
  `appkit::fenster` setze den Titel „einmal auf den Namen der Anwendung". Er
  sagt jetzt: auf die leere Zeichenkette, und warum.
- Der Modulkopf von `fenstertitel.rs` hält fest, dass C11 der Runde 2 seit
  dieser Runde fortgeschrieben ist und die elf Kriterien im Spec dieser Runde
  unter `### C2` stehen. `titel` selbst ist Zeile für Zeile unverändert, und der
  Satz sagt das mit.

`fokusanzeige_nachziehen` ist nicht angefasst; es schreibt weiter genau die fünf
Rahmenfarben und den Fenstertitel (C1.7).

## Vier neue Proben, und was sie halten

| Probe | Zusage |
|---|---|
| `die_beschriftung_ist_name_leerzeichen_version` | C1.1 |
| `die_versionszahl_steht_in_keiner_quelldatei` | C1.2 |
| `der_text_wird_genau_einmal_geschrieben` | C1.3 |
| `die_beschriftung_nimmt_den_ersthelferrang_nicht_an` | C1.6 |

Die drei Zählproben lesen über `crate::quellbaum::quelldateien()`; jede Nadel
steht aus `concat!` zusammengesetzt da, sonst fände sie sich in dieser Datei
selbst. Jede benennt ihre Blindheit im Doc-Kommentar.

**Die Nadel der Versionsprobe wird zur Prüfzeit aus `env!("CARGO_PKG_VERSION")`
genommen und nirgends hingeschrieben.** Damit findet die Probe sich nicht selbst
und zieht mit jeder Versionsänderung von allein mit. Sie hat den Modulkopf
zweimal rot gemacht, bevor sie grün war: die Skizze der Titelleiste und ein
Beispiel im Fliesstext trugen die Zahl ausgeschrieben. Beide Stellen stehen
jetzt ohne sie, und der Modulkopf sagt, warum.

**Was `der_text_wird_genau_einmal_geschrieben` nicht kann:** eine private
Funktion mit einem zweiten Schreibruf finge die erste Zählung, eine ohne
Schreibruf ginge beiden Zählungen durch. Was den Fall trägt, ist nicht die
Zählung allein, sondern dass das Feld die Datei nicht verlässt — aus `bauen`
kommt die Steuerung heraus und nicht die Beschriftung.

**C1.7 und C1.8 haben keine neue Probe bekommen, und das ist die Anweisung des
Plans.** Die Feldbreiten `[Bereich; 5]` und `[Fokus; 5]` halten den Bau ohnehin
an, und `MINDESTGROESSE` trägt seine `const _: () = assert!(…)` schon. Eine
Probe daneben wäre eine zweite Fassung derselben Zusage.

## Drei Stellen, an denen ich vom Planwortlaut abgewichen bin

**Der Modulkopf gibt den Kopf des Systems genauer wieder, als der Plan ihn
zitiert.** Der Plan schreibt, `NSTitlebarAccessoryViewController.h:23` lasse
„allein `Bottom`, `Right` und `Left`" zu. Am SDK nachgelesen steht dort mehr:
`Leading` und `Trailing` sind für Anwendungen ab 10.12 ebenfalls zulässig,
`Top` ab 10.13 zusammen mit `NSWindowStyleMaskFullSizeContentView`. Der
Modulkopf sagt das so, weil ein Satz im Baum, den das SDK widerlegt, beim
nächsten Nachlesen mehr kostet als er spart. **An der Wahl ändert das nichts:
gesetzt ist `Left`**, und die Begründung steht daneben — `Leading` wechselte die
Seite mit der Schreibrichtung, und eine Lokalisierung, die davon Gebrauch
machte, führt KRK nicht.

**Die Beschriftung entsteht leer und bekommt ihren Text in einer zweiten
Zeile.** Der Plan nennt die Bauform von `statuszeile.rs:498-503` und verlangt in
B3 eine Probe über „genau ein `setStringValue`". Beides zusammen geht nur so;
der Text liesse sich auch gleich an `labelWithString:` übergeben, dann gäbe es
gar keinen Schreibruf. Gewählt ist die Form, die der Plan prüfbar macht.

**Die Beschriftung hält oben und unten einen mitwachsenden Abstand.** Der Plan
sagt nichts über die Senkrechte. Ohne die beiden Ränder sässe der Text am
unteren Rand des Trägers, sobald AppKit ihn auf die Höhe der Titelleiste zieht.
Ob die Mitte dabei wirklich getroffen wird, ist in diesem Baum durch nichts
gemessen und gehört zum Bild in E2.

## Was offen bleibt

**Jedes Bild ist Nutzerarbeit.** Die Lage des Bereichs, sein Aussehen in hell
und dunkel, das Verhalten bei schmalem Fenster, der Klick darauf und der
Rückweg über „Fenster einblenden" — C1.1 (Lage), C1.6 (Klick), C1.9, C1.10 und
C1.11 stehen im Spec mit **(Bündel)** und sind hier nicht abgenommen. Sie
gehören in die Liste, die E2 dem Nutzer vorlegt.

**Ob der Bereich bei eingeschalteter vollständiger Tastaturbedienung den
Ersthelferrang wirklich abweist, ist nicht gemessen.** Dieselbe offene Frage
trägt die Statuszeile seit der Runde 5, und der Modulkopf verweist darauf.
`labelWithString:` baut nach dem Kopf des Systems ein nicht bearbeitbares und
nicht auswählbares Feld; das ist eine Zusage des Systems und keine Messung an
diesem Baum.

**Für C6.4 gibt es weiter keine Deckungsprobe.** Der Spec markiert das Kriterium
mit **(Probe** über die Deckung**)**, und die neue Datei trägt ihren Abschnitt;
ob und wie die Untergrenzen-Angabe überhaupt maschinell geprüft wird, ist die
offene Nutzerfrage
`shared/decisions/260811-2050_o_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`
mit drei Stufen und ihren Kosten. Eine Probe hier hätte sie vorweggenommen.

## Abnahme

```
make check   → Exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets -- -D warnings` und `cargo fmt --all --check` laufen grün.
`clippy` ist die eigentliche Prüfung, weil `unused_must_use` erst unter
`-D warnings` ein Fehler ist.

Berührte Dateien, und nur diese:

- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/appkit/titelzusatz.rs` (neu)
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/appkit/mod.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/appkit/fenster.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/appkit/anwendung.rs` (der eine Kommentar)
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/fenstertitel.rs` (der eine Modulkopf-Satz)

Dazu im Plan die Schritte B1, B2 und B3 auf `[DONE]`.
