# Das Richten der Fläche kopiert den Text eines 16-MB-Dokuments dreimal

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:1197-1209` (`flaeche_richten`), `crates/krk-core/src/text/datei.rs:404` (`versatz_nach_der_wandlung`)
**Cross-references:** `issues/260810-0215_c_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`, `issues/260810-0054_o_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md`, Commit `d5993f1`, C8

---

## Der Befund

Ein einziges eingefügtes `\r\n` löst in einer Datei nahe der 16-MB-Grenze diese
Kette aus:

```
  text_zurueckschreiben   text.string().to_string()     ~16 MB  UTF-16 -> UTF-8
  bearbeiten              in_gehaltene_form(stand)      ~16 MB  Kopie mit Wandlung
  flaeche_richten         text.string().to_string()     ~16 MB  UTF-16 -> UTF-8, zum zweiten Mal
  versatz_nach_der…       rest.to_owned()               bis 16 MB  Kopie des Restes
  versatz_nach_der…       in_gehaltene_form(rest)       bis 16 MB  zweite Kopie des Restes
  stand_einsetzen         NSString::from_str(stand)     ~16 MB  UTF-8 -> UTF-16
  stand_erneuern          darstellung_nachziehen()      ganze Datei neu einfärben
```

Der zweite Durchlauf durch `text.string()` ist im Code kommentiert und als Preis
angenommen. Die beiden Kopien in `versatz_nach_der_wandlung` sind es nicht:
`rest.to_owned()` legt den Rest des Textes auch dann an, wenn `in_gehaltene_form`
ihn danach unverändert zurückgibt — und das ist der Regelfall, denn hinter der
eingefügten Stelle steht meist nichts mehr zu wandeln.

## Warum das hier steht und nicht als Behebung

C8 sagt für das Einfügen keine Zeit zu, und der Weg läuft nicht je Tastendruck.
Der Befund ist deshalb Low. Er steht trotzdem, weil `260810-0054` schon eine
gemessene Verzögerung beim Tippen in großen Dateien führt und die Einfärbung
am Ende dieser Kette ein zweites Mal angestoßen wird.

## Was zu prüfen wäre

`in_gehaltene_form` nimmt ein `String` und gibt eines zurück; eine Fassung über
`Cow<str>` ließe den kurzen Weg ohne Kopie durchlaufen und käme weiterhin ohne
eine zweite Formulierung der Wandlungsregeln aus — das ist die Zusage, die der
Doc-Kommentar von `versatz_nach_der_wandlung` ausdrücklich hält und die eine
Zählung der weggefallenen Zeichen brechen würde.

Ob das die Signatur von `in_gehaltene_form` verändert, gehört in die Antwort:
sie wird als „die **eine** Stelle" geführt und hat drei weitere Aufrufer —
`datei::einlesen:333`, `Editormodell::bearbeiten` (`editormodell.rs:918`) und
den Ersatztext in `editormodell.rs:1154`. Der erste reicht sie als
Funktionswert an `Option::map` weiter und ist damit an ihre heutige Signatur
gebunden.
