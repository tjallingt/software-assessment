use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 400;
const WINDOW_HEIGHT: u32 = 400;

const GRID_ROWS: usize = 5;
const GRID_COLS: usize = 5;

const SQUARE_SIZE: f32 = 50.0;
const WALL_WIDTH: f32 = 10.0;

fn main() {
    // Call the `model` function to create the initial `GameState`
    // this uses "Nannou" which is a creative-coding framework for Rust
    // - Website: https://nannou.cc/
    // - Docs: https://docs.rs/nannou/latest/nannou/
    nannou::app(model).run();
}

/// Nannou calls this function to initialize the application
/// This returns the data model for the application which is the state we want to track throughout
/// the lifetime of the application. In our case this data is the game state.
fn model(app: &App) -> GameState {
    // create a new window to draw to
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        // on draw call the `view` function
        .view(view)
        // on any event (i.e. `MousePressed`) call the `event` function
        .event(event)
        .build()
        .unwrap();

    // create the data model for the app
    GameState::new()
}

/// The view function will be called to draw the current game state.
fn view(app: &App, model: &GameState, frame: Frame) {
    let draw = app.draw();

    // clear the previous frame
    draw.background().color(WHITE);

    for square in &model.squares {
        // draw the square background
        draw.rect()
            .color(square.color())
            .xy(square.rect.xy())
            .wh(square.rect.wh());

        // draw the walls
        let [left, top, right, bottom] = &square.walls;

        draw.rect()
            .color(left.color())
            .xy(left.rect.xy())
            .wh(left.rect.wh());

        draw.rect()
            .color(top.color())
            .xy(top.rect.xy())
            .wh(top.rect.wh());

        draw.rect()
            .color(right.color())
            .xy(right.rect.xy())
            .wh(right.rect.wh());

        draw.rect()
            .color(bottom.color())
            .xy(bottom.rect.xy())
            .wh(bottom.rect.wh());
    }

    draw.to_frame(app, &frame).unwrap();
}

// Update the game state based on an event that happened to the application window.
fn event(app: &App, model: &mut GameState, event: WindowEvent) {
    if matches!(event, WindowEvent::MousePressed(_)) {
        let point = Point2::new(app.mouse.x, app.mouse.y);
        println!("click at {point:?}");

        // TODO: implement the game logic
        // - check if the click hit any walls
        //   - if so mark the wall as taken by the player
        // - check if any squares where completed by the player
        //   - if so mark the squares as taken by the player
        // - if the player took a wall its the next players turn
        // - if the player took a square they get another turn
        // - if all the squares are taken the game has ended, the player with the most squares wins
        //
        // Bonus ideas:
        // - show which players turn it is
        // - show which player won at the end
        // - make the game prettier

        model.current_player = match model.current_player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
    }
}

struct GameState {
    /// All squares in the game.
    squares: Vec<Square>,
    /// The player whose turn it is.
    current_player: Player,
}

impl GameState {
    /// Create a new game state.
    fn new() -> Self {
        // compute the size of the full grid
        let grid_width = SQUARE_SIZE * GRID_ROWS as f32;
        let grid_height = SQUARE_SIZE * GRID_COLS as f32;

        // compute the middle point of grid
        let grid_half_width = grid_width / 2.0;
        let grid_half_height = grid_height / 2.0;

        let half_square_size = SQUARE_SIZE / 2.0;

        // compute offset from the center to the top left square
        let grid_offset = Point2::new(
            -grid_half_width + half_square_size,
            grid_half_height - half_square_size,
        );

        // create a rectangle in the center of the screen and shift it to the top left of the grid
        let mut current_row = Rect::from_w_h(SQUARE_SIZE, SQUARE_SIZE).shift(grid_offset);

        let mut squares = vec![];

        // start creating the squares for a row
        for _ in 0..GRID_ROWS {
            // Explicit clone to make sure we don't accidentally modify `current_row`
            #[allow(clippy::clone_on_copy)]
            let mut current_col = current_row.clone();

            // for each column in the grid create a square and move the `current_col` to its right
            // note that the sides of the squares are touching (and therefore their walls overlap)
            for _ in 0..GRID_COLS {
                let square = Square::from_rect(current_col);

                squares.push(square);

                current_col = current_col.right_of(current_col);
            }

            // move to the next row
            current_row = current_row.below(current_row);
        }

        Self {
            squares,
            current_player: Player::One,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Player {
    One,
    Two,
}

impl Player {
    /// Get the players color.
    fn color(&self) -> Rgb<u8> {
        match self {
            Player::One => RED,
            Player::Two => GREEN,
        }
    }
}

struct Square {
    /// The bounding box for the square, the walls are drawn on the edges of this [Rect].
    /// See: https://docs.rs/nannou/latest/nannou/geom/struct.Rect.html
    pub rect: Rect,
    /// All 4 walls for the square in the order; left, top, right, bottom.
    pub walls: [Wall; 4],
    /// If [None] the square has not been taken, else this contains the [Player] which has won the square.
    pub taken_by: Option<Player>,
}

struct Wall {
    /// The bounding box for the wall.
    /// See: https://docs.rs/nannou/latest/nannou/geom/struct.Rect.html
    pub rect: Rect,
    /// If [None] the wall has not been taken, else this contains the [Player] which has taken the wall.
    pub taken_by: Option<Player>,
}

impl Wall {
    /// Create a new wall from the given [Rect].
    fn from_rect(rect: Rect) -> Self {
        Self {
            rect,
            taken_by: None,
        }
    }

    /// Create a new [Wall] from the given x, y, width and height.
    fn from_x_y_w_h(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::from_rect(Rect::from_x_y_w_h(x, y, w, h))
    }

    /// Get the current color for this wall.
    fn color(&self) -> Rgb<u8> {
        self.taken_by.map(|player| player.color()).unwrap_or(BLACK)
    }
}

impl Square {
    /// Create a new square from the given [Rect].
    fn from_rect(rect: Rect) -> Self {
        let left = Wall::from_x_y_w_h(rect.left(), rect.y(), WALL_WIDTH, rect.h());

        let top = Wall::from_x_y_w_h(rect.x(), rect.top(), rect.w(), WALL_WIDTH);

        let right = Wall::from_x_y_w_h(rect.right(), rect.y(), WALL_WIDTH, rect.h());

        let bottom = Wall::from_x_y_w_h(rect.x(), rect.bottom(), rect.w(), WALL_WIDTH);

        Self {
            rect,
            walls: [left, top, right, bottom],
            taken_by: None,
        }
    }

    /// Get the current color for this square.
    fn color(&self) -> Rgb<u8> {
        self.taken_by.map(|player| player.color()).unwrap_or(WHITE)
    }
}
