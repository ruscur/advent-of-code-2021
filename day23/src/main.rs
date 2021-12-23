use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn from_room(room: usize) -> Amphipod {
        match room {
            0 => Amphipod::A,
            1 => Amphipod::B,
            2 => Amphipod::C,
            3 => Amphipod::D,
            _ => unreachable!(),
        }
    }

    fn energy(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    // Return the index of the doorway leading to our room
    fn doorway(&self) -> usize {
        match self {
            Amphipod::A => 2,
            Amphipod::B => 4,
            Amphipod::C => 6,
            Amphipod::D => 8,
        }
    }

    fn room(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RoomSize {
    Two([[Option<Amphipod>; 2]; 4]),
    Four([[Option<Amphipod>; 4]; 4]),
}

impl RoomSize {
    fn get(&self, n: usize) -> &[Option<Amphipod>] {
        match self {
            RoomSize::Two(v) => &v[n],
            RoomSize::Four(v) => &v[n],
        }
    }

    fn set(&mut self, n: usize, idx: usize, val: Option<Amphipod>) {
        match self {
            RoomSize::Two(v) => v[n][idx] = val,
            RoomSize::Four(v) => v[n][idx] = val,
        }
    }

    fn get_insert_idx(&self, room: usize) -> usize {
        match self {
            RoomSize::Two(v) => {
                let room = v[room];
                match [1, 0].iter().find(|&n| room[*n].is_none()) {
                    Some(n) => *n,
                    None => unreachable!(),
                }
            }
            RoomSize::Four(v) => {
                let room = v[room];
                match [3, 2, 1, 0].iter().find(|&n| room[*n].is_none()) {
                    Some(n) => *n,
                    None => unreachable!(),
                }
            }
        }
    }

    fn get_remove_idx(&self, room: usize) -> usize {
        match self {
            RoomSize::Two(v) => {
                let room = v[room];
                match [0, 1].iter().find(|&n| room[*n].is_some()) {
                    Some(n) => *n,
                    None => unreachable!(),
                }
            }
            RoomSize::Four(v) => {
                let room = v[room];
                match [0, 1, 2, 3].iter().find(|&n| room[*n].is_some()) {
                    Some(n) => *n,
                    None => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Burrow {
    hallway: [Option<Amphipod>; 11],
    rooms: RoomSize,
}

impl Burrow {
    fn from_2(rooms: [[Amphipod; 2]; 4]) -> Burrow {
        Burrow {
            hallway: [None; 11],
            rooms: RoomSize::Two([
                [Some(rooms[0][0]), Some(rooms[0][1])],
                [Some(rooms[1][0]), Some(rooms[1][1])],
                [Some(rooms[2][0]), Some(rooms[2][1])],
                [Some(rooms[3][0]), Some(rooms[3][1])],
            ]),
        }
    }

    fn from_4(rooms: [[Amphipod; 4]; 4]) -> Burrow {
        Burrow {
            hallway: [None; 11],
            rooms: RoomSize::Four([
                [
                    Some(rooms[0][0]),
                    Some(rooms[0][1]),
                    Some(rooms[0][2]),
                    Some(rooms[0][3]),
                ],
                [
                    Some(rooms[1][0]),
                    Some(rooms[1][1]),
                    Some(rooms[1][2]),
                    Some(rooms[1][3]),
                ],
                [
                    Some(rooms[2][0]),
                    Some(rooms[2][1]),
                    Some(rooms[2][2]),
                    Some(rooms[2][3]),
                ],
                [
                    Some(rooms[3][0]),
                    Some(rooms[3][1]),
                    Some(rooms[3][2]),
                    Some(rooms[3][3]),
                ],
            ]),
        }
    }

    // Returns the updated burrow and the distance traveled.
    fn hallway_move(&self, hallway_pos: usize, room_id: usize) -> (Burrow, usize) {
        let mut updated = self.clone();
        let hallway = self.hallway[hallway_pos];
        let doorway_pos = (room_id + 1) * 2;
        let room = self.rooms.get(room_id);

        let room_pos = match hallway {
            Some(_) => self.rooms.get_insert_idx(room_id),
            None => self.rooms.get_remove_idx(room_id),
        };

        updated.hallway[hallway_pos] = room[room_pos];
        updated.rooms.set(room_id, room_pos, hallway);

        (
            updated,
            room_pos + 1 + doorway_pos.max(hallway_pos) - doorway_pos.min(hallway_pos),
        )
    }

    // Returns the updated burrow and the distance traveled.
    fn room_move(&self, room_src: usize, room_dst: usize) -> (Burrow, usize) {
        let mut updated = self.clone();
        let src_doorway = (room_src + 1) * 2;
        let dst_doorway = (room_dst + 1) * 2;
        let hallway_dist = src_doorway.max(dst_doorway) - src_doorway.min(dst_doorway) + 1;

        let room_src_pos = self.rooms.get_remove_idx(room_src);
        let room_dst_pos = self.rooms.get_insert_idx(room_dst);

        updated.rooms.set(
            room_src,
            room_src_pos,
            self.rooms.get(room_dst)[room_dst_pos],
        );
        updated.rooms.set(
            room_dst,
            room_dst_pos,
            self.rooms.get(room_src)[room_src_pos],
        );

        (updated, 1 + room_src_pos + room_dst_pos + hallway_dist)
    }

    fn get_room_population(&self, room: usize) -> usize {
        self.rooms.get(room).iter().filter(|x| x.is_some()).count()
    }

    fn is_room_enterable(&self, room_id: usize) -> bool {
        let room = self.rooms.get(room_id);
        let allowed = [None, Some(Amphipod::from_room(room_id))];
        for pos in room {
            if !allowed.contains(pos) {
                return false;
            }
        }
        true
    }

    fn is_room_done(&self, room: usize) -> bool {
        match self.rooms {
            RoomSize::Two(v) => v[room] == DONE_2[room].map(|n| Some(n)),
            RoomSize::Four(v) => v[room] == DONE_4[room].map(|n| Some(n)),
        }
    }

    fn is_burrow_done(&self) -> bool {
        (0..4).filter(|n| self.is_room_done(*n)).count() == 4
    }

    fn is_path_clear(&self, pos1: usize, pos2: usize) -> bool {
        let start = pos1.min(pos2);
        let end = pos1.max(pos2);

        (start..=end).filter(|&p| self.hallway[p].is_some()).count() == 0
    }

    // Returns all possible burrow states from a single move and the cost to reach them.
    fn get_possible_moves(&self) -> Vec<(Burrow, usize)> {
        let mut moves = vec![];

        // Check the hallway first.  The only moves they can make is to their room.
        for i in [0, 1, 3, 5, 7, 9, 10] {
            if let Some(guy) = self.hallway[i] {
                // Check if we can enter our room.
                if self.is_room_enterable(guy.room()) {
                    // Is there anyone in the way?
                    let clear = if i < guy.doorway() {
                        self.is_path_clear(i + 1, guy.doorway())
                    } else {
                        self.is_path_clear(i - 1, guy.doorway())
                    };
                    if clear {
                        let (moved, dist) = self.hallway_move(i, guy.room());
                        moves.push((moved, dist * guy.energy()));
                    }
                }
            } else {
                // Nothing here.  Can someone from a room move here?
                for j in 0..4 {
                    if !self.is_room_done(j) && !self.is_room_enterable(j) {
                        if self.is_path_clear(i, (j + 1) * 2) {
                            let (moved, dist) = self.hallway_move(i, j);
                            let guy = moved.hallway[i].unwrap();
                            moves.push((moved, dist * guy.energy()));
                        }
                    }
                }
            }
        }

        let can_arrive: Vec<usize> = (0..4).filter(|n| self.is_room_enterable(*n)).collect();
        let can_depart: Vec<usize> = (0..4)
            .filter(|n| {
                !self.is_room_done(*n)
                    && !self.is_room_enterable(*n)
                    && self.get_room_population(*n) > 0
            })
            .collect();

        for n in can_depart {
            // Who's leaving?
            if let Some(leaver) = self.rooms.get(n)[self.rooms.get_remove_idx(n)] {
                if can_arrive.contains(&leaver.room()) {
                    // OK great, but can we path there?
                    if self.is_path_clear((n + 1) * 2, leaver.doorway()) {
                        let (moved, dist) = self.room_move(n, leaver.room());
                        moves.push((moved, dist * leaver.energy()));
                    }
                }
            }
        }

        moves
    }
}

fn solve(burrow: &Burrow, best_scores: &mut HashMap<Burrow, usize>, total: usize) {
    if let Some(score) = best_scores.get(burrow) {
        if total >= *score {
            // We've been in the same position with a better score, so there's no point in proceeding.
            return;
        }
    }
    // Either we're thus far unseen, or we have a better score than last time.
    best_scores.insert(*burrow, total);
    if !burrow.is_burrow_done() {
        for (action, score) in burrow.get_possible_moves() {
            solve(&action, best_scores, total + score);
        }
    }
}

fn find_best_outcome(start: &Burrow) -> usize {
    let mut database: HashMap<Burrow, usize> = HashMap::new();
    let finished = match start.rooms {
        RoomSize::Two(_) => Burrow::from_2(DONE_2),
        RoomSize::Four(_) => Burrow::from_4(DONE_4),
    };

    for (outcome, score) in start.get_possible_moves() {
        solve(&outcome, &mut database, score);
    }

    *database.get(&finished).unwrap()
}

const DONE_2: [[Amphipod; 2]; 4] = [
    [Amphipod::A, Amphipod::A],
    [Amphipod::B, Amphipod::B],
    [Amphipod::C, Amphipod::C],
    [Amphipod::D, Amphipod::D],
];

const INPUT_2: [[Amphipod; 2]; 4] = [
    [Amphipod::D, Amphipod::B],
    [Amphipod::D, Amphipod::C],
    [Amphipod::B, Amphipod::A],
    [Amphipod::A, Amphipod::C],
];

const DONE_4: [[Amphipod; 4]; 4] = [
    [Amphipod::A; 4],
    [Amphipod::B; 4],
    [Amphipod::C; 4],
    [Amphipod::D; 4],
];

const INPUT_4: [[Amphipod; 4]; 4] = [
    [Amphipod::D, Amphipod::D, Amphipod::D, Amphipod::B],
    [Amphipod::D, Amphipod::C, Amphipod::B, Amphipod::C],
    [Amphipod::B, Amphipod::B, Amphipod::A, Amphipod::A],
    [Amphipod::A, Amphipod::A, Amphipod::C, Amphipod::C],
];

fn main() {
    let burrow_2 = Burrow::from_2(INPUT_2);
    let burrow_4 = Burrow::from_4(INPUT_4);

    println!("Part 1: {}", find_best_outcome(&burrow_2));
    println!("Part 2: {}", find_best_outcome(&burrow_4));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_2: [[Amphipod; 2]; 4] = [
        [Amphipod::B, Amphipod::A],
        [Amphipod::C, Amphipod::D],
        [Amphipod::B, Amphipod::C],
        [Amphipod::D, Amphipod::A],
    ];

    const EXAMPLE_4: [[Amphipod; 4]; 4] = [
        [Amphipod::B, Amphipod::D, Amphipod::D, Amphipod::A],
        [Amphipod::C, Amphipod::C, Amphipod::B, Amphipod::D],
        [Amphipod::B, Amphipod::B, Amphipod::A, Amphipod::C],
        [Amphipod::D, Amphipod::A, Amphipod::C, Amphipod::A],
    ];

    #[test]
    fn swap_test() {
        let burrow = Burrow::from_2(EXAMPLE_2);

        let (ex1_swap1, dist1) = burrow.hallway_move(3, 2);

        assert!(ex1_swap1.is_room_done(0) == false);
        assert!(ex1_swap1.is_burrow_done() == false);
        assert!(ex1_swap1.hallway[3] == Some(Amphipod::B));
        assert!(ex1_swap1.rooms.get(2)[0] == None);
        assert!(dist1 == 4);
        assert!(burrow
            .get_possible_moves()
            .contains(&(ex1_swap1, dist1 * 10)));

        let (ex1_swap2, dist2) = ex1_swap1.room_move(1, 2);

        assert!(ex1_swap1
            .get_possible_moves()
            .contains(&(ex1_swap2, dist2 * 100)));
        assert!(ex1_swap2.is_room_done(2));
        assert!(dist2 == 4);

        let (ex1_swap3, dist3) = ex1_swap2.hallway_move(5, 1);

        assert!(ex1_swap2
            .get_possible_moves()
            .contains(&(ex1_swap3, dist3 * 1000)));

        let (ex1_swap4, dist4) = ex1_swap3.hallway_move(3, 1);

        assert!(ex1_swap3
            .get_possible_moves()
            .contains(&(ex1_swap4, dist4 * 10)));
        assert!(dist3 == 3);
        assert!(dist4 == 3);
        assert!(ex1_swap4.hallway[5] == Some(Amphipod::D));
        assert!(ex1_swap4.rooms.get(1)[1] == Some(Amphipod::B));

        let (ex1_swap5, dist5) = ex1_swap4.room_move(0, 1);

        assert!(dist5 == 4);
        assert!(ex1_swap5.is_room_done(1));

        let (ex1_swap6, dist6) = ex1_swap5.hallway_move(7, 3);

        assert!(dist6 == 2);

        let (ex1_swap7, dist7) = ex1_swap6.hallway_move(9, 3);

        assert!(dist7 == 3);
        assert!(ex1_swap7.rooms.get(3) == [None, None]);

        let (ex1_swap8, dist8) = ex1_swap7.hallway_move(7, 3);

        assert!(dist8 == 3);

        let (ex1_swap9, dist9) = ex1_swap8.hallway_move(5, 3);

        assert!(dist9 == 4);
        assert!(ex1_swap9.is_room_done(3));

        let (ex1_swap10, dist10) = ex1_swap9.hallway_move(9, 0);

        assert!(dist10 == 8);

        assert!(ex1_swap10.is_burrow_done());
    }

    #[test]
    fn solve_test() {
        let burrow = Burrow::from_2(EXAMPLE_2);

        assert!(find_best_outcome(&burrow) == 12521);
    }

    #[test]
    fn part2_test() {
        let burrow = Burrow::from_4(EXAMPLE_4);

        assert!(find_best_outcome(&burrow) == 44169);
    }
}
