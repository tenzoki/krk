# Veröffentlichen als achte Station der Auslieferungskette

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** `shared/planning/260821-1115_*_spec-artefakt-und-release.md` — vor diesem Circle entstanden und deshalb im gemeinsamen Speicher, mit dem Plan dazu am selben Ort: `shared/planning/260821-1221_*_plan-artefakt-und-release.md`.
**Active session history:** `shared/history/260820-2200-orchestrator-session.md`

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

Diese Runde ist gebaut, bevor sie einen Circle hatte, und der Datensatz wird deshalb nachgetragen. Der Abgleich `shared/history/260821-1532-reconciliation.md` hat den fehlenden Träger festgestellt: ein Spec mit 40 Abnahmekriterien, ein Plan mit elf Schritten, vier Durchsichten und sieben Codecommits, alles ohne aktiven Circle. Die Ablage von Spec, Plan, Durchsichten und Protokollen im gemeinsamen Speicher folgt der Herkunftsregel und bleibt, wie sie ist; dieser Circle zitiert sie an ihrem Ort und kopiert nichts.

**Die Auslieferungskette endete auf der Platte.** `cargo xtask release` fuhr sieben Stationen, von der Tagprüfung über Übersetzen, `lipo`, Montage und Signieren bis zur Beglaubigung bei Apple, und hinterließ ein fertiges `KRK.app` unter `target/`. Ein Schritt, der das Bündel weitergebbar macht und irgendwo hinstellt, wo ein Nutzer es holen kann, fehlte. Der Weg von einem beglaubigten Bündel zu einer Fassung auf einem fremden Gerät war Handarbeit ohne Anleitung.

**Zwei äußere Voraussetzungen fehlen auf dem Gerät, und beide sind Nutzerarbeit.** `gh`, das GitHub-Kommandozeilenwerkzeug, ist nicht installiert; geprüft am 260821 mit `command -v gh` und am 260821-1644 erneut. Von den 14 lokalen Git-Tags steht auf der Gegenseite genau einer, `v0.1.0`, sodass 13 Tags von `v0.2.0` bis `v0.5.5` nachzuschieben sind. Der Nachschub ist ein einmaliger Handgriff des Nutzers, `git push origin --tags`, und ausdrücklich nicht Aufgabe des neuen Befehls: ein Kommando, das dreizehn Referenzen auf einmal schiebt, gäbe die enge Begrenzung auf, unter der das Bauwerkzeug überhaupt schieben darf. Aus diesen zwei Lücken folgt, dass 15 der 40 Abnahmekriterien ohne den Nutzer nicht abzunehmen sind.

**Der Anlass der Runde ist ein Datenverlust und keine Bequemlichkeit.** Am 17.08. um 19:13:48 hat KRK seinen Ablageordner `~/Library/Application Support/KRK/` neu angelegt, drei Minuten und 46 Sekunden nach dem Auslieferungslauf zu `v0.5.1`. Der Ordner war vorher fort, und mit ihm die Lesezeichen, die gesicherte Sitzung, die abweichende Tastenbelegung und die zwei Notizzettel. Die Untersuchung `shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md` hält an dreizehn Beweisstücken fest, dass kein Schreiber im Baum die Datei anfasst und ein Überkopieren der App sie nicht leeren kann; entfernt hat den Ordner ein Löschwerkzeug, das die Stützdateien der App mitnahm. Die Betriebsregel daraus lautet, die neue Fassung über die alte zu kopieren und die alte nicht vorher zu löschen. Sie stand nirgends dort, wo der Nutzer sie beim Installieren liest, und genau diese Stelle entsteht mit dem festen Text der Releaseseite.

**Acht Nutzerantworten aus zwei Klärungsrunden haben den Umfang gesetzt**, protokolliert in `shared/history/260821-1115-shaper-artefakt-und-release.md`: Zip als Hülle, mit einem zweiten `ditto -c -k --keepParent` nach dem Anheften des Tickets; eng begrenztes Schieben von aktuellem Zweig und genau einem Tag; `gh release create` als Werkzeug ohne eigene Netzbibliothek; ein eigener Unterbefehl, der zugleich als achte Station läuft; Dateiname `KRK-<zahl>.zip`; fester Text aus dem Werkzeug statt einer erzeugten Änderungsliste; sofort öffentlich statt Entwurf; die fehlenden Tags einmalig von Hand. Der zweite `ditto`-Lauf ist dabei keine Doppelung, sondern eine Korrektur: der Beglaubigungsablauf packt das Bündel vor dem Ticket und löscht sein Zip wieder, sodass die vorhandene Datei den Nachweis nicht tragen konnte.

