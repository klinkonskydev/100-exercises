# Faça um programa que leila um número inteiro e mostre na tela o se sucessor e seu antecessor.
# usuario digita 5, antecessor é o 4 e sucessor é o 6
number = int(input("Digite um numero: "))
print("O antecessor do numero {} é {} e o sucessor é {}".format(number, number - 1, number + 1))
