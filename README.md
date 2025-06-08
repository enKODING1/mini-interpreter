# This Simple Algorithm Powers Real Interpreters: Pratt Parsing

[link](https://www.youtube.com/watch?v=0c8b7YfsBKs)

Lexing으로 문자열을 의미있는 토큰들로 만들기 

<img width="50%" src="https://github.com/user-attachments/assets/95e4c92a-765c-4db1-9dae-875b5d2deed0"/>

<img width="50%" src="https://github.com/user-attachments/assets/58ef27e5-a85c-4642-921e-a324fefb43fc"/>


## **토큰 타입정의**

```rust
enum Token{
	Atom(char),
	Op(char),
	Eof
}

struct Lexer{
	tokens: Vec<Token>
}
```

숫자 → “Atom”

연산자 → “op”

파일끝 → “EOF”

3개의 타입을 정의한다.

## **토큰화 (기본적인 렉싱)**

```rust
**impl Lexer{
	fn new(input: &str) -> Lexer{
		let mut tokens = input
			.chars()
			.filter(|it|!it.is_ascii_whitespace())
			.map(|c| match c{
				'0'..='9'|'a'..='z'|'A'..='Z' => Token::Atom(c),
				_ => Token::Op(c),
			}).collect::<Vec<_>>();
			Lexer {tokens}
	}
}**
```

<img width="189" alt="3" src="https://github.com/user-attachments/assets/2afba16e-3d60-453a-a1a1-0cde4f85b4a8" />


문자열을 타입에 따라 분류시키기 위한 위한 Lexer 코드다.

## **파싱 (파싱된 표현식을 트리가 어떻게 표현할지 정의)**

```rust
enum Expressoin{
	Atom(char),
	Operation(char, Vec<Expression>)
}
```

- Atom(char)의 경우
    - 더이상 분해할 수 없는 기본 단위 숫자 ‘1’, 변수 ‘a’, ‘b’등
- Operation(char, Vec<Expression>) 의 경우
    - 첫 번째 필드 char: 연산자 (+, -, ,,/등)
    - 두 번째 필드 Vec<Expression>: 해당 연산자의 피 연산자들
    - 재귀적 구조를 가짐 (트리 내부 노드)

<img width="80%" src="https://github.com/user-attachments/assets/49ec6362-f1e3-4fdd-bdba-b5a3cc1fc63e"/>


Expression을 사용하면 위 같은 트리를 만들 수 있다. Operation의 경우 현재 연산자와 하위 Expression을 vec로 받기에 트리를 생성한다고 생각하면 된다.

위 과정이 일련의 문장을 해석 한 과정이다. lexing → tokenization → parsing → ast 그렇다면 파싱을 진행할 때 연산자의 우선순위를 지정해줘야 한다. 

*, / 와 같이 연산중 우선적으로 계산 해야하는 부분은 결합력의 값을 높여준다. +,-는 결합력 값을 낮춰준다. 

<img width="50%" src="https://github.com/user-attachments/assets/39a312aa-2b39-429e-bb29-2a0720603760"/>


## **파싱 표현 작성하기**

```rust
fn parse_expression(lexer: &mut Lexer) -> Expression{
	// a(1hs) +(op) b(rhs) 
	let mut lhs = match lexer.next(){
		Token::Atom(it) => Expression::Atom(it),
		t => panic!("bad token: {:?}", t),
	};
	let op = match lexer.peek(){
		Token::Eof => return hs,
		Token::Op(op) => op,
		t => panic!("bad token: {:?}", t),
	};
	lexer.next();
	let mut rhs = match lexer.next(){
		Token::Atom(it) => Expression::Atom(it),
		t => panic!("bad token: {:?}", t),
	}
	Expression::Operation(op, vec![lhs, rhs]);
}
```

<img width="30%" src="https://github.com/user-attachments/assets/7b5264eb-864b-4e06-a213-314c482cb78b"/>


위코드는 parse 표현식을 생성해준다. a + b 를 실행 하면 오른쪽의 사진 처럼 트리를 생성 해준다. 하지만 이 코드에는 문제점이 존재한다. a + b * c 의 경우 b * c가 우선 계산 되어야 하는데 위 코드는 어떤 경우에도 a + b로 트리를 생성할 것이다. 왼쪽에서 부터 읽기때문에 곱하기가 존재하는지의 여부는 알 수 없으므로 위의 코드는 사실상 참고만 될 뿐 유용하지 않은 코드다.

```rust
fn infix_binding_power(op: char) -> (f32, f32){
	match op{
		'+' | '-' => (1.0, 1.1),
		'*' | '/' => (2.0, 2.1),
		_ => panic!("unknown operator: {:?}", op)
	}
}
```

```rust
fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression{
	let mut lhs = match lexer.next(){
		Token::Atom(it) => Expression::Atom(it),
		t => panic!("bad token: {:?}", t),
	};
	loop {
		let op = match lexer.peek(){
			Token::Eof => break,
			Token::Op(op) => op,
			t => panic!("bad token: {:?}", t),
		};
		lexer.next();
		let (l_bp, r_bp) = infix_binding_power(op);
		if l_bp < min_bp{
			break;
		}
		let rhs = parse_expression(lexer, r_bp);
		lhs = Expression::Operation(op, vec![lhs, rhs]);
	}
	lhs
}
```

위 코드는 결합력 값을 이용해 결합력이 큰 쪽의 계산을 우선적으로 처리하도록 만들었다. 1.1, 2.1은 동등한 연산자를 만났을 때 어떻게 처리해도 결과에는 지장이 없지만 예측가능한 동작을 위해 트릭을 이용했다. 위의 코드에선 우선순위 문제를 해결했지만 괄호문제는 해결하지 못했다. 괄호에 감싸지면 우선순위가 낮더라도 먼저 계산이 되어야 한다.

```rust
fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression{
	let mut lhs = match lexer.next(){
		Token::Atom(it) => Expression::Atom(it),
		Token::Op('(') => {
			let lhs = parse_expression(lexer, 0.0)
			assert_eq!(lexer.next(), Token::Op(')'));	
			lhs
		}
		t => panic!("bad token: {:?}", t),
	};
	loop {
		let op = match lexer.peek(){
			Token::Eof => break,
			Token::Op(')') => break,
			Token::Op(op) => op,
			t => panic!("bad token: {:?}", t),
		};
		lexer.next();
		let (l_bp, r_bp) = infix_binding_power(op);
		if l_bp < min_bp{
			break;
		}
		let rhs = parse_expression(lexer, r_bp);
		lhs = Expression::Operation(op, vec![lhs, rhs]);
	}
	lhs
}
```

괄호는 Atom으로 취급하지 않았기에 Op로 취급될 것이다. 

여는 괄호는 괄호 내부 연산의 시작이므로 결합력은 0으로 초기화 해준다.

닫는 괄호는 시작 부분에 올 수 없고, 여는 괄호의 끝에 존재해야하기에 Atom구문 다음 연산자 부분에 추가 해준며, loop를 끊어 줘야 괄호 내부의 연산이 끝났음을 알 수 있다. 

여는 괄호를 발견한 경우 닫는 괄호를 발견해 재귀 호출이 반환되었는지 확인해야한다. 그렇기 때문에 assert_eq를 이용해 기대값이 ‘)’ 인지 확인한다. 이렇게 하지 않으면 “(a + b * c” 열린 괄호만 있고 닫힌 괄호는 없는 식이 표현식이 되기 때문이다.

## 연산자 추가하기

```rust
fn infix_binding_power(op: char) -> (f32, f32){
	match op{
		'+' | '-' => (1.0, 1.1),
		'*' | '/' => (2.0, 2.1),
		'^' | '√' => (3.0, 3.1),
		_ => panic!("unknown operator: {:?}", op)
	}
}
```

연산자를 추가하더라도 핵심 논리는 변하지 않는다. 연산자를 infix_binding_power에 추가하고 결합력 값만 세팅하면 되기 때문이다. 

## 평가

<img width="216" alt="7" src="https://github.com/user-attachments/assets/b302a869-4ee4-4363-b56f-58e03d1b0193" />
