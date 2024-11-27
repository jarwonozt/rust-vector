use std::collections::VecDeque;

fn main() {
    let mut heros = vec!["lancelot", "hayabusa", "gusion", "balmond"];
    println!("Heros : {:?}", heros);


    heros.pop();
    println!("Heros pop : {:?}", heros);
    
    heros.remove(1);
    println!("Heros delete : {:?}", heros);

    heros.push("saber");
    heros.push("vexana");
    heros.push("parsha");
    println!("Heros push : {:?}", heros);
    
    heros[2] = "fanny"; //change saber to fanny
    println!("Heros update : {:?}", heros);

    let is_empty_heros = heros.is_empty(); //true
    println!("is empty heros : {is_empty_heros}");

    println!("Length : {}, capacity : {}", heros.len(), heros.capacity());
    println!();

    hero_mages();
    println!();
    hero_power();
    println!();
    vector_null();
    println!();

    // call function with param
    iterasi(&heros);
    heros.clear(); // change empty vector hero
    println!();
    vec_deque();

}

fn hero_mages()
{
    // append vector data
    let mut mages = vec!["vexana", "eudora", "nana", "xavier", "valir"];
    let mut mages_new = vec!["lilya", "change", "ourora", "selena", "zuxin"];
    mages.append(&mut mages_new);
    mages.append(&mut vec!["hearly", "kadita"]);
    mages.sort();
    println!("Mages : {:?}", mages);
    println!("Length : {}, capacity : {}", mages.len(), mages.capacity());
}

fn hero_power()
{
    let power_1 = vec![29, 32, 44]; // %
    let range_skill:Vec<f64> = vec![10.5, 20.2, 43.234];

    println!("Power : {:?}", power_1);
    println!("Range skill : {:?}", range_skill);
}

fn vector_null()
{
    let one_vector_empty:Vec<&str> = vec![];
    let two_vector_empty:Vec<&str> = Vec::new();
    let status = two_vector_empty.is_empty();
    println!("Vector One : {:?}", one_vector_empty);
    println!("Vector Two : {:?}", two_vector_empty);
    println!("is empty : {status}");
}

fn iterasi(heros: &Vec<&str>)
{
    let number = vec![1, 3, 5, 7];
    
    /* 
        this code error because ownership data
        alternative your use borrowing to data number
        use &number  
    */
    for i in &number {
        println!("Number : {}", i);
    }

    
    for i in 0..number.len() {
        println!("Number iterasi : {}", number[i]);
    }

    //or use iter()
    for i in number.iter() {
        println!("Number iter : {}", i);
    }
    println!();
    
    let hero_slice = &heros[0..3];
    println!("Hero slice : {:?}", hero_slice);
    println!();

    for hero in heros {
        println!("Hero name : {hero}");
    }
}

fn vec_deque()
{
    let mut person = VecDeque::from(vec!["Jhon", "Rambo", "Jacky Chen", "Bruclee"]);
    person.pop_front(); // delete element index 0
    person.push_front("Chuck Noris"); // add element index 0
    println!("Person data : {:?}", person);
    println!();
    person.pop_back(); // delete element last index
    person.push_back("Chuck Noris"); // add element last index
}
