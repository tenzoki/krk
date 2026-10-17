# Welche Form hat ein Ort im Dateifenster, und wie hält der Bau Fernaufrufe vom Hauptfaden fern?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md, 261016-1946-auf-welchem-weg-wird-eine-sftp-verbindung-zu-einem-ordner-im-dateifenster.md, 261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md

---

## Question

Mit Möglichkeit 2 der Machbarkeitsanalyse zeigt ein Dateifenster entweder einen lokalen Ordner oder einen Ordner auf einem Server, mit dem KRK über `/usr/bin/ssh -s … sftp` spricht. Zwei Festlegungen binden über das Arbeitspaket `261016-1945-sftp-ordner-im-dateifenster-wie-lokal` hinaus, weil jede spätere Arbeit an fernen Orten (Vorschau über einen begrenzten Lesevorgang, Editor, Duplizieren, Stapelumbenennen) auf ihnen aufsetzt. Erstens: welche Form hat der Wert, der heute als `PathBuf` sagt, welchen Ordner ein Tab zeigt, und wo ersetzt er ihn? Zweitens: woran hält der Bau fest, dass kein Aufruf, der auf das Netz wartet, den Hauptfaden erreicht? An dieser Bedingung hängen L1 und L9 aus C8, und ein ferner Ort ist der erste Gegenstand in KRK, dessen Antwortzeit allein die Gegenseite bestimmt.

## Options

1. **Ein Wert `Ort` mit zwei Fällen und ein Fadenpfand, das nur ein Arbeitsfaden besitzt.**
   - `krk_core::ort::Ort` ist `Lokal(PathBuf)` oder `Fern(FernerOrt)`. `FernerOrt` trägt einen geteilten Zugang zum Server (`Arc<Fernzugang>`) und einen absoluten Pfad auf dem Server; verglichen wird über die Serverkennung (Rechner, Port, Konto) und den Pfad. Ob ein Ort entfernt ist, entscheidet der Wert, nie ein Systemaufruf.
   - `Ort` ersetzt `PathBuf` dort, wo ein Tab auf seinen Ordner zeigt, wo ein Auftrag sein Ziel nennt und wo die Auffrischung Ordner vergleicht. Der lokale Auftrag behält seinen heutigen Typ: ein Vorgang, der einen fernen Ort berührt, ist ein eigener Auftragstyp neben ihm. Zippen und Entpacken lassen sich damit für einen fernen Ort gar nicht erst bauen.
   - Jede Anfrage an den Server verlangt ein `&Fernfaden`. Das Pfand entsteht allein in `krk_core::fern::faden`, das einen Faden startet und es dort hineinreicht, und es ist nicht `Send`. Eine Anfrage vom Hauptfaden aus übersetzt deshalb nicht.
   - Pros: Der Übersetzer hält die Zusage, ohne Aufzählung von Rufern und ohne Quelltextprobe. Der lokale Weg bleibt Zeile für Zeile, wie er ist. Die Entscheidung „lokal oder fern“ steht als vollständige Fallunterscheidung an jeder Naht.
   - Cons: Jeder Rufer von `Tabinhalt::ordner` muss sich einmal entscheiden; das ist die breite Naht, die die Analyse in Abschnitt 3b beziffert hat.
2. **Ein `trait Dateisystem` hinter jedem Pfad**, lokal und fern je eine Umsetzung, dynamischer Aufruf vor jedem Zugriff.
   - Pros: Ein Rufer sieht keinen Unterschied.
   - Cons: Jeder lokale Zugriff ginge durch eine neue Schicht, auch die gemessenen Wege von L2, L3, L6 und L10. Was ein entfernter Ort nicht kann, etwa einen Papierkorb oder ein Zip, wäre ein Laufzeitfehler statt eines Typs. Wer eine Anfrage vom Hauptfaden stellt, merkt es erst am hängenden Bündel.
3. **`Ort` wie in Möglichkeit 1, die Fadenzusage durch eine Quelltextprobe** über benannte Rümpfe des Anwendungsdelegierten, nach dem Muster von `der_verbindungsweg_stellt_auf_dem_hauptfaden_nur_den_einen_stat`.
   - Pros: Bekanntes Muster.
   - Cons: Die Probe sieht einen tiefer gerufenen Helfer nicht; ihr Doc-Kommentar sagt das selbst. Für eine Zusage, an der zwei der zehn Zeitzusagen hängen, ist eine Probe mit bekanntem blindem Fleck die schwächere Wahl, wenn der Typ dieselbe Frage vollständig beantwortet.

## Constraints

- Lokale Ordner und die zehn Zeitzusagen aus C8 bleiben unberührt (Directive des Pakets).
- `#![deny(unsafe_code)]` mit den zwei Ausnahmedateien; kein neues Paket mit `cc` oder einem Namen auf `-sys`.
- Vollständige Fallunterscheidungen ohne Auffangzweig.

## Recommendation

Möglichkeit 1. Sie macht beide Fragen am Wert und am Typ entscheidbar: „ist dieser Ort entfernt?“ ohne Systemaufruf, und „läuft diese Anfrage auf einem Arbeitsfaden?“ beim Übersetzen. Der Preis ist die breite Naht in den Tabs, und die bezahlt jede der drei Möglichkeiten; nur Möglichkeit 1 lässt dabei den lokalen Weg unverändert.

---
Implemented: crates/krk-core/src/ort.rs, crates/krk-core/src/fern/faden.rs (Modulkopf) — Moeglichkeit 1: Ort lokal oder fern, Pfand Fernfaden haelt Netzaufrufe vom Hauptfaden fern
