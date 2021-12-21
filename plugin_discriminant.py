import math

'''
Это плагин для подсчета математического квадратного корня 
его можно использоватья отдельно с помощью функции solve разроботано для души alexzai007

This is a plugin for calculating the mathematical square root,
 it can be used separately using the solve function designed for the soul alexzai007
'''
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