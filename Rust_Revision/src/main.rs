//                      3.1     Shadowing                          //
// fn main() {

//     println!(" Examples of Shadowing  \n     3.1 ");
//     let name = "Zakariya Khan";
//     println!("Hello {}, How are you",name);

//     // now printing another name by using same variable

//     let name = "yahya Khan";
//     println!("and where is {}",name);

// }

//    Data types
  
  // Scalar Data type
  // Integer, Float, Character, Bool
  
  
  // Compound Data Type
  // Tuple, Arrey
    


    //   //                    3.2    Tupple                        //
    // we can store multiple values of different types in tupple. Once the tupple 
    // is innitialized it cannot be extended 

// fn main() {    

    // println!(" Examples of Tupple \n     3.2 ");

    // //  Using tupple for saving student information //

    // //                Name                Age      ROll no            GPA  //
    // let tupname =( String::from("Uzair"),  45,       125,             3.5);
    // //    index               0             1          2               3
    
    // // We use indexing to print our values //
    // println!("second value {} Third value {}",tupname.1,tupname.3);
    // println!("{:#?}",tupname);  // WILL PRINT WHOLE TUPPLE

    //                 now indexing tupple with key                //

    // let (Name,Age,Roll_No,GPA)  = tupname;
    // println!("Name of a student {} \nRoll no of {} {} ",Name , Name, Roll_No );

    // By initializing keys to tupple now u just have to write the key and
    //  u will get the detail u want now u dont have to remember index
// }

          
    //                           3.2    Arey                                    //
    // //We can store multiple values but not of different types thier type should 
    // // be same
    // // Like tupple it also has the fixed length and cannot be change later

// fn main (){
    // println!(" Examples of Array \n     3.2 ");
    // let name_of_arrey = [90,20,25,45,22];
    // println!("First value = {},Third value = {}", name_of_arrey[0],name_of_arrey[2]);

    // let [Maths,Science,English,Islamiat,History] = name_of_arrey;
    // println!("{}",Maths );

// }


// fn main() {

    // // Arithmetic Operations
    // //  +, _, *, /, %
    // //  We can apply all these operators on same type not on different types. " % " This operator will show the remainder 
  
    // println!(" Examples of Arithmetic  \n         Operator    ");    
    // let number_1 = 30;
    // let number_2 = 4;
    
    // println!("Addition {}",number_1 + number_2);
    
    // println!("Subtraction {}",number_1 -number_2);
  
    // println!("Multiplication {}",number_1 * number_2);
    
    // println!("Division {}",number_1 / number_2);
  
    // println!("Remainder {}",number_1 % number_2);
  
// }
  
    
  
  // // Function
  
  // // It help to remove Duplication of Code or Repitation of Code
  // // Its easy to  maintain 
  // // It saves time
  // // WE write set of code in it which we want to use frequently in our program
  
  //  // // // //        Creating Function
  
  // fn hair_style_1(){
  //   let front = 7; // cm
  //   let back  = 3; // cm
  //   let right_side = 4; // cm
  //   let left_side = 2; // cm
  
  //  println!("Hair_Style_1 \nFront {} cm \nBack {} cm \nRight side {} cm \nLeft side {} cm",front,back,right_side,left_side);
  // }            // // Created // 
  
  // //    //  Now Calling it, We will just write name of our function in our main function and we can call unlimited times
  // fn main(){
  // hair_style_1();
  // hair_style_2();
  // }
  // // now creating same function again by using Shadowing and Array
  
  // fn hair_style_2(){
  // //  //     Initiating Array
  //   let lenght_of_hairs = [6,3,3,3];         //  Shadowing   and Array
  //   //  Now keys
  //   let [front,back,right_side,left_side] = lenght_of_hairs;        
  
  //  println!("Hair_Style_2\nFront {} cm\nBack {} cm\nRight side {} cm\nLeft side {} cm",front,back,right_side,left_side);
  // }            // Created

  //                    // Calling   //
  // hair_style_2();



// //  // Now function with parameters and arguments

//                    //Creating function
////                  ->-------->------->Parameters
//                    |        |     
//   fn multiplication(a:i32, b:i32){   
//     let result = a*b;                 
//     println!("{}",result );          
//   }                    

// //                           created      


// // //           >->--->  Arguments   
// //  //          | |                  
// multiplication(10,5);                
                                     

// // another way of creating function with return value

// // //               ->------>------->Parameters
// // //               |      |     
// fn multiplication_2(a:i32, b:i32)-> i32 {
//  let c = a*b;
// //  println!("{}",c);
// c
// }


// add


// fn add_and_subtract(a:i32,b:i32,)->(i32,i32){
//     (a+b,a-b)
// }

