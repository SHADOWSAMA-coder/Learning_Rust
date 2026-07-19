#[derive(Debug)]
// we will be working with structs in this chapter.
struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_sount:u64,
}
//Structs are very much similar to tuple based oreinting but we can now explicitely name our fields making it more flexible to work with.

//These are tuple structs 
struct Color(i32,i32,i32);

//Lets define a Struct with methods 

struct Rectangle {
    width:usize,
    height:usize,
}

impl Rectangle {//These are associated functions
    fn create_rectangle(&self,width:usize,height:usize) -> Rectangle {
        Rectangle{
            width,
            height,
        }
    }

    fn get_width(&self)->usize{
        self.width
    }

    fn get_height(&self)->usize{
        self.height
    }

    fn get_area(&self)->usize{
        self.height*self.width
    }

    fn set_width(&mut self,width:usize){
        self.width = width;
    }

    fn set_height(&mut self,height:usize){
        self.height = height;
    }
}//You can use the pub keyword to specify which fields and structs should be used in other files.

fn main() {
    let mut user1 = User{// the netire instance must be mutable instead of certain fields.
        active:true,
        username:String::from("gay"),
        email:String::from("gay"),
        sign_in_count=1
    };

    //to access a specific value from a struct we use the dot notation.
    user1.email = String::from("another");

    //it is useful to create a new instance having some similar value from any previous instances.
    let user2 = User {
        active:user1.active,
        username:user1.username,
        email: String::from("Another 2"),
        sign_in_count : user1.sign_in_count,
    };// we can achieve it using struct update syntax like so 

    let user3 = User{
        email: String::from("Another 2"),
        ..user1//..keeps all the values same as the fields in user1
    };//after this line we can not further use user1 since the heap data has been moved to user2
    //If it were some other data lets say active and sign_in_scount then we could have used it

}

fn build_user (email:String,username:String) -> User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1,
    }//notice how the parameters and the naming fields have the same name.
    //rust allows us to bypass this by just using it once if they are the same.
    //instead of username:username we can just type username.
}