**Der Schnitt der Runde ist eine Nutzerentscheidung gegen drei größere Wege.** Draußen blieben ein Aktualisierungshinweis in der App, ein Homebrew-Cask und der volle Selbstaustausch des Bündels. Der dritte ist der wichtigste Verzicht: er verlangte Netzcode, eine Prüfung der geladenen Fassung, das Ersetzen einer laufenden Anwendung und einen Rückweg für ein mittendrin gescheitertes Ersetzen. Genau in dieser Gegend liegt der Vorfall vom 17.08., und ein selbstgebauter Austauschweg brächte dieselbe Klasse von Verlust näher heran, statt sie zu entfernen. Keiner der drei ist dauerhaft ausgeschlossen; verworfen sind sie für diese Runde. Aus demselben Umfangsschnitt folgt, dass die Runde keine Zeile Anwendungscode ändert: die zehn Zeitzusagen aus C8 der Runde 1 sind unberührt, und eine elfte Zahl entsteht nicht.

**Zwei Entscheidungen und mehrere Defekte binden die Runde weiter.** Ob der Veröffentlichungsbefehl eine eigene Hülle wie `certify-only.sh` und ein Makefile-Ziel bekommt, ist offen (`shared/decisions/260821-1115_*_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-wie-certify-only-sh.md`); der Baum fährt die vorläufige, schmalste Fassung, und der Abgleich hat festgehalten, dass ein Baum, der einer selbsterklärt vorläufigen Empfehlung folgt, keine Antwort ergibt. Ob der Aufruf fremder Werkzeuge über den Suchpfad zur Regel wird, ist ebenfalls offen (`shared/decisions/260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`), und die Voraussetzung dieser Frage ist beim Nachmessen gefallen: `iconutil` und `rustup` stehen seit dem 260811 und dem 260806 ohne vollen Pfad da, `gh` ist also nicht die erste Ausnahme (`shared/issues/260821-1532_*_zwei-fremde-werkzeuge-werden-seit-langem-ueber-den-suchpfad-gerufen-und-drei-stellen-nennen-gh-als-die-erste-ausnahme.md`). Der Defekt zum gemeinsamen Bauort von `bundle` und `release` (`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`) ist gemildert und nicht behoben; die Runde behauptet keinen Abschluss. Das Abnahmekriterium C6.3 verbietet eine Zeichenfolge, die es selbst enthält, und ist in dieser Form unerfüllbar (`shared/issues/260821-1221_*_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md`); der Plan begrenzt die Zusage auf den Quellbaum, am Wortlaut des Kriteriums ist nichts geändert.

**Der Marker am Spec bleibt `_o_`, und das ist gemessen und nicht angenommen.** Nach der Lesart der belegten Bauarbeit stünde er auf `_c_`, denn elf von elf Planschritten sind am Baum belegt und jeder Befund der vier Durchsichten ist behoben oder als eigener Datensatz abgelegt. Nach der Lesart der Abnahmekriterien steht er nicht dort, weil 15 der 40 auf den Nutzer warten. Welche Lesart gilt, ist die offene Frage `shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`, und eine Umbenennung entschiede sie durch vollendete Tatsache.

**Baumstand bei der Anlage dieses Circles:** `ca84a59`. Der Stand der Runde selbst reicht bis `4e810f9`; geschoben ist zu diesem Zeitpunkt nichts, `origin/main` steht auf `01d2365`. Die achte Station steht als `xtask/src/veroeffentlichung.rs` im Baum, `cargo test --workspace` ist grün mit 155 Proben in `xtask`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` geben null zurück.

## Dependencies

- `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` — Runde 8 hat die Tagpflicht eingeführt, auf der die achte Station aufsetzt. Seit ihr bricht `cargo xtask release` ab, solange HEAD keinen Tag `v<version>` trägt, der zur `Cargo.toml` passt. Der neue Schritt schiebt genau diesen einen Tag und verlässt sich darauf, dass die Station 1 desselben Laufs seine Existenz schon geprüft hat. Nicht geändert, nur vorausgesetzt.
- `260802-0842-krk-mac-dateimanager-editor-git` — Runde 1 hat `cargo xtask release` selbst gebaut, Planschritt S23 in `d577295`. Die achte Station hängt sich an diese Kette an und legt keine zweite daneben.

Die Beglaubigung als siebte Station in ihrer heutigen Gestalt stammt aus `f5300f4` vom 260820 und hat keinen eigenen Circle. Sie wird deshalb als Commit zitiert und nicht als Abhängigkeit geführt. Sie liefert das Muster, dem die achte Station folgt: ein Schritt mit zwei Rufern, damit ein Lauf, der weit gekommen und am Netz gescheitert ist, dort wieder ansetzen kann, ohne beide Ziele erneut zu übersetzen.

## Turn log

- Turn 3 (Sitzung 260820-2200): Commits `77b84bb`..`4e810f9`; Coherence-Verdikt `review-needed`;
  Sitzungsprotokoll `shared/history/260820-2200-orchestrator-session.md`.
  **Der Circle bestand während dieses Turns nicht.** Er ist am 260821-1644 nachgetragen worden,
  nachdem der Abgleich `shared/history/260821-1532-reconciliation.md` den fehlenden Träger
  festgestellt hatte. Die Zeile hält den Turn fest, wie er gefahren ist, und behauptet nicht,
  er sei unter diesem Datensatz gelaufen.
