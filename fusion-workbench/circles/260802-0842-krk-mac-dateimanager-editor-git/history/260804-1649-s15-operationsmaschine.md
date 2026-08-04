# S15: Operationsmaschine (C4, Kern)

---
**Agent:** coder
**Status:** Complete
**Datum:** 260804-1649
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 15.`
**Bindende Datensätze:** `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_umbenennen-im-stapel-umfang.md`

---

## Was entstanden ist

Der Kern von C4: ein Auftrag beschreibt Quellen, Ziel, Art und Konfliktregel, läuft auf einem eigenen Arbeitsfaden und meldet Fortschritt, übersprungene Einträge und Konfliktfragen über einen Kanal an den Hauptfaden. Abgebrochen wird über ein `AtomicBool`, das sowohl zwischen zwei Einträgen als auch im Statusrückruf von `copyfile(3)` gelesen wird.

```
Auftrag ──> starten ──> Arbeitsfaden ──> ausfuehren ──> kopieren / verschieben / loeschen
                │                            │                      │
  Lauf::abbrechen ─AtomicBool────────────────┘         verzeichnis::lesen (S2)
  Lauf::meldungen <─Kanal─ Steuerung                   sys::datei_kopieren (copyfile)
  Konfliktentscheid ─Kanal─────>                       sys::im_datentraeger_verschieben
                                                       Papierkorb (Schnittstelle)
