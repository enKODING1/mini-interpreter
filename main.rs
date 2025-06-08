use std::io;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token{
	Atom(char),
	Op(char),
	EOF
}

struct Lexer{
	tokens: Vec<Token>
}

impl Lexer{
	fn new(input: &str) -> Lexer{
		let mut tokens = input
			.chars()
			.filter(|it|!it.is_ascii_whitespace())
			.map(|c| match c{
				'0'..='9'|'a'..='z'|'A'..='Z' => Token::Atom(c),
				_ => Token::Op(c),
			}).collect::<Vec<_>>();
			tokens.reverse();
			Lexer {tokens}
	}

	fn next(&mut self) -> Token{
		self.tokens.pop().unwrap_or(Token::EOF)
	}

	fn peek(&mut self) -> Token{
		self.tokens.last().copied().unwrap_or(Token::EOF)
	}
}

fn infix_binding_power(op: char) -> (f32, f32){
	match op{
		'+' | '-' => (1.0, 1.1),
		'*' | '/' => (2.0, 2.1),
		_ => panic!("unknown operator: {:?}", op)
	}
}


enum Expression{
	Atom(char),
	Operation(char, Vec<Expression>)
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
					Expression::Atom(c) => write!(f, "{}", c),
					Expression::Operation(op, exprs) => {
							write!(f, "({} ", op)?;
							for (i, expr) in exprs.iter().enumerate() {
									if i > 0 {
											write!(f, " ")?;
									}
									write!(f, "{}", expr)?;
							}
							write!(f, ")")
					}
			}
	}
}

impl Expression{
	fn from_str(input: &str) -> Expression{
		let mut lexer = Lexer::new(input);
		parse_expression(&mut lexer, 0.0)
	}
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression{
	let mut lhs = match lexer.next(){
		Token::Atom(it) => Expression::Atom(it),
		Token::Op('(') => {
			let lhs = parse_expression(lexer, 0.0);
			assert_eq!(lexer.next(), Token::Op(')'));	
			lhs
		}
		t => panic!("bad token: {:?}", t),
	};

	loop {
		let op = match lexer.peek(){
			Token::EOF => break,
			Token::Op(')') => break,
			Token::Op(op) => op,
			t => panic!("bad token: {:?}", t),
		};

		let (l_bp, r_bp) = infix_binding_power(op);
		if l_bp < min_bp{
			break;
		}
		lexer.next();

		let rhs = parse_expression(lexer, r_bp);
		lhs = Expression::Operation(op, vec![lhs, rhs]);
	}
	lhs
}


// #[test]
// fn test_1(){
// 	let s = Expression::from_str("1");
// 	assert_eq!(s.to_string(), "1");
// }

// #[test]
// fn test_2(){
// 	let s = Expression::from_str("1 + 2 * 3");
// 	assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
// }

fn main(){
	loop{
		print!(">> ");
		io::stdout().flush().unwrap();
		let input: String = {
			let mut buf: String = String::new();
			std::io::stdin().read_line(&mut buf).unwrap();
			buf
		};
		if input.trim() == "exit"{
			break;
		}
		let expr: Expression = Expression::from_str(&input);
		println!("{}", expr);

	}
}