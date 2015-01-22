use std::io::File;
use std::io::BufferedReader;

struct TranslationWord<'a> {
	source: &'a str,
	target: &'a str,
	part_of_speech: &'a str
}

pub fn read(filename: &str) {
	println!("Reading: {}", filename);
	let path = Path::new(filename);
	let mut file = BufferedReader::new(File::open(&path));

	for line in file.lines() {
		if line.is_err() { continue; }

		let text = line.unwrap();

		if !is_comment(&text) {
			let w = parse_line(&text);
			println!("{}, {}, {}", w.source, w.target, w.part_of_speech);
		}
	}
}

fn parse_line(line: &String) -> TranslationWord {
	let words: Vec<&str> =  line.words().collect();
	let source = words[0];
	let info = words[1];

	let target; let part_of_speech;

	if info.contains_char('[') {
		let parts: Vec<&str> = info.split('[').collect();
		let parts_2: Vec<&str> = parts[1].split(']').collect();

		target = parts[0];
		part_of_speech = parts_2[0];
	} else {
		target = info;
		part_of_speech = "unknown";
	}

	TranslationWord {
		source: source,
		target: target,
		part_of_speech: part_of_speech
	}
}

fn is_comment(line: &String) -> bool {
	'#' == line.char_at(0)
}