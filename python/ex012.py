# Faça uma algoritimo que leia o preço de um produto e mostre se unovo preço, com 5% de desconto.
while True:
    entry = input("Product price: ")
    try: 
        product_price = float(entry)
        break
    except ValueError:
        print("Please, type a valid number")

five_percent_of_discount = (product_price * 5) / 100
product_with_discount = product_price - five_percent_of_discount

print("Your product with 5% of discount is R${:.2f}".format(product_with_discount))
