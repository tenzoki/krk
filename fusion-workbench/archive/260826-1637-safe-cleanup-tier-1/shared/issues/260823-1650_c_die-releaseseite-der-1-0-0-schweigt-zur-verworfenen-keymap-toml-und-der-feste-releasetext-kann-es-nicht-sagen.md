Die Releaseseite der 1.0.0 schweigt zur verworfenen `keymap.toml`, und der feste Releasetext kann es nicht sagen

---

KRK 1.0.0 ist am 260823-1449 öffentlich ausgeliefert. Die Hauptzahl ist genau deshalb gestiegen, weil
eine bestehende `~/Library/Application Support/KRK/keymap.toml` **vollständig** abgewiesen wird.
Die Releaseseite sagt davon nichts, und sie **kann** es in ihrer heutigen Bauart auch nicht sagen:
`RELEASETEXT` ist eine Konstante mit genau einer Fügestelle für die Zahl. Damit ist die Möglichkeit 4
aus `shared/issues/260823-1030_c_die-umbenannte-kennung-weist-jede-bestehende-keymap-toml-vollstaendig-ab.md`
— „es bleibt, wie es ist, und die Releaseseite sagt es dem Nutzer" — mit dem vorhandenen Werkzeug
nicht ausführbar, und sie ist auch nicht ausgeführt worden.

---

**Am Baum und an der veröffentlichten Seite gelesen.**

## Was die Auslieferung tatsächlich veröffentlicht hat

`gh release view v1.0.0`, abgerufen am 260823-1645. Der Rumpf trägt vier Abschnitte: die Kopfzeile,
`**Voraussetzung:** macOS 15 oder neuer`, `## Installieren` mit drei Schritten und
`## Die alte Fassung vorher nicht löschen`. Das Wort „Tastenbelegung" kommt einmal vor, und zwar in
der Aufzählung dessen, was ein **Löschwerkzeug** mitnimmt. Kein Satz sagt, dass eine vorhandene
`keymap.toml` beim ersten Start der 1.0.0 verworfen wird.

## Warum der Text es nicht sagen kann

`xtask/src/veroeffentlichung.rs:551-575` führt `RELEASETEXT` als `const &str`, `:529` die einzige
Fügestelle `ZAHLPLATZHALTER = "{zahl}"`, und `:580` setzt sie mit `str::replace`. Der Doc-Kopf
(`:531-550`) schreibt die Absicht aus: „**Er kommt aus dem Werkzeug und nicht aus der
Versionsgeschichte.** Kein `git log`, keine `RELEASE_NOTES.md`". Ein Hinweis, der nur für **eine**
Auslieferung gilt, hat in dieser Bauart keinen Ort. Die Probe
`der_releasetext_traegt_jede_seiner_aussagen` hält jede Aussage des Texts einzeln und würde eine
per Auslieferung wechselnde Zeile nicht tragen.

Die Entscheidung, den Text fest zu halten, ist richtig und für ihren Zweck begründet: die
Betriebsregel gegen den Bestandsverlust gilt für jede Fassung. Der Umbenennungsfall ist die erste
Aussage, die **nicht** für jede gilt, und er trifft auf die Bauart, die für das Gegenteil gebaut ist.

## Warum das jetzt zählt und am 260823-1125 noch nicht zählte

Der geschlossene Datensatz `260823-1030` nimmt die Lage an, und seine Begründung ist am 260823-1140
nachgeschärft worden auf: **auf keiner der beiden Maschinen des Nutzers liegt eine `keymap.toml`**.
Der genannte Auslöser lautet dort: „liegt irgendwo eine `keymap.toml`" — „und die Antwort darauf
kennt der Entwickler nur für seine eigenen Geräte."

Am Baum erhoben, 260823-1645:

- `gh repo view tenzoki/krk` meldet `"isPrivate": false`, `"visibility": "PUBLIC"`.
- Zwei Releaseseiten bestehen: `v0.5.6` vom 260821 und `v1.0.0` vom 260823.
- `KRK-0.5.6.zip` trägt **vier** Herunterladungen, `KRK-1.0.0.zip` zwei.

Wer eine der vier 0.5.6-Kopien betreibt und darin je eine Taste umbelegt hat, hat genau die Datei,
für die dieser Fall geschrieben ist. Ob das jemand getan hat, ist von hier aus nicht entscheidbar —
und das ist der Punkt: die Schließungsbegründung von `260823-1030` stützt sich auf eine Tatsache
über zwei Maschinen, während die Auslieferung eine unbekannte Menge weiterer erreicht.

## Was zu entscheiden ist

Der Datensatz `260823-1030` ist geschlossen und bleibt es; sein Ergebnis wird hier nicht
aufgemacht. Offen ist allein, ob und wie eine auslieferungsbezogene Warnung an den Nutzer kommt.
Drei Wege, keiner hier gewählt:

1. **`RELEASETEXT` bekommt einen zweiten, optionalen Abschnitt**, den `cargo xtask release` aus
   einem Argument oder einer Datei nimmt. Kostet die Zusage „der Text kommt aus dem Werkzeug" zur
   Hälfte und braucht eine Antwort darauf, was die Probe dann noch hält.
2. **Die Releaseseite bleibt fest, und der Hinweis wird von Hand nachgetragen**, über
   `gh release edit`. Kostet nichts am Code, steht in keinem Werkzeug und wird beim nächsten Mal
   vergessen.
3. **Es bleibt, wie es ist.** Der Fall trifft niemanden, oder er trifft jemanden und dessen
   Belegung ist mit einer Zeile in der eigenen Datei wiederherzustellen
   (`editor_aus_vorschau` → `editor_rundweg`, die Gegenmaßnahme steht in `260823-1030`).

**Schwere:** Medium. Kein Verhalten ist betroffen, und die Wirkung hängt an einer Menge, die von
hier aus nicht zu erheben ist. Der Befund gehört trotzdem festgehalten, weil er die einzige
Möglichkeit betrifft, die zur Behandlung des Falls ausdrücklich genannt und dann nicht gegangen
worden ist.

**Gefunden:** reconciler, Abgleich zum Abschluss der Sitzung `260823-1424`, Baumstand `7d86420`

**Domain:** code

**Cross-references:**
`shared/issues/260823-1030_c_die-umbenannte-kennung-weist-jede-bestehende-keymap-toml-vollstaendig-ab.md`,
`shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`,
`shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`,
`xtask/src/veroeffentlichung.rs:531-575`

---
Resolved:

---
Resolved: Als Lage angenommen, nicht behoben. Der Nutzer hat am 260823-1710 entschieden, dass die
Ladezahlen der Releaseseite keine betroffenen Nutzer bedeuten: die Downloads sind seine eigenen.
Damit trifft der Verlust der Tastenbelegung niemanden, und die Seite bekommt keinen Hinweis.

**Der strukturelle Teil des Befunds bleibt wahr und ist mit dieser Schließung nicht erledigt.**
`RELEASETEXT` in `xtask/src/veroeffentlichung.rs` ist eine Konstante und für jede Fassung
dieselbe; eine Releaseseite kann bauartbedingt keinen fassungsspezifischen Umstiegshinweis tragen.
Die nächste Umbenennung einer Kennung trifft dieselbe Lücke wieder, und dann womöglich mit
Nutzern, die nicht der Entwickler sind. Wer sie dann schließen will, findet hier die Vorarbeit.
