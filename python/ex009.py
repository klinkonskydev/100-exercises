# Faça um programa que leia um número inteiro qualquer e mostre na tela a sua tabuada.
while True:
    entry = input("Digite um número: ")
    try:
        number = int(entry)
        break
    except ValueError:
        print("Por favor, digite um número válido.")

i = 1
while i < 11:
    print("{}x{} = {}".format(i, number, i * number))
    i += 1
