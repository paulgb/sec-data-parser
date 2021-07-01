# sec-data-parser

A parser for [SEC EDGAR](https://www.sec.gov/edgar/searchedgar/companysearch.html) `.nc` files,
which represent an SEC filing.

These are [SGML](https://en.wikipedia.org/wiki/Standard_Generalized_Markup_Language) files, but
the format seems to have drifted since the latest publicly-available DTD files I could find, so
the parser implemented here is partly derived from the real-world data contained in the filings.

Attempts to provide a lossless `Rust struct` representation of each filing. Dates and datetimes
are represented as `chrono` objects.

This is currently a work-in-progress, and as such is not yet on crates.io, but it successfully
parses all non-corrupt `.nc` filings I have fed into it, which range from 1995 to 2021.

Decodes binary files when provided. Extracts included `XBRL` (enclosed in `<XBRL></XBRL>` tags)
as a `String`, but does not attempt to parse XBRL, which is an entirely separate format.
