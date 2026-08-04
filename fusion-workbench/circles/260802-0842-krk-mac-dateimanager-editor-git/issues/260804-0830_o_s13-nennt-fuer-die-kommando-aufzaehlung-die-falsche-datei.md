Die Dateiliste von Schritt 13 nennt für die Aufzählung `Kommando` die falsche Datei

---

S13 führt in seiner Dateiliste den Eintrag

> `crates/krk-core/src/tasten/mod.rs` (erweitert: die neuen Kommandos aus C2 kommen in die Aufzählung `Kommando`, die S7 angelegt hat)

Die Aufzählung steht dort nicht. Sie steht seit der Umsetzung von Schritt 11 in `crates/krk-core/src/tasten/belegung.rs`, zusammen mit ihrer Kennungstabelle `Kommando::KENNUNGEN` und dem Nachschlag `Kommando::aus_kennung`; nachgesehen am 260804-0830. `tasten/mod.rs` bindet die drei Module aus S11 ein und hält die verdrahtete Tabelle aus S7 nicht mehr, die S11 abgelöst hat.

---

## Warum das mehr ist als ein falscher Dateiname

Die Regel im Kopf von `## Implementierungsschritte` verlangt von jeder Dateiliste, die einbindende und die erweiterte Datei mitzunennen, damit der Umsetzende sie nicht selbst suchen muss. Eine Liste, die auf eine Datei zeigt, in der das Genannte nicht steht, kostet genau diese Suche wieder und lädt daneben dazu ein, die Aufzählung ein zweites Mal in `mod.rs` anzulegen. Zwei Aufzählungen `Kommando` wären zwei Wahrheiten darüber, welche Kommandos KRK kennt, und die erste Abweichung zwischen ihnen fände keine Prüfung.

## Umfang

Betroffen ist S13. Die Durchsicht der übrigen Schritte auf dieselbe Verwechslung steht aus; S15, S16, S17, S18, S19 und S20 nennen `tasten/mod.rs` nicht, `crates/krk-core/src/tasten/belegung.rs` nennen S20 und dieser Datensatz.

## Dringlichkeit

Bindet S13, den nächsten Schritt nach S12, der die Aufzählung erweitert. Zu klären vor seiner Umsetzung; die Auflösung ist ein Wort in der Dateiliste und keine Entwurfsfrage.

---

Herkunft: gefunden am 260804-0830 beim Einarbeiten der Fähigkeit C10 in S13, weil die beiden neuen Kommandos der Zwischenablage in dieselbe Aufzählung wachsen.
