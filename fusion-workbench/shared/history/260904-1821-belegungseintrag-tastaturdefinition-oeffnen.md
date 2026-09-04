# Der Belegungseintrag „Tastaturdefinition öffnen"

**Status:** Complete
**Filed by:** ontocoder, Kai Stalmann <kai@qantr.com>
**Datum:** 260904-1821
**Auftrag:** Nutzerauftrag im Anschluss an den `coder`-Schritt vom 260901-0734, kein Circle aktiv

---

## Was eingetragen wurde

Der `[[funktion]]`-Block `belegungsdatei_ansehen` in `resources/default-keymap.toml`,
mit `tasten = []`, hinter `belegung_ansehen` im Abschnitt „C3: die Belegungsansicht".
Der `coder` hat die Kennung in `belegungsausgabe.rs` und in `OHNE_KOMBINATION_AB_WERK`
ans Ende der Aufzählung gesetzt; die Aufzählung dort ist die Reihenfolge dieser Datei,
und die Stelle hinter `belegung_ansehen` liegt hinter allen Einträgen ohne Kombination.

## Angefasste Dateien

- `resources/default-keymap.toml` — der neue Block samt Kommentar, und die
  Kopfzeile „# Ausgeliefert sind ... Funktionen mit zusammen ... Kombinationen."

Sonst nichts. Das Kommando stand schon.

## Die zwei Zählstände im Kopf

Beide nach dem Eintrag am Bestand gezählt und nicht aus der alten Zeile fortgerechnet:

```sh
grep -c '^\[\[funktion\]\]' resources/default-keymap.toml            # 92
grep '^tasten = ' resources/default-keymap.toml \
  | grep -o '"[a-z0-9+]*"' | wc -l                                   # 95
```

Die erste Zahl ist um eins gestiegen, die zweite nicht: der Eintrag trägt keine
Kombination. Gehalten werden beide von
`tasten::belegung::tests::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`,
die die Kopfzeile liest und selbst nachzählt.

## Was im Kommentar steht, und warum

Drei Auskünfte, die ein späterer Leser sonst nicht bekommt:

**Warum keine Kombination.** Nutzerentscheid, wie bei den Spaltenschaltern und den
zwei Schaltern der Bereichsleiste. Mit leerer Tastenliste und nicht mit
`reserviert_fuer`: das Feld heißt „benannt, aber einer späteren Runde vorbehalten",
und diese Funktion gibt es. Der Weg ist das Hauptmenü. Damit fällt der Eintrag aus
der Markdown-Ausgabe der Runde 3, die eine Funktion nur aufnimmt, wenn sie
mindestens eine Kombination trägt (`belegungsausgabe.rs`, der Filter auf
`!funktion.tasten().is_empty()`).

**Welche Datei gezeigt wird.** Die Belegungsdatei des Nutzers unter
`~/Library/Application Support/KRK/keymap.toml`, nicht die Auslieferungsfassung:
die liegt im Bündel gar nicht als Datei, sondern ist über `include_str!` zur
Bauzeit einkompiliert (`crates/krk-core/src/tasten/belegung.rs:159`, auf genau
`resources/default-keymap.toml`).

**Die zwei Schreiber.** KRK liest die Datei einmal beim Start und lädt sie im
Betrieb nicht nach (`belegung::fuer_den_betrieb`); eine Änderung von Hand wirkt
deshalb erst beim nächsten Start. Die Belegungsansicht auf `f1` schreibt beim
Verlassen die ganze Arbeitskopie zurück, die sie aus dem Stand des Starts gebaut
hat, und jede Handänderung seither ist danach fort. Der Befehl sagt es beim Öffnen
in der Statuszeile. Ob KRK mehr dagegen tun soll, ist offen:
`shared/decisions/260901-0734_*_haelt-krk-die-belegungsdatei-gegen-ihren-zweiten-schreiber-oder-bleibt-es-beim-hinweis.md`.

**Eine Formulierung im Entwurf war zu grob und ist berichtigt worden**, bevor der
Kommentar stehen blieb: „die Belegungsansicht überschreibt sie beim Verlassen"
gilt nicht unbedingt. `Anwendungsdelegierter::belegungsansicht_verlassen`
(`crates/krk-ui/src/appkit/anwendung.rs:4140`) kehrt bei `!modell.geaendert()`
zurück, ohne zu schreiben; ohne Änderung bleibt `keymap.toml` unberührt. Der
Kommentar sagt das jetzt so. Der Satz in der Statuszeile
(`kommandos::operationen::belegungsdatei_hat_zwei_schreiber`) ist die Kurzform für
den Nutzer und bleibt, wie der `coder` ihn gesetzt hat.

## Keine Zahl im Kommentar

Der Kommentar nennt weder die Zahl der Spaltenschalter noch die Zahl der Einträge
ohne Kombination. Beide wachsen: der Absatz vor `spalte_groesse_umschalten` sagt
„die drei Spaltenschalter darueber", und es sind seit der Runde 23 vier. Der neue
Kommentar sagt stattdessen „wie die Spaltenschalter und die Schalter der
Bereichsleiste darueber" und „wie jeder Eintrag ohne Kombination".

## Verifikation

`make check` → Exit 0. 24 Prüfziele, alle grün. Die vier vom `coder` als rot
gemeldeten Proben laufen jetzt durch:

- `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`
- `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`
- `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
- `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`

Dazu `tasten::belegung::tests::die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`,
die den nachgezogenen Kopf hält. Kein weiterer Fehlschlag im Lauf: `grep 'test result:
FAILED\|failures:\|panicked'` über das Protokoll findet nichts.

Die Datei parst: `Belegung::auslieferung()` liest sie in jeder dieser Proben über
`include_str!` und `toml::from_str`; ein Schreibfehler hätte den ganzen Lauf
angehalten.
