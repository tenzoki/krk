# Bündelbau, Versionsersetzung und lokale Signierung (Schritt 5)

**Datum:** 260802-1927
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 5 in der Fassung des Nachzugs vom 260802-1859
**Geänderte Dateien:** `xtask/src/main.rs` (einbindend), `xtask/src/bundle.rs` (neu), `xtask/src/sign.rs` (neu), `README.md` (neu)
**Nicht angefasst:** `resources/Info.plist`, `crates/`, `spikes/`, `Cargo.toml`, `.cargo/config.toml`

## Was entstanden ist

`cargo xtask bundle` baut `target/KRK.app`. Die Arbeit liegt in drei Dateien.

`xtask/src/main.rs` bindet die beiden neuen Module ein und trifft die
Unterbefehlsauswahl. Die Fehlerbehandlung übernimmt das Muster aus `krk-bench`:
ein `Abbruch` mit den beiden Fällen `Aufruf` (Rückgabewert 2, die Befehlszeile
stimmt nicht) und `Lauf` (Rückgabewert 1, der Bau ist gescheitert). Kein neues
Fehlerkonzept und keine neue Abhängigkeit; `xtask/Cargo.toml` bleibt ohne
Einträge unter `[dependencies]` und musste deshalb nicht in die Dateiliste.

`xtask/src/bundle.rs` baut das Bündel. **Die Reihenfolge trägt die
Fehlerpfade:** erst die Versionsersetzung, dann der Name des Binärprogramms,
dann die Signaturidentität, und erst danach wird übersetzt und geschrieben.
Alles, was scheitern kann, scheitert damit, bevor ein Verzeichnis entsteht. Wer
die Entwicklungsidentität noch nicht angelegt hat, erfährt es vor und nicht nach
einem vollständigen Übersetzungslauf.

`xtask/src/sign.rs` bestimmt die Identität und ruft `codesign`. Die Suche geht
zuerst an `KRK_SIGN_IDENTITY`, dann an den Schlüsselbund. Findet sie nichts,
bricht der Bau mit einer Anleitung ab und weicht nicht auf eine Ad-hoc-Signatur
aus.

`README.md` beschreibt Bau, Bündelbau, Signierung, Versionspflege und die
Erzeugung der Entwicklungsidentität.

## Drei Festlegungen, die der Plan offenließ

**Das Bauprofil ist `release`.** Der Plan nennt keines. `debug` wäre der
naheliegende Vorgabewert, aber dasselbe Bündel misst ab Schritt 8 die
Zeitzusagen aus C8, und eine Zahl aus einem unoptimierten Bau sagt über diese
Zusagen nichts aus. Sie würde das Gate aus Schritt 8 grundlos reißen, und dessen
Reißen bedeutet laut Plan, dass der Technologieentscheid zur Debatte steht. Das
ist der teuerste denkbare Fehlalarm, deshalb `release`. Der Grund steht als
Kommentar an der Konstante und im README.

**Der Name des Binärprogramms kommt aus der `Info.plist`.** `CFBundleExecutable`
bestimmt, welche Datei macOS im Bündel startet; der Bau liest den Namen von dort
und schreibt ihn nicht ein zweites Mal in den Programmtext. Eine Abweichung
zwischen beiden wäre ein Bündel, das sich bauen lässt und nicht startet. Die
`Info.plist` sagt das in ihrem eigenen Kommentar voraus: "Schritt 5 nimmt an
`KRK.app/Contents/MacOS/krk` ab, dieser Wert muss dazu passen."

**Kein eigener Unterbefehl `sign`.** Die Verzeichnisübersicht des Plans nennt für
`xtask/` die vier Aufgaben "bundle, sign, messen, release". `sign.rs` gibt es,
aber als Modul, das `bundle` ruft, nicht als eigenen Aufruf: kein
Abnahmekriterium von Schritt 5 ruft `cargo xtask sign`, und ein Befehl ohne
Abnehmer wäre auf Vorrat gebaut. Nachrüsten kostet fünf Zeilen, sobald ein
Schritt ihn braucht.

## Die Versionsersetzung

Die Version wohnt seit dem 260802-1911 allein im Feld `version` unter
`[workspace.package]` der `Cargo.toml`. `xtask` erbt sie über
`version.workspace = true`, `env!("CARGO_PKG_VERSION")` holt sie beim
Übersetzen, und `bundle` setzt sie an die Stelle des Platzhalters
`__KRK_VERSION__`. Ersetzt wird ausschließlich in der Kopie im Bündel; die
Quelldatei bleibt unangetastet, nachgeprüft über `git diff -- resources/`
(leer).

