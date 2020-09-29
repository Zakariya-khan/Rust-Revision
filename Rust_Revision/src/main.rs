      //                      3.1     Shadowing                          //
fn main() {

    let name = "Zakariya Khan";
    println!("Hello {}, How are you",name);

    // now printing another name by using same variable

    let name = "yahya Khan";
    println!("and where is {}",name);

// }


    //   //                    3.2    Tupple                        //
    // we can store multiple values of different types in tupple. Once the tupple 
    // is innitialized it cannot be extended 

// fn main() {    

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

    let name_of_arrey = [90,20,25,45,22];
    println!(" First value = {}, Third value = {}", name_of_arrey[0],name_of_arrey[2]);

    let [Maths,Science,English,Islamiat,History] = name_of_arrey;
    println!("{}",Maths );

}
  