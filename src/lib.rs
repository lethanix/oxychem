use http::StatusCode;
use json;
use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::{thread, time};

const DELAY: time::Duration = time::Duration::from_millis(200);

//******************************************************************/
pub fn get_cid(compound_name: String) -> Result<isize, Box<dyn Error>> {
    let url: String = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/".to_string()
        + &compound_name
        + "/cids/JSON";

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
    let url: String = "https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/".to_string()
        + &cid.to_string()
        + "/JSON?heading=CAS";

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
pub fn get_properties(cid: isize) -> Result<(String, String), Box<dyn Error>> {
    let url: String = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/".to_string()
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
pub fn get_sdf(cid: isize) -> Result<String, Box<dyn Error>> {
    let url: String = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/".to_string()
        + &cid.to_string()
        + "/SDF?record_type=2d";

    let res = reqwest::blocking::get(url)?;

    // Wait 200ms to avoid overloading the PubChem servers
    // 5 request per second TOP;
    thread::sleep(DELAY);

    Ok(res.text()?)
}
