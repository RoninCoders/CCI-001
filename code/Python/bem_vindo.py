#  Faça um programa que leia o nome de uma pessoa e mostre uma mensagem de boasvindas para ela:
# Ex:
# Qual é o seu nome? João da Silva
# Olá João da Silva, é um prazer te conhecer!

# print(); # Ela vai escrever na tela as informacoes que a gente quer
# input(); # Ela vai ler a entrada do teclado

print("Qual é o seu nome?")
nome = input() # valor que vem do teclado
print("Olá {}, É um prazer te conhecer".format(nome))