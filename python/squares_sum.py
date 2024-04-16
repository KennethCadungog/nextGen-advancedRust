import sys
sys.path.append("/Users/kennethcadungog/advanced_rust/pro_rust/target/release")
import pro_rust  

numbers:list[int] = [1, 2, 3, 4, 5]
result:int = pro_rust.sum_of_squares_python(numbers)
#print("Sum of squares: ", result)

def sum_of_squares(numbers:list[int]) -> int:
    return sum(x ** 2 for x in numbers)