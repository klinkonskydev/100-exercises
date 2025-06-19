# Crie um algoritmo que leia um número e mostre o seu dobro, triplo e raiz quadrada.
number = int(input("Digite um numero: "))
print("O dobro do numero {} é {}".format(number, (number * 2)))
raiz_quadrada = pow(number, (2/1))
print("O triplo do numero {} é {}. \n a raiz quadrada vale {}".format(number, (number * 3), raiz_quadrada))
