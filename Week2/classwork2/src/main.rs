fn main() {
   fn _add(x: i32, y: i32) {
      println!("{}",x + y)
   }

   fn add(){
        int x:i32;
        int y:i32;
        int c:i32;

        c = x+y;
        println!("The addition of these two numbers are {}", c);
   }

   let a:i32 = 4;
   let b:i32 = 3;

   _add(x:a, y:b);
   println!("{},{}",a,b);
   add(a,b);


}