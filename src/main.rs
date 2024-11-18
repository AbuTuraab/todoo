use std::collections::HashMap;
use std::env::args;
use std::fmt::Error;



struct Todo {
    map:HashMap<String, bool>,
    
}


impl Todo {
    
    fn nw_name(&mut self, key:String) {
        self.map.insert(key, true);
    }

    fn save(self)-> Result<(), std::fmt::Error> {
         let mut content = String::new();
            for (k, v) in self.map{
                let record = format!("{} {}", k, v);
                content.push_str(&record);
            } 
            println!("content {}", content);
            Ok(()) 
        }
}


fn main() {
  
let action =args().nth(1).expect("Please enter  action");
let item = args().nth(2).expect("Please enter item");

// println!("{:?}, {:?}", action, item);

let mut todo = Todo {
     map: HashMap::new(),
    
};

if action == "add" {
   
    todo.nw_name(item);
   

    match todo.save() {
         Ok(_) => println!("todo saved"),
         Err(_) => println!("an erroor occured"),
    }
   
  
}


}