```

### Neue Dateien

| Datei | Was drinsteht |
|---|---|
| `crates/krk-core/src/operation/mod.rs` | Der Ablauf: `starten`, `ausfuehren`, die Konfliktklärung `ziel_klaeren`, die Grenzfälle (Quelle gleich Ziel, Ziel in der Quelle), die Übersetzung eines Systemfehlers in einen Grund im Klartext |
| `crates/krk-core/src/operation/auftrag.rs` | `Auftrag`, `Art`, `Konfliktregel` |
| `crates/krk-core/src/operation/fortschritt.rs` | `Lauf`, `Steuerung`, `Meldung`, `Bericht`, `Uebersprungen`, `Konfliktantwort`, `Konfliktentscheid`, der Meldetakt |
| `crates/krk-core/src/operation/kopieren.rs` | Datei, Ordner, Verknüpfung; der Abstieg über den Leser aus S2 |
| `crates/krk-core/src/operation/verschieben.rs` | `rename(2)`, Verschmelzen gleichnamiger Ordner, Rückfall auf Kopieren mit Löschen bei `EXDEV` |
| `crates/krk-core/src/operation/loeschen.rs` | Die Schnittstelle `Papierkorb`, `OhnePapierkorb`, rekursives endgültiges Löschen, `baum_entfernen` |
| `crates/krk-core/src/operation/anlegen.rs` | Ordner und leere Datei anlegen |
| `crates/krk-core/src/operation/umbenennen.rs` | Namensprüfung, freier Name, einzelnes Umbenennen |
| `crates/krk-core/tests/operation.rs` | 22 Prüfungen, darunter die vier Abnahmepunkte |
| `crates/krk-ui/src/appkit/papierkorb.rs` | Die Hülle um `NSFileManager.trashItemAtURL:` |

### Geänderte Dateien

- `crates/krk-core/src/verzeichnis/sys.rs`: die Bindungen an `copyfile(3)` und `renamex_np(2)` sind hierhergekommen. Modulkopf umgeschrieben, weil das Modul jetzt drei Systemaufrufe hält und nicht mehr nur den einen des Lesers.
- `crates/krk-core/src/lib.rs`: `pub mod operation;` und der Modulkopf, der die drei Systemaufrufe nennt.
- `crates/krk-core/src/verzeichnis/mod.rs`: derselbe Nachzug im Modulkopf.
- `crates/krk-ui/src/appkit/mod.rs`: `mod papierkorb;` und der Modulkopf, jetzt vierzehn Module.

`Cargo.lock` und die `Cargo.toml`-Dateien sind unverändert: der Schritt hat keine neue Abhängigkeit gebraucht.

## Die vier Abnahmepunkte, mit den gemessenen Zahlen

Gemessen am 260804-1649 auf dem Referenzgerät, alle Prüfdaten unter `/tmp` auf demselben APFS-Datenträger.

| Abnahmepunkt | Zusage | Gemessen |
|---|---|---|
| Baum mit 500 Einträgen, verschachtelte Ordner eingeschlossen | vollständig | 501 gemeldete Einträge (500 im Baum plus der Wurzelordner), 500 am Ziel nachgezählt, nichts übersprungen |
| Verschieben einer 200-MB-Datei im selben Datenträger | unter 50 ms | **200 µs**; eine kleine Datei brauchte im selben Lauf 233 µs, die Größe spielt also keine Rolle |
| Abbruch mitten in einer 500-MB-Datei | binnen 100 ms zurück, mit der übertragenen Zahl | **2,0 ms** bis zur Abschlussmeldung, gemeldet **33.554.432 von 524.288.000 Bytes**, die halbe Datei am Ziel weggeräumt |
| Ein Eintrag ohne Leserecht | übersprungen, mit Grund, die übrigen laufen durch | übersprungen mit dem Grund "keine Rechte", die fünf übrigen Dateien und der Ordner angekommen |

Fünfmal hintereinander gelaufen, jedes Mal grün, Laufzeit der Prüfdatei zwischen 0,49 s und 0,57 s. Die beiden Zeitmessungen schließen sich über einen Mutex gegenseitig aus, damit sie sich nicht selbst messen.

### Die vier üblichen Kommandos

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, **288 bestanden und eine weiterhin übersprungen**, zusammen 289 (vorher 248) |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0, keine Warnung |
| `cargo test -p krk-core --test operation` | 0, 22 Prüfungen |

## Die vier Festlegungen des Auftrags

**Eine einzige Ausnahme von `deny(unsafe_code)`.** `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-core/src` nennt unverändert allein `crates/krk-core/src/verzeichnis/sys.rs`. `copyfile(3)` und `renamex_np(2)` liegen dort neben `getattrlistbulk(2)`.

**`krk-core` ist AppKit-frei.** Im Code gibt es keinen Treffer für `AppKit` oder `objc2`, und die `Cargo.toml` des Kerns nennt weiterhin allein `serde` und `toml`. Das wörtliche Abnahmekriterium `grep -rn 'AppKit\|objc2' crates/krk-core/src` liefert trotzdem Treffer, und zwar schon vor diesem Schritt: es sucht in Prosa mit, und die Doku des Kerns erklärt an achtzehn Stellen in sieben Dateien, was der Kern gerade nicht tut. Defekt `issues/260804-1649_o_das-appkit-abnahmekriterium-von-s15-ist-so-nicht-erfuellbar.md`.

**Der Leser aus S2 hat gereicht.** Rekursives Kopieren, Verschmelzen und endgültiges Löschen laufen alle über `verzeichnis::lesen`, `Typ` aus `eintrag.rs` entscheidet über den Abstieg. Ein zweiter Verzeichnisdurchlauf ist an keiner Stelle nötig geworden. `COPYFILE_RECURSIVE` bleibt ungenutzt, und `fs::remove_dir_all` ebenso; beide wären der zweite Durchlauf gewesen, und keiner von beiden könnte abbrechen, je Eintrag melden oder eine Einzelposition überspringen.

**Gescheiterte Einzelpositionen brechen den Stapel nicht ab.** Sie landen in `Bericht::uebersprungen` mit Pfad und Grund und gehen zugleich als eigene Meldung über den Kanal. Zwei Dinge beenden einen Vorgang vorzeitig, sonst nichts: der Abbruchbefehl und die Antwort "abbrechen" auf eine Konfliktfrage.

## Vier Entwurfsentscheidungen, die der Plan offengelassen hatte

**`renamex_np(2)` statt `rename(2)`.** Der Plan nennt `rename(2)`. Gebunden ist die Variante `renamex_np(2)`, weil allein sie `RENAME_EXCL` kennt: damit scheitert ein Verschieben mit `EEXIST`, statt ein vorhandenes Ziel zu überschreiben. Mit dem Kennzeichen 0 ist der Aufruf das gewöhnliche `rename(2)`. Ein Blick auf das Ziel vorweg wäre ein Zeitfenster, in dem eine andere Anwendung die Datei anlegt; `std::fs::rename` kann die Prüfung nicht ausdrücken. Deshalb verdient dieser eine Fremdaufruf sein `unsafe`, während Löschen und Anlegen bei `std::fs` bleiben.

**Das Ziel steht in der Art und nicht daneben.** `Art::Kopieren { ziel }` und `Art::Verschieben { ziel }` tragen ihren Zielordner, `Art::InDenPapierkorb` und `Art::EndgueltigLoeschen` haben keinen. Ein flaches Feld `ziel` hätte bei zwei der vier Arten keinen Wert, den ein Aufrufer sinnvoll füllen könnte.

**Der Fortschritt ist getaktet, die übrigen Meldungen nicht.** Höchstens eine Fortschrittsmeldung alle 8 ms, die erste sofort. Eine übersprungene Position, eine Konfliktfrage und der Abschluss gehen immer los. Ohne den Takt hinge die Zahl der Meldungen an der Zahl der Einträge: eine Kopie von 100.000 Dateien schöbe 100.000 Meldungen in einen unbegrenzten Kanal. Das berührt `issues/260803-2007_o_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`, schließt ihn aber nicht: der Defekt fragt nach dem Mechanismus in der Oberfläche, und die Bündelung auf höchstens eine Meldung je Bild bleibt S16s Aufgabe.

**Die Rückfrage bei einem Konflikt wartet auf eine Antwort.** `Meldung::Konflikt` trägt einen Rückkanal; der Arbeitsfaden blockiert daran, bis die Oberfläche antwortet. Wird der Kanal fallen gelassen, gilt das als Abbruch: lieber nichts tun als ungefragt überschreiben. Die Wahl "für alle weiteren übernehmen" ändert die Konfliktregel des laufenden Vorgangs. Ein von Hand getippter Name lässt sich dabei nicht auf weitere Einträge übertragen; "für alle weiteren" heißt dort, dass die Maschine jedes Mal selbst einen freien Namen sucht.

## Was S16 vorfindet

- `krk_core::operation::starten(auftrag, papierkorb)` liefert einen `Lauf` mit `meldungen()`, `abbrechen()` und `warten()`.
- `krk_ui::appkit::papierkorb::Systempapierkorb` ist die Implementierung der Schnittstelle. Sie trägt heute `#[expect(dead_code)]` mit dem Verweis auf S16; sobald S16 sie einhängt, verlangt das Attribut selbst seine Entfernung.
- Die vier Blätter aus S16 antworten auf `Meldung::Konflikt` mit `Konfliktentscheid::einmal(...)` oder `Konfliktentscheid::fuer_alle(...)`.
- Die 150-ms-Regel aus `### Frage 6` gehört in S16; der Kern kennt keine Zeitschwelle für ein Blatt.
- S17 findet `operation::umbenennen`, `operation::name_pruefen` und `operation::freier_name` vor. Ein zweiter Umbenennungsweg neben dem der Operationsmaschine entsteht damit nicht.

