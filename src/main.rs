extern crate zummi;

fn main(){
    if let Some(words) = std::env::args().nth(1) {
        if let Some(zummi) = zummi::zummi(&words){
            println!("{}", zummi);
        }
    }
}