// let (result_of_add,result_of_subtract) =add_and_subtract(44,66);
// println!("After add {}\nAfter subtract {}",result_of_add,result_of_subtract );




// fn multiply_and_divide(a:i32,b:i32,)->(i32,i32){
//   (a*b,a/b)
// }

// let (result_of_add,result_of_subtract) =multiply_and_divide(20,5);
// println!("After add {}\nAfter subtract {}",result_of_add,result_of_subtract );





// fn add_subtract_multiply_and_divide(a:i32,b:i32){
//   let add = a+b;
//   let subtract = a-b;
//   let multiply = a*b;
//   let divide = a/b;

//   println!("ADD {} \nSubtract {} \nMultiply {} \nDivide {}",add,subtract,multiply,divide);

// }



// add_subtract_multiply_and_divide(5,7);
// }

// use std::io;

// fn main() {

// println!("Which number should be add in  100? So when we divide it by 4, The answer will be 30 and also check the answer is even or odd.");
//  let mut attempts = 3;
// loop{
//   println!("Attempts left {}  ",attempts );
//   attempts -=1;

//   let mut answer = String::new();
//   io::stdin()
//   .read_line(&mut answer)
//   .expect("Failed to read line");
//   let variable = answer.trim().parse::<i32>().unwrap();
  
//   check(variable);
  
//   if variable == 20 {
//     break
//   println!("Congrats your answer is correct",)
//   }else{
//   println!("Try Again")
//   }
   
//   if attempts == 0 {
//     break
//   }
       

// };

// }
    
  
// // fn game(x:i32)->i32{

  
// //   let y = 20;
  
// //   if x==y{
// //     println!("Your Answer is Correct");
// //   }else{
// //     println!("Your Answer is Wrong");
// //   }

// //   y
// // }


// fn check(y:i32){
  
// if y% 2 == 0{
//   println!("Your input is Even");
// }else{
//   println!("your input is Odd");
//   };
  
// }

 
//        While Loop


// fn main(){

//   // let (name,Roll_No,Grade,Marks) =(String::from("Zakariya Khan"),28,'A',950);
//   let student_1 = [20,950,80];
  
//   let mut counter = 0;
//   while counter < student_1.len() {
//   println!("{}",student_1[counter] );
//     counter +=1;
//   }
// }





// use std::io;

// fn main() {

// println!("Which number should be add in  100? So when we divide it by 4, The answer will be 30 and also check the answer is even or odd.");
//  let mut attempts = 3;
// while attempts > 0{
//   println!("Attempts left {}  ",attempts );
//   attempts -=1;

//   let mut answer = String::new();
//   io::stdin()
//   .read_line(&mut answer)
//   .expect("Failed to read line");
//   let variable = answer.trim().parse::<i32>().unwrap();
  
//   game(variable);
//   check(variable);
  
//   if variable == 20 {
//     break
//   }else{
//   println!("Try Again")
//   }
   
//   if attempts == 0 {
//     break
//   }
       

// };

// }
    
  
// fn game(x:i32)->i32{

  
//   let y = 20;
  
//   if x==y{
//     println!("Your Answer is Correct");
//   }else{
//     println!("Your Answer is Wrong");
//   }

//   y
// }


// fn check(y:i32){
  
// if y% 2 == 0{
//   println!("Your input is Even");
// }else{
//   println!("your input is Odd");
//   };
  
// }

//  For Loop



use std::io;

fn main() {

  println!("Which number should be add in  100? So when we divide it by 4, The answer will be 30 and also check the answer is even or odd.");
 
for attempts in (1..4).rev()  {
  println!("Attempts left {} ",attempts );
  

  let mut answer = String::new();
  io::stdin()
  .read_line(&mut answer)
  .expect("Failed to read line");
  let variable = answer.trim().parse::<i32>().unwrap();
  
  game(variable);
  check(variable);
  
  if variable == 20 {
    break
  }else{
  println!("Try Again")
  }
   
  if attempts == 0 {
    break
  }
       

};


let arrey = [22,44,55,22,11];

for elements in arrey.iter(){
  println!("{}",elements );
}



let mut variable = String::from("");
for i in 0..5 {
  variable.push('*');
  println!("{}",variable );
}

for i in 1..5{
  variable.pop();

  println!("{}",variable );
}

}
    
  
fn game(x:i32)->i32{

  
  let y = 20;
  
  if x==y{
    println!("Your Answer is Correct");
  }else{
    println!("Your Answer is Wrong");
  }

  y
}


fn check(y:i32){
  
if y % 2 == 0{
  println!("Your input is Even");
}else{
  println!("your input is Odd");
  };
  

}

