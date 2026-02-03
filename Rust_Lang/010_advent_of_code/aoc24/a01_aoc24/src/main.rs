mod data;
mod logic;
mod models;

use models::{Army, Combat, Result};
use std::io::{self, Write};

fn main() -> Result<()> {
    // Test combat - part 1
    let combat = Combat {
        army1: Army::test1_immune(),
        army2: Army::test1_infection(),
    };
    let winner = combat.fight_to_end();
    writeln!(
        io::stdout(),
        "test: {} wins with {} units left",
        winner.name,
        winner.total_live_units(),
    )?;

    // Real combat - part 1
    let combat = Combat {
        army1: Army::real_immune(),
        army2: Army::real_infection(),
    };
    let winner = combat.fight_to_end();
    writeln!(
        io::stdout(),
        "real: {} wins with {} units left",
        winner.name,
        winner.total_live_units(),
    )?;

    // Test combat - part 2 (with boost)
    let mut combat = Combat {
        army1: Army::test1_immune(),
        army2: Army::test1_infection(),
    };
    combat.army1.boost(1570);
    let winner = combat.fight_to_end();
    writeln!(
        io::stdout(),
        "test: {} wins with {} units left after {} boost",
        winner.name,
        winner.total_live_units(),
        1570,
    )?;

    // Real combat - part 2 (find minimal boost)
    // Trying this with boost values of 40 or 41 results in a combat that
    // does not appear to terminate. I just kept trying higher values until
    // I saw the first combat that terminated. 42 really is apparently the
    // ultimate answer to the ultimate question of life, the Universe and
    // Everything.
    for boost in 42.. {
        let mut combat = Combat {
            army1: Army::real_immune(),
            army2: Army::real_infection(),
        };
        combat.army1.boost(boost);
        let winner = combat.fight_to_end();
        if winner.name == "immune" {
            writeln!(
                io::stdout(),
                "real: {} wins with {} units left after {} boost",
                winner.name,
                winner.total_live_units(),
                boost,
            )?;
            return Ok(());
        } else if boost % 1 == 0 {
            writeln!(
                io::stdout(),
                "real: {} wins with {} units left after {} boost",
                winner.name,
                winner.total_live_units(),
                boost,
            )?;
        }
    }
    Err(From::from("no minimal boost could be found"))
}
