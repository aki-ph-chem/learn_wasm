mod utils;

#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// セルの生死を切り替える
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    // セルの座標(row,column)からVec<Cell>におけるindexを計算する
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // 近接する生きているセルをカウントする
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0; 
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    // Get the dead and alive values of entire Universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // set cells be alive in a universe by passing the row and column
    pub fn set_cells(&mut self, cells: &[(u32,u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl Universe {
    // 次の世代を計算する
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // life gameのルールを反映
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (othterwise, _) => othterwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    // コンストラクタ
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive              
                } else {
                    Cell::Dead
                }
            })
        .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    // 宇宙を宇宙船で初期化する
    pub fn new_space_ship() -> Universe {
        let width = 64;
        let height = 64;

        let mut cells = vec![Cell::Dead; (width * height) as usize];

        let index = |i,j| { i + width as usize * j};

        // 一機目の宇宙船
        cells[index(2,0)] = Cell::Alive;

        cells[index(0,1)] = Cell::Alive;
        cells[index(4,1)] = Cell::Alive;

        cells[index(5,2)] = Cell::Alive;

        cells[index(0,3)] = Cell::Alive;
        cells[index(5,3)] = Cell::Alive;

        cells[index(1,4)] = Cell::Alive;
        cells[index(2,4)] = Cell::Alive;
        cells[index(3,4)] = Cell::Alive;
        cells[index(4,4)] = Cell::Alive;
        cells[index(5,4)] = Cell::Alive;

        // 二機目の宇宙船
        // 一機目からは(10,10)だけ離れている
        cells[index(12,10)] = Cell::Alive;

        cells[index(10,11)] = Cell::Alive;
        cells[index(14,11)] = Cell::Alive;

        cells[index(15,12)] = Cell::Alive;

        cells[index(10,13)] = Cell::Alive;
        cells[index(15,13)] = Cell::Alive;

        cells[index(11,14)] = Cell::Alive;
        cells[index(12,14)] = Cell::Alive;
        cells[index(13,14)] = Cell::Alive;
        cells[index(14,14)] = Cell::Alive;
        cells[index(15,14)] = Cell::Alive;

        // 三機目の宇宙船
        // 一機目からは(30,30)だけ離れている
        cells[index(32,30)] = Cell::Alive;

        cells[index(30,31)] = Cell::Alive;
        cells[index(34,31)] = Cell::Alive;

        cells[index(35,32)] = Cell::Alive;

        cells[index(30,33)] = Cell::Alive;
        cells[index(35,33)] = Cell::Alive;

        cells[index(31,34)] = Cell::Alive;
        cells[index(32,34)] = Cell::Alive;
        cells[index(33,34)] = Cell::Alive;
        cells[index(34,34)] = Cell::Alive;
        cells[index(35,34)] = Cell::Alive;

        Universe {
            width,
            height,
            cells,
        }
    }

    // set the width of the Universe.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_i| Cell::Dead)
            .collect();
    }

    // set the height of the Universe
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::Dead)
            .collect();
    }

    // (row, column)のセルの生死を切り替える
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    } 

    // レンダリング
    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

// Universeに対してstd::ftm::Displayを実装する
impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {'□'} else {'■'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
