pub fn c_a_msg()->String
{
use std::io;
println!("Would you like to calculate again? (y/n)");
let mut input1 = String::new();
io::stdin().read_line(&mut input1)
.expect("Failed to read line");
input1
}
pub mod calculator
{
    #[derive(Debug)]
    pub struct cal
    {
        first:i32,
        second:i32,
    }
    impl cal
    {
        pub fn calculator(first:i32,second:i32)->cal
        {
          cal
          {
            first,
            second,
          }  
        }
   pub fn addition(&self)
   {
   use std::process::exit;
    if self.first == 0 || self.second == 0
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("{} + {} is {}",self.first,self.second,self.first+self.second);
   loop
   {
    let msg = super::c_a_msg();
   if msg.trim() == "y" || msg.trim() == "Y"  
   {
       break;
   }
   else if msg.trim() == "n" || msg.trim() == "N"
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("Error - Wrong Input");
   }
   } 
   }
        }
    
    pub fn subtraction(&self)
   {
  use std::process::exit;
    if self.first == 0 || self.second == 0
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("{} - {} is {}",self.first,self.second,self.first-self.second);
   loop
   {
    let msg = super::c_a_msg();
   if msg.trim() == "y" || msg.trim() == "Y"  
   {
       break;
   }
   else if msg.trim() == "n" || msg.trim() == "N"
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("Error - Wrong Input");
   }
   } 
   }
        }

        pub fn multiplication(&self)
   {
  use std::process::exit;
    if self.first == 0 || self.second == 0
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("{} * {} is {}",self.first,self.second,self.first*self.second);
   loop
   {
    let msg = super::c_a_msg();
   if msg.trim() == "y" || msg.trim() == "Y"  
   {
       break;
   }
   else if msg.trim() == "n" || msg.trim() == "N"
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("Error - Wrong Input");
   }
   } 
   }
        }

        pub fn division(&self)
   {
  use std::process::exit;
    if self.first == 0 || self.second == 0
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("{} / {} is {}",self.first,self.second,self.first/self.second);
   loop
   {
    let msg = super::c_a_msg();
   if msg.trim() == "y" || msg.trim() == "Y"  
   {
       break;
   }
   else if msg.trim() == "n" || msg.trim() == "N"
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("Error - Wrong Input");
   }
   } 
   }
        }

        pub fn exponents(&self)
   {
  use std::process::exit;
    if self.first == 0 || self.second == 0
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("{} ^ {} is {}",self.first,self.second,self.first%self.second);
   loop
   {
    let msg = super::c_a_msg();
   if msg.trim() == "y" || msg.trim() == "Y"  
   {
       break;
   }
   else if msg.trim() == "n" || msg.trim() == "N"
   {
   println!("Bye, now.");
   exit(0);
   }
   else
   {
    println!("Error - Wrong Input");
   }
   } 
   }
        }
    }    
}