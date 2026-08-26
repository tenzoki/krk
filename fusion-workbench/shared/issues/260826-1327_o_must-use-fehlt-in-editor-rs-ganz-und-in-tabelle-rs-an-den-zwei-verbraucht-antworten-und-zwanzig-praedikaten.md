must-use fehlt in editor.rs ganz und in tabelle.rs an den zwei Verbraucht-Antworten und zwanzig Praedikaten

---

`crates/krk-ui/src/appkit/editor.rs` traegt kein einziges `#[must_use]` (5.319 Zeilen), obwohl sechs
Befehle eine `Editormeldung` zurueckgeben, deren stilles Fallenlassen genau das "kommentarlos nichts
tun" ist, das C2 und C5 verbieten. `tabelle.rs` traegt vierzehn, aber nicht an `kommando_ausfuehren`
und `filterzeichen_tippen`, deren `bool` sagt, ob der Tastendruck verbraucht ist — und der Baum laesst
diesen Wert an zwei Stellen schon still fallen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

**editor.rs, ohne `#[must_use]`:** `grep -c '#\[must_use\]' crates/krk-ui/src/appkit/editor.rs` liefert 0.
Reine Antworten ohne Markierung:

- `Editormeldung::text` (:662), `Editormeldung::markenstelle` (:648); die Aufzaehlung selbst (:520-521)
- `Umkehrpunkt::zwischen` (:789), `angewandt_auf` (:809), `getragene_bytes` (:834)
- `verlauf_fuer_umbau` (:977), `gemeinsamer_anfang` (:994), `gemeinsamer_schwanz` (:1013),
  `bis_zur_zeichengrenze` (:1033), `kopfzeile` (:3063)
- `Oeffnungsherkunft::ist_aus_sitzung` (:1234), `haelt_datei` (:1602), `hat_ungesicherten_stand` (:1611),
  `pfad` (:1626), `schreibmarkenzeile` (:2356), `suchtexte` (:2507)
- die sechs Befehle mit `Editormeldung` als Antwort: `zeile_anspringen` (:2438), `marke_anspringen`
  (:2490), `suche_beginnen` (:2533), `weitersuchen` (:2546), `rueckwaerts_suchen` (:2551),
  `treffer_ersetzen` (:2600), `alle_treffer_ersetzen` (:2686); dazu `sichern` (:1708) und
  `fremdaenderung_melden` (:1639)

Auch die Typen tragen es nicht: `Editormeldung` (editor.rs:520), `Ladeausgang` (editormodell.rs:501-502),
`Sicherungsausgang` (editormodell.rs:551-552). Zum Vergleich: `Auswahlversuch` in `tabs.rs:325` traegt
`#[must_use = "…"]`, und genau das hat den Defekt `260807-0219` gefangen.

**tabelle.rs, ohne Markierung obwohl die Antwort die Sache ist:**

- `kommando_ausfuehren(&self, Kommando) -> bool` (:1677): "hat dieses Dateifenster den Befehl
  ausgefuehrt". `anwendung.rs:7861` und `:7889` (Messmodus) lassen den Wert nackt fallen;
  `anwendung.rs:844` schreibt fuer den Nachbarn `let _ =`. Zwei Schreibweisen fuer dieselbe Sache, und
  keine haelt der Bau.
- `filterzeichen_tippen(&self, char) -> bool` (:2058): "verbraucht oder nicht" — ein nicht
  verbrauchter Anschlag laeuft an AppKit weiter (Doc :2042-2045).
- die Praedikate `zeilen` (:1611), `liest_noch` (:1619), `auswahlzeile` (:1628), `auswahl_pfad` (:1636),
  `vorgang_sichtbar` (:1652), `namenszelle_in_bearbeitung` (:2688), `tiefe_suche_steht` (:2853),
  `inhaltssuche_steht` (:2913), `filter_steht` (:2929), `zeile_markiert` (:3046),
  `zeile_steht_wegen_des_inhalts` (:3062), `seitenhoehe` (:2128), `eintrag_in_zeile` (:2349),
  `Namensfeld::delegierter` (:4606) sowie die sechs Spaltenfunktionen `kennung` (:319), `titel` (:343),
  `breiten` (:352), `ausrichtung` (:365), `aus_kennung` (:376), `typ_beschriften` (:4948).

## Warum es zaehlt

CLAUDE.md: "Ein Rueckgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt in diesem Projekt
`#[must_use]`" (Nutzerentscheid 260811-2140). Die Datei `filter_leeren` (:2988-2989) traegt es aus genau
diesem Grund, ihr Nachbar `filterzeichen_tippen` nicht.

## Querverweise

Dieselbe Luecke haben vier Durchsichten dieser Sitzung in ihren Modulgruppen gefunden:
`260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-…`,
`260826-1221_*_must-use-traegt-sieben-praedikate-des-verzeichnisbaums-…`,
`260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-…`,
`260826-1305_*_krk-bench-traegt-ein-einziges-must-use-…`. Ein Durchgang ueber den ganzen Baum ist
billiger als fuenf je Modul.

## Was zu tun waere

`#[must_use]` an die drei Typen (`Editormeldung`, `Ladeausgang`, `Sicherungsausgang`) und an die
genannten Funktionen; danach `cargo clippy --workspace --all-targets` mit `-D warnings` fahren und an
`anwendung.rs:7861` und `:7889` entscheiden, ob `let _ =` die richtige Antwort ist.

## Umfang

`krk-ui`, `appkit/tabelle.rs`, `appkit/editor.rs`, `editormodell.rs`; Rufer in `appkit/anwendung.rs`.
