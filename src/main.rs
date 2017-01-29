use std::io::*;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum BraceType {
	Round,
	Curly,
	Square,
}

#[derive(Debug,Copy,Clone)]
struct BraceInfo {
	brace_type : BraceType,
	open : bool,
	level : usize,
	line : usize,
	pos : usize,
}

fn classify(c:char) -> Option<(bool,BraceType)> {
	match c {
		'(' => Some((true,BraceType::Round)),
		'{' => Some((true,BraceType::Curly)),
		'[' => Some((true,BraceType::Square)),
		')' => Some((false,BraceType::Round)),
		'}' => Some((false,BraceType::Curly)),
		']' => Some((false,BraceType::Square)),
		_ => None,
	}
}

fn to_char(open:bool, t:BraceType) -> char {
	match (open,t) {
		(true,BraceType::Round)   => '(',
		(true,BraceType::Square)  => '[',
		(true,BraceType::Curly)   => '{',
		(false,BraceType::Round)  => ')',
		(false,BraceType::Square) => ']',
		(false,BraceType::Curly)  => '}',
	}
}

fn main() {
	let input = stdin();
	let err_count = check_braces(input.lock());
	println!("scan complete: found {} brace errors", err_count);
}

fn check_braces<T:BufRead>(input:T) -> usize {
	let lines = input.lines();

	let mut braces = Vec::new();


	for (line_num, line) in lines.map(|x|x.unwrap()).enumerate() {
		//indent level
		let indent = line.chars().take_while(|c| c.is_whitespace()).count();

		let braces_in_line : Vec<_> = line
			.chars()
			.enumerate()
			.filter_map(|(i,c)| classify(c).map(|(o,t)|(i,o,t)))
			.map(|(i,o,t)| BraceInfo {
				brace_type : t,
				open : o,
				pos : i,
				line : line_num,
				level : indent,
			})
			.collect();

		braces.extend(braces_in_line);
	}

	//reverse braces, so we can pop them off in order
	braces.reverse();

	//stack to track opening and closing brace pairs
	let mut stack = Vec::new();
	let mut err_count = 0;
	while braces.len() > 0 {
		let brace = braces.pop().unwrap();

		if brace.open {
			stack.push(brace);
			continue;
		}

		let c = to_char(brace.open, brace.brace_type);
		if stack.len() == 0 {
			println!("{},{}: Missing open brace for closing '{}'. Did you forget an open brace?", brace.line,brace.pos,c);
			err_count += 1;
			continue;
		}

		let top = stack.pop().unwrap();
		let tc = to_char(top.open, top.brace_type);


		if top.level < brace.level {
			println!(
				"{},{}: Closing brace '{}' is indented more than open brace '{}' at {},{}. Did you forget an open brace or indent too far?", 
				brace.line, brace.pos, c, 
				tc, top.line, top.pos
			);

			//assume it was missing open brace, check top for further errors
			stack.push(top);
			err_count += 1;
			continue;
		}

		if top.level > brace.level {
			println!(
				"{},{}: Closing brace '{}' is indented less than open brace '{}' at {},{}. Did you forget to close a brace or indent too little?", 
				brace.line, brace.pos, c, 
				tc, top.line, top.pos
			);

			//assume it was missing close brace, check brace for further errors
			braces.push(brace);
			err_count += 1;
			continue;
		}

		if top.brace_type != brace.brace_type {
			println!(
				"{},{}: Closing brace '{}' does not match open brace {} at {},{}. Did you use the wrong brace type, or transpose braces?", 
				brace.line, brace.pos, c, 
				tc, top.line, top.pos
			);

			//don't recheck either brace
			err_count += 1;
			continue;
		}

		//braces matched
	}

	//report any unclosed braces
	stack.reverse();
	for brace in stack {
		let c = to_char(brace.open, brace.brace_type);
		println!(
			"{},{}: Open brace '{}' has no matching close brace", 
			brace.line, brace.pos, c
		);
		err_count += 1;
	}

	err_count
}

#[cfg(test)]
mod tests {
	use super::check_braces;

	#[test]
	fn check_empty() {
		let s = b"";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 0);
	}

	#[test]
	fn check_round() {
		let s = b"()";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 0);
	}

	#[test]
	fn check_square() {
		let s = b"[]";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 0);
	}

	#[test]
	fn check_curly() {
		let s = b"{}";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 0);
	}

	#[test]
	fn check_round_open() {
		let s = b"(";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_square_open() {
		let s = b"(";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_curly_open() {
		let s = b"(";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_round_close() {
		let s = b")";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_square_close() {
		let s = b"]";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_curly_close() {
		let s = b"}";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_missing_open() {
		let s = b"{\n\t}\n}";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_missing_close() {
		let s = b"{\n\t{\n}";
		let err_count = check_braces(&s[..]);
		assert!(err_count == 1);
	}

	#[test]
	fn check_overindent() {
		let s = b"{\n\t}";
		let err_count = check_braces(&s[..]);
		assert!(err_count > 0);
	}

	#[test]
	fn check_underindent() {
		let s = b"\t{\n}";
		let err_count = check_braces(&s[..]);
		assert!(err_count > 0);
	}
}
