#![windows_subsystem = "windows"]
extern crate imagesize;
extern crate winapi;

mod mensagem;
pub use self::mensagem::cx_msg;
use imagesize::size;
use std::cmp::Ordering::*;
use std::env;

const ROTULO: &str = "Verificar o tamanho da foto...";
const RESPOSTA: [&str; 4] = [
    "10x15 ou 20x30.",
    "3x4 ou 30x40.",
    "20x25.",
    "5x7 ou 15x21.",
];

struct VerTam {
    foto: String,
    dimensoes: (i32, i32),
    resposta: &'static str,
}

impl VerTam {
    fn obter_dados() -> Self {
        if env::args().len() != 2 {
            let _ = mensagem::cx_msg(ROTULO, "Favor entrar com um arquivo.");
            panic!();
        };
        let foto = env::args()
            .nth(1)
            .unwrap()
            .split("\\")
            .last()
            .unwrap()
            .into();
        let dim = size(&foto).unwrap();
        Self {
            foto,
            dimensoes: (dim.width as i32, dim.height as i32),
            resposta: "",
        }
    }
    fn comparar(&mut self) -> &Self {
        match self.dimensoes.0.cmp(&self.dimensoes.1) {
            Greater => {
                self.dimensoes = (self.dimensoes.1, self.dimensoes.0);
                self.calcular();
            }
            Less => self.resposta = self.calcular(),
            Equal => self.resposta = "quadrado.",
        }
        self
    }
    fn calcular(&self) -> &'static str {
        [2, 3, 4, 5, 7]
            .windows(2) // Forma os PARES: [[2, 3],[3, 4],[4, 5],[5, 7]]
            .enumerate()
            .find(|f| self.dimensoes.0 as isize / f.1[0] - self.dimensoes.1 as isize / f.1[1] == 0)
            .map_or("desconhecido.", |r| RESPOSTA[r.0])
    }
    fn responder(&self) {
        let resposta = format!("O tamanho da foto < {} > é {}", self.foto, self.resposta);
        let _ = mensagem::cx_msg(ROTULO, &resposta);
    }
}

fn main() {
    VerTam::obter_dados().comparar().responder();
}

#[cfg(test)]
mod teste {
    use VerTam;
    #[test]
    fn teste() {
        let mut vertam = VerTam {
            foto: "Foto_A308.jpg".into(),
            dimensoes: (1200, 1680),
            resposta: "",
        };
        vertam.comparar().responder();
    }
}
