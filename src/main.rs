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
    Asanki(String),
    Toy(u32),
    Kametsa(f32),
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
               || self.texto[self.pos] == '\'' || self.texto[self.pos] == '.') {
            self.pos += 1;
        }

        let palabra: String = self.texto[inicio..self.pos].iter().collect();

        if self.texto[self.pos-1] == '\'' {
            Some(Token::Asanki(palabra))
        } else if palabra == "toy" {
            Some(Token::ToyKeyword)
        } else if palabra == "kametsa" {
            Some(Token::KametsaKeyword)
        } else if palabra == "asanki" {
            Some(Token::AsankiKeyword) 
        } else if palabra.contains('.') {
            Some(Token::Kametsa(palabra.parse::<f32>().expect("REASON"))) 
        } else if palabra.chars().all(|c| c.is_ascii_digit()) {
            Some(Token::Toy(palabra.parse::<u32>().expect("REASON")))
        } else if palabra == "="{
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

    println!("Código: \n{}", codigo);
    println!("\nAnálisis:");

    let mut lexer = Lexer::nuevo(&codigo);
    println!("{:?}\n",lexer.texto);
    let mut contador = 1;
    
    while let Some(token) = lexer.obtener_token() {
        match token {
            Token::ToyKeyword => println!("  {}. Palabra reservada: toy",contador),
            Token::KametsaKeyword => println!("  {}. Palabra reservada: kametsa", contador),
            Token::AsankiKeyword => println!("  {}. Palabra reservada: asanki",contador),
            Token::Variable(nombre) => println!("  {}. Variable: {}", contador, nombre),
            Token::Asanki(valor)=> println!("  {}. Valor de tipo Asanki: {}",contador,valor),
            Token::Kametsa(valor) => println!("  {}. Valor de tipo Kametsa: {}",contador,valor),
            Token::Toy(valor) => println!("  {}. Valor de tipo Toy: {}",contador,valor),
            Token::Equal => println!("  {}. '='",contador),
        }
        contador += 1;
    }
}
