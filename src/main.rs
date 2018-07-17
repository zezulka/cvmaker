// From the use cases, we can derive individual responsibilities (=modules) the cvmaker has:
//     basic structs common to all the modules (CV struct, for example)    -> base
//     Render a CV based on the selected data
//     Serialize/Deserialize JSON from the CV sources folder
//     there will also be some configuration necessary (at least folder in which the data resides)
//     DAO structs which will manipulate the CV structs                    -> dao
//     part of the application which will handle graphics and user events
//         a subset of this module should also define any necessary structs (like buttons or forms)
//
extern crate cvmaker;

fn main() {
    if let Err(e) = cvmaker::run() {
        eprintln!("Application error : {}", e);
    }
}
