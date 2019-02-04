mod board;
use std::env;

struct Fen(String);

impl Fen {
    fn to_char_array(fen:&str) -> [[char; 8];8] {
        let mut i = 0;
        let mut j = 0;
        let mut state=[['.';8]; 8];
        
        for c in fen.chars() {
            if let Ok(nombre) = c.to_string().parse::<usize>() {
                i+= nombre;
	    } else if c == '/'{
                i = 0;
	        j+=1
            } else {
	        state[j][i] = c;   
	        i+=1;
            }
        }
        return state
    }

    /*
    fn to_position (self) -> Result<Position,String> 
    example of use
    input:
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"
    
    the digits mark the number of empty space (should be between 1 and 8)

    output:111
    array 8x8 of char
    
    K..R....
    p.......
    ........
    ........
    ........
    ........
    ........
    
     */

    fn to_position (self) -> Result<board::Position,String> {
        let Fen(fen)=self;
        let position = fen.split_whitespace().collect::<Vec<&str>>();
        match position.len() {
            6 => {
                let p = board::Position{
                    board:match board::PieceArray::new(Fen::to_char_array(position[0])){
                        Ok(array) => array,
                        Err(error) => return Err(error)
                    },
                    active_color:match board::Color::new(position[1]){
                        Ok(color)=>color,
                        Err(error) => return Err(error)
                    },
                    castling:match board::Castling::new(position[2]){
                        Ok(castling)=>castling,
                        Err(error) => return Err(error)
                    },
                    enpassant:match board::Coordinate::new(position[3]){
                        Ok(enpassant)=>enpassant,
                        Err(error) => return Err(error)
                    },
                    halfmove:
                        if let Ok(m)=position[4].parse() {
                           m
                        } else {
                            return Err(String::from("bad half move"))
                        },
                    fullmove:
                        if let Ok(m)=position[5].parse() {
                           m
                        } else {
                            return Err(String::from("bad full move"))
                        }
                };
                return Ok(p);
            }
            _=> {
                return Err(String::from("bad fen number field")); 
            }
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();    
    match args.len() {
        2 => {
            match Fen(args[1][..].to_string()).to_position() {
                Ok(position) => {
                    //println!("{:?}",piece_array);
                    position.pretty_print()
                },
                Err(error) => println!("{}",error)
            }
        }
        _ => {
            println!("usage: fen_parser fen")
        }
    }
}
