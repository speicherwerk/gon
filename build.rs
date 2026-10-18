use std::process;

use parol::{ParolErrorReporter, build::Builder, parol_runtime::Report};

fn main() {
    eprintln!("hello");
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("gon.par")
        .expanded_grammar_output_file("../gon-exp.par")
        .parser_output_file("parser.rs")
        .actions_output_file("gon_grammar_trait.rs")
        .user_type_name("GonGrammar")
        .user_trait_module_name("gon_grammar")
        .trim_parse_tree()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "gon.par").unwrap_or_default();
        process::exit(1);
    }
}
