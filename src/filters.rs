use std::error::Error;

/**
 * @params
 * attr - number of the record incsv file ex. BICEP -> 7
 * value - what value we expect from attribute ex. bicep -> 7 <- VALUE
 * list (optional) - list of ID's of exercises to filter, when None, algorithm will filter all
 * deviation (optional) - Sets the maximum spread of values, if None then 0
 */
pub fn filter_by_attribute_simple(
    attr: usize,
    value: u8,
    list: Option<&[u8]>,
    deviation: Option<u8>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    if attr == 0 || attr == 1 {
        panic!("Can't filter by attribute 0(ID) or 1(Name).")
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data.csv")?;

    let mut returned_id_list: Vec<u8> = vec![];

    for result in rdr.records() {
        let record = result?;
        let check_attr: u8 = record[attr].parse()?;

        let dev = match deviation {
            Some(d) => d,
            None => 0,
        };

        if check_attr <= value + dev && check_attr as i8 >= value as i8 - dev as i8 {
            match list {
                Some(x) => {
                    let id: u8 = record[0].parse()?;
                    if x.contains(&id) {
                        returned_id_list.push(id);
                    }
                }
                None => returned_id_list.push(record[0].parse()?),
            }
        }
    }
    Ok(returned_id_list)
}
