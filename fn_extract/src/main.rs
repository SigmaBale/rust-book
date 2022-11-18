fn main() {
    
    //Removing Duplication by Extracting a Function
    //How to remove duplication in a way that doesn’t involve generic types by extracting a function that 
    //replaces specific values with a placeholder that represents multiple values.
    //We begin with the short program that finds the largest number in a list.

    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest_num = number_list[0];

    for number in number_list {
        if number > largest_num {
            largest_num = number;
        }
    }

    println!("The largest number is {}", largest_num);

    
    
    //We've now been tasked with finding the largest number in two different lists of numbers. 
    //To do so, we can choose to duplicate the code and use the same logic

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest_num = number_list[0];

    for number in number_list {
        if number > largest_num {
            largest_num = number;
        }
    }

    println!("The largest number is {}", largest_num);

    
    
    //To eliminate this duplication, we’ll create an abstraction by defining a function that operates on any list of integers passed in a parameter.
    //We extract the code that finds the largest number into a function named largest. 
    //Then we call the function to find the largest number in the two lists.

    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
    
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    //1.Identify duplicate code.
    //2.Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
    //3.Update the two instances of duplicated code to call the function instead.

}
