Die Probe "Befehl → Zweig → Wirkung" prueft Vorhandensein statt Paarung und bleibt bei vertauschten Zweigen gruen

---

`jeder_kontextbefehl_erreicht_seine_wirkung` soll die eine Stelle halten, die der Uebersetzer nicht
haelt: dass jeder der drei Kontextbefehle bei **seinem** Zweig ankommt. Sie stellt aber zwei
voneinander unabhaengige Fragen an denselben Rumpf — steht der Befehlsname darin, und steht der
Funktionsname darin —, und beide sind auch dann erfuellt, wenn die Zuordnung vertauscht ist. Ein
"Zip", das entpackt, laesst sie gruen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-ui/src/appkit/anwendung.rs:9071`:

```rust
assert!(
    verzweigung.contains(befehl) && verzweigung.contains(zweig),
    "{befehl} erreicht {zweig} nicht: der Menüeintrag stünde da und täte nichts"
);
```

`verzweigung` ist der ganze Rumpf von `kontextbefehl_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:6081-6087`), also alle drei Zeilen auf einmal. Der Rumpf
traegt alle drei Befehlsnamen und alle drei Zweignamen, gleich wie sie einander zugeordnet sind.

## Die Gegenprobe im Kopf

Waere der Rumpf so geschrieben:

```rust
Kontextbefehl::Zippen => self.entpackauftrag_stellen(seite),
Kontextbefehl::Entpacken => self.zipauftrag_stellen(seite),
Kontextbefehl::ImFinderZeigen => self.im_finder_zeigen(seite),
```

dann enthaelt `verzweigung` weiterhin "Kontextbefehl::Zippen" und weiterhin
"zipauftrag_stellen", also sind beide `contains` wahr. Die zweite Haelfte der Probe prueft danach
`rumpf(&datei, "zipauftrag_stellen")` auf `Auftrag::zippen(` — das steht dort, denn der Zweigrumpf
ist ja unveraendert. Alle drei Durchgaenge gruen, und der Eintrag "Zip" entpackt.

Das ist nicht die abwegigste denkbare Aenderung, sondern genau die, gegen die die Probe geschrieben
wurde: die drei Zeilen sehen einander aehnlich, und eine vertauschte ist beim Lesen schwer zu
sehen. Der Doc-Kommentar der Probe sagt "Jeder der drei Kontextbefehle erreicht einen Zweig" und
behauptet damit mehr, als der Rumpf prueft.

## Was die zwei Nachbarproben decken und was nicht

- `der_kontextmenue_selektor_hat_einen_empfaenger_und_einen_setzer`
  (`crates/krk-ui/src/appkit/tabelle.rs:5299`) haelt den Selektor: genau eine Erklaerung, genau ein
  Setzer, beide unter demselben Namen. Sie sagt nichts ueber die Marken.
- `der_kontextmelder_wird_beim_aufbau_gesetzt` (`crates/krk-ui/src/appkit/anwendung.rs:9012`) haelt
  die eine Aufrufstelle des Rueckrufs. Sie sagt nichts ueber die Zuordnung dahinter.
- Der Rundweg Marke → Wert → Marke ist von
  `der_rundweg_ueber_die_marke_schliesst` und `keine_marke_steht_zweimal`
  (`crates/krk-ui/src/kommandos/kontextmenue.rs:588-620`) sauber gehalten, und
  `jeder_befehl_traegt_seinen_titel_und_seine_marke` haelt Titel und Marke gegen eine von Hand
  geschriebene Tafel.

Die Kette ist damit an jedem Glied gehalten **ausser** an diesem einen: welcher Befehl welche
Wirkung ausloest.

## Vorschlag

Die Zuordnung je Zeile pruefen statt je Rumpf. Der Rumpf steht als Zeichenfolge schon da; es genuegt,
ihn zeilenweise zu lesen und zu verlangen, dass genau **eine** Zeile beide Nadeln traegt:

```rust
let treffer = verzweigung
    .lines()
    .filter(|zeile| zeile.contains(befehl) && zeile.contains(zweig))
    .count();
assert_eq!(treffer, 1, "...");
```

Mit der Vertauschung oben faellt `treffer` auf 0 und die Probe wird rot. Der Doc-Kommentar der Probe
ist im selben Zug nachzuziehen: er soll sagen, dass die **Paarung** gehalten wird, und weiterhin
benennen, was die Zaehlung nicht sieht (eine Wirkung, die in eine tiefer gerufene Hilfsfunktion
gewandert ist).

## Umfang

`krk-ui`, `appkit/anwendung.rs`, Pruefmodul `kontextproben`. Kein Code der Anwendung.
