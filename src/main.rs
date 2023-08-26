use macroquad::prelude::{collections::storage, *};
use macroquad_tiled as tiled;

struct Assets {
    tilemap: tiled::Map,
}

struct Player {
    position: Vec2,
    moves: u32,
}

fn tile_to_center(pos: (u32, u32)) -> Vec2 {
    let (x, y) = pos;
    Vec2::new(x as f32 + 0.5, y as f32 + 0.5)
}

fn dist(a: (u32, u32), b: (u32, u32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl Player {
    const MOVES: u32 = 4;
    fn create() -> Self {
        let position = storage::get::<Assets>()
            .tilemap
            .tiles("player", None)
            .find_map(|(x, y, tile)| tile.is_some().then_some(tile_to_center((x, y))))
            .unwrap();
        Self {
            position,
            moves: Self::MOVES,
        }
    }

    fn view(&self) -> Rect {
        let height = screen_height();
        let width = screen_width();
        let scale = height / 10.;
        let width = width / scale;
        let height = height / scale;
        Rect {
            x: self.position.x - width / 2.,
            y: self.position.y - height / 2.,
            w: width,
            h: height,
        }
    }

    fn update(&mut self) {
        let mut camera = Camera2D::from_display_rect(self.view());
        //TODO:fix rotated camera issue
        camera.zoom.y = -camera.zoom.y;
        set_camera(&camera);
    }

    fn refresh(&mut self) {
        self.moves = Self::MOVES;
    }

    fn tile_pos(&self) -> (u32, u32) {
        (
            self.position.x.floor() as u32,
            self.position.y.floor() as u32,
        )
    }

    fn available_moves(&self) -> Vec<(u32, u32)> {
        // TODO: make bfs
        let start = self.tile_pos();
        let map = &storage::get::<Assets>().tilemap;
        map.tiles("map", None)
            .filter_map(|(x, y, tile)| {
                (dist(start, (x, y)) <= self.moves && tile.is_none()).then_some((x, y))
            })
            .collect()
    }

    fn move_to(&mut self, pos: (u32, u32)) {
        if self.available_moves().contains(&pos) {
            self.moves -= dist(self.tile_pos(), pos);
            self.position = tile_to_center(pos);
        }
    }

    fn draw(&self) {
        let map = &storage::get::<Assets>().tilemap;
        map.spr(
            "tiles",
            1,
            Rect::new(self.position.x - 0.5, self.position.y - 0.5, 1., 1.),
        );
        let start = self.tile_pos();
        map.tiles("map", None).for_each(|(x, y, tile)| {
            if dist(start, (x, y)) <= self.moves && tile.is_none() {
            } else {
                map.spr("tiles", 4, Rect::new(x as f32, y as f32, 1., 1.));
            }
        });
    }
}

#[macroquad::main("Better find it")]
async fn main() {
    let tiled_map = load_file("tiled/tiles.json").await.unwrap();
    let tiled_map = String::from_utf8(tiled_map).unwrap();
    let tiled_tsx = load_file("tiled/tileset.json").await.unwrap();
    let tiled_tsx = String::from_utf8(tiled_tsx).unwrap();
    let tileset_texture = load_texture("tiled/tiles.png").await.unwrap();

    let tilemap = tiled::load_map(
        &tiled_map,
        &[("tiles.png", tileset_texture)],
        &[("tiles.tsx", &tiled_tsx)],
    )
    .unwrap();
    storage::store(Assets { tilemap });
    let mut player = Player::create();
    let map = &storage::get::<Assets>().tilemap;
    let map_rect = Rect {
        x: 0.,
        y: 0.,
        w: map.raw_tiled_map.width as f32,
        h: map.raw_tiled_map.height as f32,
    };
    let mut turn = 1;
    loop {
        // Field part
        // Camera is set under player's update
        push_camera_state();
        player.update();
        if player.moves == 0 {
            turn += 1;
            player.refresh();
        }

        clear_background(WHITE);
        map.draw_tiles("map", map_rect, None);
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position_local();
            let view = player.view();
            let x = (view.x + view.w * (1. + pos.x) / 2.).floor() as u32;
            let y = (view.y + view.h * (1. + pos.y) / 2.).floor() as u32;
            player.move_to((x, y));
        }
        player.draw();

        pop_camera_state();
        // UI part
        draw_text(&format!("Turn: {turn}"), 5., 30., 20., BLACK);
        next_frame().await
    }
}
