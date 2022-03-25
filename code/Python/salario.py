# Crie um programa que leia o nome e o salário de um funcionário, mostrando no
# final uma mensagem.
# Ex:
# Nome do Funcionário: Maria do Carmo
# Salário: 1850,45
# O funcionário Maria do Carmo tem um salário de R$1850,45 em Junho.

# casting = conversão
nome = input("Nome do Funcionário:")
salario = int(input("Salário:")) # Retorna um texto
print("O funcionário {} tem um salário de R${} em abril".format(nome, salario))