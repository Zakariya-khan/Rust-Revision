      //                      3.1     Shadowing                          //
fn main() {

    println!(" Examples of Shadowing  \n     3.1 ");
    let name = "Zakariya Khan";
    println!("Hello {}, How are you",name);

    // now printing another name by using same variable

    let name = "yahya Khan";
    println!("and where is {}",name);

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

    println!(" Examples of Tupple \n     3.2 ");
    //  Using tupple for saving student information //
    //                Name                Age      ROll no            GPA  //
    let tupname =( String::from("Uzair"),  45,       125,             3.5);
    //    index               0             1          2               3
    
    // We use indexing to print our values //
    
    println!("second value {} Third value {}",tupname.1,tupname.3);

    //                 now indexing tupple with key                //

    let (Name,Age,Roll_No,GPA)  = tupname;
    println!("Name of a student {} \nRoll no of {} {} ",Name , Name, Roll_No );

    // By initializing keys to tupple now u just have to write the key and
    //  u will get the detail u want now u dont have to remember index
// }

          
    //                           3.2    Arey                                    //
    // //We can store multiple values but not of different types thier type should 
    // // be same
    // // Like tupple it also has the fixed length and cannot be change later

// fn main (){
    println!(" Examples of Array \n     3.2 ");
    let name_of_arrey = [90,20,25,45,22];
    println!("First value = {},Third value = {}", name_of_arrey[0],name_of_arrey[2]);

    let [Maths,Science,English,Islamiat,History] = name_of_arrey;
    println!("{}",Maths );

// }


// fn main() {

    // // Arithmetic Operations
    // //  +, _, *, /, %
    // //  We can apply all these operators on same type not on different types. " % " This operator will show the remainder 
  
    println!(" Examples of Arithmetic  \n         Operator    ");    
    let number_1 = 30;
    let number_2 = 4;
    
    println!("Addition {}",number_1 + number_2);
    
    println!("Subtraction {}",number_1 -number_2);
  
    println!("Multiplication {}",number_1 * number_2);
    
    println!("Division {}",number_1 / number_2);
  
    println!("Remainder {}",number_1 % number_2);
  
  
  
    
  
  // // Function
  
  // // It help to remove Duplication of Code or Repitation of Code
  // // Its easy to  maintain 
  // // It saves time
  // // WE write set of code in it which we want to use frequently in our program
  
  //  // // // //        Creating Function
  fn hair_style_1(){
    let front = 7; // cm
    let back  = 3; // cm
    let right_side = 4; // cm
    let left_side = 2; // cm
  
   println!("Hair_Style_1 \nFront {} cm \nBack {} cm \nRight side {} cm \nLeft side {} cm",front,back,right_side,left_side);
  }            // // Created // 
  
  //                //  Now Calling  We will just write name of our function and we can call unlimited times
  
  hair_style_1();
  
  // now creating same function again by using Shadowing and Array
  
  fn hair_style_2(){
  //  //     Initisting Array
    let lenght_of_hairs = [6,3,3,3];                             //  Shadowing   and Array
    //  Now keys
    let [front,back,right_side,left_side] = lenght_of_hairs;        
  
   println!("Hair_Style_2 \nFront {} cm \nBack {} cm \nRight side {} cm \nLeft side {} cm",front,back,right_side,left_side);
  
   
  }            // Created
  
  //                    // Calling   //
  hair_style_2();
  
  
  }
  