// Esses são os imports em Rust.
use rand::{
    distributions::{uniform::Uniform, Distribution},
    thread_rng,
};
use std::f64::{self, consts::PI};
use wasm_bindgen::prelude::wasm_bindgen;

// Dessa parte até FIM DE HOGWARTS é bruxaria para comunicar
// com o JavaScript.
#[wasm_bindgen(module = "p5")]
extern "C" {
    pub type Color;
}

#[wasm_bindgen]
extern "C" {
    pub type P;

    #[wasm_bindgen(method)]
    pub fn push(this: &P);

    #[wasm_bindgen(method)]
    pub fn pop(this: &P);

    #[wasm_bindgen(method)]
    pub fn stroke(this: &P, color: &Color);

    #[wasm_bindgen(method)]
    pub fn color(this: &P, color: &str) -> Color;

    #[wasm_bindgen(method, js_name = "lerpColor")]
    pub fn lerp_color(this: &P, color1: &Color, color2: &Color, amt: f64) -> Color;

    #[wasm_bindgen(method)]
    pub fn fill(this: &P, color: &Color);

    #[wasm_bindgen(method)]
    pub fn circle(this: &P, x: f64, y: f64, r: f64);
}
// FIM DE HOGWARTS

/// Constante K_0, calculada com uma precisar legal a partir
/// da permissividade elétrica do vácuo.
/// static é para declarar uma contante que pode ser acessada
/// de todo o programa, imutávelmente. Ela possui tipo f64,
/// ou seja, um valor com vírgula de 64 bits na memória.
static K_0: f64 = 1.0 / (4.0 * PI * 8.854e-12);

/// A estrutura que representa uma partícula. O pub é para
/// exportar para fora desse arquivo, para o JavaScript poder
/// acessar. O #[derive(Clone)] é um macro, quer dizer que este
/// tipo pode ser clonado de uma variável para a outra.
#[derive(Clone)]
pub struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    charge: f64,
    mass: f64,
}

/// Aqui eu defino os métodos de Particle.
impl Particle {
    /// Esse é uma método estático, acessível a partir da struct, como
    /// Particle::new(argumentos...). É como se fosse o constructor do
    /// JavaScript.
    pub fn new(x: f64, y: f64, charge: f64, mass: f64) -> Particle {
        // Em Rust, se você não coloca o ; no final da expressão ele retorna
        // o valor. Isso aqui é um return.
        Particle {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            charge,
            mass,
        }
    }

    /// Move a partícula de acordo com a atração das outras, iterativamente.
    /// É chamada a cada renderização.
    /// &mut self é o this do JavaScript, e o mut quer dizer que podemos mudar
    /// o objeto.
    /// index é o índice dessa partícula na lista.
    /// particles é a lista de partículas.
    /// size_x e size_y são os tamanhos da tela.
    pub fn move_particle(
        &mut self,
        index: usize,
        particles: &[Particle],
        size_x: f64,
        size_y: f64,
    ) {
        // Se está fora da tela pela direita, limite na borda direita da tela.
        if self.x >= size_x / 2.0 {
            self.x = size_x / 2.0;
        // Se está fora da tela pela esquerda, limite na borda esquerda da tela.
        } else if self.x <= -size_x / 2.0 {
            self.x = -size_x / 2.0;
        // Se está dentro da tela, some a velocidade à posição.
        } else {
            self.x += self.vx;
        }
        // Se está fora da tela para baixo, limite na borda inferior da tela.
        if self.y >= size_y / 2.0 {
            self.y = size_y / 2.0;
        // Se está fora da tela para cima, limite na borda superior da tela.
        } else if self.y <= -size_y / 2.0 {
            self.y = -size_y / 2.0;
        // Se está dentro da tela, some a velocidade à posição.
        } else {
            self.y += self.vy;
        }

        // Itera para cada partícula que está na lista de partículas.
        for (i, particle) in particles.iter().enumerate() {
            // Se o índice de iteração atual (i) é igual ao índice dessa partícula (index),
            // passado a nós pelo chamador, pule a iteração. Da mesma forma, se a carga da
            // partícula for 0.
            if i == index || particle.charge == 0.0 {
                continue;
            }

            // Calcula o vetor da partícula analisada até nós.
            let r_vector_x = self.x - particle.x;
            let r_vector_y = self.y - particle.y;

            // Pega a distância até a partícula ao quadrado.
            // Não tirei a raiz aqui porque depois vou usar esse valor,
            // com o quadrado.
            let r_squared = r_vector_x.powi(2) + r_vector_y.powi(2);

            // Essa é a parte escalar da Lei de Coulumb.
            let scalar = K_0 * (particle.charge * self.charge / r_squared);

            // Divide cada coordenada do vetor pela sua magnitude, ou seja,
            // pega o versor. Em seguida, multiplica pela parte escalar.
            let fe_x = scalar * r_vector_x / r_squared.sqrt();
            let fe_y = scalar * r_vector_y / r_squared.sqrt();

            // Adiciona a aceleração calculada a partir da força
            // à aceleração.
            self.vx += fe_x / self.mass;
            self.vy += fe_y / self.mass;
        }
    }

