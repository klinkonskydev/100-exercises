# Crie um programa que leia o dinheiro em real e converte para Dólares, considere U$1,00 = R$3,27
dollar_price = 3.27

while True:
    entry = input("Quanto dinheiro você tem na carteira? R$")
    try: 
        real = float(entry)
        break
    except ValueError:
        print("ERROR: Digite um numero valido")

print("Com R${} você pode comprar R${:.2f}".format(real, real / 3.27))
