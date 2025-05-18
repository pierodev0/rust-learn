// fn main(){
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The values of x is: {x}");
// }

// fn main(){
//     let x = 5;
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The values of x is: {x}");
// }

// fn main(){
//     let guess: u32 = "42".parse().expect("NotKa number")
//     let number: u8 = 255;
// }


// El tipo caracter
// fn main() {
//     let c = "c";
//     let z = "Z"; // With explicit type annotation
// }


//El tipo tupla
// fn main(){
//     let tup: (i32,f64,u8) = (500,6.4,1);
//     let(x,y,z) = tup;
//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
//     println!("The value of z is: {z}");
// }

// EL TIPO ARREGLO
fn main(){
    let numbers = [1,2,3,4,5];

    let months = ["January","February","March","April","May", "June", "August", "September","October","November","December"];

    //el tipo de cada elemento y su longitud
    let a: [i32;5] = [1,2,3,4,5];

    let a = [3;5]; // Es lo mismo que let a = [3,3,3,3,3];


    //Acceder a los elementos de los arreglos
    let first = numbers[0];
}
