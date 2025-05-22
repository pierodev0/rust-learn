// fn main() {
//     println!("Hello, world!");
//     another_functions(5);
// }
// fn another_functions(x: i32){
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_meausurement(5, 'h');
// }
// fn print_labeled_meausurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main(){
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {y}")
// }

//----Funciones con valores de retorno
// fn five() -> i32 {
//     5 //Valor de retorno de la funcion
// }
//
// fn main() {
//     let x = five();
//     println!("The value of x is: {x}")
// }


fn main(){
    let x = plus_one(5);
    println!("The value of x is: {x}");
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
