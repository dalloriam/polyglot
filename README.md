# polyglot
![https://crates.io/crates/polyglot](https://img.shields.io/crates/v/polyglot?style=flat-square)

A library for format-agnostic serialization

## Target Use-Case
Polyglot is *not* meant as a one-stop shop for all your serialization needs. 
It does not expose all features of the most popular serialization libraries (e.g. pretty-printing) 
in favor of a unified interface. This library is best used when developing format-agnostic applications, like a server 
that can communicate over JSON or over MessagePack, or a service that can be configured in YML, JSON, or TOML.
Basically,if you're writing an application that cares little about the format your data is in, but that does care about
deserializing it, Polyglot is for you.
