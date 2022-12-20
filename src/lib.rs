use circles::circle;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text, Column, Container, Row},
    {theme, Color, Length}, {Alignment, Element, Sandbox},
};
use rand::seq::SliceRandom;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
};
use PlayerOrComputer::*;
mod circles;

pub const WIDTH: u32 = 8;
pub const HEIGHT: u32 = 8;

pub fn pair_to_index(i: u32, j: u32) -> u32 {
    j + i * WIDTH as u32
}
pub fn index_to_pair(id: u32) -> (u32, u32) {
    let j = id % WIDTH as u32;
    let i = id / WIDTH as u32;
    (i, j)
}

#[derive(Clone)]
struct Node {
    pub value: u32,
    pub is_original_player: bool,
    pub board: Board,
    pub children: Vec<Node>,
}
impl Node {
    fn minmax(&mut self) -> u32 {
        if self.children.is_empty() {
            self.value
        } else {
            for child in &mut self.children {
                if child.value == 0 {
                    child.value = child.minmax()
                }
            }
            if !self.is_original_player {
                self.value = self
                    .children
                    .iter()
                    .max_by_key(|child| child.value)
                    .unwrap()
                    .value;
                self.value
            } else {
                self.value = self
                    .children
                    .iter()
                    .min_by_key(|child| child.value)
                    .unwrap()
                    .value;
                self.value
            }
        }
    }

