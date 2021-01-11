use coord_2d::{Coord, Size};
use rgb24::Rgb24;
use direction::CardinalDirection;

fn main() {
    use chargrid_graphical::{Context, ContextDescriptor, Dimensions, FontBytes};
    const CELL_SIZE_PX: f64 = 24.;
    let context = Context::new(ContextDescriptor {
        font_bytes: FontBytes {
            normal: include_bytes!("fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("fonts/PxPlus_IBM_CGA.ttf").to_vec(),
        },
        title: "Wolf's Den 2021".to_string(),
        window_dimensions: Dimensions {
            width: 960.,
            height: 720.,
        },
        cell_dimensions: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        font_dimensions: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        font_source_dimensions: Dimensions {
            width: CELL_SIZE_PX as f32,
            height: CELL_SIZE_PX as f32,
        },
        underline_width: 0.1,
        underline_top_offset: 0.8,
    }).expect("Failed to initialize graphical context");
    let screen_size = Size::new(40, 30);
    let app = App::new(screen_size);
    context.run_app(app);
}

struct AppData {
    player_coord: Coord,
    screen_size: Size,
}

impl AppData {
    fn new(screen_size: Size) -> Self {
        Self {
            player_coord: screen_size.to_coord().unwrap() / 2,
            screen_size,
        }
    }

    fn maybe_move_player(&mut self, direction: CardinalDirection) {
        let new_player_coord = self.player_coord + direction.coord();
        if new_player_coord.is_valid(self.screen_size) {
            self.player_coord = new_player_coord
        }
    }

    fn handle_input(&mut self, input: chargrid::input::Input) {
        use chargrid::input::{Input, KeyboardInput};
        match input {
            Input::Keyboard(key) => match key {
                KeyboardInput::Left => self.maybe_move_player(CardinalDirection::West),
                KeyboardInput::Right => self.maybe_move_player(CardinalDirection::East),
                KeyboardInput::Up => self.maybe_move_player(CardinalDirection::North),
                KeyboardInput::Down => self.maybe_move_player(CardinalDirection::South),
                _ => {},
            },
            _ => {},
        }
    }
}

struct AppView {}

impl AppView {
    fn new() -> Self {
        Self {}
    }
}

impl<'a> chargrid::render::View<&'a AppData> for AppView {
    fn view<F: chargrid::app::Frame, C: chargrid::app::ColModify>(
        &mut self,
        data: &'a AppData,
        context: chargrid::app::ViewContext<C>,
        frame: &mut F,
    ) {
        let view_cell = chargrid::render::ViewCell::new()
            .with_character('@')
            .with_foreground(Rgb24::new_grey(255));
        frame.set_cell_relative(data.player_coord, 0, view_cell, context);
    }


}

struct App {
    data: AppData,
    view: AppView,
}

impl App {
    fn new(screen_size: Size) -> Self {
        Self {
            data: AppData::new(screen_size),
            view: AppView::new(),
        }
    }
}

impl chargrid::app::App for App {
    fn on_input(&mut self, input: chargrid::app::Input) -> Option<chargrid::app::ControlFlow> {
        use chargrid::input::{keys, Input};
        match input {
            Input::Keyboard(keys::ETX) | Input::Keyboard(keys::ESCAPE) => {
                Some(chargrid::app::ControlFlow::Exit)
            }
            other => {
                self.data.handle_input(other);
                None
            }
        }
    }

    fn on_frame<F, C>(
        &mut self,
        _since_last_frame: chargrid::app::Duration,
        view_context: chargrid::app::ViewContext<C>,
        frame: &mut F,
    ) -> Option<chargrid::app::ControlFlow>
    where 
        F: chargrid::app::Frame,
        C: chargrid::app::ColModify,
    {
        use chargrid::render::View;
        self.view.view(&self.data, view_context, frame);
        None
    }
}