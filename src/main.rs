use genetic_mazes_lib::{Evaluator, Evolutionable, GeneticAlgorithm};
use genetic_mazes_lib::{Maze, MazeEval, TileState};
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra::Vector2;
use rand;
use smart_default::SmartDefault;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("GeneticMazes", "LokiVKlokeNaAndoke")
        .conf({
            let mut c = Conf::new().window_mode(
                WindowMode::default()
                    .resizable(false)
                    .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT),
            );
            c.window_setup = WindowSetup::default().title("GeneticMazes");
            c
        })
        .build()
        .unwrap();

    let mut my_game = GeneticMazes::new(&mut ctx, GameConfig::default()).unwrap();

    event::run(&mut ctx, &mut event_loop, &mut my_game).unwrap();
}

#[derive(SmartDefault)]
struct GameConfig {
    #[default(8.)]
    camera_move_speed: f32,

    #[default((10, 10))]
    maze_size: (usize, usize),
}

struct GeneticMazes {
    config: GameConfig,

    elitism_slider: f32,
    mutations_slider: u32,
    pop_size_slider: u32,
    alg: GeneticAlgorithm<Maze, MazeEval>,

    start: (i32, i32),
    end: (i32, i32),

    cam_pos: Vector2<f32>,

    path: Vec<(i32, i32)>,
    white_rect: Mesh,
    green_rect: Mesh,
    error_rect: Mesh,
}

impl GeneticMazes {
    pub fn new(ctx: &mut Context, config: GameConfig) -> GameResult<GeneticMazes> {
        let example_maze = Maze::new_empty(config.maze_size);
        let end = (
            (config.maze_size.0 - 1) as i32,
            (config.maze_size.1 - 1) as i32,
        );
        let eval = MazeEval::new((0, 0), end);
        let alg = GeneticAlgorithm::new(100, 0.2, 10, eval, &example_maze);
        Ok(GeneticMazes {
            config,
            start: (0, 0),
            end,
            elitism_slider: 0.2,
            mutations_slider: 30,
            pop_size_slider: 100,
            alg,
            path: Vec::new(),
            white_rect: ggez::graphics::MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    Rect::new(0., 0., 16., 16.),
                    graphics::WHITE,
                )
                .build(ctx)?,
            green_rect: ggez::graphics::MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    Rect::new(0., 0., 16., 16.),
                    Color::new(124. / 255., 252. / 255., 0., 1.),
                )
                .build(ctx)?,
            error_rect: ggez::graphics::MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    Rect::new(0., 0., 16., 16.),
                    Color::new(220. / 255., 20. / 255., 60. / 255., 1.),
                )
                .build(ctx)?,
            cam_pos: [0., 0.].into(),
        })
    }

    fn convert_pos(&self, x: i32, y: i32) -> [f32; 2] {
        vector2_to_arr(Vector2::from([x as f32 * 16., y as f32 * 16.]) - self.cam_pos)
    }
}

impl EventHandler for GeneticMazes {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut xy: Vector2<f32> = [0., 0.].into();
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            xy += Vector2::from([0., -1.]);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            xy += Vector2::from([0., 1.]);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            xy += Vector2::from([-1., 0.]);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::D) {
            xy += Vector2::from([1., 0.]);
        }
        let s = xy.abs().sum();
        if s > 0. {
            self.cam_pos +=
                if s > 1. { xy.normalize() } else { xy } * self.config.camera_move_speed;
        }

        self.alg.next_generation(&mut rand::thread_rng());
        self.path = self
            .alg
            .get_best()
            .unwrap()
            .find_path(self.start, self.end)
            .unwrap_or((Vec::new(), 0))
            .0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let (maze_sz_x, maze_sz_y) = self.alg.get_best().unwrap().size();
        for x in 0..maze_sz_x {
            for y in 0..maze_sz_y {
                match self.alg.get_best().unwrap().at(x, y).unwrap() {
                    TileState::Empty => (),
                    TileState::Full => graphics::draw(
                        ctx,
                        &self.white_rect,
                        DrawParam::default().dest(self.convert_pos(x as i32, y as i32)),
                    )?,
                }
            }
        }

        for &(x, y) in self.path.iter() {
            graphics::draw(
                ctx,
                match self
                    .alg
                    .get_best()
                    .unwrap()
                    .at(x as usize, y as usize)
                    .unwrap()
                {
                    TileState::Full => &self.error_rect,
                    TileState::Empty => &self.green_rect,
                },
                DrawParam::default().dest(self.convert_pos(x, y)),
            )?;
        }

        graphics::present(ctx)
    }
}

fn vector2_to_arr(vec: Vector2<f32>) -> [f32; 2] {
    [*vec.get(0).unwrap(), *vec.get(1).unwrap()]
}
