# Simulação de partículas elétricas

Este repositório contém o código de uma simulação de partículas elétricas
interagindo em 2D, feito com Electron, p5.js, e Rust (via WebAssembly). O
algoritmo, que pode ser análisado em "src/lib.rs", função "move_particle",
é o seguinte:

1. Se a partícula não está encostada na parede, adicione sua velocidade à
   sua posição. Se está, ou já passou da parede, limite ela na parede.

2. Para cada partícula na lista de partículas, menos a atual, calcule a força
   elétrica entre as duas.

3. Some a aceleração dada pela razão entre a força elétrica e massa da
   partícula à velocidade.

4. Repita para cada frame.

A entrada do JavaScript é no arquivo "js/index.ts". Não é bem JavaScript, é
TypeScript. É a mesma coisa, porém com checagem de tipos. Esse arquivo só
serve para ficar chamando o Rust para desenhar os frames com o p5.js.

## Executar

Para executar, é necessário instalar o [Node.js](https://nodejs.org/en/).
Também a gerenciador da [linguagem Rust](https://github.com/rust-lang/rustup).
Depois, abra o cmd na pasta e rode `npm install`.

Vá na pasta com a linha o cmd e rode `npm start` para executar.
