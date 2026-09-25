# Codedurchsicht: Turn 1 der Runde 8 — Titelleiste, Über-Eintrag, Tag-Prüfung

**Datum:** 260813-1258
**Sender:** coderev
**Reviewed-range:** `59b0a6c..21dbc59`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/orchestrator-live.md`, `fusion-workbench/circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_t_circle.md`
**Grundlage:** der Spec `planning/260813-1037_o_spec-…` (sechs Fähigkeiten, 59 Kriterien), der Plan `planning/260813-1110_o_plan-…` (Stränge A bis E), die fünf beantworteten Entscheide in `decisions/` dieses Circles, die vier Sitzungsberichte der Ausführer in `history/`, und das macOS-SDK unter `MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/`.
**Nicht gefahren:** kein `make bundle`, kein `cargo xtask bundle`, kein `cargo xtask release`. `make check` ist von den Ausführern grün gemeldet und nicht wiederholt worden.

---

## Zusammenfassung

Die vier Commits setzen die zwölf Planschritte A1 bis D5 um, und der Bau trägt, was der Spec verlangt: der Titelleisten-Bereich, der Über-Eintrag, die Tag-Prüfung als Station eins und der Abschnitt in `README.md`. Vier Befunde stehen dagegen, und einer davon ist ein Verlust gegenüber heute: **die neue Schlüsselfensterbedingung sperrt „Fenster einblenden" (Cmd+N) genau in der Lage, für die dieser Befehl da ist** — nachdem das Hauptfenster geschlossen wurde. Die drei übrigen sind Prosa- und Zusagegenauigkeit und halten keinen Bau und kein Verhalten auf.

Die vier Stellen, an denen die Ausführer vom Planwortlaut abgewichen sind, sind alle vier geprüft und alle vier richtig.

## Zahlen

| Schwere | Anzahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 0 |
| Niedrig | 3 |

Jeder Befund liegt als eigener Datensatz unter `issues/` dieses Circles, alle mit dem Zeitstempel `260813-1258`.

---

## Befund 1 (hoch): Der Rückweg zum geschlossenen Fenster ist weg

**Datensatz:** `issues/260813-1258_o_fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar.md`
**Betroffen:** `crates/krk-ui` (Zulässigkeitsregel und Anwendungsdelegierter)

`schluesselfenster()` (`crates/krk-ui/src/appkit/anwendung.rs:2632-2647`) faltet zwei Lagen zu `Schluesselfenster::Fremd`: ein fremdes Fenster steht vorn, und KRK hat gar kein Schlüsselfenster. Die zweite Lage tritt ein, sobald das eine Hauptfenster geschlossen ist. `zulaessig` (`kommandos/zulaessigkeit.rs:164-173`) weist dann jeden Befehl ab, der nicht auf der Ausnahmeliste steht, und die führt zwei Namen: `Beenden` und `FensterSchliessen` (`:189-191`).

`Kommando::FensterEinblenden` steht nicht darauf. Es trägt `Wirkungsbereich::Ueberall` (`krk-core/src/tasten/belegung.rs:749`), kam vor dieser Runde also durch, und `resources/default-keymap.toml:510` nennt seine Aufgabe beim Namen: „Der Rueckweg, nachdem das Fenster geschlossen wurde." Nach der Änderung weist ihn der Ereignisabgriff ab, und `validateMenuItem:` graut denselben Eintrag im Menü „Fenster" aus, weil beide dieselbe Regel fragen. Übrig bleibt allein der Klick auf das Dock-Symbol, der über `applicationShouldHandleReopen:` läuft und die Regel nicht berührt.

Das widerspricht der ausgeschriebenen Randbedingung „Kein Verlust gegenüber heute" desselben Spec und C7 der Runde 1. Der Entscheid `decisions/260813-1110_a_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md` hat die Ausnahmeliste genau aus dieser Randbedingung begründet — und dabei nur die beiden Befehle geprüft, die schon darauf standen.

**Erhoben am Quelltext, nicht am Bündel.** Die Kette ist kurz und ohne Verzweigung; die Bestätigung gehört in die Liste aus E2.

**Abhilfe:** `Kommando::FensterEinblenden` auf die Ausnahmeliste, plus eine Probe dafür. Das folgt der Bedeutung, die der Entscheid der Liste selbst gegeben hat. Der genauere Schnitt wäre ein vierter Wert `Keines` in `Schluesselfenster`, der aber die Tafel von 280 auf 420 Fälle zieht. Die Wahl gehört dem Nutzer, wenn er sie treffen will; der kleine Weg ist der, den die bestehende Mechanik vorsieht.

## Befund 2 bis 4 (niedrig): drei Genauigkeitsmängel

| Datensatz | Ort | Was falsch ist |
|---|---|---|
| `…_o_zwei-prosastellen-in-anwendung-rs-zaehlen-noch-drei-werte-und-einen-sonderposten.md` | `appkit/anwendung.rs:2549-2552` und `:733-735` | Der Zeichenzweig nennt „dieselben drei Werte" bei vier Feldern der `Lage`; die Aufzählung an `validateMenuItem:` nennt einen Sonderposten, wo jetzt zwei in den Zweig fallen. Beide Stellen stehen in einer Datei, die der schon gemeldete Befund `260813-1420` nicht führt. |
| `…_o_die-versionszahlprobe-sagt-baum-und-liest-nur-crates.md` | `appkit/titelzusatz.rs:298-311` | Der Doc-Kommentar sagt „in keiner `.rs`-Datei des Baums"; `quellbaum::quelldateien()` liest `crates/`, und `xtask/src/release.rs` führt die Zahl viermal als Prüfstoff. Die Probe ist richtig, ihre Zusage zu weit. |
| `…_o_der-modulkopf-von-titelzusatz-laesst-die-bedingung-fuer-left-weg.md` | `appkit/titelzusatz.rs:54-68` | Der SDK-Kopf knüpft `NSLayoutAttributeLeft` an „applications linked on Mac OS 10.11 or later"; der Modulkopf zählt `Left` unter den unbedingt zulässigen Werten. Folgenlos bei einem Ziel von 15.0, aber es steht in dem Abschnitt, der die Gegenmaßnahme gegen fehlende Verfügbarkeitsangaben ist. |

---

## Die vier Abweichungen vom Planwortlaut, einzeln geprüft

**1. Der Tagvergleich prüft die ganze Zeile auf Gleichheit.** Richtig, und enger als der Plan zu Recht. `stand_pruefen` (`xtask/src/release.rs:228`) fragt `zeile.trim() == erwartet` statt eines Präfixes; `v0.1.0-rc1` und `v0.1.10` decken damit `0.1.0` nicht. C3.1 verlangt „einen Tag mit dem Namen `v<version>`", also genau Gleichheit. Die Probe `ein_aehnlicher_tag_deckt_die_version_nicht` (`:995-998`) hält es fest, mit beiden Gegenbeispielen.

**2. Der Modulkopf von `titelzusatz.rs` zitiert das SDK genauer als der Plan.** Neun von zehn Angaben stimmen wörtlich und mit ihrer Zeilennummer, am SDK nachgelesen; die Tabelle steht im Datensatz zu Befund 4. Gesetzt ist `Left` (`titelzusatz.rs:192`) und nicht `Leading` — die Stelle, an der ein Fehler zur Laufzeit abgebrochen wäre, ist richtig. Die eine Lücke ist die Bedingung an `Left`, und sie ist folgenlos.

**3. Die umbenannte Probe zählt zwei Sorten getrennt.** Richtig, und die Begründung trägt: `die_leiste_traegt_zwei_sonderposten_und_zwei_trenner` (`menuemodell.rs:857-878`) prüft die Sonderposten und die Trenner einzeln, weil eine Summe von vier auch dann stünde, wenn ein Sonderposten seinen Trenner verlöre und ein anderer einen dazubekäme. Daneben prüft `der_ueber_eintrag_steht_ganz_oben` (`:801-847`) die Stelle relativ und den Selektor ausgeschrieben, damit die Zusicherung keine Tautologie gegen die eigene Konstante ist.

**4. Die Zählprobe zählt zuerst Dateien und dann Fundstellen.** Richtig. `nur_eine_stelle_im_baum_setzt_namen_und_version_zusammen` (`titelzusatz.rs:266-284`) hält erst die Liste der Dateien auf `titelzusatz.rs` und dann die Zahl der Fundstellen darin auf zwei — die Zusammensetzung in `beschriftung` und die in ihrer Zusicherung. Nachgezählt: der Aufbau der Nadel selbst über `concat!("\"KRK \", env!(", …)` findet sich nicht mit, die Zahl zwei ist richtig.

## Was sonst geprüft und in Ordnung ist

- **Kein Verhaltenswechsel in `fokus`.** `fokus_bei` bildet den alten Ausstieg eins zu eins ab: `Hauptfenster` geht in den Ansichtsbaum, `BlattAmHauptfenster` und `Fremd` antworten `Fokus::Anderswo` (`anwendung.rs:4169-4174`). Das Blatt kommt weiter durch, weil es selbst das Schlüsselfenster ist und `gehoert_krk()` dafür `true` liefert; die Belegungsansicht ist ein Blatt und nicht ein eigenes Fenster (`appkit/belegungsansicht.rs:3`).
- **Der Messmodus ist nicht betroffen.** `oberflaeche_aufbauen` ruft `fenster_zeigen` mit `makeKeyAndOrderFront` und `activate()` (`anwendung.rs:1148`), und `messung_unmoeglich` bricht ohnehin ab, solange KRK nicht vorn steht (`messmodus.rs:743-746`).
- **`NSAlert::runModal` (`appkit/hinweis.rs:91`) gewinnt sogar.** Vor dieser Runde kam hinter dem modalen Fenster jeder `Ueberall`-Befehl durch; jetzt nur noch die Ausnahmeliste.
- **Die eine Erhebung ist eine.** `lage` liest `schluesselfenster()` einmal und reicht den Wert an das Feld und an `fokus_bei` weiter (`anwendung.rs:2663-2674`). `fokus()` bleibt als Hülle für seine fünf übrigen Aufrufer, nachgezählt: `:1168`, `:1713`, `:3454`, `:4822`, `:5286`.
- **C6.4 hält.** `titelzusatz.rs` trägt den Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`; 26 der 28 Dateien unmittelbar unter `appkit/` tragen ihn, ohne ihn sind weiter allein `koordinaten.rs` und `mod.rs`. Die Modulliste steht bei 28 Namen, und die Prosazahl im Kopf ist von „Sechsundzwanzig" (schon vorher falsch) auf „Achtundzwanzig" berichtigt.
- **Die Tag-Prüfung liest nur.** Drei Argumentlisten als Konstanten (`release.rs:104-127`), eine Probe hält sie gegen vierzehn schreibende Unterbefehle und prüft, dass `git tag` sein `--points-at` trägt und keinen Namen (`:1096-1123`). `git_fragen` ist die einzige Stelle mit `Command::new("/usr/bin/git")`, gehalten von einer Zählprobe über alle `.rs`-Dateien des Baums (`:1142-1163`).
- **`bundle::VERSION` bleibt die eine Quelle.** `xtask/Cargo.toml` erbt `version.workspace = true`, die Konstante ist von privat auf `pub(crate)` gehoben, und `release` liest sie statt die `Cargo.toml` zu zerteilen.
- **Keine Umlaute in den neuen `xtask`-Meldungen**, wie im Rest der Kiste. Die einzigen Zeichen außerhalb von ASCII in `release.rs` stehen in Kommentaren.
- **C6.1, C6.2, C6.5 und C6.6 sind Ergebnisse und nicht behauptet:** `crates/krk-core`, `resources/` und `Cargo.toml`/`Cargo.lock` tragen im ganzen Bereich keine Änderung.

