**THIS IS ABANDONED. DO NOT USE**

---

# LTXMK: A LaTeX build system

LTXMK is intended to be a replacement for [latexmk](https://mg.readthedocs.io/latexmk.html)

## Problems to be addressed

- If an error takes place in continuous mode, it will not try to recompile once
 the error is fixed
 - Cannot run continuously on multiple files
 - Bad defaults around PDF and output folder and build files
 - Arcane custom dependency rules
 - Poor CI support
 - latexmk is a large perl script
    - Offload work to other crates
 - Assume a sane and modern evironment
    - Everything is a pdf

## Status
Currently LTXMK is being abandoned and doesn't work. The LaTeX output is just too unfriendly to be parsed by anythink other than a 10kloc perl script.


## Things to support

- pdflatex
- bibtex
- biber
- makeindex

## License
Rust code licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
   
at your option.

Test data in `corpus` under different licences, see there for details.

## Contribution
Contribution's are welcome, as pull requests, bug reports, feedback or any
other form

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

