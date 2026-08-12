Die Identitätssuche zählt jede Identität doppelt und bricht den Release-Lauf immer ab

---

`cargo xtask release` bricht vor dem ersten Übersetzungslauf ab:

```
xtask: Mehrere Developer-ID-Identitaeten gefunden, die Wahl waere nicht eindeutig:

       "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
       "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
```

Zweimal derselbe Name, und im Schlüsselbund liegt genau **eine** solche Identität.

---

**Die Ursache steht in der Ausgabe von `security`.** `sign::auflisten` ruft
`security find-identity -p codesigning`, also ohne `-v`, und dieser Aufruf gibt **zwei
Abschnitte** aus:

```
Policy: Code Signing
  Matching identities
  1) 4B30A8F7… "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
  2) B2CA1443… "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     2 identities found

  Valid identities only
  1) 4B30A8F7… "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
  2) B2CA1443… "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     2 valid identities found
```

Eine gültige Identität steht in **beiden** Abschnitten. `gueltige_namen` liest über die ganze
Ausgabe und findet sie deshalb zweimal; `developer_id_namen` filtert danach auf den
Namensanfang und behält beide Vorkommen. `bestimmen_fuer_release` trifft damit den Zweig
`mehrere` und bricht ab.

**Der Weg kann nie funktioniert haben.** Bei einer gültigen Developer-ID zählt die Suche zwei,
bei zwei gültigen vier. Der Zweig `[einzige]` ist nur erreichbar, wenn eine Developer-ID
existiert und **nicht** gültig ist — also genau dann, wenn sie nicht signieren kann. Das ist
die Umkehrung dessen, was der Zweig meint. Am 260812 zum ersten Mal ausgelöst, weil dieses
Projekt bis dahin gar keine Developer-ID besaß.

**`bestimmen` für den Entwicklungsbau ist nicht betroffen**, jedenfalls nicht sichtbar: es
sucht nach dem festen Namen `KRK Entwicklung` beziehungsweise nimmt die einzige gültige
Identität, und dort trägt die Doppelzählung bisher nichts aus. Wer die Behebung baut, prüft
beide Suchen, nicht nur die für den Release.

**Die Behebung liegt an einer Stelle:** die Ausgabe von `security find-identity` ist vor dem
Auswerten auf **einen** Abschnitt zu beschränken. Welcher, ist eine echte Wahl und keine
Formsache. Der Kommentar an `bestimmen_fuer_release` begründet ausdrücklich, warum ohne `-v`
gesucht wird: „wer eine Developer-ID angelegt hat, hat sich fuer sie entschieden, und die
Suche hat sie nicht an der Vertrauensbewertung auszusortieren." Diese Absicht bleibt erhalten,
wenn allein der Abschnitt `Matching identities` gelesen wird — er enthält auch die ungültigen.
Wer stattdessen `-v` nähme, änderte die Absicht.

**Eine Probe fehlt und gehört dazu.** Die Fehlklassifikation ist an einer festen Zeichenkette
prüfbar, ohne Schlüsselbund: die Ausgabe oben als Eingabe, Erwartung genau ein Developer-ID-Name.

---

**Umgehung, die heute trägt:** `KRK_SIGN_IDENTITY` setzen. Die Umgebungsvariable wird als
erstes geprüft und überspringt die Suche vollständig. Am 260812 so gefahren:

```sh
KRK_SIGN_IDENTITY="B2CA1443DCFE16C610D45DA616D744D762270145" \
  KRK_NOTARY_PROFILE=krk-notar cargo xtask release
```

Ein SHA-1-Abdruck ist dabei die bessere Wahl als der Name: `codesign --sign` nimmt beides, und
der Name ist mehrdeutig, sobald zwei Zertifikate desselben Teams im Schlüsselbund liegen.

Herkunft: gemeinsamer Speicher. Betrifft `xtask` und den Auslieferungsweg des ganzen Projekts.

---
Resolved: `xtask/src/sign.rs` — `auflisten` beschränkt die Ausgabe von
`security find-identity -p codesigning` jetzt auf den Abschnitt `Matching identities`
(neu: `abschnitt_der_treffer`, dazu die zwei Überschriften als Konstanten). Die
Beschränkung sitzt in `auflisten` und nicht bei den Aufrufern, damit keiner von beiden sie
vergessen kann; `bestimmen` liest dieselbe Liste und ist damit mitversorgt, ohne dass sich
sein Verhalten ändert — es prüft nur auf Enthaltensein und war von der Doppelzählung nie
betroffen. Zu `-v` wurde nicht gegriffen: der erste Abschnitt führt auch die ungültigen
Identitäten, und damit bleibt die Absicht erhalten, eine angelegte Identität nicht an der
Vertrauensbewertung auszusortieren.

Fünf Proben dazu, alle ohne Schlüsselbund: die volle zweiabschnittige Ausgabe des
Referenzgeräts vom 260813 als feste Zeichenkette, daran die Doppelzählung über die ganze
Ausgabe als Befund festgehalten (vier Einträge bei zwei Identitäten), die eindeutige
Developer-ID nach der Beschränkung, das Überleben einer nur im ersten Abschnitt geführten
ungültigen Identität, und die Unverändertheit einer Ausgabe ohne zweiten Abschnitt.

Gegenprobe am Gerät: `cargo xtask release` **ohne** `KRK_SIGN_IDENTITY` kommt an der
Signaturwahl vorbei und läuft bis zum Signieren durch (Lauf am 260813-0010 dort
abgebrochen, das beglaubigte Bündel vorher gesichert und danach zurückgespielt). Die
Umgehung über `KRK_SIGN_IDENTITY` wird nicht mehr gebraucht und ist deshalb auch nicht in
das neue Ziel `make release` eingegangen.