## Die großen Prüfdateien

Beide entstehen unter `/tmp`, auf demselben APFS-Datenträger wie ihr Ziel, und werden vom `Drop` des Prüfordners wieder abgeräumt. Nach dem Lauf ist unter `/tmp` kein Rest von `krk-operation-*` mehr zu finden; nachgeprüft.

Die **200-MB-Datei** ist dünnbesetzt (`set_len`), weil `rename(2)` keinen Inhalt anfasst: ein Loch ist dafür so gut wie Daten, und der Plattenplatz bleibt frei.

Die **500-MB-Datei** trägt echte Bytes, in Blöcken zu 4 MiB geschrieben und mit `sync_all` festgeschrieben. Eine dünnbesetzte Datei taugt hier nicht, und der Modulkopf des Prüfordner-Erzeugers in `krk-bench` sagt das seit S3 selbst: "Wer diese Ordner später für eine Messung von Kopiervorgängen (L8) benutzen will, muss das wissen: dafür taugen sie nicht." Der Erzeuger aus S3 ist deshalb an keiner Stelle benutzt worden, auch nicht für den Baum mit 500 Einträgen: der legt einen **flachen** Ordner an, und geprüft werden sollte gerade die Verschachtelung.

## Angelegte Defekte

| Datei | Worum es geht |
|---|---|
| `issues/260804-1649_o_das-appkit-abnahmekriterium-von-s15-ist-so-nicht-erfuellbar.md` | Das `grep`-Kriterium sucht in Prosa mit und meldete schon vor S15 achtzehn Treffer |
| `issues/260804-1649_o_innerhalb-eines-apfs-datentraegers-gibt-es-kein-mitten-in-einer-datei.md` | `COPYFILE_CLONE` macht eine 500-MB-Kopie zu 0,42 ms; Fortschritt und Abbruch sind dort gegenstandslos |
| `issues/260804-1649_o_die-gemeldete-eintragszahl-bedeutet-beim-verschieben-etwas-anderes-als-beim-kopieren.md` | 501 beim Kopieren, 1 beim Verschieben desselben Ordners |

## Was nicht behoben wurde

Keiner der 26 offenen Defekte. `issues/260803-2025_o_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr.md` betrifft `verzeichnis/leser.rs` und hat durch diesen Schritt **keinen** neuen Aufrufer bekommen: die Operationsmaschine benutzt die einfache Form `verzeichnis::lesen` und nicht den gestückelten `Lesevorgang` mit seiner Generationsnummer. Ein Ordner, in den kopiert wird, muss vollständig gelesen sein, bevor der erste Eintrag angefasst wird; die Stückelung hat dort keinen Gegenwert. `Meldung::generation` und `Lauf::generation` stehen also weiter ohne Aufrufer da.

## Was offen bleibt

- Ein **abgebrochener Baum** hinterlässt am Ziel, was bis dahin angekommen ist. Nur die eine angefangene Datei wird weggeräumt. C4 sagt dazu nichts, und die Vorbilder halten es genauso; falls der Nutzer es anders will, ist das eine Frage an ihn und kein Defekt.
- Die **Verschmelzung zweier gleichnamiger Ordner** gilt hier ausdrücklich nicht als Konflikt. C4 nennt den Fall nicht. Sollte der Nutzer eine Rückfrage auch dafür wollen, ist es eine Zeile in `ziel_klaeren`.