    /// Desenha a partícula na tela.
    /// É chamado em cada renderização.
    /// p é objeto do p5 no JavaScript, onde existem as funções de desenho.
    pub fn draw_particle(&self, p: &P, size_x: f64, size_y: f64) {
        // Cria um novo contexto de desenho.
        p.push();
        p.stroke(&p.color("white"));
        // Interpola a cor de azul a vermelho, de acordo com a carga.
        let color = p.lerp_color(&p.color("blue"), &p.color("red"), (self.charge + 1.0) / 2.0);
        p.fill(&color);
        // Desenha a bolina.
        p.circle(
            size_x / 2.0 + self.x,
            size_y / 2.0 + self.y,
            (self.mass - 0.5e3 + 1.0) * 64.0 / 49_999_500.0,
        );
        // Volta ao contexto de desenho anterior.
        p.pop();
    }
}

/// É o objeto da simulação, exportado ao JavaScript.
/// Esses macros #[wasm_bindgen] dizem para o compilador
/// como exportar.
/// Esse objeto guarda os valores provenientes do JavaScript
/// que nos interessam, como o objeto do p5.js e os tamanhos,
/// além da lista de partículas. Fiz isso para não evitar
/// Ficar passando esses objetos de um lado para o outro,
/// eu passo uma só vez e os mantenho aqui, para melhorar
/// a performance.
#[wasm_bindgen]
pub struct Simulation {
    p5: P,
    size_x: f64,
    size_y: f64,
    particles: Vec<Particle>,
}

/// Aqui eu defino os métodos de Simulation.
#[wasm_bindgen]
impl Simulation {
    // Esse new também é como um constructor, e, de fato, por causa desse
    // #[wasm_bindgen(constructor)] ele vai ser para o JavaScript o
    // constructor da classe Simulation. Ele recebe os valores e inicializa
    // a lista de partículas com valores aleatórios.
    #[wasm_bindgen(constructor)]
    pub fn new(p5: P, size_x: f64, size_y: f64, num_particles: u32) -> Simulation {
        // Cria um novo vetor. (mutável, pode ser alterado depois.)
        let mut particles = Vec::new();
        // Cria um gerador aleatório.
        let mut rng = thread_rng();

        // Esses quatro dist_[alguma coisa] são objetos que permitem
        // pegar um número aleatório num intervalo inclusivo.
        let dist_x = Uniform::new_inclusive(-size_x / 2.0, size_x / 2.0);
        let dist_y = Uniform::new_inclusive(-size_y / 2.0, size_y / 2.0);
        let dist_charge = Uniform::new_inclusive(-1.0, 1.0);
        let dist_mass = Uniform::new_inclusive(0.5e3, 0.5e8);

        // Esse é um for, que vai repetir num_particles vezes.
        // Não estamos interessados no índice, por isso o _.
        for _ in 0..num_particles {
            // Cada new_[alguma coisa] recebe um valor aleatório dentro do intervalo
            // definido acima para a variável. Em seguida, criamos uma partícula e
            // empurramos para o fim da lista.
            let new_x = dist_x.sample(&mut rng);
            let new_y = dist_y.sample(&mut rng);
            let new_charge = dist_charge.sample(&mut rng);
            let new_mass = dist_mass.sample(&mut rng);
            particles.push(Particle::new(new_x, new_y, new_charge, new_mass));
        }
        Simulation {
            p5,
            size_x,
            size_y,
            particles,
        }
    }

    /// Essa é a função que precisa ser chamada a cada draw do p5.
    /// Ela move cada partícula e desenha elas.
    pub fn run(&mut self) {
        // Tiro um clone imutável das partículas antes de passar por referência
        // para a função move_particle. Isso é necessário porque move_particle
        // vai alterar a partícula, e por consequência a lista de partículas.
        // Então precisamos clonar e enviar a lista inalterada para operar os cálculos.
        let particles_clone = self.particles.clone();
        for (i, particle) in self.particles.iter_mut().enumerate() {
            particle.move_particle(i, &particles_clone[..], self.size_x, self.size_y);
            particle.draw_particle(&self.p5, self.size_x, self.size_y);
        }
    }
}
