# O que são algoritmos?

Um algoritmo é uma sequência de instruções ou comandos realizados de maneira sistemática com o objetivo de resolver um problema ou executar uma tarefa.

A palavra "algoritmo" faz referência ao matemático árabe Al Khowarizmi, que viveu no século IX, e descreveu regras para equações matemáticas.

Os algoritmos são como uma receita de bolo: uma sequência de ações que devem ser executadas até que o objetivo final - o bolo pronto - seja atingido.

Os algoritmos se aplicam das tarefas simples do dia a dia a programas computacionais complexos e ferramentas que identificam o comportamento do consumidor na internet.

Todas as funções dos computadores, smartphones e tablets, por exemplo, são resultado de algoritmos. Essas máquinas são capazes de realizar milhões de comandos em poucos segundos.
<hr>

### Exemplos básicos de funcionamento de algoritmos

Os algoritmos surgem na matemática para descrever as regras para as equações algébricas, mas eles podem ser aplicados a qualquer sequência de ações finitas que busquem a resolução de um problema.

Todas as tarefas que executamos no dia a dia podem ser transformadas em algoritmos, basta destrinchar todas as ações necessárias para se chegar ao objetivo determinado.

Se fizéssemos um algoritmo para o preparo do café, por exemplo, poderíamos ter as seguintes etapas:

1. ferver a água;
2. colocar o pó no filtro;
3. desligar a água;
4. passar a água pelo pó de café;

Os algoritmos utilizados em computadores são mais complexos e também envolvem possibilidades. Considere o exemplo de um mouse sobre um navegador de internet.

O comando de fechar a tela é dado por um algoritmo que entende que:
- se o mouse clicar no `"x"`, a página deve ser fechada;
- se o mouse não clicar no `"x"`, nada deve ser feito;

Esse algoritmo pode ser estruturado em um fluxograma:

![Preview](https://www.significados.com.br/foto/algoritmo-fluxograma.jpg)

Basicamente, os algoritmos são compostos por dados de entrada (**input**), processamento e dados de saída (**output**). Essa estrutura pode ser facilmente entendida com o exemplo de uma calculadora:

- **Dados de entrada**: valores e operações a serem realizadas
- **Processamento**: cáculos realizados pela máquina
- **Dados de saída**: resultado da operação


### Material de apoio

- [INTRODUÇÃO A ALGORITMOS E
PROGRAMAÇÃO](https://www.ferrari.pro.br/home/documents/FFerrari-CCechinel-Introducao-a-algoritmos.pdf)

### Primeiros algoritmos com o Portugol

- [Portugol WebStudio](https://portugol-webstudio.cubos.io/ide)

### Primeiro algoritmo com Portugol

Nosso primeiro algoritmo, que tem como funcionalidade calcular a área de um circulo:

```rs
programa {
	funcao inicio() {
		real PI = 3.14
		real raio
		leia(raio)
		real resultado
		resultado = PI * raio * raio
		escreva(resultado)
	}
}
```
Podemos conferir as implementações desse algoritmo em diferentes linguagens de programação:

- **C** - [raio.c](/code/C/raio.c)
- **Go** - [raio.go](/code/Go/raio.go)
- **Python** - [raio.py](/code/Python/raio.py)
- **Rust** - [raio.rs](/code/Rust/raio.rs)
- **Java** - [Raio.java](/code/Java/Raio.java)

### **Lista de exercícios sobre algoritmos básicos:**

Essa lista tem o propósito de incentivar o exercicio prático das atividades de programação.

- [Manipulação de textos e variáveis](https://docs.google.com/document/d/1QfvKHmw-zKMYBW5L4lAoiAOi_7Bhy4ycgPIdfxkFMpE/edit?usp=sharing)

Exercícios feitos sobre **input (stdin)**, **output (stdout)** e manipulação de variáveis.

- **C** - [salario.c](/code/C/salario.c)
- **Go** - [salario.go](/code/Go/salario.go)
- **Python** - [salario.py](/code/Python/salario.py)
- **Rust** - [salario.rs](/code/Rust/salario.rs)
- **Java** - [Salario.java](/code/Java/Salario.java)

