import p5 from "p5";
import { Simulation } from "pkg";

// Esse é o número de cargas, passado para o Rust.
const numCharges = 2048;

// Através de muita magia negra, isso importa o código em Rust.
import("../pkg").then(({ Simulation }) => {
  // Aqui, criamos o p5 em modo instancia, como em
  // https://p5js.org/examples/instance-mode-instantiation.html.
  const myP5 = new p5(p => {
    // Esse será o objeto da simulação. Tudo roda dentro dele, em Rust.
    let sim: Simulation;

    p.setup = () => {
      p.createCanvas(window.innerWidth, window.innerHeight);
      // Aqui, eu construo o objeto, que chama o Simulation::new(argumentos...)
      // do Rust.
      sim = new Simulation(
        p,
        window.innerWidth,
        window.innerHeight,
        numCharges
      );
    };

    p.draw = () => {
      p.clear();
      p.background("black");

      // E em cada frame eu rodo.
      sim.run();
    };
  }, document.getElementById("container"));
});
