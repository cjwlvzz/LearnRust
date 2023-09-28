
struct Struct {
    e: i32
}

fn main() {
    //------不可变变量绑定------
    // let a = 6;
    // println!("a = {}", a);
    // a = 7;
    // println!("a = {}", a);

    //------可变变量绑定------
    // let mut a = 6;
    // println!("a = {}", a);
    // a = 7;
    // println!("a = {}", a);

    //------未被使用的变量绑定------
    // let _y = 6;
    // let _x = 5;

    //------变量的解构------
    // let (a, mut b): (bool,bool) = (false, false);
    // // a = true,不可变; b = false，可变
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;//这里会触发断言
    // assert_eq!(a, b);


    //-------解构式赋值------
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    println!("e = {}", e);
    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);


}
