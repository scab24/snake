//3 . uso para crear algunas funciones de ayuda
//queremos traer piston_window en la que usaremos un rectangulo
use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

//creamos una constante de tamaño de bloque de 25 pixeles cada vez que creamos un bloque
const BLOCK_SIZE: f64 = 25.0;

//escribo una función para tomar una coordenada i32 para devolver un f64 y multiplicarlo por nuestro bloque
//la expresión 'pub' nos permite exportar esta función, por tanto la hace pública para todo el programa
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}


//Usamos otra función pública, quiere dibujar un bloque
pub fn draw_block (color: Color, x: i32, y: i32, con: &Context, g:&mut G2d) {
    let gui_x = to_coord (x);
    let gui_y = to_coord(y);

    rectangle 
        (color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
        );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
    [
            x, 
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
            ],
            con.transform,
            g,
    );
}