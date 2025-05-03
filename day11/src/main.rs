use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn main() {
    let initial_state = State {
        elevator: 0,
        positions: [(0, 0), (2, 1), (2, 1), (2, 1), (2, 1)],
    };
    let target_state = State {
        elevator: 3,
        positions: [(3, 3), (3, 3), (3, 3), (3, 3), (3, 3)],
    };
    let part1 = bfs(&initial_state, &target_state);

    let initial_state = State {
        elevator: 0,
        positions: [(0, 0), (2, 1), (2, 1), (2, 1), (2, 1), (0, 0), (0, 0)],
    };
    let target_state = State {
        elevator: 3,
        positions: [(3, 3), (3, 3), (3, 3), (3, 3), (3, 3), (3, 3), (3, 3)],
    };
    let part2 = bfs(&initial_state, &target_state);

    println!("{part1} {part2}");
}

fn bfs<const N: usize>(initial_state: &State<N>, target_state: &State<N>) -> u32 {
    let mut open: VecDeque<State<N>> = VecDeque::new();
    let mut distances: HashMap<State<N>, u32> = HashMap::new();

    open.push_back(initial_state.clone());
    distances.insert(initial_state.clone(), 0);

    while !distances.contains_key(&target_state) {
        let current = open.pop_front().unwrap();

        for neighbor in current.next_states() {
            if !distances.contains_key(&neighbor) {
                open.push_back(neighbor.clone());
                distances.insert(neighbor, distances[&current] + 1);
            }
        }
    }

    distances[&target_state]
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct State<const N: usize> {
    elevator: u32,
    /// [`positions[n].0`] is the floor of the microchip for the nth element,
    /// [`positions[n].1`] is the floor of the generator for the nth element.
    ///
    /// Since all elements are interchangeable (as long as we change microchips and generators simulateously),
    /// we may keep this list sorted at all times, which vastly decreases the state space without influencing the result.
    /// This is ensured in [`State<N>::new`].
    /// The speed-up factor by this optimization is ~200.
    positions: [(u32, u32); N],
}

impl<const N: usize> State<N> {
    fn new(elevator: u32, mut positions: [(u32, u32); N]) -> State<N> {
        positions.sort();
        State { elevator, positions }
    }

    fn is_valid(&self) -> bool {
        self.positions
            .iter()
            .all(|(c, g)| c == g || !self.positions.iter().any(|(_, g_2)| g_2 == c))
    }

    fn next_states(&self) -> impl IntoIterator<Item = State<N>> {
        let reachable_floors: Vec<_> = match self.elevator {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            3 => vec![2],
            _ => unreachable!(),
        };

        reachable_floors
            .into_iter()
            .flat_map(|to| {
                self.move_one_chip(to)
                    .into_iter()
                    .chain(self.move_one_gen(to))
                    .chain(self.move_two_gens(to))
                    .chain(self.move_two_chips(to))
                    .chain(self.move_one_chip_one_gen(to))
            })
            .filter(State::is_valid)
    }

    fn move_one_chip(&self, to: u32) -> impl IntoIterator<Item = State<N>> {
        self.positions
            .iter()
            .enumerate()
            .filter(move |(_, pair)| pair.0 == self.elevator)
            .map(move |(idx, (_, _))| {
                let mut new_positions = self.positions.clone();
                new_positions.get_mut(idx).unwrap().0 = to;
                State::new(to, new_positions)
            })
    }

    fn move_one_gen(&self, to: u32) -> impl IntoIterator<Item = State<N>> {
        self.positions
            .iter()
            .enumerate()
            .filter(move |(_, pair)| pair.1 == self.elevator)
            .map(move |(idx, (_, _))| {
                let mut new_positions = self.positions.clone();
                new_positions.get_mut(idx).unwrap().1 = to;
                State::new(to, new_positions)
            })
    }

    fn move_one_chip_one_gen(&self, to: u32) -> impl IntoIterator<Item = State<N>> {
        let chip_indices = self
            .positions
            .iter()
            .enumerate()
            .filter(move |(_, pair)| pair.0 == self.elevator)
            .map(|(idx, _)| idx);
        let gen_indices = self
            .positions
            .iter()
            .enumerate()
            .filter(move |(_, pair)| pair.1 == self.elevator)
            .map(|(idx, _)| idx);

        let pairs = chip_indices.cartesian_product(gen_indices);

        pairs.map(move |(idx_c, idx_g)| {
            let mut new_positions = self.positions.clone();
            new_positions.get_mut(idx_c).unwrap().0 = to;
            new_positions.get_mut(idx_g).unwrap().1 = to;
            State::new(to, new_positions)
        })
    }

    fn move_two_chips(&self, to: u32) -> impl IntoIterator<Item = State<N>> {
        let chip_indices = self
            .positions
            .iter()
            .enumerate()
            .filter(move |(_, pair)| pair.0 == self.elevator)
            .map(|(idx, _)| idx);

        let pairs = chip_indices.clone().cartesian_product(chip_indices);

        pairs
            .filter(|(idx_c1, idx_c2)| idx_c1 != idx_c2)
            .map(move |(idx_c1, idx_c2)| {
                let mut new_positions = self.positions.clone();
                new_positions.get_mut(idx_c1).unwrap().0 = to;
                new_positions.get_mut(idx_c2).unwrap().0 = to;
                State::new(to, new_positions)
            })
    }

    fn move_two_gens(&self, to: u32) -> impl IntoIterator<Item = State<N>> {
        let gen_indices = self
            .positions
            .iter()
            .enumerate()
            .filter(move |(_, pair)| pair.1 == self.elevator)
            .map(|(idx, _)| idx);

        let pairs = gen_indices.clone().cartesian_product(gen_indices);

        pairs
            .filter(|(idx_g1, idx_g2)| idx_g1 != idx_g2)
            .map(move |(idx_g1, idx_g2)| {
                let mut new_positions = self.positions.clone();
                new_positions.get_mut(idx_g1).unwrap().1 = to;
                new_positions.get_mut(idx_g2).unwrap().1 = to;
                State::new(to, new_positions)
            })
    }
}
