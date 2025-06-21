while True:
    entry = input("Informe a temperatura em ⁰C: ")
    try:
        celsius = float(entry)
        fahrenheit = 9 * celsius / 5 + 32
        break
    except:
        print("Digite uma temperatura valida.")

print("A temperatura de {}℃  corresponde a {}℉ ".format(celsius, fahrenheit))