    fn add_child(&mut self, node: Node) {
        self.children.push(node)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerOrComputer {
    Player,
    Computer,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StoneColor {
    White,
    Black,
}
impl StoneColor {
    fn reverse(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Tile(pub Option<StoneColor>);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameOutcome {
    Win(StoneColor),
    Draw,
    InProgress,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub board: Vec<Tile>,
    pub turn: StoneColor,
    pub win: GameOutcome,
    pub white_count: u32,
    pub black_count: u32,
}
impl Board {
    fn make_empty() -> Board {
        Board {
            board: (0..WIDTH * HEIGHT).map(|_| Tile(None)).collect(),
            turn: StoneColor::Black,
            win: GameOutcome::InProgress,
            white_count: 0,
            black_count: 0,
        }
    }
    fn starting_position(&mut self) {
        let id1 = pair_to_index(HEIGHT / 2 - 1, WIDTH / 2 - 1) as usize;
        let id2 = pair_to_index(HEIGHT / 2, WIDTH / 2 - 1) as usize;
        self.board[id1].0 = Some(StoneColor::White);
        self.board[id1 + 1].0 = Some(StoneColor::Black);
        self.board[id2].0 = Some(StoneColor::Black);
        self.board[id2 + 1].0 = Some(StoneColor::White);
        self.black_count = 2;
        self.white_count = 2;
    }
    pub fn new() -> Self {
        let mut board = Board::make_empty();
        board.starting_position();
        board
    }
    fn neighbours(id: u32) -> Vec<u32> {
        let (i, j) = index_to_pair(id);
        let mut out = Vec::with_capacity(8);
        if i + 1 < HEIGHT as u32 {
            let temp = pair_to_index(i + 1, j);
            out.push(temp);
        }
        if i > 0 {
            let temp = pair_to_index(i - 1, j);
            out.push(temp);
        }
        if j + 1 < WIDTH as u32 {
            let temp = pair_to_index(i, j + 1);
            out.push(temp);
        }
        if j > 0 {
            let temp = pair_to_index(i, j - 1);
            out.push(temp);
        }

        if i > 0 && j + 1 < WIDTH as u32 {
            let temp = pair_to_index(i - 1, j + 1);
            out.push(temp);
        }
        if j > 0 && i + 1 < HEIGHT as u32 {
            let temp = pair_to_index(i + 1, j - 1);
            out.push(temp);
        }
        if i + 1 < HEIGHT as u32 && j + 1 < WIDTH as u32 {
            let temp = pair_to_index(i + 1, j + 1);
            out.push(temp);
        }
        if i > 0 && j > 0 {
            let temp = pair_to_index(i - 1, j - 1);
            out.push(temp);
        }

        out
    }
    fn moves_are_possible(&self, color: StoneColor) -> bool {
        for id in 0..WIDTH * HEIGHT {
            if self.board[id as usize].0 != None
                || Self::neighbours(id).iter().all(|&neighbour| {
                    self.board[neighbour as usize].0 != Some(color.reverse())
                })
            {
                continue;
            };
            let (i, j) = index_to_pair(id);
            if self.clone().make_move(i, j, color) {
                return true;
            }
        }
        false
    }
    pub fn make_move(
        &mut self,
        row: u32,
        column: u32,
        color: StoneColor,
    ) -> bool {
        let id = pair_to_index(row, column);
        let (i2, j2) = index_to_pair(id);
        //let cloned_self = self.clone();
        let target = &mut self.board;
        let mut dumb_check = false;
        if target[id as usize].0 != None {
            return false;
        }
        for neighbour in Self::neighbours(id) {
            let (mut i, mut j) = index_to_pair(neighbour);
            let (dir_i, dir_j) = (i as i32 - i2 as i32, j as i32 - j2 as i32);

            let mut neighbour_color;
            let mut buf: Vec<u32> = Vec::with_capacity(20);
            let mut new_id = neighbour;

            let mut white_update = 0;
            let mut black_update = 0;
            if target[new_id as usize].0 == Some(color) {
                continue;
            }
            loop {
                neighbour_color = target[new_id as usize].0;
                if neighbour_color == None {
                    break;
                }

                if neighbour_color != None && neighbour_color != Some(color) {
                    match color {
                        StoneColor::Black => {
                            white_update -= 1;
                            black_update += 1;
                        }
                        StoneColor::White => {
                            white_update += 1;
                            black_update -= 1;
                        }
                    };
                    buf.push(new_id);
                } else if (neighbour_color == Some(color)) && !buf.is_empty() {
                    if !dumb_check {
                        match color {
                            StoneColor::Black => {
                                black_update += 1;
                            }
                            StoneColor::White => {
                                white_update += 1;
                            }
                        };
                        target[id as usize].0 = Some(color);
                    }
                    dumb_check = true;
                    buf.iter()
                        .for_each(|&id| target[id as usize].0 = Some(color));
                    self.black_count += black_update;
                    self.white_count += white_update;
                    break;
                }
                if (i == 0 && dir_i == -1)
                    || (i == HEIGHT - 1 && dir_i == 1)
                    || (j == 0 && dir_j == -1)
                    || (j == WIDTH - 1 && dir_j == 1)
                {
                    break;
                }
                (i, j) = ((i as i32 + dir_i) as u32, (j as i32 + dir_j) as u32);
                new_id = pair_to_index(i, j);
            }
        }
        dumb_check
    }
    fn count_of(&self, color: StoneColor) -> u32 {
        let (white_tiles, black_tiles) = (self.white_count, self.black_count);
        match color {
            StoneColor::White => white_tiles,
            StoneColor::Black => black_tiles,
        }
    }
    fn wincheck(&self) -> GameOutcome {
        let (white_tiles, black_tiles) = (self.white_count, self.black_count);
        if self.board.iter().all(|&x| x.0 != None)
            || (!self.moves_are_possible(StoneColor::Black)
                && !self.moves_are_possible(StoneColor::White))
        {
            match white_tiles.cmp(&black_tiles) {
                Ordering::Greater => GameOutcome::Win(StoneColor::White),
                Ordering::Less => GameOutcome::Win(StoneColor::Black),
                Ordering::Equal => GameOutcome::Draw,
            }
        } else {
            GameOutcome::InProgress
        }
    }
    pub fn minmax_move(&mut self, color: StoneColor) -> bool {
        let player = color;
        let opponent = player.reverse();
        let board = self.clone();
        let mut score = Node {
            is_original_player: false,
            value: 0,
            board,
            children: Vec::with_capacity(32),
        };
        let mut ids = Vec::with_capacity(32);
        for id in 0..WIDTH * HEIGHT {
            if self.board[id as usize].0 != None {
                continue;
            };
            let (row, column) = index_to_pair(id);
            let mut current_board = score.board.clone();
            if current_board.make_move(row, column, player) {
                ids.push(id);
                score.add_child(Node {
                    is_original_player: true,
                    value: 0,
                    board: current_board,
                    children: Vec::with_capacity(10),
                });
            }
        }
        for child in &mut score.children {
            Self::minmax_helper(opponent, child, 6, false, color);
        }
        let max = score.minmax();
        let mut zipped_ids_with_children_nodes: Vec<(&Node, u32)> =
            score.children.iter().zip(ids).collect();
        let mut rng = rand::thread_rng();
        zipped_ids_with_children_nodes.shuffle(&mut rng);
        for (node, id) in zipped_ids_with_children_nodes {
            if node.value == max {
                let (row, column) = index_to_pair(id);
                return self.make_move(row, column, color);
            }
        }
        false
    }
    fn minmax_helper(
        color: StoneColor,
        node: &mut Node,
        depth: u32,
        is_original_player: bool,
        orignal_color: StoneColor,
    ) {
        let player = color;
        let opponent = player.reverse();
        if depth == 2 {
            for id in 0..WIDTH * HEIGHT {
                if node.board.board[id as usize].0 != None
                    || Self::neighbours(id).iter().all(|&neighbour| {
                        node.board.board[neighbour as usize].0
                            != Some(color.reverse())
                    })
                {
                    continue;
                };
                let (row, column) = index_to_pair(id);
                let mut current_board = node.board.clone();
                let corner_ids =
                    [0, WIDTH - 1, WIDTH * HEIGHT - 1, (WIDTH) * (HEIGHT - 1)];
                let corner_boost = corner_ids.iter().fold(0, |acc, &id| {
                    if current_board.board[id as usize].0 == Some(orignal_color)
                    {
                        acc + 80
                    } else if current_board.board[id as usize].0 == None {
                        acc + 40
                    } else {
                        acc
                    }
                });
                if current_board.make_move(row, column, player) {
                    node.add_child(Node {
                        is_original_player: false,
                        value: 2 * current_board.count_of(orignal_color)
                            + 1
                            + corner_boost,
                        board: current_board,
                        children: vec![],
                    });
                }
            }
            return;
        }
        let mut max: u32 = 0;
        for id in 0..WIDTH * HEIGHT {
            if node.board.board[id as usize].0 != None {
                continue;
            };
            let (row, column) = index_to_pair(id);
            let mut current_board = node.board.clone();
            if current_board.make_move(row, column, player) {
                let score_player = current_board.count_of(player);
                if max < score_player {
                    max = score_player
                };
                if score_player > max / 2 {
                    node.add_child(Node {
                        is_original_player,
                        value: 0,
                        board: current_board.clone(),
                        children: Vec::with_capacity(10),
                    });
                }
            }
        }
        if node.children.is_empty() {
            node.value = 2 * node.board.count_of(orignal_color) + 1;
            return;
        }
        // for i in node.children.iter() {
        //     println!("{}", i.board)
        // }
        for child in &mut node.children {
            Self::minmax_helper(
                opponent,
                child,
                depth - 1,
                !is_original_player,
                orignal_color,
            );
        }
    }
    pub fn colored_move(
        &mut self,
        message: Message,
        mover_self: PlayerOrComputer,
        mover_other: PlayerOrComputer,
        color: StoneColor,
    ) {
        let (mover1, mover2) = match (mover_self, mover_other) {
            (Player, Player) => (&message, &message),
            (Player, Computer) => (&message, &Message::ComputerPlays),
            (Computer, Player) => (&Message::ComputerPlays, &message),
            (Computer, Computer) => unreachable!(),
        };

        if match *mover1 {
            Message::EmptyPressed(i, j) => self.make_move(i, j, color),
            Message::ComputerPlays => self.minmax_move(color),
            _ => false,
        } {
            let wincheck = self.wincheck();
            match wincheck {
                GameOutcome::InProgress => self.turn = color.reverse(),
                outcome => self.win = outcome,
            };
            if mover2 == &Message::ComputerPlays {
                self.colored_move(
                    mover1.clone(),
                    mover_other,
                    mover_self,
                    color.reverse(),
                )
            }
        }
        if !self.moves_are_possible(color) {
            if !self.moves_are_possible(color.reverse()) {
                let wincheck = self.wincheck();
                match wincheck {
                    GameOutcome::InProgress => {
                        unreachable!()
                    }
                    outcome => self.win = outcome,
                };
            } else {
                self.turn = color.reverse();
                self.colored_move(
                    mover2.clone(),
                    mover_other,
                    mover_self,
                    color.reverse(),
                )
            }
        }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut out = String::new();
        let (white_tiles, black_tiles) = (
            self.count_of(StoneColor::White),
            self.count_of(StoneColor::Black),
        );
        out += &format!("● {white_tiles}:{black_tiles} ○\n");
        out += "    ";
        for i in 0..WIDTH {
            out += &i.to_string().chars().next().unwrap_or(' ').to_string();
            out += " "
        }
        out += "\n  ";

        out += "  ";
        for i in 0..WIDTH {
            out += &i.to_string().chars().nth(1).unwrap_or(' ').to_string();
            out += " "
        }
        out += "\n  ";

        for _ in 0..WIDTH {
            out += "__"
        }
        out += "___";
        out += "\n |";
        for _ in 0..WIDTH {
            out += "  "
        }
        out += "   |\n";
        for i in 0..HEIGHT {
            let mut line = String::new();
            line.push_str(" |  ");
            for j in 0..WIDTH {
                let tile = &self.board[pair_to_index(i, j) as usize];
                let tile_string = match tile.0 {
                    None => "\x1B[1;93m□\x1B[0m".to_string(),
                    Some(StoneColor::White) => "●".to_string(),
                    Some(StoneColor::Black) => "○".to_string(),
                };

                line += &tile_string;
                line.push(' ')
            }

            line.push_str(" | ");
            line.push_str(&i.to_string());
            out += &line;
            out += "\n"
        }
        out += " |";
        for _ in 0..WIDTH {
            out += "__"
        }
        out += "___";
        out += "|\n";
        write!(f, "{}", out)
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    EmptyPressed(u32, u32),
    NonEmptyPressed(u32, u32),
    Reset,
    MenuMessage(MenuItem),
    ComputerPlays,
}

pub struct Game {
    game_board: Board,
    menu: Menu,
}

impl Sandbox for Game {
    type Message = Message;

    fn new() -> Self {
        Game {
            game_board: { Board::new() },
            menu: Menu::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Reversi - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MenuMessage(menu_item) => match menu_item {
                MenuItem::ChooseColor(color) => self.menu.chosen_color = color,
                MenuItem::Play => {
                    self.menu.play_pressed = true;
                    if self.game_board.turn != self.menu.chosen_color {
                        self.game_board.colored_move(
                            message,
                            Computer,
                            Player,
                            self.game_board.turn,
                        )
                    }
                }
            },
            Message::Reset => {
                *self = Self::new();
                self.game_board.starting_position();
            }
            message => {
                if self.game_board.turn == self.menu.chosen_color {
                    self.game_board.colored_move(
                        message,
                        Player,
                        Computer,
                        self.game_board.turn,
                    )
                } else {
                    self.game_board.colored_move(
                        message,
                        Computer,
                        Player,
                        self.game_board.turn,
                    )
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.menu.play_pressed {
            true => playfield(self),
            false => menu(self),
        }
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    ChooseColor(StoneColor),
    Play,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menu {
    chosen_color: StoneColor,
    play_pressed: bool,
}
impl Menu {
    fn new() -> Self {
        Menu {
            chosen_color: StoneColor::Black,
            play_pressed: false,
        }
    }
}

fn menu(game: &Game) -> Container<Message> {
    container(
        row![
            button("Play")
                .padding([40, 34])
                .on_press(Message::MenuMessage(MenuItem::Play))
                .style(theme::Button::Positive)
                .height(Length::Units(100))
                .width(Length::Units(100)),
            match game.menu.chosen_color {
                StoneColor::Black => button(circle(40.0, Color::BLACK))
                    .on_press(Message::MenuMessage(MenuItem::ChooseColor(
                        StoneColor::White
                    ))),
                StoneColor::White => button(circle(40.0, Color::WHITE))
                    .on_press(Message::MenuMessage(MenuItem::ChooseColor(
                        StoneColor::Black
                    ))),
            }
            .style(theme::Button::Positive)
            .padding(10)
            .height(Length::Units(100))
            .width(Length::Units(100))
        ]
        .spacing(10),
    )
    .center_x()
    .center_y()
}

fn playfield(game: &Game) -> Container<Message> {
    let (white_stones, black_stones) =
        (game.game_board.white_count, game.game_board.black_count);
    let tilebutton = |row: u32, column: u32| match game.game_board.board
        [pair_to_index(row, column) as usize]
        .0
    {
        Some(StoneColor::Black) => button(circle(30.0, Color::BLACK))
            .on_press(Message::EmptyPressed(row, column))
            .style(theme::Button::Positive),
        Some(StoneColor::White) => button(circle(30.0, Color::WHITE))
            .on_press(Message::EmptyPressed(row, column))
            .style(theme::Button::Positive),
        None => button(circle(30.0, Color::TRANSPARENT))
            .on_press(Message::EmptyPressed(row, column))
            .style(theme::Button::Positive),
    };
    let playboard = (0..WIDTH).fold(Row::new(), |acc, column| {
        let new_column = (0..HEIGHT).fold(Column::new(), |acc2, row| {
            acc2.push(tilebutton(row, column))
        });
        acc.push(new_column.spacing(2).align_items(Alignment::Center))
    });

    container(
        column![
            row![button("RESET")
                .on_press(Message::Reset)
                .style(theme::Button::Destructive),]
            .padding(20)
            .align_items(Alignment::Center),
            playboard.spacing(2).align_items(Alignment::Center),
            row![text(format!(
                "White:{white_stones}       Black:{black_stones}"
            ))]
            .padding(20)
            .align_items(Alignment::Center),
            row![text(match game.game_board.win {
                GameOutcome::Win(StoneColor::White) => {
                    "White wins!"
                }
                GameOutcome::Win(StoneColor::Black) => {
                    "Black wins!"
                }
                GameOutcome::Draw => {
                    "Draw!"
                }
                GameOutcome::InProgress => {
                    "Awaiting results..."
                }
            })]
        ]
        .padding(20)
        .align_items(Alignment::Center),
    )
}
