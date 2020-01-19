mod institute {
    pub mod department{
        
        pub fn dept_name(){
            println!("This is computer science department");
        } 

    }
}

use institute::department::dept_name;

fn main() {
    
    //First way of calling
    institute::department::dept_name();
    
    //Second way of calling
    //dept_name();
}