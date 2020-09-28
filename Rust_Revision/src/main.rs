      //   Shadowing                //
fn main() {

    let name = "Zakariya Khan";
    println!("Hello {}, How are you",name);

    // now printing another name by using same variable

    let name = "yahya Khan";
    println!("and where is {}",name);

// }


    //   //                        Tupple                        //
    // we can store multiple values of different types in tupple. Once the tupple 
    // is innitialized it cannot be extended 

// fn main() {    

    //  Using tupple for saving student information //
    //                Name                Age      ROll no            GPA  //
    let tupname =( String::from("Uzair"),  45,       125,             3.5);
    //    index               0             1          2               3
    
    // We use indexing to print our values //
    
    println!("second value {} forth value {}",tupname.1,tupname.2);

    //                 now indexing tupple with key                //

    let (Name,Age,Roll_No,GPA)  = tupname;
    println!("Name of a student {} \nRoll no of {} {} ",Name , Name, Roll_No );

    // By initializing keys to tupple now u just have to write the key and
    //  u will get the detail u want now u dont have to remember index
          
    //                     Arey                          //


}
