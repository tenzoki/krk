Der Entscheid zum Über-Dialog nennt zwei Befehle als Beispiel, die heute schon nicht durchkommen

---

Der Datensatz `decisions/260813-1037_a_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`
begründet die Lücke im Abschnitt `## Question` so:

> Steht der Über-Dialog vorn, kann deshalb ein Tastendruck einen Befehl im
> Fenster dahinter auslösen, und `F5` startete dann eine Kopieroperation,
> `delete` räumte in den Papierkorb.

Beide genannten Befehle können in dieser Lage nicht wirken, und zwar aus einem
Grund, der schon im Baum steht. `Kommando::Kopieren` und `Kommando::InPapierkorb`
tragen `Wirkungsbereich::Dateifenster`
(`crates/krk-core/src/tasten/belegung.rs:889` und `:902`, beide im Sammelzweig ab
`:889`). `Anwendungsdelegierter::fokus` (`crates/krk-ui/src/appkit/anwendung.rs:4043`)
fragt als erstes, ob das Schlüsselfenster das Hauptfenster ist, und antwortet
sonst `Fokus::Anderswo`. `fokus::wirkt(Dateifenster, Anderswo)` ist `false`.
Steht ein fremdes Fenster vorn, weist der dritte Bestandteil der Regel diese
beiden Befehle also bereits ab.

---

**Was wirklich durchkommt**

Die Lücke besteht, aber sie ist enger und benennbar: es sind genau die Befehle
mit `Wirkungsbereich::Ueberall`, denn für diesen Bereich sagt `fokus::wirkt`
auch bei `Fokus::Anderswo` ja. Am 260813 aus
`Kommando::wirkungsbereich` (`belegung.rs:741` bis `:873`) abgelesen sind das
vierundzwanzig der sechsundsiebzig Kommandos:

`leiste_umschalten`, `erstes_fenster_umschalten`, `zweites_fenster_umschalten`,
`vorschau_umschalten`, `editor_umschalten`, `spalte_groesse_umschalten`,
`spalte_datum_umschalten`, `spalte_typ_umschalten`, `fenster_einblenden`,
`fenster_schliessen`, `bereich_verbreitern`, `bereich_verschmaelern`,
`abbrechen`, `belegung_ansehen`, `beenden`, `weitere_instanz`, `fokus_leiste`,
`fokus_dateifenster`, `fokus_vorschau`, `fokus_editor`, `lesezeichen_anlegen`,
`tab_schliessen`, `ordner_der_datei`, `teilen`.

Darunter sind die drei, die der Defekt zum Freigabedialog namentlich nennt
(`tab_schliessen` auf Cmd+W, `ordner_der_datei` auf Opt+Cmd+O, `teilen` auf
Shift+Cmd+S), und mit `belegung_ansehen` einer, der ein Blatt am Hauptfenster
aufzieht, während ein fremdes Fenster vorn steht.

**Warum das für die Runde zählt**

Die gewählte Möglichkeit 2 bleibt richtig, und die vierte Bedingung schließt
genau diese vierundzwanzig. Die Zahl ändert aber, woran der Bau abzunehmen ist:
die Tafel in `crates/krk-ui/src/kommandos/zulaessigkeit.rs` misst über
Stellvertreter je Wirkungsbereich, und der Unterschied zwischen alter und neuer
Regel fällt allein in der Zeile `Ueberall` an. Eine Probe, die den Unterschied
an `Kommando::Oeffnen` zeigen wollte, zeigte nichts.

**Was zu tun ist**

Den Abschnitt `## Question` des Entscheids berichtigen: die beiden Beispiele
gegen zwei aus der Liste oben tauschen und die Reichweite der Lücke als
„die Befehle mit `Wirkungsbereich::Ueberall`" benennen. Der Entscheid selbst
und seine Antwort bleiben unberührt.

Gefunden beim Bau des Umsetzungsplans dieser Runde
(`planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
Strang A).

---

**Abgleich 260813-1345: zu Recht offen.** Der Abschnitt `## Question` des Entscheids
`decisions/260813-1037_i_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`
nennt unverändert `F5` und `delete`. Beide Feststellungen dieses Datensatzes am Baum
nachgelesen und bestätigt: `Kommando::Kopieren` und `Kommando::InPapierkorb` tragen
`Wirkungsbereich::Dateifenster`, und `fokus_bei` antwortet vor einem fremden Schlüsselfenster
`Fokus::Anderswo` (`crates/krk-ui/src/appkit/anwendung.rs:4169-4174`).

Die Zahl **24 von 76** hält: `Kommando` trägt 76 Varianten, davon 24 mit
`Wirkungsbereich::Ueberall`. Die Liste ist seit Turn 2 in einem Punkt anders zu lesen —
`fenster_einblenden` steht darauf und kommt seither über die Ausnahmeliste durch
(`crates/krk-ui/src/kommandos/zulaessigkeit.rs:198-201`), es sind also 23 der 24, die die neue
Bedingung wirklich sperrt.

Der Entscheid trägt seit diesem Abgleich den Marker `_i_`; die Berichtigung, die dieser Datensatz
verlangt, ist davon unberührt und bleibt zu tun.