## Querschnittliches

**Die Prosa läuft dem Code hinterher, und der Grund ist wiederkehrend.** Vier Textstellen stehen bereits als `issues/260813-1420_o_…`, zwei weitere kommen mit Befund 2 dazu. Alle sechs entstehen auf dieselbe Art: ein Planschritt zählt seine Dateien abschließend auf, die Aussage steht in einer anderen Datei, und der Ausführer hält sich zu Recht an die Liste. Das Muster ist kein Fehler der Ausführer, sondern eine Eigenschaft der Schrittgrenzen. Ein Schritt, der eine gezählte Aussage ändert („drei Bestandteile", „genau ein Sonderposten", „140 Fälle"), braucht in seiner Dateiliste die Dateien, die die Zahl nennen — zu finden mit einer Suche nach der Zahl, bevor die Liste geschrieben wird.

**Der schon gemeldete Punkt 1 jenes Befunds steht weiter offen.** `appkit/menue.rs:1132` sagt „die Tafel aus 140 Faellen", obwohl Strang C dieselbe Datei geöffnet und fünf andere Prosastellen darin nachgezogen hat. Der Datensatz hatte genau das vorgeschlagen („`menue.rs` fällt in Strang C dieser Runde an"); die Übergabe zwischen den beiden Strängen hat sie nicht getragen.

