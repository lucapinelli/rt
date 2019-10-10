use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::util::arguments::Arguments;

#[derive(Debug)]
pub struct Core {
    options: Arguments,
    dev_excludes: HashSet<String>, //HashSet::from_iter
}

impl Core {
    pub fn new(options: &Arguments) -> Core {
        let mut dev_excludes = HashSet::new();
        dev_excludes.insert(".git".to_string());
        dev_excludes.insert("target".to_string());
        dev_excludes.insert("node_modules".to_string());
        dev_excludes.insert("build".to_string());
        dev_excludes.insert("bin".to_string());
        Core {
            options: options.clone(),
            dev_excludes,
        }
    }

    fn get_name(&self, path: &Path) -> Result<(String), Box<dyn Error>> {
        match path.file_name() {
            Some(name) => Ok(name.to_str().ok_or("")?.to_string()),
            None => match path.canonicalize()?.file_name() {
                Some(name) => Ok(name.to_str().ok_or("")?.to_string()),
                None => Ok(path.canonicalize()?.to_str().ok_or("")?.to_string()),
            },
        }
    }

    fn get_relative_path(&self, path: &Path) -> Result<(String), Box<dyn Error>> {
        Ok(path.to_str().ok_or("-??-")?.to_string())
    }

    fn get_absolute_path(&self, path: &Path) -> Result<(String), Box<dyn Error>> {
        Ok(path.canonicalize()?.to_str().ok_or("-??-")?.to_string())
    }

    pub fn visit_path(&self, path: &Path, level: u32) -> Result<(), Box<dyn Error>> {
        let Arguments {
            path: _path,
            levels: max_level,
            style,
            hidden,
            development,
            tab,
            exclude,
        } = &self.options;

        let name = self.get_name(&path)?;
        if !hidden && name.starts_with('.') {
            return Ok(());
        }
        if *development && self.dev_excludes.contains(&name) {
            return Ok(());
        }
        if exclude.is_some() && exclude.as_ref().unwrap().is_match(&name) {
            return Ok(());
        }
        
        self.print_item(&path, level, &style, *tab)?;
        if *max_level != 0 as u32 && level >= *max_level {
            return Ok(());
        }
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                self.visit_path(&entry?.path(), level + 1)?;
            }
        }
        Ok(())
    }

    fn print_item(
        &self,
        path: &Path,
        level: u32,
        style: &String,
        tab: u32,
    ) -> Result<(), Box<dyn Error>> {
        let item = if style == "absolute" {
            self.get_absolute_path(path)?
        } else if style == "relative" {
            self.get_relative_path(path)?
        } else {
            let spaces = if tab > 0 { tab } else { 2 };
            let mut pad: String = format!("{: >level$}* ", "", level = (level * spaces) as usize);
            pad.push_str(&self.get_name(path)?);
            pad
        };
        println!("{}", &item);
        Ok(())
    }
}
