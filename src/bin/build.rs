use std::path::PathBuf;
use clap::{ArgMatches, SubCommand, App};
use mdbook::MDBook;
use mdbook::errors::Result;
use {get_book_dir, open};

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Build the book from the markdown files")
        .arg_from_usage("-o, --open 'Open the compiled book in a web browser'")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'The output directory for your \
             book{n}(Defaults to ./book when omitted)'",
        )
        .arg_from_usage(
            "--no-create 'Will not create non-existent files linked from SUMMARY.md'",
        )
        .arg_from_usage(
            "[dir] 'A directory for your book{n}(Defaults to Current Directory \
             when omitted)'",
        )
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::new(&book_dir).read_config()?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.book.build_dir = PathBuf::from(dest_dir);
    }

    if args.is_present("no-create") {
        book.create_missing = false;
    }

    book.build()?;

    if args.is_present("open") {
        open(book.get_destination().join("index.html"));
    }

    Ok(())
}
