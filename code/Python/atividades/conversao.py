valor_dolar = float(input("Cotação do dolar:"))
valor_real = float(input("Valor em reais:"))

valor_convertido = valor_real / valor_dolar

print(f"O valor R${valor_real} em dolares americanos é US${valor_convertido}")