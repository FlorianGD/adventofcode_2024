today := shell("date -u +%d")
tomorrow := shell("date -d '+1 day' +%d")

tomorrow: (new_day tomorrow)
today: (new_day today)
part2_today: (part2 today)

new_day day:
    echo "pub mod day{{day}};" >> src/lib.rs
    cp day_template.txt "src/day{{day}}.rs"
    sd "use adventofcode2024::\{" "use adventofcode2024::{day{{ day }}," src/main.rs
    sd ',\n    ],' ",\n solution! {{ "{" }}{{ trim_start_match(day, '0') }}, parser!{ day{{ day }}::parse_input }, solver!{ day{{ day }}::part1 }}]" src/main.rs
    cargo fmt

part2 day:
    sd ',\n    ],' ",\n solution! {{ "{" }}{{ trim_start_match(day, '0') }}, parser!{ day{{ day }}::parse_input }, solver!{ day{{ day }}::part2 }}]" src/main.rs
    cargo fmt
