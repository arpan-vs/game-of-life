use cell::Cellule;
use gloo::timers::callback::Interval;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};
use web_sys::window;

mod cell;

pub enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule(usize),
    Tick,
}

pub struct App {
    active: bool,
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    _interval: Interval,
}

impl App {
    pub fn random_mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for cellule in self.cellules.iter_mut() {
            if rng.gen_bool(0.5) {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].is_alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status = if cellule.is_alive() {
            "bg-indigo-800 hover:bg-indigo-600"
        } else {
            "bg-gray-100 hover:bg-gray-300"
        };
        
        html! {
            <div 
                key={idx} 
                class={classes!(
                    "w-3", "h-3", "sm:w-4", "sm:h-4", "md:w-5", "md:h-5",
                    "inline-block", "border-[0.5px]", "border-gray-300", "transition-colors", "duration-200", cellule_status
                )}
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        // Responsive grid size based on screen width
        fn get_responsive_grid_size() -> (usize, usize) {
            if let Some(win) = window() {
                let width = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1280.0);
                let height = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(720.0);

                // Tailwind breakpoints: 640, 768, 1024, 1280, 1536
                let (cell_px, max_w, max_h) = if width < 640.0 {
                    (12.0, 25, 30)
                } else if width < 768.0 {
                    (13.0, 25, 14)
                } else if width < 1024.0 {
                    (13.0, 30, 18)
                } else if width < 1280.0 {
                    (15.0, 45, 22)
                } else if width < 1536.0 {
                    (16.0, 55, 28)
                } else {
                    (17.0, 70, 36)
                };

                log::info!("Width: {}, Height: {}", width, height);
                // Padding and controls take up space, so subtract a bit
                let grid_width = ((width - 32.0) / cell_px).floor() as usize;
                let grid_height = ((height - 32.0) / cell_px).floor() as usize;

                log::info!("grid width {} x height {}", grid_width, grid_height);

                let width = grid_width.clamp(10, max_w);
                let height = grid_height.clamp(8, max_h);

                log::info!("{} x {}", width, height);

                (width, height)
            } else {
                (38, 18)
            }
        }

        let (cellules_width, cellules_height) = get_responsive_grid_size();

        log::info!("Grid size: {} x {}", cellules_width, cellules_height);

        Self {
            active: false,
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                true
            }
            Msg::Step => {
                self.step();
                true
            }
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                true
            }
            Msg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                true
            }
            Msg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows =
            self.cellules
                .chunks(self.cellules_width)
                .enumerate()
                .map(|(y, cellules)| {
                    let idx_offset = y * self.cellules_width;

                    let cells = cellules
                        .iter()
                        .enumerate()
                        .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link()));
                    html! {
                        <div key={y} class="flex">
                            { for cells }
                        </div>
                    }
                });

        let play_button = if self.active {
            html! {
                <button 
                    class="flex items-center justify-center bg-red-600 hover:bg-red-700 text-white font-semibold py-2 px-4 rounded-lg shadow-md transition-colors duration-300 w-24"
                    onclick={ctx.link().callback(|_| Msg::Stop)}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
                        <rect x="6" y="5" width="8" height="10" fill="currentColor"/>
                    </svg>
                    { "Stop" }
                </button>
            }
        } else {
            html! {
                <button 
                    class="flex items-center justify-center bg-green-600 hover:bg-green-700 text-white font-semibold py-2 px-4 rounded-lg shadow-md transition-colors duration-300 w-24"
                    onclick={ctx.link().callback(|_| Msg::Start)}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
                        <path d="M8 5v10l8-5-8-5z" fill="currentColor"/>
                    </svg>
                    { "Start" }
                </button>
            }
        };

        html! {
            <div class="min-h-screen bg-gradient-to-br from-gray-100 to-gray-200">
                <header class="bg-gradient-to-r from-indigo-900 to-purple-800 shadow-lg py-6">
                    <div class="container mx-auto px-2 sm:px-4">
                        <h1 class="font-bold text-2xl sm:text-3xl text-white text-center">
                            { "Conway's Game of Life" }
                        </h1>
                    </div>
                </header>
                
                <main class="container mx-auto px-2 sm:px-4 py-4 sm:py-8">
                    <div class="bg-white rounded-xl shadow-xl p-2 sm:p-6 mb-4 sm:mb-8">
                        <div class="mb-2 sm:mb-4 text-center text-gray-700 text-sm sm:text-base">
                            <p>{ "Click on cells to toggle them alive/dead, then use the controls to run the simulation." }</p>
                        </div>
                        
                        <div class="flex justify-center mb-4 sm:mb-8">
                            <div class="overflow-x-auto">
                                <div class="inline-block border border-gray-300 rounded-md p-1 bg-gray-50">
                                    { for cell_rows }
                                </div>
                            </div>
                        </div>
                        
                        <div class="flex flex-wrap justify-center gap-2 sm:gap-3">
                            <button 
                                class="flex items-center justify-center bg-purple-600 hover:bg-purple-700 text-white font-semibold py-2 px-4 rounded-lg shadow-md transition-colors duration-300"
                                onclick={ctx.link().callback(|_| Msg::Random)}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
                                </svg>
                                { "Random" }
                            </button>
                            
                            <button 
                                class="flex items-center justify-center bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg shadow-md transition-colors duration-300"
                                onclick={ctx.link().callback(|_| Msg::Step)}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                                </svg>
                                { "Step" }
                            </button>
                            
                            { play_button }
                            
                            <button 
                                class="flex items-center justify-center bg-gray-600 hover:bg-gray-700 text-white font-semibold py-2 px-4 rounded-lg shadow-md transition-colors duration-300"
                                onclick={ctx.link().callback(|_| Msg::Reset)}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1z" clip-rule="evenodd" />
                                    <path fill-rule="evenodd" d="M10.293 15.707a1 1 0 010-1.414L12.586 12l-2.293-2.293a1 1 0 111.414-1.414l3 3a1 1 0 010 1.414l-3 3a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                                </svg>
                                { "Reset" }
                            </button>
                        </div>
                    </div>
                    
                    <div class="bg-white rounded-xl shadow-lg p-2 sm:p-6">
                        <h2 class="text-lg sm:text-xl font-semibold text-gray-800 mb-2 sm:mb-4">{ "Rules of Conway's Game of Life" }</h2>
                        <ul class="list-disc list-inside space-y-1 sm:space-y-2 text-gray-700 text-sm sm:text-base">
                            <li>{ "Any live cell with fewer than two live neighbors dies (underpopulation)" }</li>
                            <li>{ "Any live cell with two or three live neighbors survives" }</li>
                            <li>{ "Any live cell with more than three live neighbors dies (overpopulation)" }</li>
                            <li>{ "Any dead cell with exactly three live neighbors becomes alive (reproduction)" }</li>
                        </ul>
                    </div>
                </main>
                
                <footer class="bg-gray-800 text-white text-center py-2 sm:py-4 mt-8 sm:mt-12 text-xs sm:text-base">
                    <p>{ "Conway's Game of Life - Implemented with Yew and Rust" }</p>
                </footer>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}