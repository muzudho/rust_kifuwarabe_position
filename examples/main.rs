extern crate kifuwarabe_position;
use kifuwarabe_position::*;

fn main() {

    let mut position0 = Position::new();

    position0.set_km_by_ms(55, Koma::Z0);

    let mut position1 = position0.clone();

    println!("ms55: {}", position1.get_km_by_ms(55));

    position1.set_km_by_ms(55, Koma::K0);

    println!("pos0ms55: {}, pos1ms55: {}", position0.get_km_by_ms(55), position1.get_km_by_ms(55));

    let mut position2 = Position::new();
    position2.set_all(&position0);

    println!("pos0ms55: {}, pos1ms55: {}, pos2ms55: {}", position0.get_km_by_ms(55), position1.get_km_by_ms(55), position2.get_km_by_ms(55));

    println!("pos0ms55: Sengo::Sen == Sengo::Sen: {}.", &Sengo::Sen == &Sengo::Sen);
    println!("pos0ms55: Sengo::Sen == Sengo::Go: {}.", &Sengo::Sen == &Sengo::Go);
    println!("pos0ms55: Sengo::Go == Sengo::Sen: {}.", &Sengo::Go == &Sengo::Sen);
    println!("pos0ms55: Sengo::Go == Sengo::Go: {}.", &Sengo::Go == &Sengo::Go);

}