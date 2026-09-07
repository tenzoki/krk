# Trägt das cmd+f-Paar auf demselben Grund wie cmd+a, oder auf einem dritten?

---
**Domain:** code
**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md` (die Zustellerregel), `260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md` (der Entscheid, der cmd+f belegt), `260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md` (der Befund, aus dem die Zeile entstand), `260907-2026_*_die-antwortzeile-zu-cmd-f-begruendet-das-paar-mit-dem-wirkungsbereich-die-regel-vom-260805-nennt-den-zusteller.md` (dieselbe Naht, Frage nach der Begründung in der Antwortzeile)

---

## Question

`resources/default-keymap.toml` trägt seit dem 260907 zwei Fälle einer Kombination
bei zwei Zustellern, cmd+a und cmd+f. Die Datei selbst hat eine dritte Belegung
dieser Bauart schon einmal abgelehnt, und die Ablehnung steht bis heute im
Kommentar bei `eintragspfad_kopieren`: „Die dritte Moeglichkeit des Datensatzes
haette es nach dem Vorbild von Cmd+A mit einem zweiten Zusteller belegt; sie ist
verworfen, solange nicht gemessen ist, ob der Fokusvorbehalt dort so trennt wie bei
Cmd+A." Für cmd+f ist diese Messung nicht gefahren; der Ablauf ist gelesen und
nicht gemessen, und der Abnahmelauf über `make tasten` und `make menue` ist
Nutzerarbeit.

Die Frage ist damit nicht, ob cmd+f funktioniert, sondern **woran es hängt**, denn
die zwei Paare trennen an verschiedenen Bestandteilen derselben Regel. Gelesen in
`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, Modulkopf `# Die vier
Bestandteile`:

- **cmd+a** trennt an Bestandteil (2), dem Fokusvorbehalt. Mit dem Fokus in der
  Liste ist `alle_markieren` zulässig und der Abgriff schluckt den Anschlag; mit
  dem Fokus im Textfeld gehört der Ersthelfer AppKit, und (2) weist ab. Die zwei
  Zusteller sehen denselben Anschlag nie.
- **cmd+f** trennt an Bestandteil (3), `fokus::wirkt`, also am Wirkungsbereich. Mit
  dem Fokus im Dateifenster gehört der Ersthelfer **KRK**, (2) sagt also ja; der
  Abgriff schlägt `editor_suchen` nach, findet es, und erst (3) weist es ab, weil
  `Kommando::EditorSuchen` `Wirkungsbereich::Editor` trägt. Danach fällt der
  Anschlag an AppKit, und dort trägt der Menüeintrag von `filter_einfuegen` das
  Kürzel.

Der Datensatz vom 260805 hält Bestandteil (3) ausdrücklich aus der
Konflikterkennung heraus („`Wirkungsbereich` ist kein zweiter Zusteller"). Für die
Konfliktfrage ist das richtig und unberührt: `Belegung::konflikte` vergleicht nur
innerhalb desselben Zustellers, und `die_auslieferungsbelegung_ist_konfliktfrei`
bleibt grün. Für die **Erreichbarkeit** des zweiten Eintrags hängt cmd+f aber genau
an jener Eigenschaft, der der Datensatz kein Gewicht gibt. Wer aus „cmd+a trägt,
also trägt cmd+f" schließt, überträgt eine Zusage über Bestandteil (2) auf einen
Fall, den Bestandteil (3) trägt.

## Options

1. **Das Paar wird als eigener Fall benannt und gemessen.** Der Abnahmelauf der
   Runde prüft ausdrücklich, dass cmd+f mit dem Fokus im Dateifenster den
   Filtertext füllt und mit dem Fokus im Editor die Suche öffnet, und die Belegung
   schreibt neben dem Kommentar bei `filter_einfuegen` aus, dass hier (3) trennt
   und nicht (2).
   - Pro: Die Zusage deckt dann den Fall, den sie meint. Der Kommentar bei
     `eintragspfad_kopieren` behält seine Bedingung, und cmd+f erfüllt sie
     sichtbar.
   - Contra: Kostet einen Abnahmepunkt mehr, und die Messung ist Nutzerarbeit.
2. **Die Bedingung im cmd+c-Kommentar fällt.** Sie war ein Vorbehalt der Runde 4;
   seither trägt cmd+a das Muster im Alltag, und der Ablauf über (3) ist im Code
   nachlesbar. Der Kommentar wird auf „verworfen, weil cmd+c am Dateifenster
   bereits `copy:` beantwortet" umgeschrieben oder gestrichen.
   - Pro: Eine Bedingung, die niemand einlöst, bindet nichts und täuscht Deckung
     vor.
   - Contra: Sie ist der einzige Ort, an dem die Datei überhaupt nach einer Messung
     dieses Musters fragt. Fällt sie, fällt die Frage.
3. **Beides bleibt, wie es ist.** cmd+f trägt ohne Messung, der cmd+c-Kommentar
   verlangt eine.
   - Pro: kostet nichts.
   - Contra: Die Datei nennt eine Bedingung und hält sie im Nachbarabsatz nicht
     ein. Der nächste Autor, der ein drittes Paar erwägt, liest zwei einander
     widersprechende Auskünfte in derselben Datei.

## Constraints

- Die Zustellerregel vom 260805 bleibt unberührt: Konflikt ist gleicher Zusteller
  auf gleicher Kombination, und der Wirkungsbereich geht in die Konflikterkennung
  nicht ein. Diese Frage betrifft die Erreichbarkeit, nicht die Konflikterkennung.
- Kein Agent kann den Abnahmelauf fahren; er verlangt KRK im Vordergrund.

## Recommendation

Keine. Möglichkeit 1 und 2 schließen einander nicht aus, und welche der beiden
Auskünfte in `default-keymap.toml` gilt, ist eine Festlegung des Nutzers und keine,
die sich aus dem Code ergibt.
