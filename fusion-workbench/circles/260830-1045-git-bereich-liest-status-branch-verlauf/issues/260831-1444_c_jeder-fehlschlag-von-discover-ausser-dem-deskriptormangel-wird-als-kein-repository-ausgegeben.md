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

---
Resolved: 260831. `Gitleser::oeffnen` hat keinen Auffangzweig mehr: `entschiedene_verneinung` zerlegt `gix::discover::Error` und `gix::discover::upwards::Error` vollständig und ohne Auffangzweig, und wahr wird genau eine Lage — bis zur Wurzel gesucht und nichts gefunden, also `NoGitRepository`, `NoGitRepositoryWithinCeiling`, `NoGitRepositoryWithinFs`. Jeder andere Fehlschlag, `Open(_)` eingeschlossen, ist `Oeffnung::Unentschieden`. Eine Variante mehr in `gix` hält damit den Bau an, statt still in die Verneinung zu fallen.

Die zweite Liste von Fehlernummern ist weg, und das ist keine Lücke, sondern die Folge: `fehlerkette_meldet_deskriptormangel` ist gestrichen, weil die drei Varianten, die überhaupt einen `io::Error` tragen können — `CurrentDir`, `CheckTrust` und `Open(_)` —, ohnehin sämtlich unentschieden sind, während die drei Varianten der entschiedenen Verneinung nichts als Pfade tragen. Die Kindprobe `kind_liest_unter_abgesenkter_deskriptorgrenze` misst es unverändert und bleibt grün: unter `ulimit -n 64` liefert `oeffnen` weiter `Unentschieden`.

**Der Abnahmetest ist auf dem zweiten seiner beiden Halbsätze erfüllt und auf dem ersten nicht in seinem Wortlaut.** Die Unterscheidung läuft über `discover::Error::Open(_)` beziehungsweise die Varianten von `upwards::Error` und über keine Fehlernummer — das ist der Maßstab, und er ist gehalten. Die verlangte Auslösung dagegen erreicht `gix` nicht: eine `.git/config` mit `chmod 000` liefert gemessen `Ok(repo)`, `gix` übergeht die Datei also. Die Probe `ein_nicht_zu_oeffnendes_repository_bleibt_unentschieden` (`crates/krk-core/tests/git.rs`) macht die Konfiguration deshalb im Sinne des Zerlegers unlesbar statt im Sinne der Rechte und trifft damit genau `Open(Config(Init(Parse)))`, die Variante, die der Abnahmetest benennt; ihre Gegenprobe hält, dass derselbe Ordner vorher einen Leser liefert. Vor der Änderung meldete sie `KeinRepository`.

**Ein Rest bleibt, und er liegt in `gix` und nicht mehr hier.** Ein unlesbares `.git`-Verzeichnis und ein `.git` mit totem `gitdir:`-Verweis melden gemessen beide `NoGitRepository`, tragen also keine Ursache, die KRK auseinanderhalten könnte; sie kommen weiter als entschiedene Verneinung beim Nutzer an. Eigener Datensatz mit der Messung aller fünf geprüften Lagen: `260831-1652_*_gix-zieht-ein-unlesbares-git-verzeichnis-und-einen-toten-gitdir-verweis-selbst-zu-kein-repository-zusammen.md`; der Modulkopf von `crates/krk-core/src/git/leser.rs` nennt die Grenze.