Fehlt der Platzhalter, bricht der Bau ab und baut kein Bündel. Der Fall ist
nicht bloß behandelt, sondern ausgelöst worden, siehe Abnahme.

## Abnahme

Alle Kommandos am 260802 auf dem Referenzgerät ausgeführt.

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | Rückgabewert 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | Rückgabewert 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | Rückgabewert 0 |
| `cargo test --workspace` | 73 Tests, 0 Fehler (16 davon neu in `xtask`) |
| `cargo clippy --workspace --all-targets` | keine Warnung |
| `cargo fmt --all --check` | Rückgabewert 0 |
| `cargo xtask bundle` | Rückgabewert 0 |
| Struktur | `Contents/{Info.plist,PkgInfo,MacOS/krk,Resources,_CodeSignature}` |
| `Contents/PkgInfo` | genau 8 Bytes, `APPL????` |
| `codesign --verify --strict target/KRK.app` | Rückgabewert 0 |
| `codesign -dv` | `flags=0x0(none)`, also keine Ad-hoc-Signatur |
| `codesign -dvv` | `Authority=KRK Entwicklung` |
| `vtool -show-build-version .../MacOS/krk` | `minos 15.0` |
| `plutil -extract CFBundleShortVersionString raw .../Contents/Info.plist` | `0.1.0`, gleich dem Wert aus `Cargo.toml` |
| `grep '__KRK_VERSION__' .../Contents/Info.plist` | kein Treffer |
| `plutil -lint` auf Quelle und Kopie | beide OK |

**Beide Abbruchpfade sind ausgelöst worden, nicht nur geschrieben.**

Ohne Identität: mit leerem Schlüsselbund gerufen, Rückgabewert 1, Anleitung auf
der Standardfehlerausgabe, `target/KRK.app` nicht angelegt.

Ohne Platzhalter: in einer Arbeitskopie des Projekts unter dem Scratchpad, deren
`Info.plist` den Platzhalter durch `0.1.0` ersetzt trug. Rückgabewert 1, kein
`target/KRK.app`, und auch kein `target/release` — der Abbruch kommt vor dem
Übersetzungslauf. Die echte `resources/Info.plist` blieb dabei unberührt.

**Die Versionsersetzung ist gegen eine andere Zahl geprüft.** Ein Vergleich von
`0.1.0` gegen `0.1.0` beweist nichts, weil beide Seiten zufällig gleich sein
können. In derselben Arbeitskopie wurde die Workspace-Version auf `9.9.9`
gesetzt und der Platzhalter wiederhergestellt; das Bündel trägt danach `9.9.9`.
Die Zahl fließt also wirklich aus der `Cargo.toml`.

## Zur Signaturidentität: was gemessen wurde und was am Gerät geändert wurde

Auf dem Referenzgerät gab es zu Beginn **keine** Signaturidentität
(`security find-identity -p codesigning` meldete 0). Ohne eine ist der
Erfolgspfad nicht ausführbar. Für die Abnahme wurde deshalb vorübergehend eine
selbstsignierte Identität `KRK Entwicklung` in einem **eigenen** Schlüsselbund
angelegt, dieser in die Suchliste des Nutzers aufgenommen, die Abnahme gefahren
und anschließend beides zurückgebaut: Schlüsselbund gelöscht, Suchliste wieder
auf `login.keychain-db` allein, Schlüsselmaterial im Scratchpad gelöscht.
Nachgeprüft: `security list-keychains -d user` und
`security find-identity -p codesigning` stehen wieder auf dem Anfangsstand.

Drei Dinge sind dabei gemessen worden, die das README trägt:

1. **`codesign` signiert mit einer nicht als vertrauenswürdig eingetragenen
   selbstsignierten Identität, und `codesign --verify --strict` nimmt das
   Ergebnis an.** Die Vertrauenseinstellung, die einen GUI-Dialog verlangt, ist
   für die Entwicklung nicht nötig.
2. **`security find-identity -v -p codesigning` findet eine solche Identität
   nicht**, weil `-v` auf die als gültig bewerteten filtert und eine
   selbstsignierte ohne Vertrauenseintrag als `CSSMERR_TP_NOT_TRUSTED` gilt. Die
   Suche in `sign.rs` läuft deshalb ohne `-v`. Mit `-v` würde der Bau eine
   Identität ablehnen, die nachweislich trägt.
