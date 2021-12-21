import os
import math

#импортируем модуль для ввода цветны изображений
import colorama
from colorama import Fore, Back, Style
colorama.init()


#выводим что да как нужно орентироаться
print("-----------------------")
print("ax^2 * bx * c = 0")
print("-----------------------")
#спрашиваем у пользователя значения
a = int(input("Введите значение a: "))
b = int(input("Введите значение b: "))
c = int(input("Введите значение c: "))


#задаем пустышки для функции
x1 = 0
x2 = 0

'''
считаем дискременант по формуле 
D = B^2 * 4 * a * c
'''

resault1 = b * b
resault2 = 4 * a * c

D = resault1 - resault2


#главная функция
def solve(a, b, c, D,):

	#корень дискременанта
	d1 = math.sqrt(D)

	#для x1 и x2
	arg1_1 = -1 * b - d1
	arg1_2 = -1 * b + d1
	arg2 = 2 * a

	#если D = 0 значит x1 = x2 тоесть ответ один
	if d1 == 0:
		resault = arg1_1 / arg2
		return(str(resault))


	#если D больше 0 то у нас 2 ответа
	elif d1 > 0:
		x1 = arg1_1 / arg2
		x2 = arg1_2 / arg2
		return(str(x1) + str(" ") + str(x2))


	#Если D еньше 0 то у кровнения нет корней
	elif d1 < 0:
		return("Нет корней")



#вызываем решение уровнени
solve(a, b, c, D)




#Выводим все что от нас просилось
print("-----------------------")
print("Входные даные")
print("")
print("Параметр a: " + str(a))
print("Параметр b: " + str(b))
print("Параметр c: " + str(c))
print("")
print("-----------------------")
print("")
print(Fore.BLUE + "дискриминант: " + str(D))
print(Style.RESET_ALL)
print(Fore.GREEN + solve(a, b, c, D))
print(Style.RESET_ALL)
print("")
print("-----------------------")

#made by alexzai007 for 1506

'''
#только для переработки в exe
end = int(input("Нажмите ENTER"))
'''