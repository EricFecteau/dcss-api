use crate::scenario_errors::{Error, ParsingError};

/// The (x, y) coordinate on a 2d grid
pub type Coord = (usize, usize);

/// Concordance key between branch `name:number` and the keyboard key.
///
/// This function takes the branch and floor number (e.g. `D:5`, `Elf:1`,
/// `Slime:2`) and returns the parsed name, the keyboard key and the floor
/// number as a [Tuple]. For example, `Slime:2` will return `(Slime, M, 2)`.
///
/// # Arguments
///
/// * `branch` - A &[str] with the name of the branch and floor number (e.g. `D:5`, `Elf:1`, `Slime:2`)
///
/// # Example
///
/// ```no_run
/// // Parse branch & floor
/// branch_keys("Slime:2")
/// ```
pub(crate) fn branch_keys(branch: &str) -> Result<(String, String, String), Error> {
    let (branch_name, branch_level) = branch.split_once(':').unwrap_or((branch, "0"));

    let branch_key = match branch_name {
        "D" => "D",
        "Dungeon" => "D",
        "Temple" => "T",
        "Lair" => "L",
        "Swamp" => "S",
        "Shoals" => "A",
        "Snake" => "P",
        "Spider" => "N",
        "Slime" => "M",
        "Orc" => "O",
        "Elf" => "E",
        "Vaults" => "V",
        "Crypt" => "C",
        "Tomb" => "W",
        "Depths" => "U",
        "Hell" => "H",
        "Dis" => "I",
        "Geh" => "G",
        "Coc" => "X",
        "Tar" => "Y",
        "Zot" => "Z",
        "Abyss" => "J",
        "Pan" => "R",
        "Zig" => "Q",
        "Bazaar" => "1",
        "Trove" => "2",
        "Sewer" => "3",
        "Ossuary" => "4",
        "Bailey" => "5",
        "IceCv" => "6",
        "Volcano" => "7",
        "WizLab" => "8",
        "Desolation" => "9",
        "Gauntlet" => "!",
        "Arena" => "\"",
        _ => Err(Error::DCSSParsing(ParsingError::UnknownBranch(
            branch_name.to_owned(),
        )))?,
    };

    Ok((
        branch_name.to_owned(),
        branch_key.to_owned(),
        branch_level.to_owned(),
    ))
}
