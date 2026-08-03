Der Bündelbau findet die Signaturidentität nur unter einem festen Namen

---

`xtask/src/sign.rs` sucht die Signaturidentität in dieser Reihenfolge:

1. die Umgebungsvariable `KRK_SIGN_IDENTITY`, falls gesetzt
2. eine Identität mit dem Namen `KRK Entwicklung`
3. sonst Abbruch mit Anleitung

Auf dem Referenzgerät liegt seit dem 260802-2040 eine gültige Apple-Identität im Anmeldeschlüsselbund: `Apple Development: Kai Stalmann (FJ8U4B3QAC)`, Team-Kennung `QYMPYB7MWM`, gültig bis 2027-08-02. Sie heißt nicht `KRK Entwicklung`, also bricht `cargo xtask bundle` ohne die Umgebungsvariable ab, obwohl genau eine brauchbare Identität vorliegt.

Das erzwingt bei jedem Bau ein Präfix:

```sh
KRK_SIGN_IDENTITY="Apple Development: Kai Stalmann (FJ8U4B3QAC)" cargo xtask bundle
```

Vergisst es jemand, meldet das Werkzeug „Keine Signaturidentität gefunden" — eine Aussage, die dann schlicht falsch ist.

---

**Was zu tun ist.** Eine vierte Stufe zwischen 2 und 3: **gibt es genau eine gültige Identität, nimm sie und schreib hin, welche.** Bei null oder mehr als einer bleibt es beim Abbruch mit Anleitung, denn nur dort ist die Wahl mehrdeutig.

Der bisherige Aufbau bleibt sonst unberührt, und die Rangfolge ist weiterhin richtig: die ausdrückliche Angabe schlägt alles, dann die projekteigene Entwicklungsidentität, dann die eindeutige Lage. Erst wenn nichts davon greift, bricht der Bau ab.

`security find-identity -v -p codesigning` liefert die gültigen Identitäten. **Die Streng-Prüfung `-v` ist hier richtig und war es vorher nicht:** der `coder` hatte am 260802-1927 bewusst ohne `-v` gesucht, weil ein selbstsigniertes Zertifikat ohne Vertrauenseintrag von `-v` nicht gefunden wird. Für die automatische Wahl aus einer Menge ist die Streng-Prüfung dagegen die richtige: ausgewählt werden soll nur, was auch signieren kann. Stufe 2 sucht weiterhin ohne `-v`, damit die selbstsignierte Entwicklungsidentität aus dem README weiter funktioniert.

---

## Zweiter Punkt: die abgelaufene Kette gehört ins README

Beim Einrichten trat ein Fehler auf, den die Anleitung nicht kennt und den niemand von selbst löst.

`codesign` scheiterte mit:

```
Warning: unable to build chain to self-signed root for signer
  "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
errSecInternalComponent
```

Zertifikat und privater Schlüssel lagen längst im Anmeldeschlüsselbund, `security find-identity -p codesigning` fand die Identität. Nur `-v` meldete null gültige.

**Die Ursache:** im System-Schlüsselbund lag das Apple-Zwischenzertifikat in seiner **alten Fassung, abgelaufen am 2023-02-07**. Das Zertifikat des Nutzers ist von der neueren G3-Instanz ausgestellt (`issuer=CN=Apple Worldwide Developer Relations Certification Authority, OU=G3`), und die fehlte.

**Die Behebung**, am 260802-2045 durchgeführt und nachgeprüft:

```sh
curl -fsS -o AppleWWDRCAG3.cer https://www.apple.com/certificateauthority/AppleWWDRCAG3.cer
security import AppleWWDRCAG3.cer -k ~/Library/Keychains/login.keychain-db
```

Danach meldet `security find-identity -v -p codesigning` eine gültige Identität, und `codesign -dvv` zeigt die vollständige Kette bis zur Apple Root CA samt `TeamIdentifier=QYMPYB7MWM`.

Das abgelaufene alte Zwischenzertifikat im System-Schlüsselbund ist **nicht** entfernt worden: es liegt in einem Bereich, der erhöhte Rechte verlangt, und die Kette baut sich auch neben ihm richtig auf. Sollte der Fehler trotz vorhandenem G3 erneut auftreten, ist das Entfernen der nächste Versuch.

**Ins README gehört das**, weil es genau die Stelle ist, an der ein Einrichtender ohne diese Notiz stecken bleibt: die Fehlermeldung nennt das Zwischenzertifikat mit keinem Wort und deutet auf die eigene Identität, die in Ordnung ist.

**Aufgefallen bei:** der Einrichtung der Signaturidentität nach Abschluss von Schritt 5, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1014-orchestrator-session.md`.

---
Resolved: `xtask/src/sign.rs` bestimmt die Identität in drei Stufen: `KRK_SIGN_IDENTITY`, dann der Name "KRK Entwicklung" ohne `-v`, dann die genau eine gültige Identität aus `security find-identity -v -p codesigning`. Bei null oder mehr als einer bricht der Bau mit einer Anleitung ab. `README.md` beschreibt die drei Stufen und den Fall der abgelaufenen Zertifikatskette. Nachgeprüft am 260803-1042: `cargo build --workspace`, `cargo test --workspace` (80 Tests, 0 Fehler), `cargo fmt --all --check` und `cargo clippy --workspace --all-targets` je Rückgabewert 0. Bericht: `history/260802-2253-signaturidentitaet-eindeutige-lage-und-zertifikatskette.md`. Der dort festgehaltene Nebenbefund zum Hilfetext ist als eigener Defekt `260803-1042_o_hilfetext-in-xtask-kennt-die-dritte-suchstufe-nicht.md` angelegt.