**Die Regel hat vier Bestandteile und niemand hat die Liste der betroffenen Befehle gelesen.** Der Sitzungsbericht von Strang A sagt richtig: „Was sich ändert, ist genau die Zeile `Wirkungsbereich::Ueberall`." Der Plan zählt die vierundzwanzig Befehle dieser Zeile sogar. Geprüft worden ist danach keiner von ihnen einzeln, und Befund 1 ist genau der eine, dessen Aufgabe die neue Lage ist. Wer eine Sperre einführt, die eine ganze Wirkungsbereichszeile trifft, geht die Zeile durch.

## Reihenfolge

1. **Befund 1 vor dem Abschluss der Runde.** Es ist ein Verlust gegenüber heute an einem zugesagten Bedienweg, und die Runde schließt sonst mit einer gebrochenen eigenen Randbedingung. Eine Zeile in `immer_erreichbar`, eine im Modulkopf, eine Probe.
2. **Befund 2 und der offene Punkt 1 von `260813-1420` zusammen**, sobald jemand `anwendung.rs` und `menue.rs` ohnehin öffnet.
3. **Befund 3 und 4** sind Aufräumarbeit ohne Frist.

Kein Befund hält die Abnahme am Bündel (E2) auf. Befund 1 gehört allerdings in ihre Beobachtungsliste: Fenster über Shift+Cmd+W schließen, Cmd+N drücken, Menü „Fenster" öffnen.

