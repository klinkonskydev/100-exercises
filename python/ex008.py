# leia um valor em metro e a exiba convertido em centimetros e milimetros.
while True:
    entry = input("Uma distância em metros: ")
    try:
        distance = float(entry)
        break
    except ValueError:
        print("Por favor, digite um número válido.")

print("A média de {} corresponse a \n{:.3f}km\n{:.2f}hm\n{:.1f}dam\n{}dm\n{}cm\n{}mm".format(
    distance,
    distance / 1000, # Kilometros
    distance / 100, # Hectometros
    distance / 10, # Decametros
    distance * 10, # Decimetros
    distance * 100, # Centimetros
    distance * 1000 # Milimetros
))
