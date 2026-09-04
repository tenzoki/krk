# Die Blattsperre meldet, dass sie abgewiesen hat

**Datum:** 2026-09-04 20:47
**Status:** Complete
**Auslöser:** Nutzerauftrag nach der Aufklärung von
`260904-1827_*_sichern-auf-einem-netzlaufwerk-schlaegt-still-fehl-die-datei-bleibt-ungesichert.md`
**Kein Circle aktiv.**

## Der Auftrag

Der Zulässigkeitsvorbehalt am Kopf von `Anwendungsdelegierter::kommando_ausfuehren`
weist ab und sagt nichts. Bei `cmd+s` kostet das Arbeit: der Nutzer hält die Datei für
gesichert und schließt sie. Eine Abweisung durch die Blattsperre soll sagen, dass sie
abgewiesen hat.

## Die vier Fragen des Auftrags

**1. Nur die Blattsperre.** Die drei übrigen Bestandteile von
`zulaessigkeit::zulaessig` weisen im Sekundentakt ab, ohne dass etwas verlorenginge:
der Fokusvorbehalt trennt die Bereiche und sieht jeden Pfeildruck im Editor, der
Ersthelferbefund weist ab, während der Nutzer in der Liste umbenennt, und ein fremdes
Schlüsselfenster heißt, dass KRK gar nicht vorn steht. Die Blattsperre ist die einzige
der vier, die den Nutzer über einen **ausgeführten** Befehl täuschen kann: das Blatt
steht sichtbar da, der Befehl kommt nicht durch, und nichts unterscheidet das für ihn
von einem Befehl, der gewirkt hat.

**2. Die Statuszeile ist sichtbar, beim Anfangsmaß des Fensters.** Gemessen mit einem
Nachbau der Fenstermaße dieses Baums (1280 × 720, Zeile 18 pt hoch auf 18 pt über der
Unterkante), sieben Blattmaße durchgefahren, Blattrahmen gegen Zeilenrahmen gehalten:
macOS 15 setzt ein Blatt senkrecht **mittig** ins Elternfenster, nicht an dessen
Oberkante. Das höchste Blatt dieses Baums ist der Notizzettel (Beigabe 332 pt, Blatt
552 pt); die Schwelle, ab der ein Blatt die Zeile erreicht, liegt bei 720 pt
Fensterhöhe zwischen 440 und 460 pt Beigabe. Bei einem klein gezogenen Fenster gilt das
nicht: bei 500 pt Höhe verdeckt der Notizzettel die Zeile, beim Mindestmaß 336 pt schon
das Stapelumbenennen. Der Rest ist eine Nutzerfrage und steht als
`260904-2047_*_wohin-geht-die-blattmeldung-wenn-das-blatt-die-statuszeile-verdeckt.md`.

**3. Der Schnitt liegt am Anschlag und nicht an einer Liste von Kommandos.** Steht ein
Blatt, gehört die Tastatur ihm. Ein Anschlag, den AppKit zum Bedienen des Blattes
braucht, ist kein abgewiesener Befehl, sondern ein angekommener Griff: `tab` rückt die
Ansteuerung weiter, `space` löst die gewählte Schaltfläche aus, `return` die
Vorgabeschaltfläche, `esc` den Abbruch, und der Pfeilblock samt `pageup`, `pagedown`,
`home` und `end` bewegt die Auswahl in einer Liste im Blatt. Diese zwölf Stellen
schweigen, alles übrige meldet. Die Gruppe ist eine Eigenschaft von AppKit und wächst
nicht mit den Kommandos dieses Baums; `krk_core::tasten::parser::TASTEN` führt sie schon
heute unter der Überschrift „der Pfeilblock, vollständig, und die übrige Bewegung im
Blatt". Gefragt ist die **Stelle** und nicht die Zusatztaste: `cmd+up` schweigt damit
wie `up`, und der Preis ist genannt und klein, denn eine nicht bewegte Auswahl kostet
keine Arbeit.

**4. Die vier Befehle, die während eines Blattes durchkommen, werden gar nicht erst
abgewiesen** und erreichen den Zweig nie. Die Regel fragt trotzdem
`operationen::waehrend_blatt_erlaubt` und `zulaessigkeit::immer_erreichbar`, also die
zwei Regeln, die den Durchlass schon heute entscheiden, und keine dritte Fassung
daneben; damit hält sie auch dann noch, wenn ein fünftes hinzukäme.
`zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` ist unberührt und
grün.

## Was gebaut wurde

| Datei | Änderung |
|---|---|
| `crates/krk-ui/src/kommandos/blattmeldung.rs` | neu: die Regel, die zwölf Stellen, der eine Satz, acht Proben |
| `crates/krk-ui/src/kommandos/mod.rs` | `pub mod blattmeldung;`, Tabelleneintrag und der Absatz, warum die Regel hinter `zulaessigkeit` steht und nicht darin |
| `crates/krk-ui/src/appkit/anwendung.rs` | der eine Aufrufer im Abweisungszweig von `kommando_ausfuehren`, samt Löschregel der Befehlsantwort |

Die Regel ist eine Konjunktion aus vier Wahrheitswerten und damit
überschneidungsfrei und vollständig ohne Tafel: es gab einen Tastendruck, es steht ein
Blatt, die Blattsperre hält diesen Befehl auf, der Anschlag bedient kein Blatt. `#[must_use]`
steht daran, weil das stille Fallenlassen genau den Defekt wiederherstellte, gegen den
sie gebaut ist.

**Der Menüweg bekommt die Meldung nicht ab, und zwar ohne einen Zweig dafür:** er
reicht `None` als Anschlag herein, wie es `rueckschritt` schon vorführt. Seine Antwort
ist die Ausgrauung.

**Die Löschregel der Befehlsantwort steht innerhalb des Meldezweigs.** Eine stumme
Abweisung — jeder Pfeildruck im Editor — räumt die stehende Antwort damit nicht weg;
das Verhalten dort ist unverändert.

Kein Untergrenzen-Abschnitt war nachzutragen: das neue Modul liegt nicht unter
`appkit/`, und der Aufrufer in `anwendung.rs` spricht keine neue Klasse an.

## Verification

- [x] `make check` — Exit 0 (Bau, `cargo test --workspace`, `fmt --check`, `clippy -D warnings`)
- [x] `kommandos::blattmeldung::tests` — acht Proben grün, darunter der gemessene Fall
      `ein_abgewiesenes_sichern_meldet_sich` und die Aufruferzählung
      `die_regel_hat_genau_einen_aufrufer`
- [x] `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` grün
- [x] Die Verdeckung der Statuszeile ist gemessen und nicht angenommen; die Tabelle
      steht im Entscheidungsdatensatz
- [ ] **Am laufenden Bündel bleibt zu prüfen**, und nur der Nutzer kann es: eine Datei
      im Editor ändern, ein Blatt aufziehen (`shift+cmd+n` legt einen Ordner an,
      `cmd+k` zieht den Notizzettel auf), `cmd+s` drücken. Erwartet: „nicht ausgeführt: über
      dem Fenster steht ein Blatt" in der Statuszeile. Dann dasselbe mit `up` und
      `tab`: erwartet ist Schweigen und die gewohnte Bedienung des Blattes.

## Offen geblieben

- `260904-2047_*_wohin-geht-die-blattmeldung-wenn-das-blatt-die-statuszeile-verdeckt.md`
  — vier Möglichkeiten, Empfehlung: beim gebauten Stand bleiben.
- `260904-1902_*_das-atomare-schreiben-weitet-die-rechte-einer-600-datei-auf-644.md`
  — unberührt, aus dem Lauf vom 260904-1905.
