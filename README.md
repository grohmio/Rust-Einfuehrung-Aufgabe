# Übungsaufgaben zum Vortrag "Einführung in Rust"

Die Aufgabe und der Code sind angelehnt an die Einführung "Programming Rust"
(O'Reilly 2017).

Originalsourcen: https://github.com/ProgrammingRust/mandelbrot (vielleicht erst
später lesen) ;-)


## Voraussetzungen

- Installierte Rust-Toolchain mit Rust 2018 (Version >= 1.31).
- Editor
- Shell
- "hello world" compiliert und läuft.


## (1) Syntax

Der Sourcecode enthält mehrere Syntaxfehler. Behebe die Fehler, so dass das
Programm überhaupt compiliert.


## (2) parse_pair

Die Funktion `parse_pair` soll eine Angabe der Bilddimension wie "640x480" in
zwei Zahlen vom Typ T zerlegen. Fehler sind entsprechend abzufangen und durch
die Tests spezifiziert.

Ergänze die Funktion so, dass alle Tests durchlaufen.

Hinweise:

- [find](https://doc.rust-lang.org/std/primitive.str.html#method.find)
- [from_str](https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str)
- [Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [match](https://doc.rust-lang.org/book/ch06-02-match.html)
- Konvertiere im Fehlerfall einen String per `into()` in das Trait-Objekt
    `Box<dyn Error>`.

Ab diesem Punkt ist das Programm prinzipiell benutzbar. Lasse einen
Release-Build laufen z.B. mit
`cargo run --release -- out.png 1600x1200 -1.2+0.35i -1+0.2i`. Öffne das
Ausgabebild mit einem Bildbetrachter deiner Wahl.


## (3) Tests für escape_time

Die Funktion `escape_time` ist das Herzstück des Programms, besitzt aber keine
Tests. Schreibe Tests für diese Funktion, die folgenden Bedingungen prüfen:

| Input c     | Resultat |
|-------------|----------|
|  0.5+0.0i   | Some(4)  |
| -1.0+0.0i   | None     |
| -1.0+0.275i | None     |
| -1.0+0.3i   | Some(34) |

Hinweise:

- [Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)


## (4) Test-Modul

Die Tests sind etwas unübersichtlich durch den Code verstreut. Erstelle ein
neues Modul `tests` (inline oder als eigene Datei) und fasse alle Tests dort
zusammen. Das Test-Modul soll nur bei Testläufen (`cargo test`) übersetzt
werden, sonst nicht.

Hinweise:

- [Modules](https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html)


## (5) struct Bounds

An mehreren Stellen im Programm wird die Bildgröße als Paar (u32,u32)
repräsentiert - nicht sehr deskriptiv. Für einen neuen Typen `Bounds` ein, der
Bildbreite und -höhe zusammenfasst.

Vorgeschlagene Signaturen:

```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds {
    w: u32,
    h: u32,
}

impl Bounds {
    fn parse(string: &str) -> Result<Self, Box<dyn Error>> {}

    fn pixels(&self) -> usize {}

    fn render(&self, window: &Window) -> Vec<u8> {}
}
```

Fasse bestehende Funktionalität in den neuen impl-Block zusammen
Ändere alle Stellen, die mit Paaren (u32,u32) arbeiten (z.B. bounds.0).


## (6) Performance

- Vergleiche die Laufzeit zwischen Debug- und Release-Build bei hinreichend
    großem Ausgabebild.

- In der Funktion `pixel2point` wird die Bildbreite und -höhe jedes Mal neu
    berechnet, obwohl sie konstant bleibt. Extrahiere diese Berechung aus der
    inneren Schleife und messe den Laufzeitunterschied.

Hinweise:

- Unter Linux/macOS kann man die Laufzeit eines Programms mit dem Shell-Utility
    `time` messen. Allerdings solltest du nicht `cargo run` benutzen, sondern
    per `cargo build` ein separates Binary erstellen (landet unter target/) und
    direkt aufrufen, um die Messungen nicht zu verfälschen.


## (7) Zusatzaufgabe: Commandline-Parsing mit clap

Für alle, die schon fertig sind und/oder gelangweilt.

Die Kommandozeile wird momentan etwas stiefmütterlich behandelt. Das Crate
[clap](https://clap.rs/) bietet leistungsstarke Funktionen, um den Aufruf
komfortabler zu gestalten.

Anregungen:
- Hilfetexte
- Default-Werte für Bildgröße etc.
- Validierung für UL <=> LR
- ... nach Belieben ...

Hinweise:

- [Clap Doku](https://docs.rs/clap/2.32.0/clap/)
- [Arg::from_usage](https://docs.rs/clap/2.32.0/clap/struct.Arg.html#method.from_usage)
    erspart viel Tipparbeit


## Starten unter Docker

- Docker Image bauen

```console
> docker build -t my-rust-app .
```

- Docker Container instanziieren und starten

```console
> docker run -it --rm --name my-running-app  my-rust-app
```