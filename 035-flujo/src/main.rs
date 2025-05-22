// ---- EXPRESION IF ----------
// fn main(){
//     let number = 7;
//
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main(){
//     let condition = true;
//     let number = if condition {5} else {6};
//
//     println!("The value of numbers is: {number}");
// }

// ------------ LOOP--------------
// fn main() {
//     loop {
//         println!("Again");
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {result}");
// }

// fn main(){
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }


// ------------Bucle while
// fn main(){
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}");
//
//         number -= 1;
//     }
//     println!("LISTOFF!!!");
// }

//-----------Recorrer arreglos con bucle while
// fn main(){
//     let a = [10,20,30,40,50];
//     let mut index = 0;
//
//     while index < a.len() {
//         println!("The value is: {}",a[index]);
//
//         index += 1;
//     }
// }


//-----------Bucle for para recorrer arreglo
// fn main(){
//     let a =  [10,20,30,40,50];
//
//     for element in a {
//         println!("The value is: {element}");
//     }
// }

/// Cuenta regresiva con for y metodo rev()
fn main(){
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
