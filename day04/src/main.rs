use std::fs;

type ElfSections = (u32, u32);
type InputRow = (ElfSections, ElfSections);
type InputType = Vec<InputRow>;


fn read_input(filename: &str) -> InputType {
    fn build_section_range(section_range: &str) -> ElfSections {
        let section_extremes: Vec<u32> = section_range.split('-')
            .map(|section| section.parse::<u32>().unwrap())
            .collect();

        (section_extremes[0], section_extremes[1])
    }

    fn build_pair_sections(pair_section: &str) -> InputRow {
        let mut parsed_sections = pair_section.split(',')
            .map(build_section_range);

        let first_sections = parsed_sections.next().unwrap();
        let second_sections = parsed_sections.next().unwrap();

        (first_sections, second_sections)
    }

    fs::read_to_string(filename)
        .expect("Cannot read the input")
        .trim()
        .split('\n')
        .map(build_pair_sections)
        .collect()
}

fn is_a_section_in_the_other(first_sections: &ElfSections, second_sections: &ElfSections) -> bool {
    let first_in_second = (first_sections.0 >= second_sections.0) && (first_sections.1 <= second_sections.1);
    let second_in_first = (first_sections.0 <= second_sections.0) && (first_sections.1 >= second_sections.1);

    first_in_second || second_in_first
}

fn step_1(elf_sections_pairs: &InputType) -> u32 {
    elf_sections_pairs.iter()
        .map(|sections_pair| is_a_section_in_the_other(&sections_pair.0, &sections_pair.1))
        .filter(|is_first_in_second| *is_first_in_second)
        .count() as u32
}

fn are_sections_overlap(first_sections: &ElfSections, second_sections: &ElfSections) -> bool {
    let is_first_extreme_in_second = first_sections.0 >= second_sections.0 && first_sections.0 <= second_sections.1;
    let is_second_extreme_in_second = first_sections.1 >= second_sections.0 && first_sections.1 <= second_sections.1;

    let is_first_extreme_in_first = second_sections.0 >= first_sections.0 && second_sections.0 <= first_sections.1;
    let is_second_extreme_in_first = second_sections.1 >= first_sections.0 && second_sections.1 <= first_sections.1;

    is_first_extreme_in_second || is_second_extreme_in_second || is_first_extreme_in_first || is_second_extreme_in_first
}

fn step_2(elf_sections_pairs: &InputType) -> u32 {
    elf_sections_pairs.iter()
        .map(|sections_pair| are_sections_overlap(&sections_pair.0, &sections_pair.1))
        .filter(|is_first_in_second| *is_first_in_second)
        .count() as u32
}

fn main() {
    let sections = read_input("input.txt");

    assert_eq!(step_1(&sections), 605);
    assert_eq!(step_2(&sections), 914);
}
