#[derive(Copy,Clone,Hash,Eq,PartialEq,Debug)]
pub enum PieceKind {
    Empty,
    Pawn,
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Outside
}
#[derive(Copy,Clone,PartialEq,Eq,Debug,Hash)]
pub enum Color {
    Black,
    White
}
#[derive(Copy,Clone,PartialEq,Eq,Debug,Hash)]
pub struct Piece {
    kind:PieceKind,
    color:Color
}
#[derive(Copy,Clone,Debug)]
pub struct Castling {
    black_queen:bool,
    black_king:bool,
    white_queen:bool,
    white_king:bool,
}

#[derive(Copy,Clone,Debug)]
pub struct Coordinate {
    x:i8,
    y:i8,
}

#[derive(Clone,Debug)]
pub struct PieceArray{
    flat:[[Piece; 8];8],
    reverse:(Vec<(PieceKind,Coordinate)>,
             Vec<(PieceKind,Coordinate)>)
}

#[derive(Clone)]
pub struct Position {
    pub board:PieceArray,
    pub active_color:Color,
    pub castling:Castling,
    pub enpassant:Coordinate,
    pub halfmove:i32,
    pub fullmove:i32
}

impl Color {
    pub fn new(color:&str) ->Result<Color,String> {
        match color {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(String::from("Bad color"))
        }
    }
}

impl Piece {

    pub fn to_char(self) -> char {
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

    pub fn new(c:char)-> Option<Piece>{
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

    pub fn to_char(self) -> char {
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

    pub fn new(c:char) -> Option<PieceKind> {
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

impl Coordinate {

    pub fn new(input:&str)-> Result<Coordinate,String>{
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

    pub fn new(castling:&str)-> Result<Castling,String>{
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

impl PieceArray{

    pub fn new(array: [[char; 8];8]) -> Result<PieceArray,
                                       String> {
        let mut state=[[Piece{kind:PieceKind::Empty,
                              color:Color::White};8]; 8];
        let mut black = Vec::new();
        let mut white = Vec::new();
        
        for (i,raw) in array.iter().enumerate() {
            for (j,&c) in raw.iter().enumerate() {
                if let Some(piece) = Piece::new(c) {
                    state[i][j] = piece;
                    
                    match piece.color {
                        Color::Black => black.push((piece.kind,Coordinate{x:i as i8,y:j as i8})),
                        Color::White => white.push((piece.kind,Coordinate{x:i as i8,y:j as i8}))
                    }
                    
                } else {
                    return Err(format!("unkown piece {:?} at {},{}",c,i,j));
                }            
            }
        }
        return Ok(PieceArray{flat:state,reverse:(white,black)})
    }
}

impl Position {
    pub fn pretty_print(self) {
        let board = self.board.flat;
        for raw in board.iter() {
            for &p in raw.iter() {
                print!("{} ",p.to_char());
            }
            println!();
        }
        println!("side to move {:#?}",self.active_color);
        println!("castle {:#?} ",self.castling);
        println!("enpassant {:#?}",self.enpassant);
        println!("halfmove {}",self.halfmove);
        println!("fullmove {}",self.fullmove);
    }
}
