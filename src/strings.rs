//! Important strings

/// Regexes for errors
const FILE_NOT_FOUND: &[&str] = &[
    r"^No file\s*(.*)\.$",
    r"^\! LaTeX Error: File `([^\']*)\' not found\.",
    r"^\! I can\'t find file `([^\']*)\'\.",
    r".*?:\d*: LaTeX Error: File `([^\']*)\' not found\.",
    r"^LaTeX Warning: File `([^\']*)\' not found",
    r"^Package .* [fF]ile `([^\']*)\' not found",
    r"^Package .* No file `([^\']*)\'",
    r"Error: pdflatex \(file ([^\)]*)\): cannot find image file",
    r": File (.*) not found:\s*$",
    r"! Unable to load picture or PDF file \'([^\']+)\'.",
];
