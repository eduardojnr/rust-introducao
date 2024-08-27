// Para compilar --> rustc <path>\main.rs
// Para executar --> <path>\main


// Com cargo --> é um gerenciador de projetos
// cargo new <projeto>
// cargo build 
// cargo run --> atualiza o projeto para o novo código


// É obrigatória a existência de uma função 'entrypoint' 


// É possível iniciar um novo escopo (para variáveis por exemplo)
// basta deixar o código desejado dentro de um bloco de chaves {}
// Nota: o escopo interno consegue acessar aquele que o envolve, o contrário não é permitido


// Declaração de variáveis
// let <nome_da_variavel>: <tipo> = <conteúdo>;  --> snake_case
// let <nome_da_variavel> = <conteúdo>_<tipo>;
// let mut [...];  --> mutável
// const <NOME_DA_CONSTANTE>: <tipo> = <conteúdo>  --> SCREAMING_SNAKE_CASE


// Tipos primitivos e suas declarações
// Escalares
    // i --> integer                 
        //8, 16, 32, 64, 128, isize (funciona de acordo com a arquitetura do SO... 32bits ou 64bits)
            // Fórmula para o intervalo de números:
            // -[2^(n-1)]  :  [2^(n-1)] - 1 || sendo N o tamanho em bits

    // u --> unsigned serve para declarar inteiros positivos (não possui sinal negativo)
        //8, 16, 32, 64, 128, usize (funciona de acordo com a arquitetura do SO... 32bits ou 64bits)
            // Fórmula para o intervalo de números:
            // 0  :  [2^n] - 1 || sendo N o tamanho em bits

    // f --> float
        //8, 16, 32, 64, 128, isize

    // bool --> booleano (true | false)

    // char --> dentro de aspas simples | caracter de até 4 bytes

    // &str --> dentro de aspas duplas
    
// Compostos
    // tuple --> permite armazenamento de vários tipos | tamanho fixo
        // let <tupla>: (i32, f64, bool) = (10, 10.5, true); ... -->
        // ... --> let (a: i32, b: f64, c: bool) = <tupla>
        // para acessar o conteúdo de um index --> <tupla>.<index>

    // array --> tipo definido | tamanho fixo | utiliza colchetes
        // let <array>: [i32;6] = [5, 10, 15, 20, 25];
        // para acessar o conteúdo de um index --> <array>.[<index>]


fn main() {
    
}

