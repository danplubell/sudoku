use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::collect_cols::collect_cols;
use crate::model::collect_regions::collect_regions;
use crate::model::collect_rows::collect_rows;
use crate::model::cols::Cols;
use crate::model::regions::Regions;
use crate::model::rows::Rows;

#[derive(Clone, PartialEq, Debug)]
pub struct Puzzle {
    cells: Cells,
    regions: Regions,
    rows: Rows,
    cols: Cols,
}
impl Puzzle {
    fn new(value: &str) -> Result<Self, ParsePuzzleError> {
        Puzzle::try_from(value)
    }
}
#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum ParsePuzzleError {
    #[error("Value string is too long")]
    TooLong,
    #[error("Value string is too short")]
    TooShort,
    #[error("Value string contains alphabetic characters")]
    HasAlpha,
}
impl TryFrom<&str> for Puzzle {
    type Error = ParsePuzzleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().any(|c| !c.is_numeric()) {
            return Err(ParsePuzzleError::HasAlpha);
        }
        if value.len() > 81 {
            return Err(ParsePuzzleError::TooLong);
        }
        if value.len() < 81 {
            return Err(ParsePuzzleError::TooShort);
        }
        let cells = Cells::from(value);
        //regions
        //rows
        //cols
        let rows:Rows = collect_rows(&cells);
        let cols:Cols = collect_cols(&cells);
        let regions:Regions = collect_regions(&cells);
        Ok(Puzzle { cells, regions, rows, cols })
    }
}



#[cfg(test)]
mod tests {
    use crate::puzzle::{ParsePuzzleError, Puzzle};
    use std::convert::TryFrom;

    #[test]
    fn too_short() {
        let err = Puzzle::try_from("1234").unwrap_err();
        println!("{}", err);
        assert_eq!(err, ParsePuzzleError::TooShort);
    }

    #[test]
    fn not_all_digits() {
        let err = Puzzle::try_from("aaaaaaaaaaaaaaa").unwrap_err();

        assert_eq!(err, ParsePuzzleError::HasAlpha);
    }
}
