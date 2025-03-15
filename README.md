# bagelwithlox

A rust implementation of the Lox language from Crafting Interpreters.

This is an experiment in learning rust and learning to build an interpreter,
created while attending [David Beazley's Crusty Interpreter
class](https://dabeaz.com/crusty.html). More work is intended to clean up and
improve the implementation, but as it stands the current state is as I left it
at the end of the course. Lox support through functions should be implemented;
class support is still remaining but the building blocks are there.

The implementation should allow running source files and executing statements
in the REPL.  The git history shows an attempt to make the language
expression-oriented, which would allow the REPL to be more useful. I intend to
return to that idea at some point in the future, as rust has convinced me
expression-oriented is really the way to go.

Some test Lox files are included in the `./loxfiles` directory.
