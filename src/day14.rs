use aoc_runner_derive::{aoc, aoc_generator};

const NUM_RECIPES: usize = 10;

#[aoc_generator(day14)]
fn input_generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
fn solve_part1(input: &usize) -> String {
    let mut recipes = vec![3, 7];
    let (mut elf_one, mut elf_two) = (0, 1);
    while recipes.len() < (input + NUM_RECIPES) {
        let a = recipes[elf_one];
        let b = recipes[elf_two];

        let mut new_recipes: Vec<usize> = (a + b)
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        recipes.append(&mut new_recipes);

        elf_one = (1 + elf_one + a) % recipes.len();
        elf_two = (1 + elf_two + b) % recipes.len();
    }
    recipes
        .iter()
        .skip(*input)
        .take(NUM_RECIPES)
        .map(|i| i.to_string())
        .collect::<String>()
}

#[aoc(day14, part2)]
fn solve_part2(input: &usize) -> usize {
    let mut recipes = vec![3, 7];
    let (mut elf_one, mut elf_two) = (0, 1);
    let target_recipe = input
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    loop {
        let a = recipes[elf_one];
        let b = recipes[elf_two];

        let new_recipes: Vec<usize> = (a + b)
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        for recipe in new_recipes {
            recipes.push(recipe);
            if recipes.ends_with(&target_recipe) {
                return recipes.len() - target_recipe.len();
            }
        }

        elf_one = (1 + elf_one + a) % recipes.len();
        elf_two = (1 + elf_two + b) % recipes.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&9), "5158916779");
        assert_eq!(solve_part1(&18), "9251071085");
        assert_eq!(solve_part1(&2018), "5941429882");
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&51589), 9);
        assert_eq!(solve_part2(&92510), 18);
        assert_eq!(solve_part2(&59414), 2018);
    }
}
