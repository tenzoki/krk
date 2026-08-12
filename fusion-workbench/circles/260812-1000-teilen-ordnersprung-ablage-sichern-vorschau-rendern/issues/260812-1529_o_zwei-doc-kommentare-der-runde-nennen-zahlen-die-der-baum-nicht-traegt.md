Zwei Doc-Kommentare der Runde nennen Zahlen, die der Baum nicht trägt

---

Dieses Projekt stützt sich in Doc-Kommentaren auf Zählungen — „der siebte
Abnehmer", „eine Klasse für zwei Flächen", „genau eine Hülle" —, und der Nutzen
davon steht und fällt damit, dass sie stimmen. Zwei aus dieser Runde stimmen
nicht.

**Erstens: „dritter Aufrufer" von `ordner_lesen`.**
`Anwendungsdelegierter::ordner_der_datei_zeigen`
(`crates/krk-ui/src/appkit/anwendung.rs:2357`) schreibt:

> **Der Sprung geht durch `DateifensterQuelle::ordner_lesen`** und wird deren
> dritter Aufrufer neben dem Aufstieg aus C2 der Runde 1 und dem Sprung aus der
> Zwischenablage aus C10.

Vor dieser Runde hatte `ordner_lesen` **neun** Aufrufer, danach zehn. Nachgezählt
am 260812:

```
crates/krk-ui/src/appkit/tabelle.rs:601, :1280, :1308, :1373
crates/krk-ui/src/appkit/anwendung.rs:1143, :1978, :2404 (neu), :4830, :4909, :4914
```

Darunter der Sprung auf ein Lesezeichen (`:1143`), das Lesen beim Start
(`:1978`), zwei Wege des Messmodus (`:4830`, `:4909`) und der Sprung im rechten
Dateifenster (`:4914`). Die Zahl stammt aus dem Abschnitt „Ausgangslage" des
Plans vom 260812-1145 („hat heute zwei Aufrufer"), war dort schon falsch und ist
mit Schritt 3 in den Baum gewandert.

Der Schaden ist nicht die Zahl, sondern was sie verspricht: wer `ordner_lesen`
umbaut und dem Satz glaubt, prüft drei Stellen und übersieht sieben.

**Zweitens: „acht Lagen" bei vier Eingaben.**
`crates/krk-ui/src/angezeigtedatei.rs:73-79`, am Prüffall
`alle_acht_kombinationen_tragen_ihre_antwort`:

> Die Tafel steht zusammen da, damit ein fehlender Fall auffaellt: zwei
> Wahrheitswerte und zwei Pfade, die es gibt oder nicht, ergeben acht Lagen,
> und jede traegt hier ihre Antwort.

Zwei Wahrheitswerte und zwei Wahlwerte ergeben sechzehn Lagen. Der Satz
begründet unmittelbar danach richtig, dass „beide sichtbar" wegfällt, weil
`Bereich::teilt_flaeche_mit` es ausschließt; das sind vier der sechzehn, es
bleiben zwölf. Geprüft sind in dieser Tafel acht, nämlich die mit genau einem
sichtbaren Bereich. Die vier Lagen „beide unsichtbar" fehlen; eine davon steht
in `ohne_sichtbaren_bereich_gibt_es_keine_angezeigte_datei` weiter unten, drei
in keiner Probe.

Die Rechnung selbst ist unverdächtig — `welche` fragt in beiden Zweigen zuerst
die Sichtbarkeit, also fallen alle vier auf `None`. Der Anspruch des
Prüffalls, „damit ein fehlender Fall auffaellt", löst sich mit acht von zwölf
Lagen aber nicht ein, und der Satz sagt das Gegenteil.

---

**Was zu tun ist**

- `anwendung.rs:2357`: den Satz auf das kürzen, was er tragen soll und was
  stimmt — der Sprung geht durch `ordner_lesen` und legt keinen zweiten
  Navigationsweg an. Die Zahl streichen; C2.7 des Plans verlangt sie nicht,
  sondern verlangt, dass kein zweiter Weg entsteht.
- `angezeigtedatei.rs:73-79`: entweder die vier fehlenden Lagen in die Tafel
  aufnehmen und „zwölf" schreiben, oder den Satz auf das beschränken, was die
  Tafel prüft („die acht Lagen mit genau einem sichtbaren Bereich"), und die
  vier übrigen dem Prüffall darunter überlassen, der dann alle vier nennt.

**Kontext**

- Beide Sätze sind aus dem Plan in den Baum gewandert, nicht im Baum entstanden.
  Der Plan bleibt, wie er ist — er ist die Aufzeichnung eines Standes und
  behält sie (`CLAUDE.md`, Abschnitt „Aufzeichnungen eines Standes"). Zu ändern
  sind allein die zwei Stellen im Code.
- Nicht betroffen ist „der siebte Abnehmer" von
  `kommandos::operationen::betroffene`, obwohl die Ausgangslage des Plans auch
  dort danebenlag (sie nennt sechs, es waren fünf). Die Runde hat **zwei** neue
  Abnehmer angelegt, den Tastenweg und `menuNeedsUpdate:`, und damit stimmt die
  Zahl sieben im Baum. Nachgezählt an `4d4402d` und an `d6eff4b`.
- Gefunden bei der Durchsicht von Turn 1 der Runde 6; nicht behoben.
