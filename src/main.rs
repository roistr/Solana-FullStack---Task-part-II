fn main() {

let string1 = String::from("Roi"); 
let string2 = String::from(" Schtreicher"); 

let cont_str = concancate_strings(&string1, &string2);

println!("{}", cont_str);




}




fn concancate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}
