use z3::ast::{Ast, Int};

type Vector3 = (f64, f64, f64);

fn parse_input(input: &str) -> Vec<(Vector3, Vector3)> {
    input
        .lines()
        .filter_map(|x| x.split_once(" @ "))
        .map(|(a, b)| {
            let av: Vec<_> = a.splitn(3, ", ").filter_map(|x| x.parse().ok()).collect();
            let bv: Vec<_> = b.splitn(3, ", ").filter_map(|x| x.parse().ok()).collect();
            ((av[0], av[1], av[2]), (bv[0], bv[1], bv[2]))
        })
        .collect()
}

fn part_one_with_area(input: &str, min: f64, max: f64) -> u32 {
    let points = parse_input(input);

    let mut c = 0;

    let test_area = |x: f64, y: f64| x <= max && x >= min && y <= max && y >= min;

    for (i, ((x1, y1, _), (dx1, dy1, _))) in points.iter().copied().enumerate() {
        for ((x2, y2, _), (dx2, dy2, _)) in points.iter().skip(i + 1).copied() {
            let n2 = (x2 - x1 + (y1 - y2) * dx1 / dy1) / (dy2 * dx1 / dy1 - dx2);
            let n1 = (x1 - x2 + (y2 - y1) * dx2 / dy2) / (dy1 * dx2 / dy2 - dx1);

            if n1 <= 0.0 || n2 <= 0.0 {
                continue;
            }

            let x = x1 + n1 * dx1;
            let y = y1 + n1 * dy1;

            if test_area(x, y) {
                c += 1;
            }
        }
    }

    c
}
pub fn part_one(input: &str) -> Option<u32> {
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    Some(part_one_with_area(input, min, max))
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);

    let ctx = z3::Context::new(&z3::Config::new());
    let solver = z3::Solver::new(&ctx);

    let zero = Int::from_u64(&ctx, 0);
    let [a, b, c, da, db, dc] = ["a", "b", "c", "da", "db", "dc"].map(|x| Int::new_const(&ctx, x));

    for (i, ((x, y, z), (dx, dy, dz))) in points.iter().enumerate().take(3) {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|w| Int::from_i64(&ctx, *w as i64));
        let t = Int::new_const(&ctx, format!("t_{i}").as_str());
        solver.assert(&t.ge(&zero));
        solver.assert(&((&x + &dx * &t)._eq(&(&a + &da * &t))));
        solver.assert(&((&y + &dy * &t)._eq(&(&b + &db * &t))));
        solver.assert(&((&z + &dz * &t)._eq(&(&c + &dc * &t))));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let res = model.eval(&(&a + &b + &c), true).unwrap();

    res.as_u64()
}

aoc::solution!(24);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::template::read_file("examples", 24);
        let min = 7.0;
        let max = 27.0;
        assert_eq!(part_one_with_area(&input, min, max), 2);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 24)),
            Some(47)
        );
    }
}
