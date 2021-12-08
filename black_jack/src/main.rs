use rand::Rng;

fn main() {
    // starting random seed
    let mut rng = rand::thread_rng();
    // creating original deck 
    let mut deck = vec![1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,11,11];
    // creating game state vari
    let mut player:Vec<i32>  = Vec::new();
    let mut dealer:Vec<i32>  = Vec::new();
    let mut hit:bool;
    let mut sum_p: i32;
    let mut sum_d: i32;
    let mut index;
    

    // Ask user if they want to play?
    let mut line = String::new();
    println!(r#"Ready to play? y/n"#);    
    std::io::stdin().read_line(&mut line).unwrap();
    if line == "y\n"{

    // Initialize game state with starting hands
    index = rng.gen_range(0..deck.len());
    player.push(deck[index]);
    deck.swap_remove(index); 
    index = rng.gen_range(0..deck.len());
    player.push(deck[rng.gen_range(0..deck.len())]);
    deck.swap_remove(index); 
    index = rng.gen_range(0..deck.len());
    dealer.push(deck[rng.gen_range(0..deck.len())]);
    deck.swap_remove(index); 
    index = rng.gen_range(0..deck.len());
    dealer.push(deck[rng.gen_range(0..deck.len())]);
    deck.swap_remove(index); 

    // Ask user if they want to continue 
    println!("Your starting cards on the table are [{}, {}] ", player[0],player[1]);
    println!("The dealers starting cards on the table are [{} , ? ]", dealer[0]);
    println!("Do you want to hit? y/n");    
    println!("Cards left in deck :: {}", deck.len());

    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    if line == "y\n"{
        hit = true;
    }
    else{

        hit = false;
    }

    // first summation to set score 
    sum_d = dealer.iter().sum();
    sum_p = player.iter().sum();


    // while we have a deck, we want to keep hitting, and neither has busted 
    while deck.len() > 0 && hit == true && sum_d < 22 && sum_p < 22
    {

        // draw new card and discard from deck 
        index = rng.gen_range(0..deck.len());
        player.push(deck[index]);
        deck.swap_remove(index); 
        println!("Your cards are now ::");
        println!("{:?}", player);

        
        sum_d = dealer.iter().sum();
        sum_p = player.iter().sum();
        println!("Cards left in deck :: {}", deck.len());

    
        // "start" game loop 
        println!("Do you want to hit? y/n");   
        line.clear(); 
        std::io::stdin().read_line(&mut line).unwrap();
        if line == "y\n"{
            hit = true;
            println!("Take a card ?");

        }
        else{
            hit = false;
        }

    }
    sum_d = dealer.iter().sum();
    sum_p = player.iter().sum();

    println!("Your cards are ::");
    println!("{:?}", player);
    println!("Dealers cards are ::");
    println!("{:?}", dealer);
    println!("Cards left in deck :: {}", deck.len());

    if sum_d == sum_p {
        println!("Tie Game! {} {} ", sum_d, sum_p)
    }
    else if sum_d <= 21 && sum_p <= 21 {
        if 21 - sum_d > 21 - sum_p{
            println!("You win!")
        }
        else{
            println!("Dealer wins, better luck next time")

        }
    }
    else if sum_d <= 21{
        println!("Dealer wins, better luck next time")

    }
    else if sum_p <= 21{
        println!("You win!")

    }

}
}