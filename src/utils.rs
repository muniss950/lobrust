use crate::cli::Args; 

pub fn get_input(args: &Args){
   print!("Search Movie/TV show: "); 
    let mut input = String::new();
    args.query=match io::stdin().read_line(&mut input) {
        Ok(n) => {
            Some(input)
        }
        Err(error) =>{ 
            println!("error: {error}") ;
            None},
    };
    query=match args.query{
        Some(query){
            query
        },
        None{
            print!("none");
        }
    } 
    println!("Querying {args.query)}..");
    
}
