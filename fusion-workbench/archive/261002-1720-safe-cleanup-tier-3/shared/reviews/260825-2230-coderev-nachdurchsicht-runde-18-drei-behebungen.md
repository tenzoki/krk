# Nachdurchsicht der Behebungsrunde zur Runde 18: drei Behebungen, drei halten

**Reviewed-range:** `ecd7e4b..1ac5dde`
**Not-opened:** none

**Durchgesehen von:** coderev, Kai Stalmann <kai@stalmann.org>
**Am:** 260825-2230
**Gelesen gegen:** die drei geschlossenen Datensätze `shared/issues/260825-2127_c_*.md`, den offenen `shared/issues/260825-2127_o_eine-unlesbare-zugriffszeit-…` und den Vorgängerbericht `shared/reviews/260825-2127-coderev-runde-18-vorschau-vertieft-und-zwei-fehler.md`.
**Zum Bereich:** `fb7db85` und `ecd7e4b` ändern allein `fusion-workbench/`; ihre Dateien sind überflogen und nicht beurteilt. `1ac5dde` ändert allein `resources/default-readers.toml`, die `ontorev` parallel liest; geöffnet ist sie hier insoweit, wie `tests/leseprofil.rs` sie über `AUSLIEFERUNGSTEXT` und `genannte_orte` liest (das Profil „fusion-Werkbank: der gemeinsame Speicher", `:12-92` jenes Abschnitts).

---

## Zusammenfassung

Die drei Behebungen tun, was ihre Datensätze verlangen, und jede ist gegen den alten Stand
rot gefahren worden — bei zweien habe ich das an der Bauform der Probe nachvollzogen, bei
der dritten glaube ich es der Commit-Botschaft, weil eine Wiederholung den Baum anfassen
müsste. `make check` ist am 260825-2230 in allen vier Teilen selbst gefahren: `cargo fmt`
sauber, `cargo clippy --workspace --all-targets` ohne Meldung, 23 Prüfziele, keine
gescheiterte Probe. Ein Befund, gering, und er liegt nicht im Code: der aktive Plan verlangt
noch die Zeile, die die zweite Behebung gestrichen hat.

---

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 0 |
| Gering | 1 |

Die sieben offenen Datensätze vom 260825-1859 bis 2127 sind vorher gelesen; nichts daraus
ist wiederholt.

---

## Die drei Behebungen, einzeln

### `f8304a7` — die Zählprobe auf `aktives_setzen`

**Hält, was der Datensatz verlangt.** Beide Punkte des Datensatzes sind umgesetzt:

- Die Probe `aktives_setzen_hat_genau_zwei_aufrufer`
  (`crates/krk-ui/src/appkit/anwendung.rs:8957`) zählt über `quellbaum::aufrufstellen`
  (`crates/krk-ui/src/quellbaum.rs:133`). Die Zählung ist an der Bauform nachvollzogen:
  Nadel ist `aktives_setzen(`, Zeilen mit `//` am Anfang fallen heraus (also auch jede
  `///`-Zeile des Doc-Kommentars), die Erklärung `fn aktives_setzen(` (`:4577`) fällt am
  `fn` davor heraus, und der Probenname `aktives_setzen_hat_…` trifft die Nadel nicht,
  weil nach `setzen` ein `_` und keine Klammer folgt. Bleiben genau `:1289`
  (`selbst.aktives_setzen(seite, mitnahme)`, der Rückruf aus `aktivierung_setzen`) und
  `:4656` (in `aktives_dem_ersthelfer_nachziehen`). Ein dritter Ruf in jeder Schreibweise,
  die `aufrufstellen` kennt — Empfänger, Pfad, Zeilenumbruch vor dem Punkt —, hebt die Zahl.
- Der Satz an `Rangmitnahme` (`crates/krk-ui/src/appkit/tabelle.rs:752-762`) trennt jetzt
  Wert und Weg und nennt die zwei Proben beim Namen; beide stehen im Modul
  `aktivschreiberproben` (`anwendung.rs:8753`), wie der Satz sagt.

**„Was sie nicht sieht" ist ehrlich.** Die zwei genannten Blindheiten sind die
wesentlichen. Die erste — ein Aufrufer aus einer anderen Datei — ist wirklich durch die
Privatheit gedeckt: `aktives_setzen` steht ohne `pub` in `impl Anwendungsdelegierter`, und
`anwendung.rs` meldet kein Untermodul in einer anderen Datei an (`grep '^\s*mod [a-z_]*;'`
liefert nichts), also gibt es keinen Ort außerhalb dieser Datei, der die Methode überhaupt
rufen könnte. Die zweite — der Wert, den ein Aufrufer hinschreibt — ist die, die der
Vorgängerbefund als eigentliche Gefahr benannt hat, und die Probe sagt ausdrücklich, dass sie
sie dem Leser aufgibt statt sie zu entscheiden. Eine dritte, die die Probe nicht nennt und
ich für vernachlässigbar halte: eine Nennung von `aktives_setzen(` in einem
Blockkommentar `/* … */` würde mitgezählt und machte die Probe rot statt still grün. Rot ist
die ungefährliche Richtung.

### `acc9671` — das Packen meldet das Ersatzdatum nicht mehr

**Beide Enden treffen jetzt dieselbe Wahl.** `zeit_uebernehmen`
(`crates/krk-core/src/operation/zippen.rs:650-692`) nimmt keine `Steuerung` mehr entgegen und
ruft `ueberspringen` an keiner Stelle; `entpacken.rs:346-353` lässt `zeit_setzen` mit
`let _ =` stumm scheitern und schreibt den Grund hin. Die Begründung steht einmal, am
Doc-Kommentar von `zeit_uebernehmen` (`:637-649`), und der Modulkopf (`:132-137`) verweist
dorthin statt sie zu wiederholen. Der Modulkopf sagt weiter (`:110`), eine Datei, die
`is_file()` nicht bejaht, bekomme ihren Grund in der Abschlussliste — das ist ein wirkliches
Überspringen und bleibt richtig.

**Mit dem Parameter ist nichts verschwunden, das der Nutzer sehen sollte.** Die drei
gestrichenen Meldungen im Einzelnen:

1. *Änderungsdatum nicht lesbar* — der Eintrag steht im Archiv mit Vorgabedatum; das ist
   Weg 1 des Datensatzes, so gewählt.
2. *Zeitpunkt außerhalb 1980 bis 2107* — dito; die Probe
   `ein_zeitpunkt_vor_1980_faellt_auf_das_vorgabedatum_und_bleibt_aus_der_abschlussliste`
   (`crates/krk-core/tests/operation.rs:1559`) hält jetzt `uebersprungen.is_empty()`, das
   Vorgabedatum 1980-01-01 im MS-DOS-Feld und `Some(0)` im erweiterten Zeitfeld nebeneinander.
   Gegen den alten Stand wäre sie am `is_empty()` rot; das ist an der Bauform ablesbar.
3. *`add_extra_data` schlägt fehl* (`zippen.rs:688`, jetzt `let _ =`) — in `zip-8.6.0`
   nachgelesen (`src/write.rs:369-370`): die Funktion scheitert allein, wenn die
   Zusatzfelder zusammen `u16::MAX` Bytes überschreiten oder die Kennung ohne das Merkmal
   `unreserved` reserviert ist. Mit neun und acht Bytes und dem gesetzten Merkmal ist keiner
   der zwei Fälle zur Laufzeit erreichbar. Die Meldung, die hier fiel, konnte nie erscheinen.

Die drei Wahlbauer `dateiwahl`, `ordnerwahl`, `verknuepfungswahl` (`:551`, `:565`, `:584`)
haben den Parameter verloren, den sie nur durchreichten; alle vier Funktionen tragen weiter
ihr `#[must_use]`. Der offene Datensatz `260825-2127_o_eine-unlesbare-zugriffszeit-…`
betrifft weiter dieselbe Bedingung, die jetzt an `zippen.rs:667` steht; die dort
vorgeschlagene Behebung passt unverändert, nur die Zeilennummer ist gewandert. Nicht
gedoppelt.

**Was diese Behebung offen lässt und wo es steht: der Plan.** Siehe den Befund unten.

### `a9868a2` — die C6.7-Probe misst das Speicherprofil

**Die Zahlen folgen aus der Datei, soweit eine exakte Probe das zulässt.**
`genannte_orte` (`crates/krk-core/tests/leseprofil.rs:2888`) sammelt die verschiedenen Orte
der Zeilen des Speicherprofils aus der eingebetteten Auslieferungsfassung, und
`gemeinsamer_speicher` (`:2918`) legt den Prüfordner aus genau dieser Liste an. Die Probe
(`:2967`) hält dann in dieser Reihenfolge: `orte.len() == 10` (`:3071`), Beschriftungen der
Zusammenfassung gleich denen des Profils (das richtige Profil hat gegriffen),
`(leselaeufe, oeffnungen) == (10, 0)`, `leselaeufe == orte.len()` (`:3100`), Abstand `2` zur
Schranke, keine Zeile `Wert::Nicht`. Die 10 ist also zweifach festgemacht — als gezählte
Orte der Datei und als gemessene Läufe —, die 0 an den Öffnungen ist eine reine Messung, und
die 2 folgt aus `HOECHSTENS_LESELAEUFE` und der 10. An der wirklichen Datei nachgelesen: das
Profil führt zwanzig Zeilen auf zehn Ordnern, jeder zweimal, alle `zeigt = "datum"`.

**Ein elfter Ort in der wirklichen Datei macht sie rot**, und zwar an der ersten
Behauptung (`:3071`, „das Speicherprofil nennt nicht mehr zehn Unterspeicher"), bevor
irgendetwas gemessen wird. Die Dauerprobe `ein_elfter_unterspeicher_kostet_einen_elften_leselauf`
(`:3140`) misst dagegen an einer **Kopie** von `AUSLIEFERUNGSTEXT` mit eingefügter Zeile und
lässt die Datei unberührt; sie belegt, dass die Messung darüber einen zusätzlichen Ort
überhaupt sieht (elf Läufe, Abstand eins). Beide zusammen sind dicht: die eine hält die Zahl,
die andere hält, dass die Zahl etwas misst.

Zwei Anmerkungen ohne Datensatz, weil kein Befund:

- Die Behauptung `step_by(2) … == Wert::Zahl(1)` (`:3124`) setzt voraus, dass in der Datei
  die Zählungszeile jedes Ordners vor seiner Datumszeile steht. Vertauscht jemand die
  Reihenfolge, wird die Probe rot mit der Meldung „die Zaehlungen sehen nicht je den einen
  Datensatz" — irreführend, aber rot und nicht still grün. Der Doc-Kommentar nennt diese
  Kopplung nicht.
- `genannte_orte` hält an, sobald ein Ort des Speicherprofils einen Platzhalter trägt, und
  sagt warum. Das ist die richtige Grenze für die Rechnung „ein Ort, ein Leselauf".

`1ac5dde` hat die Datei nach `a9868a2` geändert; das Speicherprofil ist davon nicht
berührt (der Diff trifft allein das `feldmuster` der Zeile „Sitzung" und Kommentare), und
der Testlauf bestätigt es.

---

## Befund

### L1 — Der Plan verlangt in Schritt 3 noch die Zeile, die `acc9671` gestrichen hat

`shared/issues/260825-2230_o_der-plan-der-runde-18-verlangt-in-schritt-3-noch-die-zeile-in-der-abschlussliste-die-acc9671-gestrichen-hat.md`

`shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md:243` und `:251`.
Der Entwurf sagt „mit einer Zeile in der Abschlussliste", das Abnahmekriterium „erzeugt genau
eine Zeile in der Abschlussliste; eine Probe hält beides". Beides ist seit `acc9671` das
Gegenteil des Baums, absichtlich, und der Plan trägt keinen Nachtrag dazu, obwohl er für
denselben Schritt schon einen hat (`260825-1859`). Kein Code ist zu ändern; ein zweiter
Nachtrag in der Form des ersten genügt.

**Schwere: gering.**

---

## Bindungen, nachgesehen

- `#[must_use]`: an `zeit_uebernehmen` und den drei Wahlbauern erhalten; die neue Probe gibt
  nichts zurück.
- Vollständige Fallunterscheidungen: `genannte_orte` verzweigt über alle vier `Baustein`-Werte
  ohne Auffangzweig; ein fünfter Baustein hält die Probe beim Übersetzen an. `Rangmitnahme`
  unverändert.
- Untergrenzen-Abschnitt: `anwendung.rs` und `tabelle.rs` ändern ihre Modulköpfe nicht.
- Zahlen in Prosa: „zwei Zählproben" an `Rangmitnahme` zählt zwei beim Namen genannte Proben
  und veraltet nur mit ihnen; „zehn von zwölf" und „Abstand zwei" im Kopf der C6.7-Probe hält
  die Probe selbst; „bis zu drei Zeilen je Eintrag" an `zeit_uebernehmen` ist als Vergangenheit
  formuliert („bis zum 260825").
- `Cargo.lock`: im Bereich unverändert.

---

## Was diese Durchsicht nicht sagt

- Nichts über das Gesehene; der Klick-Fokus und das Konfliktblatt sind Nutzerarbeit.
- Nichts über den Handbuchteil von `resources/default-readers.toml`; der liegt bei
  `ontorev`, und die fünf Datensätze aus `1ac5dde` sind hier nicht beurteilt.
- Nichts Neues zu den sieben offenen Datensätzen der Runde.
