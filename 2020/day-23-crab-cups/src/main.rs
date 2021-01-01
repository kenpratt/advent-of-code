static INPUT: &str = "916438275";

fn main() {
    println!("part 1 result: {:?}", part1(INPUT, 100));
    println!("part 2 result: {:?}", part2(INPUT, 10000000));
}

#[derive(Debug)]
struct Game {
    size: usize,
    pointers: Vec<usize>,
    current: usize,
}

impl Game {
    fn parse(input: &str, fill_cups: Option<usize>) -> Game {
        let mut cups: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).map(|c| c - 1).collect();

        if fill_cups.is_some() {
            let fill_from = cups.len();
            let fill_to = fill_cups.unwrap();       
            let fill: Vec<usize> = (fill_from..fill_to).collect();    
            cups.extend(fill);
        }

        let size = cups.len();

        let mut pointers: Vec<usize> = vec![0; size];
        for i in 0..size {
            let j = (i + 1) % size;
            let curr_cup = cups[i];
            let next_cup = cups[j];
            pointers[curr_cup] = next_cup;
        }

        return Game {
            size: size,
            pointers: pointers,
            current: cups[0],
        }
    }

    fn run(&mut self, moves: usize) {
        //println!("initial state: {}\n", self.labels_debug());
        for m in 0..moves {
            if m % 1000000 == 0 {
                println!("-- move {} --", m);
            }
            self.tick();
            //println!("");
        }
        //println!("final state: {}", self.labels_debug());
    }

    fn tick(&mut self) {
        //println!("cups: {}", self.labels_debug());

        let current = self.current;
        let grab0 = self.pointers[current];
        let grab1 = self.pointers[grab0];
        let grab2 = self.pointers[grab1];
        //println!("pick up: {}, {}, {}", grab0+1, grab1+1, grab2+1);

        let mut destination = (current + self.size - 1) % self.size;
        while destination == grab0 || destination == grab1 || destination == grab2 {
            destination = (destination + self.size - 1) % self.size;
        }
        //println!("destination: {}", destination+1);
        
        let after_grab2 = self.pointers[grab2];
        let after_destination = self.pointers[destination];

        // 3 pointers change:
        // - current to the number following the picked up numbers
        // - destination to the start of the picked up numbers
        // - the last picked up number to the new home
        self.pointers[current] = after_grab2;
        self.pointers[destination] = grab0;
        self.pointers[grab2] = after_destination;

        // current changes to the next number (after changing current pointer)
        self.current = self.pointers[current];
    }

    fn cups_from_reference_point(&self, reference_base_1: usize, num: usize) -> Vec<usize> {
        let reference = reference_base_1 - 1;
        let mut cup = self.pointers[reference];
        let mut acc = vec![];
        for _ in 0..num {
            acc.push(cup);
            cup = self.pointers[cup];
        }
        acc.iter().map(|c| c + 1).collect() // fix back to base 1
    }

    fn labels_from_reference_point(&self, reference_base_1: usize) -> String {
        let reference = reference_base_1 - 1;
        let mut cup = self.pointers[reference];
        let mut acc = vec![];
        while cup != reference {
            acc.push(cup);
            cup = self.pointers[cup];
        }
        acc.iter().map(|c| (c+1).to_string()).collect::<Vec<String>>().join("")
    }

    fn labels_debug(&self) -> String {
        let mut cup = self.pointers[self.current];
        let mut acc = vec![];
        while cup != self.current {
            acc.push(cup);
            cup = self.pointers[cup];
        }
        format!("({}) {}", self.current+1, acc.iter().map(|c| (c+1).to_string()).collect::<Vec<String>>().join(" "))
    }
}

fn part1(input: &str, moves: usize) -> String {
    let mut game = Game::parse(input, None);
    game.run(moves);
    game.labels_from_reference_point(1)
}

fn part2(input: &str, moves: usize) -> usize {
    let mut game = Game::parse(input, Some(1000000));
    game.run(moves);
    game.cups_from_reference_point(1, 2).iter().fold(1, |acc, c| acc * c)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "389125467";

    #[test]
    fn test_part1_example1_10_moves() {
        let result = part1(EXAMPLE1, 10);
        assert_eq!(&result, "92658374");
    }

    #[test]
    fn test_part1_example1_100_moves() {
        let result = part1(EXAMPLE1, 100);
        assert_eq!(&result, "67384529");
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(INPUT, 100);
        assert_eq!(&result, "39564287");
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1, 10000000);
        assert_eq!(result, 149245887792);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(INPUT, 10000000);
        assert_eq!(result, 404431096944);
    }
}