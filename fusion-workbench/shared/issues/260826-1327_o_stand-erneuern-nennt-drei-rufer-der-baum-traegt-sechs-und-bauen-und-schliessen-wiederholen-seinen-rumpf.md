stand_erneuern nennt drei Rufer, der Baum traegt sechs, und bauen und schliessen wiederholen seinen Rumpf

---

`Editorbereich::stand_erneuern` nennt sich "die eine Stelle", an der Text, Kopf und Darstellung
zusammen nachgezogen werden, mit "drei Aufrufern". Der Baum traegt sechs Rufer, und zwei weitere
Stellen — `bauen` und `schliessen` — schreiben dieselben drei Zeilen von Hand hin, statt die Funktion zu
rufen. Zwei Nachbar-Doc-Kommentare zaehlen ihre Rufer ebenfalls falsch.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-ui/src/appkit/editor.rs:2229-2234`:

> **Die drei Schritte, die zusammengehoeren**, und die eine Stelle, an der sie stehen … Drei Aufrufer
> gehen durch sie — ein gelungenes Oeffnen, die uebernommene zurueckgehaltene Datei und seit S37 das
> Ersetzen —, und ohne diese Funktion waeren es drei Stellen mit derselben Reihenfolge.

Rufer im Baum (`grep -n 'stand_erneuern(' editor.rs`): `:1742` (zurueckgehaltenes_uebernehmen),
`:1822` (ladeausgang_einziehen), `:2204` (umkehren), `:2318` (flaeche_richten), `:2625`
(treffer_ersetzen), `:2716` (alle_treffer_ersetzen) — sechs.

Die drei Zeilen von Hand:

- `:1550-1552` (`bauen`): `stand_einsetzen(Verlauf::Faellt)`, `kopf_nachziehen()`,
  `darstellung_nachziehen()`.
- `:1768-1773` (`schliessen`): dieselben drei Rufe in derselben Reihenfolge.

Nachbarn:

- `:2770-2771` `darstellung_nachziehen`: "vier Aufrufer: den Aufbau, ein gelungenes Oeffnen, das
  Schliessen und den Ansichtswechsel" — direkte Rufer sind `bauen`, `schliessen`, `stand_erneuern`,
  `ansicht_umschalten` (`:1552`, `:1773`, `:2248`, `:2765`); "ein gelungenes Oeffnen" ruft nicht
  direkt, das Ersetzen und das Umkehren kommen ueber `stand_erneuern` dazu.
- `:1992-1995` `kopf_nachziehen`: fuenf Anlaesse genannt; `:2247` (aus `stand_erneuern`, also
  Ersetzen, Umkehren und CRLF-Richten) fehlt.

## Was zu tun waere

`bauen` und `schliessen` auf `stand_erneuern(Verlauf::Faellt)` umstellen; die drei Zaehlungen auf den
Stand bringen oder — wie CLAUDE.md es fuer Zahlen dieser Art vorsieht — durch einen `grep` ersetzen.

## Umfang

`krk-ui`, `appkit/editor.rs`.
