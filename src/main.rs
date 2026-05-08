use std::fs;
use std::env;

#[derive(Debug)]
enum Token {
    // Palabras reservadas
    AsankiKeyword,
    ToyKeyword,
    KametsaKeyword,
    
    // Valores
    Asanki(String),
    Toy(u32),
    Kametsa(f32),
    
    // Otros
    Variable(String),
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
        self.saltar_espacios();

        if self.esta_al_final() {
            return None;
        }

        let inicio = self.pos;
        self.avanzar_mientras_sea_valido();

        let palabra = self.obtener_palabra(inicio);

        self.clasificar_token(palabra)
    }


    fn saltar_espacios(&mut self) {
        while self.pos < self.texto.len() && self.texto[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn esta_al_final(&self) -> bool {
        self.pos >= self.texto.len()
    }

    fn avanzar_mientras_sea_valido(&mut self) {
        while self.pos < self.texto.len() && self.es_caracter_valido(self.texto[self.pos]) {
            self.pos += 1;
        }
    }

    fn es_caracter_valido(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '=' || c == '\'' || c == '.'
    }

    fn obtener_palabra(&self, inicio: usize) -> String {
        self.texto[inicio..self.pos].iter().collect()
    }


    fn clasificar_token(&mut self, palabra: String) -> Option<Token> {
        if self.termina_con_comilla() {
            return Some(Token::Asanki(palabra));
        }

        match palabra.as_str() {
            "toy" => return Some(Token::ToyKeyword),
            "kametsa" => return Some(Token::KametsaKeyword),
            "asanki" => return Some(Token::AsankiKeyword),
            "=" => return Some(Token::Equal),
            _ => {}
        }

        if palabra.contains('.') {
            let valor = palabra.parse::<f32>().expect("Error al parsear flotante");
            Some(Token::Kametsa(valor))
        } else if palabra.chars().all(|c| c.is_ascii_digit()) {
            let valor = palabra.parse::<u32>().expect("Error al parsear entero");
            Some(Token::Toy(valor))
        } else {
            Some(Token::Variable(palabra))
        }
    }

    fn termina_con_comilla(&self) -> bool {
        self.pos > 0 && self.texto[self.pos - 1] == '\''
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let codigo = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("No se pudo leer el archivo")
    } else {
        "asanki stri = 'Soviet'\ntoy ent = 4\nkametsa flo = 1.2".to_string()
    };

    println!("Código:\n{}", codigo);
    println!("\nAnálisis:");

    let mut lexer = Lexer::nuevo(&codigo);
    println!("{:?}\n", lexer.texto);

    let mut contador = 1;

    while let Some(token) = lexer.obtener_token() {
        match token {
            Token::ToyKeyword => println!("  {}. Palabra reservada: toy", contador),
            Token::KametsaKeyword => println!("  {}. Palabra reservada: kametsa", contador),
            Token::AsankiKeyword => println!("  {}. Palabra reservada: asanki", contador),
            Token::Variable(nombre) => println!("  {}. Variable: {}", contador, nombre),
            Token::Asanki(valor) => println!("  {}. Valor de tipo Asanki: {}", contador, valor),
            Token::Kametsa(valor) => println!("  {}. Valor de tipo Kametsa: {}", contador, valor),
            Token::Toy(valor) => println!("  {}. Valor de tipo Toy: {}", contador, valor),
            Token::Equal => println!("  {}. '='", contador),
        }
        contador += 1;
    }
}
