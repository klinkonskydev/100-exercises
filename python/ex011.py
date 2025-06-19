# Faça um programa que leia a largura e altura de uma parede, calcule a área e a quantidade de tinta necessário para pintá-lo
# sabendo que cada litro de tinta, pinta uma área de 2m²
def read_float(prompt):
    while True:
        entry = input(prompt)
        try:
            return float(entry)
        except ValueError:
            print("Type a valid number.")

width = read_float("Largura da parede: ")
height = read_float("Altura da parede: ")

area = width * height
tint = area / 2

print(f"Sua parede tem a dimensão de {width:.2f}x{height:.2f} e sua área é de {area:.2f}m2")
print(f"Para pintar essa parede, você precisa de {tint:.2f}l de tinta.")
