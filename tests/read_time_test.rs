use std::fs;
use std::path::Path;

use estimated_read_time::Options;
use estimated_read_time::ReadTime;

#[test]
fn default_options() {
	let filename = Path::new(".")
						.join("tests")
						.join("simple_doc.txt");
	println!(" FilePath :: {}", filename.display());						
	let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
	let read_options = Options::new();
	let read_time: ReadTime = estimated_read_time::text(&content, &read_options);
	println!(" Total Words :: {}", read_time.word_count());
	println!(" Total Seconds :: {}", read_time.seconds());

	println!(" Options :: WORD LENGTH :: {0}\n WPM :: {1}\n IS TECH DOC :: {2}\n TECH DIFFICULTY :: {3}\n", 
					read_options.get_word_length(), 
					read_options.get_wpm(),
					read_options.is_technical_document(),
					read_options.get_technical_difficulty());

	println!("-----------------------------------------------------------------------------------");
}

#[test]
fn technical_options() {
	let filename = Path::new(".")
						.join("tests")
						.join("simple_doc.txt");
	println!(" FilePath :: {}", filename.display());						
	let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
	let read_options = Options::new()
							.word_length(3)
							.technical_document(true)
							.technical_difficulty(2)
							.build()
							.unwrap_or_default();
							//.unwrap_or_else(|err| {
							//	eprintln!("Error occured while building Read Options :: {}", err);
							//	Options::new()
							//});
	let read_time: ReadTime = estimated_read_time::text(&content, &read_options);
	println!(" Total Words :: {}", read_time.word_count());
	println!(" Total Seconds :: {}", read_time.seconds());

	println!(" Options :: WORD LENGTH :: {0}\n WPM :: {1}\n IS TECH DOC :: {2}\n TECH DIFFICULTY :: {3}\n", 
					read_options.get_word_length(), 
					read_options.get_wpm(),
					read_options.is_technical_document(),
					read_options.get_technical_difficulty());
	println!("-----------------------------------------------------------------------------------");
}