use std::fs;
use std::env;

#[derive(Debug)]
enum Token {
    //int = toy; float = kametsa; double = kametsa_ints; bool = iri; string = asanki; char = pitsi;
    //void = maika
    AsankiKeyword,
    ToyKeyword,
    KametsaKeyword,
    Variable(String),
    Valor(String),
    Equal,
}

struct Lexer {
    texto: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn nuevo(entrada: &str) -> Self {
        Lexer {
            texto: entrada.chars().collect(),
            pos: 0,
        }
    }

    fn obtener_token(&mut self) -> Option<Token> {
        
        while self.pos < self.texto.len() && self.texto[self.pos].is_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.texto.len() {
            return None;
        }

        let inicio = self.pos;
        while self.pos < self.texto.len() && 
              (self.texto[self.pos].is_alphanumeric() || self.texto[self.pos] == '=' 
               || self.texto[self.pos] == '\'') {
            self.pos += 1;
        }

        let palabra: String = self.texto[inicio..self.pos].iter().collect();

        if self.texto[self.pos-1] == '\'' {
            return Some(Token::Valor(palabra))
        }

        if palabra == "toy" {
            Some(Token::ToyKeyword)
        } else if palabra == "kametsa" {
            Some(Token::KametsaKeyword)
        } else if palabra == "asanki" {
            Some(Token::AsankiKeyword) 
        }else if palabra == "="{
            Some(Token::Equal)
        }else {
            Some(Token::Variable(palabra))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let codigo = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("No se pudo leer el archivo")
    } else {
        "toy x".to_string()
    };

    println!("Código: {}", codigo);
    println!("\nAnálisis:");

    let mut lexer = Lexer::nuevo(&codigo);
    println!("{:?}",lexer.texto);
    let mut contador = 1;
    
    while let Some(token) = lexer.obtener_token() {
        match token {
            Token::ToyKeyword => println!("  {}. Palabra reservada toy",contador),
            Token::KametsaKeyword => println!("  {}. Palabra reservada kametsa", contador),
            Token::Variable(nombre) => println!("  {}. Variable: {}", contador, nombre),
            Token::Equal => println!("  {}. '='",contador),
            Token::Valor(valor)=> println!("  {}. Valor {}",contador,valor),
        }
        contador += 1;
    }
}
