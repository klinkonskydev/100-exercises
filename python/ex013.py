while True:
    entry = input("Qual e o salario do funcionario? ")
    try:
        amount = float(entry)
        break
    except ValueError:
        print("Please, type a valid amount.")

salary_increased = amount + ((amount * 15.0) / 100)
print("O novo salario do funcionario com 15% de aumento e de {:.2f}".format(salary_increased))
