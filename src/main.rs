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

    heros.clear(); // change empty vector hero

    let is_empty_heros = heros.is_empty(); //true
    println!("is empty heros : {is_empty_heros}");

    println!("Length : {}, capacity : {}", heros.len(), heros.capacity());
    println!();

    hero_mages();
    println!();
    hero_power();

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
