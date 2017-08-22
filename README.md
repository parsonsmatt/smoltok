# smoltok

An implementation of Smalltalk 80 in Rust based on the freely available [blue book](http://www.mirandabanda.org/bluebook/).

## Parser

The parser is implemented using [`combine`](https://github.com/Marwes/combine).
I'm coming from Haskell, so the Parsec inspiration was familiar and welcome.

The parser was initially implemented by reading the first chapter on syntax and following along.
That proved to be a bit treachorous, with quite a few subtle implementation bugs.
Fortunately, the back of the book provided a railway syntax chart, which made it quite easy to write the parser for.
