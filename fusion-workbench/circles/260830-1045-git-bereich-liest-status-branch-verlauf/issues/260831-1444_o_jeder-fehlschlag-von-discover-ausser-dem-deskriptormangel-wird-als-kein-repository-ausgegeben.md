Jeder Fehlschlag von `discover` außer dem Deskriptormangel wird als „kein Repository" ausgegeben

---
`Gitleser::oeffnen` (`crates/krk-core/src/git/leser.rs:148-152`) teilt in drei Fälle:

```rust
match gix::discover(ordner) {
    Ok(repo) => Oeffnung::Offen(Box::new(Self { repo })),
    Err(fehler) if fehlerkette_meldet_deskriptormangel(&fehler) => Oeffnung::Unentschieden,
    Err(_) => Oeffnung::KeinRepository,
}
```

Der dritte Zweig ist eine **entschiedene** Verneinung über den Ordner, und der Nutzer bekommt sie als Satz zu sehen: `KEIN_REPOSITORY` = „Dieser Ordner liegt in keinem Git-Repository." (`git/texte.rs:29`).

`gix::discover::Error` trägt aber zwei Varianten (`gix-0.87.1/src/discover.rs:11-16`): `Discover(upwards::Error)` — darunter „kein Repository gefunden", aber auch `IoError` und `InaccessibleDirectory` — und `Open(open::Error)`, das heißt: **ein Repository ist gefunden worden und ließ sich nicht öffnen** (unlesbare Konfiguration, unbekannte Erweiterung, ein `gitdir:`-Verweis ins Leere). Alle diese Fälle liefern heute die Auskunft „hier ist keines".

Das ist genau die Zusammenziehung, die der Modulkopf von `crates/krk-core/src/git/mod.rs:23-41` für sich ausschließt („`None` heisst ‚unentschieden' und nie ‚nichts gefunden'", mit dem Verweis auf den Defekt `260815-0211`), und die Fallunterscheidung ist damit nicht disjunkt-vollständig: „kein Repository" und „nicht feststellbar" sind zwei Fragen, und heute entscheidet allein die Fehlernummer 24 zwischen ihnen.

**Abnahmetest:** ein Prüfrepository, dessen `.git/config` unlesbar gemacht ist, liefert `Oeffnung::Unentschieden` und nicht `Oeffnung::KeinRepository`; die Unterscheidung läuft über `discover::Error::Open(_)` beziehungsweise über die Varianten von `upwards::Error` und nicht über eine zweite Liste von Fehlernummern.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23, beim Prüfen von C7.8 gegen den Modulkopf von `git/mod.rs`. Die Kindprobe `kind_liest_unter_abgesenkter_deskriptorgrenze` deckt allein den Deskriptorfall ab.
