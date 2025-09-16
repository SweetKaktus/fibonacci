use std::env;

/*
 * Pour faire un programme basé sur la suite de fibonacci facilement,
 * je dois passer par une version itérative.
 * je prends l'input n
 * Je commence une boucle en faisant:
 * a = 0
 * b = 1
 * c =  a + b
 * a = b
 * b = c
 * si b == n => {println!("{}", b); break;}
 */

/*
 * Comment set la prise d'argument pour notre programme.
 * Ex:
 * let pattern = env::args().nth(1).expect("no pattern given");
 * let pattern: u8 = pattern.trim().parse().expect("Saisie incorrecte");
 * println!("pattern: {:?}", pattern);
 */


fn calculate_fibonacci(n: u32) -> u32{
    let mut i: u32 = 2;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut c: u32;

    
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        while i <= n {
            c = a + b;
            a = b;
            b = c;
            i += 1;
        }
        return b
    }
}

fn main(){
    let input_user = env::args().nth(1).expect("no input given");
    let input_user: u32 = input_user.trim().parse().expect("Saisie incorrecte, veuillez saisir un nombre entier positif.");
    println!("Le {}ème nombre de la suite de Fibonacci est le {}", input_user, calculate_fibonacci(input_user));
}