use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use glob::glob;
use rayon::prelude::*;

#[allow(non_snake_case)]
pub struct Workspace {
  pub dir: std::path::PathBuf,
  pub packageJson: serde_json::Value
}

/// .
///
/// # Panics
///
/// Panics if .
pub fn get_workspaces(pkg_glob: String) -> Vec<Workspace> {
  let mut pkg_jsons: Vec<PathBuf> = vec![];
  
  glob(&pkg_glob).expect("globbing failed")
    .map(|path| path.expect("globbing failed"))    
    .for_each(|entry| {
      pkg_jsons.push(entry);
    });
  
  let pkg_map: Vec<Workspace> = pkg_jsons.into_par_iter() 
    .map(|entry| {
      let pkg_path = Path::new(&entry);
      let file = File::open(pkg_path).expect("reader issue");
      
      let json: serde_json::Value = serde_json::from_reader(file).expect("json parsing");
      
      let pkg = Workspace { dir: entry, packageJson: json };
      pkg
    }).collect();
  
  pkg_map
}