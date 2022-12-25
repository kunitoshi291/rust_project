// fn main() {
//     let number = 9;

//     if number < 5 {
//         println!("condition was true");       // 条件は真でした
//     } else {
//         println!("condition was false");      // 条件は偽でした
//     }
// }

// fn main() {
//     let number = 3;
//     /* コンパイラは、bool型を予想したのに、int型なので、エラーになる */
//     // if number {
//     //     println!("number was three");
//     // }
//     if number != 0 {
//         println!("number was something other than zero"); // 数値は0以外の何かです
//     }
// }

// fn main() {
//     let number = 11;

//     if number % 6 == 0 {
//         // 数値は4で割り切れる
//         println!("数値は6で割り切れます");
//     }else if number % 5 == 0 {
//         println!("数値は5で割り切れます");
//     }else if  number % 4 == 0 {
//         // 数値は2で割り切れます
//         println!("数値は4で割り切れます");
//     } else {
//         // 数値は4、3、2で割り切れません
//         println!("数値は6、5、4で割り切れません");
//     }
// }

// match 
// fn main() {
//     let condition = false;
//     let number = if condition {5} else {6}
    
//     /*  ifブロックの式は整数に評価され、elseブロックの式は文字列に評価されます。
//     これでは動作しません。 変数は単独の型でなければならないからです。 */
//     // let number = if condition { 5 } else { "six" } ;

//     // numberの値は、{}です
//     println!("The value of number is: {}", number);
// }

// use std::num;

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// while文
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}", number);

//         number -= 1;
//     }
//     println!("LIFTOFF!!");
// }

// forでコレクションを覗き見
/* これだと、indexの長さが違えばプログラムがパニックになる。 
コンパイラが実行時にループの各回ごとに境界値チェックを行うようなコードを追加するから遅くなる。*/
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         // 値は{}です
//         println!("the value is: {}", a[index]);
//         index += 1; 
//     }
    
// }

fn main () {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}