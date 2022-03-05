use http::StatusCode;
use json;
use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};

const DELAY: time::Duration = time::Duration::from_millis(200);
const PUG_REST: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/";
const PUG_VIEW: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/";

//******************************************************************/
/// Search for the compound and return the cid of the first result.
pub fn get_cid(compound_name: String) -> Result<isize, Box<dyn Error>> {
    let url = PUG_REST.to_owned() + "compound/name/" + &compound_name + "/cids/JSON";

    let res = reqwest::blocking::get(url)?;

    // Wait 200ms to avoid overloading the PubChem servers
    // 5 request per second TOP;
    thread::sleep(DELAY);

    if let StatusCode::OK = res.status() {
        let json = res.json::<HashMap<String, HashMap<String, Vec<isize>>>>()?;
        Ok(json["IdentifierList"]["CID"][0])
    } else {
        Ok(-1)
    }
}

//******************************************************************/
pub fn get_cas(cid: isize) -> Result<String, Box<dyn Error>> {
    let url: String =
        PUG_VIEW.to_owned() + "data/compound/" + &cid.to_string() + "/JSON?heading=CAS";

    let res = reqwest::blocking::get(url)?;

    // Wait 200ms to avoid overloading the PubChem servers
    // 5 request per second TOP;
    thread::sleep(DELAY);

    if let StatusCode::OK = res.status() {
        let txt = res.text()?;
        let parsed = json::parse(&txt)?;
        let cas = parsed["Record"]["Section"][0]["Section"][0]["Section"][0]["Information"][0]
            ["Value"]["StringWithMarkup"][0]["String"]
            .to_string();

        Ok(cas)
    } else {
        Ok("NA".to_string())
    }
}

//******************************************************************/
/// Returns the InChIKey and Canonical Smiles of the compound.
pub fn get_properties(cid: isize) -> Result<(String, String), Box<dyn Error>> {
    let url: String = PUG_REST.to_owned()
        + "compound/cid/"
        + &cid.to_string()
        + "/property/InChIKey,CanonicalSMILES/JSON";

    let res = reqwest::blocking::get(url)?;

    // Wait 200ms to avoid overloading the PubChem servers
    // 5 request per second TOP;
    thread::sleep(DELAY);

    if let StatusCode::OK = res.status() {
        let txt = res.text()?;
        let parsed = json::parse(&txt)?;
        let ref properties = parsed["PropertyTable"]["Properties"][0];

        let smiles = properties["CanonicalSMILES"].to_string();
        let inchikey = properties["InChIKey"].to_string();

        Ok((smiles, inchikey))
    } else {
        Ok(("NA".to_string(), "NA".to_string()))
    }
}

//******************************************************************/
/// Returns the SDF content of the compound.
pub fn get_sdf(cid: isize) -> Result<String, Box<dyn Error>> {
    let url: String =
        PUG_REST.to_owned() + "compound/cid/" + &cid.to_string() + "/SDF?record_type=2d";

    let res = reqwest::blocking::get(url)?;

    // Wait 200ms to avoid overloading the PubChem servers
    // 5 request per second TOP;
    thread::sleep(DELAY);

    Ok(res.text()?)
}

//******************************************************************/
/// Search cids using the molecular formula
pub fn search_formula(formula: &str) -> Result<String, Box<dyn Error>> {
    let url = PUG_REST.to_owned() + "compound/formula/" + formula + "/JSON?MaxRecords=5";

    let res = reqwest::blocking::get(dbg!(url))?;

    // Wait 200ms to avoid overloading the PubChem servers
    // 5 request per second TOP;
    thread::sleep(DELAY);

    let txt = res.text()?;
    let parsed = json::parse(&txt)?;

    let message = parsed["Waiting"]["Message"].to_string();

    if String::from("Your request is running") == message {
        let list_key = parsed["Waiting"]["ListKey"].to_string();

        let url = PUG_REST.to_owned() + "compound/listkey/" + &list_key + "/cids/JSON";

        let res = reqwest::blocking::get(url)?;

        // Wait 200ms to avoid overloading the PubChem servers
        // 5 request per second TOP;
        thread::sleep(DELAY);

        let cid_list = json::parse(&res.text()?)?;
        dbg!(cid_list);

        Ok("complete".to_string())
    } else {
        Ok("NA".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_formula() {
        let result = search_formula("C10H21N");
        println!("result = {:?}", result);
    }
}
