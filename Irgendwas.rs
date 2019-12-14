fn summe_zweier_zahlen(a: f32, b: f32) -> f32
{
   return a + b;
}


/* Das 
könnte
wichtig
werden!
*/


/*fn main()
{
    println! ("Ay Caramba!");
    println!("Die Antwort lautet: {}", summe_zweier_zahlen(41, 1));
}*/

fn main()
{   
    println!("Hello, Universe!");
    println!("Ay Caramba Du {}er!\nDie Antwort lautet: {}\nIm Zweifelsfall {} oder {}",
    summe_zweier_zahlen(1.0,0.0), summe_zweier_zahlen(41.0, 1.0), summe_zweier_zahlen(10.0, 11.00), summe_zweier_zahlen(3.0, 2.0));
    println!("Hm?");
}