3. **OpenSSL 3 schreibt PKCS#12 in Verfahren, die der Schlüsselbund nicht
   liest.** Ohne `-macalg sha1 -certpbe PBE-SHA1-3DES -keypbe PBE-SHA1-3DES`
   scheitert `security import` mit `MAC verification failed`. Der Weg im README
   nennt die drei Angaben.

Nicht ausgeführt und deshalb nicht behauptet: der Import in den
**Anmeldeschlüsselbund** des Nutzers und der Weg über den Zertifikatsassistenten
der Schlüsselbundverwaltung. Beide stehen im README, beide sind am Gerät nicht
erprobt.

## Ein offener Punkt zur Wortlautauslegung

Das Abnahmekriterium sagt: "Ein Lauf gegen eine `Info.plist` ohne Platzhalter
bricht mit Rückgabewert ungleich 0 ab und hinterlässt kein Bündel." Umgesetzt
ist die Lesart "er erzeugt keines": der Abbruch kommt vor jedem Schreibzugriff,
und ein **älteres** Bündel aus einem früheren erfolgreichen Lauf bleibt liegen.

Die Gegenlesart wäre, das alte Bündel vor der Prüfung zu löschen. Dann könnte
niemand ein veraltetes Bündel für frisch halten — aber ein Lauf, der bloß die
Identität nicht findet, zerstörte ein funktionierendes Artefakt. Die gewählte
Lesart ist die schonendere und deckt sich mit der Begründung im Plan, es solle
kein Bündel mit veralteter Zahl "entstehen". Der Punkt ist klein, aber er ist
eine Entscheidung und keine Selbstverständlichkeit.

## Ein Widerspruch und eine Ungenauigkeit im Plan

**Der Widerspruch: erzeugt Schritt 5 die Identität oder nicht?** `### Frage 7`
schreibt: "Für die Entwicklung erzeugt S5 deshalb eine lokale, selbstsignierte
Code-Signing-Identität im Schlüsselbund und verwendet sie durchgängig."
Die `Änderungen` von Schritt 5 schreiben das Gegenteil: der Schritt "sucht [...]
eine lokale selbstsignierte Identität namens `KRK Entwicklung` und bricht mit
einer Anleitung zu ihrer Erzeugung ab, wenn auch die fehlt."

Am selben Entscheidungspunkt verlangen die beiden Stellen Verschiedenes, und der
Fall ist eingetreten: das Referenzgerät hatte keine Identität. Damit war das
Abnahmekriterium "`codesign -dv` nennt die Identität" durch den Schritt allein
nicht erreichbar; es braucht eine Handlung des Nutzers, die der Schritt nicht
enthält.

Umgesetzt ist die Fassung aus den `Änderungen`, weil sie die speziellere und die
für den Schritt normative Stelle ist: `bundle` legt keine Identität an. Die
Erzeugung steht als Anleitung in der Abbruchmeldung und im README. Die Abnahme
selbst lief gegen eine eigens angelegte und danach wieder entfernte Identität,
siehe oben. Der Punkt gehört vor den nächsten Schritt entschieden, weil Schritt
6 mit `cargo xtask bundle && open target/KRK.app` abnimmt und ohne Identität
nicht startet.

**Die Ungenauigkeit: `codesign -dv` nennt die Identität nicht.** Das
Abnahmekriterium verlangt, `codesign -dv target/KRK.app` nenne "die Identität,
nicht `adhoc`". Gemessen: `-dv` gibt die Zeile `Authority=` nicht aus. Es zeigt
`flags=0x0(none)`, womit die Ad-hoc-Signatur ausgeschlossen ist, aber der Name
der Identität erscheint erst bei `-dvv`. Beide Kommandos sind gelaufen und beide
Ausgaben stehen in der Abnahmetabelle; die Zusage des Kriteriums ist damit
erfüllt, nur nicht mit dem Kommando, das dort steht.

## Nicht gemacht

Kein Fenster, kein Menü, keine Dateiliste (Schritt 6). Kein Unterbefehl `sign`,
`messen` oder `release`. Keine dauerhafte Änderung am Schlüsselbund des Nutzers.
Kein Commit, keine Markeränderung am Plan, keine Effort-Schätzung.
