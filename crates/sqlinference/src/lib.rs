use sqruff_lib::core::linter::core::Linter;
use sqruff_lib_core::parser::parser::Parser;
use sqruff_lib_core::parser::segments::base::{ErasedSegment, Tables};

pub mod aggregate_functions;
pub mod columns;
pub mod infer_tests;
pub mod inference;
pub mod test;

pub fn parse_sql(parser: &Parser, source: &str) -> ErasedSegment {
    let tables = Tables::default();
    let (tokens, _) = Linter::lex_templated_file(&tables, source.into(), parser.dialect());

    let tokens = tokens.unwrap_or_default();
    let tables = Tables::default();
    parser.parse(&tables, &tokens, None, false).unwrap().unwrap()
}
