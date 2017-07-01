use std::env;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
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
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Color {
    Black,
    White
}
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
struct Piece {
    kind:PieceKind,
    color:Color
}
fn piece_to_char(piece: Piece) -> char {


    let c = piece_kind_to_char(piece.kind);
    if piece.color == Color::Black {

       
        if let Some(lower) = c.to_lowercase().nth(0) {
            lower
    
        } else {
            '.'
        }
       
    } else {
       c
    }
}
fn piece_kind_to_char(piece_kind: PieceKind) -> char {
    match piece_kind {
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

fn char_to_piece_kind(c:char) -> Option<PieceKind> {
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
fn char_to_piece(c:char)-> Option<Piece>{
    
    if c.is_lowercase() {
        if let Some(upper)=c.to_uppercase().next() {
            if let Some(kind) = char_to_piece_kind(upper) {
                return Some(Piece{
                    kind:kind,
                    color:Color::Black
                })
            }
        }
    
    } else {
          if let Some(kind) = char_to_piece_kind(c) {
                return Some(Piece{
                    kind:kind,
                    color:Color::White
                })
            }

    

    }
    return None;
}

fn char_array_to_piece_array(array: [[char; 8];8]) -> Result<[[Piece; 8];8],
                                                            String> {
    let mut state=[[Piece{kind:PieceKind::Empty,
                          color:Color::White};8]; 8];
    for (i,raw) in array.iter().enumerate() {
        for (j,&c) in raw.iter().enumerate() {
            if let Some(piece) = char_to_piece(c) {
                state[i][j] = piece;
            } else {
                return Err(format!("unkown piece {:?} at {},{}",c,i,j));
            }
            
        }
    }
    return Ok(state)
}

fn pretty_print(piece_array:[[Piece; 8];8]) {
    for raw in piece_array.iter() {
        for &p in raw.iter() {
            print!("{} ",piece_to_char(p));
        }
        println!();
    }
}

/*

example of use
input:
"K2R4/p7"

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

fn fen_to_char_array(fen:String) -> [[char; 8];8] {
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
fn main() {
    if let Some(arg) = env::args().nth(1) {

        match char_array_to_piece_array(fen_to_char_array(arg)) {
            Ok(piece_array) => {
                //println!("{:?}",piece_array);
                pretty_print(piece_array)
            },
            Err(error) => println!("{}",error)
        }
    }
}
