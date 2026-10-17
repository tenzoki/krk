Lange Namen und PAX-Köpfe im tar werden beim Lesen ohne Obergrenze in den Speicher gelesen
---
Das Inhaltsverzeichnis eines tar oder tar.gz und der Lesezeiger beim Herauskopieren laufen über `tar::Archive::entries` in der gewöhnlichen Form. Dort liest die Kiste jeden GNU-Langnamen, jeden langen Verweis und jeden PAX-Kopf mit `read_all` ganz in einen `Vec`, so groß, wie der Kopf ihn angibt. Ein tar.gz von wenigen Megabyte kann einen Kopf von vielen Gigabyte Nullen tragen. Schon das Betreten eines solchen Archivs belegt dann Speicher bis zum Abbruch oder zur Erschöpfung. Der Umbau hat dafür eine Grenze, `HOECHSTENS_VORKOPF` mit 16 MiB, der Leser nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (ein fremdes Archiv bringt KRK beim bloßen Betreten an die Speichergrenze; ein Absturz verliert ungesicherten Editorinhalt)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Quelle gelesen, nicht nachgestellt)

- `tar-0.4.46/src/archive.rs`, `next_entry`: `gnu_longname = Some(EntryFields::from(entry).read_all()?)`, ebenso für `gnu_longlink` und `pax_extensions`.
- `tar-0.4.46/src/entry.rs`, `read_all`: `let cap = cmp::min(self.size, 128 * 1024); … self.read_to_end(&mut v)`. Die Vorbelegung ist begrenzt, das Lesen nicht.
- `crates/krk-core/src/archiv/verzeichnis.rs`, `tar_lesen` und `tar_gz_lesen`: `archiv.entries_with_seek()` und `archiv.entries()`, ohne `raw(true)`.
- `crates/krk-core/src/archiv/lesen.rs`, `tar_suchen`: `archiv.entries_with_seek()`, ohne `raw(true)`.
- `crates/krk-core/src/archiv/umbau.rs`, `Tarstrom::schreiben`: `raw(true)` und `if groesse > HOECHSTENS_VORKOPF { return Err(…"tar: extension header too large"…) }`. Damit gibt es die Grenze schon, nur an einer der drei Stellen.
- Der `Abbruchleser` fragt je MiB, also endet das Lesen beim Verlassen des Tabs. Bis dahin wächst der Speicher.

**Behebung (Richtung)**

Leser und Lesezeiger lesen den Strom wie der Umbau mit `raw(true)` und deuten die Vorköpfe selbst, mit derselben Grenze. Die Deutung des Umbaus (`pax_saetze`, `gruppenname`) wandert dazu an eine Stelle, die alle drei nutzen. Ein Kopf über der Grenze meldet `Verzeichnisfehler::Ungueltig`.

**Abnahme**

Eine Probe baut ein tar.gz mit einem GNU-Langnamenkopf von 1 GiB Nullen. Das Inhaltsverzeichnis endet dann mit `Verzeichnisfehler::Ungueltig`, ohne mehr als einige Dutzend MiB zu belegen, gemessen etwa über `getrusage` im Kindprozess. Die Proben aus `tests/archiv.rs` zu langen Namen bleiben grün.

---
Resolved: `crates/krk-core/src/archiv/verzeichnis.rs`, Modulkopf „Die Obergrenze fuer die Vorkoepfe eines tar“: `Kopfwache` und `Bewacht` lassen die Kiste beim Suchen des naechsten Eintrags nach jedem Sprung hoechstens `HOECHSTENS_VORKOPF` (16 MiB, jetzt hier und vom Umbau geteilt) und einen Kopf lesen, darueber endet das Lesen mit `Verzeichnisfehler::Ungueltig("tar: extension header too large")`; der gepackte Strom springt ueber `Vorwaerts`. Gewaehlt statt `raw(true)` mit eigener Deutung, weil der rohe Lauf der Kiste die PAX-Groesse und die Erweiterungsbloecke einer Luecken-Datei nicht beachtet und Archive kaputt laese, die heute gelesen werden. Genutzt in `tar_lesen`, `tar_gz_lesen` und im Lesezeiger `archiv/lesen.rs`, `tar_suchen`. Probe `ein_langer_name_ueber_der_grenze_macht_das_tar_ungueltig_ohne_ihn_zu_lesen` in `crates/krk-core/tests/archiv.rs` (tar duenn und tgz aus gzip-Gliedern, je 1 GiB Langname; misst im Kind ueber `/usr/bin/time -l` den Hoechststand unter 160 MiB; ohne Behebung rot); die Proben zu langen Namen bleiben gruen.