## Zwei Anmerkungen ohne Datensatz

**Die Zeitstempel dieser Runde liegen vor der Uhr.** `date` meldet auf diesem Gerät `260813-1258`; die vier Commits stehen zwischen 12:14 und 12:47, aber `history/260813-1310-coder-strang-b-titelleiste.md` und `issues/260813-1420_o_…` tragen spätere Zeiten als jetzt. Die Konvention verlangt den Stempel aus `date +%y%m%d-%H%M`. Die Dateien dieser Durchsicht tragen deshalb `1258` und sortieren vor den beiden genannten.

**C4 ist ohne Probe abgenommen.** Die sieben Kriterien zu `README.md` tragen im Spec ein **(Probe)**, und der Plan sieht für D4 keine vor; der Ausführer hat das in seinem Bericht vermerkt. Die sieben Aussagen sind am Text nachgelesen und stehen alle sieben darin, einschließlich der Berichtigung von „Nachzuführen ist nichts" (`README.md:304-308`). Wer sie maschinell will, braucht einen Schritt, der eine Probe an der Datei vorsieht — das ist eine Planlücke und kein Befund an diesem Bau.

---

## Anmerkung des Abgleichs, 260813-1345

Kein Befund dieser Durchsicht ist widerlegt worden; die Belege sind einzeln am Baum nachgelesen.
Befund 1 ist in Turn 2 behoben (`ed0388e`), die Befunde 2 bis 4 stehen unverändert und tragen
je eine Abgleichsnotiz an ihrem Datensatz. Die vier geprüften Abweichungen vom Planwortlaut
halten alle vier.

