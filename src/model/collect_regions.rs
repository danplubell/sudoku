use crate::model::cell::Cell;
use crate::model::cells::Cells;
use crate::model::region::Region;
use crate::model::regions::Regions;
use std::array;

pub fn collect_regions(cells: &Cells) -> Regions {
    let roots: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];
    let mut regions = Regions::new();
    for (i, r) in roots.iter().enumerate() {
        let start = *r;
        let end = r + 18;
        let mut t = Vec::new();
        for i in (start..=end).step_by(9usize) {
            let start = i;
            let end = i + 3;
            for j in start..end {
                t.push(Cell::new(cells.get_at(j).unwrap().value()))
            }
        }
        let cells: [Cell; 9] = array::from_fn(|i| Cell::new(t.get(i).unwrap().value()));
        regions.add_region_at(Region::with_cells(cells), i);
    }
    regions
}

#[cfg(test)]
mod tests {
    use crate::model::cell::Cell;
    use crate::model::cells::Cells;
    use crate::model::collect_regions::collect_regions;
    use crate::model::region::Region;

    #[test]
    fn test_collect_regions() {

        let solution =
            "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
        let cells = Cells::from(solution);
        let r = collect_regions(&cells);
        let expected_cells = [
            Cell::new(3),
            Cell::new(1),
            Cell::new(8),
            Cell::new(5),
            Cell::new(7),
            Cell::new(2),
            Cell::new(9),
            Cell::new(4),
            Cell::new(6),
        ];
        let expected_region = Region::with_cells(expected_cells);
        assert_eq!(r.get_at(0), expected_region);
    }
}
