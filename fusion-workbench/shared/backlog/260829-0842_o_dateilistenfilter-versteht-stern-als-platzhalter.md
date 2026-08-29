# Der Dateilistenfilter versteht `*` als Platzhalter für eine beliebige Zeichenfolge

**Filed by:** k1

Ein `*` im Filtertext steht für eine beliebige Zeichenfolge (Glob), im ganzen Filtertext und nicht nur im Marker: `260503-1144_*_f1-zitadel-slot-rehost-and-swap-test.md` trifft `260503-1144_d_f1-…` und `260503-1144_c_f1-…`. Heute vergleicht `traegt_die_folge` (`crates/krk-core/src/verzeichnis/filter.rs`) wörtlich als Teilzeichenfolge, und `*` landet als Zeichen im Filter. Vom Nutzer am 260829 gewählt (Möglichkeit 1 von drei: Glob überall statt nur `_*_` oder Teilfolgen in Reihenfolge); ein getipptes `*` lässt sich danach nicht mehr wörtlich suchen. Berührt den einen Vergleich, den Inhaltsfilter (Runde 11) und den tiefen Durchlauf, die ihn alle rufen.