**Eine Zahl im Abschnitt „Was sonst geprüft und in Ordnung ist" stimmt nicht.** Dort steht:
„`fokus()` bleibt als Hülle für seine fünf übrigen Aufrufer, nachgezählt: `:1168`, `:1713`,
`:3454`, `:4822`, `:5286`." Es sind sechs. Der sechste steht bei `anwendung.rs:1084` und heisst
`selbst.fokus()` statt `self.fokus()`; er entgeht damit dem Muster, das die anderen fünf
gefunden hat. Dieselbe Zahl steht im Plan (A2) und im Doc-Kommentar bei `anwendung.rs:4163-4165`,
und derselbe Fehler ist der Runde 7 schon einmal an zwei Zählproben unterlaufen
(`circles/260813-0100-…/issues/260813-0540_c_zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs.md`).
Abgelegt als `issues/260813-1345_o_die-aufruferzahl-an-fokus-steht-auf-fuenf-und-der-baum-traegt-sechs.md`.
Die Aussage, um die es der Durchsicht sachlich ging — dass `lage` das Schlüsselfenster genau
einmal erhebt —, ist davon unberührt und bestätigt.

**Der Querschnitt ist grösser, als diese Durchsicht ihn fasst.** Sie führt sechs Prosastellen auf
eine Ursache zurück und schlägt als Abhilfe vor, ein Schritt möge die Dateien in seine Liste
nehmen, die die geänderte Zahl nennen. Zwei weitere Stellen stehen in `zulaessigkeit.rs` selbst,
also in der Datei, die A1 als einzige nennt (`:299` „die drei abweisenden Viertel", `:459` „die
Regel drei Bestandteile"), und eine dritte im Grenzbereich bei `anwendung.rs:2690-2693`. Die
Dateiliste ist damit die eine Hälfte der Abhilfe; die andere ist die Suche nach der Zahl
innerhalb der geänderten Datei. Abgelegt als
`issues/260813-1345_o_zwei-prosastellen-in-zulaessigkeit-rs-stehen-in-der-dateiliste-von-a1-und-sind-trotzdem-nicht-nachgezogen.md`.

**Die Anmerkung zu C4 ist aufgenommen worden.** „C4 ist ohne Probe abgenommen" gilt, und zwei
weitere Kriterien teilen die Lage: C2.8 und C2.10 tragen ebenfalls **(Probe)** und haben keine.
Neun der 59 zusammen, abgelegt als
`issues/260813-1345_o_neun-abnahmekriterien-tragen-probe-und-haben-keine.md`.

**Die Anmerkung zu den Zeitstempeln aus der Zukunft trifft weiter zu.** `date` meldet beim
Abgleich `260813-1345`; `issues/260813-1420_o_…` trägt weiterhin einen späteren Stempel als die
Uhr. Der gemeinsame Speicher führt den Fall schon
(`circles/260812-1000-…/issues/260812-1805_o_sechs-sitzungsprotokolle-tragen-einen-zeitstempel-aus-der-zukunft.md`,
offen); ein zweiter Datensatz dafür entsteht hier nicht.
