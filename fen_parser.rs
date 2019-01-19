use std::env;

#[derive(Copy,Clone)]
enum PieceKind {
    Empty,
    Pawn,
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Outside
}
#[derive(Copy,Clone,PartialEq,Debug)]
enum Color {
    Black,
    White
}

#[derive(Copy,Clone)]
struct Piece {
    kind:PieceKind,
    color:Color
}
#[derive(Copy,Clone,Debug)]
struct Castling {
    black_queen:bool,
    black_king:bool,
    white_queen:bool,
    white_king:bool,
}

#[derive(Copy,Clone,Debug)]
struct Coordinate {
    x:i8,
    y:i8,
}
#[derive(Copy,Clone)]
struct PieceArray([[Piece; 8];8]);

#[derive(Copy,Clone)]
struct Position {
    board:PieceArray,
    active_color:Color,
    castling:Castling,
    enpassant:Coordinate,
    halfmove:i32,
    fullmove:i32
}

struct Fen(String);

impl Color {
    fn new(color:&str) ->Result<Color,String> {
        match color {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(String::from("Bad color"))
        }
    }
}
impl Piece {
    fn to_char(self) -> char {


        let c = self.kind.to_char();
        if self.color == Color::Black {

       
            if let Some(lower) = c.to_lowercase().nth(0) {
                lower
    
            } else {
                '.'
            }
       
        } else {
            c
        }
    }

    fn new(c:char)-> Option<Piece>{
    
        if c.is_lowercase() {
            if let Some(upper)=c.to_uppercase().next() {
                if let Some(kind) = PieceKind::new(upper) {
                    return Some(Piece{
                        kind:kind,
                        color:Color::Black
                    })
                }
            }
    
        } else {
            if let Some(kind) = PieceKind::new(c) {
                return Some(Piece{
                    kind:kind,
                    color:Color::White
                })
            }
        }
        return None;
    }


}
impl PieceKind {
    fn to_char(self) -> char {
        match self {
            PieceKind::Empty => '.',
            PieceKind::Pawn => 'P',
            PieceKind::King => 'K',
            PieceKind::Queen => 'Q',
            PieceKind::Bishop => 'B',
            PieceKind::Knight => 'N',
            PieceKind::Rook => 'R',
            PieceKind::Outside => '#'
        }
    }


    fn new(c:char) -> Option<PieceKind> {
        match c {
            '.' => Some(PieceKind::Empty),
            'P' => Some(PieceKind::Pawn),
            'K' => Some(PieceKind::King),
            'Q' => Some(PieceKind::Queen),
            'B' => Some(PieceKind::Bishop),
            'N' => Some(PieceKind::Knight),
            'R' => Some(PieceKind::Rook),
            '#' => Some(PieceKind::Outside),
            _ => None
        }
    }
}
impl PieceArray{
    fn new(array: [[char; 8];8]) -> Result<PieceArray,
                                       String> {
        let mut state=[[Piece{kind:PieceKind::Empty,
                              color:Color::White};8]; 8];
        for (i,raw) in array.iter().enumerate() {
            for (j,&c) in raw.iter().enumerate() {
                if let Some(piece) = Piece::new(c) {
                    state[i][j] = piece;
                } else {
                return Err(format!("unkown piece {:?} at {},{}",c,i,j));
            }
            
        }
    }
    return Ok(PieceArray(state))
    }

}
impl Position {
    fn pretty_print(self) {
        let PieceArray(board)=self.board;
        for raw in board.iter() {
            for &p in raw.iter() {
                print!("{} ",p.to_char());
            }
            println!();
        }
        println!("site to move {:#?}",self.active_color);
        println!("castle {:#?} ",self.castling);
        println!("enpassant {:#?}",self.enpassant);
        println!("halfmove {}",self.halfmove);
        println!("fullmove {}",self.fullmove);
        
    }
}
impl Coordinate {
    fn new(input:&str)-> Result<Coordinate,String>{
        let e = String::from(input);
        if e=="-" {
            return Ok(Coordinate{x:-1,y:-1});
        }
        if e.len()!=2{
            return Err(String::from("bad en coordinate len!=2"));
        }
        let mut f=e.chars();
        if let Some(xx) =f.nth(0) {
            
            let x= xx as u32 - 'a' as u32;
            if !(x<=7) {
                
                return Err(String::from("bad cooridinate column"));
            }  
    
        
            if let Some(yy) =f.nth(0) {
                if let Some(y) = yy.to_digit(10){
                    let mut  z=y as i32 -1;
                   
                    if !(0<=z && z<=7) {
                        return Err(String::from("bad cooridinate rank"));
                    }
                    Ok(Coordinate{x:x as i8,y:z as i8})
                } else {
                    return Err(String::from("bad coordinate rank"));
                }
            
                
            } else {
                return Err(String:: from("not found rank"));
                           
            }
        } else {
              return Err(String:: from("not found column"));
        }
            
    }
}
impl Castling {
    fn new(castling:&str)-> Result<Castling,String>{
        let mut cast = Castling{
            black_queen:false,
            black_king:false,
            white_queen:false,
            white_king:false};
        for c in castling.chars() {
            match c {
                'q'=>cast.black_queen=true,
                'k'=>cast.black_king=true,
                'Q'=>cast.white_queen=true,
                'K'=>cast.white_king=true,
                '-'=>(),
                _=>return Err(String::from("bad castle"))
            }
        }
        Ok(cast)
    }
}

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

    fn to_position (self) -> Result<Position,String> {
        let Fen(fen)=self;
        let position = fen.split_whitespace().collect::<Vec<&str>>();
        match position.len() {
            6 => {
                
                let p = Position{
                
                    board:match PieceArray::new(Fen::to_char_array(position[0])){
                        Ok(array) => array,
                        Err(error) => return Err(error)
                    },
                    
                    active_color:match Color::new(position[1]){
                        Ok(color)=>color,
                        Err(error) => return Err(error)
                    },
                                        
                    castling:match Castling::new(position[2]){
                        Ok(castling)=>castling,
                        Err(error) => return Err(error)
                    },
                    enpassant:match Coordinate::new(position[3]){
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
/*
    fn to_piece_array(self)-> Result<PieceArray,
                                       String>{
        let Fen(fen)=self;
        PieceArray::new(Fen::to_char_array(fen))
    }
*/
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
